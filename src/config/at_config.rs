use log::{debug, info};
use serde::{Deserialize, Serialize};
use tui::widgets::TableState;

use super::{common::BoolValue, cubemx_config::CubeMXProjectType};

#[derive(Debug, Serialize, Deserialize)]
pub struct ATConfigTable {
    pub at_config: ATConfig,
    #[serde(skip)]
    pub index: TableState,
    pub len: usize,
}

impl ATConfigTable {
    pub fn default() -> Self {
        let at_config = ATConfig::default();
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        let len = at_config.to_vec().len();

        Self {
            at_config: ATConfig::default(),
            index: table_state,
            len: len,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ATConfig {
    pub enable_air724: BoolValue,
    pub enable_bc26: BoolValue,
    pub enable_bc25_28_95: BoolValue,
    pub enable_bc35_28_95_lwm2m: BoolValue,
    pub enable_ec20: BoolValue,
    pub enable_esp8266: BoolValue,
    pub enable_m26: BoolValue,
    pub enable_m5310a: BoolValue,
    pub enable_m6312: BoolValue,
    pub enable_sim800a: BoolValue,
    pub enable_sim7600ce: BoolValue,
    // TODO: Add other devices
}

impl ATConfig {
    pub fn default() -> Self {
        Self {
            enable_air724: BoolValue::new(String::from("ENABLE_AIR724"), false, String::from("Enable Air724 to use AT command")),
            enable_bc26: BoolValue::new(String::from("ENABLE_BC26"), false, String::from("Enable BC26 to use AT command")),
            enable_bc25_28_95: BoolValue::new(String::from("ENABLE_BC25_28_95"), false, String::from("Enable BC25/28/95 to use AT command")),
            enable_bc35_28_95_lwm2m: BoolValue::new(String::from("ENABLE_BC35_28_95_LWM2M"), false, String::from("Enable BC35/28/95 LWM2M to use AT command")),
            enable_ec20: BoolValue::new(String::from("ENABLE_EC20"), false, String::from("Enable EC20 to use AT command")),
            enable_esp8266: BoolValue::new(String::from("ENABLE_ESP8266"), false, String::from("Enable ESP8266 to use AT command")),
            enable_m26: BoolValue::new(String::from("ENABLE_M26"), false, String::from("Enable M26 to use AT command")),
            enable_m5310a: BoolValue::new(String::from("ENABLE_M5310A"), false, String::from("Enable M5310A to use AT command")),
            enable_m6312: BoolValue::new(String::from("ENABLE_M6312"), false, String::from("Enable M6312 to use AT command")),
            enable_sim800a: BoolValue::new(String::from("ENABLE_SIM800a"), false, String::from("Enable SIM800A to use AT command")),
            enable_sim7600ce: BoolValue::new(String::from("ENABLE_SIM7600CE"), false, String::from("Enable SIM7600CE to use AT command")),
        }
    }

    pub fn to_vec(&self) -> Vec<Vec<String>> {
        vec![
            self.enable_air724.to_vec(),
            self.enable_bc26.to_vec(),
            self.enable_bc25_28_95.to_vec(),
            self.enable_bc35_28_95_lwm2m.to_vec(),
            self.enable_ec20.to_vec(),
            self.enable_esp8266.to_vec(),
            self.enable_m26.to_vec(),
            self.enable_m5310a.to_vec(),
            self.enable_m6312.to_vec(),
            self.enable_sim800a.to_vec(),
            self.enable_sim7600ce.to_vec(),
        ]
    }

    pub fn update(&mut self, key: String, value: String) {
        match key.as_str() {
            "ENABLE_AIR724" => {
                self.enable_air724.value = value.parse::<bool>().unwrap();
                info!("Air724 Value: {:?}", value.parse::<bool>().unwrap());
                info!("Enable Air724: {:?}", self.enable_air724.value);
            }
            "ENABLE_BC26" => {
                self.enable_bc26.value = value.parse::<bool>().unwrap();
            }
            "ENABLE_BC25_28_95" => {
                self.enable_bc25_28_95.value = value.parse::<bool>().unwrap();
            }
            "ENABLE_BC35_28_95_LWM2M" => {
                self.enable_bc35_28_95_lwm2m.value = value.parse::<bool>().unwrap();
            }
            "ENABLE_EC20" => {
                self.enable_ec20.value = value.parse::<bool>().unwrap();
            }
            "ENABLE_ESP8266" => {
                self.enable_esp8266.value = value.parse::<bool>().unwrap();
            }
            "ENABLE_M26" => {
                self.enable_m26.value = value.parse::<bool>().unwrap();
            }
            "ENABLE_M5310A" => {
                self.enable_m5310a.value = value.parse::<bool>().unwrap();
            }
            "ENABLE_M6312" => {
                self.enable_m6312.value = value.parse::<bool>().unwrap();
            }
            "ENABLE_SIM800A" => {
                self.enable_sim800a.value = value.parse::<bool>().unwrap();
            }
            "ENABLE_SIM7600CE" => {
                self.enable_sim7600ce.value = value.parse::<bool>().unwrap();
            }
            _ => {
                debug!("TOSHeaderConfig update key {} not found", key);
            }
        }
    }

    pub fn is_enable(&self) -> bool {
        // check ATConfig has at least one device is enable
        self.enable_air724.value
            || self.enable_bc26.value
            || self.enable_bc25_28_95.value
            || self.enable_bc35_28_95_lwm2m.value
            || self.enable_ec20.value
            || self.enable_esp8266.value
            || self.enable_m26.value
            || self.enable_m5310a.value
            || self.enable_m6312.value
            || self.enable_sim800a.value
            || self.enable_sim7600ce.value
    }

    pub fn get_sal_module_path(&self, compiler: String) -> &'static str {
        match CubeMXProjectType::convert_to_type(compiler) {
            CubeMXProjectType::GCC => r"net/sal_module_wrapper",
            _ => r"net\sal_module_wrapper",
        }
    }

    pub fn get_first_enabled_device_source_path(&self, compiler: String) -> &'static str {
        match CubeMXProjectType::convert_to_type(compiler) {
            CubeMXProjectType::GCC => {
                if self.enable_air724.value {
                    return r"devices/air724"
                } else if self.enable_bc26.value {
                    return r"devices/air724"
                } else if self.enable_ec20.value {
                    return r"devices/ec20_200_600"
                } else if self.enable_esp8266.value {
                    return r"devices/esp8266"
                } else {
                    // Other does not support
                    return r""
                }
            },
            _ => {
                if self.enable_air724.value {
                    return r"devices\air724"
                } else if self.enable_bc26.value {
                    return r"devices\air724"
                } else if self.enable_ec20.value {
                    return r"devices\ec20_200_600"
                } else if self.enable_esp8266.value {
                    return r"devices\esp8266"
                } else {
                    // Other does not support
                    return r""
                }
            },
        }
    }
    
}
