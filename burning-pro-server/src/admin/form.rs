//! Form types.

use std::collections::HashMap;

use chrono::{DateTime, Local};

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
    #[serde(with = "optstr_fmt")]
    pub url: Option<String>,
    /// Whether the source web page is deleted or not.
    pub deleted: bool,
    /// Datetime when the phrase is published.
    #[serde(with = "optdate_fmt")]
    pub published_at: Option<DateTime<Local>>,
    /// Extra form field.
    /// Contains selected tag id.
    #[serde(flatten)]
    pub extra: HashMap<String, String>
}

/// A person.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    /// Row ID (`None` for new entry)
    pub person_id: Option<i32>,
    /// Real name.
    #[serde(with = "optstr_fmt")]
    pub real_name: Option<String>,
    /// Display name.
    #[serde(with = "optstr_fmt")]
    pub display_name: Option<String>,
    /// URLs of web pages of the person.
    #[serde(with = "strvec_fmt")]
    pub url: Vec<String>,
    /// Twitter account.
    #[serde(with = "optstr_fmt")]
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
    #[serde(with = "optstr_fmt")]
    pub description: Option<String>,
}

/// Custom Serde format for `Vec<String>`.
/// Convert comma-separated string <-> `Vec<String>`.
mod strvec_fmt {
    use serde::{Deserialize, Serializer, Deserializer};

    pub fn serialize<S>(
        str_vec: &Vec<String>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", str_vec.join(","));
        serializer.serialize_str(&s)
    }
    
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(Vec::new())
        } else {
            Ok(s.split(',')
               .map(|s| s.trim().to_string())
               .collect::<Vec<_>>())
        }
    }
}

/// Custom Serde format for `Option<String>`.
/// Convert `""` <-> `None`, `$non_empty` <-> `Some($non_empty)`.
/// (By default, empty form values are deserialized to `Some("")`.)
mod optstr_fmt {
    use serde::{Deserialize, Serializer, Deserializer};

    pub fn serialize<S>(
        optdate: &Option<String>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match optdate {
            Some(val) => serializer.serialize_str(val),
            None => serializer.serialize_none()
        }
    }
    
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(s))
        }
    }
}

/// Custom Serde format for `Option<DateTime<Local>>`
/// Convert YYYY-mm-ddTHH:MM format(maybe empty) <-> `Option<DateTime<Local>>`
mod optdate_fmt {
    use serde::{Deserialize, Serializer, Deserializer};
    use serde::de::Error;
    use chrono::{DateTime, Local, FixedOffset, TimeZone};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M";

    pub fn serialize<S>(
        optdate: &Option<DateTime<Local>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match optdate {
            Some(date) => format!("{}", date.format(FORMAT)),
            None => String::new()
        };
        serializer.serialize_str(&s)
    }
    
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<DateTime<Local>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(None)
        } else {
            let tz_offset = FixedOffset::east(9 * 60 * 60);
            Local::from_offset(&tz_offset)
                .datetime_from_str(&s, FORMAT)
                .map(|dt| Some(dt))
                .map_err(Error::custom)
        }
    }
}
