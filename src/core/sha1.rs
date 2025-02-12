use crate::utils::styles::colors::LogLevel;
use memmap2::{self, Mmap};
use sha1::{Digest, Sha1};
use std::fs::File;

pub fn check_sha1(file_path: &str) -> String {
    let error_color = LogLevel::Error.fmt();
    let info_color = LogLevel::Info.fmt();

    let file_target = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            return format!("{} Can't read file with error: {}", error_color, error);
        }
    };

    let mmap = match unsafe { Mmap::map(&file_target) } {
        Ok(mmap) => mmap,
        Err(error) => {
            return format!("{} Error when memory-mapping file {}", error_color, error);
        }
    };

    let mut sha1 = Sha1::new();
    sha1.update(&mmap);
    format!("{} SHA1: {:x}\n", info_color, sha1.finalize())
}
