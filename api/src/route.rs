use std::sync::Arc;

use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    handler::{health_check, project::Project, Create, Delete, Read, Update},
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthcheck", get(health_check))
        .route("/api/projects", post(Project::create))
        .route("/api/projects", get(Project::read_all))
        .route("/api/projects/:id", get(Project::read_id))
        .route("/api/projects/:id", put(Project::update))
        .route("/api/projects/:id", delete(Project::delete))
        .with_state(app_state)
}
