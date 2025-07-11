use crate::tui::app::App;
use crate::tui::ui::Row;

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
                index_in_todos: i,
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
