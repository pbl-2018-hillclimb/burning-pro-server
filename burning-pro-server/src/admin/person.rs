//! Handler module for person update form.

use std::sync::Arc;

use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{AsyncResponder, Form, FutureResponse, HttpRequest, HttpResponse, Path};
use futures::future::Future;
use tera::Context;

use admin::{form, list_impl, render};
use app::AppState;
use db::{upsert_entry, PersonQuery, PersonUrlQuery};

/// Processes the request for person registration index.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn index(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    debug!("request for `admin::person::index()`: {:?}", req);

    let db = req.state().db();
    let rows = db
        .send(PersonQuery::All)
        .from_err()
        .and_then(move |res| match res {
            Ok(contents) => Ok(contents
                .into_iter()
                .map(|row| (row.person_id, row.display_name))
                .collect::<Vec<_>>()),
            Err(e) => {
                error!("`db_update::person_index()`: {}", e);
                Err(ErrorInternalServerError("DB Error"))
            }
        });
    let template = Arc::clone(req.state().template());
    list_impl(template, "発言者", rows)
}

/// Internal implementation for `.../new` and `.../{id}` endpoints with GET mehod.
fn get_impl(id: Option<i32>, req: &HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let db = req.state().db();
    let template = Arc::clone(req.state().template());

    let additional: Box<dyn Future<Item = _, Error = _>> = match id {
        Some(person_id) => {
            let person =
                db.send(PersonQuery::PersonId(person_id))
                    .from_err()
                    .and_then(|res| match res {
                        Ok(mut content) => content
                            .pop()
                            .map(|content| ("person", content))
                            .ok_or_else(|| {
                                debug!("Person not found.");
                                ErrorBadRequest("Person not found")
                            }),
                        Err(e) => {
                            error!("`admin::person::get_impl()`: {}", e);
                            Err(ErrorInternalServerError("DB error"))
                        }
                    });
            let person_url = db
                .send(PersonUrlQuery::PersonId(person_id))
                .from_err()
                .and_then(|res| match res {
                    Ok(content) => Ok(("person_url", content)),
                    Err(e) => {
                        error!("`admin::person::get_impl()`: {}", e);
                        Err(ErrorInternalServerError("DB error"))
                    }
                });
            Box::new(person.join(person_url).map(Some))
        }
        None => Box::new(futures::future::ok(None)),
    };

    additional
        .map(move |additional| {
            let mut ctx = Context::new();
            if let Some((person, person_url)) = additional {
                ctx.insert(person.0, &person.1);
                ctx.insert(person_url.0, &person_url.1);
            }
            render(&template, &ctx, "register/person/update.html")
        }).responder()
}

/// Processes the request for new person registration form.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn new(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    debug!("request for `admin::person::new()`: {:?}", req);
    get_impl(None, &req)
}

/// Processes the request for person update form.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn update(path: Path<i32>, req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    debug!("request for `admin::person::update()`: {:?}", req);
    let person_id = path.into_inner();
    get_impl(Some(person_id), &req)
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
