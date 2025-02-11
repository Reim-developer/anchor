use crate::core::md5::check_md5;
use crate::utils::cant_read::can_read_file;
use crate::utils::file_exist::file_is_exist;
use crate::utils::is_dir::is_directory;

pub fn hash_command(file_path: &str) {
    if is_directory(file_path) || !file_is_exist(file_path) {
        return;
    }

    if !can_read_file(file_path) {
        return;
    }

    println!("Check sum:");
    check_md5(file_path);
}
