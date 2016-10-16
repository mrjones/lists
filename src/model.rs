extern crate chrono;
extern crate std;
extern crate mysql;
extern crate protobuf;
extern crate rustc_serialize;

use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
/*
use std::iter::FromIterator;

struct TypeInfo {
    extract_one: Box<Fn(&'static protobuf::Message, &'static protobuf::reflect::FieldDescriptor) -> Json>
}

struct ProtoToJsonConfig {
    type_infos: std::collections::HashMap<
            protobuf::descriptor::FieldDescriptorProto_Type, TypeInfo>,
}

fn extract_one_string(msg: &protobuf::Message, desc: &protobuf::reflect::FieldDescriptor) -> Json {
    return Json::String(desc.get_str(msg).to_string());
}

impl ProtoToJsonConfig {
    fn new() -> ProtoToJsonConfig {
        use protobuf::descriptor::FieldDescriptorProto_Type;
        
        return ProtoToJsonConfig{
            type_infos: std::collections::HashMap::from_iter(vec![
                (FieldDescriptorProto_Type::TYPE_STRING,
                 TypeInfo{
                     extract_one: Box::new(|m, d: &protobuf::reflect::FieldDescriptor| {
                         return Json::String(d.get_str(m).to_string());
                     })
                 }),
            ].into_iter()),
        }
    }
}
 */

fn collect_repeated(
    message: &protobuf::Message,
    field_descriptor: &protobuf::reflect::FieldDescriptor,
    extract_fn: &Fn(&protobuf::Message, usize) -> Json) -> Vec<Json> {

    let mut jsons = vec![];
    for i in 0..field_descriptor.len_field(message) {
        jsons.push(extract_fn(message, i));
    }
    return jsons;
}

fn field_to_json(
    message: &protobuf::Message,
    field_descriptor: &protobuf::reflect::FieldDescriptor) -> Json {
    
    if field_descriptor.is_repeated() {
        match field_descriptor.proto().get_field_type() {
            protobuf::descriptor::FieldDescriptorProto_Type::TYPE_MESSAGE => {
                
                return Json::Array(collect_repeated(message, field_descriptor, &|msg, i| {
                    return proto_to_json(field_descriptor.get_rep_message_item(msg, i));
                }));
            },
            protobuf::descriptor::FieldDescriptorProto_Type::TYPE_STRING => {
                return Json::Array(collect_repeated(message, field_descriptor, &|msg, i| {
                    return Json::String(field_descriptor.get_rep_str_item(msg, i).to_string());
                }));
            },
            _ => unimplemented!(),
        }
    } else {
        match field_descriptor.proto().get_field_type() {
            protobuf::descriptor::FieldDescriptorProto_Type::TYPE_MESSAGE => {
                let sub_message: &protobuf::Message =
                    field_descriptor.get_message(message);
                return proto_to_json(sub_message);
            },
            protobuf::descriptor::FieldDescriptorProto_Type::TYPE_STRING => {
                return Json::String(field_descriptor.get_str(message).to_string());
            },
            protobuf::descriptor::FieldDescriptorProto_Type::TYPE_INT32 => {
                return Json::I64(field_descriptor.get_i32(message) as i64);
            },
            protobuf::descriptor::FieldDescriptorProto_Type::TYPE_INT64 => {
                return Json::I64(field_descriptor.get_i64(message));
            },
            protobuf::descriptor::FieldDescriptorProto_Type::TYPE_UINT64 => {
                return Json::U64(field_descriptor.get_u64(message));
            }
            _ => unimplemented!(),
        }
    }
}

pub fn proto_to_json(message: &protobuf::Message) -> Json {
    let descriptor = message.descriptor();
    let mut field_map = std::collections::BTreeMap::<String, Json>::new();

    for field in descriptor.fields() {
        field_map.insert(field.name().to_string(), field_to_json(message, field));
    }

    return Json::Object(field_map);
}

