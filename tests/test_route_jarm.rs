#[path = "common.rs"]
mod common;

#[cfg(test)]
mod test_route_jarm {
    use rstest::*;
    use rocket::local::blocking::Client;
    use rocket::serde::json::serde_json::json;
    use rocket::serde::json::Value;
    use crate::common::rocket_client;

    const DUMMY_SERVER_JARM_HASH: &str = "21d19d00021d21d00021d19d21d21d1a46380b04d662f0848f508dd171125d";

    #[rstest]
    #[ignore = "Integration tests"]
    fn host_as_ip_only(rocket_client: Client) {
        let expected_response = json!({
            "host": "127.0.0.1",
            "port": "443",
            "jarm_hash": DUMMY_SERVER_JARM_HASH,
            "error": null,
        });

        let response = rocket_client.get("/jarm?host=127.0.0.1").dispatch();

        assert_eq!(response.into_json::<Value>().unwrap(), expected_response);
    }

    #[rstest]
    #[ignore = "Integration tests"]
    fn host_as_domain_only(rocket_client: Client) {
        let expected_response = json!({
            "host": "localhost",
            "port": "443",
            "jarm_hash": DUMMY_SERVER_JARM_HASH,
            "error": null,
        });

        let response = rocket_client.get("/jarm?host=localhost").dispatch();

        assert_eq!(response.into_json::<Value>().unwrap(), expected_response);
    }

    #[rstest]
    #[ignore = "Integration tests"]
    fn host_as_url_only(rocket_client: Client) {
        let expected_response = json!({
            "host": "localhost",
            "port": "443",
            "jarm_hash": DUMMY_SERVER_JARM_HASH,
            "error": null,
        });

        let response = rocket_client.get("/jarm?host=https://localhost").dispatch();

        assert_eq!(response.into_json::<Value>().unwrap(), expected_response);
    }

    #[rstest]
    #[ignore = "Integration tests"]
    fn host_with_port(rocket_client: Client) {
        let expected_response = json!({
            "host": "localhost",
            "port": "443",
            "jarm_hash": DUMMY_SERVER_JARM_HASH,
            "error": null,
        });

        let response = rocket_client.get("/jarm?host=localhost&port=443").dispatch();

        assert_eq!(response.into_json::<Value>().unwrap(), expected_response);
    }


    #[rstest]
    fn invalid_port(rocket_client: Client) {
        let expected_response = r#"{"host":"","port":"","jarm_hash":"","error":{"error_type":"Dns resolve error","error_message":"DetailedError { underlying_error: Some(Error { kind: InvalidInput, message: \"invalid port value\" }) }"}}"#;

        let response = rocket_client.get("/jarm?host=host.fr&port=invalidPort").dispatch();

        assert_eq!(response.into_string(), Some(expected_response.into()));
    }

    #[rstest]
    #[ignore = "Integration tests"]
    fn non_responding_port(rocket_client: Client) {
        let expected_response = r#"{"host":"","port":"","jarm_hash":"","error":{"error_type":"Connection error","error_message":"DetailedError { underlying_error: Some(Os { code: 111, kind: ConnectionRefused, message: \"Connection refused\" }) }"}}"#;

        let response = rocket_client.get("/jarm?host=localhost&port=444").dispatch();

        assert_eq!(response.into_string(), Some(expected_response.into()));
    }
}
