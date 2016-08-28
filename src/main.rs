extern crate handlebars_iron;
#[macro_use]
extern crate iron;
extern crate mount;
extern crate mysql;
extern crate router;
extern crate persistent;
extern crate plugin;
extern crate rustc_serialize;
extern crate staticfile;
extern crate url;

use iron::prelude::*;

use iron::status;
use plugin::Extensible;
use router::Router;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::io::Read;

#[derive(Copy, Clone)]
pub struct ConnectionPool;
impl iron::typemap::Key for ConnectionPool {
    type Value = mysql::Pool;
}

// TODO(mrjones): This is really terrible :/
// Reference: https://github.com/rust-lang-nursery/rustc-serialize/issues/46
fn to_json<E: rustc_serialize::Encodable>(obj: &E) -> Json {
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

trait DbObject {
    fn from_row(row: mysql::Row) -> Self;
}

// maps to "Users" table in MySql
#[derive(Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
struct User {
    id: i64,
    name: String,
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
struct List {
    id: i64,
    name: String,
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
struct Item {
    id: i64,
    name: String,
    description: String,
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
struct Annotation {
    id: i64,
    item_id: i64,
    kind: String,
    body: String,
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
struct AnnotatedItem {
    item: Item,
    annotations: Vec<Annotation>,
}
to_json_for_encodable!(AnnotatedItem);

#[derive(RustcEncodable)]
struct ListPage<'a> {
    id: i64,
    items: Vec<ListPageItem<'a>>,

    owner: &'a User,
    accessors: &'a Vec<User>,
    all_users: &'a Vec<User>,
}
impl<'a> ToJson for ListPage<'a> {
    fn to_json(&self) -> Json {
      return to_json(self);
    }
}

#[derive(RustcEncodable)]
struct ListPageItem<'a> {
    id: i64,
    name: &'a str,
    description: &'a str,

    link_annotations: Vec<LinkAnnotation<'a>>,
}

#[derive(RustcEncodable)]
struct LinkAnnotation<'a> {
    url: &'a str,
}



#[derive(Debug, Clone)]
enum ListsError {
    MissingParam(String),
    InvalidParam,
    DatabaseError,
    DoesNotExist,
    InconsistentDatabase(String),

    Unknown,
}

impl ListsError {
    fn str(&self) -> &str {
        match *self {
            // TODO(mrjones): print out which param is actually missing?
            ListsError::MissingParam(_) => "MissingParam", 
            ListsError::InvalidParam => "InvalidParam",
            ListsError::DatabaseError => "DatabaseError",
            ListsError::DoesNotExist => "DoesNotExist",
            ListsError::Unknown => "Unknown",
            ListsError::InconsistentDatabase(_) => "InconsistentDababase",
        }
    }
}

impl std::error::Error for ListsError {
    fn description(&self) -> &str {
        return self.str();
    }

    fn cause(&self) -> Option<&std::error::Error> {
        return None;
    }
}

impl std::fmt::Display for ListsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return f.write_str(self.str());
    }
}


impl std::convert::From<ListsError> for iron::error::IronError {
    fn from(wrapped: ListsError) -> iron::error::IronError {
        return into_iron_error(wrapped);
    }
}

fn into_iron_error<E: iron::error::Error>(err: E) -> iron::error::IronError {
    let desc = err.description().to_owned();
    return iron::error::IronError::new(err, desc);
}



struct ErrorPage;

impl iron::middleware::AfterMiddleware for ErrorPage {
    fn after(&self, _: &mut iron::request::Request, res: iron::response::Response) -> iron::IronResult<iron::response::Response> {
        return Ok(res);
    }

    fn catch(&self, _: &mut iron::request::Request, err: iron::error::IronError) -> iron::IronResult<iron::response::Response> {
        return Ok(iron::response::Response::with(
            (iron::status::NotFound, format!("ERROR PAGE!!\n{:?}", err).to_string())));
    }
}

struct RequestEnv {
//    db_pool: &mysql::Pool,
    user: iron::IronResult<User>,
}

struct RequestEnvBuilder {
//    db_pool: &mysql::Pool,
}
impl iron::typemap::Key for RequestEnvBuilder { type Value = RequestEnv; }

fn lookup_user(id: i64, db_conn: &mysql::Pool) -> iron::IronResult<User> {
    let mut result = itry!(db_conn.prep_exec("SELECT id, name FROM lists.users WHERE id = ?", (id,)));
    let row = itry!(result.next().ok_or(ListsError::DoesNotExist));
    let user = User::from_row(itry!(row));
    assert!(result.next().is_none(), "Duplicate user id!");
    return Ok(user);
}

