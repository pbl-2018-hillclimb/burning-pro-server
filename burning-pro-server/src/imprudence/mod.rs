//! Imprudence service.

use actix_web::{AsyncResponder, Error, FutureResponse, HttpRequest, Json};
use futures;
use futures::future::{Future, FutureResult};

mod response;


/// Processes the request for imprudence texts.
pub fn index(_req: HttpRequest) -> FutureResponse<Json<Vec<response::Imprudence>>, Error> {
    fetch_imprudences().map(|entries| Json(entries)).responder()
}


/// Returns the imprudences.
fn fetch_imprudences() -> FutureResult<Vec<response::Imprudence>, Error> {
    use self::response::{Imprudence, Person, Phrase, SysMeta, UserMeta};
    use chrono::Local;

    futures::future::ok(vec![
        Imprudence {
            phrase: Phrase {
                internal_id: 0,
                title: "大した問題じゃないでしょう".into(),
                phrase: "{大学}から{休講情報}が{出ない}という文句のツイートが散見されますが，{大学生}なんだから自分で判断して{休みたかったら休めばいい}のではと思ってしまいます．{授業}に{一回}くらい{出なくた}って大した問題じゃないでしょう．".into(),
                created: Local::now(),
                url: Some("https://twitter.com/wtakuo/status/688879244567445504".into()),
                deleted: true,
                datetime: None,
            },
            person: Person {
                internal_id: 0,
                created: Local::now(),
                real_name: None,
                display_name: Some("wtakuo".into()),
                url: vec!["https://twitter.com/wtakuo".into()],
                twitter: Some("wtakuo".into()),
            },
            sys_meta: SysMeta {
                use_count: 5,
                fav_count: 3,
                tags: vec!["炎上実績".into()],
            },
            user_meta: UserMeta {
                favorite: true,
                use_count: 2,
                mylists: vec!["言い訳用".into()],
            },
        },
    ])
}
