use std::{fs::{File, self}, path::Path, io::{self, Stderr}, error::Error};

use chrono::Local;
use log::info;
use tui::backend::CrosstermBackend;
use zip::ZipArchive;

use crate::{app::App, tui::Tui};

pub fn extract_zip(app: &mut App, tui: &mut Tui<CrosstermBackend<Stderr>>) -> Result<(), Box<dyn Error>> {
    // TODO: borrow to new string to skip mut
    let tos_project_config = format!("{}", app.tos_project_config.path.as_str());
    let tos_project_version = format!("{}", app.tos_project_config.version.as_str());

    let parent_path = Path::new(tos_project_config.as_str());
    let zip_path = parent_path.join(format!("{}.zip", tos_project_version));
    let out_path = Path::new(tos_project_config.as_str());

    let file = File::open(zip_path).expect("open zip file error");
    let mut archive = ZipArchive::new(file).expect("open zip archive error");

    app.unzip.set_total(archive.len() as u64);
    app.unzip.set_start_time(Local::now().timestamp() as u64);
    app.unzip_popup = true;
    info!("unzip to path {}", out_path.to_str().unwrap());

    let mut idx = 0;
    while idx < archive.len() {
        let mut file = archive.by_index(idx)?;
        let out_path = out_path.join(file.name());
        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = fs::File::create(&out_path)?;
            io::copy(&mut file, &mut outfile)?;
        }
        
        // refresh frame
        app.unzip.set_current(idx as u64);
        app.unzip.set_end_time(Local::now().timestamp() as u64);
        tui.draw(app)?;

        idx += 1;
    }
    
    app.unzip_popup = false;
    info!("unzip to path {} ok...", out_path.to_str().unwrap());

    Ok(())
}