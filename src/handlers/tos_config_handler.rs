use std::{io::Stderr};

use crossterm::event::KeyCode;

use tui::backend::CrosstermBackend;

use crate::{app::{ActiveModules, App, AppResult}, tui::Tui, components::input::InputMode};

pub fn handle_key_events(key_event: KeyCode, app: &mut App, _tui: &mut Tui<CrosstermBackend<Stderr>>) -> AppResult<()> {
    match key_event {
        KeyCode::Char('a') | KeyCode::Char('A') => choose_previous_module(app),
        KeyCode::Char('d') | KeyCode::Char('D') => choose_next_module(app),
        KeyCode::Char(' ') => choose_selected_item(app),
        KeyCode::Enter => choose_enter_item(app),
        KeyCode::Up => choose_upper_item(app),
        KeyCode::Down => choose_down_item(app),
        _ => {}
    }
    Ok(())
}

fn choose_next_module(_app: &mut App) {
    // Change to next
}

fn choose_previous_module(_app: &mut App) {
    // Change to previous
}

fn choose_upper_item(app: &mut App) {
    match app.active_modules {
        ActiveModules::TOSConfig(crate::app::TOSConfig::Config) => {
            let i = match app.tos_header_table.index.selected() {
                Some(i) => {
                    if i == 0 {
                        app.tos_header_table.len - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            app.tos_header_table.index.select(Some(i));
        }
        _ => {}
    }
}

fn choose_down_item(app: &mut App) {
    match app.active_modules {
        ActiveModules::TOSConfig(crate::app::TOSConfig::Config) => {
            let i = match app.tos_header_table.index.selected() {
                Some(i) => {
                    if i >= app.tos_header_table.len - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            app.tos_header_table.index.select(Some(i));
        }
        _ => {}
    }
}

fn choose_enter_item(app: &mut App) {
    // open popup and set focus to input
    app.input_popup = true;
    app.input.input_mode = InputMode::Editing;

    // set default value
    let binding = app.tos_header_table.tos_header_config.to_vec();
    let idx = app.tos_header_table.index.selected().expect("tos header config table index is none");

    let _key = &binding[idx][0];
    let value = &binding[idx][1];

    app.input.input = value.to_string();
    app.input.cursor_position = app.input.input.as_str().len() as usize;

    // to input handler
}

fn choose_selected_item(app: &mut App) {
    match app.active_modules {
        ActiveModules::TOSConfig(crate::app::TOSConfig::Config) => {
        }
        _ => {}
    }
}
