#[cfg(debug_assertions)]
use log::LevelFilter;
#[cfg(debug_assertions)]
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode, WriteLogger,
};
#[cfg(debug_assertions)]
use std::fs::OpenOptions;

#[cfg(debug_assertions)]
pub fn init_logger() -> Result<(), String> {
    let logger_config = ConfigBuilder::new()
        .add_filter_allow_str("youtube-dl-gui")
        .add_filter_allow_str("youtube_dl_gui")
        .add_filter_allow_str("youtube_dl_parser")
        .add_filter_allow_str("youtube-dl-parser")
        .build();

    // Delete the previous logs each time the app is launched
    // This is because the logs file can get pretty large
    let logs_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("logs.txt")
        .map_err(|err| format!("Could not open or create logs file: {err}"))?;

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::max(),
            logger_config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::max(), logger_config, logs_file),
    ])
    .map_err(|err| format!("Cannot initialize logger: {err}"))
}

#[cfg(not(debug_assertions))]
pub fn init_logger() -> Result<(), String> {
    Ok(())
}
