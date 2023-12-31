use std::env::current_dir;

use log::debug;
use regex::Regex;
use serde::{Serialize, Deserialize};
use serde_json::value::{self, Map, Value as Json};
use tui::widgets::{TableState};

use super::{common::{StringValue, BoolValue}, cubemx_config::CubeMXProjectType};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct TOSProjectConfig {
    pub path: String,
    pub version: TOSProjectVersion,
}

impl TOSProjectConfig {
    pub fn default() -> Self {
        Self {
            path: current_dir().unwrap().to_str().unwrap().to_string(),
            version: TOSProjectVersion::VERSION_2_5_0,
        }
    }
}

/// TOS project type.
/// Reference: https://github.com/OpenAtomFoundation/TencentOS-tiny/tags
#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum TOSProjectVersion {
    VERSION_2_5_0,
    VERSION_2_4_5,
    VERSION_2_1_0,
}

impl TOSProjectVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            TOSProjectVersion::VERSION_2_5_0 => "v2.5.0",
            TOSProjectVersion::VERSION_2_4_5 => "v2.4.5",
            TOSProjectVersion::VERSION_2_1_0 => "v2.1.0",
        }
    }

    pub fn convert_to_type(t: String) -> TOSProjectVersion {
        match t.as_str() {
            "v2.5.0" => TOSProjectVersion::VERSION_2_5_0,
            "v2.4.5" => TOSProjectVersion::VERSION_2_4_5,
            "v2.1.0" => TOSProjectVersion::VERSION_2_1_0,
            _ => TOSProjectVersion::VERSION_2_5_0,
        }
    }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct TOSHeaderTable {
    pub tos_header_config: TOSHeaderConfig,
    #[serde(skip)]
    pub index: TableState,
    pub len: usize,
}

