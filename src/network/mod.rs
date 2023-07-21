use ureq::{Error, Response};

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

    pub fn delete_simple(&self, msg: Message) {
        match self.delete_msg(msg) {
            Ok(r) => {
                self.handle_response(r);
            }
            Err(e) => {
                println!("Request failed! {}", e.to_string());
            }
        }
    }

    pub fn delete_msg(&self, msg: Message) -> Result<Response, Error> {
        let url = format!("{}channels/{}/messages/{}", self.base_url, msg.channel_id, msg.id);
        let agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
        (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";
        ureq::delete(&*url)
            .set("Authorization", &*self.auth_token)
            .set("User-Agent", agent)
            .call()
    }

    fn handle_response(&self, r: Response) {
        if r.status() == 204 {
            println!("Message deleted!");
        } else {
            eprintln!("Unexpected response code {}\nBody:{}",
                      r.status(),
                      r.into_string().unwrap_or("No Body".to_string()));
        }
    }
}