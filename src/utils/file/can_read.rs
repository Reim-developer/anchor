use std::fs::File;

pub fn can_read_file(file_path: &str) -> bool {
    match File::open(file_path).is_ok() {
        false => {
            eprintln!("[ERR] Can't read {}: Permission denied", file_path);
            false
        }
        true => true,
    }
}
