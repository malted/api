use mail_parser::*;
use rand::prelude::SliceRandom;
use rocket::serde::{json::Json, Serialize};
use std::path::PathBuf;

#[derive(Debug)]
struct EmailLocation {
    dir: PathBuf,
    location: Vec<String>,
}
impl EmailLocation {
    fn new() -> Self {
        let mut dir = std::env::current_dir().expect("the current directory");
        dir.push("data");
        dir.push("enron-emails");

        let mut location = Vec::new();

        // Keep choosing random directories until we find a file
        while dir.is_dir() {
            let file_name = dir
                .read_dir()
                .expect("the directory")
                .map(|entry| entry.unwrap().path())
                .collect::<Vec<_>>()
                .choose(&mut rand::thread_rng())
                .unwrap()
                .file_name()
                .expect("the file name")
                .to_str()
                .expect("a valid utf-8 string")
                .to_string();

            dir.push(&file_name);
            location.push(file_name);
        }

        Self { dir, location }
    }
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Email {
    path: Vec<String>,
    timestamp: i64,
    from: String,
    to: Vec<String>,
    subject: Option<String>,
    bcc: Vec<String>,
    cc: Vec<String>,
    body: String,
}
impl Email {
    fn random(format: Option<String>) -> Self {
        let email_location = EmailLocation::new();

        let file = std::fs::read(email_location.dir).expect("the email file exists");

        let message = Message::parse(file.as_slice()).unwrap();

        fn get_addresses(header: &HeaderValue) -> Vec<String> {
            match header {
                HeaderValue::Address(addrs) => {
                    if let Some(addr) = &addrs.address {
                        vec![addr.to_string()]
                    } else {
                        Vec::new()
                    }
                }
                HeaderValue::AddressList(addrs) => addrs
                    .iter()
                    .map(|addr| {
                        if let Some(addr) = &addr.address {
                            addr.to_string()
                        } else {
                            String::new()
                        }
                    })
                    .collect(),
                _ => Vec::new(),
            }
        }

        macro_rules! recompute_if_none {
            ($expr:expr) => {
                match $expr {
                    Some(val) => val,
                    None => return Self::random(format),
                }
            };
        }

        let body = recompute_if_none!(match format.as_deref() {
            Some("html") => message.body_html(0),
            _ => message.body_text(0),
        })
        .to_string();

        Self {
            path: email_location.location,
            timestamp: recompute_if_none!(message.date()).to_timestamp(),
            from: recompute_if_none!(get_addresses(message.from()).get(0)).to_owned(),
            to: get_addresses(message.to()),
            subject: message.subject().map(|s| s.to_string()),
            bcc: get_addresses(message.bcc()),
            cc: get_addresses(message.cc()),
            body,
        }
    }
}

#[rocket::get("/random?<format>")]
pub fn random(format: Option<String>) -> Json<Email> {
    Json(Email::random(format))
}
