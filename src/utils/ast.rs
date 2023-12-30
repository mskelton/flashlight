use swc_ecma_ast as ast;

pub fn is_named_specifier(
    specifier: &ast::ImportSpecifier,
) -> Option<&ast::ImportNamedSpecifier> {
    match specifier {
        ast::ImportSpecifier::Named(specifier) => Some(specifier),
        _ => None,
    }
}

pub fn is_jsx_attribute(item: &ast::JSXAttrOrSpread) -> Option<&ast::JSXAttr> {
    match item {
        ast::JSXAttrOrSpread::JSXAttr(attr) => Some(attr),
        _ => None,
    }
}
