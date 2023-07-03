use std::path::PathBuf;

use swc_ecma_ast as ast;

use crate::logger::Logger;
use crate::parser::ParsedModule;
use crate::utils;

pub struct AnalysisRequest {
    pub path: PathBuf,
    pub source: String,
    pub specifier: Option<String>,
}

pub struct AnalysisResponse<'a> {
    pub request: &'a AnalysisRequest,
    pub parsed: &'a ParsedModule<'a>,
    pub imports: &'a Vec<&'a ast::ImportDecl>,
    pub specifiers: Option<&'a Vec<&'a ast::ImportNamedSpecifier>>,
}

fn get_imports<'a>(
    module: &'a ast::Module,
    request: &AnalysisRequest,
) -> Vec<&'a ast::ImportDecl> {
    return module
        .body
        .iter()
        .filter_map(utils::is_module_decl)
        .filter_map(utils::is_import_decl)
        .filter(|import| import.src.value == request.source)
        .collect::<Vec<_>>();
}

fn get_specifiers<'a>(
    imports: &'a Vec<&ast::ImportDecl>,
    name: &String,
) -> Vec<&'a ast::ImportNamedSpecifier> {
    return imports
        .iter()
        .flat_map(|import| import.specifiers.iter())
        .filter_map(utils::is_named_specifier)
        .filter(|specifier| specifier.local.sym.eq(name))
        .collect::<Vec<_>>();
}

pub fn analyze<T: Logger>(
    parsed: &ParsedModule,
    request: &AnalysisRequest,
    logger: &mut T,
) {
    let imports = get_imports(&parsed.module, &request);
    if imports.len() == 0 {
        return;
    }

    let specifiers = match &request.specifier {
        Some(specifier) => {
            let specifiers = get_specifiers(&imports, &specifier);

            match specifiers.len() {
                0 => return,
                _ => Some(specifiers),
            }
        }
        None => None,
    };

    logger.log(AnalysisResponse {
        imports: &imports,
        parsed,
        request,
        specifiers: specifiers.as_ref(),
    });
}
