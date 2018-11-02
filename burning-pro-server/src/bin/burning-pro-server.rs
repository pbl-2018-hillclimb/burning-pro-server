//! Burning-pro server.

extern crate burning_pro_server;

use burning_pro_server::app::{AdminAuthenticator, AppState, AppStateBuilder};
use burning_pro_server::{admin, good_phrase};

extern crate actix;
extern crate actix_web;
extern crate actix_web_httpauth;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::env;

use actix_web::middleware::{Logger, Middleware, Started};
use actix_web::{server, App, HttpRequest};

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
                .resource("/new/", |r| {
                    r.get().with($new);
                    r.post().with($post);
                })
                .resource("/{id}/", |r| {
                    r.get().with($update);
                    r.post().with($post);
                })
        }
    };
}

/// Admin auth middleware.
#[derive(Default, Debug, Clone)]
struct AdminAuth;

impl Middleware<AppState> for AdminAuth {
    fn start(&self, req: &HttpRequest<AppState>) -> actix_web::Result<Started> {
        use actix_web::FromRequest;
        use actix_web_httpauth::extractors::{
            basic::{BasicAuth, Config},
            AuthenticationError,
        };

        let authenticator = req.state().admin_auth();
        let mut config = Config::default();
        config.realm(authenticator.realm());
        let auth_info = BasicAuth::from_request(&req, &config)?;

        if authenticator.is_authenticated(auth_info.username(), auth_info.password()) {
            Ok(Started::Done)
        } else {
            Err(AuthenticationError::from(config).into())
        }
    }
}

fn main() {
    let dotenv_result = match env::var_os("DOTENV") {
        Some(path) => {
            info!("Loading dotenv file {:?}", path);
            dotenv::from_filename(path)
        }
        None => {
            info!("Loading default dotenv file (`.env`)");
            dotenv::dotenv()
        }
    };
    match dotenv_result {
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

    // To provide safe default, this should not be `0.0.0.0:*`.
    const LISTEN_DEFAULT: &str = "localhost:8080";
    let listen = match env::var("LISTEN") {
        Ok(v) => v,
        Err(env::VarError::NotPresent) => LISTEN_DEFAULT.into(),
        Err(e) => {
            error!("Envvar `$LISTEN` has invalid value: {}", e);
            panic!("Envvar `$LISTEN` has invalid value: {}", e);
        }
    };

    let sys = actix::System::new("burning-pro-server");

    let database_url = env::var("DATABASE_URL").expect("`DATABASE_URL` envvar must be set");
    info!("Database URL: {}", database_url);
    let admin_auth = AdminAuthenticator::from_env(
        "Burning Pro admin web UI",
        "ADMIN_WEB_USER",
        "ADMIN_WEB_PASSWORD",
    )
    .expect("Failed to get admin web auth config");
    let app_state = AppStateBuilder::new()
        .database_url(database_url)
        .admin_auth(admin_auth)
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
                    .middleware(AdminAuth)
                    .resource("/", |r| r.with(admin::index))
                    .nested(
                        "/phrase",
                        regist_form_handler!(
                            admin::phrase::index,
                            admin::phrase::new,
                            admin::phrase::update,
                            admin::phrase::post
                        ),
                    )
                    .nested(
                        "/tag",
                        regist_form_handler!(
                            admin::tag::index,
                            admin::tag::new,
                            admin::tag::update,
                            admin::tag::post
                        ),
                    )
                    .nested(
                        "/person",
                        regist_form_handler!(
                            admin::person::index,
                            admin::person::new,
                            admin::person::update,
                            admin::person::post
                        ),
                    )
            })
            .scope("/request", |scope| {
                scope.resource("/phrase_app/", |r| r.with(admin::phrase_request::post))
            })
    })
    .bind(&listen)
    .unwrap_or_else(|e| {
        panic!("Failed to bind {}: {}", listen, e);
    })
    .start();

    info!("started server ({})", listen);

    let _ = sys.run();
}
