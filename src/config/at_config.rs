#[derive(Debug)]
pub struct ATConfig {
    pub enable_air724: bool,
    pub enable_bc26: bool,
    pub enable_bc25_28_95: bool,
    pub enable_bc35_28_95_lwm2m: bool,
    pub enable_ec20: bool,
    pub enable_esp8266: bool,
    pub enable_m26: bool,
    pub enable_m5310a: bool,
    pub enable_m6312: bool,
    pub enable_sim800a: bool,
    pub enable_sim7600ce: bool,
    // TODO: Add other devices
}

impl ATConfig {
    pub fn default() -> Self {
        Self {
            enable_air724: false,
            enable_bc26: false,
            enable_bc25_28_95: false,
            enable_bc35_28_95_lwm2m: false,
            enable_ec20: false,
            enable_esp8266: false,
            enable_m26: false,
            enable_m5310a: false,
            enable_m6312: false,
            enable_sim800a: false,
            enable_sim7600ce: false,
        }
    }
}
