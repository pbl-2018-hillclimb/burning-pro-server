//! Types for upsert query.

use chrono::{DateTime, Local};

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

/// A person.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Person {
    /// Row ID (`None` for new entry)
    pub person_id: Option<i32>,
    /// Real name.
    pub real_name: Option<String>,
    /// Display name.
    pub display_name: Option<String>,
    /// URLs of web pages of the person.
    pub url: Vec<String>,
    /// Twitter account.
    pub twitter: Option<String>,
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
