use jsonwebtoken::{decode, encode, errors, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
/// Disini masukan field yang akan diterima dalam token apa saja untuk laporan
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id_user: String,
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
