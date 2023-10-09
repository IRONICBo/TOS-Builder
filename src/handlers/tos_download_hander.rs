use std::{cmp::{max, min}, env::{current_dir, set_current_dir}, path::Component, thread, io::Stderr};

use crossterm::event::KeyCode;
use log::*;
use tui::backend::CrosstermBackend;

use crate::{app::{ActiveModules, App, AppResult}, utils::{path, downloader::{self, download_tos}, extract_zip::extract_zip}, config::{cubemx_config::CubeMXProjectType, tos_config::TOSProjectVersion}, components::download, tui::Tui};

pub fn handle_key_events(key_event: KeyCode, app: &mut App, tui: &mut Tui<CrosstermBackend<Stderr>>) -> AppResult<()> {
    match key_event {
        KeyCode::Char('a') | KeyCode::Char('A') => choose_previous_module(app),
        KeyCode::Char('d') | KeyCode::Char('D') => choose_next_module(app),
        KeyCode::Char(' ') => choose_selected_item(app),
        KeyCode::Enter => choose_enter_item(app, tui),
        KeyCode::Up => choose_upper_item(app),
        KeyCode::Down => choose_down_item(app),
        _ => {}
    }
    Ok(())
}

fn choose_next_module(app: &mut App) {
    // Change to next
    if app.active_modules == ActiveModules::TOSDownload(crate::app::TOSDownload::Fs) {
        app.active_modules = ActiveModules::TOSDownload(crate::app::TOSDownload::Version);
    } else if app.active_modules == ActiveModules::TOSDownload(crate::app::TOSDownload::Version) {
        app.active_modules = ActiveModules::TOSDownload(crate::app::TOSDownload::Fs);
    }
}

fn choose_previous_module(app: &mut App) {
    // Change to next
    if app.active_modules == ActiveModules::TOSDownload(crate::app::TOSDownload::Fs) {
        app.active_modules = ActiveModules::TOSDownload(crate::app::TOSDownload::Version);
    } else if app.active_modules == ActiveModules::TOSDownload(crate::app::TOSDownload::Version) {
        app.active_modules = ActiveModules::TOSDownload(crate::app::TOSDownload::Fs);
    }
}

fn choose_upper_item(app: &mut App) {
    match app.active_modules {
        ActiveModules::TOSDownload(crate::app::TOSDownload::Fs) => {
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
        ActiveModules::TOSDownload(crate::app::TOSDownload::Version) => {
            let tlist = &mut app.tl;
            let len = tlist.value.len();
            if let Some(selected) = tlist.index.selected() {
                if selected == 0 {
                    tlist.index.select(Some(len - 1));
                } else {
                    tlist.index.select(Some(max(0, selected - 1)));
                }
            } else {
                tlist.index.select(Some(0));
            }
        }
        _ => {}
    }
}

fn choose_down_item(app: &mut App) {
    match app.active_modules {
        ActiveModules::TOSDownload(crate::app::TOSDownload::Fs) => {
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
        ActiveModules::TOSDownload(crate::app::TOSDownload::Version) => {
            let tlist = &mut app.tl;
            let len = tlist.value.len();
            if let Some(selected) = tlist.index.selected() {
                if selected == len - 1 {
                    tlist.index.select(Some(0));
                } else {
                    tlist.index.select(Some(min(len - 1, selected + 1)));
                }
            } else {
                tlist.index.select(Some(0));
            }
        }
        _ => {}
    }
}

fn choose_enter_item(app: &mut App, tui: &mut Tui<CrosstermBackend<Stderr>>) {
    match app.active_modules {
        ActiveModules::TOSDownload(crate::app::TOSDownload::Fs) => {
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
        ActiveModules::TOSDownload(crate::app::TOSDownload::Version) => {
            // Start download and download to the config path
            info!("Current config path: {}", app.tos_project_config.path);
            info!("Current config version: {}", app.tos_project_config.version.as_str());

            let _ = download_tos(app, tui);

            let _ = extract_zip(app, tui);
        }
        _ => {}
    }
}

fn choose_selected_item(app: &mut App) {
    match app.active_modules {
        ActiveModules::TOSDownload(crate::app::TOSDownload::Fs) => {
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
                            app.tos_project_config.path = String::from(path.to_string_lossy());
                        }
                    }
                }
            }
        }
        ActiveModules::TOSDownload(crate::app::TOSDownload::Version) => {
            let tlist = &mut app.tl;
            if let Some(selected) = tlist.index.selected() {
                let kind = &tlist.value[selected];
                app.tos_project_config.version = TOSProjectVersion::convert_to_type(kind.to_string());

                // donwload release zip and unzip projects
            }
        }
        _ => {}
    }
}
