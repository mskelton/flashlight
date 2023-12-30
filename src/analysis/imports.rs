use std::path::PathBuf;
use swc_common::Span;
use swc_ecma_ast as ast;

use crate::parser::ParsedModule;
use crate::processor::ProcessorRequest;
use crate::utils;

pub struct ImportsRequest {
    pub path: PathBuf,
    pub source: String,
    pub specifier: Option<String>,
}

fn get_imports<'a>(
    module: &'a ast::Module,
    request: &ImportsRequest,
) -> Vec<&'a ast::ImportDecl> {
    return module
        .body
        .iter()
        .filter_map(utils::ast::is_module_decl)
        .filter_map(utils::ast::is_import_decl)
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
        .filter_map(utils::ast::is_named_specifier)
        .filter(|specifier| specifier.local.sym.eq(name))
        .collect::<Vec<_>>();
}

impl ProcessorRequest for ImportsRequest {
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn analyze(&self, parsed: &ParsedModule) -> Vec<Span> {
        let imports = get_imports(&parsed.module, &self);
        if imports.len() == 0 {
            return vec![];
        }

        // If the user provided a specifier, we'll check if there are any matches
        // and only log those.
        // TODO: Update logs to include the specifier
        match &self.specifier {
            Some(specifier) => {
                let specifiers = get_specifiers(&imports, &specifier);

                match specifiers.len() {
                    0 => return vec![],
                    _ => Some(specifiers),
                }
            }
            None => None,
        };

        return imports.iter().map(|import| import.span).collect();
    }
}
