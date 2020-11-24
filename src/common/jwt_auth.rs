use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, errors, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::common::AuthResultJwt;
use crate::config::AppConfig;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
/// Disini masukan field yang akan diterima dalam token apa saja untuk laporan
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub username: String,
    pub user_id: u32,
    pub email: String,
    pub exp: usize,
}

pub fn encode_into_jwt(the_struct: Claims) -> Result<String, ()> {
    let mut header = Header::new(Algorithm::HS512);
    header.kid = Some("blabla".to_owned());
    let key = b"secret";
    //TODO SEND KEY TO REDIS
    let encoded = encode(&header, &the_struct, &EncodingKey::from_secret(key)).unwrap();
    Ok(encoded)
}

pub fn decode_from_jwt(token: String) -> Result<TokenData<Claims>, errors::Error> {
    let key = b"secret";
    //TODO RECIEVE KEY FROM REDIS
    let decoded = decode::<Claims>(&token, &DecodingKey::from_secret(key), &Validation::new(Algorithm::HS512))?;
    Ok(decoded)
}

pub(crate) fn decode_jwt(token: Option<String>) -> AuthResultJwt {
    let token = token.unwrap_or("".to_string());
    if token == "".to_string() {
        return AuthResultJwt::NotAuthenticated;
    }

    let config = AppConfig::load_from_env();
    let secret_key = config.jwt_secret.as_str().as_bytes();

    let validation = Validation { ..Validation::default() };
    //TODO RECIEVE KEY FROM REDIS

    match decode::<Claims>(&token, &DecodingKey::from_secret(secret_key), &validation) {
        Ok(data) => return AuthResultJwt::Ok(data.claims),
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => return AuthResultJwt::TokenInvalid, // Example on how to handle a specific error
            ErrorKind::ExpiredSignature => return AuthResultJwt::TokenExpired,
            _ => panic!("Some other errors"),
        },
    };
}
