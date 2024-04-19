use crate::app::App;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};

pub(crate) fn handle_event(app: &mut App) -> Result<()> {
    let key = match event::read()? {
        Event::Key(key) => key,
        _ => return Ok(()),
    };
    if key.kind == event::KeyEventKind::Release {
        return Ok(());
    }
    match key.code {
        KeyCode::Char('f') => {
            let (project, path) = App::read_from_file("project.json");
            app.add_to_project_vector(project, path)
        }
        KeyCode::Char('s') => app.search_through_files(),
        KeyCode::Char('q') => app.exit(),
        KeyCode::Tab => app.switch_view(),
        _ => Ok(()),
    }
}
