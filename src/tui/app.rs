use crate::storage::{save_items, TodoItem};

pub struct App {
    pub todos: Vec<TodoItem>,
    pub selected: usize,
    pub expanded: Option<usize>,
}

impl App {
    pub fn new(mut todos: Vec<TodoItem>) -> Self {
        todos.sort_by_key(|t| t.done);
        Self {
            todos,
            selected: 0,
            expanded: None,
        }
    }

    pub fn next(&mut self) {
        if self.selected + 1 < self.todos.len() {
            self.selected += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn toggle_done(&mut self) {
        if let Some(todo) = self.todos.get_mut(self.selected) {
            todo.done = !todo.done;
        }
    }

    pub fn save(&self) {
        if let Err(e) = save_items(&self.todos) {
            eprintln!("Failed to save todos: {}", e);
        }
    }

    pub fn toggle_expanded(&mut self) {
        if self.expanded == Some(self.selected) {
            self.expanded = None;
        } else {
            self.expanded = Some(self.selected);
        }
    }
}
