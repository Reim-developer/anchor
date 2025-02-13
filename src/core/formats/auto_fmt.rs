use crate::utils::formats::FileTypes;

pub fn auto_formats_file(file_path: &str) -> String {
    let format_types = FileTypes::where_file(file_path);
    format_types.format_file(file_path)
}