// TODO(mrjones): This is really terrible :/
// Reference: https://github.com/rust-lang-nursery/rustc-serialize/issues/46
pub fn to_json<E: rustc_serialize::Encodable>(obj: &E) -> Json {
    let str = rustc_serialize::json::encode(obj)
        .expect("Could not encode object");
    return rustc_serialize::json::Json::from_str(&str)
        .expect("Could not re-decode object");
}

macro_rules! to_json_for_encodable {
    ($($t:ty)*) => ($(
        impl ToJson for $t {
            fn to_json(&self) -> Json {
                return to_json(self);
            }
        }
    )*)
}

// TODO(mrjone): move to another module?
pub trait DbObject {
    fn from_row(row: mysql::Row) -> Self;
}

// maps to "Users" table in MySql
#[derive(Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct User {
    pub id: i64,
    pub name: String,
}
to_json_for_encodable!(User);

impl DbObject for User {
    fn from_row(row: mysql::Row) -> User {
        let (id, name) = mysql::from_row(row);
        return User {
            id: id,
            name: name,
        };
    }
}

// maps to "Lists" table in MySql
#[derive(Debug, PartialEq, Eq, RustcEncodable)]
pub struct List {
    pub id: i64,
    pub name: String,
}
to_json_for_encodable!(List);

impl DbObject for List {
    fn from_row(row: mysql::Row) -> List {
        let (id, name) = mysql::from_row(row);
        return List {
            id: id,
            name: name,
        };
    }
}

// maps to "Items" table in MySql
#[derive(Clone, Debug, PartialEq, Eq, RustcDecodable, RustcEncodable)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub description: String,
}
to_json_for_encodable!(Item);

