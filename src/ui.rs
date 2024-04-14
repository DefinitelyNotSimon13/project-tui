use ratatui::{
    prelude::Alignment,
    style::{Color, Style, Stylize},
    symbols::{self, border},
    text::{Line, Span},
    widgets::{block::Title, Block, BorderType, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};

use crate::app::App;

pub fn draw_ui(frame: &mut Frame, app: &mut App) {
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

    frame.render_widget(list, frame.size());
}
