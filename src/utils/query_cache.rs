use std::sync::Mutex;

use once_cell::sync::OnceCell;

use crate::utils::format::Format;

/// Provides caching to make sure we only create one instance of the inputs of each subscription
static CACHE: OnceCell<Mutex<Option<(String, String, Format)>>> = OnceCell::new();

pub fn init_global() {
    let cache = Mutex::new(None);
    if CACHE.set(cache).is_err() {
        panic!("Cache was initialized twice")
    }
}

pub fn replace_global(
    cache: Option<(String, String, Format)>,
) -> Result<Option<(String, String, Format)>, String> {
    let cache_ref = CACHE.get().expect("Cache has not been initialized");

    let mut cache_lock = match cache_ref.lock() {
        Ok(cache_lock) => cache_lock,
        Err(_) => return Err("Could not obtain lock to cache".to_owned()),
    };

    let old_value = match cache {
        None => cache_lock.take(),
        Some(old_value) => cache_lock.replace(old_value),
    };

    Ok(old_value)
}
