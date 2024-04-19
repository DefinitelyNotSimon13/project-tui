mod app;
mod errors;
mod events;
mod project;
mod tui;
mod ui;
use app::App;
use color_eyre::{eyre::bail, Result};

use flexi_logger::{FileSpec, Logger, WriteMode};
use log::trace;
use tui::Tui;

fn main() -> Result<()> {
    let _logger = Logger::try_with_str("trace")?
        .log_to_file(FileSpec::default().directory("logs").basename("log"))
        .write_mode(WriteMode::BufferAndFlush)
        .create_symlink("current.log")
        .start()?;
    trace!("Starting up");
    errors::install_hooks()?;
    trace!("Error hooks installed");
    let mut terminal = tui::init()?;
    trace!("Terminal initialized");
    terminal.clear()?;
    let mut app = App::new();
    trace!("Starting event loop");
    run_app(&mut terminal, &mut app)?;
    trace!("Event loop ended, restoring terminal");
    tui::restore()?;
    trace!("Terminal restored, exiting...");
    Ok(())
}

fn run_app(terminal: &mut Tui, app: &mut App) -> Result<()> {
    loop {
        if let Err(e) = terminal.draw(|frame| ui::draw_ui(frame, app).expect("failed to render ui"))
        {
            bail!("failed to draw UI: {}", e);
        }
        if let Err(e) = events::handle_event(app) {
            bail!("failed to handle event: {}", e);
        }
        if app.exiting {
            break;
        }
    }
    Ok(())
}
