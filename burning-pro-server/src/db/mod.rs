//! DB executor actor.

use actix::prelude::*;
//use actix_web::Error;
use actix_web::error::ResponseError;
use chrono::{DateTime, FixedOffset};
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use r2d2;

use imprudence::response;
use models;
use schema;

/// DB operation error.
#[derive(Debug, Fail)]
pub enum Error {
    /// Connection pool error.
    #[fail(display = "Connection pool error: {}", _0)]
    ConnectionPool(r2d2::Error),
    /// Diesel query error.
    #[fail(display = "DB query error: {}", _0)]
    Db(diesel::result::Error),
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Error::ConnectionPool(e)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Error::Db(e)
    }
}

impl ResponseError for Error {}

/// DB executor.
pub struct DbExecutor(pub Pool<ConnectionManager<SqliteConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<GetImprudences> for DbExecutor {
    type Result = <GetImprudences as Message>::Result;

    fn handle(&mut self, _msg: GetImprudences, _ctx: &mut Self::Context) -> Self::Result {
        // Local offset.
        let tz_offset = FixedOffset::east(9 * 60 * 60);

        let conn = &self.0.get()?;
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

/// Message to get imprudences.
#[derive(Debug, Clone)]
pub struct GetImprudences;

impl Message for GetImprudences {
    type Result = Result<Vec<response::Imprudence>, Error>;
}
