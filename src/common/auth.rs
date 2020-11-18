use crate::common::AuthResult;
use crate::common::{
    jwt_auth::decode_from_jwt,
    redis_helper::{RedisMockProcessor, RedisProcessor, RedisRealProcessor},
};
use actix_web::HttpRequest;
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use redis::{cluster::ClusterClient, Client, Commands, Connection};
use serde::{Deserialize, Serialize};

//TODO had to delete the processor and replace it with getting connection
pub(crate) struct AuthProcessor {
    redix: RedisProcessor,
}
//TODO: need replacement for these validate token
pub(crate) fn validate_jwt_token(the_token: &str) -> bool {
    true
}
impl AuthProcessor {
    //TODO: get redis connection from redis helper
    pub(crate) fn authenticate_user(&mut self, token: &str, inputted_satker: &str) -> AuthResult {
        let current_time: DateTime<FixedOffset> = Utc::now().into();
        //TODO: get key from redis DB 9
        //TODO: get jwt from redis DB 10
        //TODO: check validity (token exist? token value equal? token expired? satker correct?)
        //TODO: alldata adalah placeholder extraksi dari jwt
        //TODO: jwtdate adalah placeholder extraksi dari jwt dapat tanggal expire jwtnya
        //TODO: jwtsatker adalah placeholder extraksi dari jwt dapat satker yg ada dalam jwt
        let fakedata = decode_from_jwt(token.into()).unwrap();
        let alldata = format!("{:#?}", fakedata);
        let jwtdate = fakedata.claims.exp;
        let jwtsatker = "";
        let intended_satker = inputted_satker;

        if self.is_token_value_correct(&alldata) {
            return AuthResult::TokenInvalid;
        };

        if self.is_token_expired(jwtdate) {
            // let tanggal_expired: DateTime<FixedOffset> = jwtdate.clone();
            return AuthResult::TokenExpired(Utc::now().into());
        };

        if self.is_token_satker_correct(&jwtsatker, &intended_satker) {
            let requested_satker = jwtsatker.into();
            let correct_satker = intended_satker.into();
            return AuthResult::ForbiddenAccess(requested_satker, correct_satker);
        };

        AuthResult::Ok
    }
    pub(crate) fn init(host: &str, port: u16) -> AuthProcessor {
        // let redix = RedisProcessor::new(host, port)
        let redix = RedisProcessor {
            redis_impl: Box::new(RedisMockProcessor::new(host, port)),
        };
        Self { redix }
    }
    fn is_token_satker_correct(&mut self, the_satker: &str, intended_satker: &str) -> bool {
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

    fn is_token_expired(&mut self, the_date: usize) -> bool {
        if Utc::now().timestamp() > the_date as i64 {
            return false;
        }
        true
    }

    fn is_token_value_correct(&mut self, the_value: &str) -> bool {
        if self.redix.redis_impl.find_key(the_value).is_err() {
            return false;
        }
        true
    }
}

// get header value from api request
pub fn get_header_value<'a>(req: &'a HttpRequest, header: &str) -> Option<&'a str> {
    req.headers().get(header)?.to_str().ok()
}
