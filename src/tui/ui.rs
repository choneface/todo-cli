use ratatui::{
    backend::Backend,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};


use crate::tui::app::App;

pub fn render(f: &mut Frame, app: &App) {
    let items: Vec<ListItem> = app
    .todos
    .iter()
    .enumerate()
    .map(|(i, todo)| {
        let checkbox = if todo.done { "[x]" } else { "[ ]" };
        let mut lines = vec![format!("{}. {} {}", i + 1, checkbox, todo.description)];

        if app.expanded == Some(i) {
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

        ListItem::new(lines.join("\n"))
    })
    .collect();

    let mut state = ListState::default();
    state.select(Some(app.selected));

    let list = List::new(items)
        .block(Block::default().title("Todos").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_stateful_widget(list, f.size(), &mut state);
}
