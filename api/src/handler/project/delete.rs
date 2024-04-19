use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use sqlx::types::uuid;

use super::{AppData, Delete, JsonResponse, Project};

impl Delete for Project {
    async fn delete(
        Path(id): Path<uuid::Uuid>,
        State(data): AppData,
    ) -> Result<impl IntoResponse, JsonResponse> {
        let query_result = sqlx::query!(r"DELETE FROM projects WHERE id = ?", id.to_string())
            .execute(&data.db)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": format!("{:?}",e)
                    })),
                )
            })?;

        // response
        if query_result.rows_affected() == 0 {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("project with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }

        Ok(StatusCode::NO_CONTENT)
    }
}
