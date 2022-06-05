#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use serde::Serialize;
use rocket_contrib::json::Json;
use rust_jarm::Jarm;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use jarm_online::sanitize_host;

pub struct CORS;

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    fn on_response(&self, _request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

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
    let _host = sanitize_host(&host);
    let jarm_hash = Jarm::new(
        _host.clone(),
        _port.clone(),
    )
        .hash()
        .expect("failed to connect");  // TODO handle error
    Json(JarmResponse { host: _host, port: _port, jarm_hash })
}

fn main() {
    rocket::ignite().attach(CORS).mount("/jarm", routes![jarm]).launch();
}