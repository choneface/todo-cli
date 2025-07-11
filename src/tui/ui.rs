use crate::storage::TodoItem;
use crate::tui::app::App;
use crate::tui::app::InputMode::Editing;
use crate::tui::view_model::TodoListViewModel;
use ratatui::layout::{Flex, Rect};
use ratatui::widgets::Clear;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::os::unix::raw::pid_t;

pub enum Row<'a> {
    Header(String),
    Todo {
        index_in_todos: usize,
        item: &'a TodoItem,
        is_expanded: bool,
    },
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

    if app.mode == Editing {
        let block = Block::bordered().title("Editing").borders(Borders::ALL);
        let area = popup_area(f.size(), 60, 20);
        f.render_widget(Clear, area);
        f.render_widget(block, area);
    }
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

fn render_todo_list(f: &mut Frame, app: &App, chunk: Rect) {
    let view_model = TodoListViewModel::from_app(app);
    let items: Vec<ListItem> = view_model.rows.iter().map(render_row).collect();

    let mut state = ListState::default();
    state.select(view_model.selected_index);

    let list = List::new(items)
        .block(Block::default().title("Todos").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_stateful_widget(list, chunk, &mut state);
}

fn render_keybindings(f: &mut Frame, rect: Rect) {
    let header = Paragraph::new(Line::from(vec![
        Span::raw("[↑/↓] Move    "),
        Span::raw("[⏎] Toggle Done    "),
        Span::raw("[Space] Expand    "),
        Span::raw("[e] Edit    "),
        Span::raw("[q] Quit"),
    ]))
    .block(Block::default());

    f.render_widget(header, rect);
}

fn render_row<'a>(row: &Row) -> ListItem<'a> {
    match row {
        Row::Header(text) => ListItem::new(text.clone()),
        Row::Todo {
            item, is_expanded, ..
        } => {
            let checkbox = if item.done { "[x]" } else { "[ ]" };
            let mut lines = vec![format!(" -  {} {}", checkbox, item.description)];

            if *is_expanded {
                if let Some(p) = item.priority {
                    lines.push(format!("   Priority: {}", p));
                }
                if let Some(due) = &item.due {
                    lines.push(format!("   Due: {}", due));
                }
                if let Some(tags) = &item.tags {
                    lines.push(format!("   Tags: {:?}", tags));
                }
                if let Some(notes) = &item.notes {
                    lines.push(format!("   Notes: {}", notes));
                }
            }

            ListItem::new(lines.join("\n"))
        }
    }
}
