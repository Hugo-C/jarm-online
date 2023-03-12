#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod utils;
pub mod alexa_top1m;

use crate::alexa_top1m::{AlexaTop1M, RankedDomain};

use std::env;
use std::path::Path;
use rocket::{Rocket, State};
use rocket_contrib::json::Json;
use rust_jarm::Jarm;
use serde::Serialize;
use std::time::Duration;
use rust_jarm::error::JarmError;

pub const DEFAULT_SCAN_TIMEOUT_IN_SECONDS: u64 = 15;

#[derive(Serialize)]
struct ErrorResponse {
    error_type: String,
    error_message: String,
}

#[derive(Serialize)]
struct JarmResponse {
    host: String,
    port: String,
    jarm_hash: String,
    error: Option<ErrorResponse>,
}

#[derive(Serialize)]
struct AlexaOverlapResponse {
    overlapping_domains: Vec<RankedDomain>,
}

pub fn scan_timeout_in_seconds() -> u64 {
    env::var("SCAN_TIMEOUT_IN_SECONDS")
        .unwrap_or(DEFAULT_SCAN_TIMEOUT_IN_SECONDS.to_string())
        .parse::<u64>()
        .expect("Valid timeout value")
}

pub fn alexa_top1m_raw_data_path() -> Box<Path> {
    let raw_path = env::var("ALEXA_TOP1M_RAW_DATA_PATH")
        .expect("ALEXA_TOP1M_RAW_DATA_PATH env var has to be set");
    Path::new(&raw_path).into()
}

#[get("/?<host>&<port>")]
fn jarm(host: String, port: Option<String>) -> Json<JarmResponse> {
    let _port = port.unwrap_or_else(|| "443".to_string());
    let _host = utils::sanitize_host(&host);
    let mut jarm_scan = Jarm::new(
        _host.clone(),
        _port.clone(),
    );
    jarm_scan.timeout = Duration::from_secs(scan_timeout_in_seconds());
    let jarm_hash = match jarm_scan.hash() {
        Ok(hash) => hash,
        Err(jarm_error) => {
            return build_error_json(jarm_error);
        }
    };
    Json(JarmResponse { host: _host, port: _port, jarm_hash, error: None })
}

#[get("/?<jarm_hash>")]
fn alexa_overlap(alexa_top1m: State<AlexaTop1M>, jarm_hash: String) -> Json<AlexaOverlapResponse> {  // TODO try str
    let overlap = match alexa_top1m.get(jarm_hash.as_str()) {
        None => vec![],
        Some(overlap) => overlap.to_vec()
    };
    Json(AlexaOverlapResponse { overlapping_domains: overlap })
}

fn build_error_json(jarm_error: JarmError) -> Json<JarmResponse> {
    // error_message is a debug view of a an unknown error, to be improved.
    let (error_type, error_message) = match jarm_error {
        JarmError::DnsResolve(e) => {
            ("Dns resolve error".to_string(), format!("{e:?}"))
        }
        JarmError::Connection(e) => {
            ("Connection error".to_string(), format!("{e:?}"))
        }
        JarmError::Io(e) => {
            ("Input/output error".to_string(), format!("{e:?}"))
        }
    };
    Json(JarmResponse {
        host: "".to_string(),
        port: "".to_string(),
        jarm_hash: "".to_string(),
        error: Some(ErrorResponse { error_type, error_message }),
    })
}

pub fn set_up_rocket() -> Rocket {
    let alexa_top1m = AlexaTop1M::new(&alexa_top1m_raw_data_path())
        .expect("AlexaTop1M built correctly");
    rocket::ignite()
        .mount("/jarm", routes![jarm])
        .mount("/alexa-overlap", routes![alexa_overlap])
        .manage(alexa_top1m)
}
