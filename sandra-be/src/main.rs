use actix_web::{
  middleware::{ Logger, NormalizePath, TrailingSlash },
  web::Data,
  http::header,
  App,
  HttpServer,
};
use actix_files as fs;
use actix_cors::Cors;
use utils::http_service::{ build_auth_routes, build_user_routes };
use utils::db_service::DBService;
use utils::auth::jwt_middleware;
use utils::sub_events;
mod utils;
use actix_web_lab::middleware::from_fn;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // let feed = Source {
  //     src_ip: "admin",
  //     username: "admin",
  //     password: "password2011",
  //     ..Default::default()
  // };
  // fn read_events (ev: &NotificationMessageHolderType) {
  //   println!("ev!!");
  //   println!("{}", ev.topic.inner_text);
  //   let ev_attib = ev.message.msg.data.simple_item.iter().next().unwrap();
  //   println!("{:?}", ev_attib.name);
  //   println!("{:?}", ev_attib.value);
  // }
  // sub_events::subscribe(feed, read_events).await;

  let db = DBService::init().await;

  HttpServer::new(move || {
    let auth = from_fn(jwt_middleware);

    let cors_opts = Cors::default()
      .allowed_origin_fn(|origin, _req_head| { origin.as_bytes().starts_with(b"http://localhost") })
      .allowed_methods(vec!["GET", "POST"])
      .allowed_headers(&[header::AUTHORIZATION, header::ACCEPT])
      .allowed_header(header::CONTENT_TYPE)
      .expose_headers(&[header::CONTENT_DISPOSITION]);

    let auth_scope = build_auth_routes();
    let user_scope = build_user_routes();
    let db_data = Data::new(db.clone());

    App::new()
      .wrap(NormalizePath::new(TrailingSlash::Trim))
      .wrap(Logger::default())
      .wrap(cors_opts)
      .app_data(db_data)
      .default_service(fs::Files::new("/", "../sandra-fe/dist").index_file("index.html"))
      .service(auth_scope)
      .service(user_scope.wrap(auth)) // remove me
    // .default_service(fs::Files::new("/", "./TEMP_TEST").index_file("index.html"))
  })
    .bind(("127.0.0.1", 8080))?
    .run().await
}

// have auto discovery for cameras on network.
// provide default user/pass for these cameras
// ability to add camera

// HTTP: user auth, db mongoDB,
// user: {cameras: [], }
// cameras: {src_ip, username, password, onvif_port, rtsp_url, description, name, }

//POST  "/api/auth/signup"
//POST  "/api/auth/login"
// GET "/api/auth/profile",

// GET /api/user/camera/live_feeds?cids=[str]&lfids=[str] ( or all )
// [{id: str, url: str, name: str, desc:str}]

// GET /api/user/alerts?since={DATE}
// {isRead, bool, title: str, desc: str, life_feed_id: str}

// GET /api/user/alerts/mark_read?id=str
// -> "OK"
