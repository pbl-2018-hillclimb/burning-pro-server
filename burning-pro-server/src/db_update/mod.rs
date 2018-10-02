//! DB update service.

use actix_web::{Error, HttpResponse, HttpRequest, Query};
use tera::{Tera, Context};

use std::collections::HashMap;
                
use app::AppState;

/// Processes the request for DB register form.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn index(
    (req, query): (HttpRequest<AppState>, Query<HashMap<String, String>>)
) -> Result<HttpResponse, Error> {
    debug!("request for `db_update::index()`: {:?}", req);
    render_db_register_form(req.state().template(), &query)
}

/// render DB register form.
fn render_db_register_form(
    template: &Tera, query: &Query<HashMap<String, String>>
) -> Result<HttpResponse, Error> {
    let res = template.render("form.html", &Context::new());
    match res {
        Ok(contents) => Ok(HttpResponse::Ok().content_type("text/html").body(contents)),
        Err(e) => {
            error!("`render_db_register_form()`: {}", e);
            Ok(HttpResponse::InternalServerError().into())
        }
    }
}

