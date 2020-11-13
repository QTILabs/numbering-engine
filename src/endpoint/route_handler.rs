use crate::common::auth;
use crate::common::jwt_laporan::AuthProcessor;
use crate::common::AuthResult;
use actix_web::web::Data as WebData;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use uuid::Uuid;

#[get("/v3/laporan/")]
async fn get_all_laporan(shared_auth: WebData<AuthProcessor>) -> impl Responder {
    // token validation
    let token = "sample token".to_string();
    // let is_token_valid = auth::validate_jwt_token(token);
    let satker = "".to_string();

    match shared_auth.authenticate_user(&token, &satker) {
        AuthResult::Ok => HttpResponse::Ok().body("POST Success! The ID is...").await,
        AuthResult::NotAuthenticated => HttpResponse::Unauthorized().body("You're not authenticated").await,
        AuthResult::TokenInvalid => HttpResponse::Ok().body("Your JWT token is invalid").await,
        AuthResult::TokenExpired(_expiry_date) => {
            HttpResponse::Unauthorized()
                .body("You're not authenticated, token expired!")
                .await
        }
        AuthResult::ForbiddenAccess(current_satker, expected_satker) => {
            HttpResponse::Unauthorized()
                .body(format!(
                    "Invalid Satker, expected {} and you're in {}!",
                    current_satker, expected_satker
                ))
                .await
        }
    }
}

#[get("/v3/laporan/{id}/")]
async fn get_laporan(shared_auth: WebData<AuthProcessor>, id: web::Path<Uuid>) -> impl Responder {
    // token validation
    let token = "sample token".to_string();
    // let is_token_valid = auth::validate_jwt_token(token);
    let satker = "".to_string();

    match shared_auth.authenticate_user(&token, &satker) {
        AuthResult::Ok => HttpResponse::Ok().body("POST Success! The ID is...").await,
        AuthResult::NotAuthenticated => HttpResponse::Unauthorized().body("You're not authenticated").await,
        AuthResult::TokenInvalid => HttpResponse::Ok().body("Your JWT token is invalid").await,
        AuthResult::TokenExpired(_expiry_date) => {
            HttpResponse::Unauthorized()
                .body("You're not authenticated, token expired!")
                .await
        }
        AuthResult::ForbiddenAccess(current_satker, expected_satker) => {
            HttpResponse::Unauthorized()
                .body(format!(
                    "Invalid Satker, expected {} and you're in {}!",
                    current_satker, expected_satker
                ))
                .await
        }
    }
}

#[post("/v3/laporan/")]
async fn post_laporan(shared_auth: WebData<AuthProcessor>) -> impl Responder {
    // token validation
    let token = "sample token".to_string();
    let satker = "".to_string();

    match shared_auth.authenticate_user(&token, &satker) {
        AuthResult::Ok => HttpResponse::Ok().body("POST Success! The ID is...").await,
        AuthResult::NotAuthenticated => HttpResponse::Unauthorized().body("You're not authenticated").await,
        AuthResult::TokenInvalid => HttpResponse::Ok().body("Your JWT token is invalid").await,
        AuthResult::TokenExpired(_expiry_date) => {
            HttpResponse::Unauthorized()
                .body("You're not authenticated, token expired!")
                .await
        }
        AuthResult::ForbiddenAccess(current_satker, expected_satker) => {
            HttpResponse::Unauthorized()
                .body(format!(
                    "Invalid Satker, expected {} and you're in {}!",
                    current_satker, expected_satker
                ))
                .await
        }
    }
}

#[put("/v3/laporan/{id}/")]
async fn put_laporan(shared_auth: WebData<AuthProcessor>, id: web::Path<Uuid>) -> impl Responder {
    // token validation
    let token = "sample token".to_string();
    // let is_token_valid = auth::validate_jwt_token(token);
    let satker = "".to_string();

    match shared_auth.authenticate_user(&token, &satker) {
        AuthResult::Ok => HttpResponse::Ok().body("POST Success! The ID is...").await,
        AuthResult::NotAuthenticated => HttpResponse::Unauthorized().body("You're not authenticated").await,
        AuthResult::TokenInvalid => HttpResponse::Ok().body("Your JWT token is invalid").await,
        AuthResult::TokenExpired(_expiry_date) => {
            HttpResponse::Unauthorized()
                .body("You're not authenticated, token expired!")
                .await
        }
        AuthResult::ForbiddenAccess(current_satker, expected_satker) => {
            HttpResponse::Unauthorized()
                .body(format!(
                    "Invalid Satker, expected {} and you're in {}!",
                    current_satker, expected_satker
                ))
                .await
        }
    }
}

#[delete("/v3/laporan/{id}/")]
async fn delete_laporan(shared_auth: WebData<AuthProcessor>, id: web::Path<Uuid>) -> impl Responder {
    // token validation
    let token = "sample token".to_string();
    // let is_token_valid = auth::validate_jwt_token(token);
    let satker = "".to_string();

    match shared_auth.authenticate_user(&token, &satker) {
        AuthResult::Ok => HttpResponse::Ok().body("POST Success! The ID is...").await,
        AuthResult::NotAuthenticated => HttpResponse::Unauthorized().body("You're not authenticated").await,
        AuthResult::TokenInvalid => HttpResponse::Ok().body("Your JWT token is invalid").await,
        AuthResult::TokenExpired(_expiry_date) => {
            HttpResponse::Unauthorized()
                .body("You're not authenticated, token expired!")
                .await
        }
        AuthResult::ForbiddenAccess(current_satker, expected_satker) => {
            HttpResponse::Unauthorized()
                .body(format!(
                    "Invalid Satker, expected {} and you're in {}!",
                    current_satker, expected_satker
                ))
                .await
        }
    }
}

pub(crate) fn get_shared_auth() -> WebData<AuthProcessor> {
    WebData::new(AuthProcessor::new("127.0.0.1", 6379))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_laporan);
    cfg.service(get_laporan);
    cfg.service(post_laporan);
    cfg.service(put_laporan);
    cfg.service(delete_laporan);
}
