use rocket::serde::{json::Json, Serialize};
use rocket::State;
use std::env::var;
use std::sync::{Arc, Mutex};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    success: bool,
    message: String,
}

#[rocket::patch("/?<token>&<location>")]
pub fn patch_location(
    counter: &State<Arc<Mutex<String>>>,
    token: Option<String>,
    location: Option<String>,
) -> Json<Response> {
    if token != Some(var("secret_token").unwrap()) {
        return Json(Response {
            success: false,
            message: "Invalid token".to_string(),
        });
    }
    if location.is_none() {
        return Json(Response {
            success: false,
            message: "Missing location".to_string(),
        });
    }

    *counter.lock().expect("lock counter") = location.unwrap();

    Json(Response {
        success: true,
        message: "Location saved".to_string(),
    })
}

#[rocket::get("/?<token>")]
pub fn get_location(counter: &State<Arc<Mutex<String>>>, token: Option<String>) -> Json<Response> {
    if token != Some(var("secret_token").unwrap()) {
        return Json(Response {
            success: false,
            message: "Invalid token".to_string(),
        });
    }

    let location = counter.lock().expect("lock counter").clone();

    return Json(Response {
        success: true,
        message: location,
    });
}
