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
            ListItem::new(format!("{}. {} {}", i + 1, checkbox, todo.description))
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.selected));

    let list = List::new(items)
        .block(Block::default().title("Todos").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_stateful_widget(list, f.size(), &mut state);
}
