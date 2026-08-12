pub mod db;
pub mod models;
pub mod schema;
pub mod dashboard;
pub mod signup;
pub mod login;
pub mod cli;
pub mod auth;
use axum::{Router,serve,routing::{get,post}};
use tokio::net::TcpListener;
use crate::{models::NewSignupShopkeepers, 
    signup::api::{signup_shopkeeper, signup_users},
    login::creditionals::login_user,
    dashboard::product::protected_dashboard,
    auth::jwt::JwtService
};

#[tokio::main]
async fn main() {
    let jwt_service = JwtService::new();

    let app = Router::new()
    .route("/signup_shopkeeper",post(signup_shopkeeper))
    .route("/signup_user", post(signup_users))
    .route("/login_user", post(login_user))
    .route("/dashboard",get(protected_dashboard))
    .with_state(jwt_service);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server Started Bro ");
    serve(listener, app).await.unwrap();
}