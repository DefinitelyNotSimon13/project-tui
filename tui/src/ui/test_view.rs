use ratatui::{
    layout::{Constraint, Layout},
    prelude::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{block::Title, List, ListItem, Paragraph},
    Frame,
};

use color_eyre::Result;

use super::utils;
use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame, area: Rect) -> Result<()> {
    let vertical_chunks = Layout::vertical([Constraint::Min(1), Constraint::Length(4)]).split(area);

    let test = Title::from(String::from(" List "));
    let block = utils::block_with_title(test, None);

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

    let (project, path) = &app.current_dir;
    let project: String = match project {
        Some(project) => project.name.to_owned(),
        None => "None".to_string(),
    };
    let text = vec![Line::from(project), Line::from(path.to_owned())];
    let current_dir =
        Paragraph::new(text).block(utils::block_with_title(Title::from("Current Dir"), None));
    frame.render_widget(current_dir, vertical_chunks[1]);

    Ok(())
}
