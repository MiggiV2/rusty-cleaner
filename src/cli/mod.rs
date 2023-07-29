use std::{env, fs, thread};
use std::io;
use std::io::Write;
use std::path::MAIN_SEPARATOR_STR;
use std::time::Duration;

use rand::{Rng, thread_rng};

use crate::network::Cleaner;
use crate::parser::{CSVParser, Message};

mod error_log;
mod welcome;
mod helper;

pub struct CLI {
    cleaner: Cleaner,
    package_path: String,
    current_channel_id: String,
    stopped_channel_id: String,
    stopped_msg_id: String,
}

impl CLI {
    pub fn new() -> Result<Self, String> {
        let mut args = env::args();
        args.next();
        let package_path = match args.next() {
            Some(package_path) => package_path,
            None => return Err("Pls add the package_path as argument".to_string())
        };

        let package = fs::read_dir(package_path.to_string() + "/messages");
        if package.is_err() {
            return Err("package_path dont exists or can't be read".to_string());
        }

        Self::print_welcome();

        print!("To delete your messages for you, I need your access token to Discord\nToken: ");
        let token = Self::ask_for_input();

        let mut cli = Self {
            package_path,
            cleaner: Cleaner::new(token),
            current_channel_id: String::new(),
            stopped_channel_id: String::new(),
            stopped_msg_id: String::new(),
        };

        // Restore stopped state
        if let Some(id) = cli.read_stopped_channel() {
            cli.stopped_channel_id = id;
        }
        if let Some(id) = cli.read_stopped_msg_id() {
            cli.stopped_msg_id = id;
        }

        Ok(cli)
    }

    pub fn delete_all(&mut self) -> Option<String> {
        let package = fs::read_dir(self.package_path.to_string() + MAIN_SEPARATOR_STR + "messages");
        if package.is_err() {
            return Some("package_path dont exists or can't be read".to_string());
        }

        if let Some(value) = self.create_missing_dir("rusty-cleaned") {
            return Some(value);
        }

        let package = package.unwrap();
        for channel_dir in package {
            let channel = channel_dir.unwrap().path().display().to_string();
            let parser = CSVParser::new(channel.to_string());

            let messages = parser.parse_file();
            self.current_channel_id = parser.get_channel_id_by_path().unwrap_or(String::new());
            println!("\nChannel: {}", self.current_channel_id.to_string());

            self.save_current_channel();
            self.purge_channel(messages, self.current_channel_id.to_string());
            self.move_finished(channel);

            let msg_dir = self.package_path.to_string() + MAIN_SEPARATOR_STR + "stopped" +
                MAIN_SEPARATOR_STR + "c" + self.current_channel_id.as_str();
            let res = fs::remove_dir_all(&msg_dir);
            if let Err(e) = res {
                eprintln!("Failed to remove stopped folder! Path: {} -> {}", &msg_dir, e);
            }
        }

        self.remove_msg_and_ch_files();
        None
    }

    fn purge_channel(&mut self, messages: Vec<Message>, channel_id: String) {
        let mut i = 0;
        let mut last_print_progress = 0.0;
        let has_state = !self.stopped_msg_id.is_empty() && !self.stopped_channel_id.is_empty();
        let is_stopped_ch = has_state && channel_id == self.stopped_channel_id;

        while i < messages.len() {
            let msg = messages.get(i).expect("i < len");

            if is_stopped_ch && msg.id != self.stopped_msg_id {
                i += 1;
                continue;
            }

            self.save_next_msg(msg);
            self.cleaner.delete_simple(msg, &self);
            last_print_progress = Self::print_status(messages.len(), i, last_print_progress);

            let is_last = i + 1 == messages.len();
            if !is_last {
                let delay = thread_rng().gen_range(4000..6200);
                thread::sleep(Duration::from_millis(delay));
            }
            i += 1;
        }
        println!("100%\ndone!")
    }

    fn print_status(msg_len: usize, i: usize, last_print_progress: f32) -> f32 {
        let current_progress = i as f32 / msg_len as f32 * 100.0;
        if current_progress - last_print_progress > 5.0 || current_progress == 0.0 {
            print!("{:.2}%", current_progress);
            let _ = io::stdout().flush();
            return current_progress;
        }
        let _ = io::stdout().flush();
        print!(".");
        last_print_progress
    }
}