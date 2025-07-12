use crate::storage::{Storage, TodoItem};

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct EditBuffer {
    pub description: String,
    pub priority: String,
    pub due: String,
    pub tags: String,
    pub notes: String,
    pub selected_field: usize,
}

pub struct App {
    pub todos: Vec<TodoItem>,
    pub visual_order: Vec<usize>,
    pub selected: usize,
    pub expanded: Option<usize>,
    pub mode: InputMode,
    pub edit_buffer: Option<EditBuffer>,
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
            mode: InputMode::Normal,
            edit_buffer: None,
        }
    }

    pub fn next(&mut self) {
        match self.mode {
            InputMode::Normal => {
                if self.selected + 1 < self.visual_order.len() {
                    self.selected += 1;
                }
            }
            InputMode::Editing => {
                if let Some(buf) = self.edit_buffer.as_mut() {
                    if buf.selected_field + 1 < 5 {
                        buf.selected_field += 1;
                    }
                }
            }
        }
    }

    pub fn previous(&mut self) {
        match self.mode {
            InputMode::Normal => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            InputMode::Editing => {
                if let Some(buf) = self.edit_buffer.as_mut() {
                    if buf.selected_field > 0 {
                        buf.selected_field -= 1;
                    }
                }
            }
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

    pub fn save(&self, storage: &impl Storage) {
        if let Err(e) = storage.save_items(&self.todos) {
            eprintln!("Failed to save todos: {}", e);
        }
    }

    pub fn toggle_mode(&mut self) {
        if self.mode == InputMode::Normal {
            let idx = self.visual_order[self.selected];
            let todo = &self.todos[idx];

            self.edit_buffer = Some(EditBuffer {
                description: todo.description.clone(),
                priority: todo.priority.map_or(String::new(), |p| p.to_string()),
                due: todo.due.clone().unwrap_or_default(),
                tags: todo.tags.clone().unwrap_or_default().join(", "),
                notes: todo.notes.clone().unwrap_or_default(),
                selected_field: 0,
            });

            self.mode = InputMode::Editing;
        } else {
            self.mode = InputMode::Normal;
            self.commit_edit();
            self.edit_buffer = None;
        }
    }

    fn commit_edit(&mut self) {
        if let Some(buffer) = &self.edit_buffer {
            if let Some(&idx) = self.visual_order.get(self.selected) {
                let todo = &mut self.todos[idx];

                todo.description = buffer.description.clone();
                todo.priority = buffer.priority.trim().parse().ok();
                todo.due = if buffer.due.trim().is_empty() {
                    None
                } else {
                    Some(buffer.due.clone())
                };
                todo.tags = if buffer.tags.trim().is_empty() {
                    None
                } else {
                    Some(
                        buffer
                            .tags
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect(),
                    )
                };
                todo.notes = if buffer.notes.trim().is_empty() {
                    None
                } else {
                    Some(buffer.notes.clone())
                };
            }
        }
    }

    pub fn edit_insert(&mut self, ch: char) {
        if let Some(buf) = self.edit_buffer.as_mut() {
            let field = match buf.selected_field {
                0 => &mut buf.description,
                1 => &mut buf.priority,
                2 => &mut buf.due,
                3 => &mut buf.tags,
                4 => &mut buf.notes,
                _ => return,
            };
            field.push(ch);
        }
    }

    pub fn edit_backspace(&mut self) {
        if let Some(buf) = self.edit_buffer.as_mut() {
            let field = match buf.selected_field {
                0 => &mut buf.description,
                1 => &mut buf.priority,
                2 => &mut buf.due,
                3 => &mut buf.tags,
                4 => &mut buf.notes,
                _ => return,
            };
            field.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MockStorage;
    use mockall::predicate::eq;

    #[test]
    fn next_and_prev_test() {
        let mut app = App::new(vec![make_todo("1"), make_todo("2"), make_todo("3")]);
        assert_eq!(app.selected, 0);
        app.next();
        assert_eq!(app.selected, 1);
        app.previous();
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn toggle_done_test() {
        let mut app = App::new(vec![make_todo("1")]);
        assert_eq!(app.selected, 0);
        assert_eq!(app.todos.len(), 1);

        app.toggle_done();
        let completed_todo = app.todos[0].clone();
        assert_eq!(completed_todo.done, true);

        app.toggle_done();
        let incomplete_todo = app.todos[0].clone();
        assert_eq!(incomplete_todo.done, false);
    }

    #[test]
    fn toggle_expanded_test() {
        let mut app = App::new(vec![make_todo("1")]);
        assert_eq!(app.selected, 0);
        assert_eq!(app.todos.len(), 1);
        assert_eq!(app.expanded, None);

        app.toggle_expanded();
        assert_eq!(app.expanded, Some(0));

        app.toggle_expanded();
        assert_eq!(app.expanded, None);
    }

    #[test]
    fn test_save() {
        let app = App::new(vec![make_todo("1")]);
        assert_eq!(app.selected, 0);

        let mut storage = MockStorage::new();
        storage
            .expect_save_items()
            .with(eq(vec![app.todos[0].clone()]))
            .times(1)
            .returning(|_| Ok(()));

        app.save(&storage)
    }

    fn make_todo(description: &str) -> TodoItem {
        TodoItem {
            description: description.into(),
            priority: None,
            due: None,
            tags: None,
            notes: None,
            done: false,
        }
    }
}
