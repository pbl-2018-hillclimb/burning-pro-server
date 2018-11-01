//! Server app.

mod auth;
pub mod state;

pub use self::auth::AdminAuthenticator;
pub use self::state::{AppState, AppStateBuilder};
