use std::{
    env::current_dir,
    path::{Path},
};

use serde::{Deserialize, Serialize};

/// CubeMX project config.
#[derive(Debug, Serialize, Deserialize)]
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
            // Set generated path to parent directory + generated
            // fix: copy files recursively to the same directory
            generated: current_dir().unwrap().parent().unwrap().join(Path::new("generated")).to_str().unwrap().to_string(),
            kind: CubeMXProjectType::GCC,
            arch: ArchType::CortexM4,
        }
    }
}

/// CubeMX project type.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

    pub fn get_compiler(&self) -> &'static str {
        match self {
            CubeMXProjectType::GCC => "gcc",
            CubeMXProjectType::MDK => "armcc",
            CubeMXProjectType::IAR => "iccarm",
        }
    }
}

/// CubeMX arch type.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
            ArchType::Arcem => "arcem",
            ArchType::CortexM0 => "cortex-m0",
            ArchType::CortexA7 => "cortex-a7",
            ArchType::CortexM0Plus => "cortex-m0+",
            ArchType::CortexM3 => "cortex-m3",
            ArchType::CortexM4 => "cortex-m4",
            ArchType::CortexM7 => "cortex-m7",
            ArchType::CortexM23 => "cortex-m23",
            ArchType::CortexM33 => "cortex-m33",
            ArchType::ATMega32 => "ATMega32",
            ArchType::Posix => "posix",
            ArchType::MSP430X => "MSP430X",
            ArchType::Bumblebee => "bumblebee",
            ArchType::RiscV3A => "risc-v3a",
            ArchType::Rv32i => "rv32i",
            ArchType::Spike => "spike",
            ArchType::Stm8 => "stm8",
        }
    }

    pub fn convert_to_type(t: String) -> ArchType {
        match t.as_str() {
            "arcem" => ArchType::Arcem,
            "cortex-m0" => ArchType::CortexM0,
            "cortex-a7" => ArchType::CortexA7,
            "cortex-m0+" => ArchType::CortexM0Plus,
            "cortex-m3" => ArchType::CortexM3,
            "cortex-m4" => ArchType::CortexM4,
            "cortex-m7" => ArchType::CortexM7,
            "cortex-m23" => ArchType::CortexM23,
            "cortex-m33" => ArchType::CortexM33,
            "ATMega32" => ArchType::ATMega32,
            "posix" => ArchType::Posix,
            "MSP430X" => ArchType::MSP430X,
            "bumblebee" => ArchType::Bumblebee,
            "risc-v3a" => ArchType::RiscV3A,
            "rv32i" => ArchType::Rv32i,
            "spike" => ArchType::Spike,
            "stm8" => ArchType::Stm8,
            _ => ArchType::CortexM4,
        }
    }

    pub fn get_top_arch(&self, compiler: String) -> &'static str {
        match CubeMXProjectType::convert_to_type(compiler) {
            CubeMXProjectType::GCC => match self {
                ArchType::Arcem => "arc",
                ArchType::CortexM0 => "arm/arm-v6m",
                ArchType::CortexA7 => "arm/arm-v7a",
                ArchType::CortexM0Plus => "arm/arm-v7m",
                ArchType::CortexM3 => "arm/arm-v7m",
                ArchType::CortexM4 => "arm/arm-v7m",
                ArchType::CortexM7 => "arm/arm-v7m",
                ArchType::CortexM23 => "arm/arm-v8m",
                ArchType::CortexM33 => "arm/arm-v8m",
                ArchType::ATMega32 => "avr",
                ArchType::Posix => "linux",
                ArchType::MSP430X => "msp430",
                ArchType::Bumblebee => "risc-v",
                ArchType::RiscV3A => "risc-v",
                ArchType::Rv32i => "risc-v",
                ArchType::Spike => "risc-v",
                ArchType::Stm8 => "stm8", // unsupported
            }
            _ => match self {
                ArchType::Arcem => "arc",
                ArchType::CortexM0 => "arm\\arm-v6m",
                ArchType::CortexA7 => "arm\\arm-v7a",
                ArchType::CortexM0Plus => "arm\\arm-v7m",
                ArchType::CortexM3 => "arm\\arm-v7m",
                ArchType::CortexM4 => "arm\\arm-v7m",
                ArchType::CortexM7 => "arm\\arm-v7m",
                ArchType::CortexM23 => "arm\\arm-v8m",
                ArchType::CortexM33 => "arm\\arm-v8m",
                ArchType::ATMega32 => "avr",
                ArchType::Posix => "linux",
                ArchType::MSP430X => "msp430",
                ArchType::Bumblebee => "risc-v",
                ArchType::RiscV3A => "risc-v",
                ArchType::Rv32i => "risc-v",
                ArchType::Spike => "risc-v",
                ArchType::Stm8 => "stm8", // unsupported
            }
        }
    }
}
