extern crate handlebars_iron;
extern crate iron;
extern crate mysql;
extern crate router;
extern crate params;
extern crate persistent;
extern crate plugin;
extern crate rustc_serialize;

use iron::prelude::*;

use iron::status;
use persistent::Read;
use plugin::Extensible;
use router::Router;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;

#[derive(Copy, Clone)]
pub struct ConnectionPool;
impl iron::typemap::Key for ConnectionPool {
    type Value = mysql::Pool;
}

// maps to "Users" table in MySql
#[derive(Debug, PartialEq, Eq, RustcEncodable)]
struct User {
    id: i64,
    name: String,
}

// maps to "Lists" table in MySql
#[derive(Debug, PartialEq, Eq, RustcEncodable)]
struct List {
    id: i64,
    name: String,
}

impl ToJson for List {
    fn to_json(&self) -> Json {
            let mut m: std::collections::BTreeMap<String, Json> = std::collections::BTreeMap::new();
            m.insert("name".to_string(), self.name.to_json());
            m.insert("id".to_string(), self.id.to_json());
            m.to_json()
    }
}

// maps to "Items" table in MySql
#[derive(Debug, PartialEq, Eq, RustcEncodable)]
struct Item {
    id: i64,
    name: String,
    description: String,
}

impl ToJson for Item {
    fn to_json(&self) -> Json {
            let mut m: std::collections::BTreeMap<String, Json> = std::collections::BTreeMap::new();
            m.insert("id".to_string(), self.id.to_json());
            m.insert("name".to_string(), self.name.to_json());
            m.insert("description".to_string(), self.description.to_json());
            m.to_json()
    }
}

#[derive(Debug)]
enum LoginError {
    MissingParam,
    InvalidParam,
    DatabaseError,
    DoesNotExist,

    Unknown,
}

struct RequestEnv {
    user: std::result::Result<User, LoginError>,
}

struct RequestEnvBuilder;
impl iron::typemap::Key for RequestEnvBuilder { type Value = RequestEnv; }


fn lookup_user(id: i64, db_conn: &mysql::Pool) -> std::result::Result<User, LoginError> {
    match db_conn.prep_exec("SELECT id, name FROM lists.users WHERE id = ?", (id,)) {
        Err(_) => return Err(LoginError::DatabaseError),
        Ok(mut result) => match result.next() {
            None => return Err(LoginError::DoesNotExist),
            Some(row_result) => match row_result {
                Err(_) => return Err(LoginError::DatabaseError),
                Ok(row) => {
                    let (id, name) = mysql::from_row(row);
                    return Ok(User{
                        id: id,
                        name: name,
                    })
                }
            }
        }
    }
}

fn get_user(params: &params::Map, db_conn: &mysql::Pool) -> std::result::Result<User, LoginError> {
    match params.get("user_id") {
        Some(&params::Value::String(ref id_str)) => {
            match id_str.parse::<i64>() {
                Ok(id_int) => return lookup_user(id_int, db_conn),
                Err(_) => return Err(LoginError::InvalidParam),
            }
        },
        _ => return Err(LoginError::MissingParam)
    }

    return Err(LoginError::Unknown);
}

impl iron::BeforeMiddleware for RequestEnvBuilder {
    fn before(&self, req: &mut iron::request::Request) -> iron::IronResult<()> {
        let user;
        {
            let conn = &req.get::<Read<ConnectionPool>>().unwrap();
            let params = &req.get_ref::<params::Params>().unwrap();
            user = get_user(params, conn);
        }

        req.extensions.insert::<RequestEnvBuilder>(RequestEnv{
            user: user,
        });

        return Ok(());
    }
}

fn list_users_handler(req: &mut iron::request::Request) -> iron::IronResult<iron::response::Response> {
    let conn = &req.get::<Read<ConnectionPool>>().unwrap();

    let users: Vec<User> =
        conn.prep_exec("SELECT id, name FROM lists.users", ())
        .map(|res| {
            res.map(|x| x.unwrap())
                .map(|row | {
                    let (id, name) = mysql::from_row(row);
                    User {
                        id: id,
                        name: name,
                    }
                }).collect()
        }).unwrap();

    Ok(iron::response::Response::with(
        (status::Ok, json::encode(&users).unwrap())))
}

