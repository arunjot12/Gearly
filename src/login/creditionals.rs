use crate::{
    auth::jwt::JwtService,
    db::establish_connection,
    models::{Login, SignupShopkeepers, Users},
    schema::signup_shopkeepers::{self, dsl::*},
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, http::StatusCode};
use diesel::prelude::*;

pub async fn login_shopkeeper(Json(payload): Json<Login>) -> Result<StatusCode, String> {
    let mut connection = establish_connection();
    let shopkeeper = signup_shopkeepers::table
        .filter(
            email
                .eq(&payload.username_or_email)
                .or(signup_shopkeepers::username.eq(&payload.username_or_email)),
        )
        .first::<SignupShopkeepers>(&mut connection)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    let shopkeeper = match shopkeeper {
        Ok(shopkeeper) => shopkeeper,
        Err(_) => return Err(StatusCode::UNAUTHORIZED.to_string()),
    };

    let password_db = match &shopkeeper.password {
        Some(hash) => hash,
        None => return Err(StatusCode::UNAUTHORIZED.to_string()),
    };

    if verify_password_details(&payload.password, password_db) == true {
        return Ok(StatusCode::OK);
    } else {
        return Err("Data Not Found".to_string());
    }
}

#[axum::debug_handler]
pub async fn login_user(Json(payload): Json<Login>) -> Result<Json<String>, String> {
    let mut connection = establish_connection();
    let user = crate::schema::users::table
        .filter(
            crate::schema::users::dsl::email
                .eq(&payload.username_or_email)
                .or(crate::schema::users::username.eq(&payload.username_or_email)),
        )
        .first::<Users>(&mut connection)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    let user = match user {
        Ok(user) => user,
        Err(_) => return Err(StatusCode::UNAUTHORIZED.to_string()),
    };

    let password_db = match &user.password {
        Some(hash) => hash,
        None => return Err(StatusCode::UNAUTHORIZED.to_string()),
    };

    if !verify_password_details(&payload.password, password_db) == true {
        return Err("Invalid Creditionals".to_string());
    }

    let jwt_service = JwtService::new();
    let token = jwt_service
        .create_token(user.id, "Customer".to_string())
        .map_err(|_| "Failed to create JWT".to_string())?;

    Ok(Json(token))
}

fn verify_password_details(other_password: &str, password_hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(password_hash) {
        Ok(hash) => hash,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(other_password.as_bytes(), &parsed_hash)
        .is_ok()
}
