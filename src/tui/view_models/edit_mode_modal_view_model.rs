use crate::tui::app::App;

struct Input {
    input: String,
    character_index: usize,
}

impl Input {
    pub fn new(string: &String) -> Self {
        Self {
            input: string.clone(),
            character_index: string.len() - 1,
        }
    }
}

pub struct EditModeModalViewModel {
    fields: Vec<Input>,
    done: bool,
}

impl EditModeModalViewModel {
    pub fn from_app(app: &App) -> Self {
        let todo = app.todos.get(app.selected).unwrap().clone();

        let inputs = vec![
            Input::new(&todo.description),
            Input::new(&todo.priority.unwrap_or(99).to_string()),
            Input::new(&todo.due.unwrap_or(String::new())),
            Input::new(&todo.tags.unwrap_or(vec![]).join(", ")),
            Input::new(&todo.notes.unwrap_or(String::new())),
        ];

        Self {
            fields: inputs,
            done: todo.done,
        }
    }
}
