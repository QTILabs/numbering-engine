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
    let config = AppConfig::load_from_env();
    let bind = format!("{}:{}", config.bind_ip, config.bind_port);
