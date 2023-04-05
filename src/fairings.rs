use std::sync::atomic::{AtomicUsize, Ordering};
use rocket::{Request, Orbit, Data, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Method, ContentType, Status};
use rocket::Rocket;

pub struct RequestCounter {
    count: AtomicUsize,
}
impl RequestCounter {
    pub fn new() -> Self {
        Self {
            count: AtomicUsize::new(0),
        }
    }

    pub fn count(&self) -> usize {
        self.count.load(Ordering::Relaxed)
    }
}

#[rocket::async_trait]
impl Fairing for RequestCounter {
    fn info(&self) -> Info {
        Info {
            name: "Request counter",
            kind: Kind::Liftoff | Kind::Request | Kind::Shutdown,
        }
    }

    async fn on_liftoff(&self, _rocket: &Rocket<Orbit>) {
        let count = std::fs::read_to_string("request-count.txt")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        self.count.store(count, Ordering::Relaxed);
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    async fn on_shutdown(&self, _rocket: &Rocket<Orbit>) {
        std::fs::write("request-count.txt", self.count.load(Ordering::Relaxed).to_string())
            .expect("Failed to write request count to file");
    }
}