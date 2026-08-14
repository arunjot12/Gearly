use crate::{
    AppState, NewSignupShopkeepers,
    models::NewUsers,
    signup::{
        handler::{handle_customer_signup, handle_shopkeeper_signup},
        signup_shopkeeper::check_signup_shopkeeper,
        signup_users::check_signup_user,
    },
};
use axum::{Json, extract::State, http::StatusCode};

pub async fn signup_shopkeeper(
    Json(payload): Json<NewSignupShopkeepers>,
    State(state): State<AppState>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let shopkeeper = match check_signup_shopkeeper(payload) {
        Ok(shopkeeper) => shopkeeper,
        Err(err) => {
            return Err((StatusCode::BAD_REQUEST, err.to_string()));
        }
    };
    let connection = state
        .db
        .get()
        .await
        .expect("Failed to get DB connection from pool");
    let result = connection
        .interact(move |conn| handle_shopkeeper_signup(conn, &shopkeeper))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?; // interact() error (panic in closure)

    match result {
        Ok(_) => Ok((
            StatusCode::CREATED,
            "successfully created shopkeeper".to_string(),
        )),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}

pub async fn signup_users(
    Json(payload): Json<NewUsers>,
    State(state): State<AppState>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let user = match check_signup_user(payload) {
        Ok(shopkeeper) => shopkeeper,
        Err(err) => {
            return Err((StatusCode::BAD_REQUEST, err.to_string()));
        }
    };
    let conn = state
        .db
        .get()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let result = conn
        .interact(move |conn| handle_customer_signup(conn, &user))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match result {
        Ok(_) => Ok((
            StatusCode::CREATED,
            "successfully created shopkeeper".to_string(),
        )),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}
