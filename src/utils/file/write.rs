use std::fs::OpenOptions;
use std::io::Write;

pub fn write_file <Str: AsRef<str>>(file_path: &str, content: Str) -> String{
    match OpenOptions::new().write(true).create(true).open(file_path) {
        Ok(mut file) => match file.write_all(content.as_ref().as_bytes()) {
            Ok(_) => format!("Successfully create log file"),
            Err(error) => format!("Failed create log file with error: {}", error)
        },
        Err(error) => format!("Failed to open file with error {}", error)
    }
}