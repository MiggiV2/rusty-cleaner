use std::{fs, io};
use std::io::Write;
use std::path::MAIN_SEPARATOR_STR;

use crate::cli::CLI;
use crate::parser::Message;

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
        let path = self.package_path.to_string() + MAIN_SEPARATOR_STR + "next-msg.csv";
        let contents =
            format!("ID,Timestamp,Contents,Attachments\n{},Null,Null,Null,Null", msg.id);
        self.save_content(path, contents);
    }

    pub(in crate::cli) fn save_current_channel(&self, msg: &Message) {
        let path = self.package_path.to_string() + MAIN_SEPARATOR_STR + "current-channel.txt";
        self.save_content(path, msg.channel_id.to_string());
    }

    pub(in crate::cli) fn remove_msg_and_ch_files(&self) {
        let msg_path = self.package_path.to_string() + MAIN_SEPARATOR_STR + "next-msg.csv";
        let ch_path = self.package_path.to_string() + MAIN_SEPARATOR_STR + "current-channel.txt";

        let result = fs::remove_file(&msg_path);
        if let Err(e) = result {
            eprintln!("Failed to remove next message file! Path: {} -> {}", msg_path, e);
        }

        let result = fs::remove_file(&ch_path);
        if let Err(e) = result {
            eprintln!("Failed to remove channel file! Path: {} -> {}", msg_path, e);
        }
    }
}