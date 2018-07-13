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

pub mod app;
pub mod db;
pub mod imprudence;
pub mod models;
mod schema;
