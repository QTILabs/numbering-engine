#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub(crate) mod common;
pub(crate) mod db;
pub(crate) mod endpoint;

use crate::endpoint::route_handler::init as init_handler;
use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind = "0.0.0.0:8080";

    HttpServer::new(move || App::new().wrap(middleware::Logger::default()).configure(init_handler))
        .bind(&bind)?
        .run()
        .await
}
