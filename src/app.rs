use std::{env::current_dir, error};

use crate::{
    components::{fs::FolderList, kinds::KindList},
    config::{cubemx_config::{CubeMXProjectConfig, CubeMXProjectType}, tos_config::{TOSProjectConfig, TOSProjectVersion}},
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,

    /// Routes
    pub routes: Routes,
    /// Active modules
    pub active_modules: ActiveModules,

    /// Filelist
    pub fl: FolderList,
    /// CubeMX kind list
    pub kl: KindList,
    /// TOS kind list
    pub tl: KindList,

    /// CubeMX project config
    pub cube_mx_project_config: CubeMXProjectConfig,
    /// TOS project config
    pub tos_project_config: TOSProjectConfig,
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
            fl: FolderList::default().unwrap(),
            kl: KindList::default(vec![
                CubeMXProjectType::GCC.as_str().to_string(),
                CubeMXProjectType::MDK.as_str().to_string(),
                CubeMXProjectType::IAR.as_str().to_string(),
            ])
            .unwrap(),
            tl: KindList::default(vec![
                TOSProjectVersion::VERSION_2_5_0.as_str().to_string(),
                TOSProjectVersion::VERSION_2_4_5.as_str().to_string(),
                TOSProjectVersion::VERSION_2_1_0.as_str().to_string(),
            ])
            .unwrap(),
            cube_mx_project_config: CubeMXProjectConfig::default(),
            tos_project_config: TOSProjectConfig::default(),
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
#[derive(Debug)]
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
pub enum ActiveModules {
    ProjectSelect(ProjectSelect),
    TOSDownload(TOSDownload),
    TOSConfig(TOSConfig),
    AtConfig(AtConfig),
    MakeConfig(MakeConfig),
}

/// ProjectSelect page modules
#[derive(PartialEq, Debug)]
pub enum ProjectSelect {
    Fs,
    Kind,
}

#[derive(PartialEq, Debug)]
pub enum TOSDownload {
    Fs,
    Version,
}

#[derive(PartialEq, Debug)]
pub enum TOSConfig {
    Fs,
    Type,
}

#[derive(PartialEq, Debug)]
pub enum AtConfig {
    Fs,
    Type,
}

#[derive(PartialEq, Debug)]
pub enum MakeConfig {
    Fs,
    Type,
}
