extern crate std;
extern crate mysql;
extern crate rustc_serialize;

use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;

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


#[derive(Clone, Debug, PartialEq, Eq, RustcDecodable, RustcEncodable)]
pub struct AnnotatedItem {
    pub item: Item,
    pub annotations: Vec<Annotation>,
}
to_json_for_encodable!(AnnotatedItem);

#[derive(Clone, RustcEncodable)]
pub struct FullLinkAnnotation {
    pub url: String,
}
to_json_for_encodable!(FullLinkAnnotation);

#[derive(Clone, RustcEncodable)]
pub struct FullStreetEasyAnnotation {
    pub price_usd: i32,
}

#[derive(Clone, RustcEncodable)]
pub struct FullItem {
    pub id: i64,
    pub name: String,
    pub description: String,

    pub link_annotations: Vec<FullLinkAnnotation>,
    pub streeteasy_annotations: Vec<FullStreetEasyAnnotation>,
}
to_json_for_encodable!(FullItem);

#[derive(Clone, RustcEncodable)]
pub struct FullList {
    pub name: String,
    pub items: Vec<FullItem>,
}
to_json_for_encodable!(FullList);
