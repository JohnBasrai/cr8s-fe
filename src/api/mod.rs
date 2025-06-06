pub mod crates;
pub mod rustaceans;
pub mod user;

use lazy_static::lazy_static;

lazy_static! {
    static ref APP_BASE: String = {
        let host = std::env::var("ROCKET_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("ROCKET_PORT").unwrap_or_else(|_| "8000".to_string());

        format!("http://{host}:{port}/cr8s")
    };
}

pub fn app_base() -> &'static str {
    &APP_BASE
}
