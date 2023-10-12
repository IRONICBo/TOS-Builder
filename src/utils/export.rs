use chrono::{Local, TimeZone};
use handlebars::Handlebars;
use log::*;
use regex::Regex;
use std::fs;
use std::io::{BufReader, Stderr};
use std::path::Path;

use std::{error::Error, fs::File};
use tui::backend::CrosstermBackend;

use crate::app::App;
use crate::config::cubemx_config::CubeMXProjectType;
use crate::templates;
use crate::tui::Tui;
use crate::utils::copy::copy_dir_recursive;
use crate::utils::xml_helper::{find_element_value, temp_update_element_value};

pub fn export_project(app: &mut App, tui: &mut Tui<CrosstermBackend<Stderr>>) -> Result<(), Box<dyn Error>> {
    app.export_popup = true;
    tui.draw(app)?;

    // prepare project
    // set arch & board & kernel & osal
    app.export.set_current(2);
    let _ = do_prepare_project(app, tui);
    app.export.set_end_time(Local::now().timestamp() as u64);
    tui.draw(app)?;

    // prepare kernel
    app.export.set_current(3);
    let _ = do_prepare_kernel(app, tui);
    app.export.set_end_time(Local::now().timestamp() as u64);
    tui.draw(app)?;

    // prepare tos header
    app.export.set_current(4);
    let _ = do_prepare_tos_header(app, tui);
    app.export.set_end_time(Local::now().timestamp() as u64);
    tui.draw(app)?;

    // prepare at
    app.export.set_current(5);
    let _ = do_prepeare_at(app, tui);
    app.export.set_end_time(Local::now().timestamp() as u64);
    tui.draw(app)?;

    std::thread::sleep(std::time::Duration::from_secs(1));
    app.export.set_end_time(Local::now().timestamp() as u64);
    app.export_popup = false;
    tui.draw(app)?;

    Ok(())
}

pub fn do_prepare_project(app: &mut App, _tui: &mut Tui<CrosstermBackend<Stderr>>) -> Result<(), Box<dyn Error>> {
    app.export.message = format!("set arch & board & kernel & osal dirs");
    info!("exporting message: {}", app.export.message);

    // get tos project path
    let generated = Path::new(app.cube_mx_project_config.generated.as_str());
    let cubemx_project = Path::new(app.cube_mx_project_config.path.as_str());
    let tos_dir = Path::new(app.tos_project_config.path.as_str());

    // copy arch
    let _ = copy_dir_recursive(tos_dir.join("arch").as_path(), generated.join("arch").as_path());
    info!("copy arch ok...");

    // copy board
    let project_name = cubemx_project.file_name().unwrap().clone().to_string_lossy().to_string();
    info!(
        "copy board ok... {} => {}",
        cubemx_project.to_string_lossy(),
        generated.join("board").join(project_name.clone()).as_path().to_string_lossy()
    );
    let _ = copy_dir_recursive(cubemx_project, generated.join("board").join(project_name).as_path());

    // copy kernel
    let _ = copy_dir_recursive(tos_dir.join("kernel").as_path(), generated.join("kernel").as_path());
    info!("copy kernel ok...");

    // copy osal
    let _ = copy_dir_recursive(tos_dir.join("osal").as_path(), generated.join("osal").as_path());
    info!("copy osal ok...");

    // copy at & devices
    if app.at_config_table.at_config.is_enable() {
        info!("AT frame open");
        // copy sal
        let _ = copy_dir_recursive(tos_dir.join("net").as_path(), generated.join("net").as_path());
        info!("copy at ok...");
        let _ = copy_dir_recursive(tos_dir.join("platform").as_path(), generated.join("platform").as_path());
        info!("copy platform ok...");
    } else {
        info!("AT frame is closed");
    }
    
    Ok(())
}

pub fn do_prepare_kernel(app: &mut App, tui: &mut Tui<CrosstermBackend<Stderr>>) -> Result<(), Box<dyn Error>> {
    app.export.message = format!("set basic kernel");
    info!("exporting message: {}", app.export.message);

    match app.cube_mx_project_config.kind {
        CubeMXProjectType::GCC => {
            generate_gcc_kernel(app, tui)?;
        }
        CubeMXProjectType::MDK => {
            generate_mdk_kernel(app, tui)?;
        }
        CubeMXProjectType::IAR => {
            generate_iar_kernel(app, tui)?;
        }
    }

    Ok(())
}

