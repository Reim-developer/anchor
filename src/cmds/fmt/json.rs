use crate::core::formats::json_fmt::json_formatter;
use crate::utils::time::time_calc::cal_time;

pub fn json_fmt(file_path: &str) {
    cal_time(|| {
        let mut result = String::new();

        result.push_str(&json_formatter(file_path));
        result
    });
}
