extern crate std;

// TODO:
// - Time-based expiration
// - Space-based expiration
// - Multithreaded access
pub trait Cache {
    fn get(&mut self, namespace: &str, key: &str) -> Option<String>;
    fn put(&mut self, namespace: &str, key: &str, value: &str, expiration: ExpirationPolicy);
}

pub enum ExpirationPolicy {
    Never,
    After(std::time::Duration),
}

trait Clock {
    fn now(&self) -> std::time::Instant;
}

struct RealClock;
impl Clock for RealClock {
    fn now(&self) -> std::time::Instant {
        return std::time::Instant::now();
    }
}

struct FakeClock {
    time: std::time::Instant,
}
impl Clock for FakeClock {
    fn now(&self) -> std::time::Instant {
        return self.time;
    }
}

struct MemoryCacheEntry {
    value: String,
    expiration: Option<std::time::Instant>,
}

pub struct MemoryCache {
    data: std::collections::HashMap<String, MemoryCacheEntry>,
    clock: std::sync::Arc<std::sync::Mutex<Clock>>,
}

impl MemoryCache {
    fn new() -> MemoryCache {
        return MemoryCache {
            data: std::collections::HashMap::new(),
            clock: std::sync::Arc::new(std::sync::Mutex::new(RealClock{})),
        }
    }
    
    fn map_key(namespace: &str, key: &str) -> String {
        // TODO(mrjones): support namespaces/keys with ":"
        assert!(!namespace.contains(":"));
        assert!(!key.contains(":"));
        return format!("{}:{}", namespace, key);
    }

    fn expiration(&self, expiration: ExpirationPolicy) -> Option<std::time::Instant> {
        match expiration {
            ExpirationPolicy::Never => return None,
            ExpirationPolicy::After(duration) => {
                return Some(self.clock.lock().unwrap().now() + duration)
            },
        }
    }
}

impl Cache for MemoryCache {
    fn get(&mut self, namespace: &str, key: &str) -> Option<String> {
        let key = MemoryCache::map_key(namespace, key);
        match self.data.get(&key) {
            None => return None,
            Some(ref entry) => {
                if entry.expiration.is_some() &&
                    entry.expiration.unwrap() < self.clock.lock().unwrap().now() {
                        // TODO: Clean up expired entries                    
                        return None;
                }
                return Some(entry.value.clone());
            },
        }
    }

    fn put(&mut self, namespace: &str, key: &str, value: &str, policy: ExpirationPolicy) {
        let expiration = self.expiration(policy);
        self.data.insert(
            MemoryCache::map_key(namespace, key),
            MemoryCacheEntry{
                value: value.to_string(),
                expiration: expiration,
            });
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    
    use super::Cache;
    use super::ExpirationPolicy;
    use super::FakeClock;
    use super::MemoryCache;
    
    #[test]
    fn simple_put_get() {
        let mut cache = MemoryCache::new();
        cache.put("ns", "key", "val", ExpirationPolicy::Never);
        assert_eq!("val".to_string(), cache.get("ns", "key").unwrap());
    }

    #[test]
    fn time_based_expiration() {
        let start_time = std::time::Instant::now();
        let clock = std::sync::Arc::new(std::sync::Mutex::new(
            FakeClock{time: start_time}));
        let mut cache = MemoryCache{
            data: std::collections::HashMap::new(),
            clock: clock.clone(),
        };

        let two_seconds = std::time::Duration::new(2, 0);
        
        cache.put("ns", "key", "val", ExpirationPolicy::After(two_seconds));

        clock.lock().unwrap().time =
            start_time + std::time::Duration::new(1, 0);
        assert_eq!("val".to_string(), cache.get("ns", "key").unwrap());

        clock.lock().unwrap().time =
            start_time + std::time::Duration::new(3, 0);
        assert_eq!(None, cache.get("ns", "key"));
    }
}