impl TOSHeaderTable {
    pub fn default() -> Self {
        let tos_header_config = TOSHeaderConfig::default();
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        let len = tos_header_config.to_vec().len();

        Self {
            tos_header_config: TOSHeaderConfig::default(),
            index: table_state,
            len: len,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TOSHeaderConfig {
    pub tos_cfg_header_include: StringValue,
    pub tos_cfg_task_prio_max: StringValue,
    pub tos_cfg_round_robin_en: BoolValue,
    pub tos_cfg_object_verify_en: BoolValue,
    pub tos_cfg_task_dynamic_create_en: BoolValue,
    pub tos_cfg_event_en: BoolValue,
    pub tos_cfg_mmblk_en: BoolValue,
    pub tos_cfg_mmheap_en: BoolValue,
    pub tos_cfg_mmheap_default_pool_en: BoolValue,
    pub tos_cfg_mmheap_default_pool_size: StringValue,
    pub tos_cfg_mutex_en: BoolValue,
    pub tos_cfg_message_queue_en: BoolValue,
    pub tos_cfg_mail_queue_en: BoolValue,
    pub tos_cfg_priority_message_queue_en: BoolValue,
    pub tos_cfg_priority_mail_queue_en: BoolValue,
    pub tos_cfg_timer_en: BoolValue,
    pub tos_cfg_pwr_mgr_en: BoolValue,
    pub tos_cfg_tickless_en: BoolValue,
    pub tos_cfg_sem_en: BoolValue,
    pub tos_cfg_task_stack_draught_depth_detact_en: BoolValue,
    pub tos_cfg_fault_backtrace_en: BoolValue,
    pub tos_cfg_idle_task_stk_size: StringValue,
    pub tos_cfg_cpu_tick_per_second: StringValue,
    pub tos_cfg_cpu_clock: StringValue,
    pub tos_cfg_timer_as_proc: BoolValue,
}

impl TOSHeaderConfig {
    pub fn default() -> Self {
        Self {
            tos_cfg_header_include: StringValue::new(String::from("TOS_CFG_HEADER_INCLUDE"), String::from("stm32l0xx.h"), String::from("Target chip header file, user needs to change accordingly")),
            tos_cfg_task_prio_max: StringValue::new(String::from("TOS_CFG_TASK_PRIO_MAX"), String::from("10"), String::from("Configure the maximum number of priorities supported by TencentOS tiny by default")),
            tos_cfg_round_robin_en: BoolValue::new(String::from("TOS_CFG_ROUND_ROBIN_EN"), false, String::from("Configure TencentOS tiny's kernel to enable or disable time-slice rotation.")),
            tos_cfg_object_verify_en: BoolValue::new(String::from("TOS_CFG_OBJECT_VERIFY_EN"), true, String::from("Configure whether TencentOS tiny checksums pointers for legality")),
            tos_cfg_task_dynamic_create_en: BoolValue::new(String::from("TOS_CFG_TASK_DYNAMIC_CREATE_EN"), true, String::from("TencentOS tiny Dynamic Task Creation Macros")),
            tos_cfg_event_en: BoolValue::new(String::from("TOS_CFG_EVENT_EN"), true, String::from("TencentOS tiny Event Module Macros")),
            tos_cfg_mmblk_en: BoolValue::new(String::from("TOS_CFG_MMBLK_EN"), true, String::from("Configure TencentOS tiny to enable the memory block management module")),
            tos_cfg_mmheap_en: BoolValue::new(String::from("TOS_CFG_MMHEAP_EN"), true, String::from("TencentOS tiny Configure TencentOS tiny to enable the Dynamic Memory Module")),
            tos_cfg_mmheap_default_pool_en: BoolValue::new(String::from("TOS_CFG_MMHEAP_DEFAULT_POOL_EN"), true, String::from("TencentOS tiny Default Dynamic Memory Pool Function Macro")),
            tos_cfg_mmheap_default_pool_size: StringValue::new(String::from("TOS_CFG_MMHEAP_DEFAULT_POOL_SIZE"), String::from("256"), String::from("Configure the TencentOS tiny default dynamic memory pool size")), // to hex
            tos_cfg_mutex_en: BoolValue::new(String::from("TOS_CFG_MUTEX_EN"), true, String::from("Configure TencentOS tiny to enable the Mutual Exclusion Lock module")),
            tos_cfg_message_queue_en: BoolValue::new(String::from("TOS_CFG_MESSAGE_QUEUE_EN"), true, String::from("Configure TencentOS tiny to enable the Message Queuing module")),
            tos_cfg_mail_queue_en: BoolValue::new(String::from("TOS_CFG_MAIL_QUEUE_EN"), true, String::from("Configure TencentOS tiny to enable the Message Mailbox module")),
            tos_cfg_priority_message_queue_en: BoolValue::new(String::from("TOS_CFG_PRIORITY_MESSAGE_QUEUE_EN"), true, String::from("Configure TencentOS tiny to enable the Priority Message Queuing Module")),
            tos_cfg_priority_mail_queue_en: BoolValue::new(String::from("TOS_CFG_PRIORITY_MAIL_QUEUE_EN"), true, String::from("Configuring TencentOS tiny to enable the Priority Message Mailbox module")),
            tos_cfg_timer_en: BoolValue::new(String::from("TOS_CFG_TIMER_EN"), true, String::from("Configuring TencentOS tiny to enable the Software Timer module")),
            tos_cfg_pwr_mgr_en: BoolValue::new(String::from("TOS_CFG_PWR_MGR_EN"), true, String::from("Configure TencentOS tiny to enable peripheral power management module")),
            tos_cfg_tickless_en: BoolValue::new(String::from("TOS_CFG_TICKLESS_EN"), true, String::from("Configure the Tickless low-power module switch")),
            tos_cfg_sem_en: BoolValue::new(String::from("TOS_CFG_SEM_EN"), true, String::from("Configuring TencentOS tiny to enable the semaphore module")),
            tos_cfg_task_stack_draught_depth_detact_en: BoolValue::new(String::from("TOS_CFG_TASK_STACK_DRAUGHT_DEPTH_DETACT_EN"), true, String::from("Configure TencentOS tiny to enable task stack depth detection")),
            tos_cfg_fault_backtrace_en: BoolValue::new(String::from("TOS_CFG_FAULT_BACKTRACE_EN"), true, String::from("Configure TencentOS tiny to enable anomaly stack backtracking")),
            tos_cfg_idle_task_stk_size: StringValue::new(String::from("TOS_CFG_IDLE_TASK_STK_SIZE"), String::from("128"), String::from("Configure the TencentOS tiny idle task stack size")),
            tos_cfg_cpu_tick_per_second: StringValue::new(String::from("TOS_CFG_CPU_TICK_PER_SECOND"), String::from("1000"), String::from("Configure the tick frequency of TencentOS tiny")),
            tos_cfg_cpu_clock: StringValue::new(String::from("TOS_CFG_CPU_CLOCK"), String::from("(SystemCoreClock)"), String::from("Configure TencentOS tiny CPU frequency")),
            tos_cfg_timer_as_proc: BoolValue::new(String::from("TOS_CFG_TIMER_AS_PROC"), true, String::from("Configure whether to configure TIMER to function mode")),
        }
    }

    pub fn to_vec(&self) -> Vec<Vec<String>> {
        vec![
            self.tos_cfg_header_include.to_vec(),
            self.tos_cfg_task_prio_max.to_vec(),
            self.tos_cfg_round_robin_en.to_vec(),
            self.tos_cfg_object_verify_en.to_vec(),
            self.tos_cfg_task_dynamic_create_en.to_vec(),
            self.tos_cfg_event_en.to_vec(),
            self.tos_cfg_mmblk_en.to_vec(),
            self.tos_cfg_mmheap_en.to_vec(),
            self.tos_cfg_mmheap_default_pool_en.to_vec(),
            self.tos_cfg_mmheap_default_pool_size.to_vec(),
            self.tos_cfg_mutex_en.to_vec(),
            self.tos_cfg_message_queue_en.to_vec(),
            self.tos_cfg_mail_queue_en.to_vec(),
            self.tos_cfg_priority_message_queue_en.to_vec(),
            self.tos_cfg_timer_en.to_vec(),
            self.tos_cfg_pwr_mgr_en.to_vec(),
            self.tos_cfg_tickless_en.to_vec(),
            self.tos_cfg_sem_en.to_vec(),
            self.tos_cfg_task_stack_draught_depth_detact_en.to_vec(),
            self.tos_cfg_idle_task_stk_size.to_vec(),
            self.tos_cfg_cpu_tick_per_second.to_vec(),
            self.tos_cfg_cpu_clock.to_vec(),
            self.tos_cfg_timer_as_proc.to_vec(),
        ]
    }

    pub fn to_map(&self) -> Map<String, Json> {
        let mut data = Map::new();
        data.insert(self.tos_cfg_header_include.key.clone(), value::to_value(&self.tos_cfg_header_include.value.clone()).unwrap());
        data.insert(self.tos_cfg_task_prio_max.key.clone(), value::to_value(&self.tos_cfg_task_prio_max.value.clone()).unwrap());
        data.insert(self.tos_cfg_round_robin_en.key.clone(), value::to_value(&self.tos_cfg_round_robin_en.value.clone()).unwrap());
        data.insert(self.tos_cfg_object_verify_en.key.clone(), value::to_value(&self.tos_cfg_object_verify_en.value.clone()).unwrap());
        data.insert(self.tos_cfg_task_dynamic_create_en.key.clone(), value::to_value(&self.tos_cfg_task_dynamic_create_en.value.clone()).unwrap());
        data.insert(self.tos_cfg_event_en.key.clone(), value::to_value(&self.tos_cfg_event_en.value.clone()).unwrap());
        data.insert(self.tos_cfg_mmblk_en.key.clone(), value::to_value(&self.tos_cfg_mmblk_en.value.clone()).unwrap());
        data.insert(self.tos_cfg_mmheap_en.key.clone(), value::to_value(&self.tos_cfg_mmheap_en.value.clone()).unwrap());
        data.insert(self.tos_cfg_mmheap_default_pool_en.key.clone(), value::to_value(&self.tos_cfg_mmheap_default_pool_en.value.clone()).unwrap());
        data.insert(self.tos_cfg_mmheap_default_pool_size.key.clone(), value::to_value(&self.tos_cfg_mmheap_default_pool_size.value.clone()).unwrap());
        data.insert(self.tos_cfg_mutex_en.key.clone(), value::to_value(&self.tos_cfg_mutex_en.value.clone()).unwrap());
        data.insert(self.tos_cfg_message_queue_en.key.clone(), value::to_value(&self.tos_cfg_message_queue_en.value.clone()).unwrap());
        data.insert(self.tos_cfg_mail_queue_en.key.clone(), value::to_value(&self.tos_cfg_mail_queue_en.value.clone()).unwrap());
        data.insert(self.tos_cfg_priority_message_queue_en.key.clone(), value::to_value(&self.tos_cfg_priority_message_queue_en.value.clone()).unwrap());
        data.insert(self.tos_cfg_priority_mail_queue_en.key.clone(), value::to_value(&self.tos_cfg_priority_mail_queue_en.value.clone()).unwrap());
        data.insert(self.tos_cfg_timer_en.key.clone(), value::to_value(&self.tos_cfg_timer_en.value.clone()).unwrap());
        data.insert(self.tos_cfg_pwr_mgr_en.key.clone(), value::to_value(&self.tos_cfg_pwr_mgr_en.value.clone()).unwrap());
        data.insert(self.tos_cfg_tickless_en.key.clone(), value::to_value(&self.tos_cfg_tickless_en.value.clone()).unwrap());
        data.insert(self.tos_cfg_sem_en.key.clone(), value::to_value(&self.tos_cfg_sem_en.value.clone()).unwrap());
        data.insert(self.tos_cfg_task_stack_draught_depth_detact_en.key.clone(), value::to_value(&self.tos_cfg_task_stack_draught_depth_detact_en.value.clone()).unwrap());
        data.insert(self.tos_cfg_fault_backtrace_en.key.clone(), value::to_value(&self.tos_cfg_fault_backtrace_en.value.clone()).unwrap());
        data.insert(self.tos_cfg_idle_task_stk_size.key.clone(), value::to_value(&self.tos_cfg_idle_task_stk_size.value.clone()).unwrap());
        data.insert(self.tos_cfg_cpu_tick_per_second.key.clone(), value::to_value(&self.tos_cfg_cpu_tick_per_second.value.clone()).unwrap());
        data.insert(self.tos_cfg_cpu_clock.key.clone(), value::to_value(&self.tos_cfg_cpu_clock.value.clone()).unwrap());
        data.insert(self.tos_cfg_timer_as_proc.key.clone(), value::to_value(&self.tos_cfg_timer_as_proc.value.clone()).unwrap());

        return data;
    }

    pub fn update(&mut self, key: String, value: String) {
        match key.as_str() {
            "TOS_CFG_HEADER_INCLUDE" => {
                self.tos_cfg_header_include.value = value;
            }
            "TOS_CFG_TASK_PRIO_MAX" => {
                self.tos_cfg_task_prio_max.value = value;
            }
            "TOS_CFG_ROUND_ROBIN_EN" => {
                self.tos_cfg_round_robin_en.value = value.parse::<bool>().unwrap();
            }
            "TOS_CFG_OBJECT_VERIFY_EN" => {
                self.tos_cfg_object_verify_en.value = value.parse::<bool>().unwrap();
            }
            "TOS_CFG_TASK_DYNAMIC_CREATE_EN" => {
                self.tos_cfg_task_dynamic_create_en.value = value.parse::<bool>().unwrap();
            }
            "TOS_CFG_EVENT_EN" => {
                self.tos_cfg_event_en.value = value.parse::<bool>().unwrap();
            }
            "TOS_CFG_MMBLK_EN" => {
                self.tos_cfg_mmblk_en.value = value.parse::<bool>().unwrap();
            }
            "TOS_CFG_MMHEAP_EN" => {
                self.tos_cfg_mmheap_en.value = value.parse::<bool>().unwrap();
            }
            "TOS_CFG_MMHEAP_DEFAULT_POOL_EN" => {
                self.tos_cfg_mmheap_default_pool_en.value = value.parse::<bool>().unwrap();
            }
            "TOS_CFG_MMHEAP_DEFAULT_POOL_SIZE" => {
                self.tos_cfg_mmheap_default_pool_size.value = value;
            }
            "TOS_CFG_MUTEX_EN" => {
                self.tos_cfg_mutex_en.value = value.parse::<bool>().unwrap();
            }
            "TOS_CFG_MESSAGE_QUEUE_EN" => {
                self.tos_cfg_message_queue_en.value = value.parse::<bool>().unwrap();
            }
            "TOS_CFG_MAIL_QUEUE_EN" => {
                self.tos_cfg_mail_queue_en.value = value.parse::<bool>().unwrap();
            }
            "TOS_CFG_PRIORITY_MESSAGE_QUEUE_EN" => {
                self.tos_cfg_priority_message_queue_en.value = value.parse::<bool>().unwrap();
            }
            "TOS_CFG_PRIORITY_MAIL_QUEUE_EN" => {
                self.tos_cfg_priority_mail_queue_en.value = value.parse::<bool>().unwrap();
            }
            "TOS_CFG_TIMER_EN" => {
                self.tos_cfg_timer_en.value = value.parse::<bool>().unwrap();
            }
            "TOS_CFG_PWR_MGR_EN" => {
                self.tos_cfg_pwr_mgr_en.value = value.parse::<bool>().unwrap();
            }
            "TOS_CFG_TICKLESS_EN" => {
                self.tos_cfg_tickless_en.value = value.parse::<bool>().unwrap();
            }
            "TOS_CFG_SEM_EN" => {
                self.tos_cfg_sem_en.value = value.parse::<bool>().unwrap();
            }
            "TOS_CFG_TASK_STACK_DRAUGHT_DEPTH_DETACT_EN" => {
                self.tos_cfg_task_stack_draught_depth_detact_en.value = value.parse::<bool>().unwrap();
            }
            "TOS_CFG_FAULT_BACKTRACE_EN" => {
                self.tos_cfg_fault_backtrace_en.value = value.parse::<bool>().unwrap();
            }
            "TOS_CFG_IDLE_TASK_STK_SIZE" => {
                self.tos_cfg_idle_task_stk_size.value = value;
            }
            "TOS_CFG_CPU_TICK_PER_SECOND" => {
                self.tos_cfg_cpu_tick_per_second.value = value;
            }
            "TOS_CFG_CPU_CLOCK" => {
                self.tos_cfg_cpu_clock.value = value;
            }
            "TOS_CFG_TIMER_AS_PROC" => {
                self.tos_cfg_timer_as_proc.value = value.parse::<bool>().unwrap();
            }
            _ => {
                debug!("TOSHeaderConfig update key {} not found", key);
            }
        }
    }

    // For at firmware
    pub fn get_at_hal_path(&self, compiler: String) -> &'static str {
        // get tos_cfg_header_include
        let tos_cfg_header_include = self.tos_cfg_header_include.value.clone();
        // remove suffix
        let re = Regex::new(r"\.h$").unwrap();
        let tos_cfg_header_include = re.replace_all(&tos_cfg_header_include, "").to_string();

        match CubeMXProjectType::convert_to_type(compiler) {
            CubeMXProjectType::GCC => match tos_cfg_header_include.as_str() {
                "gd32vf1xx" => {
                    r"platform/hal/gd/gd32vf1xx/src"
                }
                "mimxrt10xx" => {
                    r"platform/hal/nxp/mimxrt10xx/src"
                }
                "stm32f1xx" => {
                    r"platform/hal/st/stm32f1xx/src"
                }
                "stm32f4xx" => {
                    r"platform/hal/st/stm32f4xx/src"
                }
                "stm32f7xx" => {
                    r"platform/hal/st/stm32f7xx/src"
                }
                "stm32g0xx" => {
                    r"platform/hal/st/stm32g0xx/src"
                }
                "stm32h7xx" => {
                    r"platform/hal/st/stm32h7xx/src"
                }
                "stm32l0xx" => {
                    r"platform/hal/st/stm32l0xx/src"
                }
                "stm32l4xx" => {
                    r"platform/hal/st/stm32l4xx/src"
                }
                "stm32l5xx" => {
                    r"platform/hal/st/stm32l5xx/src"
                }
                "stm32wl" => {
                    r"platform/hal/st/stm32wl/src"
                }
                "ch32v30xx" => {
                    r"platform/hal/wch/ch32v30xx/src"
                }
                _ => {
                    r"platform/hal/st/stm32wl/src"
                }
            }
            _ => match tos_cfg_header_include.as_str() {
                "gd32vf1xx" => {
                    r"platform\hal\gd\gd32vf1xx\src"
                }
                "mimxrt10xx" => {
                    r"platform\hal\nxp\mimxrt10xx\src"
                }
                "stm32f1xx" => {
                    r"platform\hal\st\stm32f1xx\src"
                }
                "stm32f4xx" => {
                    r"platform\hal\st\stm32f4xx\src"
                }
                "stm32f7xx" => {
                    r"platform\hal\st\stm32f7xx\src"
                }
                "stm32g0xx" => {
                    r"platform\hal\st\stm32g0xx\src"
                }
                "stm32h7xx" => {
                    r"platform\hal\st\stm32h7xx\src"
                }
                "stm32l0xx" => {
                    r"platform\hal\st\stm32l0xx\src"
                }
                "stm32l4xx" => {
                    r"platform\hal\st\stm32l4xx\src"
                }
                "stm32l5xx" => {
                    r"platform\hal\st\stm32l5xx\src"
                }
                "stm32wl" => {
                    r"platform\hal\st\stm32wl\src"
                }
                "ch32v30xx" => {
                    r"platform\hal\wch\ch32v30xx\src"
                }
                _ => {
                    r"platform\hal\st\stm32wl\src"
                }
            }
        }

    }
}
