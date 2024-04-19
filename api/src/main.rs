use axum::{response::IntoResponse, routing::get, Json, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/healthcheck", get(health_check_handler));

    println!("Server started successfulle at 127.0.0.1:8080");

    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("failed to bind TcpListener");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("failed to serve");
}

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Project API Service";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}
