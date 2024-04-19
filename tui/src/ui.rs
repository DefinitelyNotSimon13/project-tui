mod details_view;
mod edit_view;
mod test_view;
mod utils;

use ratatui::{
    prelude::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{block::Title, Paragraph},
    Frame,
};

use crate::app::{App, View};

pub fn render(frame: &mut Frame, app: &mut App) {
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(2),
            Constraint::Length(5),
        ])
        .split(frame.size());

    // Header
    let block = utils::block_with_title(Title::from(""), None);
    let header = Paragraph::new(Span::styled(
        "A TUI Project Manager",
        Style::default().fg(Color::Green).bold(),
    ))
    .block(block.clone());
    frame.render_widget(header, vertical_layout[0]);

    // Main View
    match &app.current_view {
        View::Test => test_view::render(app, frame, vertical_layout[1]),
        View::Edit => edit_view::render(app, frame, vertical_layout[1]),
        View::Details => details_view::render(app, frame, vertical_layout[1]),
    };
    let text = vec![
        Line::from(Span::styled(
            "Keyhints TBD",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(Span::styled("Simon Blum", Style::default().italic())),
    ];
    let footer = Paragraph::new(text).block(block);
    frame.render_widget(footer, vertical_layout[2]);
}
