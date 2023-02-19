#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

pub mod utils;

use std::time::Duration;
use serde::Serialize;
use rocket_contrib::json::Json;
use rust_jarm::Jarm;
use rocket::Rocket;

const SCAN_TIMEOUT_IN_SECONDS: u64 = 15;


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
    let _port = port.unwrap_or_else(|| "443".to_string());
    let _host = utils::sanitize_host(&host);
    let mut jarm_scan = Jarm::new(
        _host.clone(),
        _port.clone(),
    );
    jarm_scan.timeout = Duration::from_secs(SCAN_TIMEOUT_IN_SECONDS);
    let jarm_hash = match jarm_scan.hash() {
        Ok(hash) => hash,
        Err(e) => {
            // println!("Error: {e:?}");
            // return;
            // TODO test
            panic!("AHHH");
        }
    };
    Json(JarmResponse { host: _host, port: _port, jarm_hash })
}

pub fn set_up_rocket() -> Rocket {
    rocket::ignite()
        .mount("/jarm", routes![jarm])
        .mount("/", routes![index])
}