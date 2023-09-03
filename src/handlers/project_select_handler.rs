use std::cmp::{max, min};

use crossterm::event::KeyCode;

use crate::app::{ActiveModules, App, AppResult};

pub fn handle_key_events(key_event: KeyCode, app: &mut App) -> AppResult<()> {
    match key_event {
        KeyCode::Char('a') | KeyCode::Char('A') => choose_next_module(app),
        KeyCode::Char('d') | KeyCode::Char('D') => choose_previous_module(app),
        KeyCode::Up => choose_upper_item(app),
        KeyCode::Down => choose_down_item(app),
        KeyCode::Enter => choose_enter_item(app),
        KeyCode::Char(' ') => choose_selected_item(app),
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
                if selected == len {
                    flist.index.select(Some(0));
                }
                flist.index.select(Some(max(0, selected + 1)));
            } else {
                flist.index.select(Some(0));
            }
        }
        ActiveModules::ProjectSelect(crate::app::ProjectSelect::Kind) => {
            let klist = &mut app.kl;
            let len = klist.value.len();
            if let Some(selected) = klist.index.selected() {
                if selected == len {
                    klist.index.select(Some(0));
                }
                klist.index.select(Some(max(0, selected + 1)));
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
                }
                flist.index.select(Some(min(len, selected + 1)));
            } else {
                flist.index.select(Some(0));
            }
        }
        ActiveModules::ProjectSelect(crate::app::ProjectSelect::Kind) => {
            let klist = &mut app.kl;
            let len = klist.value.len();
            if let Some(selected) = klist.index.selected() {
                if selected == len {
                    klist.index.select(Some(0));
                }
                klist.index.select(Some(min(len, selected + 1)));
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
            let len = flist.dirs.len() + flist.files.len();
            if let Some(selected) = flist.index.selected() {
                if selected == len {
                    flist.index.select(Some(0));
                }
                flist.index.select(Some(min(len, selected + 1)));
            } else {
                flist.index.select(Some(0));
            }
        }
        ActiveModules::ProjectSelect(crate::app::ProjectSelect::Kind) => {
            let klist = &mut app.kl;
            let len = klist.value.len();
            if let Some(selected) = klist.index.selected() {
                if selected == len {
                    klist.index.select(Some(0));
                }
                klist.index.select(Some(min(len, selected + 1)));
            } else {
                klist.index.select(Some(0));
            }
        }
        _ => {}
    }
}

fn choose_selected_item(app: &mut App) {
    match app.active_modules {
        ActiveModules::ProjectSelect(crate::app::ProjectSelect::Fs) => {
            let flist = &mut app.fl;
            let len = flist.dirs.len() + flist.files.len();
            if let Some(selected) = flist.index.selected() {
                if selected == len {
                    flist.index.select(Some(0));
                }
                flist.index.select(Some(min(len, selected + 1)));
            } else {
                flist.index.select(Some(0));
            }
        }
        ActiveModules::ProjectSelect(crate::app::ProjectSelect::Kind) => {
            let klist = &mut app.kl;
            let len = klist.value.len();
            if let Some(selected) = klist.index.selected() {
                if selected == len {
                    klist.index.select(Some(0));
                }
                klist.index.select(Some(min(len, selected + 1)));
            } else {
                klist.index.select(Some(0));
            }
        }
        _ => {}
    }
}
