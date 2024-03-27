#[path = "common.rs"]
mod common;


#[cfg(test)]
mod test_route_tranco_overlap {
    use async_std::task;
    use std::sync::MutexGuard;
    use std::time::Duration;

    use rocket::http;
    use rocket::local::blocking::Client;
    use rocket_db_pools::deadpool_redis::{Config};
    use rstest::*;
    use jarm_online::tranco_top1m::TrancoTop1M;
    use jarm_online::tranco_top1m::RankedDomain;

    use crate::common::{REDIS_URL, rocket_client, rocket_client_without_tranco_init};
    use crate::common::clean_redis;

    #[rstest]
    #[ignore = "Integration tests"]
    fn db_not_yet_loaded(_clean_redis: MutexGuard<'_, ()>, rocket_client_without_tranco_init: Client) {
        let rocket_client = rocket_client_without_tranco_init;

        let expected_response = r#"{"error":"db not yet loaded"}"#;

        let response = rocket_client.get("/tranco-overlap?jarm_hash=123").dispatch();

        assert_eq!(response.status(), http::Status::ServiceUnavailable);
        assert_eq!(response.into_string(), Some(expected_response.into()));
    }

    #[rstest]
    #[ignore = "Integration tests"]
    async fn no_overlap(_clean_redis: MutexGuard<'_, ()>, rocket_client: Client) {
        let expected_response = r#"{"overlapping_domains":[]}"#;

        let uri = "/tranco-overlap?jarm_hash=123";
        let mut response = rocket_client.get(uri).dispatch();

        while response.status() == http::Status::ServiceUnavailable { // Wait for redis to be initialized
            task::sleep(Duration::from_millis(10)).await;
            response = rocket_client.get(uri).dispatch();
        }
        assert_eq!(response.status(), http::Status::Ok);
        assert_eq!(response.into_string(), Some(expected_response.into()));
    }


    #[rstest]
    #[ignore = "Integration tests"]
    async fn single_overlap(_clean_redis: MutexGuard<'_, ()>, rocket_client: Client) {
        let expected_response = r#"{"overlapping_domains":[{"rank":9,"domain":"zhihu.com"}]}"#;
        let jarm_hash = "3fd3fd20d3fd3fd21c3fd3fd3fd3fd2b66a312d81ed1efa0f55830f7490cb2";

        let uri = format!("/tranco-overlap?jarm_hash={jarm_hash}");
        let mut response = rocket_client.get(uri.clone()).dispatch();

        while response.status() == http::Status::ServiceUnavailable { // Wait for redis to be initialized
            task::sleep(Duration::from_millis(10)).await;
            response = rocket_client.get(uri.clone()).dispatch();
        }

        assert_eq!(response.status(), http::Status::Ok);
        assert_eq!(response.into_string(), Some(expected_response.into()));
    }


    #[rstest]
    #[ignore = "Integration tests"]
    async fn multiple_overlap(_clean_redis: MutexGuard<'_, ()>, rocket_client: Client) {
        let expected_response = r#"{"overlapping_domains":[{"rank":11,"domain":"fake_site_1.com"},{"rank":12,"domain":"fake_site_2.com"}]}"#;
        let jarm_hash = "21d19d00021d21d00021d19d21d21d1a46380b04d662f0848f508dd171125d";

        let uri = format!("/tranco-overlap?jarm_hash={jarm_hash}");
        let mut response = rocket_client.get(uri.clone()).dispatch();

        while response.status() == http::Status::ServiceUnavailable { // Wait for redis to be initialized
            task::sleep(Duration::from_millis(10)).await;
            response = rocket_client.get(uri.clone()).dispatch();
        }

        assert_eq!(response.status(), http::Status::Ok);
        assert_eq!(response.into_string(), Some(expected_response.into()));
    }

    #[rstest]
    #[ignore = "Integration tests"]
    async fn clear_tranco_values(_clean_redis: MutexGuard<'_, ()>, _rocket_client: Client) {
        let cfg = Config::from_url(REDIS_URL);
        let pool = cfg.create_pool(None).unwrap();
        let connection = pool.get().await.unwrap();
        let mut tranco = TrancoTop1M::new(connection);

        while tranco.is_initialized().await != true { // Wait for redis to be initialized
            task::sleep(Duration::from_millis(10)).await;
        }
        let jarm_hash = "3fd3fd20d3fd3fd21c3fd3fd3fd3fd2b66a312d81ed1efa0f55830f7490cb2";
        assert_eq!(tranco.get(jarm_hash.to_string()).await, vec![RankedDomain { rank: 9, domain: "zhihu.com".to_string() }]);

        tranco.destroy_db().await.unwrap();  // Clearing values in db means no hash will be found

        assert_eq!(tranco.get(jarm_hash.to_string()).await, vec![]);
        assert_eq!(tranco.is_initialized().await, false);
    }
}
