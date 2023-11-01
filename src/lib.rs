#[path = "dinos/dinos.rs"]
pub mod dinos;

#[path = "enron/enron.rs"]
pub mod enron;

#[path = "location/location.rs"]
pub mod location;

#[path = "metrics/metrics.rs"]
pub mod metrics;

#[path = "opengraph/opengraph.rs"]
pub mod opengraph;

#[path = "root/root.rs"]
pub mod root;

#[path = "slow/slow.rs"]
pub mod slow;

use parking_lot::Mutex;
use reqwest::Client;
use std::sync::Arc;

pub struct State {
    pub client: Client,
    pub db_connection: Arc<Mutex<rusqlite::Connection>>,
}

use rocket::response::status;
use rocket::serde::{json::Json, Serialize};

pub type SimpleResponse = status::Custom<Json<SimpleResponseStructure>>;
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SimpleResponseStructure {
    success: bool,
    message: serde_json::Value,
}

#[macro_export]
macro_rules! verify_token {
    ($token:expr) => {
        if $token.is_none() {
            return rocket::response::status::Custom(
                rocket::http::Status::Unauthorized,
                rocket::serde::json::Json(SimpleResponseStructure {
                    success: false,
                    message: "Missing token.".into(),
                }),
            );
        }
        if $token != Some(std::env::var("secret_token").expect("the secret token")) {
            return rocket::response::status::Custom(
                rocket::http::Status::Unauthorized,
                rocket::serde::json::Json(SimpleResponseStructure {
                    success: false,
                    message: "Bad token. Please stop.".into(),
                }),
            );
        }
    };
}

#[macro_export]
macro_rules! ok_response {
    ($message:expr) => {
        return rocket::response::status::Custom(
            rocket::http::Status::Ok,
            rocket::serde::json::Json(SimpleResponseStructure {
                success: true,
                message: $message.into(),
            }),
        )
    };
}

#[macro_export]
macro_rules! err_response {
    ($message:expr) => {
        rocket::response::status::Custom(
            rocket::http::Status::InternalServerError,
            rocket::serde::json::Json(SimpleResponseStructure {
                success: false,
                message: $message.into(),
            }),
        )
    };
}
