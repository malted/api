use rocket::serde::{json::Json, Serialize};
use std::env::var;
use std::io::{Read, Write};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    success: bool,
    message: String,
}

#[rocket::patch("/?<token>&<lat>&<lon>")]
pub fn patch_location(
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

    // Write the string "123" to file locations.txt
    let mut file = std::fs::File::create("location.csv").expect("the file to be created");
    file.write_all(format!("{},{}", lat.unwrap(), lon.unwrap()).as_bytes())
        .expect("the file to be written to");

    Json(Response {
        success: true,
        message: "Location saved".to_string(),
    })
}

#[rocket::get("/?<token>")]
pub fn get_location(token: Option<String>) -> Json<Response> {
    if token != Some(var("secret_token").unwrap()) {
        return Json(Response {
            success: false,
            message: "Invalid token".to_string(),
        });
    }

    if let Ok(mut file) = std::fs::File::open("location.csv") {
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("the file to be read");

        return Json(Response {
            success: true,
            message: contents,
        });
    } else {
        return Json(Response {
            success: false,
            message: "No location saved".to_string(),
        });
    }
}
