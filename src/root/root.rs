use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RootResponse {
    success: bool,
}

#[rocket::get("/")]
pub fn root() -> Json<RootResponse> {
    Json(RootResponse { success: true })
}
