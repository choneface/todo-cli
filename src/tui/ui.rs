use crate::storage::TodoItem;
use crate::tui::app::App;
use crate::tui::app::InputMode::Editing;
use crate::tui::view_models::edit_mode_modal_view_model::{EditModeModalViewModel, Input};
use crate::tui::view_models::todo_view_model::TodoListViewModel;
use ratatui::layout::{Alignment, Flex, Margin, Rect};
use ratatui::prelude::Color;
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
        let outer_block = Block::bordered().borders(Borders::ALL);
        let outer_area = popup_area(f.size(), 60, 50);
        f.render_widget(Clear, outer_area);
        f.render_widget(outer_block, outer_area);

        // Inset area to avoid overlapping with the border and title
        let inner_area = outer_area.inner(&Margin {
            vertical: 1,
            horizontal: 1,
        });

        // Layout inside the inset area
        let inner_chunks = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(8),
            Constraint::Length(1),
            Constraint::Length(2),
        ])
        .split(inner_area);

        let header = Paragraph::new(Line::from(vec![
            Span::raw("[↑/↓] Move field    "),
            Span::raw("[←/→] Move cursor    "),
            Span::raw("[esc] Save & exit    "),
            Span::raw("[⏎] Toggle Done    "),
        ]))
        .block(Block::default());
        f.render_widget(header, inner_chunks[0]);

        let view_model = EditModeModalViewModel::from_app(&app);
        let fields: Vec<Paragraph> = view_model.fields.iter().map(render_field).collect();
        for (i, field) in fields.iter().enumerate() {
            f.render_widget(field, inner_chunks[i + 1])
        }

        let status_span = if view_model.done {
            Span::styled(
                "Done",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled(
                "Not done",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )
        };

        let status = Paragraph::new(Line::from(vec![status_span])).alignment(Alignment::Center);
        f.render_widget(status, inner_chunks[7]);

        let selected_input = view_model.fields.get(view_model.selected_index).unwrap();
        let x = inner_area.x + selected_input.character_index as u16 + 1;
        let y = inner_area.y + 3 + (3 * view_model.selected_index as u16);
        f.set_cursor(x, y)
    }
}

fn render_field<'a>(input: &Input) -> Paragraph<'a> {
    Paragraph::new(input.value.clone())
        .block(Block::bordered().title(input.title.clone()))
        .style(match input.selected {
            true => Style::default().fg(Color::Yellow),
            false => Style::default().fg(Color::White),
        })
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
