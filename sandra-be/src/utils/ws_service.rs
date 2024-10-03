use actix::prelude::{ Actor, Handler, Recipient };
use std::collections::HashMap;
use uuid::Uuid;
use actix::StreamHandler;
use actix_web::{ get, web::Payload, Error, HttpResponse, HttpRequest };
use actix_web_actors::ws;
use std::time::Instant;
use std::sync::RwLock;
use actix::prelude::Message;
use actix_web_actors::ws::Message::Text;
use serde::{ Deserialize, Serialize };

// #[derive(Message)]
// #[rtype(result = "String")]
// pub struct WsMessage(pub String);
// type Socket = Recipient<WsMessage>;

// pub struct WsParty {
//     name: String,
//     subd_users: RwLock<HashMap<String, WsUser>>,
// }

// impl WsParty {
//     pub fn new(name: String) -> WsParty {
//         WsParty {
//             name,
//             subd_users: RwLock::new(HashMap::new()),
//         }
//     }
//     pub fn broadcast(msg: &str) {}

//     pub fn add_user() {}

//     pub fn remove_user() {}
// }

pub struct WsUser {
  username: String,
  gid: Uuid,
}

impl WsUser {
  fn send_message(msg: &str) {}
  fn remove_user() {}
}
pub struct WsHandler {
  users: RwLock<HashMap<String, WsUser>>,
  parties: RwLock<HashMap<String, WsUser>>,
  hb: Instant,
  listener_cb: Option<Box<dyn Fn(WsJSON)>>,
}

impl Default for WsHandler {
  fn default() -> WsHandler {
    WsHandler {
      users: RwLock::new(HashMap::new()),
      parties: RwLock::new(HashMap::new()),
      hb: Instant::now(),
      listener_cb: None,
    }
  }
}

impl WsHandler {
  // fn send_message(&self, message: &str, id_to: &Uuid) {
  //     if let Some(socket_recipient) = self.users.get(id_to) {
  //         let _ = socket_recipient.do_send(WsMessage(message.to_owned()));
  //     } else {
  //         print!("attempting to send msg but no user id found")
  //     }
  // }

  fn broadcast(&self, msg: &str) {}

  fn add_user(&self, user: WsUser) {}
  fn add_party_group(&self, name: String) {
    // self.parties.
  }
  fn set_listener(&mut self, callback: Box<dyn Fn(WsJSON)>) {
    self.listener_cb = Some(callback);
  }
}

impl Actor for WsHandler {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    // Logic when a user joins
    println!("User {} has joined.", "self.id");

    // Optionally send a welcome message
    ctx.text(format!("Welcome! You are user #{}", "self.id"));

    // You could also broadcast to other users here
    // e.g., broadcast_message("A new user has joined!");
  }

  fn stopped(&mut self, _: &mut Self::Context) {
    // Logic when a user disconnects
    println!("User {} has left.", "self.id");
  }
}

// #[get("/ws")]
// pub async fn start_connection(req: HttpRequest, stream: Payload) -> Result<HttpResponse, Error> {
//   let ws = WsHandler::default();

//   let resp = ws::start(ws, &req, stream).expect("Could not start WS");
//   Ok(resp)
// }

// pub fn create_start_connection(
//   ws: WsHandler
// ) -> fn(req: HttpRequest, stream: Payload) -> Result<HttpResponse, Error> {
//   // #[get("/ws")]

//   async fn start_connection(req: HttpRequest, stream: Payload) -> Result<HttpResponse, Error> {
//     let resp = ws::start(ws, &req, stream).expect("Could not start WS");
//     Ok(resp)
//   }
//   start_connection
// }

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
  pub username: String,
  pub msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WsJSON {
  ev: String,
  msg: String,
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsHandler {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Ping(msg)) => {
        self.hb = Instant::now();
        ctx.pong(&msg)
      }
      Ok(ws::Message::Pong(_)) => {
        self.hb = Instant::now();
      }
      Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
      Ok(ws::Message::Close(reason)) => {
        ctx.close(reason);
        // ctx.stop()
      }
      Ok(ws::Message::Continuation(_)) => {
        // ctx.stop();
      }
      Ok(ws::Message::Nop) => (),
      Ok(Text(s)) => {
        match serde_json::from_str::<WsJSON>(&s) {
          Ok(json) => {
            println!("Parsed: {:?}", json);

            if let Some(ref callback) = self.listener_cb {
              callback(json);
            }
          }
          Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
          }
        }
      }
      Err(e) => panic!("{}", e),
    }
  }
}
