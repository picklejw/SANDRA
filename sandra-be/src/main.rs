use actix_cors::Cors;
use actix_files as fs;
use actix_web::{
  http::header,
  middleware::{Logger, NormalizePath, TrailingSlash},
  web,
  web::Data,
  App, HttpServer,
};
use rust_jwt_actix::jwt_middleware;
use utils::db_service::DBService;
use utils::dhcp::start_dhcp;
use utils::rtsp_to_webrtc::WebRTCManager;
use utils::sub_events::CameraList;
use utils::{
  http_service::{build_auth_routes, build_user_routes},
  ws_service,
};
mod utils;
use actix_web_lab::middleware::from_fn;
use awc::Client;
use env_logger;
use std::sync::Arc;
use std::{thread, time};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  start_dhcp();

  env_logger::init();
  let db_arc = Arc::new(DBService::init().await);

  let camera_mngr =
    CameraList::scan_for_devices("admin".to_string(), "password2011".to_string()).await;
  let webrtc_mngr = WebRTCManager::new("8081");
  thread::sleep(time::Duration::from_secs(5));
  for dev in &camera_mngr.devices {
    println!("{}", "dev.device.media_urls.clone().unwrap()");
    println!("{}", dev.device.media_urls.clone().unwrap());
    match webrtc_mngr
      .add_rtsp_url(
        dev.device.name.to_owned(),
        dev.device.media_urls.to_owned().unwrap(),
      )
      .await
    {
      Ok(_) => {
        println!("Added: {}!", dev.device.media_urls.as_ref().unwrap());
      }
      Err(e) => {
        println!("{}", e);
      }
    }
  }
  let webrtc_arc = Arc::new(webrtc_mngr);
  let camera_arc = Arc::new(camera_mngr);

  HttpServer::new(move || {
    // let auth = from_fn(jwt_middleware);

    let cors_opts = Cors::default()
      .allowed_origin_fn(|origin, _req_head| origin.as_bytes().starts_with(b"http://localhost"))
      .allowed_methods(vec!["GET", "POST"])
      .allowed_headers(&[header::AUTHORIZATION, header::ACCEPT])
      .allowed_header(header::CONTENT_TYPE)
      .expose_headers(&[header::CONTENT_DISPOSITION]);

    let auth_scope = build_auth_routes();
    let user_scope = build_user_routes();
    let db_data = Data::new(db_arc.clone());
    let webrtc_clone = Data::new(webrtc_arc.clone());
    let camera_clone = Data::new(camera_arc.clone());

    App::new()
      .wrap(NormalizePath::new(TrailingSlash::Trim))
      .wrap(Logger::default())
      // .wrap(cors_opts)
      .wrap(Cors::permissive())
      // .wrap(
      //   Cors::default()
      //     .allowed_origin("http://localhost:8085") // Replace with your React app URL
      //     .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]) // Specify allowed methods
      //     .allowed_headers(vec!["Content-Type"]) // Specify allowed headers
      //     .max_age(3600)
      // )
      .app_data(db_data)
      .app_data(webrtc_clone)
      .app_data(camera_clone)
      .app_data(Data::new(Client::default()))
      .default_service(fs::Files::new("/", "../sandra-fe/dist").index_file("index.html"))
      // .default_service(fs::Files::new("/", "./TEMP_TEST").index_file("index.html"))
      .service(auth_scope)
      .service(user_scope) // .wrap(auth)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
