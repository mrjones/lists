extern crate iron;
extern crate mysql;
extern crate router;
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

fn main_page_handler(req: &mut iron::request::Request) -> iron::IronResult<iron::response::Response> {
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

fn main() {
    println!("Running.");
    let mut router = Router::new();
    router.get(r"/", main_page_handler);
    
    let mut chain = iron::Chain::new(router);

    chain.link(persistent::Read::<ConnectionPool>::both(
        mysql::Pool::new("mysql://lists:lists@localhost").unwrap()));
    println!("Serving on port 2345");
    iron::Iron::new(chain).http("0.0.0.0:2345").unwrap();
}
