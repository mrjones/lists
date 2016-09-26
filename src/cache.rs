extern crate rustc_serialize;
extern crate std;

use result::ListsError;
use result::ListsResult;
use std::io::Read;
use std::io::Write;
use std::ops::DerefMut;

// TODO:
// - Time-based expiration
// - Space-based expiration
// - Multithreaded access
pub trait Cache {
    fn get(&self, namespace: &str, key: &str) -> Option<String>;
    fn put(&self, namespace: &str, key: &str, value: &str, expiration: ExpirationPolicy) -> ListsResult<()>;
}

pub enum ExpirationPolicy {
    Never,
    After(std::time::Duration),
}

trait Clock {
    fn now(&self) -> std::time::SystemTime;
}

struct RealClock;
impl Clock for RealClock {
    fn now(&self) -> std::time::SystemTime {
        return std::time::SystemTime::now();
    }
}

struct FakeClock {
    time: std::time::SystemTime,
}
impl FakeClock {
    fn new() -> FakeClock {
        return FakeClock{
            time: std::time::SystemTime::now(),
        };
    }
}
impl Clock for FakeClock {
    fn now(&self) -> std::time::SystemTime {
        return self.time;
    }
}

//
// FileCache
//

#[derive(RustcEncodable, RustcDecodable)]
struct FileCacheOnDiskFormat {
    value: String,
    has_expiration_time: bool,
    expiration_time_seconds: u64,
}

pub struct FileCache {
    cache_dir: std::path::PathBuf,
    clock: std::sync::Arc<std::sync::Mutex<Clock + std::marker::Send>>,
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

    fn get_result(&self, namespace: &str, key: &str) -> ListsResult<String> {
        let cache_filename = self.cache_filename(namespace, key);

        if !cache_filename.exists() {
            return Err(ListsError::DoesNotExist);
        }
        
        let mut file = try!(std::fs::File::open(&cache_filename));
        let mut data = String::new();
        try!(file.read_to_string(&mut data));

        let record : FileCacheOnDiskFormat =
            try!(rustc_serialize::json::decode(&data));

        if record.has_expiration_time {
            let now = self.clock.lock().unwrap().now();
            let expiration = std::time::UNIX_EPOCH +
                std::time::Duration::from_secs(record.expiration_time_seconds);
            if expiration < now {
                std::fs::remove_file(&cache_filename).ok();
                return Err(ListsError::DoesNotExist);
            }
        }
        
        return Ok(record.value);
    }
}

impl Cache for FileCache {
    fn get(&self, namespace: &str, key: &str) -> Option<String> {
        return match self.get_result(namespace, key) {
            Err(_) => None,
            Ok(data) => Some(data),
        }
    }

    fn put(&self, namespace: &str, key: &str, value: &str, expiration: ExpirationPolicy) -> ListsResult<()> {
        let cache_filename = self.cache_filename(namespace, key);
        let mut file = try!(std::fs::File::create(cache_filename));

        let record = match expiration {
            ExpirationPolicy::Never => FileCacheOnDiskFormat {
                value: value.to_string(),
                has_expiration_time: false,
                expiration_time_seconds: 0,
            },
            ExpirationPolicy::After(duration) => {
                let now = self.clock.lock().unwrap().now();
                let expiration = now + duration;
                let epoch_duration = try!(expiration.duration_since(
                    std::time::UNIX_EPOCH));
                
                FileCacheOnDiskFormat {
                    value: value.to_string(),
                    has_expiration_time: true,
                    expiration_time_seconds: epoch_duration.as_secs(),
                }
            },
        };

        let encoded : String = try!(rustc_serialize::json::encode(&record));
        try!(file.write_all(encoded.as_bytes()));

        return Ok(());
    }
}

//
// Memory Cache
//

struct MemoryCacheEntry {
    value: String,
    expiration: Option<std::time::SystemTime>,
}

pub struct MemoryCache {
    data: std::sync::Mutex<std::collections::HashMap<String, MemoryCacheEntry>>,
    clock: std::sync::Arc<std::sync::Mutex<Clock>>,
}

impl MemoryCache {
    fn new() -> MemoryCache {
        return MemoryCache {
            data: std::sync::Mutex::new(std::collections::HashMap::new()),
            clock: std::sync::Arc::new(std::sync::Mutex::new(RealClock{})),
        }
    }
    
