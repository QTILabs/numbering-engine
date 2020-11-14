use actix_web::HttpRequest;

pub fn validate_jwt_token(token: String) -> bool {
    // Token validation here
    println!("{:?}", token);

    true
}

// get header value from api request
pub fn get_header_value<'a>(req: &'a HttpRequest, header: &str) -> Option<&'a str> {
    req.headers().get(header)?.to_str().ok()
}
