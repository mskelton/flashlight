use std::path::PathBuf;

use swc_common::Spanned;
use swc_ecma_ast as ast;

use crate::parser::ParsedModule;
use crate::utils;

pub struct AnalysisRequest {
    pub path: PathBuf,
    pub source: String,
    pub specifier: Option<String>,
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

pub fn analyze(parsed: &ParsedModule, request: &AnalysisRequest) {
    let imports = get_imports(&parsed.module, &request);
    if imports.len() == 0 {
        return;
    }

    if let Some(specifier) = &request.specifier {
        let specifiers = get_specifiers(&imports, specifier);
        if specifiers.len() == 0 {
            return;
        }
    }

    let span = parsed.module.span_lo();
    let lo = parsed.source_map.lookup_char_pos_adj(span);

    // TODO: Print import
    println!("{}:{}:{}: {}", lo.filename, lo.line, lo.col.0 + 1, "");
}
