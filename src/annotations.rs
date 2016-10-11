extern crate protobuf;
extern crate std;

use data;
use model;
use storage_format;
use streeteasy;

use protobuf::Message;
use std::hash::Hash;
use std::hash::Hasher;

fn attach_user_annotation(item: &mut model::FullItem, user_annotation: &model::Annotation) {
    match user_annotation.kind.as_str() {
        "LINK" => {
            item.link_annotations.push(
                model::FullLinkAnnotation{
                    url: user_annotation.body.clone(),
                    id: user_annotation.id,
                }
            );
        },
        "TEXT" => {
            item.text_annotations.push(
                model::FullTextAnnotation{
                    text: user_annotation.body.clone(),
                    id: user_annotation.id,
                }
            );
        },
        _ => println!("Ignoring annotation: {:?}", user_annotation),
    }
}

fn attach_auto_annotation(item: &mut model::FullItem, auto_annotation: &model::AutoAnnotation) {
    match auto_annotation.kind.as_str() {
        "STREETEASY" => {
            let mut saved_data = storage_format::StreetEasyAnnotation::new();
            saved_data.merge_from_bytes(&(auto_annotation.body)).unwrap();
            item.streeteasy_annotations.push(model::FullStreetEasyAnnotation{
                hash: saved_data.get_hash(),
                name: saved_data.get_name().to_string(),
                price_usd: saved_data.get_price_usd(),
            });
        }
        _ => println!("Ignoring auto annotation: {:?}", auto_annotation),
    }
}

pub fn parse_and_attach_annotations_single(
    item_metadata: model::Item,
    user_annotations: Vec<model::Annotation>,
    auto_annotations: Vec<model::AutoAnnotation>) -> model::FullItem {

    let mut out = parse_and_attach_annotations(
        vec![item_metadata], user_annotations, auto_annotations);
    assert_eq!(1, out.len());
    return out.remove(0);
}

pub fn parse_and_attach_annotations(
    item_metadatas: Vec<model::Item>,
    user_annotations: Vec<model::Annotation>,
    auto_annotations: Vec<model::AutoAnnotation>) -> Vec<model::FullItem> {

    let mut complete_items : Vec<model::FullItem> = vec![];
        
    for item_metadata in item_metadatas {
        complete_items.push(model::FullItem{
            id: item_metadata.id,
            name: item_metadata.name,
            description: item_metadata.description,
            link_annotations: vec![],
            streeteasy_annotations: vec![],
            text_annotations: vec![],
        });
    }

    for user_annotation in user_annotations {
        let index = complete_items.binary_search_by_key(
            &user_annotation.item_id, |complete_item| complete_item.id)
            .expect("dangling annotation");
        
        attach_user_annotation(&mut complete_items[index], &user_annotation);
    }

    for auto_annotation in auto_annotations {
        let index = complete_items.binary_search_by_key(
            &auto_annotation.item_id, |complete_item| complete_item.id)
            .expect("dangling annotation");

        attach_auto_annotation(&mut complete_items[index], &auto_annotation);
    }
    
    return complete_items;
}

pub struct AnnotationExpander {
    db: std::sync::Arc<std::sync::Mutex<data::Db>>,
    se_client: streeteasy::StreetEasyClient,
}

impl AnnotationExpander {
    pub fn new(db: std::sync::Arc<std::sync::Mutex<data::Db>>) -> AnnotationExpander {
        return AnnotationExpander{
            db: db,
            se_client: streeteasy::StreetEasyClient::new(),
        }
    }
    
    pub fn generate_auto_annotations(&self, item_id: i64, annotation_id: i64, kind: &str, body: &str) {
        if kind == "LINK" && body.contains("streeteasy.com") {
            self.generate_streeteasy_annotation(item_id, annotation_id, body);
        }
    }

    fn generate_streeteasy_annotation(&self, item_id: i64, parent_id: i64, url: &str) {
        // TODO(mrjones): Do fetching asynchronously.
        let listing_result = self.se_client.lookup_listing(url);
        return match listing_result {
            Ok(listing) => {
                let mut hasher = std::hash::SipHasher::new();
                listing.name.hash(&mut hasher);
                hasher.write_i64(item_id);

                let mut a = storage_format::StreetEasyAnnotation::new();
                a.set_price_usd(listing.price_usd);
                a.set_name(listing.name);
                a.set_hash(hasher.finish());
                
                self.db.lock().unwrap().add_auto_annotation(
                    item_id, parent_id, "STREETEASY",
                    &a.write_to_bytes().unwrap()).unwrap();

                println!("Expander says: {:?}", a);
            },
            Err(_) => (),
        };
    }
}
