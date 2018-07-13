//! DB executor actor.

use actix::prelude::*;
use actix_web::error::ResponseError;
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use r2d2;

pub use self::get_imprudences::GetImprudences;

mod get_imprudences;

/// DB operation error.
#[derive(Debug, Fail)]
pub enum Error {
    /// Connection pool error.
    #[fail(display = "Connection pool error: {}", _0)]
    ConnectionPool(r2d2::Error),
    /// Diesel query error.
    #[fail(display = "DB query error: {}", _0)]
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

/// DB executor.
pub struct DbExecutor(pub Pool<ConnectionManager<SqliteConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
