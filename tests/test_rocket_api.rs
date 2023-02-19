#[cfg(test)]
mod tests_api {
    use rocket::local::Client;
    use rocket::http::Status;
    use jarm_online::set_up_rocket;

    #[test]
    fn hello_world() {
        let client = Client::new(set_up_rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
}