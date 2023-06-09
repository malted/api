#[macro_use]
extern crate rocket;
mod fairings;
use api::{db, dinos, enron, location, metrics, opengraph, root, slow, State};
use dotenv::dotenv;
use rocket::fairing::AdHoc;
use std::sync::{Arc, Mutex};

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let client = reqwest::Client::new();
    let db_connection = Arc::new(Mutex::new(
        rusqlite::Connection::open("../api-db.sqlite").expect("open the db"),
    ));

    let _f = fairings::Counter::default();

    rocket::build()
        // .attach(f)
        .attach(AdHoc::on_ignite("Location state", |rocket| async {
            let location = Arc::new(Mutex::new(String::new()));
            rocket.manage(location)
        }))
        .manage(State {
            client,
            db_connection,
        })
        .mount("/", routes![root::root])
        .mount("/enron", routes![enron::random])
        .mount("/dinos", routes![dinos::random])
        .mount("/slow", routes![slow::root])
        .mount(
            "/metrics",
            routes![
                metrics::visitors::increment_visitors,
                metrics::visitors::get_visitors
            ],
        )
        .mount(
            "/location",
            routes![location::patch_location, location::get_location],
        )
        .mount("/opengraph", routes![opengraph::get_ogp])
        .mount("/db", routes![db::set_kv])
}
