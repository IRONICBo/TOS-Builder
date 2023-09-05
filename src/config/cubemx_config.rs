use std::env::current_dir;

/// CubeMX project config.
#[derive(Debug)]
pub struct CubeMXProjectConfig {
    /// Path to the CubeMX project.
    pub path: String,
    pub kind: CubeMXProjectType,
}

impl CubeMXProjectConfig {
    pub fn default() -> Self {
        Self {
            path: current_dir().unwrap().to_str().unwrap().to_string(),
            kind: CubeMXProjectType::GCC,
        }
    }
}

/// CubeMX project type.
#[derive(Debug, PartialEq)]
pub enum CubeMXProjectType {
    GCC,
    MDK,
    IAR,
}

impl CubeMXProjectType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CubeMXProjectType::GCC => "GCC",
            CubeMXProjectType::MDK => "MDK",
            CubeMXProjectType::IAR => "IAR",
        }
    }

    pub fn convert_to_type(t: String) -> CubeMXProjectType {
        match t.as_str() {
            "GCC" => CubeMXProjectType::GCC,
            "MDK" => CubeMXProjectType::MDK,
            "IAR" => CubeMXProjectType::IAR,
            _ => CubeMXProjectType::GCC,
        }
    }
}
