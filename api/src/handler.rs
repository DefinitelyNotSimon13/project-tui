use std::{future::Future, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    schema::{
        project::{CreateProjectSchema, UpdateProjectSchema},
        FilterOptions,
    },
    AppState,
};

pub mod project;

pub type AppData = State<Arc<AppState>>;
pub type JsonResponse = (StatusCode, Json<serde_json::Value>);

pub trait Create {
    fn create(
        data: AppData,
        body: Json<CreateProjectSchema>,
    ) -> impl Future<Output = Result<impl IntoResponse, JsonResponse>> + Send;
}

pub trait Update {
    fn update(
        id: Path<uuid::Uuid>,
        data: AppData,
        body: Json<UpdateProjectSchema>,
    ) -> impl Future<Output = Result<impl IntoResponse, JsonResponse>> + Send;
}

pub trait Read {
    fn read_all(
        opts: Option<Query<FilterOptions>>,
        data: AppData,
    ) -> impl Future<Output = Result<impl IntoResponse, JsonResponse>> + Send;
    fn read_id(
        id: Path<uuid::Uuid>,
        data: AppData,
    ) -> impl Future<Output = Result<impl IntoResponse, JsonResponse>> + Send;
}

pub trait Delete {
    fn delete(
        id: Path<uuid::Uuid>,
        data: AppData,
    ) -> impl Future<Output = Result<impl IntoResponse, JsonResponse>> + Send;
}

pub async fn health_check() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "status": "ok",
        "message": "Project API Service"
    });

    Json(json_response)
}
