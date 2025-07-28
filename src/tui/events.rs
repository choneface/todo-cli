use crate::tui::app::InputMode;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub enum InputEvent {
    Quit,
    Down,
    Up,
    Left,
    Right,
    ToggleDone,
    ToggleExpand,
    EnableEditing,
    DisableEditing,
    Backspace,
    PromotePriority,
    DemotePriority,
    Char(char),
    TodoSplit,
    None,
}

pub fn poll_input(timeout: Duration, mode: InputMode) -> std::io::Result<InputEvent> {
    if event::poll(timeout)? {
        if let Event::Key(key) = event::read()? {
            return Ok(match mode {
                InputMode::Normal => match_key_code_for_normal_mode(key.code),
                InputMode::Editing => match_key_code_for_edit_mode(key.code),
            });
        }
    }
    Ok(InputEvent::None)
}

fn match_key_code_for_normal_mode(code: KeyCode) -> InputEvent {
    match code {
        KeyCode::Char('q') => InputEvent::Quit,
        KeyCode::Down => InputEvent::Down,
        KeyCode::Up => InputEvent::Up,
        KeyCode::Left => InputEvent::Left,
        KeyCode::Right => InputEvent::Right,
        KeyCode::Enter => InputEvent::ToggleDone,
        KeyCode::Char(' ') => InputEvent::ToggleExpand,
        KeyCode::Char('e') => InputEvent::EnableEditing,
        KeyCode::Esc => InputEvent::DisableEditing,
        KeyCode::Backspace => InputEvent::Backspace,
        KeyCode::Char('p') => InputEvent::PromotePriority,
        KeyCode::Char('l') => InputEvent::DemotePriority,
        KeyCode::Char('b') => InputEvent::TodoSplit,
        _ => InputEvent::None,
    }
}

fn match_key_code_for_edit_mode(code: KeyCode) -> InputEvent {
    match code {
        KeyCode::Down => InputEvent::Down,
        KeyCode::Up => InputEvent::Up,
        KeyCode::Left => InputEvent::Left,
        KeyCode::Right => InputEvent::Right,
        KeyCode::Esc => InputEvent::DisableEditing,
        KeyCode::Backspace => InputEvent::Backspace,
        KeyCode::Enter => InputEvent::ToggleDone,
        KeyCode::Char(c) => InputEvent::Char(c),
        _ => InputEvent::None,
    }
}
