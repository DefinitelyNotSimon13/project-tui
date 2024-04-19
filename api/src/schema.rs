use serde::{Deserialize, Serialize};
pub mod project;

// List
#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

// Read/Delete
#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}
