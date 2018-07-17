//! `GetGoodPhrases` message.

use actix::prelude::*;
use chrono::{DateTime, FixedOffset};
use diesel::prelude::*;

use db::{DbExecutor, Error};
use good_phrase::response;
use models;
use schema;

/// A message type to get good_phrases.
///
/// This may have query parameters to limit or filter results (while it does not
/// for now).
#[derive(Debug, Clone)]
pub struct GetGoodPhrases;

impl Message for GetGoodPhrases {
    type Result = Result<Vec<response::GoodPhrase>, Error>;
}

impl Handler<GetGoodPhrases> for DbExecutor {
    type Result = <GetGoodPhrases as Message>::Result;

    fn handle(&mut self, _msg: GetGoodPhrases, _ctx: &mut Self::Context) -> Self::Result {
        // Local offset.
        let tz_offset = FixedOffset::east(9 * 60 * 60);

        let conn = &self.pool().get()?;
        let good_phrases_and_persons = schema::good_phrases::table
            .inner_join(schema::persons::table)
            .load::<(models::GoodPhrase, models::Person)>(conn)?;
        let mut result = Vec::new();
        for (good_phrase, person) in good_phrases_and_persons {
            let urls = models::PersonUrl::belonging_to(&person).load::<models::PersonUrl>(conn)?;
            let tags = models::GoodPhraseAndTag::belonging_to(&good_phrase)
                .inner_join(schema::good_phrase_tags::table)
                .load::<(models::GoodPhraseAndTag, models::GoodPhraseTag)>(conn)?;

            let phrase = response::Phrase {
                internal_id: good_phrase.good_phrase_id,
                title: good_phrase.title.clone(),
                phrase: good_phrase.phrase.clone(),
                created: DateTime::from_utc(good_phrase.created_at, tz_offset),
                url: good_phrase.url.clone(),
                deleted: good_phrase.deleted,
                datetime: good_phrase
                    .published_at
                    .map(|t| DateTime::from_utc(t, tz_offset)),
            };
            let person = response::Person {
                internal_id: person.person_id,
                created: DateTime::from_utc(person.created_at, tz_offset),
                real_name: person.real_name.clone(),
                display_name: person.display_name.clone(),
                url: urls.into_iter().map(|url| url.url).collect(),
                twitter: person.twitter.clone(),
            };
            // TODO: sys_meta: `use_count` and `fav_count` is dummy.
            let sys_meta = response::SysMeta {
                use_count: 5,
                fav_count: 3,
                tags: tags.into_iter().map(|(_tag_rel, tag)| tag.name).collect(),
            };
            // TODO: user_meta: dummy.
            let user_meta = response::UserMeta {
                favorite: true,
                use_count: 2,
                mylists: vec!["言い訳用".into()],
            };
            result.push(response::GoodPhrase {
                phrase,
                person,
                sys_meta,
                user_meta,
            });
        }
        Ok(result)
    }
}
