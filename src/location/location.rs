use parking_lot::Mutex;
use rocket::serde::{json::Json, Serialize};
use rocket::State;
use std::env::var;
use std::sync::Arc;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    success: bool,
    message: String,
}

#[derive(Serialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct Location {
    coords: String,
    city: String,
    country: String,
    timestamp: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LocationResponse {
    success: bool,
    location: Option<Location>,
}

#[rocket::patch("/?<token>&<coords>&<city>&<country>&<timestamp>")]
pub fn patch_location(
    counter: &State<Arc<Mutex<Location>>>,
    token: Option<String>,
    coords: Option<String>,
    city: Option<String>,
    country: Option<String>,
    timestamp: Option<String>,
) -> Json<Response> {
    if token != Some(var("secret_token").unwrap()) {
        return Json(Response {
            success: false,
            message: "Invalid token".to_string(),
        });
    }

    if coords.is_none() {
        return Json(Response {
            success: false,
            message: "Missing coordinates".to_string(),
        });
    }

    let location = Location {
        coords: coords.unwrap(),
        city: city.unwrap_or("".to_string()),
        country: country.unwrap_or("".to_string()),
        timestamp: timestamp.unwrap_or("0".to_string());
    };

    *counter.lock() = location;

    Json(Response {
        success: true,
        message: "Location saved".to_string(),
    })
}

#[rocket::get("/?<token>")]
pub fn get_location(
    counter: &State<Arc<Mutex<Location>>>,
    token: Option<String>,
) -> Json<LocationResponse> {
    if token != Some(var("secret_token").unwrap()) {
        return Json(LocationResponse {
            success: false,
            location: None,
        });
    }

    let location = counter.lock();

    return Json(LocationResponse {
        success: true,
        location: Some(Location {
            coords: location.coords.clone(),
            city: location.city.clone(),
            country: location.country.clone(),
            timestamp: location.timestamp.clone(),
        }),
    });
}