pub fn do_prepare_tos_header(app: &mut App, _tui: &mut Tui<CrosstermBackend<Stderr>>) -> Result<(), Box<dyn Error>> {
    app.export.message = format!("set tos header");
    info!("exporting message: {}", app.export.message);

    // generate tos header path
    let generated = Path::new(app.cube_mx_project_config.generated.as_str());
    let cubemx_project = Path::new(app.cube_mx_project_config.path.as_str());
    let project_name = cubemx_project.file_name().unwrap().clone().to_string_lossy().to_string();
    fs::create_dir_all(generated.join("board").join(project_name.clone()).join("TOS_CONFIG"))?;

    // generate tos header file & write to path
    let mut tos_header_file = File::create(generated.join("board").join(project_name.clone()).join("TOS_CONFIG").join("tos_config.h"))?;
    let tos_header_template = templates::tos_config::TOS_CONFIG;

    // render to template
    let mut reg = Handlebars::new();
    reg.register_template_string("tos_header", tos_header_template);
    reg.render_to_write("tos_header", &app.tos_header_table.tos_header_config.to_map(), &mut tos_header_file)?;

    Ok(())
}

pub fn do_prepeare_at(app: &mut App, _tui: &mut Tui<CrosstermBackend<Stderr>>) -> Result<(), Box<dyn Error>> {
    app.export.message = format!("set at & devices");
    info!("exporting message: {}", app.export.message);

    Ok(())
}

