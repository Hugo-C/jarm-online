#[macro_use]
extern crate rocket;

pub mod utils;
pub mod tranco_top1m;

use rocket_db_pools::{Connection, deadpool_redis};
use crate::tranco_top1m::{TrancoTop1M};
use crate::tranco_top1m::RankedDomain as TrancoRankedDomain;

use std::env;
use rocket::{Build, fairing, Rocket};
use rocket::serde::json::Json;
use rust_jarm::Jarm;
use serde::Serialize;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use reqwest::Url;
use rocket::fairing::AdHoc;
use rocket::response::status::Custom;
use rocket::http::Status;
use rocket::serde::Deserialize;
use rocket::serde::json::serde_json;
use rust_jarm::error::JarmError;
use rocket_db_pools::{Database};
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;

pub const DEFAULT_SCAN_TIMEOUT_IN_SECONDS: u64 = 15;
pub const REDIS_LAST_SCAN_LIST_KEY: &str = "redis_last_scan_list_key";
pub const SHODAN_HOST_COUNT_URL: &str = "https://api.shodan.io/shodan/host/count";

pub const LAST_SCAN_SIZE_RETURNED: isize = 10;

#[derive(Database)]
#[database("redis_db")]
pub struct Db(deadpool_redis::Pool);


#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
struct JarmErrorResponse {
    error_type: String,
    error_message: String,
}

#[derive(Serialize)]
struct JarmResponse {
    host: String,
    port: String,
    jarm_hash: String,
    error: Option<JarmErrorResponse>,
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
struct TrancoOverlapResponse {
    overlapping_domains: Vec<TrancoRankedDomain>,
}

#[derive(Serialize)]
struct ShodanHostCountResponse {
    total: u64,
}

pub fn scan_timeout_in_seconds() -> u64 {
    env::var("SCAN_TIMEOUT_IN_SECONDS")
        .unwrap_or(DEFAULT_SCAN_TIMEOUT_IN_SECONDS.to_string())
        .parse::<u64>()
        .expect("Valid timeout value")
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
async fn tranco_overlap(redis_client: Connection<Db>, jarm_hash: String) -> Result<Json<TrancoOverlapResponse>, Custom<Json<ErrorResponse>>> {
    let mut tranco = TrancoTop1M::from(redis_client);
    if !tranco.is_initialized().await {
        return Err(Custom(Status::ServiceUnavailable, Json(ErrorResponse { error: "db not yet loaded".to_string()})))
    }
    let overlapping_domains = tranco.get(jarm_hash).await;
    Ok(Json(TrancoOverlapResponse { overlapping_domains }))
}

#[get("/?<jarm_hash>")]
async fn shodan_host_count(jarm_hash: String) -> Json<ShodanHostCountResponse> {
    let shodan_api_key = env::var("SHODAN_API_KEY").unwrap_or_default();
    let query_param = format!("ssl.jarm:{jarm_hash}");
    let url = Url::parse_with_params(SHODAN_HOST_COUNT_URL,&[
        ("query", query_param),
        ("key", shodan_api_key),
    ]).unwrap();
    let client = reqwest::Client::new();
    let response = client.get(url)
        .header("Accept", "application/json")
        .send().await.unwrap();
    let json_response = response.json::<serde_json::Value>().await.unwrap();
    let total = json_response["total"].as_u64().unwrap();
    Json(ShodanHostCountResponse { total })
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
        error: Some(JarmErrorResponse { error_type, error_message }),
    })
}

pub fn build_rocket_without_tranco_initialisation() -> Rocket<Build> {
    rocket::build()
        .mount("/jarm", routes![jarm])
        .mount("/last-scans", routes![last_scans])
        .mount("/tranco-overlap", routes![tranco_overlap])
        .mount("/shodan-host-count", routes![shodan_host_count])
        .attach(Db::init())
}

pub fn build_rocket() -> Rocket<Build> {
    let rocket = build_rocket_without_tranco_initialisation();
    rocket.attach(AdHoc::try_on_ignite("Initialize tranco", initialize_tranco_in_redis))
}

async fn initialize_tranco_in_redis(rocket: Rocket<Build>) -> fairing::Result {
    let pool = match Db::fetch(&rocket) {
        Some(db) => db.0.clone(),
        None => return Err(rocket)
    };

    rocket::tokio::task::spawn(async move {
        let connection = match pool.get().await {
            Ok(connection) => connection,
            Err(_) => return,
        };
        let mut tranco = TrancoTop1M::new(connection);
        tranco.initialize().await;
    });
    // We don't wait for the initialization to complete.
    // This means it can be stopped unexpectedly and must be able to recover from it on the next run
    Ok(rocket)
}
