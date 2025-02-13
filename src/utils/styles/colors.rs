use owo_colors::OwoColorize;

#[derive(Debug)]
pub enum LogLevel {
    Info,
    Error,
    Success,
}

impl LogLevel {
    pub fn fmt(&self) -> String {
        match self {
            LogLevel::Info => "[INFO]".bright_black().on_bright_cyan().bold().to_string(),
            LogLevel::Error => "[ERR]".bright_black().on_bright_red().bold().to_string(),
            LogLevel::Success => "[SUCCESS]"
                .bright_black()
                .on_bright_green()
                .bold()
                .to_string(),
        }
    }
}
