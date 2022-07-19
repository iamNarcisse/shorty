#[macro_use]
extern crate rocket;
mod hasher;
mod service;
mod store;

use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use service::Service;
use std::sync::RwLock;

#[derive(Deserialize, Serialize)]
struct Payload {
    url: String,
}

#[derive(Deserialize, Serialize)]
struct ErrorResponse {
    message: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/shorten", format = "json", data = "<payload>")]
fn store_url(srv: &State<RwLock<Service>>, payload: Json<Payload>) -> Json<Payload> {
    let mut service = srv.write().unwrap();
    let id = service.store(&payload.url);
    Json(Payload { url: id })
}

#[get("/<id>")]
async fn retrieve(
    srv: &State<RwLock<Service>>,
    id: String,
) -> Result<Json<Payload>, Custom<String>> {
    let mut service = srv.write().unwrap();

    let url = match service.retrieve(&id) {
        Some(value) => value,
        None => {
            return Err(Custom(
                Status::BadRequest,
                "The provided link does not exist".to_string(),
            ))
        }
    };

    Ok(Json(Payload { url }))
}

#[launch]
fn rocket() -> _ {
    let srv = Service::new();
    let service = RwLock::new(srv);
    rocket::build()
        .manage(service)
        .mount("/", routes![index, retrieve, store_url])
}
