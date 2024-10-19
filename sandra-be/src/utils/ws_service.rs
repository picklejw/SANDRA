use crate::utils::models::{WsMsg, WsUser};
use actix_web::{rt, web, HttpRequest, HttpResponse};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

pub async fn ws_index(
  req: HttpRequest,
  stream: web::Payload,
  clients: web::Data<Arc<RwLock<HashMap<String, WsUser>>>>,
) -> Result<HttpResponse, actix_web::Error> {
  let (res, session, stream) = actix_ws::handle(&req, stream)?;

  let (tx, mut rx) = mpsc::channel::<String>(100);
  let ws_user = WsUser {
    username: " username.clone()".to_string(),
    send_msg: tx,
    gid: "0".to_string(),
  };
  clients.write().await.insert(" username.clone()".to_string(), ws_user);

  let mut stream = stream.aggregate_continuations().max_continuation_size((2_usize).pow(20));

  let mut s_c = session.clone();
  rt::spawn(async move {
    while let Some(msg) = stream.next().await {
      match msg {
        Ok(AggregatedMessage::Text(text)) => {
          s_c.text(text).await.unwrap();
        }

        Ok(AggregatedMessage::Binary(bin)) => {
          s_c.binary(bin).await.unwrap();
        }

        Ok(AggregatedMessage::Ping(msg)) => {
          s_c.pong(&msg).await.unwrap();
        }

        _ => {}
      }
    }
  });

  let mut s_cl = session.clone();

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
