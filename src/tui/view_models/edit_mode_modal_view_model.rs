use crate::tui::app::App;

pub struct Input {
    pub title: String,
    pub value: String,
    pub character_index: usize,
}

impl Input {
    pub fn new(title: &String, value: &String) -> Self {
        Self {
            title: title.clone(),
            value: value.clone(),
            character_index: value.len(),
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
        let idx = app.visual_order.get(app.selected).unwrap();
        let todo = app.todos.get(*idx).unwrap().clone();

        let inputs = vec![
            Input::new(&"Description".to_string(), &todo.description),
            Input::new(
                &"Priority".to_string(),
                &todo.priority.unwrap_or(99).to_string(),
            ),
            Input::new(&"Due Date".to_string(), &todo.due.unwrap_or(String::new())),
            Input::new(&"Tags".to_string(), &todo.tags.unwrap_or(vec![]).join(", ")),
            Input::new(&"Notes".to_string(), &todo.notes.unwrap_or(String::new())),
        ];

        Self {
            fields: inputs,
            done: todo.done,
            selected_index: 0,
        }
    }
}
