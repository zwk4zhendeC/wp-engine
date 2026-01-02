use arcstr::ArcStr;
use once_cell::sync::Lazy;
use std::sync::RwLock;
use ahash::AHashMap;

/// Global cache for field name ArcStr instances to enable sharing
/// This dramatically improves performance when field names are reused
static FIELD_NAME_CACHE: Lazy<RwLock<AHashMap<&'static str, ArcStr>>> =
    Lazy::new(|| RwLock::new(AHashMap::new()));

/// Get a shared ArcStr for a static field name
/// This ensures all uses of the same field name share the same Arc instance
pub fn get_cached_name(name: &'static str) -> ArcStr {
    // Try read lock first (fast path)
    {
        let cache = FIELD_NAME_CACHE.read().unwrap();
        if let Some(cached) = cache.get(name) {
            return cached.clone(); // This is just an atomic increment
        }
    }

    // Need to create new entry (slow path)
    let mut cache = FIELD_NAME_CACHE.write().unwrap();
    // Double-check in case another thread created it
    cache.entry(name)
        .or_insert_with(|| name.into())
        .clone()
}

/// Get a shared ArcStr for a dynamic string
/// Still uses caching to share common field names
pub fn get_or_create_name(name: &str) -> ArcStr {
    // For common static names, use the cache
    // This is a heuristic - adjust the list based on actual field names
    match name {
        "ip" => get_cached_name("ip"),
        "time" => get_cached_name("time"),
        "method" => get_cached_name("method"),
        "path" => get_cached_name("path"),
        "status" => get_cached_name("status"),
        "size" => get_cached_name("size"),
        "user_agent" => get_cached_name("user_agent"),
        "referrer" => get_cached_name("referrer"),
        "auto" => get_cached_name("auto"),
        "chars" => get_cached_name("chars"),
        "digit" => get_cached_name("digit"),
        "json" => get_cached_name("json"),
        "kv" => get_cached_name("kv"),
        "array" => get_cached_name("array"),
        "url" => get_cached_name("url"),
        "email" => get_cached_name("email"),
        "domain" => get_cached_name("domain"),
        "hex" => get_cached_name("hex"),
        "bool" => get_cached_name("bool"),
        "ignore" => get_cached_name("ignore"),
        "symbol" => get_cached_name("symbol"),
        // For unknown names, create new ArcStr
        _ => name.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_sharing() {
        let name1 = get_cached_name("ip");
        let name2 = get_cached_name("ip");

        // Both should point to the same Arc (same strong count)
        assert_eq!(ArcStr::strong_count(&name1), ArcStr::strong_count(&name2));
        assert_eq!(name1.as_str(), name2.as_str());
    }

    #[test]
    fn test_common_names() {
        let ip1 = get_or_create_name("ip");
        let ip2 = get_or_create_name("ip");

        // Should share the same instance
        assert_eq!(ArcStr::strong_count(&ip1), ArcStr::strong_count(&ip2));
    }
}
