extern crate kuchiki;
extern crate regex;
extern crate rustc_serialize;
extern crate std;

use self::kuchiki::traits::TendrilSink;
use cache::Cache;
use cache::ExpirationPolicy;
use cache::FileCache;
use result::ListsError;
use result::ListsResult;
use scrape::HyperHttpClient;
use scrape::Scraper;
use std::ops::Deref;

pub struct StreetEasyClient {
    scraper: Scraper,
    cache: FileCache,

    price_regex: regex::Regex,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct ListingData {
    pub name: String,
    pub price_usd: i32,
}

impl ListingData {
    fn to_json(&self) -> String {
        return rustc_serialize::json::encode(self).unwrap();
    }

    fn from_json(s: &str) -> ListsResult<ListingData> {
        return Ok(try!(rustc_serialize::json::decode(s)));
    }
}

const PARSE_CACHE_NAMESPACE: &'static str = "PARSE";

impl StreetEasyClient {
    pub fn new() -> StreetEasyClient {
        let cache_dir = "/home/mrjones/lists.cache/".to_string();
        std::fs::create_dir_all(cache_dir.clone()).unwrap();

        return StreetEasyClient{
            scraper: Scraper::new(
                std::sync::Arc::new(std::sync::Mutex::new(
                    HyperHttpClient::new())),
                Box::new(FileCache::new(&cache_dir))),
            cache: FileCache::new(&cache_dir),
            price_regex: regex::Regex::new("(\\$[0-9,]+)").unwrap(),
        }
    }

    fn parse_cache_lookup(&self, url: &str) -> ListsResult<ListingData> {
        let data = self.cache.get(PARSE_CACHE_NAMESPACE, url);
        if !data.is_some() {
            return Err(ListsError::DoesNotExist);
        }
        return ListingData::from_json(data.unwrap().as_str());
    }

    fn parse_cache_save(&self, url: &str, listing: &ListingData) -> ListsResult<()> {
        let data = listing.to_json();
        return self.cache.put(PARSE_CACHE_NAMESPACE, url, &data,
                       ExpirationPolicy::After(
                           std::time::Duration::from_secs(86400)));
    }
    
    pub fn lookup_listing(&self, url: &str) -> ListsResult<ListingData> {
        let cache_entry = self.parse_cache_lookup(url);
        if cache_entry.is_ok() {
            return cache_entry;
        }
        
        let page = try!(self.scraper.fetch(&url));
        let document = kuchiki::parse_html().one(page);

        let mut price = -1;
        let mut name = String::new();
        
        // element: kuchiki::NodeDataRef
        for element in document.select(".building-title a").unwrap() {
            let node: &kuchiki::NodeRef = element.as_node();
            for child in node.children() {
                match child.data() {
                    &kuchiki::NodeData::Text(ref val) => {
                        name.push_str(val.borrow().deref());
                    }
                    _ => (),
                }
            }
        }
        for element in document.select(".details_info_price .price").unwrap() {
            let node: &kuchiki::NodeRef = element.as_node();
            for child in node.children() {
                match child.data() {
                    &kuchiki::NodeData::Text(ref val) => {
                        let val_ref = val.borrow();
                        let text = val_ref.deref();
                        for capture in self.price_regex.captures_iter(text) {
                            let formatted = capture.at(1).unwrap();
                            let unformatted = formatted.replace(",", "").replace("$", "");
                            price = unformatted.parse::<i32>().unwrap();
                        }
                            
                    },
                    _ => (),
                }
            }
        }

        let listing = ListingData{
            price_usd: price,
            name: name,
        };

        self.parse_cache_save(url, &listing).ok();
        return Ok(listing);
    }
}

#[cfg(test)]
mod tests {
    use super::StreetEasyClient;

    #[test]
    fn parse_price() {
        let client = StreetEasyClient::new();
        let listing = client.lookup_listing("http://streeteasy.com/sale/1241009");
        assert_eq!(2350000, listing.unwrap().price_usd);
    }
}
