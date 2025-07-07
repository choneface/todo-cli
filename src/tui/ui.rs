use ratatui::{
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::tui::app::App;

enum Row<'a> {
    Header(String),
    Todo(usize, &'a crate::storage::TodoItem), // (index_in_todos, item)
}

pub fn render(f: &mut Frame, app: &App) {
    let mut rows: Vec<Row> = Vec::new();

    // Use the visual_order to determine how we display items
    let mut last_priority: Option<u8> = None;
    for &i in &app.visual_order {
        let todo = &app.todos[i];
        let priority = todo.priority.unwrap_or(99);

        if Some(priority) != last_priority {
            rows.push(Row::Header(format!(
                "Priority {}",
                if priority == 99 { "None".to_string() } else { priority.to_string() }
            )));
            last_priority = Some(priority);
        }

        rows.push(Row::Todo(i, todo));
    }

    let mut visual_items = Vec::new();
    let mut visual_index_for_selected = 0;

    for row in rows.iter() {
        match row {
            Row::Header(text) => {
                visual_items.push(ListItem::new(text.clone()));
            }
            Row::Todo(i, todo) => {
                let checkbox = if todo.done { "[x]" } else { "[ ]" };
                let mut lines = vec![format!(" -  {} {}", checkbox, todo.description)];

                if app.expanded == Some(*i) {
                    if let Some(p) = todo.priority {
                        lines.push(format!("   Priority: {}", p));
                    }
                    if let Some(due) = &todo.due {
                        lines.push(format!("   Due: {}", due));
                    }
                    if let Some(tags) = &todo.tags {
                        lines.push(format!("   Tags: {:?}", tags));
                    }
                }

                visual_items.push(ListItem::new(lines.join("\n")));

                if Some(*i) == app.visual_order.get(app.selected).copied() {
                    visual_index_for_selected = visual_items.len() - 1;
                }
            }
        }
    }

    let mut state = ListState::default();
    state.select(Some(visual_index_for_selected));

    let list = List::new(visual_items)
        .block(Block::default().title("Todos").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_stateful_widget(list, f.size(), &mut state);
}
