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

pub struct CLI {
    cleaner: Cleaner,
    package_path: String,
    current_channel_id: String,
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

        let cli = Self {
            package_path,
            cleaner: Cleaner::new(token),
            current_channel_id: String::new(),
        };
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
            self.current_channel_id = String::new();
            self.purge_channel(messages);
            self.move_finished(channel);
        }
        None
    }

    fn move_finished(&mut self, channel: String) {
        if !self.current_channel_id.is_empty() {
            let new_name = format!("{}/rusty-cleaned/c{}",
                                   self.package_path.to_string(),
                                   self.current_channel_id);
            let result = fs::rename(channel.to_string(), new_name.to_string());
            if let Err(e) = result {
                eprintln!("Failed to move - {} from {} to {}", e.to_string(), channel, new_name);
            }
        }
    }

    fn purge_channel(&mut self, messages: Vec<Message>) {
        let mut i = 0;
        let mut last_print_progress = 0.0;
        while i < messages.len() {
            let current_progress = i as f32 / messages.len() as f32 * 100.0;
            let msg = messages.get(i).expect("i < len");
            if self.current_channel_id.is_empty() {
                self.current_channel_id = msg.channel_id.to_string();
                println!("\nChannel: {}", self.current_channel_id);
            }

            let clone = Message::new(msg.channel_id.to_string(), msg.id.to_string());
            self.cleaner.delete_simple(clone, &self);
            if current_progress - last_print_progress > 5.0 || current_progress == 0.0 {
                print!("{:.2}%", current_progress);
                last_print_progress = current_progress;
            }
            print!(".");

            let _ = io::stdout().flush();
            let is_last = i + 1 == messages.len();
            if !is_last {
                let delay = thread_rng().gen_range(4000..6200);
                thread::sleep(Duration::from_millis(delay));
            }
            i += 1;
        }
    }

    fn ask_for_input() -> String {
        let mut user_input = String::new();
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        user_input.trim().to_string()
    }
}