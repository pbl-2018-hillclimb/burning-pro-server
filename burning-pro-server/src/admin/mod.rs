//! DB update service.

use actix_web::error::{ResponseError, UrlencodedError};
use actix_web::{Error, HttpRequest, HttpResponse};
use tera::{Context, Tera};

use app::AppState;

pub mod form;
pub mod person;
pub mod phrase;
pub mod tag;

#[derive(Fail, Debug)]
#[fail(display = "form request error")]
struct FormRequestError {
    err: UrlencodedError,
}

impl ResponseError for FormRequestError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().body(format!("Bad Request: {:#?}", self.err))
    }
}

/// Handles form deserialization error
#[allow(unknown_lints, needless_pass_by_value)]
pub fn post_err_handler(err: UrlencodedError, req: &HttpRequest<AppState>) -> Error {
    error!("fail to deserialize request: {:?}", req);
    (FormRequestError { err }).into()
}

/// Renders web pages.
fn render(template: &Tera, ctx: &Context, path: &str) -> HttpResponse {
    let res = template.render(path, ctx);
    match res {
        Ok(contents) => HttpResponse::Ok().content_type("text/html").body(contents),
        Err(e) => {
            error!("`db_update::render()`: {}", e);
            HttpResponse::InternalServerError().into()
        }
    }
}

/// Processes the request for DB register form index.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn index(req: HttpRequest<AppState>) -> HttpResponse {
    debug!("request for `db_update::index()`: {:?}", req);
    render(
        req.state().template(),
        &Context::new(),
        "register/index.html",
    )
}
