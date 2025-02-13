use crate::utils::file::write::write_file;
use crate::utils::styles::colors::LogLevel;
use memmap2::Mmap;
use serde_json::Value;
use std::fs::File;

pub fn json_formatter(file_path: &str) -> String {
    let error_color = LogLevel::Error.fmt();

    let file_target = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            return format!("{} Cannot open file with error: {}\n", error_color, error);
        }
    };

    let mmap = match unsafe { Mmap::map(&file_target) } {
        Ok(mmap) => mmap,
        Err(error) => {
            return format!(
                "{} Cannot memory-mapping file with error {}\n",
                error_color, error
            );
        }
    };

    let json_string = match std::str::from_utf8(&mmap) {
        Ok(json_string) => json_string,
        Err(error) => {
            return format!("{} Cannot resolve file not UTF-8: {}\n", error_color, error);
        }
    };

    let json_fmt: Value = match serde_json::from_str(json_string) {
        Ok(json_fmt) => json_fmt,
        Err(error) => {
            return format!("{} Cannot parser JSON with error: {}\n", error_color, error);
        }
    };

    match serde_json::to_string_pretty(&json_fmt) {
        Ok(json_format) => {
            return format!(
                "{}",
                write_file(file_path, &json_format, "Successfully format JSON file!")
            );
        }
        Err(error) => {
            return format!(
                "{} Cannot format JSON file with error: {}\n",
                error_color, error
            );
        }
    }
}
