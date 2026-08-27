//! 🧰 Shared path addressing for direct JSON mutations.
use crate::artifacts::json::schema::diff::{JsonArrayDiff, JsonArrayModified, JsonDiff, JsonObjectDiff, JsonObjectModified, JsonValueDiff};
use crate::artifacts::json::schema::snapshot::JsonValue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum JsonPathSegment { Key(String), Index(usize) }
pub type JsonPath = Vec<JsonPathSegment>;

pub fn resolve<'a>(root: &'a JsonValue, path: &[JsonPathSegment]) -> Option<&'a JsonValue> {
    let mut node = root;
    for segment in path {
        node = match (segment, node) {
            (JsonPathSegment::Key(key), JsonValue::Object { members }) => &members.iter().find(|member| &member.key == key)?.value,
            (JsonPathSegment::Index(index), JsonValue::Array { items }) => items.get(*index)?,
            _ => return None,
        };
    }
    Some(node)
}

pub fn diff_at_path(path: &[JsonPathSegment], leaf: Option<JsonValueDiff>) -> JsonDiff { JsonDiff { value: leaf.map(|value| wrap_at_path(path, value)) } }
fn wrap_at_path(path: &[JsonPathSegment], leaf: JsonValueDiff) -> JsonValueDiff {
    match path.split_first() {
        None => leaf,
        Some((JsonPathSegment::Key(key), rest)) => JsonValueDiff::Object { diff: JsonObjectDiff { removed: Vec::new(), added: Vec::new(), modified: vec![JsonObjectModified { key: key.clone(), diff: wrap_at_path(rest, leaf) }] } },
        Some((JsonPathSegment::Index(index), rest)) => JsonValueDiff::Array { diff: JsonArrayDiff { removed: Vec::new(), added: Vec::new(), modified: vec![JsonArrayModified { index: *index, diff: wrap_at_path(rest, leaf) }] } },
    }
}
