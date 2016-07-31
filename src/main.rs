extern crate iron;
extern crate mysql;
extern crate router;
extern crate r2d2;
extern crate r2d2_mysql;
extern crate persistent;
extern crate rustc_serialize;

use iron::prelude::*;

use iron::status;
use persistent::Read;
use router::Router;
use rustc_serialize::json;
use std::boxed::Box;
use std::io;

#[derive(Copy, Clone)]
pub struct R2d2ConnectionPool;
impl iron::typemap::Key for R2d2ConnectionPool {
    type Value = r2d2::Pool<r2d2_mysql::MysqlConnectionManager>;
}

#[derive(Copy, Clone)]
pub struct NativeConnectionPool;
impl iron::typemap::Key for NativeConnectionPool {
    type Value = mysql::Pool;
}



// maps to "Users" table in MySql
#[derive(Debug, PartialEq, Eq, RustcEncodable)]
struct User {
    id: i64,
    name: String,
}

fn main_page_handler(req: &mut iron::request::Request) -> iron::IronResult<iron::response::Response> {

//    let db_conn_pool = &req.get::<Read<R2d2ConnectionPool>>().unwrap();
//    let mut conn : r2d2::PooledConnection<r2d2_mysql::MysqlConnectionManager> =
//        db_conn_pool.get().unwrap();

    let conn = &req.get::<Read<NativeConnectionPool>>().unwrap();

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

//    let result : mysql::Result<mysql::QueryResult> =
//        conn.prep_exec("SELECT id, name FROM lists.users", ());

//    for row in conn.prep_exec("SELECT id, name FROM lists.users", ()).unwrap() {
//        let (id, name) : (i64, String) = mysql::from_row(row.unwrap());
//        println!("Name: {}, Id: {}", name, id);
//    }

//    let users : Vec<User> = vec![];

    Ok(iron::response::Response::with(
        (status::Ok, json::encode(&users).unwrap())))
}

fn main() {
    println!("Running...");
//    let pool = my::Pool::new("mysql://lists:lists@localhost").unwrap();
//    for row in pool.prep_exec("SELECT id, name FROM lists.users", ()).unwrap() {
//        let (id, name) : (i64, String) = mysql::from_row(row.unwrap());
//        println!("Name: {}, Id: {}", name, id);
//    }
    let mut router = Router::new();
    router.get(r"/", main_page_handler);
    
    let mut chain = iron::Chain::new(router);

    let db_conn_pool_config = r2d2::Config::default();
    let conn_manager = r2d2_mysql::MysqlConnectionManager::new(
        "mysql://lists:lists@localhost").unwrap();

    chain.link(persistent::Read::<R2d2ConnectionPool>::both(
        r2d2::Pool::new(db_conn_pool_config, conn_manager).unwrap()));
    chain.link(persistent::Read::<NativeConnectionPool>::both(
        mysql::Pool::new("mysql://lists:lists@localhost").unwrap()));
    iron::Iron::new(chain).http("0.0.0.0:2345").unwrap();
}
