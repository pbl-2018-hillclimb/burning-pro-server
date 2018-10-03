//! Handler module for phrase update form.

use std::sync::Arc;

use actix::Addr;
use actix_web::{Error, AsyncResponder, FutureResponse, HttpResponse,
                HttpRequest, Form, Path};
use actix_web::error::{ErrorInternalServerError, ErrorBadRequest};
use tera::Context;
use futures::future::Future;

use app::AppState;
use db::{upsert_entry, DbExecutor, GoodPhraseQuery, GoodPhraseTagQuery, PersonQuery};
use admin::{form, render};

/// Processes the request for phrase registration index.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn index(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    debug!("request for `admin::phrase::index()`: {:?}", req);
    let db = req.state().db();
    let template = Arc::clone(req.state().template());
    db.send(GoodPhraseQuery::All)
        .from_err()
        .and_then(move |res| match res {
            Ok(contents) => {
                let mut ctx = Context::new();
                ctx.insert("rows", &contents);
                Ok(render(&template, &ctx, "register/phrase/index.html"))
            }
            Err(e) => {
                error!("`admin::phrase::index()`: {}", e);
                Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}

/// Prepare context for rendering phrase form.
fn make_phrase_ctx(
    db: &Addr<DbExecutor>
) -> impl Future<Item = Context, Error = Error> {
    let all_tag = db.send(GoodPhraseTagQuery::All)
        .from_err()
        .and_then(|res| match res {
            Ok(content) => Ok(content),
            Err(e) => {
                error!("`admin::phrase::make_phrase_ctx()`: {}", e);
                Err(ErrorInternalServerError("DB error"))
            }
        });
    let all_person = db.send(PersonQuery::All)
        .from_err()
        .and_then(|res| match res {
            Ok(content) => Ok(content),
            Err(e) => {
                error!("`admin::phrase::make_phrase_ctx()`: {}", e);
                Err(ErrorInternalServerError("DB error"))
            }
        });
    all_tag.join(all_person)
        .map(|(all_tag, all_person)| {
            let mut ctx = Context::new();
            ctx.insert("all_tag", &all_tag);
            ctx.insert("all_person", &all_person);
            ctx
        })
}

/// Processes the request for new phrase registration form.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn new(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    debug!("request for `admin::phrase::new()`: {:?}", req);
    let db = req.state().db();
    let template = Arc::clone(req.state().template());
    make_phrase_ctx(db)
        .map(move |ctx| render(&template, &ctx, "register/phrase/new.html"))
        .responder()
}

/// Processes the request for phrase update form.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn update(
    path: Path<i32>, req: HttpRequest<AppState>
) -> FutureResponse<HttpResponse> {
    debug!("request for `admin::phrase::update()`: {:?}", req);
    let phrase_id = path.into_inner();
    let db = req.state().db();
    let ctx = make_phrase_ctx(db);
    let phrase = db.send(GoodPhraseQuery::PhraseId(phrase_id))
        .from_err()
        .and_then(|res| match res {
            Ok(content) => if content.is_empty() {
                debug!("Phrase not found.");
                Err(ErrorBadRequest("Phrase not found"))
            } else {
                Ok(content[0].to_owned())
            },
            Err(e) => {
                error!("`admin::phrase::update()`: {}", e);
                Err(ErrorInternalServerError("DB error"))
            }
        });
    let phrase_tag = db.send(GoodPhraseTagQuery::PhraseId(phrase_id))
        .from_err()
        .and_then(|res| match res {
            Ok(content) => Ok(content),
            Err(e) => {
                error!("`admin::phrase::update()`: {}", e);
                Err(ErrorInternalServerError("DB error"))
            }
        });
    let template = Arc::clone(req.state().template());
    ctx.join3(phrase, phrase_tag)
        .map(|(mut ctx, phrase, phrase_tag)| {
            ctx.insert("phrase", &phrase);
            ctx.insert("phrase_tag", &phrase_tag);
            ctx
        })
        .map(move |ctx| render(&template, &ctx, "register/phrase/update.html"))
        .responder()
}

/// Processes the phrase update query.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn post(
    req: HttpRequest<AppState>, form: Form<form::Phrase>
) -> HttpResponse {
    debug!("request for `admin::phrase::post()`: {:?}", req);
    
    let form_content = &form.into_inner();
    
    debug!("receive form:\n{:#?}", &form_content);
    // TODO: register phrase
    
    let template = req.state().template();
    let mut ctx = Context::new();
    ctx.insert("phrase", &form_content);    
    render(template, &ctx, "register/phrase/post.html")
}
