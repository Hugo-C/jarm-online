use std::env;
use std::sync::Arc;
use sentry::TransactionContext;
use ::rocket_sentry::RocketSentry;
use env_logger::Env;
use jarm_online::build_rocket;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let rocket_instance = build_rocket();
    // Get the default configured sample rate from `Rocket.toml`
    let default_rate = rocket_instance
        .figment()
        .extract_inner::<f32>("sentry_traces_sample_rate")
        .unwrap_or(1.);
    let last_scans_sample_rate_default: f32 = 0.;
    let last_scans_sample_rate = match env::var("LAST_SCAN_SAMPLE_RATE"){
        Ok(value) => value.parse::<f32>().unwrap_or(last_scans_sample_rate_default),
        Err(_) => last_scans_sample_rate_default,
    };
    let traces_sampler = move |ctx: &TransactionContext| -> f32 {
        match ctx.name() {
            "GET /last-scans" => {
                if default_rate == 0. {
                    0.  // Allow to disable Sentry completely
                } else {
                    last_scans_sample_rate
                }
            },
            _ => default_rate,
        }
    };
    let rocket_sentry = RocketSentry::builder()
        .traces_sampler(Arc::new(traces_sampler))
        .build();

    rocket_instance
        .attach(CORS)
        .attach(rocket_sentry)
        .launch()
        .await?;
    Ok(())
}
