use serde::Deserialize;

#[derive(Debug)]
pub struct UserInfo{
    pub id: i32,
    pub email: String,
    pub first_name: String
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest{
    pub refresh_request: String
}

#[derive(Debug, Deserialize)]
pub struct AutheResponse{
    pub user: UserInfo,
    pub tokens: TokenPair
}