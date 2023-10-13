# 文件解压和基本的配置修改

### 基本的文件解压库参考

> Ref: https://github.com/zip-rs/zip/blob/master/examples

### 关于解压之后的逻辑

需要在解压之后，然后重新选择Tencentos-tiny的路径。

### rust的借用问题

```bash
error[E0502]: cannot borrow `*app` as mutable because it is also borrowed as immutable
  --> src/utils/extract_zip.rs:45:9
   |
11 |     let tos_project_config = app.tos_project_config.path.as_str();
   |                              ------------------------------------ immutable borrow occurs here
...
29 |         let out_path = out_path.join(file.name());
   |                        -------------------------- immutable borrow later used here
...
45 |         tui.draw(app)?;
   |         ^^^^^^^^^^^^^ mutable borrow occurs here
```

看起来好像是draw的问题，其他的还需要继续弄。
```rs
for i in 0..archive.len() {
    info!("unzip {}/{}", i, archive.len());
    let mut file = archive.by_index(i)?;
    let out_path = out_path.join(file.name());
    if (&*file.name()).ends_with('/') {
        fs::create_dir_all(&out_path)?;
    } else {
        if let Some(p) = out_path.parent() {
            if !p.exists() {
                fs::create_dir_all(&p)?;
            }
        }
        let mut outfile = fs::File::create(&out_path)?;
        io::copy(&mut file, &mut outfile)?;
    }
    
    // refresh frame
    app.unzip.set_current(i as u64);
    app.unzip.set_end_time(Local::now().timestamp() as u64);
}

tui.draw(app)?;
```

### 关于使用input修改数据的逻辑实现

基本上都是可以封装的hhh，大概就是设置显示，设置聚焦，然后输入数据，最后enter会写入数据并且重置聚焦逻辑！！！

其他模块关于enter的实现，主要是进入聚焦，并且进入input的实现！！！
```bash
fn choose_enter_item(app: &mut App) {
    // open popup and set focus to input
    app.input_popup = true;
    app.input.input_mode = InputMode::Editing;
    match app.active_modules {
        ActiveModules::AtConfig(crate::app::AtConfig::Config) => {
        }
        _ => {}
    }
}
```

input模块中关于enter的实现，主要是取消聚焦
```bash
fn choose_enter_item(app: &mut App) {
    // Write to input
    match app.active_modules {
        ActiveModules::AtConfig(crate::app::AtConfig::Config) => {
        }
        _ => {}
    }

    // close popup and unset focus to input
    app.input_popup = false;
    app.input.input_mode = InputMode::Normal;
}
```

### 打印数组的值

