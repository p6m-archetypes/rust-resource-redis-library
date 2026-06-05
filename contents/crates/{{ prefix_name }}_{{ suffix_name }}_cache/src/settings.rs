use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheSettings {
    pub url: String,
    pub pool_size: Option<usize>,
}

impl Default for CacheSettings {
    fn default() -> Self {
        Self {
            url: url_from_env(),
            pool_size: None,
        }
    }
}

// When deployed via the platform operator, individual connection fields are
// injected as CACHE_HOST / CACHE_PORT / CACHE_USERNAME / CACHE_PASSWORD.
// Assemble a URL from those if present; fall back to a local dev default
// that can also be overridden via APP_CACHE__URL.
fn url_from_env() -> String {
    if let (Ok(host), Ok(port)) = (
        std::env::var("CACHE_HOST"),
        std::env::var("CACHE_PORT"),
    ) {
        let auth = match (std::env::var("CACHE_USERNAME"), std::env::var("CACHE_PASSWORD")) {
            (Ok(user), Ok(pass)) => format!("{}:{}@", user, pass),
            _ => String::new(),
        };
        return format!("redis://{}{}:{}", auth, host, port);
    }
    "redis://localhost:6379".to_string()
}
