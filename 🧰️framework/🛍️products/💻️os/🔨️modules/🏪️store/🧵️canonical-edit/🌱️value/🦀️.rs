//! 🌱️ Indexed framework dynamic-value projection for Store canonical JSON encoding.

use super::{ArtifactCanonicalJson, ArtifactCanonicalJsonNode, ARTIFACT_CANONICAL_JSON_DEPTH};
use crate::DslValue;

fn indexed_value<'a>(root: &'a DslValue, path: &[usize]) -> Result<&'a DslValue, String> {
    if path.len() >= ARTIFACT_CANONICAL_JSON_DEPTH { return Err("canonical-edit.depth-limit".into()); }
    let mut current = root;
    for index in path {
        current = match current {
            DslValue::Array(values) => values.get(*index),
            DslValue::Object(values) => values.get(*index).map(|(_, value)| value),
            _ => None,
        }.ok_or_else(super::invalid_path)?;
    }
    Ok(current)
}

impl ArtifactCanonicalJson for DslValue {
    fn canonical_json_node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, String> {
        use ArtifactCanonicalJsonNode as N;
        use protocol::value::Number;
        Ok(match indexed_value(self, path)? {
            DslValue::Null => N::Null,
            DslValue::Bool(value) => N::Bool(*value),
            DslValue::Number(Number::UInt(value)) => N::U64(*value),
            DslValue::Number(Number::Int(value)) => N::I64(*value),
            DslValue::Number(Number::Float(value)) => N::F64(*value),
            DslValue::String(value) => N::String(value),
            DslValue::Array(values) => N::Array(values.len()),
            DslValue::Object(values) => N::Object(values.len()),
        })
    }

    fn canonical_json_key(&self, object_path: &[usize], index: usize) -> Result<&str, String> {
        let DslValue::Object(values) = indexed_value(self, object_path)? else { return Err(super::invalid_path()); };
        values.get(index).map(|(key, _)| key.as_str()).ok_or_else(super::invalid_path)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
