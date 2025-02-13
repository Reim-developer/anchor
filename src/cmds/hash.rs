use crate::core::hash::md5::check_md5;
use crate::core::hash::sha1::check_sha1;
use crate::core::hash::sha256::check_sha256;
use crate::core::hash::sha512::check_sha512;
use crate::utils::file::file_exist::file_is_exist;
use crate::utils::file::is_dir::is_directory;
use crate::utils::mode::cmd_mode::debug_mode;
use crate::utils::time::time_calc::cal_time;
use rayon::join;

pub fn hash_command(file_path: &str, is_debug_mode: bool) {
    if is_directory(file_path) || !file_is_exist(file_path) {
        return;
    }

    cal_time(|| {
        let mut result = String::with_capacity(256);

        let (md5, sha1) = join(|| check_md5(file_path), || check_sha1(file_path));
        let (sha256, sha512) = join(|| check_sha256(file_path), || check_sha512(file_path));

        result.push_str(&md5);
        result.push_str(&sha1);
        result.push_str(&sha256);
        result.push_str(&sha512);
        result.push_str(&debug_mode(is_debug_mode, &result));
        result
    });
}
