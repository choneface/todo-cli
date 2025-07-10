use crate::storage::TodoItem;
use crate::tui::app::App;
use ratatui::layout::Rect;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

enum Row<'a> {
    Header(String),
    Todo(usize, &'a TodoItem), // (index_in_todos, item)
}

pub fn render(f: &mut Frame, app: &App) {
    // Layout: top = keybindings, bottom = list
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2), // keybindings
            Constraint::Min(0),    // list
        ])
        .split(f.size());

    render_keybindings(f, chunks[0]);
    render_todo_list(f, app, chunks[1]);
}

fn render_todo_list(f: &mut Frame, app: &App, chunk: Rect) {
    let (rows, visual_index_for_selected) = build_rows(&app);
    let mut visual_items = Vec::new();
    for row in rows.iter() {
        match row {
            Row::Header(text) => {
                visual_items.push(build_header(text));
            }
            Row::Todo(i, todo) => {
                visual_items.push(render_todo(todo, app.expanded == Some(i.clone())));
            }
        }
    }

    let mut state = ListState::default();
    state.select(Some(visual_index_for_selected));

    let list = List::new(visual_items)
        .block(Block::default().title("Todos").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_stateful_widget(list, chunk, &mut state);
}

fn build_rows(app: &App) -> (Vec<Row>, usize) {
    let mut rows: Vec<Row> = Vec::new();
    let mut last_priority: Option<u8> = None;
    let mut visual_index_for_selected = 0;
    for &i in &app.visual_order {
        let todo = &app.todos[i];
        let priority = todo.priority.unwrap_or(99);

        if Some(priority) != last_priority {
            rows.push(Row::Header(format!(
                "Priority {}",
                if priority == 99 {
                    "None".to_string()
                } else {
                    priority.to_string()
                }
            )));
            last_priority = Some(priority);
        }

        rows.push(Row::Todo(i, todo));
        if Some(i) == app.visual_order.get(app.selected).copied() {
            visual_index_for_selected = rows.len() - 1;
        }
    }
    (rows, visual_index_for_selected)
}

fn render_keybindings(f: &mut Frame, rect: Rect) {
    let header = Paragraph::new(Line::from(vec![
        Span::raw("[↑/↓] Move    "),
        Span::raw("[⏎] Toggle Done    "),
        Span::raw("[Space] Expand    "),
        Span::raw("[q] Quit"),
    ]))
    .block(Block::default());

    f.render_widget(header, rect);
}

fn build_header(text: &str) -> ListItem {
    ListItem::new(text)
}

fn render_todo(todo: &TodoItem, is_expanded: bool) -> ListItem<'static> {
    let checkbox = if todo.done { "[x]" } else { "[ ]" };
    let mut lines = vec![format!(" -  {} {}", checkbox, todo.description)];

    if is_expanded {
        if let Some(p) = todo.priority {
            lines.push(format!("   Priority: {}", p));
        }
        if let Some(due) = &todo.due {
            lines.push(format!("   Due: {}", due));
        }
        if let Some(tags) = &todo.tags {
            lines.push(format!("   Tags: {:?}", tags));
        }
        if let Some(notes) = &todo.notes {
            lines.push(format!("   Notes: {}", notes));
        }
    }
    ListItem::new(lines.join("\n"))
}
