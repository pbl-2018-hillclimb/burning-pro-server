//! Burning-pro server.
#![warn(missing_docs)]

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::env;


/// Setup global logger.
fn setup_logger() {
    const DEFAULT_LOG_LEVEL: &str = "trace";
    let underscored_name = env!("CARGO_PKG_NAME").replace('-', "_");
    let defval = format!("{}={}", underscored_name, DEFAULT_LOG_LEVEL);

    let newval = match env::var("RUST_LOG") {
        Ok(v) => format!("{},{}", defval, v),
        Err(_) => defval,
    };
    env::set_var("RUST_LOG", &newval);

    pretty_env_logger::init();

    trace!("RUST_LOG={}", newval);
}


fn main() {
    setup_logger();

    info!(
        "{}, version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
}
