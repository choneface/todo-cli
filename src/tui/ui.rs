use crate::storage::TodoItem;
use crate::tui::app::App;
use crate::tui::app::InputMode::Editing;
use crate::tui::uis::{edit_modal, todo_list};
use ratatui::Frame;

pub enum Row<'a> {
    Header(String),
    Todo {
        item: &'a TodoItem,
        is_expanded: bool,
    },
}

pub fn render(f: &mut Frame, app: &App) {
    todo_list::render(f, app);
    if app.mode == Editing {
        edit_modal::render(f, app)
    }
}
