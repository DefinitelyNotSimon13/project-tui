use ratatui::{
    prelude::Rect,
    widgets::{block::Title, Paragraph},
    Frame,
};

use super::utils;
use crate::app::App;

pub fn render(_app: &mut App, frame: &mut Frame, area: Rect) {
    let temp_box = utils::block_with_title(Title::from("Edit View"), None);
    let paragraph = Paragraph::default().block(temp_box);

    frame.render_widget(paragraph, area);
}
