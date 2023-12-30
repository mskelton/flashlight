use console::style;
use swc_common::Loc;

use crate::utils::absolute_path;

pub trait Logger {
    fn log(&mut self, text: String, loc: Loc);
}

pub struct ConsoleLogger;
pub struct QuickfixLogger;
pub struct JsonLogger;

pub enum LoggerType {
    Console(ConsoleLogger),
    Quickfix(QuickfixLogger),
    Json(JsonLogger),
}

impl Logger for LoggerType {
    fn log(&mut self, text: String, loc: Loc) {
        match self {
            LoggerType::Console(logger) => logger.log(text, loc),
            LoggerType::Quickfix(logger) => logger.log(text, loc),
            LoggerType::Json(logger) => logger.log(text, loc),
        }
    }
}

impl ConsoleLogger {
    pub fn new() -> ConsoleLogger {
        ConsoleLogger
    }
}

impl Logger for ConsoleLogger {
    fn log(&mut self, text: String, loc: Loc) {
        println!(
            "{}:{}:{} {}",
            loc.file.name,
            loc.line,
            loc.col.0 + 1,
            style(text).cyan()
        );
    }
}

impl QuickfixLogger {
    pub fn new() -> QuickfixLogger {
        QuickfixLogger
    }
}

impl Logger for QuickfixLogger {
    fn log(&mut self, text: String, loc: Loc) {
        println!(
            "{}:{}:{}: {}",
            absolute_path(loc.file),
            loc.line,
            loc.col.0 + 1,
            text
        );
    }
}

impl JsonLogger {
    pub fn new() -> JsonLogger {
        JsonLogger
    }
}

impl Logger for JsonLogger {
    fn log(&mut self, text: String, loc: Loc) {
        println!(
            "{{\"file\": \"{}\", \"line\": {}, \"column\": {}, \"text\": \"{}\"}}",
            absolute_path(loc.file),
            loc.line,
            loc.col.0 + 1,
            text,
        );
    }
}
