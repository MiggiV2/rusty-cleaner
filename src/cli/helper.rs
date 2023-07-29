use std::{fs, io};
use std::fs::{create_dir_all, File};
use std::io::{BufReader, Read, Write};
use std::path::MAIN_SEPARATOR_STR;

use crate::cli::CLI;
use crate::parser::{CSVParser, Message};

impl CLI {
    pub(in crate::cli) fn ask_for_input() -> String {
        let mut user_input = String::new();
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        user_input.trim().to_string()
    }


    pub(in crate::cli) fn move_finished(&mut self, channel: String) {
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

    pub(in crate::cli) fn save_next_msg(&self, msg: &Message) {
        let msg_dir = self.package_path.to_string() + MAIN_SEPARATOR_STR + "stopped" +
            MAIN_SEPARATOR_STR + "c" + msg.channel_id.as_str();
        let path = msg_dir.to_string() + MAIN_SEPARATOR_STR + "messages.csv";
        let contents =
            format!("ID,Timestamp,Contents,Attachments\n{},Null,Null,Null,Null", msg.id);
        let _ = create_dir_all(msg_dir);
        self.save_content(path, contents);
    }

    pub(in crate::cli) fn save_current_channel(&self) {
        let stopped_dir = self.package_path.to_string() + MAIN_SEPARATOR_STR + "stopped";
        let ch_path = stopped_dir.to_string() + MAIN_SEPARATOR_STR + "channel.txt";
        let _ = create_dir_all(&stopped_dir);
        self.save_content(ch_path, self.current_channel_id.to_string());
    }

    pub(in crate::cli) fn remove_msg_and_ch_files(&self) {
        let stopped_dir = self.package_path.to_string() + MAIN_SEPARATOR_STR + "stopped";
        let res = fs::remove_dir_all(&stopped_dir);
        if let Err(e) = res {
            eprintln!("Failed to remove stopped folder! Path: {} -> {}", &stopped_dir, e);
        }
    }

    /**
    Stopped channel has to be set before!
     */
    pub(in crate::cli) fn read_stopped_msg_id(&self) -> Option<String> {
        let msg_dir = self.package_path.to_string() + MAIN_SEPARATOR_STR + "stopped" +
            MAIN_SEPARATOR_STR + "c" + self.stopped_channel_id.as_str();
        let parser = CSVParser::new(msg_dir);
        if let Some(first) = parser.parse_file().first() {
            return Some(first.id.to_string());
        }
        None
    }

    pub(in crate::cli) fn read_stopped_channel(&self) -> Option<String> {
        let ch_path = self.package_path.to_string() + MAIN_SEPARATOR_STR + "stopped"
            + MAIN_SEPARATOR_STR + "channel.txt";
        return match File::open(&ch_path) {
            Ok(f) => {
                Self::read_file(f)
            }
            Err(_) => {
                None
            }
        };
    }

    fn read_file(f: File) -> Option<String> {
        let mut content = String::new();
        let mut buf_reader = BufReader::new(f);
        let res = buf_reader.read_to_string(&mut content);
        return match res {
            Ok(_) => {
                Some(content)
            }
            Err(_) => {
                None
            }
        };
    }
}