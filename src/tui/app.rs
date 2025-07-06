use crate::storage::TodoItem;

pub struct App {
    pub todos: Vec<TodoItem>,
    pub selected: usize,
}

impl App {
    pub fn new(mut todos: Vec<TodoItem>) -> Self {
        todos.sort_by_key(|t| t.done);
        Self {
            todos,
            selected: 0,
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
}
