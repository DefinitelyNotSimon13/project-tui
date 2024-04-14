use crate::project::Project;
use color_eyre::eyre::bail;
use color_eyre::Result;
use std::fs;
use walkdir::WalkDir;

pub struct App {
    pub projects: Vec<(Project, String)>,
    pub test_contents: String,
    pub exiting: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            projects: vec![],
            test_contents: "".to_string(),
            exiting: false,
        }
    }

    pub fn read_from_file(&mut self, file: &str) -> Result<()> {
        let content = fs::read_to_string(file);
        let content = match content {
            Ok(contents) => contents,
            Err(_) => bail!("error reading file"),
        };
        let project: Project = match serde_json::from_str(&content) {
            Ok(project) => project,
            Err(e) => Project::error(e.to_string()),
        };

        let path = fs::canonicalize(file)?.to_string_lossy().into_owned();
        self.projects.push((project, path));

        Ok(())
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
            self.read_from_file(file)?;
        }
        Ok(())
    }

    pub(crate) fn exit(&mut self) -> Result<()> {
        self.exiting = true;
        Ok(())
    }
}
