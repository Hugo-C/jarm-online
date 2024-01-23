use std::env;
use std::path::Path;
use std::sync::{Mutex, MutexGuard};
use lazy_static::lazy_static;
use rstest::*;
use jarm_online::build_rocket;
use rocket::local::blocking::Client;

#[allow(dead_code)]  // used in tests
pub const DUMMY_SERVER_JARM_HASH: &str = "21d19d00021d21d00021d19d21d21d1a46380b04d662f0848f508dd171125d";


lazy_static! {
    static ref REDIS_MUTEX: Mutex<()> = Mutex::default();  // restrict redis parallel access
}


#[fixture]
#[once]
pub fn alexa_top1m_path() -> &'static Path {
    Path::new("tests/fixtures_data/alexa_top1M.csv")
}

#[fixture]
#[once]
pub fn set_env_var_alexa_top1m_path(alexa_top1m_path: &'static Path) -> &'static Path {
    env::set_var("ALEXA_TOP1M_RAW_DATA_PATH", alexa_top1m_path.to_str().expect("valid path"));
    alexa_top1m_path
}

#[fixture]
#[allow(unused_variables)]
pub fn rocket_client(set_env_var_alexa_top1m_path: &'static Path) -> Client {
    let test_rocket = build_rocket();
    Client::tracked(test_rocket).expect("valid rocket instance")
}

#[fixture]
pub fn clean_redis<'a>() -> MutexGuard<'a, ()> {
    let redis_lock = REDIS_MUTEX.lock().unwrap();
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().expect("redis up");
    let _:() = redis::cmd("FLUSHDB").query(&mut con).expect("redis can be cleared before tests");
    return redis_lock;
}