use actix_web::{ cookie::{ Cookie, CookieBuilder }, web, HttpResponse, Responder, Scope };

use mongodb::bson::oid::ObjectId;
use crate::utils::auth::AUTH_STATE;
use crate::utils::db_service::DBService;
use crate::utils::models::{ AccessLevel, User };
use serde::{ Deserialize, Serialize };
use std::fmt;

use mongodb::{ bson, bson::doc, Client, Collection };

use super::models::Camera;

pub fn build_user_routes() -> Scope {
  web
    ::scope("/api/user")
    .route("/echo", web::get().to(echo))
    .route("/add_camera", web::post().to(add_camera))
}
pub fn build_auth_routes() -> Scope {
  web
    ::scope("/api/auth")
    .route("/signup", web::post().to(signup))
    .route("/login", web::post().to(login))
}

#[derive(Serialize)]
struct AuthReply {
  error: Option<String>,
  success: bool,
  user: User,
}
// Route Handlers
async fn login(req_body: web::Json<User>, db: web::Data<DBService>) -> impl Responder {
  let user = User {
    username: req_body.username.to_owned(),
    password: req_body.password.to_owned(),
    gid: req_body.gid,
    group_data: None,
    access_level: None,
  };

  if let Some(user) = db.check_login(user.username, user.password).await {
    let n_tokens = AUTH_STATE.write()
      .expect("Unable to write to AUTH_STATE")
      .renew_tokens_by_id(&user.username.clone().expect("Could not find username on /login"));

    let at_cookie = CookieBuilder::new(
      "access_token",
      n_tokens.access_token.expect("Could not get new access token to set on login")
    )
      .path("/")
      .http_only(true)
      .finish();
    let rt_cookie = CookieBuilder::new(
      "refresh_token",
      n_tokens.refresh_token.expect("Could not get new refresh token to set on login")
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

async fn signup(req_body: web::Json<User>, db: web::Data<DBService>) -> impl Responder {
  let user_collection = db.__get_all_users().await;
  let user = User { // FIX THIS!!
    username: req_body.username.to_owned(),
    password: req_body.password.to_owned(),
    gid: req_body.gid,
    group_data: None,
    access_level: None,
  };
  db.create_new_user(
    req_body.username.to_owned(),
    req_body.password.to_owned(),
    req_body.gid
  ).await.expect("Creating user failed, seemingly unhandled");

  let n_tokens = AUTH_STATE.write()
    .expect("Unable to write to AUTH_STATE")
    .renew_tokens_by_id(&user.username.clone().expect("Could not find username on /signupo"));

  let at_cookie = CookieBuilder::new(
    "access_token",
    n_tokens.access_token.expect("Could not get new access token to set on login")
  )
    .path("/")
    .http_only(true)
    .finish();
  let rt_cookie = CookieBuilder::new(
    "refresh_token",
    n_tokens.refresh_token.expect("Could not get new refresh token to set on login")
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

async fn add_camera(
  // user: User,
  // req_body: web::Json<Camera>,
  // db: web::Data<DBService>
) -> impl Responder {
  format!("Hello user with, i see you in group! , and are a ")
  // Ok(HttpResponse::Ok().json(db.add_camera_by_gid(user.gid, Some(req_body.into_inner())).await))
}

// /api/user/echo
async fn echo() -> impl Responder {
  format!("Hello user with, i see you in group! , and are a ")
}
