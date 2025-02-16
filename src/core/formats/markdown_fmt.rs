use std::fs;
use comrak::{markdown_to_html, ComrakOptions};
use html2md;
use crate::utils::{file::write::write_file, styles::colors::LogLevel};

pub fn markdown_format(file_path: &str) -> String {
    let error_color = LogLevel::Error.fmt();

    let markdown_source = match fs::read_to_string(file_path) {
        Ok(content) => {
            if content.trim().is_empty() {
                return format!("{} Markdown source is empty, skipping formatting.", error_color);
            }
            content
        }
        Err(error) => {
            return format!(
                "{} Cannot read Markdown source with error: {}",
                error_color, error
            );
        }
    };

    let html = markdown_to_html(&markdown_source, &ComrakOptions::default());
    let formatted_markdown = html2md::parse_html(&html);
    
    write_file(
        file_path,
        formatted_markdown,
        "Successfully formatted Markdown source!",
    ) 
}
