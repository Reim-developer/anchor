use crate::utils::styles::colors::LogLevel;
use std::fs::OpenOptions;
use std::io::Write;

pub fn write_file<Str: AsRef<str>>(file_path: &str, content: Str, success_msg: &str) -> String {
    let error_color = LogLevel::Error.fmt();
    let info_color = LogLevel::Info.fmt();

    match OpenOptions::new().write(true).create(true).open(file_path) {
        Ok(mut file) => match file.write_all(content.as_ref().as_bytes()) {
            Ok(_) => format!("{} {}\n", info_color, success_msg),
            Err(error) => format!("{} Failed create file with error: {}\n", error_color, error),
        },
        Err(error) => format!("{} Failed to open file with error {}", error_color, error),
    }
}
