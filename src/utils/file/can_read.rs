use std::fs::File;
use crate::utils::styles::colors::LogLevel;
pub fn can_read_file(file_path: &str) -> String {
    let error_color = LogLevel::Error.fmt();
    let info_color = LogLevel::Info.fmt();
    
    match File::open(file_path) {
       Ok(_) => format!("{} Checking read permission ... ", info_color),
        Err(error) => format!("{} Can't read file with error: {}", error_color, error)
    }
}
