use std::path::Path;

use console::style;
use ignore::types::TypesBuilder;
use ignore::WalkBuilder;

use crate::analysis::{self, AnalysisRequest};
use crate::parser::{self, ParseError};

pub struct Processor {
    request: AnalysisRequest,
}

impl Processor {
    pub fn new(request: AnalysisRequest) -> Processor {
        Processor { request }
    }

    pub fn process(&self) {
        let matcher = TypesBuilder::new()
            .add_defaults()
            .select("js")
            .select("ts")
            .build()
            .unwrap();

        WalkBuilder::new(&self.request.path)
            .hidden(false)
            .types(matcher)
            .build()
            .filter_map(|entry| entry.ok())
            .filter(|file| file.file_type().map_or(false, |ft| ft.is_file()))
            .for_each(|file| match parser::parse(&file.path()) {
                Ok(parsed) => analysis::analyze(&parsed, &self.request),
                Err(err) => self.print_error(&file.path(), err),
            });
    }

    fn print_error(&self, path: &Path, err: ParseError) {
        let error_text = style("Error").red().bold();

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
