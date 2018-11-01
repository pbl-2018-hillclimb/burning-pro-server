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
extern crate r2d2;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate tera;

pub mod admin;
pub mod app;
pub mod db;
pub mod good_phrase;
pub(crate) mod models;
// Temporal silence until diesel-1.4.
// See <https://github.com/diesel-rs/diesel/issues/1785#issuecomment-422579609>.
#[allow(proc_macro_derive_resolution_fallback)]
mod schema;
