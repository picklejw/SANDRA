use actix_files as fs;
use actix_web::{
    middleware::{Logger, NormalizePath, TrailingSlash},
    web::{self, Data},
    App, HttpServer,
};
use utils::http_service::{build_auth_routes, build_user_routes, User};
use utils::sub_events::{subscribe, Source};
mod utils;

use actix_jwt_auth_middleware::use_jwt::UseJWTOnApp;
use actix_jwt_auth_middleware::{Authority, TokenSigner};

use ed25519_compact::KeyPair;
use jwt_compact::alg::Ed25519;

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

    HttpServer::new(|| {
        let KeyPair {
            pk: public_key,
            sk: secret_key,
        } = KeyPair::generate();

        let authority = Authority::<User, Ed25519, _, _>::new()
            .refresh_authorizer(|| async move { Ok(()) })
            .token_signer(Some(
                TokenSigner::new()
                    .signing_key(secret_key.clone())
                    .algorithm(Ed25519)
                    .build()
                    .expect(""),
            ))
            .verifying_key(public_key)
            .build()
            .expect("");

        let auth_scope = build_auth_routes();

        let user_scope = build_user_routes();

        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(Logger::default())
            .service(auth_scope)
            .use_jwt(authority, user_scope)
            // .default_service(fs::Files::new("/", "../sandra-fe/dist"  ).index_file("index.html"))
            .default_service(fs::Files::new("/", "./TEMP_TEST").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
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
