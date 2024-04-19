pub mod create;
pub mod delete;
pub mod read;
pub mod update;

pub use super::{AppData, Create, Delete, JsonResponse, Read, Update};
use crate::model::project;

pub struct Project;

// Convert DB Model to response
fn to_response(project: &project::Model) -> project::ModelResponse {
    project::ModelResponse {
        id: project.id.clone(),
        name: project.name.clone(),
        description: project.description.clone(),
        major_version: project.major_version,
        minor_version: project.minor_version,
        patch_version: project.patch_version,
        githup_repo: project.github_repo.clone(),
        created_at: project.created_at.unwrap(),
        updated_at: project.updated_at.unwrap(),
    }
}
