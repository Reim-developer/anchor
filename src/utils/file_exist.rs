use std::path::Path;

pub fn file_is_exist(file_path: &str) -> bool {
    let file_target = Path::new(file_path);

    match file_target.exists() {
        false => {
            eprintln!("{}: File not found", file_target.to_string_lossy());
            false
        }
        true => true,
    }
}
