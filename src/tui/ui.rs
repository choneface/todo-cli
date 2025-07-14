use crate::tui::app::App;
use crate::tui::app::InputMode::Editing;
use crate::tui::views::{edit_modal, todo_list};
use ratatui::Frame;

pub fn render(f: &mut Frame, app: &App) {
    todo_list::render(f, app);
    if app.mode == Editing {
        edit_modal::render(f, app)
    }
}
