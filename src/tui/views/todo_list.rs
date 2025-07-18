use crate::storage::TodoItem;
use crate::tui::app::App;
use crate::tui::view_models::todo_view_model::TodoListViewModel;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Line, Modifier, Span, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

pub enum Row<'a> {
    Header(String),
    Todo {
        item: &'a TodoItem,
        is_expanded: bool,
    },
}

pub fn render(f: &mut Frame, app: &App) {
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
        Span::raw("[⌫] Delete    "),
        Span::raw("[p/l] Toggle Priority    "),
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
