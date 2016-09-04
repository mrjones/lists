extern crate handlebars_iron;
#[macro_use]
extern crate iron;
extern crate mount;
extern crate mysql;
extern crate router;
extern crate persistent;
extern crate plugin;
#[macro_use]
extern crate rustful;
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

mod data;
mod model;
mod result;
mod util;

use model::*;
use result::ListsError;
use result::ListsResult;
use util::to_vector;

#[derive(Copy, Clone)]
pub struct DbHandle;
impl iron::typemap::Key for DbHandle {
    type Value = data::Db;
}

//
// Iron-specific information
//

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

fn get_user(params: &std::collections::BTreeMap<String, String>, db: &data::Db) -> iron::IronResult<User> {
    let id_str = itry!(params.get("user_id").ok_or(
        ListsError::MissingParam("user_id".to_string())));
    let id_int = itry!(id_str.parse::<i64>());
    return db.lookup_user(id_int).map_err(into_iron_error);
}

impl iron::BeforeMiddleware for RequestEnvBuilder {
    fn before(&self, req: &mut iron::request::Request) -> iron::IronResult<()> {
        let user;
        {
            let db = &req.extensions.get::<persistent::Read<DbHandle>>().unwrap();
            let params = params_map(req);
            user = get_user(&params, db);
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

fn pick_user_immutable_handler(req: &iron::request::Request) -> iron::IronResult<iron::response::Response> {
    let db = req.extensions.get::<persistent::Read<DbHandle>>().unwrap();
    let users = itry!(db.fetch_all_users());

    let mut data : std::collections::BTreeMap<String, Json> =
        std::collections::BTreeMap::new();
    data.insert("users".to_string(), to_json(&users));

    let mut response = iron::response::Response::new();
    response
        .set_mut(handlebars_iron::Template::new("pick-user", data))
        .set_mut(status::Ok);
    return Ok(response);
}


fn show_all_lists(user: &User, db: &data::Db) -> iron::IronResult<iron::response::Response> {

    let mut data : std::collections::BTreeMap<String, Json> =
        std::collections::BTreeMap::new();
    data.insert("lists".to_string(),
                itry!(db.fetch_all_lists(user)).to_json());
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
            let db = &req.extensions.get::<persistent::Read<DbHandle>>().unwrap();
            return show_all_lists(user, db);
        },
    }
}

fn show_one_list_handler(req: &mut iron::request::Request) -> iron::IronResult<iron::response::Response> {
    let db = &req.get::<persistent::Read<DbHandle>>().unwrap();
    let conn = db.conn.as_ref();
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
    let db = &req.get::<persistent::Read<DbHandle>>().unwrap();
    let env = &req.extensions().get::<RequestEnvBuilder>().unwrap();
    let mut conn = db.conn.get_conn().unwrap();

    match env.user {
        Err(_) => return pick_user_immutable_handler(req),
        Ok(ref user) => {
            let mut parse = url::form_urlencoded::parse(body.as_bytes());
            let body_params = parse_to_map(&mut parse);

            let name = itry!(body_params.get("name").ok_or(
                ListsError::MissingParam("name".to_string())));

            itry!(conn.prep_exec("INSERT INTO lists.lists (name) VALUES (?)", (name,)));
            itry!(conn.prep_exec("INSERT INTO lists.list_users (list_id, user_id) VALUES (LAST_INSERT_ID(), ?)", (user.id,)));
            return show_all_lists(user, db);
        }
    }    
}

fn add_list_item_json_handler(req: &mut iron::request::Request) -> iron::IronResult<iron::response::Response> {
    let body = read_body(req);
    let db = &req.get::<persistent::Read<DbHandle>>().unwrap();
    let conn = db.conn.as_ref();
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
    let db = &req.get::<persistent::Read<DbHandle>>().unwrap();
    let conn = db.conn.as_ref();
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
    let db = &req.get::<persistent::Read<DbHandle>>().unwrap();
    let conn = db.conn.as_ref();
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
    let db = &req.get::<persistent::Read<DbHandle>>().unwrap();
    let conn = db.conn.as_ref();
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

//
// Rustful-specific functionality
//

struct ServerContext {
//    conn_pool: Box<mysql::Pool>,
    db: data::Db,
}

impl ServerContext {
    fn new(conn_pool: mysql::Pool) -> ServerContext {
        return ServerContext {
            db: data::Db{conn: Box::new(conn_pool)},
        }
    }
}

fn list_users(server_context: &ServerContext, _: rustful::Context) -> ListsResult<Box<ToJson>> {
    match server_context.db.fetch_all_users() {
        Ok(users) => return Ok(Box::new(users)),
        Err(e) => {
            println!("DB Error: {:?}", e);
            return Err(ListsError::DatabaseError);
        },
    }
}

fn all_lists(server_context: &ServerContext, user: &User, _: rustful::Context) -> ListsResult<Box<ToJson>> {
    match server_context.db.fetch_all_lists(user) {
        Ok(lists) => return Ok(Box::new(lists)),
        Err(e) => {
            println!("DB Error: {:?}", e);
            return Err(ListsError::DatabaseError);
        },
    }
}

fn one_list(server_context: &ServerContext, user: &User, context: rustful::Context) -> ListsResult<Box<ToJson>> {
    let list_id = context.variables.get("list_id")
        .expect("no list_id param")
        .to_string().parse::<i64>()
        .expect("couldn't parse list_id");
    match server_context.db.lookup_list(list_id) {
        Ok(list) => return Ok(Box::new(list)),
        Err(e) => {
            println!("DB Error: {:?}", e);
            return Err(ListsError::DatabaseError);
        },
    }
}

enum Api {
    StaticFile{
        filename: &'static str
    },
    LoggedInHandler {
        // TODO(mrjones): "Encodable" would be nicer than "ToJson"...
        handler: fn(&ServerContext, &User, rustful::Context) -> ListsResult<Box<ToJson>>
    },
    LoggedOutHandler{
        handler: fn(&ServerContext, rustful::Context) -> ListsResult<Box<ToJson>>
    }
}

impl rustful::Handler for Api {
    fn handle_request(&self, context: rustful::Context, response: rustful::Response) {
        match *self {
            Api::StaticFile { ref filename } => {
                let res = response.send_file(filename)
                    .or_else(|e| e.send_not_found("the file was not found"))
                    .or_else(|e| e.ignore_send_error());
            },
            Api::LoggedOutHandler { ref handler } => {
                let server_context : &ServerContext =
                    context.global.get().expect("Couldn't get server_context");
                match handler(server_context, context) {
                    Ok(obj) => response.send(rustc_serialize::json::encode(
                        &obj.to_json()).unwrap()),
                    Err(err) => println!("ERROR! {:?}", err),
                }
            },
            Api::LoggedInHandler { ref handler } => {
                let server_context : &ServerContext =
                    context.global.get().expect("Couldn't get server_context");

                let user_id = context.variables.get("user_id")
                    .expect("no user_id param")
                    .to_string()
                    .parse::<i64>()
                    .expect("couldn't parse user_id");
                let user = server_context.db.lookup_user(user_id)
                    .expect("couldn't look up user");

                match handler(server_context, &user, context) {
                    Ok(obj) => response.send(rustc_serialize::json::encode(
                        &obj.to_json()).unwrap()),
                    Err(err) => println!("ERROR! {:?}", err),
                }
            }
        }
    }
}

fn serve_rustful(port: u16) {
    let my_router = insert_routes!{
        rustful::TreeRouter::new() => {
            Get: Api::StaticFile{filename: "static/index.html"},
            "/static/app.js" => {
                Get: Api::StaticFile{filename: "static/app.js"},
            },
            "/users" => {
                Get: Api::LoggedOutHandler{handler: list_users},
            },
            "/lists/:user_id" => {
                Get: Api::LoggedInHandler{handler: all_lists},
                "/list/:list_id" => {
                    Get: Api::LoggedInHandler{handler: one_list},
                },
            }
            
        }
    };

    let server_context = ServerContext::new(
        mysql::Pool::new("mysql://lists:lists@localhost").unwrap());
    
    match (rustful::Server{
        handlers: my_router,
        global: Box::new(server_context).into(),
        host: rustful::server::Host::any_v4(port),
        ..rustful::Server::default()
    }.run()) {
        Ok(_server) => println!("Serving rustful on port {}", port),
        Err(_) => println!("Could not start rustful server.")
    }
}

fn main() {
    let mut handlebars = handlebars_iron::HandlebarsEngine::new();
    handlebars.add(Box::new(
        handlebars_iron::DirectorySource::new("./templates/", ".html")));
    if let Err(r) = handlebars.reload() {
        panic!("{:?}", r);
    }

    std::thread::spawn(|| { serve_rustful(2346); });
    
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

    let handle_reader : persistent::Read<DbHandle> = 
        persistent::Read::<DbHandle>::one(
            data::Db{
                conn: Box::new(mysql::Pool::new("mysql://lists:lists@localhost").unwrap()),
            });
    chain.link_before(handle_reader);
    chain.link_before(RequestEnvBuilder{
//        db_pool: mysql::Pool::new("mysql://lists:lists@localhost").unwrap(),
    });
    chain.link_after(ErrorPage);
    chain.link_after(handlebars);
    
    println!("Serving iron on port 2345");
    iron::Iron::new(chain).http("0.0.0.0:2345").unwrap();
}
