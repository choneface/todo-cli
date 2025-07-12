use crate::storage::{Storage, TodoItem};

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Clone)]
pub struct FieldBuffer {
    pub value: String,
    pub cursor: usize,
}

impl FieldBuffer {
    pub fn new(value: String) -> Self {
        let cursor = value.chars().count();
        Self { value, cursor }
    }

    fn byte_index(&self) -> usize {
        self.value
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor)
            .unwrap_or(self.value.len())
    }

    fn clamp(&self, pos: usize) -> usize {
        pos.clamp(0, self.value.chars().count())
    }

    pub fn move_left(&mut self) {
        self.cursor = self.clamp(self.cursor.saturating_sub(1));
    }

    pub fn move_right(&mut self) {
        self.cursor = self.clamp(self.cursor.saturating_add(1));
    }

    pub fn reset_cursor(&mut self) {
        self.cursor = self.value.chars().count();
    }

    pub fn insert_char(&mut self, ch: char) {
        let idx = self.byte_index();
        self.value.insert(idx, ch);
        self.move_right();
    }

    pub fn backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }
        let left = self.cursor - 1;
        let head = self.value.chars().take(left);
        let tail = self.value.chars().skip(self.cursor);
        self.value = head.chain(tail).collect();
        self.move_left();
    }
}

pub struct EditBuffer {
    pub fields: [FieldBuffer; 5], // 0-4: desc, prio, due, tags, notes
    pub selected_field: usize,
}

impl EditBuffer {
    pub fn current_field_mut(&mut self) -> &mut FieldBuffer {
        &mut self.fields[self.selected_field]
    }
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
                        buf.fields[buf.selected_field].reset_cursor();
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
                        buf.fields[buf.selected_field].reset_cursor();
                        buf.selected_field -= 1;
                    }
                }
            }
        }
    }

    pub fn left(&mut self) {
        if let Some(buf) = self.edit_buffer.as_mut() {
            buf.current_field_mut().move_left();
        }
    }

    pub fn right(&mut self) {
        if let Some(buf) = self.edit_buffer.as_mut() {
            buf.current_field_mut().move_right();
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

    pub fn edit_insert(&mut self, ch: char) {
        if let Some(buf) = self.edit_buffer.as_mut() {
            buf.current_field_mut().insert_char(ch);
        }
    }

    pub fn edit_backspace(&mut self) {
        if let Some(buf) = self.edit_buffer.as_mut() {
            buf.current_field_mut().backspace();
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
                fields: [
                    FieldBuffer::new(todo.description.clone()),
                    FieldBuffer::new(todo.priority.map_or(String::new(), |p| p.to_string())),
                    FieldBuffer::new(todo.due.clone().unwrap_or_default()),
                    FieldBuffer::new(todo.tags.clone().unwrap_or_default().join(", ")),
                    FieldBuffer::new(todo.notes.clone().unwrap_or_default()),
                ],
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
        if let Some(buf) = &self.edit_buffer {
            if let Some(&idx) = self.visual_order.get(self.selected) {
                let todo = &mut self.todos[idx];

                // 0 ─ Description ---------------------------------------------------
                todo.description = buf.fields[0].value.clone();

                // 1 ─ Priority (empty ⇒ None) --------------------------------------
                todo.priority = buf.fields[1].value.trim().parse::<u8>().ok();

                // 2 ─ Due date (empty ⇒ None) --------------------------------------
                todo.due = match buf.fields[2].value.trim() {
                    "" => None,
                    s => Some(s.to_string()),
                };

                // 3 ─ Tags (comma-separated, empty ⇒ None) -------------------------
                todo.tags = if buf.fields[3].value.trim().is_empty() {
                    None
                } else {
                    Some(
                        buf.fields[3]
                            .value
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect(),
                    )
                };

                // 4 ─ Notes (empty ⇒ None) -----------------------------------------
                todo.notes = match buf.fields[4].value.trim() {
                    "" => None,
                    s => Some(s.to_string()),
                };
            }
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
