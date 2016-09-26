extern crate mysql;
#[macro_use]
extern crate rustful;
extern crate rustc_serialize;
extern crate url;

use rustc_serialize::json::ToJson;

mod cache;
mod data;
mod model;
mod result;
mod scrape;
mod streeteasy;

use model::*;
use result::ListsError;
use result::ListsResult;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::DerefMut;

struct ServerContext {
//    conn_pool: Box<mysql::Pool>,
    db: data::Db,
    streeteasy: std::sync::Mutex<streeteasy::StreetEasyClient>,
}

impl ServerContext {
    fn new(conn_pool: mysql::Pool) -> ServerContext {
        return ServerContext {
            db: data::Db{conn: Box::new(conn_pool)},
            streeteasy: std::sync::Mutex::new(
                streeteasy::StreetEasyClient::new()),
        }
    }
}

fn lookup_param<T : std::str::FromStr>(param_name: &str, context: &rustful::Context) -> ListsResult<T> {
    let param_str =
        try!(context.variables.get(param_name)
             .ok_or(ListsError::MissingParam(param_name.to_string())))
        .to_string();

    return param_str.parse::<T>().map_err(|_| {
        return ListsError::InvalidParam;
    });
}

fn all_users(server_context: &ServerContext, _: rustful::Context) -> ListsResult<Box<ToJson>> {
    return Ok(Box::new(try!(server_context.db.fetch_all_users())));
}

fn all_lists(server_context: &ServerContext, user: &User, _: rustful::Context) -> ListsResult<Box<ToJson>> {
    return Ok(Box::new(try!(server_context.db.fetch_all_lists(user))));
}

fn expand_list_annotations(list: &mut FullList, se_client: &mut streeteasy::StreetEasyClient) {
    for mut item in &mut list.items {
        expand_item_annotations(&mut item, se_client);
    }
}

fn expand_item_annotations(item: &mut FullItem, se_client: &mut streeteasy::StreetEasyClient) {
    item.streeteasy_annotations = item.link_annotations.iter().filter_map(|link| {
        if !link.url.contains("streeteasy.com") {
            return None;
        }
        let listing_result = se_client.lookup_listing(&link.url);
        return match listing_result {
            Ok(listing) => {
                let mut hasher = std::hash::SipHasher::new();
                listing.name.hash(&mut hasher);
                return Some(model::FullStreetEasyAnnotation{
                    price_usd: listing.price_usd,
                    name: listing.name,
                    hash: hasher.finish(),
                })
            },
            Err(_) => None,
        };
    }).collect();
}

fn one_list(server_context: &ServerContext, _: &User, context: rustful::Context) -> ListsResult<Box<ToJson>> {
    let list_id = try!(lookup_param::<i64>("list_id", &context));
    let mut list = try!(server_context.db.lookup_list(list_id));
    expand_list_annotations(&mut list, server_context.streeteasy.lock().unwrap().deref_mut());
    return Ok(Box::new(list));
}

fn list_accessors(server_context: &ServerContext, _: &User, context: rustful::Context) -> ListsResult<Box<ToJson>> {
    let list_id = try!(lookup_param::<i64>("list_id", &context));
    return Ok(Box::new(try!(server_context.db.fetch_list_accessors(list_id))));
}

fn add_user_to_list(server_context: &ServerContext, user: &User, mut context: rustful::Context) -> ListsResult<Box<ToJson>> {
    #[derive(Debug, RustcDecodable)]
    struct NewUser {
        id: i64,
    }
    let new_user : NewUser = context.body.decode_json_body().expect("decoding user");
    println!("add_user_to_list :: they posted: {:?}", new_user);

    // TODO: lift out a level?
    let list_id = try!(lookup_param::<i64>("list_id", &context));
    try!(server_context.db.add_user_to_list(list_id, new_user.id));
    
    return list_accessors(server_context, user, context);
}

fn remove_user_from_list(server_context: &ServerContext, user: &User, mut context: rustful::Context) -> ListsResult<Box<ToJson>> {
    #[derive(Debug, RustcDecodable)]
    struct OldUser {
        id: i64,
    }
    let old_user : OldUser = context.body.decode_json_body().expect("decoding user");
    println!("remove_user_from_list :: they posted: {:?}", old_user);

    // TODO: lift out a level?
    let list_id = try!(lookup_param::<i64>("list_id", &context));
    try!(server_context.db.remove_user_from_list(list_id, old_user.id));
    
    return list_accessors(server_context, user, context);
}