fn parse_to_map(parse: &mut url::form_urlencoded::Parse) -> std::collections::BTreeMap<String, String> {
    let mut result = std::collections::BTreeMap::new();
    for (k,v) in parse {
        let key : String = k.to_string();
        let val : String = v.to_string();
        let _ = result.insert(key, val);
    }
    return result;    
}

fn params_map(req: &iron::request::Request) -> std::collections::BTreeMap<String, String> {
    let url : url::Url = req.url.clone().into_generic_url();
    return parse_to_map(&mut url.query_pairs());
}

fn get_user(params: &std::collections::BTreeMap<String, String>, db_conn: &mysql::Pool) -> iron::IronResult<User> {
    let id_str = itry!(params.get("user_id").ok_or(
        ListsError::MissingParam("user_id".to_string())));
    let id_int = itry!(id_str.parse::<i64>());
    return lookup_user(id_int, db_conn);
}

impl iron::BeforeMiddleware for RequestEnvBuilder {
    fn before(&self, req: &mut iron::request::Request) -> iron::IronResult<()> {
        let user;
        {
            let conn = &req.extensions.get::<persistent::Read<ConnectionPool>>().unwrap();
            let params = params_map(req);
            user = get_user(&params, conn);
        }

        req.extensions.insert::<RequestEnvBuilder>(RequestEnv{
            user: user,
//            db_pool: self.db_pool,
        });


        return Ok(());
    }
}

fn pick_user_handler(req: &mut iron::request::Request) -> iron::IronResult<iron::response::Response> {
    return pick_user_immutable_handler(req);
}

fn to_vector<T: DbObject>(query_result: mysql::QueryResult) -> mysql::error::Result<Vec<T>> {
    return query_result.map(|row_result| {
        return Ok(T::from_row(try!(row_result)));
    }).collect();
}

fn pick_user_immutable_handler(req: &iron::request::Request) -> iron::IronResult<iron::response::Response> {
    let conn = &req.extensions.get::<persistent::Read<ConnectionPool>>().unwrap();

    let users = itry!(to_vector::<User>(
        itry!(conn.prep_exec("SELECT id, name FROM lists.users", ()))));

    let mut data : std::collections::BTreeMap<String, Json> =
        std::collections::BTreeMap::new();
    data.insert("users".to_string(), to_json(&users));

    let mut response = iron::response::Response::new();
    response
        .set_mut(handlebars_iron::Template::new("pick-user", data))
        .set_mut(status::Ok);
    return Ok(response);
}

fn show_all_lists(user: &User, conn: &mysql::Pool) -> iron::IronResult<iron::response::Response> {
    let lists = itry!(to_vector::<List>(
        itry!(conn.prep_exec("SELECT lists.lists.id, lists.lists.name FROM lists.list_users LEFT JOIN lists.lists ON lists.list_users.list_id = lists.lists.id WHERE lists.list_users.user_id = ?", (user.id,)))));

    let mut data : std::collections::BTreeMap<String, Json> =
        std::collections::BTreeMap::new();
    data.insert("lists".to_string(), lists.to_json());
    data.insert("user_id".to_string(), user.id.to_json());
    
    let mut response = iron::response::Response::new();
    response
        .set_mut(handlebars_iron::Template::new("my-lists", data))
        .set_mut(status::Ok);
            
    return Ok(response);
}

fn show_all_lists_handler(req: &mut iron::request::Request) -> iron::IronResult<iron::response::Response> {
    let env = &req.extensions.get::<RequestEnvBuilder>().unwrap();
    match env.user {
        Err(_) => return pick_user_immutable_handler(req),
        Ok(ref user) => {
            let conn = &req.extensions.get::<persistent::Read<ConnectionPool>>().unwrap();
            return show_all_lists(user, conn);
        },
    }
}

fn show_one_list_handler(req: &mut iron::request::Request) -> iron::IronResult<iron::response::Response> {
    let conn = &req.get::<persistent::Read<ConnectionPool>>().unwrap();
    let params = params_map(req);
    let env = &req.extensions().get::<RequestEnvBuilder>().unwrap();

//    let user = try!(env.user);
//    let list_id = params.get("list_id").unwrap();
//    return show_list(list_id, &user, conn);

    match env.user {
        // TODO(mrjones): propagate error
        Err(_) => return Err(into_iron_error(ListsError::Unknown)),
        Ok(ref user) => {
            // TODO(mrjones): check permissions?
            let list_id = itry!(
                params.get("list_id").ok_or(
                    ListsError::MissingParam("list_id".to_string())));
            return show_list(list_id, user, conn);
        },
    }
}

fn to_link_annotation<'a>(annotation: &'a Annotation) -> Option<LinkAnnotation<'a>> {
    if annotation.kind != "LINK" {
        return None
    }
    return Some(LinkAnnotation{url: &annotation.body});
}

