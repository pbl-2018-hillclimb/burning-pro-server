//! Data types insertable to DB.

// Temporal silence until diesel-1.4.
// See <https://github.com/diesel-rs/diesel/issues/1785#issuecomment-422579609>.
#![allow(proc_macro_derive_resolution_fallback)]

use chrono::NaiveDateTime;

use schema::*;

/// GoodPhrase tag.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Serialize, Identifiable, Insertable)]
#[table_name = "good_phrase_tags"]
#[primary_key(good_phrase_tag_id)]
pub struct NewGoodPhraseTag<'a> {
    /// Row ID.
    pub good_phrase_tag_id: Option<i32>,
    /// UTC datetime the row is created at.
    pub created_at: &'a NaiveDateTime,
    /// UTC datetime the row is last modified at.
    pub modified_at: &'a NaiveDateTime,
    /// Tag name.
    pub name: &'a str,
    /// Tag description.
    pub description: Option<&'a str>,
}

/// GoodPhrase.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Serialize, Identifiable, Insertable)]
#[table_name = "good_phrases"]
#[primary_key(good_phrase_id)]
pub struct NewGoodPhrase<'a> {
    /// Row ID.
    pub good_phrase_id: Option<i32>,
    /// UTC datetime the row is created at.
    pub created_at: &'a NaiveDateTime,
    /// UTC datetime the row is last modified at.
    pub modified_at: &'a NaiveDateTime,
    /// Title.
    pub title: &'a str,
    /// Phrase.
    pub phrase: &'a str,
    /// Person ID of the author.
    pub person_id: i32,
    /// URL of the phrase if exists.
    pub url: Option<&'a str>,
    /// Whether the phrase is deleted.
    pub deleted: bool,
    /// UTC datetime the phrase is published at (if known).
    pub published_at: Option<&'a NaiveDateTime>,
}

/// GoodPhraseRequest.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Serialize, Identifiable, Insertable)]
#[table_name = "good_phrase_requests"]
#[primary_key(good_phrase_request_id)]
pub struct NewGoodPhraseRequest<'a> {
    /// Row ID.
    pub good_phrase_request_id: Option<i32>,
    /// Title.
    pub title: &'a str,
    /// Phrase.
    pub phrase: &'a str,
    /// Author.
    pub person: &'a str,
    /// URL of the phrase if exists.
    pub url: Option<&'a str>,
    /// Whether the phrase is deleted.
    pub deleted: bool,
    /// UTC datetime the phrase is published at (if known).
    pub published_at: Option<&'a NaiveDateTime>,
    /// Tags
    pub tags: Option<&'a str>,
}

/// GoodPhrase and tag.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Serialize, Identifiable, Insertable)]
#[table_name = "good_phrases_and_tags"]
#[primary_key(good_phrase_and_tag_id)]
pub struct NewGoodPhraseAndTag<'a> {
    /// Row ID.
    pub good_phrase_and_tag_id: Option<i32>,
    /// UTC datetime the row is created at.
    pub created_at: &'a NaiveDateTime,
    /// UTC datetime the row is last modified at.
    pub modified_at: &'a NaiveDateTime,
    /// GoodPhrase ID.
    pub good_phrase_id: i32,
    /// GoodPhrase tag ID.
    pub good_phrase_tag_id: i32,
}

/// Person and URL.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Serialize, Identifiable, Queryable, Insertable,
)]
#[table_name = "person_urls"]
#[primary_key(person_url_id)]
pub struct NewPersonUrl<'a> {
    /// Row ID.
    pub person_url_id: Option<i32>,
    /// UTC datetime the row is created at.
    pub created_at: &'a NaiveDateTime,
    /// UTC datetime the row is last modified at.
    pub modified_at: &'a NaiveDateTime,
    /// Person ID.
    pub person_id: i32,
    /// URL.
    pub url: &'a str,
}

/// Person.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Serialize, Identifiable, Queryable, Insertable,
)]
#[table_name = "persons"]
#[primary_key(person_id)]
pub struct NewPerson<'a> {
    /// Row ID.
    pub person_id: Option<i32>,
    /// UTC datetime the row is created at.
    pub created_at: &'a NaiveDateTime,
    /// UTC datetime the row is last modified at.
    pub modified_at: &'a NaiveDateTime,
    /// Real name.
    pub real_name: Option<&'a str>,
    /// Display name.
    ///
    /// This can be unofficial name.
    pub display_name: &'a str,
    /// Twitter account (if known).
    pub twitter: Option<&'a str>,
}