```bash
`Vec<std::string::String>` doesn't implement `std::fmt::Display`
the trait `std::fmt::Display` is not implemented for `Vec<std::string::String>`
in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

好像是没有基本的`Display`特征，需要使用`{:?}`之类的。

```bash
2023-10-07T14:58:47.071821+08:00 [INFO] Choose enter item: ["ENABLE_AIR724", "false", "Enable Air724 to use AT command"]
```

现在好像是可以了，都采用这种新的变量进行导出，感觉还可以，可以直接匹配！！！

最后还是选择在结构体内部硬编码代码，然后方便使用，这里传入的是String，配合使用parse完成类型转换。
```rs
pub fn update(&mut self, key: String, value: String) {
    match key.as_str() {
        "ENABLE_AIR724" => {
            self.enable_air724.value = value.parse::<bool>().unwrap();
        }
```

### 更新修改的信息

```bash
2023-10-07T15:20:41.87127+08:00 [DEBUG] (1) tosbuilder::handler: Activate modules: AtConfig(Config) Key event: KeyEvent { code: Char('e'), modifiers: NONE, kind: Press, state: NONE }
2023-10-07T15:20:42.126805+08:00 [DEBUG] (1) tosbuilder::handler: Activate modules: AtConfig(Config) Key event: KeyEvent { code: Enter, modifiers: NONE, kind: Press, state: NONE }
2023-10-07T15:20:42.127561+08:00 [INFO] Air724 Value: true
2023-10-07T15:20:42.127802+08:00 [INFO] Enable Air724: true
```

更新一下修改后的信息，现在看起来使用是正常的！！！
```rs
match app.active_modules {
    ActiveModules::AtConfig(crate::app::AtConfig::Config) => {
        let binding = app.at_config_table.at_config.to_vec();
        let idx = app.at_config_table.index.selected().expect("at config table index is none");
        let key = binding[idx][0].clone();
        // let value = binding[idx][1].clone();

        // info!("Choose enter item: {:?}", key);
        // update value
        app.at_config_table.at_config.update(key, app.input.input.clone());
    }
    _ => {}
}
```

会有校验信息，输入错误的话会直接exit退出。。。

### 添加配置文件的序列化和反序列化

在 Rust 中，可以使用多个 derive 宏来自动生成实现特定 trait 的代码。在你提供的代码中，结构体 Download 使用了 #[derive(Debug)] 和 #[derive(Serialize, Deserialize)] 这两个宏。

### 将app的信息序列化到json文件里面！！！

现在显示是可以的！！！重载可能有点问题，但是起码有导出的功能了！！！

```json
{
  "active_modules": {
    "ProjectSelect": "Fs"
  },
  "cube_mx_project_config": {
    "path": "/Users/asklv/Projects/TSOC/TOS-Builder",
    "kind": "GCC"
  },
  "tos_project_config": {
    "path": "/Users/asklv/Projects/TSOC/TOS-Builder",
    "version": "VERSION_2_5_0"
  },
  "tos_header_table": {
    "tos_header_config": {
      "tos_cfg_header_include": {
        "key": "TOS_CFG_HEADER_INCLUDE",
        "value": "stm32l0xx.h",
        "comment": "Target chip header file, user needs to change accordingly"
      },
      "tos_cfg_task_prio_max": {
        "key": "TOS_CFG_TASK_PRIO_MAX",
        "value": "10",
        "comment": "Configure the maximum number of priorities supported by TencentOS tiny by default"
      },
      "tos_cfg_round_robin_en": {
        "key": "TOS_CFG_ROUND_ROBIN_EN",
        "value": false,
        "comment": "Configure TencentOS tiny's kernel to enable or disable time-slice rotation."
      },
      "tos_cfg_object_verify_en": {
        "key": "TOS_CFG_OBJECT_VERIFY_EN",
        "value": true,
        "comment": "Configure whether TencentOS tiny checksums pointers for legality"
      },
      "tos_cfg_task_dynamic_create_en": {
        "key": "TOS_CFG_TASK_DYNAMIC_CREATE_EN",
        "value": true,
        "comment": "TencentOS tiny Dynamic Task Creation Macros"
      },
      "tos_cfg_event_en": {
        "key": "TOS_CFG_EVENT_EN",
        "value": true,
        "comment": "TencentOS tiny Event Module Macros"
      },
      "tos_cfg_mmblk_en": {
        "key": "TOS_CFG_MMBLK_EN",
        "value": true,
        "comment": "Configure TencentOS tiny to enable the memory block management module"
      },
      "tos_cfg_mmheap_en": {
        "key": "TOS_CFG_MMHEAP_EN",
        "value": true,
        "comment": "TencentOS tiny Configure TencentOS tiny to enable the Dynamic Memory Module"
      },
      "tos_cfg_mmheap_default_pool_en": {
        "key": "TOS_CFG_MMHEAP_DEFAULT_POOL_EN",
        "value": true,
        "comment": "TencentOS tiny Default Dynamic Memory Pool Function Macro"
      },
      "tos_cfg_mmheap_default_pool_size": {
        "key": "TOS_CFG_MMHEAP_DEFAULT_POOL_SIZE",
        "value": "256",
        "comment": "Configure the TencentOS tiny default dynamic memory pool size"
      },
      "tos_cfg_mutex_en": {
        "key": "TOS_CFG_MUTEX_EN",
        "value": true,
        "comment": "Configure TencentOS tiny to enable the Mutual Exclusion Lock module"
      },
      "tos_cfg_message_queue_en": {
        "key": "TOS_CFG_MESSAGE_QUEUE_EN",
        "value": true,
        "comment": "Configure TencentOS tiny to enable the Message Queuing module"
      },
      "tos_cfg_mail_queue_en": {
        "key": "TOS_CFG_MAIL_QUEUE_EN",
        "value": true,
        "comment": "Configure TencentOS tiny to enable the Message Mailbox module"
      },
      "tos_cfg_priority_message_queue_en": {
        "key": "TOS_CFG_PRIORITY_MESSAGE_QUEUE_EN",
        "value": true,
        "comment": "Configure TencentOS tiny to enable the Priority Message Queuing Module"
      },
      "tos_cfg_priority_mail_queue_en": {
        "key": "TOS_CFG_PRIORITY_MAIL_QUEUE_EN",
        "value": true,
        "comment": "Configuring TencentOS tiny to enable the Priority Message Mailbox module"
      },
      "tos_cfg_timer_en": {
        "key": "TOS_CFG_TIMER_EN",
        "value": true,
        "comment": "Configuring TencentOS tiny to enable the Software Timer module"
      },
      "tos_cfg_pwr_mgr_en": {
        "key": "TOS_CFG_PWR_MGR_EN",
        "value": true,
        "comment": "Configure TencentOS tiny to enable peripheral power management module"
      },
      "tos_cfg_tickless_en": {
        "key": "TOS_CFG_TICKLESS_EN",
        "value": true,
        "comment": "Configure the Tickless low-power module switch"
      },
      "tos_cfg_sem_en": {
        "key": "TOS_CFG_SEM_EN",
        "value": true,
        "comment": "Configuring TencentOS tiny to enable the semaphore module"
      },
      "tos_cfg_task_stack_draught_depth_detact_en": {
        "key": "TOS_CFG_TASK_STACK_DRAUGHT_DEPTH_DETACT_EN",
        "value": true,
        "comment": "Configure TencentOS tiny to enable task stack depth detection"
      },
      "tos_cfg_fault_backtrace_en": {
        "key": "TOS_CFG_FAULT_BACKTRACE_EN",
        "value": true,
        "comment": "Configure TencentOS tiny to enable anomaly stack backtracking"
      },
      "tos_cfg_idle_task_stk_size": {
        "key": "TOS_CFG_IDLE_TASK_STK_SIZE",
        "value": "128",
        "comment": "Configure the TencentOS tiny idle task stack size"
      },
      "tos_cfg_cpu_tick_per_second": {
        "key": "TOS_CFG_CPU_TICK_PER_SECOND",
        "value": "1000",
        "comment": "Configure the tick frequency of TencentOS tiny"
      },
      "tos_cfg_cpu_clock": {
        "key": "TOS_CFG_CPU_CLOCK",
        "value": "(SystemCoreClock)",
        "comment": "Configure TencentOS tiny CPU frequency"
      },
      "tos_cfg_timer_as_proc": {
        "key": "TOS_CFG_TIMER_AS_PROC",
        "value": true,
        "comment": "Configure whether to configure TIMER to function mode"
      }
    },
    "len": 23
  },
  "at_config_table": {
    "at_config": {
      "enable_air724": {
        "key": "ENABLE_AIR724",
        "value": false,
        "comment": "Enable Air724 to use AT command"
      },
      "enable_bc26": {
        "key": "ENABLE_BC26",
        "value": false,
        "comment": "Enable BC26 to use AT command"
      },
      "enable_bc25_28_95": {
        "key": "ENABLE_BC25_28_95",
        "value": false,
        "comment": "Enable BC25/28/95 to use AT command"
      },
      "enable_bc35_28_95_lwm2m": {
        "key": "ENABLE_BC35_28_95_LWM2M",
        "value": false,
        "comment": "Enable BC35/28/95 LWM2M to use AT command"
      },
      "enable_ec20": {
        "key": "ENABLE_EC20",
        "value": false,
        "comment": "Enable EC20 to use AT command"
      },
      "enable_esp8266": {
        "key": "ENABLE_ESP8266",
        "value": false,
        "comment": "Enable ESP8266 to use AT command"
      },
      "enable_m26": {
        "key": "ENABLE_M26",
        "value": false,
        "comment": "Enable M26 to use AT command"
      },
      "enable_m5310a": {
        "key": "ENABLE_M5310A",
        "value": false,
        "comment": "Enable M5310A to use AT command"
      },
      "enable_m6312": {
        "key": "ENABLE_M6312",
        "value": false,
        "comment": "Enable M6312 to use AT command"
      },
      "enable_sim800a": {
        "key": "ENABLE_SIM800a",
        "value": false,
        "comment": "Enable SIM800A to use AT command"
      },
      "enable_sim7600ce": {
        "key": "ENABLE_SIM7600CE",
        "value": false,
        "comment": "Enable SIM7600CE to use AT command"
      }
    },
    "len": 11
  }
}
```

目前来看运行是正常的，主要是需要添加一些trait默认实现，方便导出数据！！！

```bash
/// Application.
#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    // Is the application running?
    #[serde(skip)]
    pub running: bool,
```

注意，这里的导出数据需要记录！！！