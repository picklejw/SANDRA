use actix_web::{cookie::Cookie, web, HttpResponse, Responder, Scope};

use jwt_compact::alg::Ed25519;
// use actix::{Actor, Addr, Context, Handler};
use actix_jwt_auth_middleware::{AuthResult, FromRequest, TokenSigner};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, FromRequest)]

pub struct User {
    pub id: u32,
}

pub fn build_user_routes() -> Scope {
    web::scope("/api/user").route("/echo", web::get().to(echo))
}
pub fn build_auth_routes() -> Scope {
    web::scope("/api/auth").route("/login", web::get().to(login))
}

fn format_cookie(cookie: AuthResult<Cookie<'static>>) -> Cookie<'_> {
    let mut cookie_inst = cookie.unwrap();
    cookie_inst.set_http_only(true);
    cookie_inst.set_path("/");
    cookie_inst
}

// Route Handlers
async fn login(cookie_signer: web::Data<TokenSigner<User, Ed25519>>) -> AuthResult<HttpResponse> {
    let user = User { id: 1 };
    let a_cookie = format_cookie(cookie_signer.create_access_cookie(&user));
    let r_cookie = format_cookie(cookie_signer.create_refresh_cookie(&user));

    Ok(HttpResponse::Ok()
        .cookie(a_cookie)
        .cookie(r_cookie)
        .body("You are now logged in"))
}

async fn echo() -> impl Responder {
    HttpResponse::Ok().body("Hello echo!")
}
