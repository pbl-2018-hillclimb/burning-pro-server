//! DB executor actor.

use actix::prelude::*;
use actix_web::error::ResponseError;
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use r2d2;

pub use self::get_good_phrases::GetGoodPhrases;

pub mod admin;
mod get_good_phrases;

/// DB operation error.
#[derive(Debug, Fail)]
pub enum Error {
    /// Connection pool error.
    #[fail(display = "Connection pool error: {}", _0)]
    ConnectionPool(r2d2::Error),
    /// Diesel operation error.
    #[fail(display = "DB operation error: {}", _0)]
    Db(diesel::result::Error),
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Error::ConnectionPool(e)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Error::Db(e)
    }
}

impl ResponseError for Error {}

/// DB operation executor.
///
/// Objects of this type are actors, which receives DB operation request
/// messages and return `future`s (handles of async calculation result) of DB
/// operation result.
pub struct DbExecutor {
    /// Connection pool.
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl DbExecutor {
    /// Creates a new `DbExecutor` from the given connection pool.
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        Self { pool }
    }

    /// Returns a connection pool.
    pub fn pool(&self) -> &Pool<ConnectionManager<SqliteConnection>> {
        &self.pool
    }
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
