use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub language: String,
    pub major_version: String,
    pub minor_version: String,
    pub patch_version: String,
    pub category: String,
    pub github_repo: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl Project {
    pub fn error(message: &str) -> Self {
        Project {
            name: "Error".to_string(),
            description: message.to_string(),
            language: "Error".to_string(),
            major_version: "Error".to_string(),
            minor_version: "Error".to_string(),
            patch_version: "Error".to_string(),
            category: "Error".to_string(),
            github_repo: None,
            tags: None,
        }
    }
}