fn delete_list(server_context: &ServerContext, _: &User, context: rustful::Context) -> ListsResult<Box<ToJson>> {
    let list_id = try!(lookup_param::<i64>("list_id", &context));
    println!("delete_list :: {}", list_id);
    return Ok(Box::new(try!(server_context.db.delete_list(list_id))));
}

fn delete_item(server_context: &ServerContext, _: &User, context: rustful::Context) -> ListsResult<Box<ToJson>> {
    // TODO: check item belongs to list
    // TODO: check user can edit list

    let item_id = try!(lookup_param::<i64>("item_id", &context));
    return Ok(Box::new(try!(server_context.db.delete_item(item_id))));
}

fn add_list(server_context: &ServerContext, user: &User, mut context: rustful::Context) -> ListsResult<Box<ToJson>> {
    #[derive(Debug, RustcDecodable)]
    struct NewList {
        name: String,
    }
    let list : NewList = context.body.decode_json_body().expect("decoding item");
    println!("add_list :: they posted: {:?}", list);

    // TODO: lift out a level?
    let db_list = try!(server_context.db.add_list(&list.name, user.id));
    
    return Ok(Box::new(db_list));
}

fn add_item(server_context: &ServerContext, _: &User, mut context: rustful::Context) -> ListsResult<Box<ToJson>> {
    #[derive(Debug, RustcDecodable)]
    struct NewItem {
        name: String,
        description: String,
    }
    let item : NewItem = context.body.decode_json_body().expect("decoding item");
    println!("add_item :: they posted: {:?}", item);

    // TODO: lift out a level?
    let list_id = try!(lookup_param::<i64>("list_id", &context));
    let db_item = try!(server_context.db.add_item(list_id, &item.name, &item.description));
    
    return Ok(Box::new(FullItem{
        id: db_item.id,
        name: db_item.name,
        description: db_item.description,
        link_annotations: vec![],
        streeteasy_annotations: vec![],
        text_annotations: vec![],
    }));
}

fn add_annotation(server_context: &ServerContext, _: &User, mut context: rustful::Context) -> ListsResult<Box<ToJson>> {

    #[derive(Debug, RustcDecodable)]
    struct NewAnnotation {
        kind: String,
        body: String,

    }
    let annotation : NewAnnotation = context.body.decode_json_body().expect("decoding annotation");
    println!("add_annotation :: they posted: {:?}", annotation);

    // TODO: lift out a level?
    let list_id = try!(lookup_param::<i64>("list_id", &context));
    let item_id = try!(lookup_param::<i64>("item_id", &context));
    // TODO: check item belongs to list and user has permission
    try!(server_context.db.add_annotation(item_id, &annotation.kind, &annotation.body));

    let mut item = try!(server_context.db.lookup_list_item(list_id, item_id));
    expand_item_annotations(&mut item, server_context.streeteasy.lock().unwrap().deref_mut());
    
    return Ok(Box::new(item));
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
                let _ = response.send_file(filename)
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

                let user_id = lookup_param::<i64>("user_id", &context).unwrap();
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
                Get: Api::LoggedOutHandler{handler: all_users},
            },
            "/lists/:user_id" => {
                Get: Api::LoggedInHandler{handler: all_lists},
                "/list/" => {
                    Post: Api::LoggedInHandler{handler: add_list},
                    ":list_id" => {
                        Get: Api::LoggedInHandler{handler: one_list},
                        Delete: Api::LoggedInHandler{handler: delete_list},
                        "/items" => {
                            Post: Api::LoggedInHandler{handler: add_item},
                            "/:item_id" => {
                                Delete: Api::LoggedInHandler{handler: delete_item},
                                "/annotations" => {
                                    Post: Api::LoggedInHandler{handler: add_annotation},
                                },
                            },
                        },
                        "/accessors" => {
                            Get: Api::LoggedInHandler{handler: list_accessors},
                            Post: Api::LoggedInHandler{handler: add_user_to_list},
                            Delete: Api::LoggedInHandler{handler: remove_user_from_list},
                        },
                    },
                }
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
    serve_rustful(2346);
}
