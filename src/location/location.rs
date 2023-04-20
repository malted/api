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

#[rocket::patch("/?<token>&<lat>&<lon>")]
pub fn patch_location(
    counter: &State<Arc<Mutex<(String, String)>>>,
    token: Option<String>,
    lat: Option<String>,
    lon: Option<String>,
) -> Json<Response> {
    if token != Some(var("secret_token").unwrap()) {
        return Json(Response {
            success: false,
            message: "Invalid token".to_string(),
        });
    }
    if lat.is_none() || lon.is_none() {
        return Json(Response {
            success: false,
            message: "Missing lat/lon".to_string(),
        });
    }

    *counter.lock().expect("lock counter") = (lat.unwrap(), lon.unwrap());

    Json(Response {
        success: true,
        message: "Location saved".to_string(),
    })
}

#[rocket::get("/?<token>")]
pub fn get_location(
    counter: &State<Arc<Mutex<(String, String)>>>,
    token: Option<String>,
) -> Json<Response> {
    if token != Some(var("secret_token").unwrap()) {
        return Json(Response {
            success: false,
            message: "Invalid token".to_string(),
        });
    }

    let (lat, lon) = counter.lock().expect("lock counter").clone();

    return Json(Response {
        success: true,
        message: format!("{},{}", lat, lon),
    });
}
