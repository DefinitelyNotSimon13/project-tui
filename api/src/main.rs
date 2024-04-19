use std::sync::Arc;

use axum::http::{header::CONTENT_TYPE, Method};

use dotenv::dotenv;
use tokio::net::TcpListener;

use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub mod model;

pub mod handler;

use tower_http::cors::{Any, CorsLayer};
pub mod schema;

pub mod route;

pub struct AppState {
    db: MySqlPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("🌟 Project API 🌟");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Successfully connected to database!");
            pool
        }
        Err(e) => {
            println!("❌ Failed to connect  to database: {e:?}");

            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = route::create_router(Arc::new(AppState { db: pool.clone() })).layer(cors);

    let listener = TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("failed to bind TcpListener");

    println!("✅ Server started successfully at 127.0.0.1:8000");

    axum::serve(listener, app.into_make_service())
        .await
        .expect("failed to serve");
}
