extern crate chrono;
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

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct OpenHouse {
    pub info: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct ListingData {
    pub name: String,
    pub price_usd: i32,
    pub open_houses: Vec<OpenHouse>,
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
        let cache_dir = "./lists.cache/".to_string();
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
            println!("streeteasy::lookup_listing - hit cache");
            return cache_entry;
        }

        println!("streeteasy::lookup_listing - generating data");
        let page = try!(self.scraper.fetch(&url));
        let document = kuchiki::parse_html().one(page);
        
        let mut price = -1;
        let mut name = String::new();
        let mut open_houses = vec![];

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

        for element in document.select(".details_info").unwrap() {
            let node: &kuchiki::NodeRef = element.as_node();
            let mut is_open_house = false;
            for child in node.children() {
                match child.data() {
                    &kuchiki::NodeData::Element(ref element_data) => {
                        if element_data.name.local.as_ref() == "h6" {
                            for sub_child in child.children() {
                                if sub_child.as_text().is_some() {
                                    let txt = sub_child.as_text().unwrap().borrow().deref().clone();
                                    if txt == "Open House" || txt == "Open Houses" {
                                        println!("OPEN HOUSE!");
                                        is_open_house = true;
                                    }
                                }
                            }
                        } else if element_data.name.local.as_ref() == "span" {
                            if is_open_house {
                                open_houses.push(OpenHouse{
                                    info: trim_whitespace(&extract_text_children_no_descend(&child)),
                                });
                            }
                        }
                    },
                    _ => (),
                }
            }
        }

        let listing = ListingData{
            price_usd: price,
            name: name,
            open_houses: open_houses,
        };

        println!("Street easy client listing: {:?}", listing);

        self.parse_cache_save(url, &listing).ok();
        return Ok(listing);
    }
}

fn extract_text_children_no_descend(node: &kuchiki::NodeRef) -> String {
    let mut acc = String::new();
    for child in node.children() {
        match child.data() {
            &kuchiki::NodeData::Text(ref val) => {
                acc.push_str(val.borrow().deref());
            },
            _ => (),
        }
    }

    return acc;
}

fn trim_whitespace(input: &str) -> String {
    let regex = regex::Regex::new("^\\s*(.*?)\\s*$").unwrap();
    return regex.captures_iter(input).fold(
        "".to_string(), |acc, x| {
            println!("Append: \"{}\"", x.at(1).unwrap());
            return acc + x.at(1).unwrap();
        });
}

#[cfg(test)]
mod tests {
    use super::StreetEasyClient;

    fn one_trim_test(expected: &str, input: &str) {
        assert_eq!(expected, super::trim_whitespace(input),
                   "Input was: \"{}\"", input);
    }
    
    #[test]
    fn trim_whitespace() {
        one_trim_test("foo", " foo ");
    }
    
    #[test]
    fn parse_price() {
        let client = StreetEasyClient::new();
        let listing = client.lookup_listing("http://streeteasy.com/sale/1241009");
        assert_eq!(2350000, listing.unwrap().price_usd);
    }
}
