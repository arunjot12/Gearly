use axum::{
    extract::FromRequestParts, http::{StatusCode, header,request::Parts}, response::IntoResponse,
};
use thiserror::Error;
use super::{
    claims::Claims,
};
use crate::auth::jwt::JwtService;

#[derive(Debug,Error)]
pub enum AuthError{
    #[error("Missing Authenication Token")]
    MissingToken,
    #[error("Invalid Token")]
    InvalidToken
}

impl IntoResponse for AuthError{
        fn into_response(self) -> axum::response::Response {
            match self {
                AuthError::InvalidToken => (
                    StatusCode::UNAUTHORIZED,
                    "Invalid Token"
                ).into_response(),
                AuthError::MissingToken => (
                    StatusCode::UNAUTHORIZED,
                    "Missing Authenication Code"
                ).into_response()
            }
        }
}

impl FromRequestParts<JwtService> for Claims
{
    type Rejection = AuthError;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &JwtService,
    ) -> Result<Self, Self::Rejection>
    {
        let auth_headers = parts.headers.get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .filter(|value| value.starts_with("Bearer"))
        .map(|value | &value[7..]); 
       
        match state.verify_token(auth_headers.expect("Reason")) {
            Ok(claims) => Ok(claims),
            Err(_) => Err(AuthError::InvalidToken),
        }
}
}