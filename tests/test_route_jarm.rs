#[path = "common.rs"]
mod common;

#[cfg(test)]
mod test_route_jarm {
    use std::path::Path;
    use rstest::*;
    use jarm_online::set_up_rocket;
    use rocket::local::Client;
        use crate::common::set_env_var_alexa_top1m_path;


    #[rstest]
    fn invalid_port(set_env_var_alexa_top1m_path: &Path) {
        let client = Client::new(set_up_rocket()).expect("valid rocket instance");
        let expected_response = r#"{"host":"","port":"","jarm_hash":"","error":{"error_type":"Dns resolve error","error_message":"DetailedError { underlying_error: Some(Error { kind: InvalidInput, message: \"invalid port value\" }) }"}}"#;


        let mut response = client.get("/jarm?host=host.fr&port=invalidPort").dispatch();

        assert_eq!(response.body_string(), Some(expected_response.into()));
    }
}
