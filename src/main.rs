#[macro_use]
extern crate rocket;

use std::sync::Arc;

mod dinos;
mod enron;
mod fairings;
mod root;

pub struct MainState {
    request_counter: Arc<fairings::RequestCounter>,
}

#[launch]
fn rocket() -> _ {
    let request_counter = Arc::new(fairings::RequestCounter::new());

    rocket::build()
        .manage(MainState {
            request_counter: request_counter.clone(),
        })
        .attach(request_counter)
        .mount("/", routes![root::root])
        .mount("/enron", routes![enron::random])
        .mount("/dinos", routes![dinos::random])
}
