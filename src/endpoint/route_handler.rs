use crate::common::auth;
use crate::common::jwt_auth;
use crate::common::AuthResultJwt;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use jwt_auth::decode_jwt;
use uuid::Uuid;

#[get("/v3/laporan/")]
async fn get_all_laporan(req: HttpRequest) -> impl Responder {
    // get jwt token from header Authorization
    let authorize_token = match auth::get_header_value(&req, "Authorization") {
        Some(token) => Some(token.to_string()),
        None => None,
    };

    // token validation
    let verified_token = decode_jwt(authorize_token);
    match verified_token {
        AuthResultJwt::Ok(claims) => {
            println!("{:?}", claims);
            return HttpResponse::Ok().body("get all v3 laporan");
        }
        AuthResultJwt::NotAuthenticated => return HttpResponse::Unauthorized().body("Not Authenticated"),
        AuthResultJwt::TokenInvalid => return HttpResponse::Unauthorized().body("Token Invalid"),
        AuthResultJwt::TokenExpired => return HttpResponse::Unauthorized().body("Token Expired"),
        _ => return HttpResponse::InternalServerError().body("Internal server error"),
    }
}

#[get("/v3/laporan/{id}/")]
async fn get_laporan(req: HttpRequest, id: web::Path<Uuid>) -> impl Responder {
    // get jwt token from header Authorization
    let authorize_token = match auth::get_header_value(&req, "Authorization") {
        Some(token) => Some(token.to_string()),
        None => None,
    };

    // token validation
    let verified_token = decode_jwt(authorize_token);
    match verified_token {
        AuthResultJwt::Ok(claims) => {
            println!("{:?}", claims);
            return HttpResponse::Ok().body(format!("get v3 laporan id {}", id));
        }
        AuthResultJwt::NotAuthenticated => return HttpResponse::Unauthorized().body("Not Authenticated"),
        AuthResultJwt::TokenInvalid => return HttpResponse::Unauthorized().body("Token Invalid"),
        AuthResultJwt::TokenExpired => return HttpResponse::Unauthorized().body("Token Expired"),
        _ => return HttpResponse::InternalServerError().body("Internal server error"),
    }
}

#[post("/v3/laporan/")]
async fn post_laporan(req: HttpRequest) -> impl Responder {
    // get jwt token from header Authorization
    let authorize_token = match auth::get_header_value(&req, "Authorization") {
        Some(token) => Some(token.to_string()),
        None => None,
    };

    // token validation
    let verified_token = decode_jwt(authorize_token);
    match verified_token {
        AuthResultJwt::Ok(claims) => {
            println!("{:?}", claims);
            return HttpResponse::Ok().body("post v3 laporan");
        }
        AuthResultJwt::NotAuthenticated => return HttpResponse::Unauthorized().body("Not Authenticated"),
        AuthResultJwt::TokenInvalid => return HttpResponse::Unauthorized().body("Token Invalid"),
        AuthResultJwt::TokenExpired => return HttpResponse::Unauthorized().body("Token Expired"),
        _ => return HttpResponse::InternalServerError().body("Internal server error"),
    }
}

#[put("/v3/laporan/{id}/")]
async fn put_laporan(req: HttpRequest, id: web::Path<Uuid>) -> impl Responder {
    // get jwt token from header Authorization
    let authorize_token = match auth::get_header_value(&req, "Authorization") {
        Some(token) => Some(token.to_string()),
        None => None,
    };

    // token validation
    let verified_token = decode_jwt(authorize_token);
    match verified_token {
        AuthResultJwt::Ok(claims) => {
            println!("{:?}", claims);
            return HttpResponse::Ok().body(format!("put v3 laporan id {}", id));
        }
        AuthResultJwt::NotAuthenticated => return HttpResponse::Unauthorized().body("Not Authenticated"),
        AuthResultJwt::TokenInvalid => return HttpResponse::Unauthorized().body("Token Invalid"),
        AuthResultJwt::TokenExpired => return HttpResponse::Unauthorized().body("Token Expired"),
        _ => return HttpResponse::InternalServerError().body("Internal server error"),
    }
}

#[delete("/v3/laporan/{id}/")]
async fn delete_laporan(req: HttpRequest, id: web::Path<Uuid>) -> impl Responder {
    // get jwt token from header Authorization
    let authorize_token = match auth::get_header_value(&req, "Authorization") {
        Some(token) => Some(token.to_string()),
        None => None,
    };

    // token validation
    let verified_token = decode_jwt(authorize_token);
    match verified_token {
        AuthResultJwt::Ok(claims) => {
            println!("{:?}", claims);
            return HttpResponse::Ok().body(format!("delete v3 laporan id {}", id));
        }
        AuthResultJwt::NotAuthenticated => return HttpResponse::Unauthorized().body("Not Authenticated"),
        AuthResultJwt::TokenInvalid => return HttpResponse::Unauthorized().body("Token Invalid"),
        AuthResultJwt::TokenExpired => return HttpResponse::Unauthorized().body("Token Expired"),
        _ => return HttpResponse::InternalServerError().body("Internal server error"),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_laporan);
    cfg.service(get_laporan);
    cfg.service(post_laporan);
    cfg.service(put_laporan);
    cfg.service(delete_laporan);
}
