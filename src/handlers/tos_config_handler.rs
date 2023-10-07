use std::{cmp::{max, min}, env::{current_dir, set_current_dir}, path::Component, io::Stderr};

use crossterm::event::KeyCode;
use log::*;
use tui::backend::CrosstermBackend;

use crate::{app::{ActiveModules, App, AppResult}, utils::path, config::cubemx_config::CubeMXProjectType, tui::Tui};

pub fn handle_key_events(key_event: KeyCode, app: &mut App, tui: &mut Tui<CrosstermBackend<Stderr>>) -> AppResult<()> {
    match key_event {
        KeyCode::Char('a') | KeyCode::Char('A') => choose_next_module(app),
        KeyCode::Char('d') | KeyCode::Char('D') => choose_previous_module(app),
        KeyCode::Char(' ') => choose_selected_item(app),
        KeyCode::Enter => choose_enter_item(app),
        KeyCode::Up => choose_upper_item(app),
        KeyCode::Down => choose_down_item(app),
        _ => {}
    }
    Ok(())
}

fn choose_next_module(app: &mut App) {
    // Change to next
}

fn choose_previous_module(app: &mut App) {
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
    match app.active_modules {
        ActiveModules::TOSConfig(crate::app::TOSConfig::Config) => {
            // open popup
            app.input_popup = true;
        }
        _ => {}
    }
}

fn choose_selected_item(app: &mut App) {
    match app.active_modules {
        ActiveModules::TOSConfig(crate::app::TOSConfig::Config) => {
        }
        _ => {}
    }
}
