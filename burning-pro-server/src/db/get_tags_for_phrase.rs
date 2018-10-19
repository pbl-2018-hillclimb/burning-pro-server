//! `GetTagsForPhrase` message.

use actix::prelude::*;
use chrono::{DateTime, FixedOffset};
use diesel::prelude::*;

use db::{DbExecutor, Error};
use models;
use schema;

/// A message type to get tags for a good_phrase.
#[derive(Debug, Clone)]
pub struct GetTagsForGoodPhrase {
    good_phrase_id: i32,
}

impl GetTagsForGoodPhrase {
    /// Creates a new `GetTagsForGoodPhrase` from the given good phrase ID.
    pub fn new(good_phrase_id: i32) -> Self {
        Self { good_phrase_id }
    }
}

impl Message for GetTagsForGoodPhrase {
    type Result = Result<Vec<models::GoodPhraseTag>, Error>;
}

impl Handler<GetTagsForGoodPhrase> for DbExecutor {
    type Result = <GetTagsForGoodPhrase as Message>::Result;

    fn handle(&mut self, msg: GetTagsForGoodPhrase, _ctx: &mut Self::Context) -> Self::Result {
        let conn = &self.pool().get()?;
        let tags = schema::good_phrases_and_tags::table
            .filter(schema::good_phrases_and_tags::columns::good_phrase_id.eq(msg.good_phrase_id))
            .inner_join(schema::good_phrase_tags::table)
            .select(schema::good_phrase_tags::all_columns)
            .load::<models::GoodPhraseTag>(conn)?;
        Ok(tags)
    }
}
