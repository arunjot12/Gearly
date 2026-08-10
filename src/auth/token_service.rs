use chrono::Duration;
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Validation};
use std::env;

use super::claims::{AccessClaims, RefreshClaims};
use super::auth::TokenPair;

#[derive(Clone)]
pub struct TokenService{
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_token_till: Duration,
    refresh_token_till: Duration
}

impl TokenService{
    pub fn new() -> Self {
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-super-secret-key-for-dev-only".to_string());
        
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            access_token_till: Duration::minutes(15),
            refresh_token_till: Duration::days(7),
        }
    }

    pub fn generate_tokens(&self, user_id: i32, email: String, roles: Vec<String>) -> Result<TokenPair, jsonwebtoken::errors::Error> {
        let access_claims = AccessClaims::new(user_id, email, roles, self.access_token_till);
        let refresh_claims = RefreshClaims::new(user_id, self.refresh_token_till);

        let access_token = encode(&Header::default(), &access_claims, &self.encoding_key)?;
        let refresh_token = encode(&Header::default(), &refresh_claims, &self.encoding_key)?;

        Ok(TokenPair {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.access_token_till.num_seconds().to_string(),
        })
    }

    pub fn verify_access_token(&self, token: &str) -> Result<AccessClaims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::default();
        validation.validate_exp = true;
        validation.set_required_spec_claims(&["exp", "sub", "iat"]);
        
        let token_data = decode::<AccessClaims>(token, &self.decoding_key, &validation)?;
        if token_data.claims.typ != "access" {
            return Err(jsonwebtoken::errors::ErrorKind::InvalidToken.into());
        }
        Ok(token_data.claims)
    }

    pub fn verify_refresh_token(&self, token: &str) -> Result<RefreshClaims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::default();
        validation.validate_exp = true;
        validation.set_required_spec_claims(&["exp", "sub", "iat"]);
        
        let token_data = decode::<RefreshClaims>(token, &self.decoding_key, &validation)?;
        if token_data.claims.typ != "refresh" {
            return Err(jsonwebtoken::errors::ErrorKind::InvalidToken.into());
        }
        Ok(token_data.claims)
    }
}