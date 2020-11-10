use serde::{Serialize, Deserialize};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode, errors};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
/// Disini masukan field yang akan diterima dalam token apa saja untuk laporan
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize, 
}
pub(crate) fn encode_into_jwt(the_struct:Claims) -> Result<String,()>{
    let mut header = Header::new(Algorithm::HS512);
    header.kid = Some("blabla".to_owned());
    let key = "secret".as_ref(); 
    let encoded = encode(&header, &the_struct, &EncodingKey::from_secret(key)).unwrap();
    Ok(encoded)
}
pub(crate) fn decode_from_jwt(token:String) -> Result<TokenData<Claims>,errors::Error> {
    let key = "secret".as_ref();
    let decoded = decode::<Claims>(&token, &DecodingKey::from_secret(key), &Validation::new(Algorithm::HS512))?;
    Ok(decoded)
}
