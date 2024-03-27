#[path = "common.rs"]
mod common;


#[cfg(test)]
mod test_route_tranco_overlap {
    use rocket::http;
    use rocket::local::blocking::Client;
    use rstest::*;
    use crate::common::rocket_client;


    #[rstest]
    fn db_not_yet_loaded(rocket_client: Client) {
        let expected_response = r#"{"error":"db not yet loaded"}"#;

        let response = rocket_client.get("/tranco-overlap?jarm_hash=123").dispatch();

        assert_eq!(response.status(), http::Status::ServiceUnavailable);
        assert_eq!(response.into_string(), Some(expected_response.into()));
    }
}
