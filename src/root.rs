use crate::MainState;
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RootResponse {
    success: bool,
    requests: usize,
}

#[get("/")]
pub fn root(state: &rocket::State<MainState>) -> Json<RootResponse> {
    let requests = state.request_counter.count();

    Json(RootResponse {
        success: true,
        requests,
    })
}
