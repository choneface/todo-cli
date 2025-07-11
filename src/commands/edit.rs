use std::io;
use std::time::Duration;

use crate::storage::Storage;
use crate::tui::{app::App, events::poll_input, ui::render};

use crate::tui::app::InputMode;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

pub fn run(storage: impl Storage) {
    if let Err(e) = launch_ui(storage) {
        eprintln!("Error: {}", e);
    }
}

fn launch_ui(storage: impl Storage) -> Result<(), Box<dyn std::error::Error>> {
    let todos = storage.load_items()?;
    let mut app = App::new(todos);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| render(f, &app))?;

        match poll_input(Duration::from_millis(200))? {
            crate::tui::events::InputEvent::Quit => break,
            crate::tui::events::InputEvent::Down => app.next(),
            crate::tui::events::InputEvent::Up => app.previous(),
            crate::tui::events::InputEvent::ToggleDone => {
                app.toggle_done();
                app.save(&storage);
            }
            crate::tui::events::InputEvent::ToggleExpand => {
                app.toggle_expanded();
            }
            crate::tui::events::InputEvent::EnableEditing => app.mode = InputMode::Editing,
            crate::tui::events::InputEvent::DisableEditing => app.mode = InputMode::Normal,
            _ => {}
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
