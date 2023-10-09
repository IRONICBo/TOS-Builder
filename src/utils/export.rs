use chrono::{DateTime, FixedOffset, Local,  Utc, TimeZone};
use std::path::{PathBuf, Path};
use std::str::FromStr;
use std::{error::Error, fs::File};
use std::io::{prelude::*, Stderr};
use log::*;
use tui::backend::CrosstermBackend;

use crate::app::App;
use crate::tui::Tui;
use crate::utils::copy::copy_dir_recursive;

pub fn export_project(app: &mut App, tui: &mut Tui<CrosstermBackend<Stderr>>) ->  Result<(), Box<dyn Error>> {
    app.export_popup = true;
    tui.draw(app)?;

    // prepare project
    // set arch & board & kernel & osal
    app.export.set_current(2);
    let _ = do_prepare_project(app, tui);
    app.export.set_end_time(Local::now().timestamp() as u64);
    tui.draw(app)?;

    // prepare kernel
    app.export.set_current(3);
    let _ = do_prepare_kernel(app, tui);
    app.export.set_end_time(Local::now().timestamp() as u64);
    tui.draw(app)?;

    // prepare tos header
    app.export.set_current(4);
    let _ = do_prepare_tos_header(app, tui);
    app.export.set_end_time(Local::now().timestamp() as u64);
    tui.draw(app)?;

    // prepare at
    app.export.set_current(5);
    let _ = do_prepeare_at(app, tui);
    app.export.set_end_time(Local::now().timestamp() as u64);
    tui.draw(app)?;

    std::thread::sleep(std::time::Duration::from_secs(1));
    app.export.set_end_time(Local::now().timestamp() as u64);
    app.export_popup = false;
    tui.draw(app)?;
    
    Ok(())
}

pub fn do_prepare_project(app: &mut App, tui: &mut Tui<CrosstermBackend<Stderr>>) ->  Result<(), Box<dyn Error>> {
    app.export.message = format!("set arch & board & kernel & osal dirs");
    info!("exporting message: {}", app.export.message);

    // get tos project path
    let mut generated = Path::new(app.cube_mx_project_config.generated.as_str());
    let mut cubemx_project = Path::new(app.cube_mx_project_config.path.as_str());
    let mut tos_dir = Path::new(app.tos_project_config.path.as_str());

    // copy arch
    let _  = copy_dir_recursive(tos_dir.join("arch").as_path(), generated.join("arch").as_path());
    info!("copy arch ok...");

    // copy board
    let project_name = cubemx_project.file_name().unwrap().clone().to_string_lossy().to_string();
    info!("copy board ok... {} => {}", cubemx_project.to_string_lossy(), generated.join("board").join(project_name.clone()).as_path().to_string_lossy());
    let _ = copy_dir_recursive(cubemx_project, generated.join("board").join(project_name).as_path());

    // copy kernel
    let _ = copy_dir_recursive(tos_dir.join("kernel").as_path(), generated.join("kernel").as_path());
    info!("copy kernel ok...");

    // copy osal
    let _ = copy_dir_recursive(tos_dir.join("osal").as_path(), generated.join("osal").as_path());
    info!("copy osal ok...");

    Ok(())  
}

pub fn do_prepare_kernel(app: &mut App, tui: &mut Tui<CrosstermBackend<Stderr>>) ->  Result<(), Box<dyn Error>> {
    app.export.message = format!("set basic kernel");
    info!("exporting message: {}", app.export.message);


    Ok(())  
}

pub fn do_prepare_tos_header(app: &mut App, tui: &mut Tui<CrosstermBackend<Stderr>>) ->  Result<(), Box<dyn Error>> {
    app.export.message = format!("set tos header");
    info!("exporting message: {}", app.export.message);


    Ok(())  
}

pub fn do_prepeare_at(app: &mut App, tui: &mut Tui<CrosstermBackend<Stderr>>) ->  Result<(), Box<dyn Error>> {
    app.export.message = format!("set at & devices");
    info!("exporting message: {}", app.export.message);


    Ok(())  
}