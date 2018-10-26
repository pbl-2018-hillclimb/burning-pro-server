//! Handler module for person update form.

use std::sync::Arc;

use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{AsyncResponder, Form, FutureResponse, HttpRequest, HttpResponse, Path};
use futures::future::Future;
use tera::Context;

use admin::{form, render};
use app::AppState;
use db::{upsert_entry, PersonQuery, PersonUrlQuery};

/// Processes the request for person registration index.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn index(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    debug!("request for `admin::person::index()`: {:?}", req);
    let db = req.state().db();
    let template = Arc::clone(req.state().template());
    db.send(PersonQuery::All)
        .from_err()
        .and_then(move |res| match res {
            Ok(contents) => {
                let mut ctx = Context::new();
                ctx.insert("table_name", "発言者");
                let rows = contents
                    .into_iter()
                    .map(|row| (row.person_id, row.display_name))
                    .collect::<Vec<_>>();
                ctx.insert("rows", &rows);
                Ok(render(&template, &ctx, "register/list.html"))
            }
            Err(e) => {
                error!("`db_update::person_index()`: {}", e);
                Err(ErrorInternalServerError("DB Error"))
            }
        }).responder()
}

/// Processes the request for new person registration form.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn new(req: HttpRequest<AppState>) -> HttpResponse {
    debug!("request for `admin::person::new()`: {:?}", req);
    let template = req.state().template();
    render(template, &Context::new(), "register/person/new.html")
}

/// Processes the request for person update form.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn update(path: Path<i32>, req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    debug!("request for `admin::person::update()`: {:?}", req);
    let person_id = path.into_inner();
    let db = req.state().db();
    let person = db
        .send(PersonQuery::PersonId(person_id))
        .from_err()
        .and_then(|res| match res {
            Ok(mut content) => if content.is_empty() {
                debug!("Person not found.");
                Err(ErrorBadRequest("Person not found"))
            } else {
                Ok(content.swap_remove(0))
            },
            Err(e) => {
                error!("`admin::person::update()`: {}", e);
                Err(ErrorInternalServerError("DB error"))
            }
        });
    let person_url = db
        .send(PersonUrlQuery::PersonId(person_id))
        .from_err()
        .and_then(|res| match res {
            Ok(content) => Ok(content),
            Err(e) => {
                error!("`admin::person::update()`: {}", e);
                Err(ErrorInternalServerError("DB error"))
            }
        });
    let template = Arc::clone(req.state().template());
    person
        .join(person_url)
        .map(|(person, person_url)| {
            let mut ctx = Context::new();
            ctx.insert("person", &person);
            ctx.insert("person_url", &person_url);
            ctx
        }).map(move |ctx| render(&template, &ctx, "register/person/update.html"))
        .responder()
}

/// Processes the person update query.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn post(req: HttpRequest<AppState>, form: Form<form::Person>) -> FutureResponse<HttpResponse> {
    debug!("request for `admin::person::post()`: {:?}", req);
    let form_content = form.into_inner();
    debug!("receive form:\n{:#?}", &form_content);
    let upsert_msg = match form_content.to_owned() {
        form::Person {
            person_id,
            real_name,
            display_name,
            url,
            twitter,
        } => upsert_entry::Person {
            person_id,
            real_name,
            display_name,
            url,
            twitter,
        },
    };
    let db = req.state().db();
    let template = Arc::clone(req.state().template());
    db.send(upsert_msg)
        .from_err()
        .and_then(move |res| match res {
            Ok(_) => {
                let mut ctx = Context::new();
                ctx.insert("person", &form_content);
                Ok(render(&template, &ctx, "register/person/post.html"))
            }
            Err(e) => {
                error!("`admin::person::update()`: {}", e);
                Err(ErrorInternalServerError("DB error"))
            }
        }).responder()
}
