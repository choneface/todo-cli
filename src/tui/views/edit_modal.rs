use crate::tui::app::App;
use crate::tui::view_models::edit_mode_modal_view_model::{EditModeModalViewModel, Input};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Flex, Layout, Margin, Rect};
use ratatui::prelude::{Color, Line, Modifier, Span, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};

pub fn render(f: &mut Frame, app: &App) {
    let outer_block = Block::bordered().borders(Borders::ALL);
    let outer_area = popup_area(f.size(), 60, 50);
    f.render_widget(Clear, outer_area);
    f.render_widget(outer_block, outer_area);

    let inner_area = outer_area.inner(&Margin {
        vertical: 1,
        horizontal: 1,
    });

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

    let view_model = EditModeModalViewModel::from_app(&app);
    render_edit_header(f, inner_chunks[0]);
    render_edit_fields(f, inner_chunks[1..6].to_vec(), &view_model);
    render_status_span(f, inner_chunks[7], view_model.done);
    render_cursor(f, inner_area, &view_model)
}

fn render_edit_header(f: &mut Frame, area: Rect) {
    let header = Paragraph::new(Line::from(vec![
        Span::raw("[↑/↓] Move field    "),
        Span::raw("[←/→] Move cursor    "),
        Span::raw("[esc] Save & exit    "),
        Span::raw("[⏎] Toggle Done    "),
    ]))
    .block(Block::default());
    f.render_widget(header, area);
}

fn render_edit_fields(f: &mut Frame, chunks: Vec<Rect>, view_model: &EditModeModalViewModel) {
    let fields: Vec<Paragraph> = view_model.fields.iter().map(render_field).collect();
    for (i, field) in fields.iter().enumerate() {
        f.render_widget(field, chunks[i])
    }
}

fn render_status_span(f: &mut Frame, area: Rect, is_done: bool) {
    let status_span = if is_done {
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
    f.render_widget(status, area);
}

fn render_cursor(f: &mut Frame, area: Rect, view_model: &EditModeModalViewModel) {
    let selected_input = view_model.fields.get(view_model.selected_index).unwrap();
    let x = area.x + selected_input.character_index as u16 + 1;
    let y = area.y + 3 + (3 * view_model.selected_index as u16);
    f.set_cursor(x, y)
}

fn render_field<'a>(input: &Input) -> Paragraph<'a> {
    Paragraph::new(input.value.clone())
        .block(Block::bordered().title(input.title.clone()))
        .style(match input.selected {
            true => Style::default().fg(Color::Yellow),
            false => Style::default().fg(Color::White),
        })
        .wrap(Wrap { trim: true })
}
fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
