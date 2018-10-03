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

pub mod app;
pub mod db;
pub mod good_phrase;
pub mod admin;
pub(crate) mod models;
mod schema;
