use std::path::PathBuf;
use swc_common::Span;
use swc_ecma_ast::{
    Expr, JSXAttr, JSXAttrName, JSXAttrValue, JSXElement, JSXElementName,
    JSXExpr, JSXMemberExpr, JSXObject, Lit,
};
use swc_ecma_visit::{Visit, VisitWith};

use crate::parser::ParsedModule;
use crate::processor::ProcessorRequest;
use crate::utils;

pub struct TagsRequest {
    pub path: PathBuf,
    pub name: String,
    pub attribute: Option<String>,
    pub value: Option<String>,
}

impl ProcessorRequest for TagsRequest {
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn analyze(&self, parsed: &ParsedModule) -> Vec<Span> {
        let mut visitor =
            ElementVisitor { elements: Vec::new(), request: &self };

        visitor.visit_module(&parsed.module);
        visitor.elements
    }
}

struct ElementVisitor<'a> {
    elements: Vec<Span>,
    request: &'a TagsRequest,
}

impl<'a> Visit for ElementVisitor<'a> {
    fn visit_jsx_element(&mut self, node: &JSXElement) {
        // First check that the element name matches the request
        if get_element_name(node) == self.request.name {
            // If an attribute is provided, check that the element contains
            // the given attribute and optionally the value specified.
            let has_attr = match &self.request.attribute {
                Some(attr) => {
                    has_required_attr(node, &attr, &self.request.value)
                }
                None => true,
            };

            if has_attr {
                self.elements.push(node.span);
            }
        }

        node.visit_children_with(self)
    }
}

fn get_member_name(expr: &JSXMemberExpr) -> String {
    let prefix = match &expr.obj {
        JSXObject::JSXMemberExpr(expr) => get_member_name(expr),
        JSXObject::Ident(ident) => ident.sym.to_string(),
    };

    format!("{}{}", prefix, expr.prop)
}

fn get_element_name(element: &JSXElement) -> String {
    match &element.opening.name {
        JSXElementName::Ident(ident) => ident.sym.to_string(),
        JSXElementName::JSXMemberExpr(expr) => get_member_name(expr),
        JSXElementName::JSXNamespacedName(name) => {
            format!("{}{}", name.ns, name.name)
        }
    }
}

fn get_attribute_name(attr: &JSXAttr) -> String {
    match &attr.name {
        JSXAttrName::Ident(ident) => ident.sym.to_string(),
        JSXAttrName::JSXNamespacedName(name) => {
            format!("{}{}", name.ns, name.name)
        }
    }
}

fn get_lit_value(lit: &Lit) -> String {
    match lit {
        Lit::Str(str) => str.value.to_string(),
        Lit::Bool(bool) => bool.value.to_string(),
        Lit::Num(num) => num.value.to_string(),
        Lit::BigInt(bigint) => bigint.value.to_string(),
        Lit::Regex(regex) => regex.exp.to_string(),
        Lit::JSXText(text) => text.value.to_string(),
        Lit::Null(_) => String::from("null"),
    }
}

fn get_expr_value(expr: &JSXExpr) -> String {
    match expr {
        JSXExpr::Expr(expr) => match *expr.clone() {
            Expr::Lit(lit) => get_lit_value(&lit),
            _ => "".to_string(),
        },
        _ => "".to_string(),
    }
}

fn get_attribute_value(attr: &JSXAttr) -> String {
    match &attr.value {
        Some(value) => match value {
            JSXAttrValue::JSXExprContainer(expr) => get_expr_value(&expr.expr),
            JSXAttrValue::Lit(lit) => get_lit_value(lit),
            _ => "".to_string(),
        },
        None => "".to_string(),
    }
}

fn has_required_attr(
    element: &JSXElement,
    name: &String,
    value: &Option<String>,
) -> bool {
    let count = element
        .opening
        .attrs
        .iter()
        .filter_map(utils::ast::is_jsx_attribute)
        .filter(|attr| get_attribute_name(attr) == *name)
        .filter(|attr| match value {
            Some(value) => get_attribute_value(attr) == *value,
            None => true,
        })
        .count();

    count > 0
}
