//! Handler module for tag update form.

use std::sync::Arc;

use actix_web::{AsyncResponder, FutureResponse, HttpResponse,
                HttpRequest, Form, Path};
use actix_web::error::{ErrorInternalServerError, ErrorBadRequest};
use tera::Context;
use futures::future::Future;

use app::AppState;
use db::{upsert_entry, GoodPhraseTagQuery};
use admin::{form, render};

/// Processes the request for tag registration index.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn index(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    debug!("request for `admin::tag::index()`: {:?}", req);
    let db = req.state().db();
    let template = Arc::clone(req.state().template());
    db.send(GoodPhraseTagQuery::All)
        .from_err()
        .and_then(move |res| match res {
            Ok(contents) => {
                let mut ctx = Context::new();
                ctx.insert("rows", &contents);
                Ok(render(&template, &ctx, "register/tag/index.html"))
            }
            Err(e) => {
                error!("`admin::tag::index()`: {}", e);
                Err(ErrorInternalServerError("DB error"))
            }
        })
        .responder()
}

/// Processes the request for new tag registration form.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn new(req: HttpRequest<AppState>) -> HttpResponse {
    debug!("request for `admin::tag::new()`: {:?}", req);
    let template = req.state().template();
    render(template, &Context::new(), "register/tag/new.html")
}

/// Processes the request for tag update form.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn update(
    path: Path<i32>, req: HttpRequest<AppState>
) -> FutureResponse<HttpResponse> {
    debug!("request for `admin::tag::update()`: {:?}", req);
    let tag_id = path.into_inner();
    let db = req.state().db();
    let template = Arc::clone(req.state().template());
    db.send(GoodPhraseTagQuery::TagId(tag_id))
        .from_err()
        .and_then(move |res| match res {
            Ok(content) => if content.is_empty() {
                debug!("Tag not found.");
                Err(ErrorBadRequest("Tag not found"))
            } else {
                let mut ctx = Context::new();
                ctx.insert("tag", &content[0]);
                Ok(render(&template, &ctx, "register/tag/update.html"))
            },
            Err(e) => {
                error!("`admin::tag::update()`: {}", e);
                Err(ErrorInternalServerError("DB error"))
            }
        })
        .responder()
}

/// Processes the tag update query.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn post(
    req: HttpRequest<AppState>, form: Form<form::Tag>
) -> HttpResponse {
    debug!("request for `admin::tag::post()`: {:?}", req);
    
    let form_content = &form.into_inner();
    
    debug!("receive form:\n{:#?}", &form_content);
        
    // TODO: register tag
    
    let template = req.state().template();
    let mut ctx = Context::new();
    ctx.insert("tag", &form_content);    
    render(template, &ctx, "register/tag/post.html")
}
