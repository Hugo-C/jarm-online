#[cfg(test)]
mod tests_api {
    use jarm_online::set_up_rocket;
    use rocket::local::Client;

    #[test]
    fn invalid_port() {
        let client = Client::new(set_up_rocket()).expect("valid rocket instance");
        let expected_response = r#"{"host":"","port":"","jarm_hash":"","error":{"error_type":"Dns resolve error","error_message":"DetailedError { underlying_error: Some(Error { kind: InvalidInput, message: \"invalid port value\" }) }"}}"#;


        let mut response = client.get("/jarm?host=host.fr&port=invalidPort").dispatch();

        assert_eq!(response.body_string(), Some(expected_response.into()));
    }
}
