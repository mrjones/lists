extern crate hyper;
extern crate std;

use result::ListsError;
use result::ListsResult;
use std::io::Read;
use std::io::Write;

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

pub struct Scraper {
    client: std::sync::Arc<std::sync::Mutex<HttpClient + std::marker::Send>>,
    cache_dir: String,
}

impl Scraper {
    pub fn new(client: std::sync::Arc<std::sync::Mutex<HttpClient + std::marker::Send>>,
               cache_dir: &str) -> Scraper {
        std::fs::create_dir_all(cache_dir).unwrap();
        return Scraper{
            client: client,
            cache_dir: cache_dir.to_string(),
        };
    }

    fn cache_filename(&self, url: &str) -> std::path::PathBuf {
        let mut path = std::path::PathBuf::from(self.cache_dir.clone());
        path.push(url.to_string().replace("/", "_").replace(".", "_"));
        return path;
    }

    fn get_age(cache_filename: &std::path::Path) -> ListsResult<std::time::Duration> {
        let md = try!(std::fs::metadata(cache_filename));
        let mtime = try!(md.modified());
        return match std::time::SystemTime::now().duration_since(mtime) {
            Ok(age) => Ok(age),
            Err(e) => Err(ListsError::Unknown(format!("{}", e))),
        }
    }
    
    fn has_recent_cache(&self, cache_filename: &std::path::Path) -> bool {
        let maybe_age = Scraper::get_age(cache_filename);
        return maybe_age.is_ok() &&
            maybe_age.unwrap() < std::time::Duration::new(60 * 60, 0);
    }
    
    pub fn fetch(&self, url: &str) -> ListsResult<String> {
        let cache_filename = self.cache_filename(url);

        if self.has_recent_cache(&cache_filename) {
            println!("Using cache {}", cache_filename.as_path().to_str().unwrap());
            let mut cache_file = try!(std::fs::File::open(cache_filename));
            let mut body = String::new();
            try!(cache_file.read_to_string(&mut body));
            return Ok(body);
        }
        
        let body = try!(self.client.lock().unwrap().get(url));
        let mut cache_file = try!(std::fs::File::create(
            cache_filename));

        try!(cache_file.write_all(body.as_bytes()));
        
        return Ok(body);
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    
    use super::HttpClient;
    use super::FakeHttpClient;

    use std::ops::DerefMut;

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
            let scraper = super::Scraper::new(client.clone(), CACHE_DIR);
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
            let scraper = super::Scraper::new(client.clone(), CACHE_DIR);

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
