use std::path::PathBuf;
use swc_ecma_ast as ast;

use crate::logger::LogEntry;
use crate::parser::ParsedModule;
use crate::processor::ProcessorRequest;
use crate::utils;

pub struct TagsRequest {
    pub path: PathBuf,
    pub name: String,
    pub attribute: Option<String>,
    pub value: Option<String>,
}

fn get_jsx_member_expression_name(expr: &ast::JSXMemberExpr) -> String {
    let prefix = match expr.obj {
        ast::JSXObject::JSXMemberExpr(expr) => {
            get_jsx_member_expression_name(&expr)
        }
        ast::JSXObject::Ident(ident) => ident.sym.to_string(),
    };

    format!("{}{}", prefix, expr.prop)
}

fn get_jsx_element_name(element: &ast::JSXElement) -> String {
    match element.opening.name {
        ast::JSXElementName::Ident(ident) => ident.sym.to_string(),
        ast::JSXElementName::JSXMemberExpr(expr) => {
            get_jsx_member_expression_name(&expr)
        }
        ast::JSXElementName::JSXNamespacedName(name) => {
            format!("{}{}", name.ns, name.name)
        }
    }
}

fn get_elements<'a>(
    module: &'a ast::Module,
    request: &TagsRequest,
) -> Vec<&'a ast::JSXElement> {
    return module
        .body
        .iter()
        // .filter_map(utils::ast::is_module_decl)
        .filter_map(utils::ast::is_jsx_element)
        .filter(|element| get_jsx_element_name(element) == request.name)
        .collect::<Vec<_>>();
}

fn get_attributes<'a>(
    elements: &'a Vec<&ast::JSXElement>,
    name: &String,
    value: &Option<String>,
) -> Vec<&'a ast::JSXAttr> {
    return elements
        .iter()
        .flat_map(|element| element.opening.attrs.iter())
        .filter_map(utils::ast::is_jsx_attribute)
        .collect::<Vec<_>>();
}

impl ProcessorRequest for TagsRequest {
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn analyze(&self, parsed: &ParsedModule) -> Vec<LogEntry> {
        let elements = get_elements(&parsed.module, &self);
        if elements.len() == 0 {
            return vec![];
        }

        // If the user provided a specifier, we'll check if there are any matches
        // and only log those.
        // TODO: Update logs to include the specifier
        match &self.attribute {
            Some(attr) => {
                let attrs = get_attributes(&elements, &attr, &self.value);

                match attrs.len() {
                    0 => return vec![],
                    _ => Some(attrs),
                }
            }
            None => None,
        };

        let source = &parsed.source_map;
        return elements
            .iter()
            .filter_map(|element| {
                let lines = match source.span_to_lines(element.span) {
                    Ok(lines) => lines,
                    Err(_) => return None,
                };

                let file = lines.file;
                let loc = source.lookup_char_pos(element.span.lo);
                let line = file.lookup_line(element.span.lo)?;
                let text = file.get_line(line)?.to_string();

                Some(LogEntry { file, loc, text })
            })
            .collect();
    }
}
