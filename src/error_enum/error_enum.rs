use chrono::{DateTime, FixedOffset, Utc};
use serde::{Serialize, Deserialize};
use redis::{Commands,Connection,Client,cluster::ClusterClient};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) enum AuthResult {
    Ok,
    NotAuthenticated,
    TokenInvalid,
    TokenExpired(DateTime<FixedOffset>),
    ForbiddenAccess(String, String),
}

pub(crate) fn authenticate_user(user_jwt: &str, satker: &str) -> AuthResult {
    let current_time: DateTime<FixedOffset> = Utc::now().into();
    //TODO: get key from redis DB 9
    //TODO: get jwt from redis DB 10
    //TODO: check validity (token exist? token value equal? token expired? satker correct?)
    let token = "the true only token";
    let alldata = token;
    let jwtdate:DateTime<FixedOffset> = Utc::now().into();
    let jwtsatker = token;
    let intended_satker=token;

    if is_token_value_correct(&alldata){
        return AuthResult::TokenInvalid;
    };
    if is_token_expired(&jwtdate) {
        let tanggal_expired:DateTime<FixedOffset> = jwtdate.clone();
        return AuthResult::TokenExpired(tanggal_expired);
    };
    if is_token_satker_correct(&jwtsatker,&intended_satker){
        let requested_satker = jwtsatker.clone().into();
        let correct_satker = intended_satker.clone().into();
        return AuthResult::ForbiddenAccess(requested_satker,correct_satker);
    };
    AuthResult::Ok
}

pub(crate) struct AuthProcessor {
    redis_key: redis::cluster::ClusterConnection,
    redis_jwt: redis::cluster::ClusterConnection,
}

impl AuthProcessor {
    pub(crate) fn create_new(hostname: &str, port: u16) -> Self {
        let nodes = vec![format!("redis://{}:{}/9",hostname,port)];
        let redis_key_client = ClusterClient::open(nodes).unwrap();
        let redis_key = redis_key_client.get_connection().unwrap();
        let nodes = vec![format!("redis://{}:{}/10",hostname,port)];
        let redis_jwt_client = ClusterClient::open(nodes).unwrap();
        let redis_jwt = redis_jwt_client.get_connection().unwrap();
        Self {
            redis_key,
            redis_jwt,
        }
    }

    pub(crate) fn authenticate_user(&mut self){

    }

}
fn is_token_satker_correct(the_satker:&str,intended_satker:&str) -> bool{
    if(the_satker == ""){
        return false
    }
    if(intended_satker == ""){
        return false
    }
    if(intended_satker != intended_satker){
        return false
    }
    true
}
fn is_token_expired(the_date:&DateTime<FixedOffset>)-> bool{
    if the_date > &Utc::now(){
        return false
    }
    true
}
fn is_token_value_correct(the_value:&str)->bool{
    true
}
