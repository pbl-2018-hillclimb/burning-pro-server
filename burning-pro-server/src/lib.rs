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
extern crate r2d2;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use actix::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

use db::DbExecutor;

pub mod db;
pub mod imprudence;
pub mod models;
mod schema;

/// Application-wide states.
#[derive(Clone)]
pub struct AppState {
    /// Address of DB executor actor.
    pub db: Addr<Syn, DbExecutor>,
}

impl AppState {
    /// Creates a new `AppState` from the given database URL.
    pub fn from_database_url(database_url: String) -> Self {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create connection pool");
        let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));
        Self {
            db: addr,
        }
    }
}
