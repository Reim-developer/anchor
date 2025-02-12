use std::fs::File;

use memmap2::Mmap;
use sha1::Digest;

use crate::utils::styles::colors::LogLevel;

pub fn check_sha256(file_path: &str) -> String {
    let error_color = LogLevel::Error.fmt();
    let info_color = LogLevel::Info.fmt();

    let file_target = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => return format!("{} Can't open file with error: {}", error_color, error),
    };

    let mmap_file = match unsafe { Mmap::map(&file_target) } {
        Ok(mmap_file) => mmap_file,
        Err(error) => {
            return format!(
                "{} Can't memory-mapping file with error {}",
                error_color, error
            )
        }
    };

    let mut sha256 = sha2::Sha256::new();
    sha256.update(&mmap_file);

    format!("{} SHA256: {:x}\n", info_color, sha256.finalize())
}
