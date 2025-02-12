use crate::utils::file::can_write::can_write_file;
use crate::utils::styles::colors::LogLevel;
use crate::utils::file::can_create::can_create;
use crate::utils::file::can_read::can_read_file;
use crate::utils::file::write::write_file;

pub fn debug_mode(is_debug: bool, content: &String) -> String {
    let info_color = LogLevel::Info.fmt();

    if !is_debug {
        return format!("{} To use debug mode, just typing --debug", info_color);
    }
    
    can_create("hash_log.txt");
    can_read_file("hash_log.txt");
    can_write_file("hash_log.txt");
    write_file("hash_log.txt", content);
    format!("{} Saved log as name hash_log.txt", info_color)
}