// Generate GCC kernel
pub fn generate_gcc_kernel(app: &mut App, _tui: &mut Tui<CrosstermBackend<Stderr>>) -> Result<(), Box<dyn Error>> {
    info!("generate gcc kernel");

    let generated = Path::new(app.cube_mx_project_config.generated.as_str());
    let cubemx_project = Path::new(app.cube_mx_project_config.path.as_str());
    let project_name = cubemx_project.file_name().unwrap().clone().to_string_lossy().to_string();
    let makefile_path = generated.join("board").join(project_name.clone()).join("Makefile");

    let mut content = fs::read_to_string(makefile_path.clone()).expect("Failed to read file");

    // add TOP_DIR
    let pattern = Regex::new(format!("TARGET = {}", project_name.clone()).as_str()).expect("Failed to create regex");
    content = pattern
        .replace(
            &content,
            format!(
                r#"TARGET = {}

TOP_DIR = ../../"#,
                project_name.clone()
            ),
        )
        .to_string();

    // add files path
    let pattern = Regex::new(format!("# ASM sources").as_str()).expect("Failed to create regex");
    content = pattern
        .replace(
            &content,
            format!(
                r#"ARCH_SRC = \
$${{wildcard $(TOP_DIR)/arch/{}/{}/{}/*.c}} \
$${{wildcard $(TOP_DIR)/arch/{}/common/*.c}}
C_SOURCES += $(ARCH_SRC)
KERNEL_SRC = \
$${{wildcard $(TOP_DIR)/kernel/core/*.c}} \
$${{wildcard $(TOP_DIR)/kernel/pm/*.c}}
C_SOURCES += $(KERNEL_SRC)
{}
CMSIS_SRC = \
$${{wildcard $(TOP_DIR)/osal/cmsis_os/*.c}}
C_SOURCES += $(CMSIS_SRC)

ASM_SOURCES_S = \
$(TOP_DIR)/arch/{}/{}/{}/port_s.S

# ASM sources"#,
                app.cube_mx_project_config.arch.get_top_arch(app.cube_mx_project_config.kind.as_str().to_string()),
                app.cube_mx_project_config.arch.as_str(),
                app.cube_mx_project_config.kind.get_compiler(),
                app.cube_mx_project_config.arch.get_top_arch(app.cube_mx_project_config.kind.as_str().to_string()),
                generate_gcc_at_sources_path(app),
                app.cube_mx_project_config.arch.get_top_arch(app.cube_mx_project_config.kind.as_str().to_string()),
                app.cube_mx_project_config.arch.as_str(),
                app.cube_mx_project_config.kind.get_compiler()
            ),
        )
        .to_string();

    // add include path
    let pattern = Regex::new(format!("# compile gcc flags").as_str()).expect("Failed to create regex");
    content = pattern
        .replace(
            &content,
            format!(
                r#"KERNEL_INC = \
-I $(TOP_DIR)/kernel/core/include \
-I $(TOP_DIR)/kernel/pm/include \
-I $(TOP_DIR)/arch/{}/common/include \
-I $(TOP_DIR)/arch/{}/{}/{} \
-I $(TOP_DIR)/board/{}/TOS_CONFIG
C_INCLUDES += $(KERNEL_INC)
{}
CMSIS_INC = \
-I $(TOP_DIR)/osal/cmsis_os
C_INCLUDES += $(CMSIS_INC)

# compile gcc flags"#,
                app.cube_mx_project_config.arch.get_top_arch(app.cube_mx_project_config.kind.as_str().to_string()),
                app.cube_mx_project_config.arch.get_top_arch(app.cube_mx_project_config.kind.as_str().to_string()),
                app.cube_mx_project_config.arch.as_str(),
                app.cube_mx_project_config.kind.get_compiler(),
                project_name.clone(),
                generate_gcc_at_include_path(app),
            ),
        )
        .to_string();

    // add compile options
    let pattern = Regex::new(format!("# list of ASM program objects").as_str()).expect("Failed to create regex");
    content = pattern
        .replace(
            &content,
            format!(
                r#"# list of ASM_SOURCES_S program objects

OBJECTS += $(addprefix $(BUILD_DIR)/,$(notdir $(ASM_SOURCES:.s=.o)))
vpath %.S $(sort $(dir $(ASM_SOURCES_S)))

$(BUILD_DIR)/%.o: %.S Makefile | $(BUILD_DIR)
	$(AS) -c $(CFLAGS) $< -o $@
# list of ASM program objects
"#
            ),
        )
        .to_string();

    fs::write(makefile_path, content).expect("Failed to write file");

    Ok(())
}

pub fn generate_gcc_at_sources_path(app: &mut App) -> String {
    info!("generate at sources path");

    let mut at_sources_path = String::new();

    // 1. Add arch common path
    at_sources_path.push_str(
        format!(
            r#"AT_SRC = \
$${{wildcard $(TOP_DIR)/net/at/src/*.c}}
$${{wildcard $(TOP_DIR)/{}/*.c}}
C_SOURCES += $(AT_SRC)
SAL_SRC = \
$${{wildcard $(TOP_DIR)/net/sal_module_wrapper/*.c}}
C_SOURCES += $(SAL_SRC)
DEVICE_SRC = \
$${{wildcard $(TOP_DIR)/{}/*.c}} \
C_SOURCES += $(DEVICE_SRC)
"#,
            app.tos_header_table.tos_header_config.get_at_hal_path(app.cube_mx_project_config.kind.as_str().to_string()),
            app.at_config_table.at_config.get_first_enabled_device_source_path(app.cube_mx_project_config.kind.as_str().to_string()),
        )
        .as_str(),
    );

    info!("at sources path: {}", at_sources_path);

    return at_sources_path;
}

pub fn generate_gcc_at_include_path(app: &mut App) -> String {
  info!("generate at sources path");

  let mut at_sources_path = String::new();

  // 1. Add arch common path
  at_sources_path.push_str(
      format!(
          r#"AT_INC = \
-I $(TOP_DIR)/net/at/include \
-I $(TOP_DIR)/kernel/hal/include
C_INCLUDES += $(AT_INC)
SAL_INC = \
-I $(TOP_DIR)/net/sal_module_wrapper
C_INCLUDES += $(SAL_INC)
DEVICE_INC = \
-I $(TOP_DIR)/{}
C_INCLUDES += $(DEVICE_INC)
"#,
          app.at_config_table.at_config.get_first_enabled_device_source_path(app.cube_mx_project_config.kind.as_str().to_string()),
      )
      .as_str(),
  );

  info!("at sources path: {}", at_sources_path);

  return at_sources_path;
}

// Generate MDK kernel
pub fn generate_mdk_kernel(app: &mut App, _tui: &mut Tui<CrosstermBackend<Stderr>>) -> Result<(), Box<dyn Error>> {
    info!("generate mdk kernel");

    let generated = Path::new(app.cube_mx_project_config.generated.as_str());
    let cubemx_project = Path::new(app.cube_mx_project_config.path.as_str());
    let project_name = cubemx_project.file_name().unwrap().clone().to_string_lossy().to_string();
    let mdk_filepath = generated.join("board").join(project_name.clone()).join("MDK-ARM").join(format!("{}.uvprojx", project_name.clone()));

    let file = File::open(mdk_filepath.clone()).expect("Failed to open file");
    let reader = BufReader::new(file);

    // 1. Add header path
    // Targets -> Target -> TargetOption -> TargetArmAds -> ArmAdsMisc -> Cads -> VariousControls -> IncludePath
    let target_path: Vec<&str> = vec!["Targets", "Target", "TargetOption", "TargetArmAds", "Cads", "VariousControls", "IncludePath"];
    let include_path_value = find_element_value(reader, &target_path);

    // Add include path
    let new_include_header_path_value = format!("{}{}", include_path_value, generate_mdk_include_header_path(app).as_str(),);
    let _ = temp_update_element_value(
        mdk_filepath.clone().to_str().unwrap(),
        mdk_filepath.clone().to_str().unwrap(),
        include_path_value.as_str(),
        new_include_header_path_value.as_str(),
    );

    // 2. Add include path
    // Replace </Groups> to xxx</Groups> to add include path
    // </Groups> is unique
    let new_include_file_path_value = format!("{}{}", generate_mdk_include_files_path(app).as_str(), "</Groups>",);
    let _ = temp_update_element_value(
        mdk_filepath.clone().to_str().unwrap(),
        mdk_filepath.clone().to_str().unwrap(),
        "</Groups>",
        new_include_file_path_value.as_str(),
    );

    Ok(())
}

// Generate IAR kernel
// TODO: Update xml finder
pub fn generate_iar_kernel(app: &mut App, _tui: &mut Tui<CrosstermBackend<Stderr>>) -> Result<(), Box<dyn Error>> {
    info!("generate iar kernel");

    let generated = Path::new(app.cube_mx_project_config.generated.as_str());
    let cubemx_project = Path::new(app.cube_mx_project_config.path.as_str());
    let project_name = cubemx_project.file_name().unwrap().clone().to_string_lossy().to_string();
    let iar_filepath = generated.join("board").join(project_name.clone()).join("EWARM").join(format!("{}.ewp", project_name.clone()));

    let file = File::open(iar_filepath.clone()).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut content = fs::read_to_string(iar_filepath.clone()).expect("Failed to read file");

    // Add include path
    let pattern = Regex::new(format!(r"../Core/Inc</state>").as_str()).expect("Failed to create regex");
    let new_include_header_path_value = format!("{}\n{}", "../Core/Inc</state>", generate_iar_include_header_path(app).as_str(),);
    content = pattern.replace(&content, new_include_header_path_value).to_string();

    
    // Add include path
    let pattern = Regex::new(format!(r"<group>").as_str()).expect("Failed to create regex");
    let new_include_header_path_value = format!("{}\n{}", r"<group>", generate_iar_include_files_path(app).as_str());
    content = pattern.replace(&content, new_include_header_path_value).to_string();

    fs::write(iar_filepath, content).expect("Failed to write file");

    Ok(())
}

// MDK and IAR for windows path
pub fn generate_iar_include_header_path(app: &mut App) -> String {
  info!("generate include path");

  let mut include_path = String::new();

  // 1. Add arch common path
  include_path.push_str(
      format!(
          r#"<state>$$PROJ_DIR$$/../../../arch/{}/common/include</state>"#,
          app.cube_mx_project_config.arch.get_top_arch(CubeMXProjectType::GCC.as_str().to_string())
      )
      .as_str(),
  );
  include_path.push_str(
      format!(
          r#"<state>$$PROJ_DIR$$/../../../arch/{}/{}/{}</state>"#,
          app.cube_mx_project_config.arch.get_top_arch(app.cube_mx_project_config.kind.as_str().to_string()),
          app.cube_mx_project_config.arch.as_str(),
          app.cube_mx_project_config.kind.get_compiler()
      )
      .as_str(),
  );
  include_path.push_str(format!(r#"<state>$$PROJ_DIR$$/../../../kernel/core/include</state>"#).as_str());
  include_path.push_str(format!(r#"<state>$$PROJ_DIR$$/../../../kernel/pm/include</state>"#).as_str());
  include_path.push_str(format!(r#"<state>$$PROJ_DIR$$/../../../osal/cmsis_os</state>"#).as_str());
  include_path.push_str(format!(r#"<state>$$PROJ_DIR$$/../TOS_CONFIG</state>"#).as_str());

  // TODO: Add at path

  info!("include path: {}", include_path);

  return include_path;
}

pub fn generate_iar_include_files_path(app: &mut App) -> String {
  info!("generate include path");

  let mut include_path = String::new();

  include_path.push_str(
      format!(
          r#"<group>
<name>tos/arch</name>
<file>
  <name>$$PROJ_DIR$$/../../../arch/{}/common/tos_cpu.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../arch/{}/{}/{}/port_c.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../arch/{}/{}/{}/port_s.S</name>
</file>
</group>"#,
          app.cube_mx_project_config.arch.get_top_arch(app.cube_mx_project_config.kind.as_str().to_string()),
          app.cube_mx_project_config.arch.get_top_arch(app.cube_mx_project_config.kind.as_str().to_string()),
          app.cube_mx_project_config.arch.as_str(),
          app.cube_mx_project_config.kind.get_compiler(),
          app.cube_mx_project_config.arch.get_top_arch(app.cube_mx_project_config.kind.as_str().to_string()),
          app.cube_mx_project_config.arch.as_str(),
          app.cube_mx_project_config.kind.get_compiler()
      )
      .as_str(),
  );

  include_path.push_str(
      format!(
          r#"<Group>
<name>tos/kernel</name>
<files>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_barrier.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_binary_heap.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_bitmap.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_char_fifo.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_completion.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_countdownlatch.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_event.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_global.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_mail_queue.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_message_queue.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_mmblk.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_mmheap.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_mutex.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_pend.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_priority_mail_queue.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_priority_message_queue.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_priority_queue.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_ring_queue.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_robin.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_rwlock.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_sched.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_sem.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_stopwatch.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_sys.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_task.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_tick.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_time.c</name>
</file>
<file>
  <name>$$PROJ_DIR$$/../../../kernel/core/tos_timer.c</name>
</file>
</group>
<group>
<name>TOS-CONFIG</name>
<file>
  <name>$$PROJ_DIR$$/../TOS_CONFIG/tos_config.h</name>
</file>
</group>"#
      )
      .as_str(),
  );

  return include_path;
}

// MDK and IAR for windows path
pub fn generate_mdk_include_header_path(app: &mut App) -> String {
    info!("generate include path");

    let mut include_path = String::new();

    // ;..\..\..\arch\arm\arm-v7m\common\include;..\..\..\arch\arm\arm-v7m\cortex-m4\armcc;..\..\..\kernel\core\include;..\..\..\kernel\pm\include;..\..\..\osal\cmsis_os;..\TOS_CONFIG
    // 1. Add arch common path
    include_path.push_str(
        format!(
            r#";..\..\..\arch\{}\common\include"#,
            app.cube_mx_project_config.arch.get_top_arch(app.cube_mx_project_config.kind.as_str().to_string())
        )
        .as_str(),
    );
    include_path.push_str(
        format!(
            r#";..\..\..\arch\{}\{}\{}"#,
            app.cube_mx_project_config.arch.get_top_arch(app.cube_mx_project_config.kind.as_str().to_string()),
            app.cube_mx_project_config.arch.as_str(),
            app.cube_mx_project_config.kind.get_compiler()
        )
        .as_str(),
    );
    include_path.push_str(format!(r#";..\..\..\kernel\core\include"#).as_str());
    include_path.push_str(format!(r#";..\..\..\kernel\pm\include"#).as_str());
    include_path.push_str(format!(r#";..\..\..\osal\cmsis_os"#).as_str());
    include_path.push_str(format!(r#"..\TOS_CONFIG"#).as_str());

    // TODO: Add at path

    return include_path;
}

pub fn generate_mdk_include_files_path(app: &mut App) -> String {
    info!("generate include path");

    let mut include_path = String::new();

    include_path.push_str(
        format!(
            r#"<Group>
<GroupName>tos/arch</GroupName>
<Files>
  <File>
    <FileName>tos_cpu.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\arch\{}\common\tos_cpu.c</FilePath>
  </File>
  <File>
    <FileName>port_c.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\arch\{}\{}\{}\port_c.c</FilePath>
  </File>
  <File>
    <FileName>port_s.S</FileName>
    <FileType>2</FileType>
    <FilePath>..\..\..\arch\{}\{}\{}\port_s.S</FilePath>
  </File>
</Files>
</Group>"#,
            app.cube_mx_project_config.arch.get_top_arch(app.cube_mx_project_config.kind.as_str().to_string()),
            app.cube_mx_project_config.arch.get_top_arch(app.cube_mx_project_config.kind.as_str().to_string()),
            app.cube_mx_project_config.arch.as_str(),
            app.cube_mx_project_config.kind.get_compiler(),
            app.cube_mx_project_config.arch.get_top_arch(app.cube_mx_project_config.kind.as_str().to_string()),
            app.cube_mx_project_config.arch.as_str(),
            app.cube_mx_project_config.kind.get_compiler()
        )
        .as_str(),
    );

    include_path.push_str(
        format!(
            r#"<Group>
<GroupName>tos/kernel</GroupName>
<Files>
  <File>
    <FileName>tos_barrier.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_barrier.c</FilePath>
  </File>
  <File>
    <FileName>tos_binary_heap.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_binary_heap.c</FilePath>
  </File>
  <File>
    <FileName>tos_bitmap.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_bitmap.c</FilePath>
  </File>
  <File>
    <FileName>tos_char_fifo.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_char_fifo.c</FilePath>
  </File>
  <File>
    <FileName>tos_completion.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_completion.c</FilePath>
  </File>
  <File>
    <FileName>tos_countdownlatch.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_countdownlatch.c</FilePath>
  </File>
  <File>
    <FileName>tos_event.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_event.c</FilePath>
  </File>
  <File>
    <FileName>tos_global.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_global.c</FilePath>
  </File>
  <File>
    <FileName>tos_mail_queue.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_mail_queue.c</FilePath>
  </File>
  <File>
    <FileName>tos_message_queue.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_message_queue.c</FilePath>
  </File>
  <File>
    <FileName>tos_mmblk.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_mmblk.c</FilePath>
  </File>
  <File>
    <FileName>tos_mmheap.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_mmheap.c</FilePath>
  </File>
  <File>
    <FileName>tos_mutex.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_mutex.c</FilePath>
  </File>
  <File>
    <FileName>tos_pend.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_pend.c</FilePath>
  </File>
  <File>
    <FileName>tos_priority_mail_queue.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_priority_mail_queue.c</FilePath>
  </File>
  <File>
    <FileName>tos_priority_message_queue.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_priority_message_queue.c</FilePath>
  </File>
  <File>
    <FileName>tos_priority_queue.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_priority_queue.c</FilePath>
  </File>
  <File>
    <FileName>tos_ring_queue.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_ring_queue.c</FilePath>
  </File>
  <File>
    <FileName>tos_robin.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_robin.c</FilePath>
  </File>
  <File>
    <FileName>tos_rwlock.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_rwlock.c</FilePath>
  </File>
  <File>
    <FileName>tos_sched.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_sched.c</FilePath>
  </File>
  <File>
    <FileName>tos_sem.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_sem.c</FilePath>
  </File>
  <File>
    <FileName>tos_stopwatch.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_stopwatch.c</FilePath>
  </File>
  <File>
    <FileName>tos_sys.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_sys.c</FilePath>
  </File>
  <File>
    <FileName>tos_task.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_task.c</FilePath>
  </File>
  <File>
    <FileName>tos_tick.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_tick.c</FilePath>
  </File>
  <File>
    <FileName>tos_time.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_time.c</FilePath>
  </File>
  <File>
    <FileName>tos_timer.c</FileName>
    <FileType>1</FileType>
    <FilePath>..\..\..\kernel\core\tos_timer.c</FilePath>
  </File>
</Files>
</Group>
<Group>
<GroupName>TOS-CONFIG</GroupName>
<Files>
  <File>
    <FileName>tos_config.h</FileName>
    <FileType>5</FileType>
    <FilePath>..\TOS_CONFIG\tos_config.h</FilePath>
  </File>
</Files>
</Group>"#
        )
        .as_str(),
    );

    return include_path;
}

mod tests {
    use std::{
        fs::{self, File},
        io::BufReader,
        path::Path,
    };

    use regex::Regex;
    use xml::{reader::XmlEvent, EventReader};

    #[test]
    fn test_get_include_path() {
        let mdk_filepath = Path::new("/Users/asklv/TOS_Test/generated/board/Tencentos-tiny/MDK-ARM/Tencentos-tiny.uvprojx");

        let file = File::open(mdk_filepath).expect("Failed to open file");
        let reader: BufReader<File> = BufReader::new(file);

        let parser = EventReader::new(reader);
        let mut depth = 0;

        let mut current_element = String::new();
        let mut inside_target_element = false;

        // finite-state machine
        let mut targets_level = 0;

        for e in parser {
            // Targets -> Target -> TargetOption -> TargetArmAds -> ArmAdsMisc -> Cads -> VariousControls -> IncludePath
            // Find IncludePath
            match e {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    println!("------ Level {}", targets_level);
                    match name.local_name.as_str() {
                        "Targets" => targets_level += 1,
                        "Target" => targets_level += 1,
                        "TargetOption" => targets_level += 1,
                        "TargetArmAds" => targets_level += 1,
                        "ArmAdsMisc" => targets_level += 1,
                        "Cads" => targets_level += 1,
                        "VariousControls" => targets_level += 1,
                        "IncludePath" => targets_level += 1,
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name }) => match name.local_name.as_str() {
                    "Targets" => targets_level -= 1,
                    "Target" => targets_level -= 1,
                    "TargetOption" => targets_level -= 1,
                    "TargetArmAds" => targets_level -= 1,
                    "ArmAdsMisc" => targets_level -= 1,
                    "Cads" => targets_level -= 1,
                    "VariousControls" => targets_level -= 1,
                    "IncludePath" => targets_level -= 1,
                    _ => {}
                },
                Ok(XmlEvent::Characters(text)) if targets_level == 7 => {
                    println!("Element '{}' Value: {} Level {}", current_element, text, targets_level);
                    // Element 'ScatterFile' Value: stm32wle5xx_flash.sct
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    break;
                }
                // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
                _ => {}
            }
        }
    }

    #[test]
    fn test_print_xml() {
        let mdk_filepath = Path::new("/Users/asklv/TOS_Test/generated/board/Tencentos-tiny/MDK-ARM/Tencentos-tiny.uvprojx");

        let file = File::open(mdk_filepath).expect("Failed to open file");
        let reader = BufReader::new(file);

        let parser = EventReader::new(reader);
        let mut depth = 0;

        let mut current_element = String::new();
        let mut inside_target_element = false;

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    let vec = attributes.to_vec();
                    print!("{:?}", vec);
                    println!("{:spaces$}+{name}:{depth}", "", spaces = depth * 2);
                    depth += 1;

                    if name.local_name == "IncludePath" {
                        inside_target_element = true;
                    }
                    current_element = name.local_name;
                }
                Ok(XmlEvent::EndElement { name }) => {
                    depth -= 1;
                    println!("{:spaces$}-{name}", "", spaces = depth * 2);
                    if name.local_name == "IncludePath" {
                        inside_target_element = false;
                    }
                    current_element.clear();
                }
                Ok(XmlEvent::Characters(text)) if inside_target_element => {
                    println!("Element '{}' Value: {}", current_element, text);
                    // Element 'ScatterFile' Value: stm32wle5xx_flash.sct
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    break;
                }
                // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
                _ => {}
            }
        }
    }

    #[test]
    fn test_regex() {
        let mut content = fs::read_to_string("/Users/asklv/WebDownloads/tos/Tencentos_tiny_gcc_ori/Tencentos_tiny_gcc/Makefile").expect("Failed to read file");
        let pattern = Regex::new(r#"# C sources(.*?)# ASM sources"#).expect("Failed to create regex");

        print!("{}", content);
    }
}
