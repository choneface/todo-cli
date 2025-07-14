use crate::tui::app::App;
use crate::tui::views::todo_list::Row;

pub struct TodoListViewModel<'a> {
    pub rows: Vec<Row<'a>>,
    pub selected_index: Option<usize>,
}

impl<'a> TodoListViewModel<'a> {
    pub fn from_app(app: &'a App) -> Self {
        let mut rows = Vec::new();
        let mut last_priority: Option<u8> = None;
        let mut selected_index = None;

        for &i in &app.visual_order {
            let todo = &app.todos[i];
            let priority = todo.priority.unwrap_or(99);

            if Some(priority) != last_priority {
                rows.push(Row::Header(match priority {
                    99 => "Priority None".to_string(),
                    p => format!("Priority {}", p),
                }));
                last_priority = Some(priority);
            }

            let is_expanded = app.expanded == Some(i);
            if Some(i) == app.visual_order.get(app.selected).copied() {
                selected_index = Some(rows.len());
            }

            rows.push(Row::Todo {
                item: todo,
                is_expanded,
            });
        }

        Self {
            rows,
            selected_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::TodoItem;
    use crate::tui::app::{App, InputMode};

    fn make_todo(
        description: &str,
        priority: Option<u8>,
        done: bool,
        expanded: bool,
    ) -> (TodoItem, bool) {
        (
            TodoItem {
                description: description.to_string(),
                priority,
                due: None,
                tags: None,
                done,
                notes: None,
            },
            expanded,
        )
    }

    #[test]
    fn builds_rows_with_headers_and_selection_correctly() {
        let todos = vec![
            make_todo("first", Some(1), false, false),
            make_todo("second", Some(1), true, true),
            make_todo("third", Some(2), false, false),
            make_todo("no-priority", None, false, false),
        ]
        .into_iter()
        .map(|(todo, _)| todo)
        .collect::<Vec<_>>();

        let app = App {
            todos,
            visual_order: vec![0, 1, 2, 3],
            selected: 1, // select the second tod0
            expanded: Some(1),
            mode: InputMode::Normal,
            edit_buffer: None,
        };

        let vm = TodoListViewModel::from_app(&app);

        let headers = vm
            .rows
            .iter()
            .filter_map(|row| {
                if let Row::Header(h) = row {
                    Some(h.as_str())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        assert_eq!(headers, vec!["Priority 1", "Priority 2", "Priority None"]);
        assert_eq!(vm.rows.len(), 7); // 3 headers and 4 todos
        assert_eq!(vm.selected_index, Some(2)); // header, 1st tod0, second tod0
    }

    #[test]
    fn marks_expanded_flag_correctly() {
        let todos = vec![
            make_todo("one", Some(1), false, false),
            make_todo("two", Some(2), false, true),
        ]
        .into_iter()
        .map(|(todo, _)| todo)
        .collect::<Vec<_>>();

        let app = App {
            todos,
            visual_order: vec![0, 1],
            selected: 0,
            expanded: Some(1),
            mode: InputMode::Normal,
            edit_buffer: None,
        };

        let vm = TodoListViewModel::from_app(&app);

        // get a list of is_expanded
        let flags = vm
            .rows
            .iter()
            .filter_map(|row| {
                if let Row::Todo { is_expanded, .. } = row {
                    Some(*is_expanded)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // only the second one should be expanded
        assert_eq!(flags, vec![false, true]);
    }

    #[test]
    fn handles_empty_state() {
        let app = App {
            todos: vec![],
            visual_order: vec![],
            selected: 0,
            expanded: None,
            mode: InputMode::Normal,
            edit_buffer: None,
        };

        let vm = TodoListViewModel::from_app(&app);

        assert!(vm.rows.is_empty());
        assert_eq!(vm.selected_index, None);
    }
}
