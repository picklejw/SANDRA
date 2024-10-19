use actix_cors::Cors;
use actix_files as fs;
use actix_web::{
  http::header,
  middleware::from_fn,
  middleware::{Logger, NormalizePath, TrailingSlash},
  web::Data,
  App, HttpServer,
};
use rust_jwt_actix::jwt_middleware;
use utils::db_service::DBService;
use utils::dhcp::start_dhcp;
use utils::models::WsUser;
use utils::rtsp_to_webrtc::WebRTCManager;
use utils::sub_events::CameraList;
use utils::{
  http_service::{build_auth_routes, build_user_routes},
  ws_service,
};
mod utils;
use dotenv::dotenv;
use env_logger;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  start_dhcp();

  env_logger::init();
  let ws_clients: Arc<RwLock<HashMap<String, WsUser>>> = Arc::new(RwLock::new(HashMap::new()));

  let db_arc = Arc::new(DBService::init().await);

  let camera_mngr = CameraList::scan_for_devices(
    std::env::var("DEFAULT_USERNAME").unwrap_or("admin".to_string()),
    std::env::var("DEFAULT_PASSWORD").unwrap_or("password".to_string()),
  )
  .await;
  let webrtc_mngr = WebRTCManager::new("8081");
  // thread::sleep(time::Duration::from_secs(5));
  for dev in &camera_mngr.devices {
    match webrtc_mngr.add_rtsp_url(dev.device.name.to_owned(), dev.device.media_urls.to_owned().unwrap()).await {
      Ok(_) => {
        println!("Added: {}!", dev.device.media_urls.as_ref().unwrap());
        let ws_clients_clone = ws_clients.clone();
        dev
          .sub_events(move |ev: utils::models::OnvifEvMsg| {
            let ws_clients_clone = ws_clients_clone.clone();
            Box::pin(async move {
              let users_lock = ws_clients_clone.read().await;
              for (key, user) in users_lock.iter() {
                println!("Found WS User: {:?}", key);
                let msg = ws_service::create_ws_msg("Cam_Motion".to_string(), &ev);
                if let Err(_) = user.send_msg.send(msg).await {
                  println!("Receiver dropped for user: {}", key);
                  return;
                }
              }
            })
          })
          .await;
      }
      Err(e) => {
        println!("{}", e);
      }
    }
  }
  let webrtc_arc = Arc::new(webrtc_mngr);
  let camera_arc = Arc::new(camera_mngr);

  HttpServer::new(move || {
    let auth = from_fn(jwt_middleware);

    let cors_opts = Cors::default()
      .allowed_origin_fn(|origin, _req_head| origin.as_bytes().starts_with(b"http://localhost"))
      .allowed_methods(vec!["GET", "POST"])
      .allowed_headers(&[header::AUTHORIZATION, header::ACCEPT])
      .allowed_header(header::CONTENT_TYPE)
      .expose_headers(&[header::CONTENT_DISPOSITION]);

    let auth_scope = build_auth_routes();
    let user_scope = build_user_routes();
    let db_data = Data::new(db_arc.clone());
    let ws_clients_c = ws_clients.clone();
    let webrtc_clone = Data::new(webrtc_arc.clone());
    let camera_clone = Data::new(camera_arc.clone());

    App::new()
      .wrap(NormalizePath::new(TrailingSlash::Trim))
      .wrap(Logger::default())
      .wrap(cors_opts)
      .wrap(Cors::permissive())
      .app_data(db_data)
      .app_data(webrtc_clone)
      .app_data(ws_clients_c)
      .app_data(camera_clone)
      .app_data(Data::new(ws_clients.clone()))
      .default_service(
        fs::Files::new(
          "/",
          std::env::var("DEFAULT_STATIC_WEB_PATH").unwrap_or("../sandra-fe/dist".to_string()),
        )
        .index_file("index.html"),
      )
      .service(auth_scope)
      .service(user_scope)
      .wrap(auth)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
