use super::get_exts::get_file_ext;
use crate::core::formats::json_fmt::json_formatter;
use crate::core::formats::xml_fmt::format_xml;

pub enum FileTypes {
    Json,
    Xml,
    Unknow,
}

impl FileTypes {
    pub fn where_file(file_path: &str) -> Self {
        match get_file_ext(file_path).as_ref() {
            "json" => Self::Json,
            "xml" => Self::Xml,
            _ => Self::Unknow,
        }
    }

    pub fn format_file(&self, file_path: &str) -> String {
        match self {
            Self::Json => json_formatter(file_path),
            Self::Xml => format_xml(file_path),
            Self::Unknow => format!("Unknow file extension, skipping.."),
        }
    }
}
