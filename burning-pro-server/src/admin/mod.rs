//! DB update service.

use std::sync::Arc;

use actix_web::{AsyncResponder, Error, FutureResponse, HttpRequest, HttpResponse};
use futures::future::Future;
use tera::{Context, Tera};

use app::AppState;

pub mod form;
pub mod person;
pub mod phrase;
pub mod phrase_request;
pub mod tag;

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

/// Show list of rows for a table.
fn list_impl(
    template: Arc<Tera>,
    table_name: &str,
    rows: impl 'static + Future<Item = Vec<(i32, String)>, Error = Error>,
) -> FutureResponse<HttpResponse> {
    let mut ctx = Context::new();
    ctx.insert("table_name", table_name);
    rows.map(move |rows| {
        ctx.insert("rows", &rows);
        render(&template, &ctx, "register/list.html")
    }).responder()
}
