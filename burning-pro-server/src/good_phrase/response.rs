//! Response types.

use chrono::{DateTime, Local};

/// An good_phrase.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GoodPhrase {
    /// Phrase.
    pub phrase: Phrase,
    /// Person.
    pub person: Person,
    /// System metadata.
    pub sys_meta: SysMeta,
    /// User metadata.
    pub user_meta: UserMeta,
}

/// A phrase.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Phrase {
    /// Internal ID for DB.
    pub internal_id: i32,
    /// Title (short summary).
    pub title: String,
    /// Phrase.
    pub phrase: String,
    /// Datetime when the entry is created.
    pub created: DateTime<Local>,
    /// URL of the phrase if it is posted or published to the WWW.
    pub url: Option<String>,
    /// Whether the source web page is deleted or not.
    pub deleted: bool,
    /// Datetime when the phrase is published.
    pub datetime: Option<DateTime<Local>>,
}

/// A person.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Person {
    /// Internal ID for DB.
    pub internal_id: i32,
    /// Datetime when the entry is created.
    pub created: DateTime<Local>,
    /// Real name.
    pub real_name: Option<String>,
    /// Display name.
    pub display_name: String,
    /// URLs of web pages of the person.
    pub url: Vec<String>,
    /// Twitter account.
    pub twitter: Option<String>,
}

/// System-wide metadata.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SysMeta {
    /// Count of all users' use of the phrase.
    pub use_count: u64,
    /// Count of all users' favorites for the phrase.
    pub fav_count: u64,
    /// System-wide "tag" for the phrase.
    pub tags: Vec<String>,
}

/// User-local metadata.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct UserMeta {
    /// Whether the user favorited the phrase.
    pub favorite: bool,
    /// Count of the user's use of the phrase.
    pub use_count: u64,
    /// Mylists which the phrase is registered to.
    pub mylists: Vec<String>,
}
