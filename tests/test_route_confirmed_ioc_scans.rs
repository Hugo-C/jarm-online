#[path = "common.rs"]
mod common;

#[cfg(test)]
mod test_route_confirmed_ioc_scans {
    use crate::common::clean_sqlite;
    use crate::common::rocket_client;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::serde::json::serde_json::{json, Map};
    use rocket::serde::json::Value;
    use rocket::serde::json::Value::Null;
    use rstest::*;
    use std::sync::MutexGuard;

    #[rstest]
    fn add_confirmed_ioc_scans_returns_http_201(
        _clean_sqlite: MutexGuard<'_, ()>,
        rocket_client: Client,
    ) {
        let body = json!({
            "host": "some.host.com",
            "port": "443",
            "jarm_hash": "abcabc",
            "scan_timestamp": 1732661308,
            "threat_fox_first_seen": 1700061308,
            "threat_fox_confidence_level": 25,
            "threat_fox_malware": "cobalt",
        });

        let response = rocket_client
            .post("/confirmed-ioc-scans")
            .json(&body)
            .dispatch();

        assert_eq!(response.status(), Status::Created);
    }

    #[rstest]
    fn get_confirmed_ioc_scans_returns_previously_added_single_value(
        _clean_sqlite: MutexGuard<'_, ()>,
        rocket_client: Client,
    ) {
        let confirmed_ioc_scan = json!({
            "host": "some.host.com",
            "port": "443",
            "jarm_hash": "abcabc",
            "scan_timestamp": 1732661308,
            "threat_fox_first_seen": 1700061308,
            "threat_fox_confidence_level": 25,
            "threat_fox_malware": "cobalt",
        });

        rocket_client
            .post("/confirmed-ioc-scans")
            .json(&confirmed_ioc_scan)
            .dispatch();

        let response = rocket_client.get("/confirmed-ioc-scans").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let received_response = response.into_json::<Map<String, Value>>().unwrap();
        let next = received_response.get("next").unwrap();
        assert_eq!(next, &Null);
        let last_scans = received_response
            .get("results")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(last_scans.len(), 1);
        let mut received_confirmed_ioc_scan =
            last_scans.get(0).unwrap().as_object().unwrap().clone();
        let id = received_confirmed_ioc_scan.get("id");
        assert!(id.is_some());
        received_confirmed_ioc_scan.remove("id");
        let received_without_id = Value::Object(received_confirmed_ioc_scan);
        assert_eq!(received_without_id, confirmed_ioc_scan);
    }

    #[rstest]
    fn get_confirmed_ioc_scans_returns_previously_added_multiple_values(
        _clean_sqlite: MutexGuard<'_, ()>,
        rocket_client: Client,
    ) {
        let confirmed_ioc_scan1 = json!({
            "host": "some.host1.com",
            "port": "443",
            "jarm_hash": "abcabc",
            "scan_timestamp": 1732661308,
            "threat_fox_first_seen": 1700061308,
            "threat_fox_confidence_level": 25,
            "threat_fox_malware": "cobalt",
        });

        rocket_client
            .post("/confirmed-ioc-scans")
            .json(&confirmed_ioc_scan1)
            .dispatch();

        let confirmed_ioc_scan2 = json!({
            "host": "some.host2.com",
            "port": "443",
            "jarm_hash": "abcabc",
            "scan_timestamp": 1732661308,
            "threat_fox_first_seen": 1700061308,
            "threat_fox_confidence_level": 25,
            "threat_fox_malware": "cobalt",
        });

        rocket_client
            .post("/confirmed-ioc-scans")
            .json(&confirmed_ioc_scan2)
            .dispatch();

        let response = rocket_client.get("/confirmed-ioc-scans").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let received_response = response.into_json::<Map<String, Value>>().unwrap();
        let next = received_response.get("next").unwrap();
        assert_eq!(next, &Null);
        let last_scans = received_response
            .get("results")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(last_scans.len(), 2);
    }
}
