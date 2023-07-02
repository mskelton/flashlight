use std::rc::Rc;

use console::style;
use swc_common::{SourceFile, Spanned};

use crate::analysis::AnalysisResponse;
use crate::utils::absolute_path;

pub trait Logger {
    fn log(&mut self, res: AnalysisResponse);
    fn end(&self) {}
}

pub struct ConsoleLogger;
pub struct QuickfixLogger;
pub struct JsonLogger {
    logs: Vec<String>,
}

pub enum LoggerType {
    Console(ConsoleLogger),
    Quickfix(QuickfixLogger),
    Json(JsonLogger),
}

impl Logger for LoggerType {
    fn log(&mut self, res: AnalysisResponse) {
        match self {
            LoggerType::Console(logger) => logger.log(res),
            LoggerType::Quickfix(logger) => logger.log(res),
            LoggerType::Json(logger) => logger.log(res),
        }
    }

    fn end(&self) {
        match self {
            LoggerType::Console(logger) => logger.end(),
            LoggerType::Quickfix(logger) => logger.end(),
            LoggerType::Json(logger) => logger.end(),
        }
    }
}

impl ConsoleLogger {
    pub fn new() -> ConsoleLogger {
        ConsoleLogger
    }
}

impl Logger for ConsoleLogger {
    fn log(&mut self, res: AnalysisResponse) {
        let (file, line) = match get_import_text(&res) {
            Some((file, line)) => (file, line),
            None => return,
        };

        println!("{}:{}:{}: {}", file.name, 1, 1, style(line).cyan());
    }
}

impl QuickfixLogger {
    pub fn new() -> QuickfixLogger {
        QuickfixLogger
    }
}

impl Logger for QuickfixLogger {
    fn log(&mut self, res: AnalysisResponse) {
        let (file, line) = match get_import_text(&res) {
            Some((file, line)) => (file, line),
            None => return,
        };

        println!("{}:{}:{}: {}", absolute_path(file), 1, 1, line);
    }
}

impl JsonLogger {
    pub fn new() -> JsonLogger {
        JsonLogger { logs: Vec::new() }
    }
}

impl Logger for JsonLogger {
    fn log(&mut self, res: AnalysisResponse) {
        let (file, line) = match get_import_text(&res) {
            Some((file, line)) => (file, line),
            None => return,
        };

        self.logs.push(format!(
            "{}:{}:{}: {}",
            file.name,
            1,
            1,
            style(line).cyan()
        ));
    }

    fn end(&self) {
        println!("{}", self.logs.join("\n"));
    }
}

fn get_import_text(res: &AnalysisResponse) -> Option<(Rc<SourceFile>, String)> {
    let source = &res.parsed.source_map;
    let import = match res.imports.first() {
        Some(i) => i,
        None => return None,
    };

    let lines = match source.span_to_lines(import.span()) {
        Ok(lines) => lines,
        Err(_) => return None,
    };

    let file = lines.file;
    let line_index = file.lookup_line(import.span_lo())?;
    let line = file.get_line(line_index)?;

    // TODO: clone
    return Some((file.clone(), line.to_string()));
}
