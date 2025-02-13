use crate::utils::file::can_create::can_create;
use crate::utils::file::can_read::can_read_file;
use crate::utils::file::can_write::can_write_file;
use crate::utils::file::write::write_file;
use crate::utils::styles::colors::LogLevel;
use rayon::join;

pub fn debug_mode(is_debug: bool, content: &String) -> String {
    let info_color = LogLevel::Info.fmt();
    let success_color = LogLevel::Success.fmt();

    if !is_debug {
        return format!("{} To use debug mode, just typing --debug", info_color);
    }

    let mut result = String::new();

    let (create_check, read_check) = join(
        || can_create("hash_log.txt"),
        || can_read_file("hash_log.txt"),
    );
    let (write_check, write_task) = join(
        || can_write_file("hash_log.txt"),
        || {
            write_file(
                "hash_log.txt",
                content,
                "Successfully write all bytes to file!",
            )
        },
    );

    result.push_str(&create_check);
    result.push_str(&read_check);
    result.push_str(&write_check);
    result.push_str(&write_task);
    result.push_str(&format!("{} Saved log as name hash_log.txt", success_color));

    result
}
