extern crate mysql;
extern crate protobuf;
#[macro_use]
extern crate rustful;
extern crate rustc_serialize;
extern crate url;
extern crate websocket;

use rustc_serialize::json::ToJson;

mod annotations;
mod cache;
mod data;
mod model;
mod result;
mod scrape;
mod sockets_api;
mod storage_format;
mod streaming;
mod streeteasy;
mod workqueue;

use model::*;
use result::ListsError;
use result::ListsResult;

struct ServerContext {
    db: std::sync::Arc<std::sync::Mutex<data::Db>>,
    expander: annotations::AnnotationExpander,
    stream_manager: std::sync::Arc<streaming::StreamManager>,
}

impl ServerContext {
    fn new(conn_pool: mysql::Pool,
           work_ready: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Sender<()>>>,
           item_updated: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Sender<annotations::ItemUpdate>>>,
           stream_manager: std::sync::Arc<streaming::StreamManager>) -> ServerContext {
        let db = std::sync::Arc::new(std::sync::Mutex::new(
            data::Db{conn: Box::new(conn_pool)}));
        let workqueue = std::sync::Arc::new(std::sync::Mutex::new(
            workqueue::DbWorkQueue::new(
                std::time::Duration::new(60, 0),
                "annotations",
                db.clone())));
        return ServerContext {
            db: db.clone(),
            expander: annotations::AnnotationExpander::new(
                db, workqueue, work_ready, item_updated),
            stream_manager: stream_manager,
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
    return Ok(Box::new(try!(server_context.db.lock().unwrap().fetch_all_users())));
}

fn all_lists(server_context: &ServerContext, user: &User, _: rustful::Context) -> ListsResult<Box<ToJson>> {
    return Ok(Box::new(try!(server_context.db.lock().unwrap().fetch_all_lists(user))));
}

fn one_list(server_context: &ServerContext, _: &User, context: rustful::Context) -> ListsResult<Box<ToJson>> {
    let list_id = try!(lookup_param::<i64>("list_id", &context));
    let (db_list, items, user_annotations, auto_annotations) =
        try!(server_context.db.lock().unwrap().lookup_list(list_id));
;

    let list = FullList{
        name: db_list.name,
        items: annotations::parse_and_attach_annotations(
            items, user_annotations, auto_annotations)
    };
    
//    expand_list_annotations(&mut list, &server_context.streeteasy);
    return Ok(Box::new(list));
}

fn list_accessors(server_context: &ServerContext, _: &User, context: rustful::Context) -> ListsResult<Box<ToJson>> {
    let list_id = try!(lookup_param::<i64>("list_id", &context));
    return Ok(Box::new(try!(server_context.db.lock().unwrap().fetch_list_accessors(list_id))));
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
    try!(server_context.db.lock().unwrap().add_user_to_list(list_id, new_user.id));
    
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
    try!(server_context.db.lock().unwrap().remove_user_from_list(list_id, old_user.id));
    
    return list_accessors(server_context, user, context);
}

fn delete_list(server_context: &ServerContext, _: &User, context: rustful::Context) -> ListsResult<Box<ToJson>> {
    let list_id = try!(lookup_param::<i64>("list_id", &context));
    println!("delete_list :: {}", list_id);
    return Ok(Box::new(try!(server_context.db.lock().unwrap().delete_list(list_id))));
}

fn delete_item(server_context: &ServerContext, _: &User, context: rustful::Context) -> ListsResult<Box<ToJson>> {
    // TODO: check item belongs to list
    // TODO: check user can edit list

    let item_id = try!(lookup_param::<i64>("item_id", &context));
    return Ok(Box::new(try!(server_context.db.lock().unwrap().delete_item(item_id))));
}

fn add_list(server_context: &ServerContext, user: &User, mut context: rustful::Context) -> ListsResult<Box<ToJson>> {
    #[derive(Debug, RustcDecodable)]
    struct NewList {
        name: String,
    }
    let list : NewList = context.body.decode_json_body().expect("decoding item");
    println!("add_list :: they posted: {:?}", list);

    // TODO: lift out a level?
    let db_list = try!(server_context.db.lock().unwrap().add_list(&list.name, user.id));
    
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
    let db_item = try!(server_context.db.lock().unwrap().add_item(list_id, &item.name, &item.description));
    
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
    let saved_annotation = try!(server_context.db.lock().unwrap().add_annotation(item_id, &annotation.kind, &annotation.body));

    server_context.expander.generate_auto_annotations(
        list_id, item_id, saved_annotation.id,
        &saved_annotation.kind, &saved_annotation.body);

    let (item, user_annotations, auto_annotations) =
        try!(server_context.db.lock().unwrap().lookup_list_item(list_id, item_id));
    
    return Ok(Box::new(annotations::parse_and_attach_annotations_single(
        item, user_annotations, auto_annotations)));
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
                let server_context : &std::sync::Arc<ServerContext> =
                    context.global.get().expect("Couldn't get server_context");
                match handler(server_context, context) {
                    Ok(obj) => response.send(rustc_serialize::json::encode(
                        &obj.to_json()).unwrap()),
                    Err(err) => println!("ERROR! {:?}", err),
                }
            },
            Api::LoggedInHandler { ref handler } => {
                let server_context : &std::sync::Arc<ServerContext> =
                    context.global.get().expect("Couldn't get server_context");

                let user_id = lookup_param::<i64>("user_id", &context).unwrap();
                let user = server_context.db.lock().unwrap().lookup_user(user_id)
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

fn serve_websockets(
     stream_manager: std::sync::Arc<streaming::StreamManager>) {
    std::thread::spawn(move || {
        stream_manager.serve();
    });
}

fn serve_rustful(
    port: u16,
    stream_manager: std::sync::Arc<streaming::StreamManager>) {
    
    let my_router = insert_routes!{
        rustful::TreeRouter::new() => {
            Get: Api::StaticFile{filename: "static/index.html"},
            "/static/app.js" => {
                Get: Api::StaticFile{filename: "static/app.js"},
            },
            "/static/style.css" => {
                Get: Api::StaticFile{filename: "static/style.css"},
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

    let (work_sender, work_receiver) = std::sync::mpsc::channel::<()>();
    let (item_updated_sender, item_updated_receiver) =
        std::sync::mpsc::channel::<annotations::ItemUpdate>();

    let server_context = std::sync::Arc::new(ServerContext::new(
        mysql::Pool::new("mysql://lists:lists@localhost").unwrap(),
        std::sync::Arc::new(std::sync::Mutex::new(work_sender)),
        std::sync::Arc::new(std::sync::Mutex::new(item_updated_sender)),
        stream_manager));

    let mut global = rustful::server::Global::default();
    global.insert(server_context.clone());

    let sc = server_context.clone();
    std::thread::spawn(move || {
        println!("Worker thread running...");
        loop {
            sc.expander.process_work_queue();
            let _ = work_receiver.recv_timeout(std::time::Duration::new(60, 0));
        }
    });

    std::thread::spawn(move || {
        println!("Stream relay thread running...");
        loop {
            let update = item_updated_receiver.recv().unwrap();
            println!("Relay thread passing on {:?}", update);

            let (item, user_annotations, auto_annotations) =
                server_context.db.lock().unwrap().lookup_list_item(
                    update.list_id, update.item_id).unwrap();
    
            let complete_item = annotations::parse_and_attach_annotations_single(
                item, user_annotations, auto_annotations);

            let update_text = rustc_serialize::json::encode(
                        &complete_item.to_json()).unwrap();
            
            server_context.stream_manager.notify_observers(
                update.list_id, &update_text);
        }
    });
    
    match (rustful::Server{
        handlers: my_router,
        global: global,
        host: rustful::server::Host::any_v4(port),
        ..rustful::Server::default()
    }.run()) {
        Ok(_server) => println!("Serving rustful on port {}", port),
        Err(_) => println!("Could not start rustful server.")
    }
}


fn main() {
    let stream_manager = std::sync::Arc::new(
        streaming::StreamManager::new(2347));
    serve_websockets(stream_manager.clone());
    serve_rustful(2346, stream_manager);
}
