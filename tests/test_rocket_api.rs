#[cfg(test)]
mod tests_api {
    use std::env;
    use std::sync::Mutex;
    use lazy_static::lazy_static;
    use jarm_online::{DEFAULT_SCAN_TIMEOUT_IN_SECONDS, scan_timeout_in_seconds, set_up_rocket};
    use rocket::local::Client;

    lazy_static! {
        static ref ENV_VAR_MUTEX: Mutex<()> = Mutex::default();  // env variables are a shared resource
    }

    #[test]
    fn scan_timeout_in_seconds_is_correct_default() {
        let _mutex = ENV_VAR_MUTEX.lock().unwrap();  // take the mutex and release it at the end of the function
        env::remove_var("SCAN_TIMEOUT_IN_SECONDS");

        let timeout = scan_timeout_in_seconds();

        assert_eq!(timeout, DEFAULT_SCAN_TIMEOUT_IN_SECONDS);
    }

    #[test]
    fn scan_timeout_in_seconds_can_be_changed() {
        let _mutex = ENV_VAR_MUTEX.lock().unwrap();
        env::set_var("SCAN_TIMEOUT_IN_SECONDS", "168");

        let timeout = scan_timeout_in_seconds();

        assert_eq!(timeout, 168);
    }

    #[test]
    #[ignore = "Crash other tests ran in parallel"]
    fn scan_timeout_in_seconds_fail_on_invalid_values() {
        let _mutex = ENV_VAR_MUTEX.lock().unwrap();
        env::set_var("SCAN_TIMEOUT_IN_SECONDS", "-1");

        let result = std::panic::catch_unwind(|| scan_timeout_in_seconds());
        assert!(result.is_err());
        env::remove_var("SCAN_TIMEOUT_IN_SECONDS");  // cleanup the env var
    }

    #[test]
    fn invalid_port() {
        let client = Client::new(set_up_rocket()).expect("valid rocket instance");
        let expected_response = r#"{"host":"","port":"","jarm_hash":"","error":{"error_type":"Dns resolve error","error_message":"DetailedError { underlying_error: Some(Error { kind: InvalidInput, message: \"invalid port value\" }) }"}}"#;


        let mut response = client.get("/jarm?host=host.fr&port=invalidPort").dispatch();

        assert_eq!(response.body_string(), Some(expected_response.into()));
    }
}
