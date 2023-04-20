#[rocket::get("/<delay>")]
pub async fn root(delay: u64) {
    rocket::tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
}
