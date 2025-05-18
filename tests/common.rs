use jarm_online::{build_rocket, build_rocket_without_tranco_initialisation};
use lazy_static::lazy_static;
use redis::RedisError;
use rocket::http::Header;
use rocket::local::blocking::Client;
use rocket::warn;
use rstest::*;
use std::path::Path;
use std::sync::{Mutex, MutexGuard};
use std::time::Duration;
use std::{env, thread};

#[allow(dead_code)] // used in tests
pub const DUMMY_SERVER_JARM_HASH: &str =
    "21d19d00021d21d00021d19d21d21d1a46380b04d662f0848f508dd171125d";

pub const REDIS_URL: &str = "redis://127.0.0.1/";
pub const SQLITE_PATH: &str = "sqlite_data/db.sqlite"; // should be the same as the one defined in Rocket.toml

lazy_static! {
    static ref REDIS_MUTEX: Mutex<()> = Mutex::default();  // restrict redis parallel access
    static ref SQLITE_MUTEX: Mutex<()> = Mutex::default();  // restrict sqlite parallel access
}

#[fixture]
#[once]
pub fn tranco_top1m_path() -> &'static Path {
    Path::new("tests/fixtures_data/tranco_top1M.csv")
}

#[fixture]
#[once]
pub fn set_env_var_top1m_path(tranco_top1m_path: &'static Path) {
    unsafe {
        env::set_var(
            "FORCE_TRANCO_TOP1M_RAW_DATA_PATH",
            tranco_top1m_path.to_str().expect("valid path"),
        );
    }
}

#[fixture]
#[once]
pub fn set_env_var_auth_token() {
    unsafe { env::set_var("AUTH_TOKEN", "valid_api_key") };
}

#[fixture]
#[allow(unused_variables)]
pub fn rocket_client(set_env_var_top1m_path: (), set_env_var_auth_token: ()) -> Client {
    let test_rocket = build_rocket();
    Client::tracked(test_rocket).expect("valid rocket instance")
}

#[fixture]
#[allow(unused_variables)]
pub fn rocket_client_without_tranco_init(set_env_var_top1m_path: ()) -> Client {
    let test_rocket = build_rocket_without_tranco_initialisation();
    Client::tracked(test_rocket).expect("valid rocket instance")
}

#[allow(dead_code)] // used in tests
pub fn auth_header() -> Header<'static> {
    Header::new("Authorization", "Token valid_api_key")
}

#[fixture]
pub fn clean_redis<'a>() -> MutexGuard<'a, ()> {
    let redis_lock = REDIS_MUTEX.lock().unwrap_or_else(|e| {
        REDIS_MUTEX.clear_poison();
        e.into_inner() // Prevent a failing test to fail the tests that follow
    });
    let client = redis::Client::open(REDIS_URL).unwrap();
    for i in 1..=10 {
        match clean_redis_commands(&client) {
            Ok(_) => break,
            Err(_) => {
                if i == 10 {
                    panic!("Redis cannot be reached");
                } else {
                    thread::sleep(Duration::from_millis(100 * i));
                }
            }
        }
    }
    redis_lock
}

fn clean_redis_commands(client: &redis::Client) -> Result<(), RedisError> {
    let mut con = client.get_connection()?;
    let _: () = redis::cmd("FLUSHDB").query(&mut con)?;
    Ok(())
}

#[fixture]
pub fn clean_sqlite<'a>() -> MutexGuard<'a, ()> {
    let sqlite_lock = SQLITE_MUTEX.lock().unwrap_or_else(|e| {
        SQLITE_MUTEX.clear_poison();
        e.into_inner() // Prevent a failing test to fail the tests that follow
    });
    match std::fs::remove_file(SQLITE_PATH) {
        Ok(_) => {}
        Err(err) => {
            warn!("Could not delete sqlite db: {:?}", err);  // Warn but skip error
        }
    }
    sqlite_lock
}