impl DbObject for Item {
    fn from_row(row: mysql::Row) -> Item {
        let (id, name, description) = mysql::from_row(row);
        return Item {
            id: id,
            name: name,
            description: description,
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq, RustcDecodable, RustcEncodable)]
pub struct Annotation {
    pub id: i64,
    pub item_id: i64,
    pub kind: String,
    pub body: String,
}
to_json_for_encodable!(Annotation);

impl DbObject for Annotation {
    fn from_row(row: mysql::Row) -> Annotation {
        let (id, item_id, kind, body) = mysql::from_row(row);
        return Annotation {
            id: id,
            item_id: item_id,
            kind: kind,
            body: body,
        };
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutoAnnotation {
    pub id: i64,
    pub item_id: i64,
    pub parent_id: i64,
    pub kind: String,
    pub body: Vec<u8>,
    pub mtime: chrono::NaiveDateTime,
}

impl DbObject for AutoAnnotation {
    fn from_row(row: mysql::Row) -> AutoAnnotation {
        let (id, item_id, parent_id, kind, body, mtime) = mysql::from_row(row);
        return AutoAnnotation {
            id: id,
            item_id: item_id,
            parent_id: parent_id,
            kind: kind,
            body: body,
            mtime: mtime,
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnnotatedItem {
    pub item: Item,
    pub annotations: Vec<Annotation>,
    pub auto_annotations: Vec<AutoAnnotation>,
}

#[derive(Clone, RustcEncodable)]
pub struct FullLinkAnnotation {
    pub id: i64,
    pub url: String,
}
to_json_for_encodable!(FullLinkAnnotation);

#[derive(Clone, RustcEncodable)]
pub struct FullTextAnnotation {
    pub id: i64,
    pub text: String,
}
to_json_for_encodable!(FullTextAnnotation);

#[derive(Clone, Debug, RustcEncodable)]
pub struct FullStreetEasyAnnotation {
    pub hash: u64,
    pub price_usd: i32,
    pub name: String,
    pub open_houses: Vec<String>,
}


#[derive(Clone, RustcEncodable)]
pub struct FullItem {
    pub id: i64,
    pub name: String,
    pub description: String,

    pub link_annotations: Vec<FullLinkAnnotation>,
    pub streeteasy_annotations: Vec<FullStreetEasyAnnotation>,
    pub text_annotations: Vec<FullTextAnnotation>,
}
to_json_for_encodable!(FullItem);

#[derive(Clone, RustcEncodable)]
pub struct FullList {
    pub name: String,
    pub items: Vec<FullItem>,
}
to_json_for_encodable!(FullList);


pub struct DbWorkQueueTask {
    pub id: i64,
    pub payload: Vec<u8>,
}
impl DbObject for DbWorkQueueTask {
    fn from_row(row: mysql::Row) -> DbWorkQueueTask {
        let (id, payload) = mysql::from_row(row);
        return DbWorkQueueTask {
            id: id,
            payload: payload,
        };
    }
}

pub struct DbWorkQueueLease {
    pub id: i64,
    pub payload: Vec<u8>,
    pub expiration: std::time::SystemTime,
}
impl DbObject for DbWorkQueueLease {
    fn from_row(row: mysql::Row) -> DbWorkQueueLease {
        let (id, payload, epoch_expiration) = mysql::from_row(row);
        return DbWorkQueueLease {
            id: id,
            payload: payload,
            expiration: std::time::UNIX_EPOCH +
                std::time::Duration::new(epoch_expiration, 0),
        };
    }
}

#[cfg(test)]
mod tests {
    extern crate protobuf;
    extern crate std;
    
    use api;

    use rustc_serialize::json::Json;
    use std::collections::BTreeMap;
    use std::iter::FromIterator;
    
    #[test]
    fn simple_proto_to_json() {
        let mut user = api::User::new();
        user.set_name("matt".to_string());
        user.set_id(1);

        assert_eq!(
            super::proto_to_json(&user),
            Json::Object(
                std::collections::BTreeMap::from_iter(vec![
                    ("name".to_string(), Json::String("matt".to_string())),
                    ("id".to_string(), Json::I64(1))
                ].into_iter())));
        
        assert_eq!(
            "{\"id\":1,\"name\":\"matt\"}".to_string(),
            super::proto_to_json(&user).to_string());
    }

    #[test]
    fn complicated_proto_to_json() {
        let mut list_item = api::ListItem::new();
        list_item.set_name("item".to_string());
        list_item.set_id(1);
        list_item.set_link_annotations(
            protobuf::RepeatedField::<api::LinkAnnotation>::from_vec(
                vec![("foo", 0), ("bar", 1), ("baz", 2)].into_iter().map(|(n, i)| {
                    let mut ann = api::LinkAnnotation::new();
                    ann.set_id(i);
                    ann.set_url(n.to_string());
                    return ann;
                }).collect::<Vec<api::LinkAnnotation>>()));

        assert_eq!(
            super::proto_to_json(&list_item),
            Json::Object(
                BTreeMap::from_iter(vec![
                    ("name".to_string(), Json::String("item".to_string())),
                    ("id".to_string(), Json::I64(1)),
                    ("description".to_string(), Json::String("".to_string())),
                    ("link_annotations".to_string(), Json::Array(vec![
                        Json::Object(
                            BTreeMap::from_iter(vec![
                                ("id".to_string(), Json::I64(0)),
                                ("url".to_string(), Json::String("foo".to_string())),
                            ])
                        ),
                        Json::Object(
                            BTreeMap::from_iter(vec![
                                ("id".to_string(), Json::I64(1)),
                                ("url".to_string(), Json::String("bar".to_string())),
                            ])
                        ),
                        Json::Object(
                            BTreeMap::from_iter(vec![
                                ("id".to_string(), Json::I64(2)),
                                ("url".to_string(), Json::String("baz".to_string())),
                            ])
                        ),
                    ])),
                    ("text_annotations".to_string(), Json::Array(vec![])),
                    ("streeteasy_annotations".to_string(), Json::Array(vec![])),
                ].into_iter())));

    }
}
