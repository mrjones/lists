extern crate hyper;
extern crate std;

use cache::Cache;
use cache::ExpirationPolicy;
use result::ListsResult;
use std::io::Read;

pub trait HttpClient {
    fn get(&mut self, url: &str) -> ListsResult<String>;
}

pub struct HyperHttpClient {
    client: hyper::client::Client,
}

impl HttpClient for HyperHttpClient {
    fn get(&mut self, url: &str) -> ListsResult<String> {
        println!("GET {}", url);
        let mut response = try!(self.client.get(url).send());
        let mut body = String::new();
        try!(response.read_to_string(&mut body));
        return Ok(body);
    }
}

impl HyperHttpClient {
    pub fn new() -> HyperHttpClient {
        return HyperHttpClient {
            client: hyper::client::Client::new(),
        }
    }
}

const CACHE_NAMESPACE: &'static str = "scraper";

pub struct Scraper {
    client: std::sync::Arc<std::sync::Mutex<HttpClient + std::marker::Send>>,
    cache: Box<Cache + std::marker::Send + std::marker::Sync>,
}

impl Scraper {
    pub fn new(client: std::sync::Arc<std::sync::Mutex<HttpClient + std::marker::Send>>,
               cache: Box<Cache + std::marker::Send + std::marker::Sync>) -> Scraper {
        return Scraper{
            client: client,
            cache: cache,
        };
    }
    
    pub fn fetch(&self, url: &str) -> ListsResult<String> {
        match self.cache.get(CACHE_NAMESPACE, url) {
            Some(data) => return Ok(data),
            None => (),
        }

        let body = try!(self.client.lock().unwrap().get(url));

        self.cache.put(CACHE_NAMESPACE, url, &body,
                       ExpirationPolicy::After(
                           std::time::Duration::from_secs(86400))).ok();
        return Ok(body);
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    
    use super::HttpClient;

    use cache::FileCache;
    use std::ops::DerefMut;
    use result::ListsError;
    use result::ListsResult;
    
    const CACHE_DIR: &'static str = "/tmp/scrapecache/";
    
    fn populate_pages(client: &mut FakeHttpClient) {
        client.add_page(
            "http://www.google.com".to_string(),
            "It's google!".to_string());
    }
    
    #[test]
    fn simple_get() {
        let mut client = FakeHttpClient::new();
        populate_pages(&mut client);

        assert_eq!("It's google!".to_string(),
                   client.get("http://www.google.com").unwrap());
    }

    #[test]
    fn simple_scrape() {
        std::fs::remove_dir_all(CACHE_DIR).ok();

        let client = std::sync::Arc::new(std::sync::Mutex::new(
            FakeHttpClient::new()));
        populate_pages(client.lock().unwrap().deref_mut());

        {
            let scraper = super::Scraper::new(
                client.clone(),
                Box::new(FileCache::new(CACHE_DIR)));
            assert_eq!("It's google!".to_string(),
                       scraper.fetch("http://www.google.com").unwrap());
        }
    }

    #[test]
    fn scrapes_are_cached() {
        std::fs::remove_dir_all(CACHE_DIR).ok();

        let client = std::sync::Arc::new(std::sync::Mutex::new(
            FakeHttpClient::new()));
        populate_pages(client.lock().unwrap().deref_mut());

        {
            let scraper = super::Scraper::new(
                client.clone(),
                Box::new(FileCache::new(CACHE_DIR)));

            assert_eq!("It's google!".to_string(),
                       scraper.fetch("http://www.google.com").unwrap());

            assert_eq!("It's google!".to_string(),
                       scraper.fetch("http://www.google.com").unwrap());
        }

        assert_eq!(1, client.lock().unwrap().fetch_count());
    }

    struct FakeHttpClient {
        pages: std::collections::HashMap<String, String>,
        fetches: i32,
    }

    impl HttpClient for FakeHttpClient {
        fn get(&mut self, url: &str) -> ListsResult<String> {
            self.fetches = self.fetches + 1;
            match self.pages.get(url) {
                Some(body) => Ok(body.clone()),
                None => Err(ListsError::DoesNotExist),
            }
        }
    }

    impl FakeHttpClient {
        fn new() -> FakeHttpClient {
            return FakeHttpClient{
                pages: std::collections::HashMap::new(),
                fetches: 0,
            }
        }
    
        fn add_page(&mut self, url: String, body: String) {
            self.pages.insert(url, body);
        }

        fn fetch_count(&self) -> i32 {
            return self.fetches;
        }
    }
}
