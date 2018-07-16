//! Imprudence service.

use actix::prelude::*;
use actix_web::{AsyncResponder, Error, FutureResponse, HttpRequest, HttpResponse};
use futures::future::Future;

use app::AppState;
use db::{DbExecutor, GetImprudences};

pub mod response;

/// Processes the request for imprudence texts.
#[allow(unknown_lints, needless_pass_by_value)]
pub fn index(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    debug!("request for `imprudences::index()`: {:?}", req);
    fetch_imprudences(req.state().db()).responder()
}

/// Returns the imprudences.
fn fetch_imprudences(
    db: &Addr<Syn, DbExecutor>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(GetImprudences)
        .from_err()
        .and_then(|res| match res {
            Ok(contents) => Ok(HttpResponse::Ok().json(contents)),
            Err(e) => {
                error!("`fetch_imprudences()`: {}", e);
                Ok(HttpResponse::InternalServerError().into())
            },
        })
}
