use crate::utils::colors::LogLevel;
use md5;
use memmap2::Mmap;
use std::fs::File;

pub fn check_md5(file_path: &str) {
    let error_color = LogLevel::Error.fmt();
    let info_color = LogLevel::Info.fmt();

    let file_target = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("{} Can't read file with error: {}", error_color, error);
            return;
        }
    };

    let mmap = match unsafe { Mmap::map(&file_target) } {
        Ok(_mmap) => _mmap,
        Err(error) => {
            eprintln!("{} Error when memory-mapping file: {}", error_color, error);
            return;
        }
    };

    let digest = md5::compute(&mmap);
    println!("{} MD5: {:?}", info_color, digest);
}
