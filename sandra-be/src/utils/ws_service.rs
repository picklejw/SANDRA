// use actix::prelude::Message;
// use actix::prelude::{ Actor, Handler, Recipient };
// use actix::StreamHandler;
// use actix_web::{ web, App, HttpServer, HttpRequest, HttpResponse };
// use actix_web_actors::ws;
// use actix_web::{ get, web::Payload, Error };
// use actix_web_actors::ws::Message::Text;
// use actix_web_actors::ws::{ self as ws_self, Message as WsMessage, ProtocolError };

// use serde::{ Deserialize, Serialize };
// use std::collections::HashMap;
// use std::sync::{ Arc, Mutex };
// use tokio::sync::RwLock;

// use std::time::Instant;
// use uuid::Uuid;
// use tokio::sync::mpsc;
// use lazy_static::lazy_static;

// use actix_web::{ web, HttpRequest, HttpResponse, Result };
// use actix_web_actors::ws;
// use std::sync::{ Arc, RwLock };
// use std::collections::HashMap;
use crate::utils::models::{ WsMsg };

use actix_web::{ rt, web, App, Error, HttpRequest, HttpResponse, HttpServer };
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;
use std::collections::HashMap;
use std::sync::{ Arc };
use tokio::sync::{ RwLock, mpsc };
use std::time::Duration;
use serde::{ Deserialize, Serialize };
// #[derive(Message)]
// #[rtype(result = "()")] // No return value
// struct SendMessage(String);
// #[rtype(result = "()")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsJSON {
  pub ev: String,
  pub msg: String,
}

#[derive(Clone)]
pub struct WsUser {
  username: String,
  pub send_msg: tokio::sync::mpsc::Sender<std::string::String>,
  gid: String,
}
// #[derive(Clone)]
// struct WsHandler {
//   clients: Arc<RwLock<HashMap<String, WsUser>>>,
// }

// impl actix::Handler<SendMessage> for WsHandler {
//   type Result = ();

//   fn handle(&mut self, msg: SendMessage, ctx: &mut Self::Context) {
//     ctx.text(msg.0); // Send the message back to the WebSocket client
//   }
// }

// impl WsHandler {
//   pub fn new(clients: Arc<RwLock<HashMap<String, WsUser>>>) -> Self {
//     WsHandler { clients }
//   }

//   pub fn send_message(&self, msg: &str) {
//     // Use the context to send a message to the WebSocket
//     // This requires a mutable context, which you may need to handle correctly
//   }

//   async fn broadcast(&self, msg: &WsJSON) {
//     // let users = self.users.read().await;
//     // for user in users.values() {
//     //   // let _ = (user.ws./)(msg.clone());
//     // }
//   }

//   async fn add_user(&self, user: WsUser) {
//     // let mut users_lock = self.users.write().await;
//     // users_lock.insert(user.username.clone(), user);
//   }

//   fn add_party_group(&self, name: String) {
//     // self.parties.
//     // self.
//   }
// }

// impl actix::Actor for WsHandler {
//   type Context = ws::WebsocketContext<Self>;

//   fn started(&mut self, ctx: &mut Self::Context) {
//     // Logic when a user joins
//     println!("User {} has joined.", "self.id");

//     let (tx, mut rx) = mpsc::channel::<String>(30);
//     let n_user = WsUser {
//       username: "TEDST".to_string(),
//       send_msg: tx,
//       gid: "S".to_string(),
//     };

//     // Clone context and clients reference
//     let clients = Arc::clone(&self.clients);
//     let ctx_c = Arc::clone(ctx); // Clone context for later use

//     // Spawn a new task for handling message reception
//     tokio::spawn(async move {
//       {
//         let mut users_map = clients.lock().unwrap(); // Locking for use
//         users_map.insert("TEMP".to_string(), n_user);
//       }

//       // Handle receiving messages asynchronously
//       while let Some(received) = rx.recv().await {
//         println!("Received: {}", received);
//         // Send message to WebSocket context
//         ctx_c.text(received);
//       }
//     });
//   }

//   // Optionally send a welcome message
//   // ctx.text(format!("Welcome! You are user #{}", "self.id"));

//   // You could also broadcast to other users here
//   // e.g., broadcast_message("A new user has joined!");

//   fn stopped(&mut self, _: &mut Self::Context) {
//     // Logic when a user disconnects
//     println!("User {} has left.", "self.id");
//   }
// }

// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsHandler {
//   fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//     match msg {
//       Ok(ws::Message::Text(text)) => {
//         // Deserialize the message
//         if let Ok(ws_json) = serde_json::from_str::<WsJSON>(&text) {
//           // Broadcast the received message to all users
// tokio::runtime::Runtime
//   ::new()
//   .unwrap()
//   .block_on(async {
//     self.broadcast(&ws_json).await;
//   })
//         }
//       }
//       Ok(ws::Message::Close(_)) => {
//         ctx.close(None);
//         // ctx.stop();
//       }
//       _ => (),
//     }
//   }

//   // fn started(&mut self, ctx: &mut Self::Context) {
//   //   println!("ANYTHING????");
//   // }
// }
pub async fn ws_index(
  req: HttpRequest,
  stream: web::Payload,
  clients: web::Data<Arc<RwLock<HashMap<String, WsUser>>>>
) -> Result<HttpResponse, actix_web::Error> {
  let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

  // let session_arc  = Arc::new(session.clone());
  let (tx, mut rx) = mpsc::channel::<String>(100);
  let ws_user = WsUser {
    username: " username.clone()".to_string(),
    send_msg: tx,
    gid: "0".to_string(),
  };
  clients.write().await.insert(" username.clone()".to_string(), ws_user);

  let mut stream = stream
    .aggregate_continuations()
    // aggregate continuation frames up to 1MiB
    .max_continuation_size((2_usize).pow(20));

  // start task but don't wait for it
  let mut s_c = session.clone();
  rt::spawn(async move {
    // receive messages from websocket
    while let Some(msg) = stream.next().await {
      match msg {
        Ok(AggregatedMessage::Text(text)) => {
          // echo text message
          s_c.text(text).await.unwrap();
        }

        Ok(AggregatedMessage::Binary(bin)) => {
          // echo binary message
          s_c.binary(bin).await.unwrap();
        }

        Ok(AggregatedMessage::Ping(msg)) => {
          // respond to PING frame with PONG frame
          s_c.pong(&msg).await.unwrap();
        }

        _ => {}
      }
    }
  });

  let mut s_cl = session.clone();

  // let ws_clients_clone = clients.clone();
  tokio::spawn(async move {
    while let Some(message) = rx.recv().await {
      if let Err(_) = s_cl.text(message).await {
        println!(
          "Error sending message to {}: {}",
          " username.clone()".to_string(),
          "message".to_string()
        );
      }
    }
  });

  // respond immediately with response connected to WS session
  Ok(res)
}

pub fn create_ws_msg<T: Serialize>(event_name: String, body: T) -> String {
  let body_json = serde_json::to_string(&body).expect("Failed to serialize body");
  let ws_m = WsMsg {
    ev: event_name,
    body: body_json,
  };
  serde_json::to_string(&ws_m).expect("Failed to serialize ws_message")
}
