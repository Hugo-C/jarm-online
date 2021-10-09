#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rust_jarm::Jarm;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/?<host>&<port>")]
fn jarm(host: String, port: Option<String>) -> String {
    let host_port = port.unwrap_or("443".to_string());
    let hash = Jarm::new(host, host_port).hash().expect("failed to connect");
    format!("Jarm hash is {}!", hash)
}

fn main() {
    rocket::ignite().mount("/jarm", routes![jarm]).launch();
}