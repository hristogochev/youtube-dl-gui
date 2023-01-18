use crate::utils::format::Format;
use anyhow::{anyhow, bail, Context, Result};
use once_cell::sync::OnceCell;
use std::sync::Mutex;

/// Provides caching to make sure we only create one instance of the inputs of each subscription
static CACHE: OnceCell<Mutex<Option<(String, String, Format)>>> = OnceCell::new();

pub fn init_global() -> Result<()> {
    if !is_set_global() {
        let cache = Mutex::new(None);
        return CACHE
            .set(cache)
            .map_err(|_| anyhow!("Cache was initialized more than once"));
    }
    Ok(())
}

fn is_set_global() -> bool {
    CACHE.get().is_some()
}

pub fn replace_global(
    cache: Option<(String, String, Format)>,
) -> Result<Option<(String, String, Format)>> {
    let cache_ref = CACHE
        .get()
        .with_context(|| "Cache has not been initialized")?;

    let mut cache_lock = match cache_ref.lock() {
        Ok(cache_lock) => cache_lock,
        Err(_) => bail!("Could not obtain lock to cache"),
    };

    let old_value = match cache {
        None => cache_lock.take(),
        Some(old_value) => cache_lock.replace(old_value),
    };

    Ok(old_value)
}
