use std::sync::Arc;

use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    handler::{health_check, project},
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthcheck", get(health_check))
        .route("/api/projects", post(project::create))
        .route("/api/projects", get(project::read::all))
        .route("/api/projects/:id", get(project::read::by_id))
        .route("/api/projects/:id", put(project::update))
        .route("/api/projects/:id", delete(project::delete))
        .with_state(app_state)
}
