use crate::utils::file::write::write_file;
use crate::utils::styles::colors::LogLevel;
use memmap2::Mmap;
use quick_xml::{
    events::{BytesDecl, Event},
    Writer,
};
use roxmltree::Document;
use std::{fs::File, io::Cursor};

fn parser_xml(xml_content: &str, error_color: String) -> String {
    match Document::parse(xml_content) {
        Ok(_) => String::new(),
        Err(error) => {
            let position = error.pos();
            return format!(
                "{} XML syntax error at line {}, column {}: {}",
                error_color, position.row, position.col, error
            );
        }
    }
}

pub fn format_xml(file_path: &str) -> String {
    let error_color = LogLevel::Error.fmt();

    let file_target = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            return format!("{} Cannot open file with error {}", error_color, error);
        }
    };

    let mmap = match unsafe { Mmap::map(&file_target) } {
        Ok(map) => map,
        Err(error) => {
            return format!(
                "{} Cannot memory mapping file with error: {}",
                error_color, error
            );
        }
    };

    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 4);
    let mut reader = quick_xml::Reader::from_reader(mmap.as_ref());
    let mut buffer = Vec::new();

    while let Ok(event) = reader.read_event_into(&mut buffer) {
        match event {
            Event::Eof => break,
            Event::Decl(_) => {
                if let Err(error) =
                    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
                {
                    return format!("{} Writting XML failed with error: {}", error_color, error);
                }
            }
            _ => {
                if let Err(error) = writer.write_event(event) {
                    return format!("{} Writting XML failed with error: {}", error_color, error);
                }
            }
        }

        buffer.clear();
    }

    let xml_content = match String::from_utf8(writer.into_inner().into_inner()) {
        Ok(xml) => xml,
        Err(error) => {
            return format!(
                "{} Cannot convert XML content to UTF-8: {}",
                error_color, error
            );
        }
    };

    let check_syntax = parser_xml(&xml_content, error_color);
    if !check_syntax.is_empty() {
        return check_syntax;
    }

    write_file(file_path, xml_content, "Successfully format XML file!")
}
