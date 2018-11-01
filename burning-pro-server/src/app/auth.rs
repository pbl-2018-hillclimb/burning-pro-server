//! Authenticators.

use std::env;
use std::error;

/// Admin authenticator.
#[derive(Debug, Clone)]
pub struct AdminAuthenticator {
    /// Realm.
    realm: String,
    /// User name.
    user: String,
    /// Password.
    password: Option<String>,
}

impl AdminAuthenticator {
    /// Creates a new admin authenticator.
    pub fn new<R, U, P>(realm: R, user: U, password: P) -> Self
    where
        R: Into<String>,
        U: Into<String>,
        P: Into<Option<String>>,
    {
        Self {
            realm: realm.into(),
            user: user.into(),
            password: password.into(),
        }
    }

    /// Creates a new admin authenticator from the environment variables.
    pub fn from_env<R>(
        realm: R,
        user_env: &str,
        password_env: &str,
    ) -> Result<Self, Box<error::Error + Send + Sync>>
    where
        R: Into<String>,
    {
        let user =
            env::var(user_env).map_err(|e| format!("`{}` envvar must be set: {}", user_env, e))?;
        let password = env::var(password_env)
            .map_err(|e| format!("`{}` envvar must be set: {}", password_env, e))?;
        Ok(Self::new(realm.into(), user, Some(password)))
    }

    /// Returns the realm.
    pub fn realm(&self) -> &str {
        &self.realm
    }

    /// Checks whether the authentication should be success.
    pub fn is_authenticated(&self, user: &str, password: Option<&str>) -> bool {
        self.user == user && self.password.as_ref().map(|s| s.as_ref()) == password
    }
}
