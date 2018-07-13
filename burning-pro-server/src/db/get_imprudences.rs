//! `GetImprudences` message.

use actix::prelude::*;
use chrono::{DateTime, FixedOffset};
use diesel::prelude::*;

use db::{DbExecutor, Error};
use imprudence::response;
use models;
use schema;

/// A message type to get imprudences.
///
/// This may have query parameters to limit or filter results (while it does not
/// for now).
#[derive(Debug, Clone)]
pub struct GetImprudences;

impl Message for GetImprudences {
    type Result = Result<Vec<response::Imprudence>, Error>;
}

impl Handler<GetImprudences> for DbExecutor {
    type Result = <GetImprudences as Message>::Result;

    fn handle(&mut self, _msg: GetImprudences, _ctx: &mut Self::Context) -> Self::Result {
        // Local offset.
        let tz_offset = FixedOffset::east(9 * 60 * 60);

        let conn = &self.pool().get()?;
        let imprudences_and_persons = schema::imprudences::table
            .inner_join(schema::persons::table)
            .load::<(models::Imprudence, models::Person)>(conn)?;
        let mut result = Vec::new();
        for (imprudence, person) in imprudences_and_persons {
            let urls = models::PersonUrl::belonging_to(&person).load::<models::PersonUrl>(conn)?;
            let tags = models::ImprudenceAndTag::belonging_to(&imprudence)
                .inner_join(schema::imprudence_tags::table)
                .load::<(models::ImprudenceAndTag, models::ImprudenceTag)>(conn)?;

            let phrase = response::Phrase {
                internal_id: imprudence.imprudence_id,
                title: imprudence.title.clone(),
                phrase: imprudence.phrase.clone(),
                created: DateTime::from_utc(imprudence.created_at, tz_offset),
                url: imprudence.url.clone(),
                deleted: imprudence.deleted,
                datetime: imprudence
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
            result.push(response::Imprudence {
                phrase,
                person,
                sys_meta,
                user_meta,
            });
        }
        Ok(result)
    }
}
