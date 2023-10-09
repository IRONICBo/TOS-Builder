use std::{env::current_dir, path::{self, Path}};

use serde::{Deserialize, Serialize};

/// CubeMX project config.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct CubeMXProjectConfig {
    /// Path to the CubeMX project.
    pub path: String,
    pub generated: String,
    pub kind: CubeMXProjectType,
    pub arch: ArchType,
}

impl CubeMXProjectConfig {
    pub fn default() -> Self {
        Self {
            path: current_dir().unwrap().to_str().unwrap().to_string(),
            // Set generated path to current directory + generated
            generated: current_dir().unwrap().join(Path::new("generated")).to_str().unwrap().to_string(),
            kind: CubeMXProjectType::GCC,
            arch: ArchType::CortexM4,
        }
    }
}

/// CubeMX project type.
#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
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

/// CubeMX arch type.
#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum ArchType {
    Arcem,
    CortexM0,
    CortexA7,
    CortexM0Plus,
    CortexM3,
    CortexM4,
    CortexM7,
    CortexM23,
    CortexM33,
    ATMega32,
    Posix,
    MSP430X,
    Bumblebee,
    RiscV3A,
    Rv32i,
    Spike,
    Stm8,
}

impl ArchType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ArchType::Arcem => "Arcem",
            ArchType::CortexM0 => "Cortex-M0",
            ArchType::CortexA7 => "Cortex-A7",
            ArchType::CortexM0Plus => "Cortex-M0+",
            ArchType::CortexM3 => "Cortex-M3",
            ArchType::CortexM4 => "Cortex-M4",
            ArchType::CortexM7 => "Cortex-M7",
            ArchType::CortexM23 => "Cortex-M23",
            ArchType::CortexM33 => "Cortex-M33",
            ArchType::ATMega32 => "ATMega32",
            ArchType::Posix => "Posix",
            ArchType::MSP430X => "MSP430X",
            ArchType::Bumblebee => "Bumblebee",
            ArchType::RiscV3A => "RiscV3A",
            ArchType::Rv32i => "Rv32i",
            ArchType::Spike => "Spike",
            ArchType::Stm8 => "Stm8",
        }
    }

    pub fn convert_to_type(t: String) -> ArchType {
        match t.as_str() {
            "Arcem" => ArchType::Arcem,
            "Cortex-M0" => ArchType::CortexM0,
            "Cortex-A7" => ArchType::CortexA7,
            "Cortex-M0+" => ArchType::CortexM0Plus,
            "Cortex-M3" => ArchType::CortexM3,
            "Cortex-M4" => ArchType::CortexM4,
            "Cortex-M7" => ArchType::CortexM7,
            "Cortex-M23" => ArchType::CortexM23,
            "Cortex-M33" => ArchType::CortexM33,
            "ATMega32" => ArchType::ATMega32,
            "Posix" => ArchType::Posix,
            "MSP430X" => ArchType::MSP430X,
            "Bumblebee" => ArchType::Bumblebee,
            "RiscV3A" => ArchType::RiscV3A,
            "Rv32i" => ArchType::Rv32i,
            "Spike" => ArchType::Spike,
            "Stm8" => ArchType::Stm8,
            _ => ArchType::CortexM4,
        }
    }
}
