//! Models.
use chrono::NaiveDateTime;

use schema::*;


/// Imprudence tag.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Serialize, Identifiable, Queryable)]
#[primary_key(imprudence_tag_id)]
pub struct ImprudenceTag {
    /// Row ID.
    pub imprudence_tag_id: i32,
    /// UTC datetime the row is created at.
    pub created_at: NaiveDateTime,
    /// UTC datetime the row is last modified at.
    pub modified_at: NaiveDateTime,
    /// Tag name.
    pub name: String,
    /// Tag description.
    pub description: Option<String>,
}


/// Imprudence.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Serialize, Associations, Identifiable,
         Queryable)]
#[belongs_to(Person)]
#[primary_key(imprudence_id)]
pub struct Imprudence {
    /// Row ID.
    pub imprudence_id: i32,
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


/// Imprudence and tag.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Serialize, Associations, Identifiable,
         Queryable)]
#[belongs_to(Imprudence)]
#[belongs_to(ImprudenceTag)]
#[table_name = "imprudences_and_tags"]
#[primary_key(imprudence_and_tag_id)]
pub struct ImprudenceAndTag {
    /// Row ID.
    pub imprudence_and_tag_id: i32,
    /// UTC datetime the row is created at.
    pub created_at: NaiveDateTime,
    /// UTC datetime the row is last modified at.
    pub modified_at: NaiveDateTime,
    /// Imprudence ID.
    pub imprudence_id: i32,
    /// Imprudence tag ID.
    pub imprudence_tag_id: i32,
}


/// Person and URL.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Serialize, Associations, Identifiable,
         Queryable)]
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Serialize, Identifiable, Queryable)]
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
    pub display_name: Option<String>,
    /// Twitter account (if known).
    pub twitter: Option<String>,
}
