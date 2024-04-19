use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use sqlx::types::uuid;

use crate::{model::project, schema::FilterOptions, AppState};

use super::to_response;

pub async fn all(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Param
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    // Query with macro
    let projects = sqlx::query_as!(
        project::Model,
        r"SELECT * FROM projects ORDER by id LIMIT ? OFFSET ?",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    // Response
    let project_responses = projects
        .iter()
        .map(|project| to_response(&project))
        .collect::<Vec<project::ModelResponse>>();

    let json_response = serde_json::json!({
        "status": "ok",
        "count": project_responses.len(),
        "notes": project_responses
    });

    Ok(Json(json_response))
}

pub async fn by_id(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // get using query macro
    let query_result = sqlx::query_as!(
        project::Model,
        r"SELECT * FROM projects WHERE id = ?",
        id.to_string()
    )
    .fetch_one(&data.db)
    .await;

    // check and response
    match query_result {
        Ok(project) => {
            let project_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "project": to_response(&project)
                })
            });

            return Ok(Json(project_response));
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Note with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status":"error", "message":format!("{:?}",e)})),
            ));
        }
    };
}
