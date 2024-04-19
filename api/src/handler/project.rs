pub mod create;
pub use create::create;
pub mod delete;
pub use delete::delete;
pub mod read;
pub mod update;
pub use update::update;

use crate::model::project;

// Convert DB Model to response
fn to_response(project: &project::Model) -> project::ModelResponse {
    project::ModelResponse {
        id: project.id.to_owned(),
        name: project.name.to_owned(),
        description: project.name.to_owned(),
        major_version: project.major_version,
        minor_version: project.minor_version,
        patch_version: project.patch_version,
        githup_repo: project.github_repo.to_owned(),
        created_at: project.created_at.unwrap(),
        updated_at: project.updated_at.unwrap(),
    }
}
