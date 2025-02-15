use crate::core::formats::auto_fmt::auto_formats_file;
use crate::utils::time::time_calc::cal_time;

pub fn fmt(file_path: &str) {
    cal_time(|| {
        let mut result = String::new();
        result.push_str(&auto_formats_file(file_path));
        result
    });
}
