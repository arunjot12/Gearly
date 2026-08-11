use axum::{
    extract::FromRequestParts, http::{StatusCode, header, request::Parts}, response::IntoResponse,
};
use thiserror::Error;
use super::{
    claims::Claims,
    jwt::JwtService,
};

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

impl<S> FromRequestParts<S> for Claims
where 
    S: Send + Sync 
{
    type Rejection = AuthError;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection>
    {
        
    }
}