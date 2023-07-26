use std::thread;
use std::time::Duration;

use ureq::{Error, Response};

use crate::cli::CLI;
use crate::parser::Message;

pub struct Cleaner {
    base_url: String,
    auth_token: String,
}

impl Cleaner {
    pub fn new(auth_token: String) -> Self {
        Self {
            base_url: "https://discord.com/api/v9/".to_string(),
            auth_token,
        }
    }

    pub fn delete_simple(&self, msg: &Message, cli: &CLI) {
        let has_error;

        match self.delete_msg(msg) {
            Ok(r) => {
                has_error = self.handle_response(r);
            }
            Err(e) => {
                println!("Request failed! {}", e.to_string());
                has_error = true
            }
        }

        if has_error {
            let res = cli.save_failed_msg(msg);
            if res.is_some() {
                eprintln!("Failed to store failed msg -> {}", res.unwrap());
            }
        }
    }

    pub fn delete_msg(&self, msg: &Message) -> Result<Response, Error> {
        let url = format!("{}channels/{}/messages/{}", self.base_url, msg.channel_id, msg.id);
        let agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
        (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";
        ureq::delete(&*url)
            .set("Authorization", self.auth_token.as_str())
            .set("User-Agent", agent)
            .call()
    }

    /**
    return true on error!
     */
    fn handle_response(&self, r: Response) -> bool {
        match r.status() {
            204 => {
                println!("Message deleted!");
                false
            }
            403 => {
                eprintln!("Forbidden");
                false // dont save as failed -> dont try again!
            }
            429 => {
                eprintln!("Too Many Requests");
                thread::sleep(Duration::from_secs(2));
                true
            }
            _ => {
                eprintln!("Unexpected response code {}\nBody:{}",
                          r.status(),
                          r.into_string().unwrap_or("No Body".to_string()));
                true
            }
        }
    }
}