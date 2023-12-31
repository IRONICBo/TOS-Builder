use std::{error};

use serde::{Serialize, Deserialize};

use crate::{
    components::{fs::FolderList, kinds::KindList, input::Input, download::Download, unzip::Unzip, export::Export},
    config::{cubemx_config::{CubeMXProjectConfig, CubeMXProjectType, ArchType}, tos_config::{TOSProjectConfig, TOSProjectVersion, TOSHeaderTable}, at_config::ATConfigTable},
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    // Is the application running?
    pub running: bool,
    // counter
    pub counter: u8,

    // Routes
    pub routes: Routes,
    // Active modules
    pub active_modules: ActiveModules,
    // Input popup
    pub input_popup: bool,
    // Input
    pub input: Input,
    // Download popup
    pub download_popup: bool,
    // Donwload
    pub download: Download,
    // Unzip popup
    pub unzip_popup: bool,
    // Unzip
    pub unzip: Unzip,
    // Export popup
    pub export_popup: bool,
    // Export
    pub export: Export,

    // Filelist
    pub fl: FolderList,
    // CubeMX kind list
    pub kl: KindList,
    // CubeMX arch list
    pub arch: KindList,
    // TOS kind list
    pub tl: KindList,

    // CubeMX project config
    pub cube_mx_project_config: CubeMXProjectConfig,
    // TOS project config
    pub tos_project_config: TOSProjectConfig,
    // TOS header config
    pub tos_header_table: TOSHeaderTable,
    // AT config
    pub at_config_table: ATConfigTable
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            routes: Routes::new(
                vec![
                    "Project Select".to_string(),
                    "TOS Download".to_string(),
                    "TOS Config".to_string(),
                    "AT Config".to_string(),
                    "Make Config".to_string(),
                ],
                0,
            ),
            active_modules: ActiveModules::ProjectSelect(ProjectSelect::Fs),
            input_popup: false,
            input: Input::default(),
            download_popup: false,
            download: Download::default(),
            unzip_popup: false,
            unzip: Unzip::default(),
            export_popup: false,
            export: Export::default(),
            fl: FolderList::default().unwrap(),
            kl: KindList::default(vec![
                CubeMXProjectType::GCC.as_str().to_string(),
                CubeMXProjectType::MDK.as_str().to_string(),
                CubeMXProjectType::IAR.as_str().to_string(),
            ])
            .unwrap(),
            arch: KindList::default(vec![
                ArchType::Arcem.as_str().to_string(),
                ArchType::CortexM0.as_str().to_string(),
                ArchType::CortexA7.as_str().to_string(),
                ArchType::CortexM0Plus.as_str().to_string(),
                ArchType::CortexM3.as_str().to_string(),
                ArchType::CortexM4.as_str().to_string(),
                ArchType::CortexM7.as_str().to_string(),
                ArchType::CortexM23.as_str().to_string(),
                ArchType::CortexM33.as_str().to_string(),
                ArchType::ATMega32.as_str().to_string(),
                ArchType::Posix.as_str().to_string(),
                ArchType::MSP430X.as_str().to_string(),
                ArchType::Bumblebee.as_str().to_string(),
                ArchType::RiscV3A.as_str().to_string(),
                ArchType::Rv32i.as_str().to_string(),
                ArchType::Spike.as_str().to_string(),
                ArchType::Stm8.as_str().to_string(),
            ]).unwrap(),
            tl: KindList::default(vec![
                TOSProjectVersion::VERSION_2_5_0.as_str().to_string(),
                TOSProjectVersion::VERSION_2_4_5.as_str().to_string(),
                TOSProjectVersion::VERSION_2_1_0.as_str().to_string(),
            ])
            .unwrap(),
            cube_mx_project_config: CubeMXProjectConfig::default(),
            tos_project_config: TOSProjectConfig::default(),
            tos_header_table: TOSHeaderTable::default(),
            at_config_table: ATConfigTable::default(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_routes(&mut self, routes: Routes) {
        self.routes = routes;
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}

/// Routes.
#[derive(Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Routes {
    /// Name of the route.
    pub name: Vec<String>,
    /// Index of the route.
    pub current: usize,
}

impl Routes {
    /// Constructs a new instance of [`Routes`].
    pub fn new(name: Vec<String>, current: usize) -> Self {
        Self { name, current }
    }

    pub fn next(&mut self) {
        self.current = (self.current + 1) % self.name.len();
    }

    pub fn previous(&mut self) {
        if self.current > 0 {
            self.current -= 1;
        } else {
            self.current = self.name.len() - 1;
        }
    }
}

/// Selected modules
#[derive(PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
pub enum ActiveModules {
    ProjectSelect(ProjectSelect),
    TOSDownload(TOSDownload),
    TOSConfig(TOSConfig),
    AtConfig(AtConfig),
    MakeConfig(MakeConfig),
}

/// ProjectSelect page modules
#[derive(PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
pub enum ProjectSelect {
    Fs,
    Kind,
    Arch,
}

#[derive(PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
pub enum TOSDownload {
    Fs,
    Version,
}

#[derive(PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
pub enum TOSConfig {
    Config
}

#[derive(PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
pub enum AtConfig {
    Config
}

#[derive(PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
pub enum MakeConfig {
    Config
}
