pub mod settings;

use anyhow::Result;
use deadpool_redis::{Config, Pool, Runtime};
use settings::CacheSettings;

pub use deadpool_redis::Pool as CachePool;

pub async fn connect(settings: &CacheSettings) -> Result<Pool> {
    let cfg = Config::from_url(&settings.url);
    let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
    tracing::info!("Redis pool connected to {}", settings.url);
    Ok(pool)
}
