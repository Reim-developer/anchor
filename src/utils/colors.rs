use owo_colors::OwoColorize;

pub enum LogLevel {
    Info,
    Error,
}

impl LogLevel {
    pub fn fmt(&self) -> String {
        match self {
            LogLevel::Info => "[INFO]".on_blue().bold().to_string(),
            LogLevel::Error => "[ERR]".on_red().bold().to_string(),
        }
    }
}
