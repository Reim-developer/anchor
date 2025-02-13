use crate::utils::styles::colors::LogLevel;
use std::fs::File;

pub fn can_create(file_path: &str) -> String {
    let error_color = LogLevel::Error.fmt();
    let info_color = LogLevel::Info.fmt();

    match File::create(file_path) {
        Ok(_) => format!("{} Successfully created log file\n", info_color),
        Err(error) => format!("{} Can't create file with error: {}\n", error_color, error),
    }
}
