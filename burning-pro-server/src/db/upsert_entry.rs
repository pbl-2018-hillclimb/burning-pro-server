//! Types for upsert query.

// Temporal silence until diesel-1.4.
// See <https://github.com/diesel-rs/diesel/issues/1785#issuecomment-422579609>.
#![allow(proc_macro_derive_resolution_fallback)]

use actix::prelude::*;
use chrono::{DateTime, Local};
use diesel;
use diesel::prelude::*;

use db::{DbExecutor, Error};
use models;
use schema;

no_arg_sql_function!(last_insert_rowid, i32, "Returns last inserted row ID.");

/// A phrase.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GoodPhrase {
    /// Row ID (`None` for new entry)
    pub good_phrase_id: Option<i32>,
    /// Title (short summary).
    pub title: String,
    /// Phrase.
    pub phrase: String,
    /// Author's person id.
    pub person_id: i32,
    /// URL of the phrase if it is posted or published to the WWW.
    pub url: Option<String>,
    /// Whether the source web page is deleted or not.
    pub deleted: bool,
    /// Datetime when the phrase is published.
    pub published_at: Option<DateTime<Local>>,
    /// Tag ids.
    pub tag_ids: Vec<i32>,
}

impl Message for GoodPhrase {
    type Result = Result<(), Error>;
}

impl Handler<GoodPhrase> for DbExecutor {
    type Result = <GoodPhrase as Message>::Result;

    fn handle(&mut self, msg: GoodPhrase, _ctx: &mut Self::Context) -> Self::Result {
        use schema::good_phrases::columns;

        let conn = &self.pool().get()?;
        let now_utc = Local::now().naive_utc();

        let GoodPhrase {
            good_phrase_id,
            title,
            phrase,
            person_id,
            url,
            deleted,
            published_at,
            tag_ids,
        } = msg;

        // Use transaction to get correct `last_insert_rowid` result.
        conn.transaction::<_, Error, _>(|| {
            let good_phrase_id = match good_phrase_id {
                Some(good_phrase_id) => {
                    // Update.
                    diesel::update(
                        schema::good_phrases::table
                            .filter(columns::good_phrase_id.eq(good_phrase_id)),
                    ).set((
                        columns::modified_at.eq(now_utc),
                        columns::title.eq(title),
                        columns::phrase.eq(phrase),
                        columns::person_id.eq(person_id),
                        columns::url.eq(url),
                        columns::deleted.eq(deleted),
                        columns::published_at.eq(published_at.map(|dt| dt.naive_utc())),
                    )).execute(conn)?;
                    good_phrase_id
                }
                None => {
                    // Insert.
                    let published_at_utc = published_at.map(|dt| dt.naive_utc());
                    let new_row = models::NewGoodPhrase {
                        good_phrase_id: None,
                        created_at: &now_utc,
                        modified_at: &now_utc,
                        title: &title,
                        phrase: &phrase,
                        person_id: person_id,
                        url: url.as_ref().map(AsRef::as_ref),
                        deleted: deleted,
                        published_at: published_at_utc.as_ref(),
                    };
                    // NOTE: SQLite backend does not support "returning clause".
                    // See <https://docs.diesel.rs/diesel/backend/trait.SupportsReturningClause.html>.
                    // Although you can retrieve last inserted row ID:
                    // See <https://github.com/diesel-rs/diesel/issues/771>.
                    diesel::insert_into(schema::good_phrases::table)
                        .values(new_row)
                        .execute(conn)?;
                    diesel::select(last_insert_rowid).execute(conn)? as i32
                }
            };

            // Update tags relations.
            let current_ids = schema::good_phrases_and_tags::table
                .filter(schema::good_phrases_and_tags::columns::good_phrase_id.eq(good_phrase_id))
                .select(schema::good_phrases_and_tags::columns::good_phrase_tag_id)
                .load::<i32>(conn)?;
            for delete_id in current_ids.iter().filter(|id| !tag_ids.contains(id)) {
                diesel::delete(
                    schema::good_phrases_and_tags::table
                        .filter(
                            schema::good_phrases_and_tags::columns::good_phrase_id
                                .eq(good_phrase_id),
                        ).filter(
                            schema::good_phrases_and_tags::columns::good_phrase_tag_id
                                .eq(delete_id),
                        ),
                ).execute(conn)?;
            }
            for insert_id in tag_ids.iter().filter(|id| !current_ids.contains(id)) {
                let row = models::NewGoodPhraseAndTag {
                    good_phrase_and_tag_id: None,
                    created_at: &now_utc,
                    modified_at: &now_utc,
                    good_phrase_id,
                    good_phrase_tag_id: *insert_id,
                };
                diesel::insert_into(schema::good_phrases_and_tags::table)
                    .values(row)
                    .execute(conn)?;
            }
            Ok(())
        })
    }
}

/// A phrase request
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GoodPhraseRequest {
    /// Title (short summary)
    pub title: String,
    /// Phrase.
    pub phrase: String,
    /// Author's name.
    pub person: String,
    /// URL of the phrase if it is posted or published to the WWW.
    pub url: Option<String>,
    /// Whether the source web page is deleted or not.
    pub deleted: bool,
    /// Datetime when the phrase is published.
    pub published_at: Option<DateTime<Local>>,
    /// Tags.
    pub tags: Option<String>,
}

impl Message for GoodPhraseRequest {
    type Result = Result<(), Error>;
}

impl Handler<GoodPhraseRequest> for DbExecutor {
    type Result = <GoodPhraseRequest as Message>::Result;

