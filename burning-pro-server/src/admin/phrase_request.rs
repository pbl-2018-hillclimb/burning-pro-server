//! Handler module for phrase request

use std::sync::Arc;

use actix_web::error::ErrorInternalServerError;
use actix_web::{AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Json};
use futures::future::Future;

use admin::{form, list_impl, render};
use app::AppState;
use db::{upsert_entry, GoodPhraseRequestQuery};
use tera::Context;

/// Processes the request for phrase request registration index.
pub fn index(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    debug!("request for `admin::phrase_request::index()`: {:?}", req);

    let db = req.state().db();
    let rows = db
        .send(GoodPhraseRequestQuery::All)
        .from_err()
        .and_then(move |res| match res {
            Ok(contents) => Ok(contents),
            Err(e) => {
                error!("`admin::phrase_request::index()`: {}", e);
                Err(ErrorInternalServerError("DB Error"))
            }
        });
    let template = Arc::clone(req.state().template());
    let mut ctx = Context::new();
    rows.map(move |rows| {
        ctx.insert("rows", &rows);
        render(
            &template,
            &ctx,
            "register/phrase_request/phrase_request.html",
        )
    }).responder()
}

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
        phrase,
        person,
        url,
        deleted,
        published_at,
    } = form_content.to_owned();

    let upsert_msg = upsert_entry::GoodPhraseRequest {
        phrase,
        person,
        url,
        deleted,
        published_at,
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
