use serde::{Deserialize, Serialize};

// Model for Sqlx
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub description: String,
    pub language_id: Option<String>,
    pub category_id: Option<String>,
    pub major_version: i32,
    pub minor_version: i32,
    pub patch_version: i32,
    pub github_repo: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ModelResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub major_version: i32,
    pub minor_version: i32,
    pub patch_version: i32,
    pub githup_repo: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
