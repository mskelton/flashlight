use std::path::{Path, PathBuf};

use console::style;
use ignore::types::TypesBuilder;
use ignore::WalkBuilder;
use swc_common::Span;

use crate::logger::Logger;
use crate::parser::{self, ParseError, ParsedModule};

pub trait ProcessorRequest {
    fn path(&self) -> &PathBuf;
    fn analyze(&self, module: &ParsedModule) -> Vec<Span>;
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
                Ok(parsed) => self.analyze(parsed),
                Err(err) => self.print_error(&file.path(), err),
            });
    }

    fn analyze(&mut self, parsed: ParsedModule) {
        let source = &parsed.source_map;

        self.request
            .analyze(&parsed)
            .into_iter()
            .filter_map(|span| {
                let lines = match source.span_to_lines(span) {
                    Ok(lines) => lines,
                    Err(_) => return None,
                };

                let loc = source.lookup_char_pos(span.lo);
                let line = lines.file.lookup_line(span.lo)?;
                let text = lines.file.get_line(line)?.to_string();

                Some((text, loc))
            })
            .for_each(|(text, loc)| self.logger.log(text, loc));
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
