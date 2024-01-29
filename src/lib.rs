#[macro_use]
extern crate rocket;

pub mod utils;
pub mod alexa_top1m;

use rocket_db_pools::{Connection, deadpool_redis};
use crate::alexa_top1m::{AlexaTop1M, RankedDomain};

use std::env;
use std::path::Path;
use rocket::{Build, Rocket, State};
use rocket::serde::json::Json;
use rust_jarm::Jarm;
use serde::Serialize;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use rocket::serde::Deserialize;
use rocket::serde::json::serde_json;
use rust_jarm::error::JarmError;
use rocket_db_pools::{Database};
use rocket_db_pools::deadpool_redis::redis::{AsyncCommands};

pub const DEFAULT_SCAN_TIMEOUT_IN_SECONDS: u64 = 15;
pub const REDIS_LAST_SCAN_LIST_KEY: &str = "redis_last_scan_list_key";

pub const LAST_SCAN_SIZE_RETURNED: isize = 10;

#[derive(Database)]
#[database("redis_db")]
struct Db(deadpool_redis::Pool);

#[derive(Serialize)]
struct ErrorResponse {
    // TODO rename in JarmErrorResponse
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

#[derive(Serialize, Deserialize)]
struct LastScanResponse {
    host: String,
    port: String,
    jarm_hash: String,
}  // TODO timestamp ?

#[derive(Serialize)]
struct LastScanListResponse {
    last_scans: Vec<LastScanResponse>,
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
async fn jarm(host: String, port: Option<String>, mut redis_client: Connection<Db>) -> Json<JarmResponse> {
    let _port = port.unwrap_or_else(|| "443".to_string());
    let _host = utils::sanitize_host(&host);
    let jarm_hash = {
        let mut jarm_scan = Jarm::new(
            _host.clone(),
            _port.clone(),
        );
        jarm_scan.timeout = Duration::from_secs(scan_timeout_in_seconds());
        match jarm_scan.hash() {
            Ok(hash) => hash,
            Err(jarm_error) => {
                return build_error_json(jarm_error);
            }
        }
    };

    // We save jarm results only if valid
    let scan = LastScanResponse { host: _host, port: _port, jarm_hash };
    let serialized_scan = serde_json::to_string(&scan).unwrap();
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let epoch = since_the_epoch.as_secs();
    let _: () = redis_client.zadd(REDIS_LAST_SCAN_LIST_KEY, serialized_scan, epoch).await.unwrap();

    let last_scan_count: isize = redis_client.zcount(REDIS_LAST_SCAN_LIST_KEY, "-inf", "+inf").await.unwrap();
    if last_scan_count > LAST_SCAN_SIZE_RETURNED {  // pop the results above the defined limit
        let pop_number = last_scan_count - LAST_SCAN_SIZE_RETURNED;
        let _: () = redis_client.zpopmin(REDIS_LAST_SCAN_LIST_KEY, pop_number).await.unwrap();
    }
    Json(JarmResponse { host: scan.host, port: scan.port, jarm_hash: scan.jarm_hash, error: None })
}

#[get("/")]
async fn last_scans(mut redis_client: Connection<Db>) -> Json<LastScanListResponse> {
    let redis_last_scans: Vec<String> = redis_client.zrangebyscore(REDIS_LAST_SCAN_LIST_KEY, "-inf", "+inf").await.unwrap();
    let mut last_scans = vec![];
    for scan in redis_last_scans {
        last_scans.push(serde_json::from_str(&scan).unwrap());
    }
    Json(LastScanListResponse { last_scans })
}

#[get("/?<jarm_hash>")]
fn alexa_overlap(alexa_top1m: &State<AlexaTop1M>, jarm_hash: String) -> Json<AlexaOverlapResponse> {  // TODO try str
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

pub fn build_rocket() -> Rocket<Build> {
    let alexa_top1m = AlexaTop1M::new(&alexa_top1m_raw_data_path())
        .expect("AlexaTop1M built correctly");
    rocket::build()
        .mount("/jarm", routes![jarm])
        .mount("/last-scans", routes![last_scans])
        .mount("/alexa-overlap", routes![alexa_overlap])
        .attach(Db::init())
        .manage(alexa_top1m)
}