    fn map_key(namespace: &str, key: &str) -> String {
        // TODO(mrjones): support namespaces/keys with ":"
        assert!(!namespace.contains(":"));
        assert!(!key.contains(":"));
        return format!("{}:{}", namespace, key);
    }

    fn expiration(&self, expiration: ExpirationPolicy) -> Option<std::time::SystemTime> {
        match expiration {
            ExpirationPolicy::Never => return None,
            ExpirationPolicy::After(duration) => {
                return Some(self.clock.lock().unwrap().now() + duration)
            },
        }
    }
}

impl Cache for MemoryCache {
    fn get(&self, namespace: &str, key: &str) -> Option<String> {
        let key = MemoryCache::map_key(namespace, key);
        match self.data.lock().unwrap().get(&key) {
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

    fn put(&self, namespace: &str, key: &str, value: &str, policy: ExpirationPolicy) -> ListsResult<()> {
        let expiration = self.expiration(policy);
        self.data.lock().unwrap().deref_mut().insert(
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
    use super::Clock;
    use super::FileCache;
    use super::ExpirationPolicy;
    use super::FakeClock;
    use super::MemoryCache;
    
    const CACHE_DIR: &'static str = "/tmp/cache.test/";
    const MEMORY_LABEL: &'static str = "MEMORY_CACHE";
    const FILE_LABEL: &'static str = "FILE_CACHE";
    
    struct CacheHandle {
        cache: Box<Cache>,
        clock: std::sync::Arc<std::sync::Mutex<FakeClock>>,
        debug_label: &'static str,
    }

    fn new_memory_cache() -> CacheHandle {
        let clock = std::sync::Arc::new(std::sync::Mutex::new(FakeClock::new()));
        return CacheHandle{
            cache: Box::new(MemoryCache{
                data: std::sync::Mutex::new(std::collections::HashMap::new()),
                clock: clock.clone(),
            }),
            clock: clock,
            debug_label: MEMORY_LABEL,
        };
    }

    fn new_file_cache() -> CacheHandle {
        let clock = std::sync::Arc::new(std::sync::Mutex::new(FakeClock::new()));
        return CacheHandle{
            cache: Box::new(FileCache{
                cache_dir: std::path::PathBuf::from(CACHE_DIR),
                clock: clock.clone(),
            }),
            clock: clock,
            debug_label: FILE_LABEL,
        }        
    }

    fn all_caches() -> std::vec::Vec<CacheHandle> {
        return vec![new_memory_cache(), new_file_cache()];
    }

    #[test]
    fn simple_put_get() {
        for handle in &mut all_caches() {
            println!("== {} ==", handle.debug_label);
            let cache = &mut handle.cache;
            cache.put("ns", "key", "val", ExpirationPolicy::Never)
                .expect(handle.debug_label);
            assert_eq!("val".to_string(),
                       cache.get("ns", "key").expect(handle.debug_label));
        }
    }

    #[test]
    fn overwrite() {
        for handle in &mut all_caches() {
            println!("== {} ==", handle.debug_label);
            let cache = &mut handle.cache;
            cache.put("ns", "key", "val1", ExpirationPolicy::Never)
                .expect(handle.debug_label);
            cache.put("ns", "key", "val2", ExpirationPolicy::Never)
                .expect(handle.debug_label);
            assert_eq!("val2".to_string(),
                       cache.get("ns", "key").expect(handle.debug_label));
        }
    }

    #[test]
    fn time_based_expiration() {
        for handle in &mut all_caches() {
            println!("== {} ==", handle.debug_label);
            let cache = &mut handle.cache;
            let clock = &mut handle.clock;
            let start_time = clock.lock().unwrap().now();

            let two_seconds = std::time::Duration::new(2, 0);
        
            cache.put("ns", "key", "val", ExpirationPolicy::After(two_seconds)).unwrap();

            clock.lock().unwrap().time =
                start_time + std::time::Duration::new(1, 0);
            assert_eq!("val".to_string(), cache.get("ns", "key").unwrap());

            clock.lock().unwrap().time =
                start_time + std::time::Duration::new(3, 0);
            assert_eq!(None, cache.get("ns", "key"),
                       "{}: Should have expired item.", handle.debug_label);
        }
    }
}
