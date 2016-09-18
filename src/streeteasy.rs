extern crate kuchiki;
extern crate regex;
extern crate std;

use self::kuchiki::traits::TendrilSink;
use result::ListsResult;
use scrape::HyperHttpClient;
use scrape::Scraper;
use std::ops::Deref;

pub struct StreetEasyClient {
    scraper: Scraper,

    price_regex: regex::Regex,
}

pub struct ListingData {
    price_usd: i32,
}

impl StreetEasyClient {
    pub fn new() -> StreetEasyClient {
        return StreetEasyClient{
            scraper: Scraper::new(
                std::sync::Arc::new(std::sync::Mutex::new(
                    HyperHttpClient::new())),
                "/home/mrjones/lists.cache/"),
            price_regex: regex::Regex::new("(\\$[0-9,]+)").unwrap(),
        }
    }

    pub fn lookup_listing(&mut self, sale_id: &str) -> ListsResult<ListingData> {
        let url = format!("http://streeteasy.com/sale/{}", sale_id);
        let page = try!(self.scraper.fetch(&url));

        let document = kuchiki::parse_html().one(page);

        let mut price = -1;
        
        // element: kuchiki::NodeDataRef
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

        return Ok(ListingData{
            price_usd: price,
        });
    }
        /*
        html5ever::parse_document(
            html5ever::rcdom::RcDom::default(),
            std::default::Default::default())
            .from_utf8()
            .read_from(page)
            .unwrap();

        
        
        return ListingData{price_usd: 1};
    }

    fn walk_page(handle: html5ever::rcdom::Handle) -> ListingData {
        let node = handle.borrow();
        match node.node {
            
        }
    }
*/
}

#[cfg(test)]
mod tests {
    use super::StreetEasyClient;

    #[test]
    fn parse_price() {
        let mut client = StreetEasyClient::new();
        let listing = client.lookup_listing("1241009");
        assert_eq!(2350000, listing.unwrap().price_usd);
    }
}
