//! DB administration mesasages.

use std::marker::PhantomData;

use actix::prelude::*;
use diesel::prelude::*;

use db::{DbExecutor, Error};
use models;
use schema;

/// A message type to get table rows.
///
/// This may have query parameters to limit or filter results (while it does not
/// for now).
#[derive(Debug, Clone)]
pub struct GetTableRows<T>(PhantomData<T>);

impl<T: 'static> Message for GetTableRows<T> {
    type Result = Result<Vec<T>, Error>;
}

macro_rules! impl_handler_for_model {
    ($schema_name:ident, $model:ty) => {
        impl Handler<GetTableRows<$model>> for DbExecutor {
            type Result = <GetTableRows<$model> as Message>::Result;

            fn handle(
                &mut self,
                _msg: GetTableRows<$model>,
                _ctx: &mut Self::Context,
            ) -> Self::Result {
                let conn = &self.pool().get()?;
                Ok(schema::$schema_name::table.load::<$model>(conn)?)
            }
        }
    };
}

impl_handler_for_model!(good_phrase_tags, models::GoodPhraseTag);
impl_handler_for_model!(good_phrases, models::GoodPhrase);
impl_handler_for_model!(good_phrases_and_tags, models::GoodPhraseAndTag);
impl_handler_for_model!(person_urls, models::PersonUrl);
impl_handler_for_model!(persons, models::Person);
