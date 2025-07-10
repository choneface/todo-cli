use crate::storage::{Storage, TodoItem};

pub struct App {
    pub todos: Vec<TodoItem>,
    pub visual_order: Vec<usize>,
    pub selected: usize,
    pub expanded: Option<usize>,
}

impl App {
    pub fn new(todos: Vec<TodoItem>) -> Self {
        let mut priority_sorted = todos
            .iter()
            .enumerate()
            .collect::<Vec<(usize, &TodoItem)>>();
    
        priority_sorted.sort_by_key(|(_, t)| t.priority.unwrap_or(99));
        let visual_order = priority_sorted.into_iter().map(|(i, _)| i).collect();
    
        Self {
            todos,
            visual_order,
            selected: 0,
            expanded: None,
        }
    }

    pub fn next(&mut self) {
        if self.selected + 1 < self.visual_order.len() {
            self.selected += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn toggle_done(&mut self) {
        if let Some(&actual_index) = self.visual_order.get(self.selected) {
            self.todos[actual_index].done = !self.todos[actual_index].done;
        }
    }

    pub fn toggle_expanded(&mut self) {
        if let Some(&actual_index) = self.visual_order.get(self.selected) {
            if self.expanded == Some(actual_index) {
                self.expanded = None;
            } else {
                self.expanded = Some(actual_index);
            }
        }
    }

    pub fn save(&self) {
        let storage = Storage::new("todo.json");
        if let Err(e) = storage.save_items(&self.todos) {
            eprintln!("Failed to save todos: {}", e);
        }
    }
}
