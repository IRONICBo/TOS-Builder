use std::{cmp::{max, min}, env::{current_dir, set_current_dir}, path::Component};

use crossterm::event::KeyCode;
use log::*;

use crate::{app::{ActiveModules, App, AppResult}, utils::path, config::{cubemx_config::CubeMXProjectType, at_config}, components::input::InputMode};

pub fn handle_key_events(key_event: KeyCode, app: &mut App) -> AppResult<()> {
    match key_event {
        KeyCode::Char(to_insert) => {
            app.input.enter_char(to_insert);
        }
        KeyCode::Backspace => {
            app.input.delete_char();
        }
        KeyCode::Left => {
            app.input.move_cursor_left();
        }
        KeyCode::Right => {
            app.input.move_cursor_right();
        }
        KeyCode::Esc => {
            app.input.input_mode = InputMode::Normal;
        }
        KeyCode::Enter => choose_enter_item(app),
        _ => {}
    }
    Ok(())
}

fn choose_next_module(app: &mut App) {
    // Change to next
    if app.active_modules == ActiveModules::ProjectSelect(crate::app::ProjectSelect::Fs) {
        app.active_modules = ActiveModules::ProjectSelect(crate::app::ProjectSelect::Kind);
    } else if app.active_modules == ActiveModules::ProjectSelect(crate::app::ProjectSelect::Kind) {
        app.active_modules = ActiveModules::ProjectSelect(crate::app::ProjectSelect::Fs);
    }
}

fn choose_previous_module(app: &mut App) {
    // Change to next
    if app.active_modules == ActiveModules::ProjectSelect(crate::app::ProjectSelect::Fs) {
        app.active_modules = ActiveModules::ProjectSelect(crate::app::ProjectSelect::Kind);
    } else if app.active_modules == ActiveModules::ProjectSelect(crate::app::ProjectSelect::Kind) {
        app.active_modules = ActiveModules::ProjectSelect(crate::app::ProjectSelect::Fs);
    }
}

fn choose_upper_item(app: &mut App) {
    match app.active_modules {
        ActiveModules::ProjectSelect(crate::app::ProjectSelect::Fs) => {
            let flist = &mut app.fl;
            let len = flist.dirs.len() + flist.files.len();
            if let Some(selected) = flist.index.selected() {
                if selected == 0 {
                    // It has .. previous item, so choose len
                    flist.index.select(Some(len));
                } else {
                    flist.index.select(Some(max(0,selected - 1)));
                }
            } else {
                flist.index.select(Some(0));
            }
        }
        ActiveModules::ProjectSelect(crate::app::ProjectSelect::Kind) => {
            let klist = &mut app.kl;
            let len = klist.value.len();
            if let Some(selected) = klist.index.selected() {
                if selected == 0 {
                    klist.index.select(Some(len - 1));
                } else {
                    klist.index.select(Some(max(0, selected - 1)));
                }
            } else {
                klist.index.select(Some(0));
            }
        }
        _ => {}
    }
}

fn choose_down_item(app: &mut App) {
    match app.active_modules {
        ActiveModules::ProjectSelect(crate::app::ProjectSelect::Fs) => {
            let flist = &mut app.fl;
            let len = flist.dirs.len() + flist.files.len();
            if let Some(selected) = flist.index.selected() {
                if selected == len {
                    flist.index.select(Some(0));
                } else {
                    // It has .. previous item, so choose len
                    flist.index.select(Some(min(len, selected + 1)));
                }
            } else {
                flist.index.select(Some(0));
            }
        }
        ActiveModules::ProjectSelect(crate::app::ProjectSelect::Kind) => {
            let klist = &mut app.kl;
            let len = klist.value.len();
            if let Some(selected) = klist.index.selected() {
                if selected == len - 1 {
                    klist.index.select(Some(0));
                } else {
                    klist.index.select(Some(min(len - 1, selected + 1)));
                }
            } else {
                klist.index.select(Some(0));
            }
        }
        _ => {}
    }
}

fn choose_enter_item(app: &mut App) {
    // Write to input
    match app.active_modules {
        ActiveModules::AtConfig(crate::app::AtConfig::Config) => {
            let binding = app.at_config_table.at_config.to_vec();
            let idx = app.at_config_table.index.selected().expect("at config table index is none");
            let key = binding[idx][0].clone();

            // info!("Choose enter item: {:?}", key);
            // update value
            app.at_config_table.at_config.update(key, app.input.input.clone());
        },
        ActiveModules::TOSConfig(crate::app::TOSConfig::Config) => {
            let binding = app.tos_header_table.tos_header_config.to_vec();
            let idx = app.tos_header_table.index.selected().expect("tos header config table index is none");
            let key = binding[idx][0].clone();

            // info!("Choose enter item: {:?}", key);
            // update value
            app.tos_header_table.tos_header_config.update(key, app.input.input.clone());
        }
        _ => {}
    }

    // close popup and unset focus to input
    app.input_popup = false;
    app.input.input_mode = InputMode::Normal;
}

fn choose_selected_item(app: &mut App) {
    match app.active_modules {
        ActiveModules::ProjectSelect(crate::app::ProjectSelect::Fs) => {
            // Check is dir and only choose dir
            let flist = &mut app.fl;
            if let Some(selected) = flist.index.selected() {
                if selected <= flist.dirs.len() {
                    let dir = current_dir().unwrap();
                    match selected {
                        // .. to parent dir
                        0 => {
                        },
                        // to child dir
                        num => {
                            let dir_entry = &flist.dirs[num - 1];
                            let path = dir_entry.path();
                            app.cube_mx_project_config.path = String::from(path.to_string_lossy());
                        }
                    }
                }
            }
        }
        ActiveModules::ProjectSelect(crate::app::ProjectSelect::Kind) => {
            let klist = &mut app.kl;
            if let Some(selected) = klist.index.selected() {
                let kind = &klist.value[selected];
                app.cube_mx_project_config.kind = CubeMXProjectType::convert_to_type(kind.to_string());
            }
        }
        _ => {}
    }
}
