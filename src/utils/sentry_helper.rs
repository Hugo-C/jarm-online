use sentry::{Hub, TransactionContext, TransactionOrSpan};

pub enum SentryOp {
    Fingerprinting,
    Redis
}

impl SentryOp {
    fn as_str(&self) -> &'static str {
        match self {
            SentryOp::Fingerprinting => "fingerprinting",
            SentryOp::Redis => "redis",
        }
    }
}

pub fn start_sentry_span(op: SentryOp, description: &str) -> TransactionOrSpan {
    let parent = Hub::current().configure_scope(|scope| scope.get_span());
    match parent {
        Some(parent) => parent.start_child(op.as_str(), description).into(),
        None => {
            let context = TransactionContext::new(description, op.as_str());
            sentry::start_transaction(context).into()
        }
    }
}