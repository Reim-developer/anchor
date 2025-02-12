use crate::core::md5::check_md5;
use crate::core::sha1::check_sha1;
use crate::utils::file::can_read::can_read_file;
use crate::utils::file::file_exist::file_is_exist;
use crate::utils::file::is_dir::is_directory;
use crate::utils::time::time_calc::cal_time;

pub fn hash_command(file_path: &str) {
    if is_directory(file_path) || !file_is_exist(file_path) {
        return;
    }

    if !can_read_file(file_path) {
        return;
    }

    cal_time(|| {
        let mut result = String::new();
        result.push_str(&check_md5(file_path));
        result.push_str(&check_sha1(file_path));
        result
    });
}
