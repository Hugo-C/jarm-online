use testcontainers_modules::testcontainers::runners::SyncRunner;
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
use testcontainers::{Container, GenericImage};
use testcontainers::core::IntoContainerPort;
use testcontainers_modules::redis::{Redis, REDIS_PORT};
use testcontainers_modules::testcontainers::ImageExt;
use testcontainers::core::WaitFor;

#[allow(dead_code)] // used in tests
pub const DUMMY_SERVER_JARM_HASH: &str =
    "21d19d00021d21d00021d19d21d21d1a46380b04d662f0848f508dd171125d";

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
#[once]
pub fn set_env_var_redis_url(redis_container_url: &String) {
    let redis_url = format!("{{redis_db={{url=\"{redis_container_url}\"}}}}");
    unsafe {env::set_var("ROCKET_DATABASES", redis_url) };
}

#[fixture]
#[allow(unused_variables)]
pub fn rocket_client(set_env_var_top1m_path: (), set_env_var_auth_token: (), set_env_var_redis_url: ()) -> Client {
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
#[once]
pub fn redis_container() -> Container<Redis> {
    Redis::default().with_tag("8-alpine").start().unwrap()
}

#[fixture]
#[once]
pub fn redis_container_url(redis_container: &Container<Redis>) -> String {
    let host_ip = redis_container.get_host().unwrap();
    let host_port = redis_container.get_host_port_ipv4(REDIS_PORT).unwrap();
    format!("redis://{host_ip}:{host_port}")
}

#[fixture]
#[once]
pub fn dummy_server_container() -> Container<GenericImage> {
    let image = GenericImage::new("hugocker/nginx_tls_dummy_server", "latest")
        .with_wait_for(WaitFor::Duration { length: Duration::from_secs(1) });
    let mut container_request = image.with_mapped_port(443, 443.tcp());
    for port in 400..443 {
        container_request = container_request.with_mapped_port(port, port.tcp())
    }
    let container = container_request
        .start()
        .expect("Failed to start dummy server");
    container
}

#[fixture]
pub fn clean_redis<'a>(redis_container: &Container<Redis>) -> MutexGuard<'a, ()> {
    let redis_lock = REDIS_MUTEX.lock().unwrap_or_else(|e| {
        REDIS_MUTEX.clear_poison();
        e.into_inner() // Prevent a failing test to fail the tests that follow
    });


    let host_ip = redis_container.get_host().unwrap();
    let host_port = redis_container.get_host_port_ipv4(REDIS_PORT).unwrap();
    let redis_container_url = format!("redis://{host_ip}:{host_port}");
    let client = redis::Client::open(redis_container_url.clone()).unwrap();
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
