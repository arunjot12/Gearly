use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
    pub roles: String,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: i32, roles: String, expires_in: Duration) -> Self {
        let now = Utc::now();
        Self {
            sub: user_id,
            exp: (now + expires_in).timestamp(),
            roles,
            iat: now.timestamp(),
        }
    }
}
