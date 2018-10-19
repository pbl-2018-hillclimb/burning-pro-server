//! DB mesasages to get rows of a specific type.

use actix::prelude::*;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::query_source::Table;

use db::{DbExecutor, Error};
use models;
use schema;

/// A trait for table rows query type.
pub trait RowQuery {
    /// DB model type to be queried as a row.
    type Row: HasTable;

    /// Returns a table object.
    fn table() -> <<Self as RowQuery>::Row as HasTable>::Table {
        <Self::Row as HasTable>::table()
    }

    /// Returns all columns object.
    fn all_columns() -> <<<Self as RowQuery>::Row as HasTable>::Table as Table>::AllColumns {
        <<Self as RowQuery>::Row as HasTable>::Table::all_columns()
    }

    /// Returns primary key column object.
    fn primary_key() -> <<<Self as RowQuery>::Row as HasTable>::Table as Table>::PrimaryKey {
        Self::table().primary_key()
    }
}

/// Query type for [`GoodPhraseTag`][`models::GoodPhraseTag`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GoodPhraseTagQuery {
    /// Query all rows.
    All,
    /// Query rows for the given phrase id.
    PhraseId(i32),
}

impl RowQuery for GoodPhraseTagQuery {
    type Row = models::GoodPhraseTag;
}

impl Message for GoodPhraseTagQuery {
    type Result = Result<Vec<<GoodPhraseTagQuery as RowQuery>::Row>, Error>;
}

impl Handler<GoodPhraseTagQuery> for DbExecutor {
    type Result = <GoodPhraseTagQuery as Message>::Result;

    fn handle(&mut self, msg: GoodPhraseTagQuery, _ctx: &mut Self::Context) -> Self::Result {
        let conn = &self.pool().get()?;
        let res = match msg {
            GoodPhraseTagQuery::All => {
                GoodPhraseTagQuery::table().load::<models::GoodPhraseTag>(conn)?
            }
            GoodPhraseTagQuery::PhraseId(phrase_id) => schema::good_phrases_and_tags::table
                .filter(schema::good_phrases_and_tags::columns::good_phrase_id.eq(phrase_id))
                .inner_join(GoodPhraseTagQuery::table())
                .select(GoodPhraseTagQuery::all_columns())
                .load::<models::GoodPhraseTag>(conn)?,
        };
        Ok(res)
    }
}

/// Query type for [`GoodPhrase`][`models::GoodPhrase`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GoodPhraseQuery {
    /// Query all rows.
    All,
    /// Query rows for the given phrase id.
    PhraseId(i32),
}

impl RowQuery for GoodPhraseQuery {
    type Row = models::GoodPhrase;
}

impl Message for GoodPhraseQuery {
    type Result = Result<Vec<<GoodPhraseQuery as RowQuery>::Row>, Error>;
}

impl Handler<GoodPhraseQuery> for DbExecutor {
    type Result = <GoodPhraseQuery as Message>::Result;

    fn handle(&mut self, msg: GoodPhraseQuery, _ctx: &mut Self::Context) -> Self::Result {
        let conn = &self.pool().get()?;
        let res = match msg {
            GoodPhraseQuery::All => GoodPhraseQuery::table().load::<models::GoodPhrase>(conn)?,
            GoodPhraseQuery::PhraseId(phrase_id) => {
                vec![GoodPhraseQuery::table().find(phrase_id).first(conn)?]
            }
        };
        Ok(res)
    }
}

/// Query type for [`PersonUrl`][`models::PersonUrl`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PersonUrlQuery {
    /// Query all rows.
    All,
    /// Query rows for the given person id.
    PersonId(i32),
}

impl RowQuery for PersonUrlQuery {
    type Row = models::PersonUrl;
}

impl Message for PersonUrlQuery {
    type Result = Result<Vec<<PersonUrlQuery as RowQuery>::Row>, Error>;
}

impl Handler<PersonUrlQuery> for DbExecutor {
    type Result = <PersonUrlQuery as Message>::Result;

    fn handle(&mut self, msg: PersonUrlQuery, _ctx: &mut Self::Context) -> Self::Result {
        let conn = &self.pool().get()?;
        let res = match msg {
            PersonUrlQuery::All => PersonUrlQuery::table().load::<models::PersonUrl>(conn)?,
            PersonUrlQuery::PersonId(person_id) => PersonUrlQuery::table()
                .filter(schema::person_urls::columns::person_id.eq(person_id))
                .load::<models::PersonUrl>(conn)?,
        };
        Ok(res)
    }
}

/// Query type for [`Person`][`models::Person`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PersonQuery {
    /// Query all rows.
    All,
    /// Query rows for the given person id.
    PersonId(i32),
}

impl RowQuery for PersonQuery {
    type Row = models::Person;
}

impl Message for PersonQuery {
    type Result = Result<Vec<<PersonQuery as RowQuery>::Row>, Error>;
}

impl Handler<PersonQuery> for DbExecutor {
    type Result = <PersonQuery as Message>::Result;

    fn handle(&mut self, msg: PersonQuery, _ctx: &mut Self::Context) -> Self::Result {
        let conn = &self.pool().get()?;
        let res = match msg {
            PersonQuery::All => PersonQuery::table().load::<models::Person>(conn)?,
            PersonQuery::PersonId(person_id) => {
                vec![PersonQuery::table().find(person_id).first(conn)?]
            }
        };
        Ok(res)
    }
}
