use std::path::Path;

pub fn is_directory(file_path: &str) -> bool {
    let file_target = Path::new(file_path);

    match file_target.is_dir() {
        true => match file_target.file_name() {
            Some(folder_name) => {
                println!("{}: Is a directory", folder_name.to_string_lossy());
                true
            }
            None => {
                eprintln!("Can't get directory name");
                true
            }
        },
        false => false,
    }
}
