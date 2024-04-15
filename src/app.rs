use crate::project::Project;
use color_eyre::Result;
use std::fs;
use walkdir::WalkDir;

pub struct App {
    pub projects: Vec<(Project, String)>,
    pub test_contents: String,
    pub current_dir: (Option<Project>, String),
    pub exiting: bool,
}

impl App {
    pub fn new() -> Self {
        let (project, path) = App::read_from_file("project.json");
        App {
            projects: vec![],
            test_contents: "".to_string(),
            current_dir: (project, path),
            exiting: false,
        }
    }

    pub fn add_to_project_vector(&mut self, project: Option<Project>, path: String) -> Result<()> {
        match project {
            Some(project) => self.projects.push((project, path)),
            None => self.projects.push((Project::error(&path), path)),
        }
        Ok(())
    }

    pub fn read_from_file(file: &str) -> (Option<Project>, String) {
        let content = fs::read_to_string(file);
        let path = fs::canonicalize(file);
        let path = match path {
            Ok(path) => path.to_string_lossy().into_owned(),
            Err(e) => e.to_string(),
        };
        let content = match content {
            Ok(contents) => contents,
            Err(_) => return (None, path),
        };
        let project: Option<Project> = match serde_json::from_str(&content) {
            Ok(project) => project,
            Err(e) => None,
        };

        (project, path)
    }

    pub(crate) fn search_through_files(&mut self) -> Result<()> {
        let filename_to_search = "project.json";
        let mut files = vec![];
        for entry in WalkDir::new("/home/simon/1_Coding") {
            let entry = entry.unwrap();
            if entry.file_name().to_string_lossy() == filename_to_search {
                files.push(entry.path().to_str().unwrap().to_string());
            }
        }

        for file in &files {
            let (project, path) = App::read_from_file(file);
            self.add_to_project_vector(project, path);
        }
        Ok(())
    }

    pub(crate) fn exit(&mut self) -> Result<()> {
        self.exiting = true;
        Ok(())
    }
}
