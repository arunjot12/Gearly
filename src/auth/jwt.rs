use axum::{
    Extension, Json, RequestPartsExt,
    extract::FromRequestParts,
    http::{StatusCode, header, request::Parts},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::sync::Arc;

use super::claims::AccessClaims;
use super::token_service::TokenService;

pub enum AuthError {
    MissingToken,
    InvalidToken,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: &'static str,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authorization token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid or expired token"),
        };
        let body = Json(ErrorResponse {
            error: error_message,
        });
        (status, body).into_response()
    }
}

// This extractor will allow you to use `claims: AccessClaims` as a handler parameter.
// It assumes you have added `Extension(Arc::new(token_service))` to your axum router layers.
impl<S> FromRequestParts<S> for AccessClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Get the token service from the extensions
        let Extension(token_service) = parts
            .extract::<Extension<Arc<TokenService>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        // Extract the token from the authorization header
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .filter(|value| value.starts_with("Bearer "))
            .map(|value| &value[7..]);

        let token = match auth_header {
            Some(token) => token,
            None => return Err(AuthError::MissingToken),
        };

        // Verify the token
        match token_service.verify_access_token(token) {
            Ok(claims) => Ok(claims),
            Err(_) => Err(AuthError::InvalidToken),
        }
    }
}
