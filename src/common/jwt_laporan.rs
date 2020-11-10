use crate::common::AuthResult;
use chrono::{DateTime, FixedOffset, Utc};
use jsonwebtoken::{decode, encode, errors, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use redis::{cluster::ClusterClient, Client, Commands, Connection};
use serde::{Deserialize, Serialize};
/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
/// Disini masukan field yang akan diterima dalam token apa saja untuk laporan
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

pub fn encode_into_jwt(the_struct: Claims) -> Result<String, ()> {
    let mut header = Header::new(Algorithm::HS512);
    header.kid = Some("blabla".to_owned());
    let key = "secret".as_ref();
    let encoded = encode(&header, &the_struct, &EncodingKey::from_secret(key)).unwrap();
    Ok(encoded)
}

pub fn decode_from_jwt(token: String) -> Result<TokenData<Claims>, errors::Error> {
    let key = "secret".as_ref();
    let decoded = decode::<Claims>(&token, &DecodingKey::from_secret(key), &Validation::new(Algorithm::HS512))?;
    Ok(decoded)
}

pub(crate) struct AuthProcessor {
    redis_key: redis::cluster::ClusterConnection,
    redis_jwt: redis::cluster::ClusterConnection,
}

impl AuthProcessor {
    pub(crate) fn create_new(hostname: &str, port: u16) -> Self {
        let nodes = vec![format!("redis://{}:{}/9", hostname, port)];
        let redis_key_client = ClusterClient::open(nodes).unwrap();
        let redis_key = redis_key_client.get_connection().unwrap();
        let nodes = vec![format!("redis://{}:{}/10", hostname, port)];
        let redis_jwt_client = ClusterClient::open(nodes).unwrap();
        let redis_jwt = redis_jwt_client.get_connection().unwrap();

        Self { redis_key, redis_jwt }
    }

    pub(crate) fn authenticate_user(&mut self, token: &str, inputted_satker: &str) -> AuthResult {
        let current_time: DateTime<FixedOffset> = Utc::now().into();
        //TODO: get key from redis DB 9
        //TODO: get jwt from redis DB 10
        //TODO: check validity (token exist? token value equal? token expired? satker correct?)
        //TODO: alldata adalah placeholder extraksi dari jwt
        //TODO: jwtdate adalah placeholder extraksi dari jwt dapat tanggal expire jwtnya
        //TODO: jwtsatker adalah placeholder extraksi dari jwt dapat satker yg ada dalam jwt
        let alldata = "";
        let jwtdate = current_time;
        let jwtsatker = "";
        let intended_satker = inputted_satker;

        if is_token_value_correct(&alldata) {
            return AuthResult::TokenInvalid;
        };

        if is_token_expired(&jwtdate) {
            let tanggal_expired: DateTime<FixedOffset> = jwtdate.clone();
            return AuthResult::TokenExpired(tanggal_expired);
        };

        if is_token_satker_correct(&jwtsatker, &intended_satker) {
            let requested_satker = jwtsatker.clone().into();
            let correct_satker = intended_satker.clone().into();
            return AuthResult::ForbiddenAccess(requested_satker, correct_satker);
        };

        AuthResult::Ok
    }
}

fn is_token_satker_correct(the_satker: &str, intended_satker: &str) -> bool {
    if the_satker == "" {
        return false;
    }

    if intended_satker == "" {
        return false;
    }

    if the_satker != intended_satker {
        return false;
    }

    true
}

fn is_token_expired(the_date: &DateTime<FixedOffset>) -> bool {
    if the_date > &Utc::now() {
        return false;
    }
    true
}

fn is_token_value_correct(the_value: &str) -> bool {
    true
}