fn show_all_lists_handler(req: &mut iron::request::Request) -> iron::IronResult<iron::response::Response> {
    let conn = &req.get::<Read<ConnectionPool>>().unwrap();

    let env = &req.extensions.get::<RequestEnvBuilder>().unwrap();
    match env.user {
        Err(ref err) => return Ok(iron::response::Response::with(
            (iron::status::NotFound, format!("ERROR: {:?}", err).to_string()))),
        Ok(ref user) => {
            let lists: Vec<List> =
                conn.prep_exec("SELECT lists.lists.id, lists.lists.name FROM lists.list_users LEFT JOIN lists.lists ON lists.list_users.list_id = lists.lists.id WHERE lists.list_users.user_id = ?", (user.id,))
                .map(|res| {
                    res.map(|x| x.unwrap())
                        .map(|row | {
                            let (id, name) = mysql::from_row(row);
                            List {
                                id: id,
                                name: name,
                            }
                        }).collect()
                }).unwrap();
            

            let mut data : std::collections::BTreeMap<String, Json> =
                std::collections::BTreeMap::new();
            data.insert("lists".to_string(), lists.to_json());
            data.insert("user_id".to_string(), user.id.to_json());

            let mut response = iron::response::Response::new();
            response
                .set_mut(handlebars_iron::Template::new("my-lists", data))
                .set_mut(status::Ok);
            
            return Ok(response);
        },
    }
}

fn get_param(name: &str, params: &params::Map) -> std::option::Option<String> {
    match params.get(name) {
        Some(&params::Value::String(ref str)) => Some(str.to_string()),
        _ => None,
    }
}

fn show_one_list_handler(req: &mut iron::request::Request) -> iron::IronResult<iron::response::Response> {
    let conn = &req.get::<Read<ConnectionPool>>().unwrap();
    let params = &req.get::<params::Params>().unwrap();
    let env = &req.extensions_mut().get::<RequestEnvBuilder>().unwrap();

    match env.user {
        Err(ref err) => return Ok(iron::response::Response::with(
            (iron::status::NotFound, format!("ERROR: {:?}", err).to_string()))),
        Ok(ref user) => {
            // TODO(mrjones): check permissions?
            let mut data : std::collections::BTreeMap<String, Json> =
                std::collections::BTreeMap::new();

            let list_id = get_param("list_id", &params).unwrap(); 
            // Fetch metadata for list
            // TODO(mrjones): fetch name from DB
            data.insert("id".to_string(), list_id.to_json());
                
            // Fetch items for list
            let items: Vec<Item> =
                conn.prep_exec("SELECT id, name, description FROM lists.items WHERE list_id = ?", (list_id,))
                .map(|res| {
                    res.map(|x| x.unwrap())
                        .map(|row | {
                            let (id, name, description) = mysql::from_row(row);
                            Item {
                                id: id,
                                name: name,
                                description: description,
                            }
                        }).collect()
                }).unwrap();
            data.insert("items".to_string(), items.to_json());

            let mut response = iron::response::Response::new();
            response
                .set_mut(handlebars_iron::Template::new("one-list", data))
                .set_mut(status::Ok);
            
            return Ok(response);
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

    println!("Running.");
    let mut router = Router::new();
    router.get(r"/", show_all_lists_handler);
    router.get(r"/list", show_one_list_handler);
    router.get(r"/users", list_users_handler);
    
    let mut chain = iron::Chain::new(router);

    chain.link(persistent::Read::<ConnectionPool>::both(
        mysql::Pool::new("mysql://lists:lists@localhost").unwrap()));
    chain.link_before(RequestEnvBuilder);
    chain.link_after(handlebars);

    println!("Serving on port 2345");
    iron::Iron::new(chain).http("0.0.0.0:2345").unwrap();
}
