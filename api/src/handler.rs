use axum::{response::IntoResponse, Json};

pub mod project;

pub async fn health_check() -> impl IntoResponse {
    const MESSAGE: &str = "Project API Service";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}
