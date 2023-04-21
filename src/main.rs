#[macro_use]
extern crate rocket;
mod fairings;
use api::{dinos, enron, location, metrics, root, slow};
use dotenv::dotenv;
use rocket::fairing::AdHoc;
use std::sync::{Arc, Mutex};

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let _f = fairings::Counter::default();

    rocket::build()
        // .attach(f)
        .attach(AdHoc::on_ignite("Location state", |rocket| async {
            let location = Arc::new(Mutex::new(String::new()));
            rocket.manage(location)
        }))
        .mount("/", routes![root::root])
        .mount("/enron", routes![enron::random])
        .mount("/dinos", routes![dinos::random])
        .mount("/slow", routes![slow::root])
        .mount(
            "/metrics",
            routes![
                metrics::visitors::patch_visitors,
                metrics::visitors::get_visitors
            ],
        )
        .mount(
            "/location",
            routes![location::patch_location, location::get_location],
        )
}
