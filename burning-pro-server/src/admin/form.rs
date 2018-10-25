//! Form types.

use std::collections::HashMap;
use std::fmt;

use chrono::{DateTime, FixedOffset, Local, TimeZone};
use serde::de;

/// A phrase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phrase {
    /// Row ID (`None` for new entry)
    pub good_phrase_id: Option<i32>,
    /// Title (short summary).
    pub title: String,
    /// Phrase.
    pub phrase: String,
    /// Author's person id.
    pub person_id: i32,
    /// URL of the phrase if it is posted or published to the WWW.
    #[serde(deserialize_with = "deserialize_optstr")]
    pub url: Option<String>,
    /// Whether the source web page is deleted or not.
    pub deleted: bool,
    /// Datetime when the phrase is published.
    #[serde(deserialize_with = "deserialize_optdate")]
    pub published_at: Option<DateTime<Local>>,
    /// Extra form field.
    ///
    /// Contains selected tag_ids, map person_id to display_name, and
    /// mapping of persons / tags from indices to names.
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}

/// A person.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    /// Row ID (`None` for new entry)
    pub person_id: Option<i32>,
    /// Real name.
    #[serde(deserialize_with = "deserialize_optstr")]
    pub real_name: Option<String>,
    /// Display name.
    #[serde(deserialize_with = "deserialize_optstr")]
    pub display_name: Option<String>,
    /// URLs of web pages of the person.
    #[serde(deserialize_with = "deserialize_strvec")]
    //#[serde(with = "strvec_fmt")]
    pub url: Vec<String>,
    /// Twitter account.
    #[serde(deserialize_with = "deserialize_optstr")]
    pub twitter: Option<String>,
}

/// A tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// Row ID (`None` for new entry)
    pub good_phrase_tag_id: Option<i32>,
    /// Name.
    pub name: String,
    /// Description of tag.
    #[serde(deserialize_with = "deserialize_optstr")]
    pub description: Option<String>,
}

/// Custom deserializer for `Option<String>`.
///
/// Convert `""` -> `None`, `$non_empty` -> `Some($non_empty)`.
/// (By default, empty form values are deserialized to `Some("")`.)
fn deserialize_strvec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct StrvecVisitor;

    impl<'de> de::Visitor<'de> for StrvecVisitor {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a comma-separated string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let s = v.to_string();
            if s.is_empty() {
                Ok(Vec::new())
            } else {
                Ok(s.split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<_>>())
            }
        }
    }

    deserializer.deserialize_any(StrvecVisitor)
}

/// Custom deserializer for `Option<String>`.
///
/// Convert `""` -> `None`, `$non_empty` -> `Some($non_empty)`.
/// (By default, empty form values are deserialized to `Some("")`.)
fn deserialize_optstr<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct OptstrVisitor;

    impl<'de> de::Visitor<'de> for OptstrVisitor {
        type Value = Option<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.is_empty() {
                Ok(None)
            } else {
                Ok(Some(v.to_string()))
            }
        }
    }

    deserializer.deserialize_any(OptstrVisitor)
}

/// Custom deserializer for `Option<DateTime<Local>>`
///
/// Convert YYYY-MM-DDThh:mm:ss format(maybe empty) -> `Option<DateTime<Local>>`
fn deserialize_optdate<'de, D>(deserializer: D) -> Result<Option<DateTime<Local>>, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct OptdateVisitor;

    impl<'de> de::Visitor<'de> for OptdateVisitor {
        type Value = Option<DateTime<Local>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string `YYYY-MM-DDThh:mm:ss`")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.is_empty() {
                Ok(None)
            } else {
                let tz_offset = FixedOffset::east(9 * 60 * 60);
                Local::from_offset(&tz_offset)
                    .datetime_from_str(v, "%Y-%m-%dT%H:%M:%S")
                    .map(|dt| Some(dt))
                    .map_err(de::Error::custom)
            }
        }
    }

    deserializer.deserialize_any(OptdateVisitor)
}
