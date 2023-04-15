#[get("/<delay>")]
pub async fn root(delay: Option<String>) {
    let delay = delay.map_or(0, |d| d.parse::<u64>().unwrap_or(0));

    rocket::tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
}
