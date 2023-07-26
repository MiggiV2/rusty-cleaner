use std::{fs, io};
use std::io::Write;

use crate::cli::CLI;

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
}