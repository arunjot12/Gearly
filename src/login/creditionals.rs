use crate::{models::SignupShopkeepers, schema::signup_shopkeepers, 
    schema::signup_shopkeepers::dsl::*,
    db::establish_connection, models::Login
};
use axum::{Json, http::StatusCode};
use diesel::prelude::*;
use argon2::{PasswordHash, Argon2, PasswordVerifier};

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

    // Parse the Argon2 hash
    let parsed_hash = PasswordHash::new(password_db)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    // Verify the password from JSON against the DB hash
   let _ =  Argon2::default()
        .verify_password(
            payload.password.as_bytes(),
            &parsed_hash,
        )
        .map_err(|_| StatusCode::UNAUTHORIZED);

    Ok(StatusCode::OK)
}
