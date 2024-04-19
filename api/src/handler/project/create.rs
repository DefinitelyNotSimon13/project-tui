use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use sqlx::types::uuid;

use super::{to_response, AppData, Create, JsonResponse, Project};
use crate::{model::project, schema::project::CreateProjectSchema};

impl Create for Project {
    async fn create(
        State(data): AppData,
        Json(body): Json<CreateProjectSchema>,
    ) -> Result<impl IntoResponse, JsonResponse> {
        // Insert
        let id = uuid::Uuid::new_v4().to_string();
        let query_result = sqlx::query(
            r"INSERT INTO projects
        (
            id, 
            name, 
            description, 
            major_version, 
            minor_version, 
            patch_version, 
            github_repo
        ) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(id.clone())
        .bind(body.name.to_string())
        .bind(body.description.to_string())
        .bind(body.major_version.unwrap_or(0))
        .bind(body.minor_version.unwrap_or(0))
        .bind(body.patch_version.unwrap_or(0))
        .bind(body.github_repo)
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

        // Duplicate err check
        if let Err(err) = query_result {
            if err.contains("Duplicate entry") {
                let error_response = serde_json::json!({
                    "status": "error",
                    "message": "Project already exists!"
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }

            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message":format!("{:?}", err)})),
            ));
        }

        // Get inserted project by ID
        let project = sqlx::query_as!(project::Model, r"SELECT * FROM projects WHERE id = ?", id)
            .fetch_one(&data.db)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"status":"error","message":format!("{:?}", e)})),
                )
            })?;

        let project_response = serde_json::json!({
            "status": "success",
            "data": serde_json::json!({
                "project": to_response(&project)
            })
        });

        Ok(Json(project_response))
    }
}
