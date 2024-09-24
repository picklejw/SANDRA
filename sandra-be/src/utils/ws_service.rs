use actix_web::{ web, App, HttpServer, HttpResponse, HttpRequest };
use actix::{ Actor, AsyncContext, StreamHandler };
use actix_web_actors::ws;
use futures::StreamExt;
use onvif_utils::onvif::Message;
use retina::{ RtspClient, Frame };

struct VideoSession {
  // This will hold a WebSocket connection
  ws: Option<actix_web_actors::ws::WebsocketContext<VideoSession>>,
}

impl Actor for VideoSession {
  type Context = ws::WebsocketContext<Self>;
}

// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for VideoSession {
//   fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//     match msg {
//       Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
//       Ok(ws::Message::Text(text)) => ctx.text(text),
//       Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
//       _ => (),
//     }
// }

impl actix_web_actors::ws::WsMessageHandler for VideoSession {
  fn handle_message(
    &mut self,
    msg: actix_web_actors::ws::Message,
    ctx: &mut actix_web_actors::ws::WebSocketContext<Self>
  ) {
    match msg {
      actix_web_actors::ws::Message::Ping(ping) => ctx.pong(ping),
      _ => (),
    }
  }
}

pub async fn video_stream(
  req: HttpRequest,
  stream: web::Payload,
  srv: web::Data<VideoSession>
) -> HttpResponse {
  let mut rtsp_client = RtspClient::new("rtsp://your_rtsp_source").await.unwrap();
  let (sink, stream) = ws.split();

  // Spawn a task to handle the RTSP stream and send frames over the WebSocket
  tokio::spawn(async move {
    while let Some(frame) = rtsp_client.next_frame().await.unwrap() {
      // Encode the frame to a suitable format (e.g., base64 or binary data)
      let data = encode_frame_to_websocket(&frame);
      // Send the encoded frame over WebSocket
      sink.send(actix_web_actors::ws::Message::Binary(data)).await.unwrap();
    }
  });

  HttpResponse::Ok().finish()
}

fn encode_frame_to_websocket(frame: &Frame) -> Vec<u8> {
  // Convert the frame to a binary format suitable for WebSocket
  // This is a placeholder implementation
  frame.data.clone()
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//   HttpServer::new(|| {
//     App::new()
//       .app_data(web::Data::new(VideoSession { ws: None }))
//       .route("/ws", web::get().to(video_stream))
//   })
//     .bind("127.0.0.1:8080")?
//     .run().await
// }
