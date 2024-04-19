use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use sqlx::types::uuid;

use super::{to_response, AppData, JsonResponse, Project, Update};
use crate::{model::project, schema::project::UpdateProjectSchema};

impl Update for Project {
    async fn update(
        Path(id): Path<uuid::Uuid>,
        State(data): AppData,
        Json(body): Json<UpdateProjectSchema>,
    ) -> Result<impl IntoResponse, JsonResponse> {
        // validate project with query macro
        let query_result = sqlx::query_as!(
            project::Model,
            r"SELECT * FROM projects WHERE id = ?",
            id.to_string()
        )
        .fetch_one(&data.db)
        .await;

        // fetch the result
        let project = match query_result {
            Ok(project) => project,
            Err(sqlx::Error::RowNotFound) => {
                let error_response = serde_json::json!({
                    "status": "error",
                    "message": format!("Project with ID: {} not found", id)
                });
                return Err((StatusCode::NOT_FOUND, Json(error_response)));
            }
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                    "status":"error",
                    "message":format!("{:?}",e)
                    })),
                ));
            }
        };

        let update_result = sqlx::query(
            r"UPDATE projects 
            SET name = ?,
            SET description = ?,
            SET major_version = ?,
            SET minor_version = ?,
            SET patch_version = ?,
            SET github_repo = ?
            WHERE id = ?
        ",
        )
        .bind(body.name.clone().unwrap_or_else(|| project.name.clone()))
        .bind(
            body.description
                .clone()
                .unwrap_or_else(|| project.description.clone()),
        )
        .bind(body.major_version.unwrap_or(project.major_version))
        .bind(body.minor_version.unwrap_or(project.minor_version))
        .bind(body.patch_version.unwrap_or(project.patch_version))
        .bind(match body.github_repo {
            Some(repo) => Some(repo),
            None => match project.github_repo {
                Some(_) => project.github_repo.clone(),
                None => None,
            },
        })
        .bind(id.to_string())
        .execute(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                "status":"error",
                "message":format!("{:?}",e)
                })),
            )
        })?;

        // if no data affected
        if update_result.rows_affected() == 0 {
            let error_response = serde_json::json!({
                "status":"error",
                "message":format!("Note with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }

        // get updated data
        let updated_project = sqlx::query_as!(
            project::Model,
            r"SELECT * FROM projects WHERE id = ?",
            id.to_string()
        )
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message":format!("{:?}", e)})),
            )
        })?;

        let project_response = serde_json::json!({
            "status": "success",
            "data": serde_json::json!({
                "project": to_response(&updated_project)
            })
        });

        Ok(Json(project_response))
    }
}
