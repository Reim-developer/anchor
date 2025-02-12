use crate::utils::styles::colors::LogLevel;
use std::path::Path;

pub fn file_is_exist(file_path: &str) -> bool {
    let file_target = Path::new(file_path);

    let error_color = LogLevel::Error.fmt();
    match file_target.exists() {
        false => {
            eprintln!(
                "{} {}: File not found",
                error_color,
                file_target.to_string_lossy()
            );
            false
        }
        true => true,
    }
}
