use crate::storage::{Storage, TodoItem};
use crate::tui::state::edit_buffer::EditBuffer;

#[derive(PartialEq, Debug)]
pub enum InputMode {
    Normal,
    Editing,
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
            self.edit_buffer = Some(EditBuffer::new(todo));
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
                buf.update_todo(todo);
                self.recompute_visual_order(idx)
            }
        }
    }

    fn recompute_visual_order(&mut self, edited_idx: usize) {
        // Re-sort
        let mut pairs: Vec<(usize, &TodoItem)> = self.todos.iter().enumerate().collect();
        pairs.sort_by_key(|(_, t)| t.priority.unwrap_or(99));

        self.visual_order = pairs.into_iter().map(|(i, _)| i).collect();

        // Where did the edited tod0 land?
        if let Some(pos) = self.visual_order.iter().position(|&i| i == edited_idx) {
            self.selected = pos;
        } else {
            self.selected = 0; // fallback (shouldn’t happen)
        }
    }

    pub fn remove_selected(&mut self) {
        if let Some(&idx) = self.visual_order.get(self.selected) {
            self.todos.remove(idx);
            self.visual_order.remove(self.selected);
            for i in self.visual_order.iter_mut() {
                if *i > idx {
                    *i -= 1;
                }
            }
            self.selected = self
                .selected
                .saturating_sub(1)
                .clamp(0, self.visual_order.len());
        }
    }
    pub fn promote_selected(&mut self) {
        let idx = self.visual_order[self.selected];
        let new_priority = match self.todos[idx].priority {
            Some(p) if p > 0 => Some(p - 1),
            Some(_) => Some(0), // already zero
            None => self.get_last_non_none_priority(),
        };

        self.todos[idx].priority = new_priority;
        self.recompute_visual_order(idx);
    }

    pub fn demote_selected(&mut self) {
        let idx = self.visual_order[self.selected];
        let new_priority = match self.todos[idx].priority {
            // 99 == None
            Some(p) if p < 98 => Some(p + 1),
            _ => None,
        };

        self.todos[idx].priority = new_priority;
        self.recompute_visual_order(idx);
    }

    pub fn get_last_non_none_priority(&mut self) -> Option<u8> {
        self.visual_order
            .iter()
            .rev()
            .filter_map(|&i| self.todos[i].priority)
            .next()
    }

    pub fn split_current(&mut self) {
        if let Some(idx) = self.visual_order.get(self.selected) {
            if let Some(current) = self.todos.get_mut(*idx) {
                let current_clone = current.clone();
                let new = TodoItem::create_part_two(current_clone);
                current.description += " - part 1";
                self.todos.push(new);
                self.recompute_visual_order(*idx);
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

    #[test]
    fn toggle_mode_enters_and_exits_editing() {
        let mut app = App::new(vec![make_todo("x")]);

        // enter
        app.toggle_mode();
        assert_eq!(app.mode, InputMode::Editing);
        assert!(app.edit_buffer.is_some());
        assert_eq!(app.edit_buffer.as_ref().unwrap().selected_field, 0);

        // exit -> buffer cleared
        app.toggle_mode();
        assert_eq!(app.mode, InputMode::Normal);
        assert!(app.edit_buffer.is_none());
    }

    #[test]
    fn editing_next_previous_change_selected_field_and_reset_cursor() {
        let mut app = App::new(vec![make_todo("desc")]); // description = "desc"
        app.toggle_mode();

        app.left();
        let buf = app.edit_buffer.as_ref().unwrap();
        assert_eq!(buf.selected_field, 0);
        // the cursor should not be at the end anymore
        assert_ne!(buf.fields[0].cursor, buf.fields[0].value.chars().count());

        app.next();
        let buf = app.edit_buffer.as_ref().unwrap();
        assert_eq!(buf.selected_field, 1);
        // the previous field should have had its cursor set back to end
        assert_eq!(buf.fields[0].cursor, buf.fields[0].value.chars().count());
    }

    #[test]
    fn left_and_right_move_cursor_within_field() {
        let mut app = App::new(vec![make_todo("abc")]);
        app.toggle_mode();

        {
            let buf = app.edit_buffer.as_mut().unwrap();
            buf.current_field_mut().move_left(); // cursor -> 2
        }

        app.left(); // cursor -> 1
        assert_eq!(app.edit_buffer.as_ref().unwrap().fields[0].cursor, 1);

        app.right(); // cursor -> 2
        assert_eq!(app.edit_buffer.as_ref().unwrap().fields[0].cursor, 2);
    }

    #[test]
    fn insert_and_backspace_modify_field_and_cursor() {
        let mut app = App::new(vec![make_todo("ac")]);
        app.toggle_mode();

        {
            let buf = app.edit_buffer.as_mut().unwrap();
            buf.current_field_mut().move_left(); // cursor after 'a'
        }

        app.edit_insert('b'); // "abc"
        assert_eq!(app.edit_buffer.as_ref().unwrap().fields[0].value, "abc");
        assert_eq!(app.edit_buffer.as_ref().unwrap().fields[0].cursor, 2);

        app.edit_backspace(); // delete 'b' -> "ac"
        assert_eq!(app.edit_buffer.as_ref().unwrap().fields[0].value, "ac");
        assert_eq!(app.edit_buffer.as_ref().unwrap().fields[0].cursor, 1);
    }

    #[test]
    fn commit_edit_updates_todo_and_resorts_by_priority() {
        // Two todos with priorities None (99) and 1
        let mut app = App::new(vec![
            todo_with("a", None),    // visual idx 1 after sort
            todo_with("b", Some(1)), // visual idx 0
        ]);
        // Select the low-priority tod0 (visual index 1)
        app.selected = 1;

        // Enter edit mode and change its priority to 0
        app.toggle_mode();
        {
            let buf = app.edit_buffer.as_mut().unwrap();
            buf.fields[1].value = "0".into(); // priority field
        }
        app.toggle_mode(); // exits Editing -> commits & resorts

        // After resort, the edited tod0 should now be at visual index 0
        assert_eq!(app.selected, 0);
        assert_eq!(app.todos[0].priority, Some(0));
    }

    #[test]
    fn remove_selected_removes_item_and_updates_visual_order() {
        let mut app = App::new(vec![
            todo_with("a", Some(2)), // idx 0
            todo_with("b", Some(1)), // idx 1
            todo_with("c", Some(3)), // idx 2
        ]);

        // Priority order should be: b (1), a (2), c (3)
        // visual_order: [1, 0, 2]
        assert_eq!(app.visual_order, vec![1, 0, 2]);

        app.selected = 0; // selecting "b" (idx 1 in todos)
        app.remove_selected();

        // "b" should be gone
        assert_eq!(app.todos.len(), 2);
        assert!(app.todos.iter().all(|t| t.description != "b"));

        // visual_order should be rebuilt and reindexed correctly
        assert_eq!(app.visual_order.len(), 2);
        assert_eq!(app.visual_order.iter().all(|&i| i < app.todos.len()), true);

        // selected index should be clamped to 0
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn remove_selected_decrements_indices_after_removed_todo() {
        let mut app = App::new(vec![
            todo_with("a", Some(1)), // idx 0
            todo_with("b", Some(2)), // idx 1
            todo_with("c", Some(3)), // idx 2
        ]);

        // visual_order should be: [0, 1, 2]
        assert_eq!(app.visual_order, vec![0, 1, 2]);

        app.selected = 1; // select "b"
        app.remove_selected();

        // "b" is removed from todos
        assert_eq!(app.todos.len(), 2);
        assert_eq!(
            app.todos.iter().map(|t| &t.description).collect::<Vec<_>>(),
            vec!["a", "c"]
        );

        // visual_order should now reference updated indices: [0, 1]
        assert_eq!(app.visual_order, vec![0, 1]);

        // selected should move to 0
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn remove_selected_from_empty_does_nothing() {
        let mut app = App::new(vec![]);

        app.remove_selected();

        // still empty, no panic
        assert_eq!(app.todos.len(), 0);
        assert_eq!(app.visual_order.len(), 0);
    }

    #[test]
    fn remove_selected_with_last_item_selected() {
        let mut app = App::new(vec![
            todo_with("a", Some(1)), // idx 0
            todo_with("b", Some(2)), // idx 1
        ]);

        // visual_order: [0, 1]
        app.selected = 1;
        app.remove_selected();

        // Should remove "b"
        assert_eq!(app.todos.len(), 1);
        assert_eq!(app.todos[0].description, "a");

        // selected should clamp to 0
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn get_last_non_none_priority_returns_highest_defined_priority() {
        let mut app = App::new(vec![
            todo_with("a", Some(3)),
            todo_with("b", Some(1)),
            todo_with("c", None),
            todo_with("d", Some(5)),
            todo_with("e", None),
        ]);

        // visual_order will sort by priority.unwrap_or(99):
        // Order: b (1), a (3), d (5), c (None), e (None)
        // Reversed: e, c, d, a, b → last non-none = d = 5

        assert_eq!(app.get_last_non_none_priority(), Some(5));
    }

    #[test]
    fn get_last_non_none_priority_skips_all_nones() {
        let mut app = App::new(vec![
            todo_with("a", None),
            todo_with("b", None),
            todo_with("c", None),
        ]);

        assert_eq!(app.get_last_non_none_priority(), None);
    }

    #[test]
    fn get_last_non_none_priority_returns_lowest_if_only_one_defined() {
        let mut app = App::new(vec![
            todo_with("a", None),
            todo_with("b", Some(2)),
            todo_with("c", None),
        ]);

        assert_eq!(app.get_last_non_none_priority(), Some(2));
    }

    #[test]
    fn get_last_non_none_priority_with_empty_list() {
        let mut app = App::new(vec![]);
        assert_eq!(app.get_last_non_none_priority(), None);
    }

    #[test]
    fn promote_selected_decreases_priority() {
        let mut app = App::new(vec![todo_with("a", Some(3)), todo_with("b", Some(1))]);

        app.selected = 1; // "a" with priority 3
        app.promote_selected();
        assert_eq!(app.todos[app.visual_order[1]].priority, Some(2));

        app.promote_selected();
        assert_eq!(app.todos[app.visual_order[1]].priority, Some(1));

        app.promote_selected();
        // at this point it has become the first element in the list
        assert_eq!(app.todos[app.visual_order[0]].priority, Some(0));

        // promoting again at 0 should clamp at 0
        app.promote_selected();
        assert_eq!(app.todos[app.visual_order[0]].priority, Some(0));
    }

    #[test]
    fn promote_selected_sets_priority_from_last_non_none_if_none() {
        let mut app = App::new(vec![todo_with("a", Some(2)), todo_with("b", None)]);

        app.selected = 1; // select "b" which has None
        app.promote_selected();

        // it should take priority from "a" (2)
        assert_eq!(app.todos[app.visual_order[1]].priority, Some(2));
    }

    #[test]
    fn demote_selected_increases_priority() {
        let mut app = App::new(vec![todo_with("a", Some(0)), todo_with("b", Some(98))]);

        app.selected = 0;
        app.demote_selected();
        assert_eq!(app.todos[app.visual_order[0]].priority, Some(1));

        app.selected = 1;
        app.demote_selected();
        assert_eq!(app.todos[app.visual_order[1]].priority, None);

        app.demote_selected();
        // stays None
        assert_eq!(app.todos[app.visual_order[1]].priority, None);
    }

    #[test]
    fn demote_selected_keeps_none_priority() {
        let mut app = App::new(vec![todo_with("a", None)]);

        app.selected = 0;
        app.demote_selected();
        assert_eq!(app.todos[app.visual_order[0]].priority, None);
    }

    fn todo_with(desc: &str, prio: Option<u8>) -> TodoItem {
        TodoItem {
            description: desc.into(),
            priority: prio,
            due: None,
            tags: None,
            notes: None,
            done: false,
        }
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
