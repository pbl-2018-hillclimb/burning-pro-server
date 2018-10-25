//! DB administration mesasages.

use std::iter;
use std::marker::PhantomData;

use actix::prelude::*;
use diesel;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;

use db::{DbExecutor, Error};
use models;

/// A message type to get table rows.
///
/// This may have query parameters to limit or filter results (while it does not
/// for now).
///
/// NOTE: Deprecated. Use `db::*Query` types instead.
#[derive(Debug, Clone)]
pub struct GetTableRows<T>(PhantomData<T>);

impl<T: 'static> Message for GetTableRows<T> {
    type Result = Result<Vec<T>, Error>;
}

/// A message type to upsert rows to DB table.
#[derive(Debug, Clone)]
pub struct UpsertTableRows<T> {
    rows: Vec<T>,
}

impl<T> UpsertTableRows<T> {
    /// Creates a new `UpsertTableRows`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `UpsertTableRows` from the given row.
    pub fn from_row(row: T) -> Self {
        Self { rows: vec![row] }
    }

    /// Creates a new `UpsertTableRows` from the given rows.
    pub fn from_rows(rows: Vec<T>) -> Self {
        Self { rows }
    }
}

impl<T> Default for UpsertTableRows<T> {
    fn default() -> Self {
        Self {
            rows: Default::default(),
        }
    }
}

impl<T> iter::Extend<T> for UpsertTableRows<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.rows.extend(iter)
    }
}

impl<T: 'static> Message for UpsertTableRows<T> {
    type Result = Result<usize, Error>;
}

macro_rules! impl_handler_for_model {
    ($model:ty) => {
        impl Handler<GetTableRows<$model>> for DbExecutor {
            type Result = <GetTableRows<$model> as Message>::Result;

            fn handle(
                &mut self,
                _msg: GetTableRows<$model>,
                _ctx: &mut Self::Context,
            ) -> Self::Result {
                use diesel::associations::HasTable;

                let conn = &self.pool().get()?;
                Ok(<$model>::table().load::<$model>(conn)?)
            }
        }

        impl Handler<UpsertTableRows<$model>> for DbExecutor {
            type Result = <UpsertTableRows<$model> as Message>::Result;

            fn handle(
                &mut self,
                msg: UpsertTableRows<$model>,
                _ctx: &mut Self::Context,
            ) -> Self::Result {
                use diesel::associations::HasTable;

                let conn = &self.pool().get()?;

                // NOTE: SQLite backend does not support batch insert.
                // See <https://github.com/diesel-rs/diesel/issues/1822>.
                let result = msg
                    .rows
                    .iter()
                    .map(|row| {
                        // NOTE: SQLite backend does not support upsert.
                        // See <https://github.com/diesel-rs/diesel/issues/1854>.
                        // Use `replace_into` to do upsert with SQLite backend.
                        diesel::replace_into(<$model>::table())
                            .values(row)
                            .execute(conn)
                            .map(|_| ())
                    }).collect::<Result<Vec<_>, _>>()?;
                Ok(result.len())
            }
        }
    };
}

impl_handler_for_model!(models::GoodPhraseTag);
impl_handler_for_model!(models::GoodPhrase);
impl_handler_for_model!(models::GoodPhraseAndTag);
impl_handler_for_model!(models::PersonUrl);
impl_handler_for_model!(models::Person);
