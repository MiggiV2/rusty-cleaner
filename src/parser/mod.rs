use std::fs::File;
use std::io::{BufRead, BufReader};

mod test_parser;

pub struct CSVParser {
    file_path: String,
}

pub struct Message {
    id: String,
    channel_id: String,
}

impl CSVParser {
    pub fn new(file_path: String) -> Self {
        Self {
            file_path
        }
    }

    pub fn parse_file(&self) -> Vec<Message> {
        let file = File::open(self.file_path.to_string() + "/messages.csv").unwrap();
        let reader = BufReader::new(file);
        let mut messages = vec![];

        for (index, line) in reader.lines().enumerate() {
            if index == 0 {
                continue;
            }
            let res = self.parse_line(line.unwrap());
            if res.is_err() {
                continue;
            }
            let msg = res.unwrap();
            messages.push(msg);
        }

        return messages;
    }

    pub fn parse_line(&self, line: String) -> Result<Message, String> {
        let items = line.trim().split(",");
        let mut list: Vec<&str> = vec![];
        let channel_id = self.get_channel_id_by_path();

        for item in items {
            list.push(item);
        }
        if list.len() < 3 {
            return Err(format!("Expected min. 3 commas! Found {}", list.len()));
        }
        if channel_id.is_none() {
            return Err(String::from("Can't find channel id in file path!"));
        }

        let msg = Message {
            id: list.first().unwrap().to_string(),
            channel_id: channel_id.expect("Checked").to_string(),
        };
        let is_id_valid = msg.id.parse::<u64>().is_ok();
        if !is_id_valid {
            return Err(format!("ID should be a number -> {}", msg.id));
        }
        Ok(msg)
    }

    pub fn get_channel_id_by_path(&self) -> Option<String> {
        let folders;

        if self.file_path.contains("/") {
            folders = self.file_path.split("/");
        } else {
            folders = self.file_path.split("\\");
        }

        let last = folders.last();
        if last.is_none() {
            return None;
        }
        return Some(last.unwrap().to_string().replace("c", ""));
    }
}