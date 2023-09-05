use std::env::current_dir;

#[derive(Debug)]
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
pub struct TOSConfig {
    pub tos_cfg_task_prio_max: String,
    pub tos_cfg_round_robin_en: bool,
    pub tos_cfg_object_verify_en: bool,
    pub tos_cfg_task_dynamic_create_en: bool,
    pub tos_cfg_event_en: bool,
    pub tos_cfg_mmblk_en: bool,
    pub tos_cfg_mmheap_en: bool,
    pub tos_cfg_mmheap_default_pool_en: bool,
    pub tos_cfg_mmheap_default_pool_size: String,
    pub tos_cfg_mutex_en: bool,
    pub tos_cfg_message_queue_en: bool,
    pub tos_cfg_mail_queue_en: bool,
    pub tos_cfg_priority_message_queue_en: bool,
    pub tos_cfg_priority_mail_queue_en: bool,
    pub tos_cfg_timer_en: bool,
    pub tos_cfg_pwr_mgr_en: bool,
    pub tos_cfg_tickless_en: bool,
    pub tos_cfg_sem_en: bool,
    pub tos_cfg_task_stack_draught_depth_detact_en: bool,
    pub tos_cfg_fault_backtrace_en: bool,
    pub tos_cfg_idle_task_stk_size: String,
    pub tos_cfg_cpu_tick_per_second: String,
    pub tos_cfg_cpu_clock: String,
    pub tos_cfg_timer_as_proc: bool,
}

impl TOSConfig {
    pub fn default() -> Self {
        Self {
            tos_cfg_task_prio_max: String::from("10"),
            tos_cfg_round_robin_en: false,
            tos_cfg_object_verify_en: true,
            tos_cfg_task_dynamic_create_en: true,
            tos_cfg_event_en: true,
            tos_cfg_mmblk_en: true,
            tos_cfg_mmheap_en: true,
            tos_cfg_mmheap_default_pool_en: true,
            tos_cfg_mmheap_default_pool_size: String::from("256"), // to hex
            tos_cfg_mutex_en: true,
            tos_cfg_message_queue_en: true,
            tos_cfg_mail_queue_en: true,
            tos_cfg_priority_message_queue_en: true,
            tos_cfg_priority_mail_queue_en: true,
            tos_cfg_timer_en: true,
            tos_cfg_pwr_mgr_en: true,
            tos_cfg_tickless_en: true,
            tos_cfg_sem_en: true,
            tos_cfg_task_stack_draught_depth_detact_en: true,
            tos_cfg_fault_backtrace_en: true,
            tos_cfg_idle_task_stk_size: String::from("128"),
            tos_cfg_cpu_tick_per_second: String::from("1000"),
            tos_cfg_cpu_clock: String::from("(SystemCoreClock)"),
            tos_cfg_timer_as_proc: true,
        }
    }
}
