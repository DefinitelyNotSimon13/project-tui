use crate::app::App;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};

pub(crate) fn handle_event(app: &mut App) -> Result<()> {
    let key = match event::read()? {
        Event::Key(key) => key,
        _ => return Ok(()),
    };
    if key.kind == event::KeyEventKind::Release {
        return Ok(());
    }
    match key.code {
        KeyCode::Char('f') => app.read_from_file("project.json"),
        KeyCode::Char('s') => app.search_through_files(),
        KeyCode::Char('q') => app.exit(),
        _ => Ok(()),
    }
}
