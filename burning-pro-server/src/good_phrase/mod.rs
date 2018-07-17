//! GoodPhrase service.

use actix::prelude::*;
use actix_web::{AsyncResponder, Error, FutureResponse, HttpRequest, HttpResponse};
use futures::future::Future;

use app::AppState;
use db::{DbExecutor, GetGoodPhrases};

pub mod response;

/// Processes the request for good_phrase texts.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn index(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    debug!("request for `good_phrases::index()`: {:?}", req);
    fetch_good_phrases(req.state().db()).responder()
}

/// Returns the good_phrases.
fn fetch_good_phrases(
    db: &Addr<Syn, DbExecutor>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(GetGoodPhrases)
        .from_err()
        .and_then(|res| match res {
            Ok(contents) => Ok(HttpResponse::Ok().json(contents)),
            Err(e) => {
                error!("`fetch_good_phrases()`: {}", e);
                Ok(HttpResponse::InternalServerError().into())
            },
        })
}
