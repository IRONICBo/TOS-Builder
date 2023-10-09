use std::{cmp::{max, min}, env::{current_dir, set_current_dir}, path::Component, io::Stderr, thread::sleep};

use chrono::Local;
use crossterm::event::KeyCode;
use log::*;
use tui::backend::CrosstermBackend;

use crate::{app::{ActiveModules, App, AppResult}, utils::{path, config::export_config, export::export_project}, config::cubemx_config::CubeMXProjectType, tui::Tui, components::input::InputMode};

pub fn handle_key_events(key_event: KeyCode, app: &mut App, tui: &mut Tui<CrosstermBackend<Stderr>>) -> AppResult<()> {
    match key_event {
        KeyCode::Char('g') | KeyCode::Char('G') => choose_export_project(app, tui),
        _ => {}
    }
    Ok(())
}

pub fn choose_export_project(app: &mut App, tui: &mut Tui<CrosstermBackend<Stderr>>) {    
    // Start download and download to the config path
    info!("Start export...");
    app.export.set_start_time(Local::now().timestamp() as u64);
    app.export.set_total(5);
    app.export.set_end_time(Local::now().timestamp() as u64);

    // export config
    app.export.set_current(1);
    let _ = export_config(app, "config.json".to_string());

    // export project
    let _ = export_project(app, tui);
}
