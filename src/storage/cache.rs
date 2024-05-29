mod core;
mod statics;

pub use core::*;
pub use statics::*;

use moka::future::Cache as MokCache;

pub struct Cache<V: Send + Sync + Clone>
where
    V: Send + Sync + Clone,
{
    inner: MokCache<String, V>,
}

impl<V> Cache<V>
where
    V: Send + Sync + Clone + 'static,
{
    fn new(time_to_live: u64, time_to_idle: u64, max_capacity: Option<u64>) -> Self {
        let mut cache_builder = MokCache::builder()
            .time_to_idle(std::time::Duration::from_secs(time_to_live))
            .time_to_idle(std::time::Duration::from_secs(time_to_idle));

        if let Some(capacity) = max_capacity {
            cache_builder = cache_builder.max_capacity(capacity * 1024 * 1024);
        }

        Self {
            inner: cache_builder.build(),
        }
    }

    pub async fn push(&self, key: String, val: V) {
        self.inner.insert(key, val).await;
    }

    pub async fn get(&self, key: &str) -> Option<V> {
        self.inner.get(key).await
    }
}