fn show_list(list_id: &str, user: &User, conn: &mysql::Pool) -> iron::IronResult<iron::response::Response> {
    let items = itry!(to_vector::<Item>(
        itry!(conn.prep_exec("SELECT id, name, description FROM lists.items WHERE list_id = ?", (list_id,)))));


    let annotations = itry!(to_vector::<Annotation>(
        itry!(conn.prep_exec("SELECT lists.item_annotations.id, lists.items.id, lists.item_annotations.kind, lists.item_annotations.body FROM lists.items JOIN lists.item_annotations ON lists.items.id = lists.item_annotations.item_id WHERE lists.items.list_id = ?", (list_id,)))));

    let mut annotated_items : std::collections::BTreeMap<i64, AnnotatedItem> = std::collections::BTreeMap::new();
    for item in &items {
        // TODO(mrjones): Make AnnotatedItem be references?
        annotated_items.insert(item.id, AnnotatedItem{item: item.clone(), annotations: Vec::new()});
    }
    for annotation in &annotations {
        itry!(annotated_items.get_mut(&annotation.item_id)
              .ok_or(ListsError::InconsistentDatabase("dangling annotation".to_string())))
            .annotations.push(annotation.clone());
    }

    let annotated_items_vec : Vec<AnnotatedItem> = annotated_items.values().cloned().collect();

    let all_users = itry!(to_vector::<User>(
        itry!(conn.prep_exec("SELECT id, name FROM lists.users ORDER BY name ASC", ()))));
    
    let accessors = itry!(to_vector::<User>(
        itry!(conn.prep_exec("SELECT lists.users.id, lists.users.name FROM lists.list_users LEFT JOIN lists.users ON lists.list_users.user_id = lists.users.id WHERE lists.list_users.list_id = ?", (list_id,)))));

    let list_page = ListPage{
        // TODO(mrjones): fetch name from DB
        id: itry!(list_id.parse::<i64>()),
        owner: &user,
        accessors: &accessors,
        all_users: &all_users,
        items: annotated_items_vec.iter().map(|item| {
            ListPageItem {
                id: item.item.id,
                name: &item.item.name,
                description: &item.item.description,
                link_annotations: item.annotations.iter()
                    .filter_map(to_link_annotation).collect(),
            }
        }).collect(),
    };

    let mut data : std::collections::BTreeMap<String, Json> =
        std::collections::BTreeMap::new();
    data.insert("page".to_string(), list_page.to_json());

    let mut response = iron::response::Response::new();
    response
        .set_mut(handlebars_iron::Template::new("one-list", data))
        .set_mut(status::Ok);
            
    return Ok(response);
}

fn read_body(req: &mut iron::request::Request) -> String {
    let mut buf = String::new();
    req.body.by_ref().read_to_string(&mut buf).unwrap();
    return buf;
}

fn add_list_handler(req: &mut iron::request::Request) -> iron::IronResult<iron::response::Response> {
    let body = read_body(req);
    let pool = &req.get::<persistent::Read<ConnectionPool>>().unwrap();
    let env = &req.extensions().get::<RequestEnvBuilder>().unwrap();
    let mut conn = pool.get_conn().unwrap();

    match env.user {
        Err(_) => return pick_user_immutable_handler(req),
        Ok(ref user) => {
            let mut parse = url::form_urlencoded::parse(body.as_bytes());
            let body_params = parse_to_map(&mut parse);

            let name = itry!(body_params.get("name").ok_or(
                ListsError::MissingParam("name".to_string())));

            itry!(conn.prep_exec("INSERT INTO lists.lists (name) VALUES (?)", (name,)));
            itry!(conn.prep_exec("INSERT INTO lists.list_users (list_id, user_id) VALUES (LAST_INSERT_ID(), ?)", (user.id,)));
            return show_all_lists(user, pool);
        }
    }    
}

fn add_list_item_json_handler(req: &mut iron::request::Request) -> iron::IronResult<iron::response::Response> {
    let body = read_body(req);
    let conn = &req.get::<persistent::Read<ConnectionPool>>().unwrap();
    let env = &req.extensions().get::<RequestEnvBuilder>().unwrap();

    match env.user {
        Err(_) => return pick_user_immutable_handler(req),
        Ok(ref user) => {
            let item : AnnotatedItem = rustc_serialize::json::decode(&body)
                .expect("Couldn't parse JSON");
            println!("Parsed {:?}.", item);
            return Ok(iron::response::Response::new());
        }
    }
}

