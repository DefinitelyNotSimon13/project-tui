use ratatui::{
    layout::{Constraint, Layout},
    prelude::Alignment,
    style::{Color, Style},
    symbols::border,
    text::{Line, Span},
    widgets::{block::Title, Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw_ui(frame: &mut Frame, app: &mut App) {
    let vertical_chunks =
        Layout::vertical([Constraint::Min(1), Constraint::Length(4)]).split(frame.size());

    let test = Title::from(String::from(" List "));
    let block = Block::default()
        .title(test.alignment(Alignment::Center))
        .borders(Borders::ALL)
        .border_set(border::THICK)
        .border_type(BorderType::Rounded);

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
    let current_dir = Paragraph::new(text).block(
        Block::new()
            .title(Title::from("Current dir").alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );
    frame.render_widget(current_dir, vertical_chunks[1])
}
