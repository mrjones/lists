extern crate protobuf;
extern crate std;

use data;
use model;
use result;
use storage_format;
use streeteasy;
use workqueue;

use protobuf::Message;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug)]
pub struct ItemUpdate {
    pub list_id: i64,
    pub item_id: i64,
}

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
                open_houses: saved_data.get_open_house().iter()
                    .map(|x| x.to_string()).collect(),
                
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
    workqueue: std::sync::Arc<std::sync::Mutex<workqueue::WorkQueue + std::marker::Send>>,
    work_ready: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Sender<()>>>,
    item_updated: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Sender<ItemUpdate>>>,
}

impl AnnotationExpander {
    pub fn new(db: std::sync::Arc<std::sync::Mutex<data::Db>>,
               workqueue: std::sync::Arc<std::sync::Mutex<workqueue::WorkQueue + std::marker::Send>>,
               work_ready: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Sender<()>>>,
               item_updated: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Sender<ItemUpdate>>>) -> AnnotationExpander {
        return AnnotationExpander{
            db: db,
            se_client: streeteasy::StreetEasyClient::new(),
            workqueue: workqueue,
            work_ready: work_ready,
            item_updated: item_updated,
        }
    }

    fn finish_task<T: std::fmt::Debug>(&self, list_id: i64, item_id: i64, result: result::ListsResult<T>, task: &workqueue::Task) {
        println!("Finished task (l:{}, i:{}) with result {:?}", list_id, item_id, result);
        match result {
            Ok(_) => {
                println!("Updating workqueue...");
                self.workqueue.lock().unwrap().finish(task.id).unwrap();
                let iu = ItemUpdate{
                    list_id: list_id,
                    item_id: item_id,
                };
                println!("Pusing notification: {:?}", iu);
                self.item_updated.lock().unwrap().send(iu).unwrap();
                println!("Task success. Updated workqueue and pushed a notification");
            },
            Err(_) => self.workqueue.lock().unwrap().abort(task.id).unwrap(),
        }
    }
    
    pub fn process_work_queue(&self) {
        let res = self.workqueue.lock().unwrap().dequeue();
        match res {
            Some((raw_task, _)) => {
                let mut task = storage_format::RefreshStreetEasyTask::new();
                task.merge_from_bytes(&raw_task.payload).unwrap();
                println!("Got work: {:?}", task);
                
                self.finish_task(
                    task.get_list_id(),
                    task.get_item_id(),
                    self.generate_streeteasy_annotation(
                        task.get_item_id(), task.get_parent_id(), task.get_url()),
                    &raw_task);
            },
            None => {
                println!("No work found");
            },
        }
    }
    
    pub fn generate_auto_annotations(&self, list_id: i64, item_id: i64, annotation_id: i64, kind: &str, body: &str) {
        if kind == "LINK" && body.contains("streeteasy.com") {
            let mut task = storage_format::RefreshStreetEasyTask::new();
            task.set_list_id(list_id);
            task.set_item_id(item_id);
            task.set_parent_id(annotation_id);
            task.set_url(body.to_string());

            self.workqueue.lock().unwrap().enqueue(
                &task.write_to_bytes().unwrap()).unwrap();
            self.work_ready.lock().unwrap().send(()).unwrap();
//            self.generate_streeteasy_annotation(item_id, annotation_id, body);
        }
    }

    fn generate_streeteasy_annotation(&self, item_id: i64, parent_id: i64, url: &str) -> result::ListsResult<()> {
        println!("Working on task: {}", url);
        // TODO(mrjones): Update annotation if already exists
        let listing = try!(self.se_client.lookup_listing(url));
        let mut hasher = std::hash::SipHasher::new();
        listing.name.hash(&mut hasher);
        hasher.write_i64(item_id);

        let mut a = storage_format::StreetEasyAnnotation::new();
        a.set_price_usd(listing.price_usd);
        a.set_name(listing.name);
        a.set_hash(hasher.finish());
        for oh in listing.open_houses {
            a.mut_open_house().push(oh.info);
        }
                
        try!(self.db.lock().unwrap().add_auto_annotation(
            item_id, parent_id, "STREETEASY",
            &a.write_to_bytes().unwrap()));

        println!("Expander says: {:?}", a);
        return Ok(());
    }
}
