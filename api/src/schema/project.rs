use serde::{Deserialize, Serialize};

// Create
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProjectSchema {
    pub name: String,
    pub description: String,
    pub language: String,
    pub category: String,
    pub major_version: Option<i32>,
    pub minor_version: Option<i32>,
    pub patch_version: Option<i32>,
    pub github_repo: Option<String>,
}

// Update
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateProjectSchema {
    pub name: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub category: Option<String>,
    pub major_version: Option<i32>,
    pub minor_version: Option<i32>,
    pub patch_version: Option<i32>,
    pub github_repo: Option<String>,
}
