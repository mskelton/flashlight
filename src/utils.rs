use std::rc::Rc;

use swc_common::{FileName, SourceFile};
use swc_ecma_ast as ast;

pub fn is_module_decl(item: &ast::ModuleItem) -> Option<&ast::ModuleDecl> {
    match item {
        ast::ModuleItem::ModuleDecl(decl) => Some(decl),
        _ => None,
    }
}

pub fn is_import_decl(import: &ast::ModuleDecl) -> Option<&ast::ImportDecl> {
    match import {
        ast::ModuleDecl::Import(decl) => Some(decl),
        _ => None,
    }
}

pub fn is_named_specifier(
    specifier: &ast::ImportSpecifier,
) -> Option<&ast::ImportNamedSpecifier> {
    match specifier {
        ast::ImportSpecifier::Named(specifier) => Some(specifier),
        _ => None,
    }
}

pub fn absolute_path(file: Rc<SourceFile>) -> String {
    file.unmapped_path
        .as_ref()
        .and_then(|f| match f {
            FileName::Real(path) => {
                Some(path.canonicalize().ok()?.to_str()?.to_string())
            }
            _ => None,
        })
        .unwrap_or_else(|| file.name.to_string())
}
