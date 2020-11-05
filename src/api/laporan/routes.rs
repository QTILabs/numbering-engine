use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use crate::common::auth;

#[get("/v3/laporan/")]
async fn get_all_laporan() -> impl Responder {

    // token validation
    let token = "sample token".to_string();
    let is_token_valid = auth::validate_jwt_token(token);
    if is_token_valid == true {
        HttpResponse::Ok().body("get all v3 laporan")
    } else {
        HttpResponse::Unauthorized().body("unauthorized")
    }
}

#[get("/v3/laporan/{id}/")]
async fn get_laporan(id: web::Path<i32>) -> impl Responder {

    // token validation
    let token = "sample token".to_string();
    let is_token_valid = auth::validate_jwt_token(token);
    if is_token_valid == true {

        let id_string = id.to_string();
        let mut message = "get v3 laporan id: ".to_string();
        message.push_str(&id_string);
        HttpResponse::Ok().body(message)

    } else {
        HttpResponse::Unauthorized().body("unauthorized")
    }
    
}

#[post("/v3/laporan/")]
async fn post_laporan() -> impl Responder {

    // token validation
    let token = "sample token".to_string();
    let is_token_valid = auth::validate_jwt_token(token);
    if is_token_valid == true {
        HttpResponse::Ok().body("post v3 laporan")
    } else {
        HttpResponse::Unauthorized().body("unauthorized")
    }
}

#[put("/v3/laporan/{id}/")]
async fn put_laporan(id: web::Path<i32>) -> impl Responder {
    
    // token validation
    let token = "sample token".to_string();
    let is_token_valid = auth::validate_jwt_token(token);
    if is_token_valid == true {

        let id_string = id.to_string();
        let mut message = "put v3 laporan id: ".to_string();
        message.push_str(&id_string);
        HttpResponse::Ok().body(message)

    } else {
        HttpResponse::Unauthorized().body("unauthorized")
    }
}

#[delete("/v3/laporan/{id}/")]
async fn delete_laporan(id: web::Path<i32>) -> impl Responder {
    
    // token validation
    let token = "sample token".to_string();
    let is_token_valid = auth::validate_jwt_token(token);
    if is_token_valid == true {

        let id_string = id.to_string();
        let mut message = "delete v3 laporan id: ".to_string();
        message.push_str(&id_string);
        HttpResponse::Ok().body(message)

    } else {
        HttpResponse::Unauthorized().body("unauthorized")
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_laporan);
    cfg.service(get_laporan);
    cfg.service(post_laporan);
    cfg.service(put_laporan);
    cfg.service(delete_laporan);
}
