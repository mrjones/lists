extern crate std;
extern crate websocket;

use std::borrow::Borrow;
use websocket::Sender;
use websocket::Receiver;

pub struct StreamManager {
    port: u16,
    streams: std::sync::Mutex<std::collections::HashMap<i64, std::sync::Arc<OneStream>>>,
    next_id: std::sync::Mutex<i64>,
}

impl StreamManager {
    pub fn new(port: u16) -> StreamManager {
        return StreamManager{
            port: port,
            streams: std::sync::Mutex::new(std::collections::HashMap::new()),
            next_id: std::sync::Mutex::new(0),
        };
    }    

    fn next_id(&self) -> i64 {
        let mut id = self.next_id.lock().unwrap();
        let ret = *id;
        *id += 1;
        return ret;
    }

    pub fn notify_observers(&self, list_id: i64, message: &str) {
        println!("Got a message for {}", list_id);
        for (_, stream) in self.streams.lock().unwrap().iter() {
            println!("Stream {} is watching {}", stream.id, list_id);
            if *stream.watch_target.lock().unwrap() == Some(list_id) {
                stream.send_message(message);
            }
        }
    }
    
    pub fn serve(&self) {
        let server = websocket::Server::bind(("0.0.0.0", self.port)).unwrap();
        println!("Serving websockets on port {}", self.port);
        for connection in server {
            let id = self.next_id();
            let stream = std::sync::Arc::new(
                OneStream::handle(id, connection.unwrap()));
            self.streams.lock().unwrap().insert(id, stream.clone());
            std::thread::spawn(move || {
                stream.process_incoming();
            });
        }
    }
}

type WebSocketConnection = websocket::server::Connection<websocket::stream::WebSocketStream, websocket::stream::WebSocketStream>;

type WebSocketSender = websocket::sender::Sender<websocket::stream::WebSocketStream>;
type WebSocketReceiver = websocket::receiver::Receiver<websocket::stream::WebSocketStream>;

struct OneStream {
    sender: std::sync::Mutex<WebSocketSender>,
    receiver: std::sync::Mutex<WebSocketReceiver>,
    ip: std::net::SocketAddr,
    id: i64,
    watch_target: std::sync::Mutex<Option<i64>>,
}

impl OneStream {
    fn handle(id: i64, connection: WebSocketConnection) -> OneStream {
        let request = connection.read_request().unwrap();
        let mut client = request.accept().send().unwrap(); 
        let ip = client.get_mut_sender().get_mut().peer_addr().unwrap();
        let (sender, receiver) = client.split();
        println!("Connection with {}", ip);
        return OneStream{
            id: id,
            ip: ip,
            sender: std::sync::Mutex::new(sender),
            receiver: std::sync::Mutex::new(receiver),
            watch_target: std::sync::Mutex::new(None),
        }
    }

    fn send_message(&self, payload: &str) {
        println!("Pusing message to socket: {}", payload);
        self.sender.lock().unwrap().send_message(
            &websocket::Message::text(payload)).unwrap();
    }

    fn process_text(&self, payload_bytes: &[u8]) {
        let payload = std::str::from_utf8(payload_bytes).unwrap();
        println!("Text from {}: {}", self.ip, payload);
                    
        // TODO: use a proto
        let parts: Vec<&str> = payload.split(":").collect();
        if parts.len() != 2 {
            self.sender.lock().unwrap().send_message(
                &websocket::Message::text(
                    format!("Malformed Message: {}", payload))).unwrap();
            return;
        }
                    
        let command = parts[0];
        let argument = parts[1];
                    
        println!("{} sent {}({})", self.ip, command, argument);

        assert_eq!(command, "watch");
        let new_target = argument.parse::<i64>().unwrap();
        *self.watch_target.lock().unwrap() = Some(new_target);

        println!("{} watching {:?}", self.id, *self.watch_target.lock().unwrap());
        
        self.sender.lock().unwrap().send_message(
            &websocket::Message::text("Got it!")).unwrap();
    }
    
    fn process_incoming(&self) {
        for message in self.receiver.lock().unwrap().incoming_messages() {
            let message: websocket::Message = message.unwrap();
            match message.opcode {
                websocket::message::Type::Close => {
                    self.sender.lock().unwrap().send_message(
                        &websocket::Message::close()).unwrap();
                    *self.watch_target.lock().unwrap() = None;
                    println!("Closed connection to {}", self.ip);
                    return;
                },
                websocket::message::Type::Ping => {
                    self.sender.lock().unwrap().send_message(
                        &websocket::Message::pong(message.payload)).unwrap();
                    println!("Ping from {}", self.ip);
                },
                websocket::message::Type::Pong => {
                    println!("Pong from {}", self.ip);
                },
                websocket::message::Type::Text => {
                    let payload_bytes: &[u8] = message.payload.borrow();
                    self.process_text(payload_bytes);
                },
                websocket::message::Type::Binary => {
                    println!("Binary from {}: {:?}", self.ip, message.payload);
                },
            }
        }
    }
}
    
