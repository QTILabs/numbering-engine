#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub(crate) mod common;
pub(crate) mod config;
pub(crate) mod db;
pub(crate) mod endpoint;

use crate::config::AppConfig;
use crate::endpoint::route_handler::init as init_handler;
use actix_web::{middleware, App, HttpServer};
use crate::common::jwt_laporan;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
<<<<<<< HEAD
    let config = AppConfig::load_from_env();
    let bind = format!("{}:{}", config.bind_ip, config.bind_port);
=======
    let bind = "0.0.0.0:8080";
    let astruct = api::laporan::Claims {
        sub : "asdf".to_string(),
        company : "asdf".to_string(),
        exp : 10000000100
    };  
    let token =api::laporan::encode_into_jwt(astruct).unwrap();
    println!("{}",token);
    let decoded = api::laporan::decode_from_jwt(token).unwrap();
    println!("{:?}",decoded);
    HttpServer::new(move || App::new().wrap(middleware::Logger::default()).configure(api::laporan::init))
        .bind(&bind)?
        .run()
        .await
}
>>>>>>> test gpg
