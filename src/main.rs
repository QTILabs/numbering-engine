#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod api;
mod common {
    pub mod auth;
    pub mod db;
} // make it accesible to parent folder

use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind = "127.0.0.1:8080";

    HttpServer::new(move || App::new().wrap(middleware::Logger::default()).configure(api::laporan::init))
        .bind(&bind)?
        .run()
        .await
}
