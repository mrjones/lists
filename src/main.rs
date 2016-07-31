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

struct RequestEnv {
    user_id: String,
}

struct RequestEnvBuilder;
impl iron::typemap::Key for RequestEnvBuilder { type Value = RequestEnv; }

impl iron::BeforeMiddleware for RequestEnvBuilder {
    fn before(&self, req: &mut iron::request::Request) -> iron::IronResult<()> {
        let mut user_id = "".to_string();
        {
            let params = &req.get_ref::<params::Params>().unwrap();
            match params.get("user_id") {
                Some(&params::Value::String(ref id_str)) => {
                    user_id = id_str.to_string()
                },
                _ => ()
            }
        }

        req.extensions.insert::<RequestEnvBuilder>(RequestEnv{
            user_id: user_id,
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

fn main_page_handler(req: &mut iron::request::Request) -> iron::IronResult<iron::response::Response> {
//    let conn = &req.get::<Read<ConnectionPool>>().unwrap();

    let env = &req.extensions.get::<RequestEnvBuilder>().unwrap();
    if env.user_id.is_empty() {
        return Ok(iron::response::Response::with(
            (iron::status::NotFound, "Missing user_id param")));
    } else {
        return Ok(iron::response::Response::with(
            (status::Ok, format!("ID string: {}", env.user_id).to_string())));
    }

//   let params = &req.get_ref::<params::Params>().unwrap();
//
//    match params.get("user_id") {
//        Some(&params::Value::String(ref id_str)) =>
//            Ok(iron::response::Response::with(
//            (status::Ok, format!("{}", id_str).to_string()))),
//        _ => Ok(iron::response::Response::with(
//            (iron::status::NotFound, "Missing user_id param"))),
//    }
}

fn main() {
    println!("Running.");
    let mut router = Router::new();
    router.get(r"/", main_page_handler);
    router.get(r"/users", list_users_handler);
    
    let mut chain = iron::Chain::new(router);

    chain.link_before(RequestEnvBuilder);
    chain.link(persistent::Read::<ConnectionPool>::both(
        mysql::Pool::new("mysql://lists:lists@localhost").unwrap()));
    println!("Serving on port 2345");
    iron::Iron::new(chain).http("0.0.0.0:2345").unwrap();
}
