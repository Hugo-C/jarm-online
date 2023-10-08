#[path = "common.rs"]
mod common;


#[cfg(test)]
mod test_route_jarm {
    use rocket::local::blocking::Client;
    use rstest::*;
    use crate::common::rocket_client;


    #[rstest]
    fn no_overlap(rocket_client: Client) {
        let expected_response = r#"{"overlapping_domains":[]}"#;

        let response = rocket_client.get("/alexa-overlap?jarm_hash=123").dispatch();

        assert_eq!(response.into_string(), Some(expected_response.into()));
    }


    #[rstest]
    fn single_overlap(rocket_client: Client) {
        let expected_response = r#"{"overlapping_domains":[{"rank":9,"domain":"zhihu.com"}]}"#;
        let jarm_hash = "3fd3fd20d3fd3fd21c3fd3fd3fd3fd2b66a312d81ed1efa0f55830f7490cb2";

        let response = rocket_client.get(format!("/alexa-overlap?jarm_hash={jarm_hash}")).dispatch();

        assert_eq!(response.into_string(), Some(expected_response.into()));
    }

    #[rstest]
    fn multiple_overlap(rocket_client: Client) {
        let expected_response = r#"{"overlapping_domains":[{"rank":1,"domain":"google.com"},{"rank":2,"domain":"youtube.com"}]}"#;
        let jarm_hash = "29d3fd00029d29d21c42d43d00041d188e8965256b2536432a9bd447ae607f";

        let response = rocket_client.get(format!("/alexa-overlap?jarm_hash={jarm_hash}")).dispatch();

        assert_eq!(response.into_string(), Some(expected_response.into()));
    }
}
