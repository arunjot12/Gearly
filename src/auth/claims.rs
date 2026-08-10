use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessClaims{
    pub sub: i32,
    pub exp:i64,
    pub email: String,
    pub roles: Vec<String>,
    pub iat: i64
}

impl AccessClaims{
    pub fn new(user_id: i32, email: String, roles: Vec<String>, expires_in: Duration) -> Self{
        let now = Utc::now();
            Self { 
                sub: user_id, 
                exp: (now+ expires_in).timestamp(), 
                email,
                roles, 
                iat: now.timestamp() 
            }
        }

    pub fn is_expired(&self) -> bool{
        Utc::now().timestamp() > self.exp
    }
}

