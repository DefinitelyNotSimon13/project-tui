use crate::app::App;
use crossterm::event::{self, Event, KeyCode};

pub(crate) fn handle_event(app: &mut App) {
    let Event::Key(key) = event::read().expect("failed to read key") else {
        return;
    };
    if key.kind == event::KeyEventKind::Release {
        return;
    }
    match key.code {
        KeyCode::Char('f') => {
            let (project, path) = App::read_from_file("project.json");
            app.add_to_project_vector(project, path);
        }
        KeyCode::Char('s') => app.search_through_files(),
        KeyCode::Char('q') => app.exit(),
        KeyCode::Tab => app.switch_view(),
        _ => {}
    }
}
