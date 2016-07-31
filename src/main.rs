extern crate iron;
extern crate mysql;
extern crate router;
extern crate params;
extern crate persistent;
extern crate rustc_serialize;

use iron::prelude::*;

use iron::status;
use persistent::Read;
use router::Router;
use rustc_serialize::json;

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

#[derive(Debug)]
struct StringError(String);

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for StringError {
    fn description(&self) -> &str { &*self.0 }
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

fn main_page_handler(req: &mut iron::request::Request) -> iron::IronResult<iron::response::Response> {
//    let conn = &req.get::<Read<ConnectionPool>>().unwrap();
    let params = &req.get_ref::<params::Params>().unwrap();

    match params.get("user_id") {
        Some(&params::Value::String(ref id_str)) =>
            Ok(iron::response::Response::with(
            (status::Ok, format!("{}", id_str).to_string()))),
        _ => Ok(iron::response::Response::with(
            (iron::status::NotFound, "Missing user_id param"))),
    }
}

fn main() {
    println!("Running.");
    let mut router = Router::new();
    router.get(r"/", main_page_handler);
    router.get(r"/users", list_users_handler);
    
    let mut chain = iron::Chain::new(router);

    chain.link(persistent::Read::<ConnectionPool>::both(
        mysql::Pool::new("mysql://lists:lists@localhost").unwrap()));
    println!("Serving on port 2345");
    iron::Iron::new(chain).http("0.0.0.0:2345").unwrap();
}
