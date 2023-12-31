use std::fs::{create_dir_all, File, read_dir};
use std::io::{BufReader, Read, Write};
use std::path::MAIN_SEPARATOR_STR;

use crate::cli::CLI;
use crate::parser::Message;

impl CLI {
    pub fn save_failed_msg(&self, msg: &Message) -> Option<String> {
        let channel_dir = format!("failed_msg{}{}", MAIN_SEPARATOR_STR, msg.channel_id.to_string());
        if let Some(error_msg) = self.create_missing_dir(channel_dir.as_str()) {
            return Some(error_msg);
        }

        let path = self.package_path.to_string() + MAIN_SEPARATOR_STR + channel_dir.as_str()
            + MAIN_SEPARATOR_STR + "messages.csv";

        let contents = self.read_content(msg, &path);
        self.save_content(path, contents);
        None
    }

    pub(in crate::cli) fn save_content(&self, path: String, contents: String) {
        let file = File::create(path.to_string());
        if file.is_err() {
            eprintln!("Can't create message.csv -> {} - {}", path, file.err().unwrap());
        } else {
            let mut file = file.expect("Checked");
            let res = file.write_all(contents.as_bytes());
            if res.is_err() {
                eprintln!("Can't write to file!");
            }
        }
    }

    fn read_content(&self, msg: &Message, path: &String) -> String {
        let mut contents = "ID,Timestamp,Contents,Attachments\n".to_string()
            + msg.id.as_str() + ",NULL,NULL,NULL";
        if let Ok(f) = File::open(path) {
            contents = String::new();
            let mut buf_reader = BufReader::new(f);
            let res = buf_reader.read_to_string(&mut contents);
            if res.is_err() {
                eprintln!("Can't read msg file!");
            }
            contents = contents + "\n" + msg.id.as_str() + ",NULL,NULL,NULL";
        }
        contents
    }

    pub(in crate::cli) fn create_missing_dir(&self, dir: &str) -> Option<String> {
        let path = self.package_path.to_string() + MAIN_SEPARATOR_STR + dir;
        let error_dir = read_dir(path.to_string());
        if error_dir.is_err() {
            let error_dir = create_dir_all(path);
            if let Err(_) = error_dir {
                return Some("Failed to create removed folder!".to_string());
            }
        }
        None
    }
}