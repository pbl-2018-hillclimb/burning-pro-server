//! Burning-pro server.
#![warn(missing_docs)]

extern crate actix;
extern crate actix_web;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate failure;
extern crate futures;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate r2d2;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;

use actix::prelude::*;
use actix_web::{server, App, HttpRequest};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

use db::DbExecutor;

pub mod db;
mod imprudence;
pub mod models;
mod schema;

/// Application-wide states.
pub struct AppState {
    /// Address of DB executor actor.
    pub db: Addr<Syn, DbExecutor>,
}

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

#[allow(unknown_lints, needless_pass_by_value)]
fn fire(req: HttpRequest<AppState>) -> &'static str {
    debug!("request for `fire()`: {:?}", req);
    // Fire.
    "\u{1F525}"
}

fn main() {
    match dotenv::dotenv() {
        Ok(path) => info!("Successfully loaded dotenv file: {}", path.display()),
        Err(e) => {
            if e.not_found() {
                info!("No dotenv file found");
            } else {
                error!("Dotenv initialization failed: {}", e);
                panic!("Dotenv initialization failed: {:?}", e);
            }
        },
    }
    setup_logger();

    info!(
        "{}, version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let listen = "127.0.0.1:8080";

    let sys = actix::System::new("burning-pro-server");

    let database_url = env::var("DATABASE_URL").expect("`DATABASE_URL` envvar must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool");

    let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));

    info!("starting server ({})...", listen);
    server::new(move || {
        App::with_state(AppState { db: addr.clone() })
            .resource("/", |r| r.with(fire))
            .resource("/imprudences/", |r| r.with(imprudence::index))
    }).bind(listen)
        .unwrap_or_else(|e| {
            panic!("Failed to bind {}: {}", listen, e);
        })
        .start();

    info!("started server ({})", listen);

    let _ = sys.run();
}
