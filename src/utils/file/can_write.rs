use crate::utils::styles::colors::LogLevel;
use std::fs::OpenOptions;

pub fn can_write_file(file_path: &str) -> String {
    let error_color = LogLevel::Error.fmt();
    let info_color = LogLevel::Info.fmt();

    match OpenOptions::new().write(true).open(file_path) {
        Ok(_) => format!("{} Checking write permission ... Done!", info_color),
        Err(error) => {
            format!("{} Can't write log file with error {}", error_color, error)
        }
    }
}
