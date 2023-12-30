pub mod ast;

use std::rc::Rc;

use swc_common::{FileName, SourceFile};

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

pub fn parse_key_value(
    attribute: Option<String>,
) -> (Option<String>, Option<String>) {
    match attribute {
        Some(attribute) => {
            let mut parts = attribute.split('=');
            let key = parts.next().map(|v| v.to_string());
            let value = parts.next().map(|v| v.to_string());

            (key, value)
        }
        None => (None, None),
    }
}
