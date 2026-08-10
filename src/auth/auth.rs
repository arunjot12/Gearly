use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo{
    pub id: i32,
    pub email: String,
    pub first_name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair{
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in : String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshRequest{
    pub refresh_token: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthResponse{
    pub user: UserInfo,
    pub tokens: TokenPair
}