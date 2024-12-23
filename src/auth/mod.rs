use constant_time_eq::constant_time_eq;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use std::env;

#[allow(dead_code)] // not used in views for now
pub(crate) struct AuthToken<'r>(&'r str);

#[derive(Debug)]
pub(crate) enum AuthTokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthToken<'r> {
    type Error = AuthTokenError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if token is valid
        fn is_valid(token: &str) -> bool {
            let raw_token = env::var("AUTH_TOKEN").expect("AUTH_TOKEN is required to be set");
            let expected_token = format!("Token {raw_token}");
            constant_time_eq(token.as_bytes(), expected_token.as_bytes())
        }

        match req.headers().get_one("Authorization") {
            None => Outcome::Error((Status::Unauthorized, AuthTokenError::Missing)),
            Some(token) if is_valid(token) => Outcome::Success(AuthToken(token)),
            Some(_) => Outcome::Error((Status::Unauthorized, AuthTokenError::Invalid)),
        }
    }
}
