pub(crate) mod auth;
pub(crate) mod jwt_auth;
pub(crate) mod jwt_laporan;
pub(crate) mod redis_helper;
use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) enum AuthResult {
    Ok,
    NotAuthenticated,
    TokenInvalid,
    TokenExpired(DateTime<FixedOffset>),
    ForbiddenAccess(String, String),
}
