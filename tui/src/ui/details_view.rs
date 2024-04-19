use color_eyre::Result;
use ratatui::{
    prelude::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{block::Title, Paragraph},
    Frame,
};

use super::utils;
use crate::{app::App, project::Project};

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) -> Result<()> {
    let block = utils::block_with_title(Title::from("Details View"), None);

    let (project, _) = &app.current_dir;
    let project = match project {
        Some(project) => project.clone(),
        None => Project::error("Not in project directory"),
    };
    let details = vec![
        Line::from(vec![
            Span::styled("Name: ", Style::default()),
            Span::styled(project.name.clone(), Style::default()),
        ]),
        Line::from(vec![
            Span::styled("Description: ", Style::default()),
            Span::styled(project.description.clone(), Style::default()),
        ]),
        Line::from(vec![
            Span::styled("Version: ", Style::default()),
            Span::styled(project.version_string(), Style::default()),
        ]),
        Line::from(vec![
            Span::styled("", Style::default()),
            Span::styled("", Style::default()),
        ]),
    ];
    let paragraph = Paragraph::new(details).block(block);
    frame.render_widget(paragraph, area);

    Ok(())
}
