use std::rc::Rc;

use console::style;
use swc_common::{Loc, SourceFile};

use crate::utils::absolute_path;

pub struct LogEntry {
    pub file: Rc<SourceFile>,
    pub loc: Loc,
    pub text: String,
}

pub trait Logger {
    fn log(&mut self, entry: LogEntry);
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
    fn log(&mut self, entry: LogEntry) {
        match self {
            LoggerType::Console(logger) => logger.log(entry),
            LoggerType::Quickfix(logger) => logger.log(entry),
            LoggerType::Json(logger) => logger.log(entry),
        }
    }
}

impl ConsoleLogger {
    pub fn new() -> ConsoleLogger {
        ConsoleLogger
    }
}

impl Logger for ConsoleLogger {
    fn log(&mut self, entry: LogEntry) {
        println!(
            "{}:{}:{} {}",
            entry.file.name,
            entry.loc.line,
            entry.loc.col.0 + 1,
            style(entry.text).cyan()
        );
    }
}

impl QuickfixLogger {
    pub fn new() -> QuickfixLogger {
        QuickfixLogger
    }
}

impl Logger for QuickfixLogger {
    fn log(&mut self, entry: LogEntry) {
        println!(
            "{}:{}:{}: {}",
            absolute_path(entry.file),
            entry.loc.line,
            entry.loc.col.0 + 1,
            entry.text
        );
    }
}

impl JsonLogger {
    pub fn new() -> JsonLogger {
        JsonLogger
    }
}

impl Logger for JsonLogger {
    fn log(&mut self, entry: LogEntry) {
        println!(
            "{{\"file\": \"{}\", \"line\": {}, \"column\": {}, \"text\": \"{}\"}}",
            absolute_path(entry.file),
            entry.loc.line,
            entry.loc.col.0 + 1,
            entry.text,
        );
    }
}
