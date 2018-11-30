//! Models.

// Temporal silence until diesel-1.4.
// See <https://github.com/diesel-rs/diesel/issues/1785#issuecomment-422579609>.
#![allow(proc_macro_derive_resolution_fallback)]

use chrono::NaiveDateTime;

use schema::*;

pub use self::update::*;

pub mod update;

/// GoodPhrase tag.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Serialize, Identifiable, Queryable, Insertable,
)]
#[primary_key(good_phrase_tag_id)]
pub struct GoodPhraseTag {
    /// Row ID.
    pub good_phrase_tag_id: i32,
    /// UTC datetime the row is created at.
    pub created_at: NaiveDateTime,
    /// UTC datetime the row is last modified at.
    pub modified_at: NaiveDateTime,
    /// Tag name.
    pub name: String,
    /// Tag description.
    pub description: Option<String>,
}

/// GoodPhrase.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Hash,
    Serialize,
    Associations,
    Identifiable,
    Queryable,
    Insertable,
)]
#[belongs_to(Person)]
#[primary_key(good_phrase_id)]
pub struct GoodPhrase {
    /// Row ID.
    pub good_phrase_id: i32,
    /// UTC datetime the row is created at.
    pub created_at: NaiveDateTime,
    /// UTC datetime the row is last modified at.
    pub modified_at: NaiveDateTime,
    /// Title.
    pub title: String,
    /// Phrase.
    pub phrase: String,
    /// Person ID of the author.
    pub person_id: i32,
    /// URL of the phrase if exists.
    pub url: Option<String>,
    /// Whether the phrase is deleted.
    pub deleted: bool,
    /// UTC datetime the phrase is published at (if known).
    pub published_at: Option<NaiveDateTime>,
}

/// GoodPhrase and tag.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Hash,
    Serialize,
    Associations,
    Identifiable,
    Queryable,
    Insertable,
)]
#[belongs_to(GoodPhrase)]
#[belongs_to(GoodPhraseTag)]
#[table_name = "good_phrases_and_tags"]
#[primary_key(good_phrase_and_tag_id)]
pub struct GoodPhraseAndTag {
    /// Row ID.
    pub good_phrase_and_tag_id: i32,
    /// UTC datetime the row is created at.
    pub created_at: NaiveDateTime,
    /// UTC datetime the row is last modified at.
    pub modified_at: NaiveDateTime,
    /// GoodPhrase ID.
    pub good_phrase_id: i32,
    /// GoodPhrase tag ID.
    pub good_phrase_tag_id: i32,
}

/// Person and URL.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Hash,
    Serialize,
    Associations,
    Identifiable,
    Queryable,
    Insertable,
)]
#[belongs_to(Person)]
#[table_name = "person_urls"]
#[primary_key(person_url_id)]
pub struct PersonUrl {
    /// Row ID.
    pub person_url_id: i32,
    /// UTC datetime the row is created at.
    pub created_at: NaiveDateTime,
    /// UTC datetime the row is last modified at.
    pub modified_at: NaiveDateTime,
    /// Person ID.
    pub person_id: i32,
    /// URL.
    pub url: String,
}

/// Person.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Serialize, Identifiable, Queryable, Insertable,
)]
#[primary_key(person_id)]
pub struct Person {
    /// Row ID.
    pub person_id: i32,
    /// UTC datetime the row is created at.
    pub created_at: NaiveDateTime,
    /// UTC datetime the row is last modified at.
    pub modified_at: NaiveDateTime,
    /// Real name.
    pub real_name: Option<String>,
    /// Display name.
    ///
    /// This can be unofficial name.
    pub display_name: String,
    /// Twitter account (if known).
    pub twitter: Option<String>,
}

/// GoodPhraseRequest.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Hash,
    Serialize,
    Associations,
    Identifiable,
    Queryable,
    Insertable,
)]
#[primary_key(good_phrase_request_id)]
pub struct GoodPhraseRequest {
    /// Row ID.
    pub good_phrase_request_id: i32,
    /// Phrase.
    pub phrase: String,
    /// Author.
    pub person: String,
    /// URL of the phrase if exists.
    pub url: Option<String>,
    /// Whether the phrase is deleted.
    pub deleted: bool,
    /// UTC datetime the phrase is published at (if known).
    pub published_at: Option<NaiveDateTime>,
}
