use crate::tui::app::App;

pub struct Input {
    pub title: String,
    pub value: String,
    pub character_index: usize,
    pub selected: bool,
}

impl Input {
    pub fn new(title: &String, value: &String, selected: bool) -> Self {
        Self {
            title: title.clone(),
            value: value.clone(),
            character_index: value.len(),
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
        let idx = app.visual_order.get(app.selected).unwrap();
        let todo = app.todos.get(*idx).unwrap().clone();

        let inputs = vec![
            Input::new(
                &"Description".to_string(),
                &todo.description,
                app.selected_edit_field == 0,
            ),
            Input::new(
                &"Priority".to_string(),
                &todo.priority.unwrap_or(99).to_string(),
                app.selected_edit_field == 1,
            ),
            Input::new(
                &"Due Date".to_string(),
                &todo.due.unwrap_or(String::new()),
                app.selected_edit_field == 2,
            ),
            Input::new(
                &"Tags".to_string(),
                &todo.tags.unwrap_or(vec![]).join(", "),
                app.selected_edit_field == 3,
            ),
            Input::new(
                &"Notes".to_string(),
                &todo.notes.unwrap_or(String::new()),
                app.selected_edit_field == 4,
            ),
        ];

        Self {
            fields: inputs,
            done: todo.done,
            selected_index: app.selected_edit_field,
        }
    }
}
