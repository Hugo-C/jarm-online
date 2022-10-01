// Modified version of https://github.com/intgr/rocket-sentry (which requires a fixed sentry version with openssl)

use std::sync::Mutex;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;
use sentry::ClientInitGuard;

pub struct RocketSentry {
    guard: Mutex<Option<ClientInitGuard>>,
}

impl RocketSentry {
    pub fn fairing() -> impl Fairing {
        RocketSentry {
            guard: Mutex::new(None),
        }
    }

    fn init(&self, dsn: &str) {
        let guard = sentry::init(dsn);
        if guard.is_enabled() {
            // Tuck the ClientInitGuard in the fairing, so it lives as long as the server.
            let mut self_guard = self.guard.lock().unwrap();
            *self_guard = Some(guard);

            println!("Sentry enabled.");
        } else {
            eprintln!("Sentry did not initialize.");
        }
    }
}

impl Fairing for RocketSentry {
    fn info(&self) -> Info {
        Info {
            name: "rocket-sentry",
            // Kind::Response is necessary, otherwise Rocket dealloc's our SentryGuard reference.
            kind: Kind::Attach | Kind::Response,
        }
    }

    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        match rocket.config().get_str("sentry_dsn") {
            Ok("") => {
                println!("Sentry disabled.");
            }
            Ok(dsn) => {
                self.init(dsn);
            }
            Err(err) => eprintln!("Sentry disabled: {}", err),
        }
        Ok(rocket)
    }
}
