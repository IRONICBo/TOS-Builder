use std::{cmp::{max, min}, env::{current_dir, set_current_dir}, path::Component};

use crossterm::event::KeyCode;
use log::*;

use crate::{app::{ActiveModules, App, AppResult}, utils::path, config::cubemx_config::CubeMXProjectType, components::input::InputMode};

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
    match app.active_modules {
        ActiveModules::ProjectSelect(crate::app::ProjectSelect::Fs) => {
            let flist = &mut app.fl;
            if let Some(selected) = flist.index.selected() {
                if selected <= flist.dirs.len() {
                    let dir = current_dir().unwrap();
                    match selected {
                        // .. to parent dir
                        0 => match dir.parent() {
                            Some(dir) => {
                                set_current_dir(dir).unwrap();
                                flist.current = dir.to_string_lossy().to_string();
                                flist.index.select(Some(0));
                            }
                            None => {
                                // Set to root path
                                set_current_dir(Component::RootDir.as_os_str().to_str().unwrap()).unwrap();
                                flist.current = dir.to_string_lossy().to_string();
                                flist.index.select(Some(0));
                            }
                        },
                        // to child dir
                        num => {
                            let dir_entry = &flist.dirs[num - 1];
                            let path = dir_entry.path();
                            flist.current = String::from(path.to_string_lossy());
                            set_current_dir(path).unwrap();
                            flist.index.select(Some(0));
                        }
                    }
                    flist.refresh();
                }
            } else {
                flist.index.select(Some(0));
            }
        }
        ActiveModules::ProjectSelect(crate::app::ProjectSelect::Kind) => {
        }
        _ => {}
    }
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
