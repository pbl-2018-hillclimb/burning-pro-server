//! Burning-pro server.
#![warn(missing_docs)]

extern crate actix_web;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::env;

use actix_web::{server, App, HttpRequest};


/// Setup global logger.
fn setup_logger() {
    const DEFAULT_LOG_LEVEL: &str = "trace";
    let underscored_name = env!("CARGO_PKG_NAME").replace('-', "_");
    let defval = format!("{}={}", underscored_name, DEFAULT_LOG_LEVEL);

    let newval = match env::var("RUST_LOG") {
        Ok(v) => format!("{},{}", defval, v),
        Err(_) => defval,
    };
    env::set_var("RUST_LOG", &newval);

    pretty_env_logger::init();

    trace!("RUST_LOG={}", newval);
}


fn fire(req: HttpRequest) -> &'static str {
    debug!("request for `fire()`: {:?}", req);
    // Fire.
    "\u{1F525}"
}


fn main() {
    setup_logger();

    info!(
        "{}, version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    info!("starting server...");
    server::new(|| App::new().resource("/", |r| r.f(fire)))
        .bind("127.0.0.1:8080")
        .expect("Failed to bind 127.0.0.1:8080")
        .run();
}
