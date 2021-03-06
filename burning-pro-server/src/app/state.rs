//! Server app state.

use std::error;
use std::sync::Arc;

use actix::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2;
use tera::Tera;

use app::AdminAuthenticator;
use db::DbExecutor;

/// Application-wide states.
#[derive(Clone)]
pub struct AppState {
    /// Address of DB executor actor.
    db: Addr<DbExecutor>,
    /// tera(template engine) template.
    template: Arc<Tera>,
    /// Admin authenticator.
    admin_auth: AdminAuthenticator,
}

impl AppState {
    /// Returns an address for DB executor actor.
    pub fn db(&self) -> &Addr<DbExecutor> {
        &self.db
    }

    /// Returns a tera template.
    pub fn template(&self) -> &Arc<Tera> {
        &self.template
    }

    /// Returns an admin authenticator.
    pub fn admin_auth(&self) -> &AdminAuthenticator {
        &self.admin_auth
    }
}

/// `AppState` builder.
///
/// This type should be used to initialize `AppState`.
///
/// `AppState` may have many types of fields and may require complex
/// initialization.
/// `AppStateBuilder` makes it easy for developers to set variety of fields and
/// does some initialization if necessary.
#[derive(Default, Debug, Clone)]
pub struct AppStateBuilder {
    /// Database URL.
    database_url: Option<String>,
    /// Admin authenticator.
    admin_auth: Option<AdminAuthenticator>,
}

impl AppStateBuilder {
    /// Creates a new `AppStateBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets `database_url` field with the given database URL.
    ///
    /// This setter does not do any DB operation, such as I/O operations and
    /// connection pool initialization.
    pub fn database_url<S: Into<String>>(self, database_url: S) -> Self {
        Self {
            database_url: Some(database_url.into()),
            ..self
        }
    }

    /// Sets the given authenticator to `admin_auth` field.
    pub fn admin_auth(self, admin_auth: AdminAuthenticator) -> Self {
        Self {
            admin_auth: Some(admin_auth),
            ..self
        }
    }

    /// Builds the `AppState`.
    ///
    /// This method will ensure that all of required field values are set and
    /// some objects / resources are correctly initialized.
    ///
    /// If the builder lacks required field values or resource initializations
    /// fail, then it will return `Err(_)`.
    pub fn build(self) -> Result<AppState, Box<error::Error + Send + Sync>> {
        let db = {
            let database_url = self
                .database_url
                .ok_or("`database_url` field is required but not set")?;
            let manager = ConnectionManager::<SqliteConnection>::new(database_url);
            let pool = r2d2::Pool::builder().build(manager)?;
            SyncArbiter::start(3, move || DbExecutor::new(pool.clone()))
        };
        let template = {
            let glob = concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*");
            Arc::new(compile_templates!(glob))
        };
        let admin_auth = self
            .admin_auth
            .ok_or("`admin_auth` field is required but not set")?;
        Ok(AppState {
            db,
            template,
            admin_auth,
        })
    }
}
