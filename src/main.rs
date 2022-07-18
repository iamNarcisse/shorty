#[macro_use]
extern crate rocket;
mod hasher;
mod service;
mod store;

use service::Service;

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::sync::RwLock;

#[derive(Deserialize, Serialize)]
struct Payload {
    url: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/shorten", format = "json", data = "<payload>")]
fn store_url(srv: &State<RwLock<Service>>, payload: Json<Payload>) -> Json<Payload> {
    let mut service = srv.write().unwrap();
    let id = service.store(&payload.url);
    println!("{}", id);
    Json(Payload { url: id })
}

#[get("/<id>")]
async fn retrieve(srv: &State<RwLock<Service>>, id: String) -> String {
    let mut service = srv.write().unwrap();

    match service.retrieve(&id) {
        Some(value) => value,
        None => "".to_string(),
    };

    format!("{}", id);
    id
}

#[launch]
fn rocket() -> _ {
    let srv = Service::new();
    let service = RwLock::new(srv);
    rocket::build()
        .manage(service)
        .mount("/", routes![index, retrieve, store_url])
}
