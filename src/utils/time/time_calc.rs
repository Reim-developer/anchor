use indicatif::{ProgressBar, ProgressStyle};

use crate::utils::styles::colors::LogLevel;
use std::time::{Duration, Instant};

pub fn cal_time<Func>(func: Func)
where
    Func: FnOnce() -> String,
{
    let status_bar = ProgressBar::new_spinner();
    let error_color = LogLevel::Error.fmt();

    match ProgressStyle::default_spinner()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
        .template("{spinner} {msg}")
    {
        Ok(style) => status_bar.set_style(style),
        Err(error) => {
            eprintln!("{} Failed to set process bar: {}", error_color, error);
            return;
        }
    }

    status_bar.set_message("Processing....");
    status_bar.enable_steady_tick(Duration::from_millis(100));

    let info_color = LogLevel::Info.fmt();
    let start = Instant::now();

    let result = func();

    status_bar.finish_and_clear();
    let end_time = start.elapsed();

    status_bar.println(result);
    status_bar.println(format!("{} Finished in {:.2?}", info_color, end_time));
}
