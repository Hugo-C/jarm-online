#[path = "common.rs"]
mod common;

#[cfg(test)]
mod test_route_last_scans {
    use std::sync::MutexGuard;
    use rstest::*;
    use rocket::local::blocking::Client;
    use rocket::serde::json::serde_json::json;
    use rocket::serde::json::Value;
    use crate::common::{clean_redis, rocket_client};
    use crate::common::DUMMY_SERVER_JARM_HASH;

    #[rstest]
    #[ignore = "Integration tests"]
    fn no_last_scans(_clean_redis: MutexGuard<'_, ()>, rocket_client: Client) {
        let expected_response = json!({
            "last_scans": [],
        });

        let response = rocket_client.get("/last-scans").dispatch();

        assert_eq!(response.into_json::<Value>().unwrap(), expected_response);
    }

    #[rstest]
    #[ignore = "Integration tests"]
    fn a_single_last_scan(_clean_redis: MutexGuard<'_, ()>, rocket_client: Client) {
        let expected_response = json!({
            "last_scans": [{
                "host": "localhost",
                "port": "443",
                "jarm_hash": DUMMY_SERVER_JARM_HASH,
            }],
        });
        rocket_client.get("/jarm?host=localhost").dispatch();

        let response = rocket_client.get("/last-scans").dispatch();

        assert_eq!(response.into_json::<Value>().unwrap(), expected_response);
    }
}
