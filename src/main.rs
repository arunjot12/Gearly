pub mod db;
pub mod models;
pub mod schema;
pub mod signup;
pub mod login;
pub mod cli;
use axum::{Router,serve,routing::post};
use tokio::net::TcpListener;
use crate::{models::NewSignupShopkeepers, signup::api::{signup_shopkeeper, signup_users} ,login::creditionals::login_user};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/signup_shopkeeper",post(signup_shopkeeper))
    .route("/signup_user", post(signup_users))
    .route("/login_user", post(login_user))
    ;
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server Started Bro ");
    serve(listener, app).await.unwrap();
}
