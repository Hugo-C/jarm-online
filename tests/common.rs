use std::{env, thread};
use std::path::Path;
use std::sync::{Mutex, MutexGuard};
use std::time::Duration;
use lazy_static::lazy_static;
use redis::RedisError;
use rstest::*;
use jarm_online::{build_rocket, build_rocket_without_tranco_initialisation};
use rocket::local::blocking::Client;

#[allow(dead_code)]  // used in tests
pub const DUMMY_SERVER_JARM_HASH: &str = "21d19d00021d21d00021d19d21d21d1a46380b04d662f0848f508dd171125d";

pub const REDIS_URL: &str = "redis://127.0.0.1/";


lazy_static! {
    static ref REDIS_MUTEX: Mutex<()> = Mutex::default();  // restrict redis parallel access
}

#[fixture]
#[once]
pub fn tranco_top1m_path() -> &'static Path {
    Path::new("tests/fixtures_data/tranco_top1M.csv")
}

#[fixture]
#[once]
pub fn set_env_var_top1m_path(tranco_top1m_path: &'static Path) {
    env::set_var("FORCE_TRANCO_TOP1M_RAW_DATA_PATH", tranco_top1m_path.to_str().expect("valid path"));
}

#[fixture]
#[allow(unused_variables)]
pub fn rocket_client(set_env_var_top1m_path: ()) -> Client {
    let test_rocket = build_rocket();
    Client::tracked(test_rocket).expect("valid rocket instance")
}

#[fixture]
#[allow(unused_variables)]
pub fn rocket_client_without_tranco_init(set_env_var_top1m_path: ()) -> Client {
    let test_rocket = build_rocket_without_tranco_initialisation();
    Client::tracked(test_rocket).expect("valid rocket instance")
}

#[fixture]
pub fn clean_redis<'a>() -> MutexGuard<'a, ()> {
    let redis_lock = REDIS_MUTEX.lock().unwrap_or_else(|e| {
        REDIS_MUTEX.clear_poison();
        e.into_inner()  // Prevent a failing test to fail the tests that follow
    });
    let client = redis::Client::open(REDIS_URL).unwrap();
    for i in 1..=10 {
        match clean_redis_commands(&client)   {
            Ok(_) => break,
            Err(_) => {
                if i == 10 {
                    panic!("Redis cannot be reached");
                } else {
                    thread::sleep(Duration::from_millis(100 * i));
                }
            },
        }
    }
    return redis_lock;
}

fn clean_redis_commands(client: &redis::Client) -> Result<(), RedisError> {
    let mut con = client.get_connection()?;
    let _:() = redis::cmd("FLUSHDB").query(&mut con)?;
    Ok(())
}