// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let mut res = String::new();
    let log_level = match level{
        LogLevel::Error => "[ERROR]: ",
        LogLevel::Info => "[INFO]: ",
        LogLevel::Warning => "[WARNING]: ",
    };

    res.push_str(log_level);
    res.push_str(message);
    return res;
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
