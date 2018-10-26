//! Burning-pro server.

extern crate burning_pro_server;

use burning_pro_server::app::{AppState, AppStateBuilder};
use burning_pro_server::{admin, good_phrase};

extern crate actix;
extern crate actix_web;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::env;

use actix_web::middleware::Logger;
use actix_web::{http, server, App, HttpRequest};

/// Setup global logger.
fn setup_logger() {
    const DEFAULT_LOG_LEVEL: &str = "trace";
    let underscored_name = env!("CARGO_PKG_NAME").replace('-', "_");
    let defval = format!("{}={}", underscored_name, DEFAULT_LOG_LEVEL);

    // `actix_web=info` to print access log by `actix_web::middleware::Logger`.
    let newval = match env::var("RUST_LOG") {
        Ok(v) => format!("actix_web=info,{},{}", defval, v),
        Err(_) => defval,
    };
    env::set_var("RUST_LOG", &newval);

    pretty_env_logger::init();

    trace!("RUST_LOG={}", newval);
}

#[allow(unknown_lints, needless_pass_by_value)]
fn fire(req: HttpRequest<AppState>) -> &'static str {
    debug!("request for `fire()`: {:?}", req);
    // Fire.
    "\u{1F525}"
}

macro_rules! regist_form_handler {
    ($root:path, $new:path, $update:path, $post:path) => {
        |scope| {
            scope
                .resource("/", |r| r.with($root))
                .resource("/new/", |r| r.with($new))
                .resource("/update/{id}/", |r| r.with($update))
                .resource("/post/", |r| {
                    r.method(http::Method::POST).with_config($post, |(_, cfg)| {
                        cfg.error_handler(admin::post_err_handler);
                    })
                })
        }
    };
}

fn main() {
    match dotenv::dotenv() {
        Ok(path) => info!("Successfully loaded dotenv file: {}", path.display()),
        Err(e) => {
            if e.not_found() {
                info!("No dotenv file found");
            } else {
                error!("Dotenv initialization failed: {}", e);
                panic!("Dotenv initialization failed: {:?}", e);
            }
        }
    }
    setup_logger();

    info!(
        "{}, version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    // TODO: Use config file to determine address and port to listen.
    let listen = "0.0.0.0:8080";

    let sys = actix::System::new("burning-pro-server");

    let database_url = env::var("DATABASE_URL").expect("`DATABASE_URL` envvar must be set");
    info!("Database URL: {}", database_url);
    let app_state = AppStateBuilder::new()
        .database_url(database_url)
        .build()
        .expect("Failed to build application state");

    info!("starting server ({})...", listen);
    server::new(move || {
        App::with_state(app_state.clone())
            .middleware(Logger::default())
            .resource("/", |r| r.with(fire))
            .resource("/good_phrases/", |r| r.with(good_phrase::index))
            .scope("/register", |scope| {
                scope
                    .resource("/", |r| r.with(admin::index))
                    .nested(
                        "/phrase",
                        regist_form_handler!(
                            admin::phrase::index,
                            admin::phrase::new,
                            admin::phrase::update,
                            admin::phrase::post
                        ),
                    ).nested(
                        "/tag",
                        regist_form_handler!(
                            admin::tag::index,
                            admin::tag::new,
                            admin::tag::update,
                            admin::tag::post
                        ),
                    ).nested(
                        "/person",
                        regist_form_handler!(
                            admin::person::index,
                            admin::person::new,
                            admin::person::update,
                            admin::person::post
                        ),
                    )
            })
    }).bind(listen)
    .unwrap_or_else(|e| {
        panic!("Failed to bind {}: {}", listen, e);
    }).start();

    info!("started server ({})", listen);

    let _ = sys.run();
}
