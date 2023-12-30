use std::path::PathBuf;
use swc_common::Span;
use swc_ecma_ast::ImportDecl;
use swc_ecma_visit::{Visit, VisitWith};

use crate::parser::ParsedModule;
use crate::processor::ProcessorRequest;
use crate::utils;

pub struct ImportsRequest {
    pub path: PathBuf,
    pub source: String,
    pub specifier: Option<String>,
}

impl ProcessorRequest for ImportsRequest {
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn analyze(&self, parsed: &ParsedModule) -> Vec<Span> {
        let mut visitor = ImportVisitor { imports: Vec::new(), request: &self };

        visitor.visit_module(&parsed.module);
        visitor.imports
    }
}

struct ImportVisitor<'a> {
    imports: Vec<Span>,
    request: &'a ImportsRequest,
}

impl<'a> Visit for ImportVisitor<'a> {
    fn visit_import_decl(&mut self, node: &ImportDecl) {
        // First check that the import source matches the request
        if node.src.value == self.request.source {
            // If a specifier is provided, check that the import contains
            // the given specifier.
            let has_specifier = match &self.request.specifier {
                Some(spec) => has_required_specifier(node, &spec),
                None => true,
            };

            if has_specifier {
                self.imports.push(node.span);
            }
        }

        node.visit_children_with(self)
    }
}

fn has_required_specifier(import: &ImportDecl, name: &String) -> bool {
    let count = import
        .specifiers
        .iter()
        .filter_map(utils::ast::is_named_specifier)
        .filter(|specifier| specifier.local.sym.eq(name))
        .count();

    count > 0
}
