use std::path::Path;

use swc_common::sync::Lrc;
use swc_common::SourceMap;
use swc_ecma_ast::{EsVersion, Module};
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::{EsConfig, Parser, StringInput, Syntax, TsConfig};

pub struct ParsedModule<'a> {
    pub path: &'a Path,
    pub source_map: Lrc<SourceMap>,
    pub module: Module,
}

pub enum ParseError {
    IO(std::io::Error),
    Parse(swc_ecma_parser::error::Error, Lrc<SourceMap>),
}

pub fn parse(path: &Path) -> Result<ParsedModule, ParseError> {
    let source_map: Lrc<SourceMap> = Default::default();
    let file = source_map.load_file(path).map_err(|err| ParseError::IO(err))?;

    let lexer = Lexer::new(
        guess_syntax(path),
        EsVersion::EsNext,
        StringInput::from(&*file),
        None,
    );

    match Parser::new_from(lexer).parse_module() {
        Ok(module) => Ok(ParsedModule { path, module, source_map }),
        Err(err) => Err(ParseError::Parse(err, source_map)),
    }
}

/// Guess the syntax of the file based on the file extension
fn guess_syntax(path: &Path) -> Syntax {
    let ext = path.extension().map_or("", |ext| ext.to_str().unwrap_or(""));

    return match ext {
        "ts" | "cts" | "mts" => Syntax::Typescript(TsConfig {
            decorators: true,
            ..Default::default()
        }),
        "tsx" => Syntax::Typescript(TsConfig {
            decorators: true,
            tsx: true,
            ..Default::default()
        }),
        &_ => Syntax::Es(EsConfig { jsx: true, ..Default::default() }),
    };
}
