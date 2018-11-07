//! Handler module for phrase request

use actix_web::error::ErrorInternalServerError;
use actix_web::{AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Json};
use futures::future::Future;

use admin::form;
use app::AppState;
use db::upsert_entry;

/// Processes the phrase request query
#[allow(unknown_lints, needless_pass_by_value)]
pub fn post(
    req: HttpRequest<AppState>,
    form: Json<form::PhraseRequest>,
) -> FutureResponse<HttpResponse> {
    debug!("request for `admin::phrase_request()`: {:?}", req);

    let mut form_content = form.into_inner();
    form_content.phrase = form_content.phrase.trim().to_string();
    debug!("receive form:\n{:#?}", &form_content);
    let form::PhraseRequest {
        title,
        phrase,
        person,
        url,
        deleted,
        published_at,
        tags,
    } = form_content.to_owned();

    let upsert_msg = upsert_entry::GoodPhraseRequest {
        title,
        phrase,
        person,
        url,
        deleted,
        published_at,
        tags,
    };

    let db = req.state().db();
    db.send(upsert_msg)
        .from_err()
        .and_then(move |res| match res {
            Ok(_) => Ok(HttpResponse::Ok().into()),
            Err(e) => {
                error!("`admin::phrase_request::post()`: {}", e);
                Err(ErrorInternalServerError("DB error"))
            }
        }).responder()
}
