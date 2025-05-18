use rocket::request::{FromRequest, Outcome, Request};
use sentry::Transaction;

pub(crate) struct SentryRequestTransaction<'r>(&'r Option<Transaction>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SentryRequestTransaction<'r> {
    type Error = std::convert::Infallible;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(SentryRequestTransaction())
    }
}
