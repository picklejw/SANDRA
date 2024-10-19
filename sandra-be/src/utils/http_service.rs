use super::models::Camera;
use crate::utils::db_service::DBService;
use crate::utils::models::{SPDIncomming, User};
use crate::utils::rtsp_to_webrtc::WebRTCManager;
use crate::utils::sub_events::CameraList;
use crate::utils::ws_service::ws_index;
use actix_web::{cookie::CookieBuilder, web, Error, HttpResponse, Responder, Scope};
use mongodb::bson::doc;
use rust_jwt_actix::AUTH_STATE;
use serde::Serialize;
use std::sync::Arc;

pub fn build_auth_routes() -> Scope {
  web::scope("/api/auth").route("/signup", web::post().to(signup)).route("/login", web::post().to(login))
}

pub fn build_user_routes() -> Scope {
  web::scope("/api/user")
    .route("/camera_webrtc", web::get().to(upstream_get_codec))
    .route("/camera_webrtc", web::post().to(upstream_send_spd))
    .route("/get_cameras", web::get().to(get_camera_feeds))
    .route("/ws", web::get().to(ws_index))
}

#[derive(Serialize)]
struct AuthReply {
  error: Option<String>,
  success: bool,
  user: User,
}
// Route Handlers
async fn login(req_body: web::Json<User>, db: web::Data<Arc<DBService>>) -> impl Responder {
  let user = User {
    username: req_body.username.to_owned(),
    password: req_body.password.to_owned(),
    gid: req_body.gid,
    group_data: None,
    access_level: None,
  };

  if let Some(user) = db.check_login(user.username, user.password).await {
    let n_tokens = AUTH_STATE
      .write()
      .expect("Unable to write to AUTH_STATE")
      .renew_tokens_by_id(&user.username.clone().expect("Could not find username on /login"));

    let at_cookie = CookieBuilder::new(
      "access_token",
      n_tokens.access_token.expect("Could not get new access token to set on login"),
    )
    .path("/")
    .http_only(true)
    .finish();
    let rt_cookie = CookieBuilder::new(
      "refresh_token",
      n_tokens.refresh_token.expect("Could not get new refresh token to set on login"),
    )
    .path("/")
    .http_only(true)
    .finish();

    HttpResponse::Ok().cookie(at_cookie).cookie(rt_cookie).json(AuthReply {
      error: None,
      success: true,
      user,
    })
  } else {
    HttpResponse::Unauthorized().body("User not found.")
  }
}

async fn signup(req_body: web::Json<User>, db: web::Data<Arc<DBService>>) -> impl Responder {
  let _ = db.__get_all_users().await;
  let user = User {
    username: req_body.username.to_owned(),
    password: req_body.password.to_owned(),
    gid: req_body.gid,
    group_data: None,
    access_level: None,
  };
  db.create_new_user(req_body.username.to_owned(), req_body.password.to_owned(), req_body.gid)
    .await
    .expect("Creating user failed, seemingly unhandled");

  let n_tokens = AUTH_STATE
    .write()
    .expect("Unable to write to AUTH_STATE")
    .renew_tokens_by_id(&user.username.clone().expect("Could not find username on /signupo"));

  let at_cookie = CookieBuilder::new(
    "access_token",
    n_tokens.access_token.expect("Could not get new access token to set on login"),
  )
  .path("/")
  .http_only(true)
  .finish();
  let rt_cookie = CookieBuilder::new(
    "refresh_token",
    n_tokens.refresh_token.expect("Could not get new refresh token to set on login"),
  )
  .path("/")
  .http_only(true)
  .finish();

  HttpResponse::Ok().cookie(at_cookie).cookie(rt_cookie).json(AuthReply {
    error: None,
    success: true,
    user,
  })
}

async fn upstream_get_codec(
  webrtc_mngr: web::Data<Arc<WebRTCManager>>,
  query_params: web::Query<SPDIncomming>,
) -> Result<impl Responder, Error> {
  let suuid = match query_params.suuid.clone() {
    Some(suuid) => suuid,
    None => {
      return Err(actix_web::error::ErrorBadRequest("Missing SUUID"));
    }
  };
  if let Some(ctrl) = webrtc_mngr.get_rtsp_controller(&suuid) {
    match ctrl.get_codec_info(suuid).await {
      Ok(codec_info) => Ok(codec_info),
      Err(err) => Err(actix_web::error::ErrorBadRequest(err)),
    }
  } else {
    println!("Controller not found.");
    Err(actix_web::error::ErrorNotFound("Controller not found"))
  }
}

async fn upstream_send_spd(
  req_body: web::Json<SPDIncomming>,
  webrtc_mngr: web::Data<Arc<WebRTCManager>>,
) -> Result<impl Responder, Error> {
  let suuid = match req_body.suuid.clone() {
    Some(suuid) => suuid,
    None => {
      return Err(actix_web::error::ErrorBadRequest("Missing SUUID"));
    }
  };
  if let Some(ctrl) = webrtc_mngr.get_rtsp_controller(&suuid) {
    match ctrl.get_remote_spd(suuid, req_body.into_inner()).await {
      Ok(codec_info) => Ok(HttpResponse::Ok().body(codec_info)),
      Err(err) => Err(actix_web::error::ErrorBadRequest(err)),
    }
  } else {
    println!("Controller not found.");
    Err(actix_web::error::ErrorNotFound("Controller not found"))
  }
}

async fn get_camera_feeds(camera_dev_mngr: web::Data<Arc<CameraList>>) -> Result<impl Responder, Error> {
  let all_cameras = camera_dev_mngr.get_all_devices();
  let mut rtsp_urls: Vec<Camera> = Vec::new();
  for camera in all_cameras.into_iter() {
    rtsp_urls.push(Camera {
      name: camera.device.name.clone(),
      rtsp_url: camera.device.media_urls.clone().expect("Could not get rtsp string"),
    });
  }
  Ok(HttpResponse::Ok().json(rtsp_urls))
}
