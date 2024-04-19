use ratatui::{
    prelude::{Alignment, Constraint, Direction, Layout, Rect},
    symbols::border,
    widgets::{block::Title, Block, BorderType, Borders},
};

// helper function to create centered rect
#[allow(dead_code)]
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn block_with_title(title: Title, alignment: Option<Alignment>) -> Block {
    let alignment = match alignment {
        Some(alignment) => alignment,
        None => Alignment::Center,
    };
    Block::default()
        .title(title.alignment(alignment))
        .borders(Borders::ALL)
        .border_set(border::THICK)
        .border_type(BorderType::Rounded)
}
