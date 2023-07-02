use std::path::Path;

use swc_common::sync::Lrc;
use swc_common::SourceMap;
use swc_ecma_ast::{EsVersion, Module};
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::{Parser, StringInput, Syntax, TsConfig};

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
        Syntax::Typescript(TsConfig {
            decorators: true,
            tsx: true,
            ..Default::default()
        }),
        EsVersion::EsNext,
        StringInput::from(&*file),
        None,
    );

    match Parser::new_from(lexer).parse_module() {
        Ok(module) => Ok(ParsedModule { path, module, source_map }),
        Err(err) => Err(ParseError::Parse(err, source_map)),
    }
}
