use actix_web::{ cookie::Cookie, web, HttpResponse, Responder, Scope };
use mongodb::bson::oid::ObjectId;

use jwt_compact::alg::Ed25519;
// use actix::{Actor, Addr, Context, Handler};
use crate::utils::db_service::DBService;
use crate::utils::models::{ AccessLevel, User };
use actix_jwt_auth_middleware::{ AuthResult, FromRequest, TokenSigner };
use serde::{ Deserialize, Serialize };
use std::fmt;

use mongodb::{ bson, bson::doc, Client, Collection };

pub fn build_user_routes() -> Scope {
  web::scope("/api/user").route("/echo", web::get().to(echo))
}
pub fn build_auth_routes() -> Scope {
  web
    ::scope("/api/auth")
    .route("/signup", web::post().to(signup))
    .route("/login", web::post().to(login))
}

fn format_cookie(cookie: AuthResult<Cookie<'static>>) -> Cookie<'_> {
  let mut cookie_inst = cookie.unwrap();
  cookie_inst.set_http_only(true);
  cookie_inst.set_path("/");
  cookie_inst
}

#[derive(Serialize)]
struct AuthReply {
  error: Option<String>,
  success: bool,
}
// Route Handlers
async fn login(
  cookie_signer: web::Data<TokenSigner<User, Ed25519>>,
  req_body: web::Json<User>,
  db: web::Data<DBService>
) -> AuthResult<HttpResponse> {
  let user = User { // FIX THIS!!
    username: req_body.username.to_owned(),
    password: req_body.password.to_owned(),
    gid: req_body.gid,
    access_level: None,
  };

  // let user: User = db
  //   .check_login(
  //     req_body.username.unwrap().to_string(),
  //     req_body.password.unwrap().to_string()
  //   ).await
  //   .expect("Someone made api login call and got error on db query");

  match
    db.check_login(
      req_body.username.unwrap().to_string(),
      req_body.password.unwrap().to_string()
    ).await
  {
    Some(user) => {
      println!("Found user with id {}: {}", user.id, user.name);
      // Process user data further if needed
    }
    None => {
      println!("User with id {} not found", user_id_to_find);
      // Handle the case where no user is found
    }
  }

  let a_cookie = format_cookie(cookie_signer.create_access_cookie(&user));
  let r_cookie = format_cookie(cookie_signer.create_refresh_cookie(&user));

  Ok(HttpResponse::Ok().cookie(a_cookie).cookie(r_cookie).body("You are now logged in"));

  let a_cookie = format_cookie(cookie_signer.create_access_cookie(&user));
  let r_cookie = format_cookie(cookie_signer.create_refresh_cookie(&user));

  Ok(HttpResponse::Ok().cookie(a_cookie).cookie(r_cookie).body("You are now logged in"))
}

async fn signup(
  cookie_signer: web::Data<TokenSigner<User, Ed25519>>,
  req_body: web::Json<User>,
  db: web::Data<DBService>
) -> AuthResult<HttpResponse> {
  let user_collection = db.__get_all_users().await;
  println!("{:?}", user_collection.unwrap());

  let user = User { // FIX THIS!!
    username: req_body.username.to_owned(),
    password: req_body.password.to_owned(),
    gid: req_body.gid,
    access_level: None,
  };
  println!("{}", "user");
  println!("{:?}", user);
  db.create_new_user(
    req_body.username.to_owned(),
    req_body.password.to_owned(),
    req_body.gid
  ).await.expect("Creating user failed, seemingly unhandled");

  let a_cookie = format_cookie(cookie_signer.create_access_cookie(&user));
  let r_cookie = format_cookie(cookie_signer.create_refresh_cookie(&user));

  Ok(
    HttpResponse::Ok().cookie(a_cookie).cookie(r_cookie).json(AuthReply {
      error: None,
      success: true,
    })
  )
}

// /api/user/echo
async fn echo(user_claims: User) -> impl Responder {
  format!(
    "Hello user with, i see you in group {:?}! , and are a {}",
    user_claims.gid,
    user_claims.access_level.unwrap().to_string()
  )
}
