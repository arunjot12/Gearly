use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessClaims {
    pub sub: i32,
    pub exp: i64,
    pub email: String,
    pub roles: Vec<String>,
    pub iat: i64,
    pub typ: String,
}

impl AccessClaims {
    pub fn new(user_id: i32, email: String, roles: Vec<String>, expires_in: Duration) -> Self {
        let now = Utc::now();
        Self {
            sub: user_id,
            exp: (now + expires_in).timestamp(),
            email,
            roles,
            typ: "access".to_string(),
            iat: now.timestamp(),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshClaims {
    pub sub: i32,
    pub exp: i64,
    pub iat: i64,
    pub typ: String,
}

impl RefreshClaims {
    pub fn new(user_id: i32, expires_in: Duration) -> Self {
        let now = Utc::now();

        Self {
            sub: user_id,
            exp: (now + expires_in).timestamp(),
            iat: now.timestamp(),
            typ: "refresh".to_string(),
        }
    }
}
