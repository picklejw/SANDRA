use actix_web::{cookie::Cookie, web, HttpResponse, Responder, Scope};
use mongodb::bson::oid::ObjectId;

use jwt_compact::alg::Ed25519;
// use actix::{Actor, Addr, Context, Handler};
use actix_jwt_auth_middleware::{AuthResult, FromRequest, TokenSigner};
use serde::{Deserialize, Serialize};
use std::fmt;
use crate::utils::db_service::DBService;
use crate::utils::models::{AccessLevel,User};

use mongodb::{ 
  bson::doc,
  bson,
  Client,
  Collection
};



pub fn build_user_routes() -> Scope {
    web::scope("/api/user").route("/echo", web::get().to(echo))
}
pub fn build_auth_routes() -> Scope {
    web::scope("/api/auth")
        .route("/signup", web::post().to(signup))
        .route("/login", web::get().to(login))
}

fn format_cookie(cookie: AuthResult<Cookie<'static>>) -> Cookie<'_> {
    let mut cookie_inst = cookie.unwrap();
    cookie_inst.set_http_only(true);
    cookie_inst.set_path("/");
    cookie_inst
}

// Route Handlers
async fn login(cookie_signer: web::Data<TokenSigner<User, Ed25519>>) -> AuthResult<HttpResponse> {
    let user = User { // FIX THIS!!
        username: "".to_string(),
        password: "".to_string(),
        id: ObjectId::new(),
        gid: ObjectId::new(),
        access_level: AccessLevel::Admin,
    };

    let a_cookie = format_cookie(cookie_signer.create_access_cookie(&user));
    let r_cookie = format_cookie(cookie_signer.create_refresh_cookie(&user));

    Ok(HttpResponse::Ok()
        .cookie(a_cookie)
        .cookie(r_cookie)
        .body("You are now logged in"))
}

#[derive(Deserialize, Clone)]
struct SignupReq {
    username: String,
    password: String
}

async fn signup(cookie_signer: web::Data<TokenSigner<User, Ed25519>>, req_body: web::Json<SignupReq>, db: web::Data<DBService>,) -> AuthResult<HttpResponse> {
  let user_collection = db.__get_all_users().await;
  // user_collection.
  println!("{:?}", user_collection.unwrap());
  // let bson = bson::to_document(&User {
  //   id: ObjectId::new(),
  //   gid: ObjectId::new(),
  //   access_level: AccessLevel::Admin
  // }).unwrap();

  // user_collection.insert_one(bson);
  println!("{}", req_body.username);
  println!("{}", req_body.password);
  db.create_new_user(req_body.username.clone(), req_body.password.clone(), None).await.expect("Creating user failed, seemingly unhandled");
  // get_mongo

    // get_db_collection("users").insert_one(bson::doc! {"name": name}, None)
    // let user = User { id: 1, gid: 1, access_level: AccessLevel::Admin };

    // get_db_collection("users").insert_one(bson::doc! {"name": name}, None)

    //   let a_cookie = format_cookie(cookie_signer.create_access_cookie(&user));
    //   let r_cookie = format_cookie(cookie_signer.create_refresh_cookie(&user));

      Ok(HttpResponse::Ok()
          .body("req_body.username"))
}

// /api/user/echo
async fn echo(user_claims: User) -> impl Responder {
    format!(
        "Hello user with id: {}, i see you in group {:?}! , and are a {}",
        user_claims.id,
        user_claims.gid,
        user_claims.access_level.to_string()
    )
}
