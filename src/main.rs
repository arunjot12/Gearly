pub mod db;
pub mod models;
pub mod schema;
pub mod dashboard;
pub mod signup;
pub mod login;
pub mod cli;
pub mod auth;

use axum::{Json, Router, routing::{get,post}, serve};
use tokio::net::TcpListener;
use serde_json::{json,Value};
use crate::{
    auth::jwt::JwtService, 
    dashboard::product::protected_dashboard, 
    db::{create_pool,DbPool}, login::creditionals::login_user, models::NewSignupShopkeepers, 
    signup::api::{signup_shopkeeper, signup_users},
};

#[derive(Clone)]
pub struct AppState{
    pub db: DbPool,
    pub jwt: JwtService
}

#[tokio::main]
async fn main() {
    let jwt_service = JwtService::new();
    let pool = create_pool();

    let state = AppState{ db: pool, jwt:jwt_service};

    let app = Router::new()
    .route("/signup_shopkeeper",post(signup_shopkeeper))
    .route("/signup_user", post(signup_users))
    .route("/login_user", post(login_user))
    .route("/dashboard",get(protected_dashboard))
    .route("/health",get(health_check))
    .with_state(state);

    let port: u16 = std::env::var("PORT")
    .ok()
    .and_then(|p| p.parse().ok())
    .unwrap_or(3000);

    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))
    .await
    .unwrap_or_else(|e| panic!("failed to bind to port {port}: {e}"));

    tracing::info!(port, "🚀 server listening");

    print_startup_info();
    serve(listener, app).await.unwrap();
}

pub fn print_startup_info() {
    println!();
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║                    🚗  GEARLY API                        ║");
    println!("║              Car Parts Marketplace Backend               ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    println!("  ✓ Database       Connected");
    println!("  ✓ JWT            Initialized");
    println!("  ✓ Server         Ready");
    println!();
    println!("  Routes");
    println!("  ────────────────────────────────────────────────────────");
    println!("  POST   /signup_shopkeeper");
    println!("  POST   /signup_user");
    println!("  POST   /login_user");
    println!("  GET    /dashboard");
    println!("  GET    /health");

    println!();

    println!("  🚀 Server running at http://127.0.0.1:3000");
    println!();
}

#[axum::debug_handler]
pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ready"
    }
    ))
}