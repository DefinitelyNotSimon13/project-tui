use async_std::future::Future;
use async_std::task;
use color_eyre::Result;
use ratatui::{
    layout::{Constraint, Layout},
    prelude::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{block::Title, List, ListItem, Paragraph},
    Frame,
};
use reqwest::Response;
use serde::Deserialize;

use super::utils;
use crate::app::App;

#[derive(Deserialize, Debug)]
struct TestAnswer {
    count: i32,
    projects: Vec<TestProject>,
}

#[derive(Deserialize, Debug)]
struct TestProject {
    id: String,
    name: String,
    description: String,
    major_version: i32,
    minor_version: i32,
    patch_version: i32,
    github_repo: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) {
    let vertical_chunks = Layout::vertical([
        Constraint::Min(1),
        Constraint::Min(1),
        Constraint::Length(4),
    ])
    .split(area);

    let test_title = Title::from(String::from(" List "));
    let block = utils::block_with_title(test_title, None);

    let mut list_items = Vec::<ListItem>::new();

    for project in &app.projects {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!(
                "{}: {} - {}",
                project.0.name, project.0.description, project.1
            ),
            Style::default().fg(Color::Green),
        ))));
    }
    let list = List::new(list_items).block(block);

    frame.render_widget(list, vertical_chunks[0]);

    let request_url = "http://127.0.0.1:8000/api/projects";
    let response: TestAnswer = reqwest::blocking::get(request_url).unwrap().json().unwrap();

    let url = vec![
        Line::from(format!("Count: {}", response.count)),
        Line::from(format!("Id 1: {}", response.projects[0].id)),
        Line::from(format!("Name 1: {}", response.projects[0].name)),
        Line::from(format!(
            "Description 1: {}",
            response.projects[0].description
        )),
    ];

    let url = Paragraph::new(url).block(utils::block_with_title(Title::from("Request URL"), None));
    frame.render_widget(url, vertical_chunks[1]);

    let (project, path) = &app.current_dir;
    let project: String = match project {
        Some(project) => project.name.clone(),
        None => "None".to_string(),
    };
    let text = vec![Line::from(project), Line::from(path.to_owned())];
    let current_dir =
        Paragraph::new(text).block(utils::block_with_title(Title::from("Current Dir"), None));
    frame.render_widget(current_dir, vertical_chunks[2]);
}
