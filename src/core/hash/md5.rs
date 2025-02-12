use crate::utils::styles::colors::LogLevel;
use md5;
use memmap2::Mmap;
use std::fs::File;

pub fn check_md5(file_path: &str) -> String {
    let error_color = LogLevel::Error.fmt();
    let info_color = LogLevel::Info.fmt();

    let file_target = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            return format!("{} Can't read file with error: {}", error_color, error);
        }
    };

    let mmap = match unsafe { Mmap::map(&file_target) } {
        Ok(_mmap) => _mmap,
        Err(error) => {
            return format!("{} Error when memory-mapping file: {}", error_color, error);
        }
    };

    let digest = md5::compute(&mmap);
    format!("{} MD5: {:?}\n", info_color, digest)
}
