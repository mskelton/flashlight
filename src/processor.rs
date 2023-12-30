use std::path::{Path, PathBuf};

use console::style;
use ignore::types::TypesBuilder;
use ignore::WalkBuilder;

use crate::logger::{LogEntry, Logger};
use crate::parser::{self, ParseError, ParsedModule};

pub trait ProcessorRequest {
    fn path(&self) -> &PathBuf;
    fn analyze(&self, module: &ParsedModule) -> Vec<LogEntry>;
}

pub struct Processor<'a, L, R>
where
    L: Logger,
    R: ProcessorRequest,
{
    request: R,
    logger: &'a mut L,
}

impl<'a, L, R> Processor<'a, L, R>
where
    L: Logger,
    R: ProcessorRequest,
{
    pub fn new(request: R, logger: &'a mut L) -> Processor<'a, L, R> {
        Processor { logger, request }
    }

    pub fn process(&mut self) {
        let matcher = TypesBuilder::new()
            .add_defaults()
            .select("js")
            .select("ts")
            .build()
            .unwrap();

        WalkBuilder::new(&self.request.path())
            .hidden(false)
            .types(matcher)
            .build()
            .filter_map(|entry| entry.ok())
            .filter(|file| file.file_type().map_or(false, |ft| ft.is_file()))
            .for_each(|file| match parser::parse(&file.path()) {
                Ok(parsed) => {
                    self.request
                        .analyze(&parsed)
                        .into_iter()
                        .for_each(|entry| self.logger.log(entry));
                }
                Err(err) => self.print_error(&file.path(), err),
            });
    }

    fn print_error(&self, path: &Path, err: ParseError) {
        let error_text = style("error").red().bold();

        match err {
            ParseError::IO(err) => {
                println!(
                    "{}: Failed to load {} with status code {}.",
                    error_text,
                    path.display(),
                    err.raw_os_error()
                        .map_or("unknown".to_string(), |code| code.to_string())
                )
            }
            ParseError::Parse(err, _) => {
                println!(
                    "{}: Encountered parsing error while reading {}.\n  {}",
                    error_text,
                    path.display(),
                    style(err.kind().msg()).dim()
                )
            }
        };
    }
}
