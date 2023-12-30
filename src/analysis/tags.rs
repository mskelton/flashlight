use std::path::PathBuf;
use swc_common::Span;
use swc_ecma_ast::{
    Expr, JSXAttr, JSXAttrName, JSXAttrValue, JSXElement, JSXElementName,
    JSXExpr, JSXMemberExpr, JSXObject, Lit, Module,
};
use swc_ecma_visit::{Visit, VisitWith};

use crate::parser::ParsedModule;
use crate::processor::ProcessorRequest;
use crate::utils;

struct ElementVisitor {
    elements: Vec<JSXElement>,
    name: String,
}

impl Visit for ElementVisitor {
    fn visit_jsx_element(&mut self, node: &JSXElement) {
        if get_element_name(node) == self.name {
            self.elements.push(node.clone());
        }

        node.visit_children_with(self)
    }
}

pub struct TagsRequest {
    pub path: PathBuf,
    pub name: String,
    pub attribute: Option<String>,
    pub value: Option<String>,
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

fn get_elements<'a>(
    module: &'a Module,
    request: &'a TagsRequest,
) -> Vec<JSXElement> {
    let mut visitor =
        ElementVisitor { elements: vec![], name: request.name.clone() };

    visitor.visit_module(module);
    visitor.elements
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

impl ProcessorRequest for TagsRequest {
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn analyze(&self, parsed: &ParsedModule) -> Vec<Span> {
        get_elements(&parsed.module, &self)
            .iter()
            .filter(|element| match &self.attribute {
                Some(attr) => has_required_attr(&element, &attr, &self.value),
                None => true,
            })
            .map(|element| element.span)
            .collect()
    }
}
