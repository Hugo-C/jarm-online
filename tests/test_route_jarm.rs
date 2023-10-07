#[path = "common.rs"]
mod common;

#[cfg(test)]
mod test_route_jarm {
    use rstest::*;
    use rocket::local::blocking::Client;
    use crate::common::rocket_client;


    #[rstest]
    fn invalid_port(rocket_client: Client) {
        let expected_response = r#"{"host":"","port":"","jarm_hash":"","error":{"error_type":"Dns resolve error","error_message":"DetailedError { underlying_error: Some(Error { kind: InvalidInput, message: \"invalid port value\" }) }"}}"#;

        let response = rocket_client.get("/jarm?host=host.fr&port=invalidPort").dispatch();

        assert_eq!(response.into_string(), Some(expected_response.into()));
    }
}
