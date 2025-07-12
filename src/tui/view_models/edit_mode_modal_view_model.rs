use crate::tui::app::App;
use crate::tui::state::field_buffer::FieldBuffer;

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
        let buf = app.edit_buffer.as_ref().expect("missing buffer");
        let to_input = |title: &str, fb: &FieldBuffer, idx: usize| Input {
            title: title.to_string(),
            value: fb.value.clone(),
            character_index: fb.cursor,
            selected: idx == buf.selected_field,
        };

        Self {
            fields: vec![
                to_input("Description", &buf.fields[0], 0),
                to_input("Priority", &buf.fields[1], 1),
                to_input("Due Date", &buf.fields[2], 2),
                to_input("Tags", &buf.fields[3], 3),
                to_input("Notes", &buf.fields[4], 4),
            ],
            done: app.todos[app.visual_order[app.selected]].done,
            selected_index: buf.selected_field,
        }
    }
}
