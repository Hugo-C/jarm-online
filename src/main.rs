#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use serde::Serialize;
use rocket_contrib::json::Json;
use rust_jarm::Jarm;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize)]
struct JarmResponse {
    host: String,
    port: String,
    jarm_hash: String
}

#[get("/?<host>&<port>")]
fn jarm(host: String, port: Option<String>) -> Json<JarmResponse> {
    let _port = port.unwrap_or("443".to_string());
    let jarm_hash = Jarm::new(host.clone(), _port.clone()).hash()
        .expect("failed to connect");  // TODO handle error
    Json(JarmResponse { host, port: _port, jarm_hash })
}

fn main() {
    rocket::ignite().mount("/jarm", routes![jarm]).launch();
}