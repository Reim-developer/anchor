use std::fs;

use crate::utils::file::write::write_file;
use crate::utils::styles::colors::LogLevel;
use yaml_rust::YamlLoader;

pub fn yaml_fmt(file_path: &str) -> String {
    let error_color = LogLevel::Error.fmt();

    let yaml_source = match fs::read_to_string(file_path) {
        Ok(source) => source,
        Err(error) => {
            return format!("{} Cannot open file with error: {}", error_color, error);
        }
    };

    match YamlLoader::load_from_str(&yaml_source) {
        Ok(yaml_source) => yaml_source,
        Err(error) => {
            return format!("{} YAML syntax error: {}", error_color, error);
        }
    };

    let yaml_value: serde_yaml::Value = match serde_yaml::from_str(&yaml_source) {
        Ok(yaml_value) => yaml_value,
        Err(error) => {
            return format!("{} Cannot get YAML value with error {}", error_color, error);
        }
    };

    match serde_yaml::to_string(&yaml_value) {
        Ok(yaml_format) => write_file(file_path, yaml_format, "Successfully format YAML file!"),
        Err(error) => {
            return format!(
                "{} Cannot format YAML file with error: {}",
                error_color, error
            );
        }
    }
}