    fn handle(&mut self, msg: GoodPhraseRequest, _ctx: &mut Self::Context) -> Self::Result {
        use schema::good_phrase_requests::columns;

        let conn = &self.pool().get()?;
        let now_utc = Local::now().naive_utc();

        let GoodPhraseRequest {
            title,
            phrase,
            person,
            url,
            deleted,
            published_at,
            tags,
        } = msg;

        let published_at_utc = published_at.map(|dt| dt.naive_utc());

        let new_row = models::NewGoodPhraseRequest {
            good_phrase_request_id: None,
            title: &title,
            phrase: &phrase,
            person: &person,
            url: url.as_ref().map(AsRef::as_ref),
            deleted: deleted,
            published_at: published_at_utc.as_ref(),
            tags: tags.as_ref().map(AsRef::as_ref),
        };

        diesel::insert_into(schema::good_phrase_requests::table)
            .values(new_row)
            .execute(conn)?;

        Ok(())
    }
}

/// A person.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Person {
    /// Row ID (`None` for new entry)
    pub person_id: Option<i32>,
    /// Real name.
    pub real_name: Option<String>,
    /// Display name.
    pub display_name: String,
    /// URLs of web pages of the person.
    pub url: Vec<String>,
    /// Twitter account.
    pub twitter: Option<String>,
}

impl Message for Person {
    type Result = Result<(), Error>;
}

impl Handler<Person> for DbExecutor {
    type Result = <Person as Message>::Result;

    fn handle(&mut self, msg: Person, _ctx: &mut Self::Context) -> Self::Result {
        use schema::persons::columns;

        let conn = &self.pool().get()?;
        let now_utc = Local::now().naive_utc();

        let Person {
            person_id,
            real_name,
            display_name,
            url: urls,
            twitter,
        } = msg;

        // Use transaction to get correct `last_insert_rowid` result.
        conn.transaction::<_, Error, _>(|| {
            let person_id = match person_id {
                Some(person_id) => {
                    // Update.
                    diesel::update(schema::persons::table.filter(columns::person_id.eq(person_id)))
                        .set((
                            columns::modified_at.eq(now_utc),
                            columns::real_name.eq(real_name),
                            columns::display_name.eq(display_name),
                            columns::twitter.eq(twitter),
                        )).execute(conn)?;
                    person_id
                }
                None => {
                    let new_row = models::NewPerson {
                        person_id: None,
                        created_at: &now_utc,
                        modified_at: &now_utc,
                        real_name: real_name.as_ref().map(AsRef::as_ref),
                        display_name: display_name.as_ref(),
                        twitter: twitter.as_ref().map(AsRef::as_ref),
                    };
                    // NOTE: SQLite backend does not support "returning clause".
                    // See <https://docs.diesel.rs/diesel/backend/trait.SupportsReturningClause.html>.
                    // Although you can retrieve last inserted row ID:
                    // See <https://github.com/diesel-rs/diesel/issues/771>.
                    diesel::insert_into(schema::persons::table)
                        .values(new_row)
                        .execute(conn)?;
                    diesel::select(last_insert_rowid).execute(conn)? as i32
                }
            };

            // Update URLs.
            let current_urls = schema::person_urls::table
                .filter(schema::person_urls::columns::person_id.eq(person_id))
                .select((
                    schema::person_urls::columns::person_url_id,
                    schema::person_urls::columns::url,
                )).load::<(i32, String)>(conn)?;

            for (delete_id, _) in current_urls.iter().filter(|(_, url)| !urls.contains(url)) {
                diesel::delete(
                    schema::person_urls::table
                        .filter(schema::person_urls::columns::person_url_id.eq(delete_id)),
                ).execute(conn)?;
            }
            for url in urls
                .iter()
                .filter(|url| !current_urls.iter().any(|(_, v)| v == *url))
            {
                let row = models::NewPersonUrl {
                    person_url_id: None,
                    created_at: &now_utc,
                    modified_at: &now_utc,
                    person_id: person_id,
                    url: url,
                };
                diesel::insert_into(schema::person_urls::table)
                    .values(row)
                    .execute(conn)?;
            }
            Ok(())
        })
    }
}

/// A tag.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GoodPhraseTag {
    /// Row ID (`None` for new entry)
    pub good_phrase_tag_id: Option<i32>,
    /// Name.
    pub name: String,
    /// Description of tag.
    pub description: Option<String>,
}

impl Message for GoodPhraseTag {
    type Result = Result<(), Error>;
}

impl Handler<GoodPhraseTag> for DbExecutor {
    type Result = <GoodPhraseTag as Message>::Result;

    fn handle(&mut self, msg: GoodPhraseTag, _ctx: &mut Self::Context) -> Self::Result {
        use schema::good_phrase_tags::{columns, table};

        let conn = &self.pool().get()?;

        let GoodPhraseTag {
            good_phrase_tag_id,
            name,
            description,
        } = msg;

        // Use transaction to get correct `last_insert_rowid` result.
        match good_phrase_tag_id {
            Some(good_phrase_tag_id) => {
                // Update.
                diesel::update(table.filter(columns::good_phrase_tag_id.eq(good_phrase_tag_id)))
                    .set((columns::name.eq(name), columns::description.eq(description)))
                    .execute(conn)?;
            }
            None => {
                let now_utc = Local::now().naive_utc();
                let new_row = models::NewGoodPhraseTag {
                    good_phrase_tag_id: None,
                    created_at: &now_utc,
                    modified_at: &now_utc,
                    name: &name,
                    description: description.as_ref().map(AsRef::as_ref),
                };
                // NOTE: SQLite backend does not support "returning clause".
                // See <https://docs.diesel.rs/diesel/backend/trait.SupportsReturningClause.html>.
                // Although you can retrieve last inserted row ID:
                // See <https://github.com/diesel-rs/diesel/issues/771>.
                diesel::insert_into(table).values(new_row).execute(conn)?;
            }
        }
        Ok(())
    }
}
