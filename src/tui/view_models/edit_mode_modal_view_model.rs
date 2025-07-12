use crate::tui::app::App;

/// One line/paragraph shown in the edit modal.
pub struct Input {
    pub title: String,
    pub value: String,
    pub character_index: usize,
    pub selected: bool,
}

impl Input {
    pub fn new(title: &str, value: &str, selected: bool) -> Self {
        Self {
            title: title.to_string(),
            value: value.to_string(),
            character_index: value.chars().count(),
            selected,
        }
    }
}

pub struct EditModeModalViewModel {
    pub fields: Vec<Input>,
    pub done: bool,
    pub selected_index: usize,
}

impl EditModeModalViewModel {
    pub fn from_app(app: &App) -> Self {
        let buf = app.edit_buffer.as_ref().unwrap();

        let inputs = vec![
            Input::new("Description", &buf.description, buf.selected_field == 0),
            Input::new("Priority", &buf.priority, buf.selected_field == 1),
            Input::new("Due Date", &buf.due, buf.selected_field == 2),
            Input::new("Tags", &buf.tags, buf.selected_field == 3),
            Input::new("Notes", &buf.notes, buf.selected_field == 4),
        ];

        let todo_idx = app.visual_order[app.selected];
        let done = app.todos[todo_idx].done;

        Self {
            fields: inputs,
            done,
            selected_index: buf.selected_field,
        }
    }
}
