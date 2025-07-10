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

enum ListViewItem<'a> {
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

    // Draw the keybindings
    render_keybindings(f, chunks[0]);

    // Now build the list of todos
    let mut list_view_items: Vec<ListViewItem> = Vec::new();

    let mut last_priority: Option<u8> = None;
    for &i in &app.visual_order {
        let todo = &app.todos[i];
        let priority = todo.priority.unwrap_or(99);

        if Some(priority) != last_priority {
            list_view_items.push(ListViewItem::Header(format!(
                "Priority {}",
                if priority == 99 {
                    "None".to_string()
                } else {
                    priority.to_string()
                }
            )));
            last_priority = Some(priority);
        }

        list_view_items.push(ListViewItem::Todo(i, todo));
    }

    let mut visual_items = Vec::new();
    let mut visual_index_for_selected = 0;

    for row in list_view_items.iter() {
        match row {
            ListViewItem::Header(text) => {
                visual_items.push(build_header(text));
            }
            ListViewItem::Todo(i, todo) => {
                visual_items.push(build_todo(i, todo, app));

                // sort of ugly but can't think of a way to abstract this out
                // at 11:38 at night, sorry.
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

    f.render_stateful_widget(list, chunks[1], &mut state);
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

fn build_todo<'a>(i: &usize, todo: &&TodoItem, app: &App) -> ListItem<'a> {
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
        if let Some(notes) = &todo.notes {
            lines.push(format!("   Notes: {}", notes));
        }
    }
    ListItem::new(lines.join("\n"))
}
