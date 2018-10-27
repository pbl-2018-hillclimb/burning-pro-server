//! Handler module for tag update form.

use std::sync::Arc;

use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{AsyncResponder, Form, FutureResponse, HttpRequest, HttpResponse, Path};
use futures::future::Future;
use tera::Context;

use admin::{form, render};
use app::AppState;
use db::{upsert_entry, GoodPhraseTagQuery};

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
                ctx.insert("table_name", "タグ");
                let rows = contents
                    .into_iter()
                    .map(|row| (row.good_phrase_tag_id, row.name))
                    .collect::<Vec<_>>();
                ctx.insert("rows", &rows);
                Ok(render(&template, &ctx, "register/list.html"))
            }
            Err(e) => {
                error!("`admin::tag::index()`: {}", e);
                Err(ErrorInternalServerError("DB error"))
            }
        }).responder()
}

/// Internal implementation for `.../new` and `.../{id}` endpoints with GET mehod.
fn get_impl(id: Option<i32>, req: &HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let db = req.state().db();
    let template = Arc::clone(req.state().template());

    let additional: Box<dyn Future<Item = _, Error = _>> =
        match id {
            Some(tag_id) => Box::new(Some(
                db.send(GoodPhraseTagQuery::TagId(tag_id))
                    .from_err()
                    .and_then(|res| match res {
                        Ok(mut content) => content
                            .pop()
                            .map(|content| ("tag", content))
                            .ok_or_else(|| {
                                debug!("Tag not found.");
                                ErrorBadRequest("Tag not found")
                            }),
                        Err(e) => {
                            error!("`admin::tag::get_impl()`: {}", e);
                            Err(ErrorInternalServerError("DB error"))
                        }
                    }),
            )),
            None => Box::new(futures::future::ok(None)),
        };

    additional
        .map(move |additional| {
            let mut ctx = Context::new();
            if let Some(tag) = additional {
                ctx.insert(tag.0, &tag.1);
            }
            render(&template, &ctx, "register/tag/update.html")
        }).responder()
}

/// Processes the request for new tag registration form.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn new(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    debug!("request for `admin::tag::new()`: {:?}", req);
    get_impl(None, &req)
}

/// Processes the request for tag update form.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn update(path: Path<i32>, req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    debug!("request for `admin::tag::update()`: {:?}", req);
    let tag_id = path.into_inner();
    get_impl(Some(tag_id), &req)
}

/// Processes the tag update query.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn post(req: HttpRequest<AppState>, form: Form<form::Tag>) -> FutureResponse<HttpResponse> {
    debug!("request for `admin::tag::post()`: {:?}", req);
    let mut form_content = form.into_inner();
    form_content.description = form_content.description.map(|x| x.trim().to_string());
    debug!("receive form:\n{:#?}", &form_content);
    let upsert_msg = match form_content.to_owned() {
        form::Tag {
            good_phrase_tag_id,
            name,
            mut description,
        } => upsert_entry::GoodPhraseTag {
            good_phrase_tag_id,
            name,
            description,
        },
    };
    let db = req.state().db();
    let template = Arc::clone(req.state().template());
    db.send(upsert_msg)
        .from_err()
        .and_then(move |res| match res {
            Ok(_) => {
                let mut ctx = Context::new();
                ctx.insert("tag", &form_content);
                Ok(render(&template, &ctx, "register/tag/post.html"))
            }
            Err(e) => {
                error!("`admin::tag::update()`: {}", e);
                Err(ErrorInternalServerError("DB error"))
            }
        }).responder()
}
