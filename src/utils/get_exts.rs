use crate::utils::styles::colors::LogLevel;
use std::borrow::Cow;
use std::path::Path;

pub fn get_file_ext(file_path: &str) -> Cow<str> {
    let error_color = LogLevel::Error.fmt();

    match Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
    {
        Some(ext) => Cow::Borrowed(ext),
        None => Cow::Owned(format!("{} Cannot resolve file extension", error_color)),
    }
}
