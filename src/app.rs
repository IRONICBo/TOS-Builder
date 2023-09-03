use std::error;

use crate::components::fs::FolderList;

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
    /// Filelist
    pub fl: FolderList,

    /// TOS config
    pub cube_mx_project_config: CubeMXProjectConfig,
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
            fl: FolderList::default().unwrap(),
            cube_mx_project_config: CubeMXProjectConfig {
                path: "".to_string(),
                kind: CubeMXProjectType::GCC,
            },
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
    Fs,
}

/// CubeMX project config.
#[derive(Debug)]
pub struct CubeMXProjectConfig {
    /// Path to the CubeMX project.
    pub path: String,
    pub kind: CubeMXProjectType,
}

/// CubeMX project type.
#[derive(Debug)]
pub enum CubeMXProjectType {
    GCC,
    KEIL,
    IAR,
}
