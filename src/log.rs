use chrono::{DateTime, Utc};
use term_painter::ToStyle;
use term_painter::Color::*;

const BOLD_X: char = '\u{1d76c}'; 
const CHECK_MARK: char = '\u{2713}';
const TRIANGLE: char = '\u{25BD}';
const ARROW_RIGHT: char = '\u{2794}';

struct Log {
    log_type: LogType,
    first_character: char,
    message: String
}

impl Log {

    fn new(log_type: LogType, first_character: char, message: String) -> Log {
        Log {
            log_type,
            first_character,
            message
        }
    }

    fn ready_message(&self) -> String {
        let now: DateTime<Utc> = Utc::now(); 
        format!(">>> [{}] [{:?}] - {} {}", now.to_rfc2822(), self.log_type, self.first_character, self.message)
    }
}

#[derive(Debug)]
pub enum LogType {
    TRACE,
    WARNING,
    INFO,
    ERROR
}

pub fn trace(message: &str){
    log(LogType::TRACE, message.to_string());
}

pub fn warning(message: &str){
    log(LogType::WARNING, message.to_string());
}

pub fn info(message: &str){
    log(LogType::INFO, message.to_string());
}

pub fn error(message: &str){
    log(LogType::ERROR, message.to_string());
}

fn log(log_type : LogType, message: String){
    match log_type {
        LogType::TRACE => println!("{}", White.paint(Log::new(log_type, ARROW_RIGHT, message).ready_message())),
        LogType::WARNING => println!("{}", Yellow.paint(Log::new(log_type, TRIANGLE, message).ready_message())),
        LogType::INFO => println!("{}", BrightBlue.paint(Log::new(log_type, CHECK_MARK, message).ready_message())),
        LogType::ERROR => println!("{}", Red.paint(Log::new(log_type, BOLD_X, message).ready_message()))
    }
}