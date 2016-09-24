extern crate std;

use result::ListsError;
use result::ListsResult;
use std::io::Read;
use std::io::Write;

// TODO:
// - Time-based expiration
// - Space-based expiration
// - Multithreaded access
pub trait Cache {
    fn get(&mut self, namespace: &str, key: &str) -> Option<String>;
    fn put(&mut self, namespace: &str, key: &str, value: &str, expiration: ExpirationPolicy) -> ListsResult<()>;
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

//
// FileCache
//

pub struct FileCache {
    cache_dir: std::path::PathBuf,
    clock: std::sync::Arc<std::sync::Mutex<Clock>>,
}

impl FileCache {
    pub fn new<P: std::convert::AsRef<std::path::Path>>(cache_dir: P) -> FileCache {
        let cache_path = 
            std::path::PathBuf::from(cache_dir.as_ref());
        std::fs::create_dir_all(&cache_path).unwrap();
        return FileCache{
            cache_dir: cache_path,
            clock: std::sync::Arc::new(std::sync::Mutex::new(RealClock{})),
        };
    }

    fn sanitize(s: &str) -> String {
        return s.to_string().replace("/", "_").replace(".", "_");
    }

    fn cache_filename(&self, namespace: &str, key: &str) -> std::path::PathBuf {
        let mut path = self.cache_dir.clone();
        path.push(format!("{}:{}",
                          FileCache::sanitize(namespace),
                          FileCache::sanitize(key)));
        return path;
    }

    fn get_result(&mut self, namespace: &str, key: &str) -> ListsResult<String> {
        let cache_filename = self.cache_filename(namespace, key);

        if !cache_filename.exists() {
            return Err(ListsError::DoesNotExist);
        }

        let mut file = try!(std::fs::File::open(cache_filename));
        let mut data = String::new();
        try!(file.read_to_string(&mut data));

        return Ok(data);
    }
}

impl Cache for FileCache {
    fn get(&mut self, namespace: &str, key: &str) -> Option<String> {
        return match self.get_result(namespace, key) {
            Err(_) => None,
            Ok(data) => Some(data),
        }
    }

    fn put(&mut self, namespace: &str, key: &str, value: &str, expiration: ExpirationPolicy) -> ListsResult<()> {
        let cache_filename = self.cache_filename(namespace, key);
        let mut file = try!(std::fs::File::create(cache_filename));
        try!(file.write_all(value.as_bytes()));

        return Ok(());
    }
}

//
// Memory Cache
//

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

    fn put(&mut self, namespace: &str, key: &str, value: &str, policy: ExpirationPolicy) -> ListsResult<()> {
        let expiration = self.expiration(policy);
        self.data.insert(
            MemoryCache::map_key(namespace, key),
            MemoryCacheEntry{
                value: value.to_string(),
                expiration: expiration,
            }
        );
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    
    use super::Cache;
    use super::FileCache;
    use super::ExpirationPolicy;
    use super::FakeClock;
    use super::MemoryCache;
    
    const CACHE_DIR: &'static str = "/tmp/cache.test/";

    #[test]
    fn simple_put_get_memory() {
        let mut cache = MemoryCache::new();
        cache.put("ns", "key", "val", ExpirationPolicy::Never).unwrap();
        assert_eq!("val".to_string(), cache.get("ns", "key").unwrap());
    }

    #[test]
    fn simple_put_get_file() {
        let mut cache = FileCache::new(CACHE_DIR);
        cache.put("ns", "key", "val", ExpirationPolicy::Never).unwrap();
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
        
        cache.put("ns", "key", "val", ExpirationPolicy::After(two_seconds)).unwrap();

        clock.lock().unwrap().time =
            start_time + std::time::Duration::new(1, 0);
        assert_eq!("val".to_string(), cache.get("ns", "key").unwrap());

        clock.lock().unwrap().time =
            start_time + std::time::Duration::new(3, 0);
        assert_eq!(None, cache.get("ns", "key"));
    }
}
