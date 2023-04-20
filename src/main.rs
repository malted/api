#[macro_use]
extern crate rocket;
mod fairings;
use api::{dinos, enron, location, metrics, root, slow};
use dotenv::dotenv;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    
    let f = fairings::Counter::default();

    rocket::build()
        .attach(f)
        .mount("/", routes![root::root])
        .mount("/enron", routes![enron::random])
        .mount("/dinos", routes![dinos::random])
        .mount("/slow", routes![slow::root])
        .mount("/metrics", routes![metrics::visitors::patch_visitors])
        .mount(
            "/location",
            routes![location::patch_location, location::get_location],
        )
}