fn add_list_item_handler(req: &mut iron::request::Request) -> iron::IronResult<iron::response::Response> {
    let body = read_body(req);
    let conn = &req.get::<persistent::Read<ConnectionPool>>().unwrap();
    let env = &req.extensions().get::<RequestEnvBuilder>().unwrap();

    match env.user {
        Err(_) => return pick_user_immutable_handler(req),
        Ok(ref user) => {
            let mut parse = url::form_urlencoded::parse(body.as_bytes());
            let body_params = parse_to_map(&mut parse);

            let list_id = itry!(body_params.get("list_id").ok_or(
                ListsError::MissingParam("list_id".to_string())));
            let name = itry!(body_params.get("name").ok_or(
                ListsError::MissingParam("name".to_string())));
            let description = itry!(body_params.get("description").ok_or(
                ListsError::MissingParam("description".to_string())));

            itry!(conn.prep_exec("INSERT INTO lists.items (list_id, name, description) VALUES (?, ?, ?)", (list_id, name, description)));

            let maybe_link = body_params.get("link");
            if maybe_link.is_some() && !maybe_link.unwrap().is_empty() {
                let link_str = maybe_link.unwrap();
                itry!(url::Url::parse(link_str));

                itry!(conn.prep_exec("INSERT INTO lists.item_annotations (item_id, kind, body) VALUES (LAST_INSERT_ID(), 'LINK', ?)", (link_str,)));
            }

            return show_list(list_id, user, conn);
        },
    }
}

fn add_list_user_handler(req: &mut iron::request::Request) -> iron::IronResult<iron::response::Response> {
    let body = read_body(req);
    let conn = &req.get::<persistent::Read<ConnectionPool>>().unwrap();
    let env = &req.extensions().get::<RequestEnvBuilder>().unwrap();

    match env.user {
        Err(_) => return pick_user_immutable_handler(req),
        Ok(ref user) => {
            let mut parse = url::form_urlencoded::parse(body.as_bytes());
            let body_params = parse_to_map(&mut parse);

            let list_id = itry!(body_params.get("list_id").ok_or(
                ListsError::MissingParam("list_id".to_string())));
            let new_user_id = itry!(body_params.get("new_user_id").ok_or(
                ListsError::MissingParam("new_user_id".to_string())));

            itry!(conn.prep_exec("INSERT INTO lists.list_users (list_id, user_id) VALUES (?, ?)", (list_id, new_user_id)));
            return show_list(list_id, user, conn);
        },
    }
}

fn remove_list_user_handler(req: &mut iron::request::Request) -> iron::IronResult<iron::response::Response> {
    let body = read_body(req);
    let conn = &req.get::<persistent::Read<ConnectionPool>>().unwrap();
    let env = &req.extensions().get::<RequestEnvBuilder>().unwrap();

    match env.user {
        Err(_) => return pick_user_immutable_handler(req),
        Ok(ref user) => {
            let mut parse = url::form_urlencoded::parse(body.as_bytes());
            let body_params = parse_to_map(&mut parse);

            let list_id = itry!(body_params.get("list_id").ok_or(
                ListsError::MissingParam("list_id".to_string())));
            let removed_user_id = itry!(body_params.get("removed_user_id").ok_or(
                ListsError::MissingParam("removed_user_id".to_string())));

            itry!(conn.prep_exec("DELETE FROM lists.list_users WHERE list_id = ? AND  user_id = ?", (list_id, removed_user_id)));
            return show_list(list_id, user, conn);
        },
    }
}

fn main() {
    let mut handlebars = handlebars_iron::HandlebarsEngine::new();
    handlebars.add(Box::new(
        handlebars_iron::DirectorySource::new("./templates/", ".html")));
    if let Err(r) = handlebars.reload() {
        panic!("{:?}", r);
    }

    
    
    let mut router = Router::new();
    router.get(r"/lists", show_all_lists_handler);
    router.get(r"/list", show_one_list_handler);
    router.get(r"/users", pick_user_handler);
    router.any(r"/add_list_item", add_list_item_handler);
    router.any(r"/add_list_item_json", add_list_item_json_handler);
    router.any(r"/add_list_user", add_list_user_handler);
    router.any(r"/add_list", add_list_handler);
    router.any(r"/remove_list_user", remove_list_user_handler);
    router.get(r"/", show_all_lists_handler);
    
    println!("Running.");
    let mut mount = mount::Mount::new();
    mount.mount("/", router);
    mount.mount("/static/", staticfile::Static::new(
        std::path::Path::new("static")));
        
    let mut chain = iron::Chain::new(mount);

    let pool_reader : persistent::Read<ConnectionPool> = 
        persistent::Read::<ConnectionPool>::one(
            mysql::Pool::new("mysql://lists:lists@localhost").unwrap());
    chain.link_before(pool_reader);
    chain.link_before(RequestEnvBuilder{
//        db_pool: mysql::Pool::new("mysql://lists:lists@localhost").unwrap(),
    });
    chain.link_after(ErrorPage);
    chain.link_after(handlebars);

    println!("Serving on port 2345");
    iron::Iron::new(chain).http("0.0.0.0:2345").unwrap();
}
