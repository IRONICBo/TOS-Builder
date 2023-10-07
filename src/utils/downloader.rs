use chrono::{DateTime, FixedOffset, Local,  Utc, TimeZone};
use std::path::PathBuf;
use std::str::FromStr;
use std::{error::Error, fs::File};
use std::io::{prelude::*, Stderr};
use log::*;
use tui::backend::CrosstermBackend;

use crate::app::App;
use crate::tui::Tui;

// link: https://github.com/OpenAtomFoundation/TencentOS-tiny/releases/tag/v2.5.0
const TOS_GITHUB_RELEASE_URL: &str = "https://codeload.github.com/OpenAtomFoundation/TencentOS-tiny/zip/refs/tags/";
const DUMMY_MAX_DOWNLOAD_SIZE: u64 = 320 * 1024 * 1024; // max master zip file is 320MB

// download tos with version and download to destionation path
#[tokio::main]
pub async fn download_tos(app: &mut App, tui: &mut Tui<CrosstermBackend<Stderr>>) -> Result<(), Box<dyn Error>> {
    let version = app.tos_project_config.version.as_str();
    let path = app.tos_project_config.path.as_str();
    
    let url = format!("{}{}", TOS_GITHUB_RELEASE_URL, version);
    let filename = format!("{}.zip", version);
    
    let mut file_path = PathBuf::from_str(path).unwrap();
    file_path.push(filename.as_str());
    let file_path_str = file_path.to_string_lossy();

    // Download to path
    // let mut resp: reqwest::Response = reqwest::get("https://fanyi-cdn.cdn.bcebos.com/static/translation/img/header/logo_e835568.png").await?;
    info!("fetch tos url {}", url);
    let mut resp: reqwest::Response = reqwest::get(url).await?;

    let len = resp.content_length().unwrap_or(DUMMY_MAX_DOWNLOAD_SIZE);
    app.download.set_total(len);
    info!("tos content_length {}", len);

    info!("downloading to path {}", file_path_str);
    let mut dest = File::create(file_path_str.to_string())?;

    let mut downloaded = 0;
    app.download_popup = true;
    app.download.set_start_time(Local::now().timestamp() as u64);
    while let Some(chunk) = resp.chunk().await? {
        downloaded += chunk.len();
        dest.write_all(&chunk)?;

        app.download.set_current(downloaded as u64);
        app.download.set_end_time(Utc::now().timestamp() as u64);

        // debug!("downloaded {}/{} cost({} s), ", downloaded, len, app.download.end_time - app.download.start_time);
        tui.draw(app)?;
    }

    // adust download size
    app.download.set_total(downloaded as u64);

    info!("download to path {} ok", file_path_str);
    app.download_popup = false;

    Ok(())
}


// pub fn unzip_tos()