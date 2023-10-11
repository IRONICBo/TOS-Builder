use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};
use std::fs::File;

pub fn init_logger() {
    // Set default config, default log level is off
    let config = ConfigBuilder::new()
    .set_time_format_rfc3339()
    .set_time_offset_to_local()
    .expect("Cannot not init logger config")
    .build();

    // Create config file
    let file_name = "tos_builder.log";
    let log_file = File::create(file_name).expect("Cannot create log file");

    // Save current running log
    // TODO: Set dynamic log level
    WriteLogger::init(LevelFilter::Debug, config, log_file).expect("Cannot init logger");
}
