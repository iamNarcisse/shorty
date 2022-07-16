#[macro_use]
extern crate rocket;
mod hasher;

use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Payload {
    link: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/post", format = "json", data = "<payload>")]
fn store(payload: Json<Payload>) -> Json<Payload> {
    println!("{}", payload.link);
    payload
}

#[get("/<id>")]
fn retrieve(id: String) -> String {
    format!("{}", id);
    id
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, retrieve, store])
}
