use std::{env, fs, thread};
use std::io;
use std::io::Write;
use std::time::Duration;

use rand::{Rng, thread_rng};

use crate::network::Cleaner;
use crate::parser::CSVParser;

pub struct CLI {
    cleaner: Cleaner,
    package_path: String,
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

        print!("To delete your messages for you, I need your access token to Discord\nToken: ");
        let token = Self::ask_for_input();

        let cli = Self {
            package_path,
            cleaner: Cleaner::new(token),
        };
        Ok(cli)
    }

    pub fn delete_all(&self) -> Option<String> {
        let package = fs::read_dir(self.package_path.to_string() + "\\messages");
        if package.is_err() {
            return Some("package_path dont exists or can't be read".to_string());
        }

        let removed_dir = fs::read_dir(self.package_path.to_string() + "/rusty-cleaned");
        if removed_dir.is_err() {
            let removed_exist = fs::create_dir(self.package_path.to_string() + "/rusty-cleaned");
            if let Err(_) = removed_exist {
                return Some("Failed to create removed folder!".to_string());
            }
        }

        let package = package.unwrap();
        for channel_dir in package {
            let channel = channel_dir.unwrap().path().display().to_string();
            let parser = CSVParser::new(channel.to_string());

            let mut channel_id = String::new();
            for msg in parser.parse_file() {
                if channel_id.is_empty() {
                    channel_id = msg.channel_id.to_string();
                }
                println!("channel:{} - id:{}", msg.channel_id, msg.id);
                // self.cleaner.delete_simple(msg);
                let delay = thread_rng().gen_range(4000..7500);
                thread::sleep(Duration::from_millis(delay));
            }

            if !channel_id.is_empty() {
                let new_name = format!("{}/rusty-cleaned/c{}", self.package_path.to_string(), channel_id);
                let result = fs::rename(channel.to_string(), new_name.to_string());
                if let Err(e) = result {
                    eprintln!("Failed to move - {} from {} to {}", e.to_string(), channel, new_name);
                }
            }
        }
        None
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