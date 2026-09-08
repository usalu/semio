//! 🧾 `outline` — one named inference: this JSON document's own tree shape. `nodeCount` is a
//! real recursive walk over every `JsonValue` node (including the root and every array item /
//! object member value); `maxDepth` is the deepest nesting level (a bare scalar root is depth 1);
//! `rootKind` names the root value's own kind (`"object"`/`"array"`/`"string"`/`"number"`/
//! `"bool"`/`"null"`).

use crate::schema::snapshot::JsonValue;
use crate::JsonSnapshot;

//#region 🔖️Outline
/// 🧾️ `Json` document outline.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JsonOutline {
    pub node_count: u32,
    pub max_depth: u32,
    pub root_kind: String,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn root_kind_name(value: &JsonValue) -> &'static str {
    match value {
        JsonValue::Null => "null",
        JsonValue::Bool { .. } => "bool",
        JsonValue::Number { .. } => "number",
        JsonValue::String { .. } => "string",
        JsonValue::Array { .. } => "array",
        JsonValue::Object { .. } => "object",
    }
}

/// 🌳️ Recursively walks `value`, returning `(node_count, max_depth)` — `depth` is the caller's
/// own nesting level (the root call passes `1`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn walk(value: &JsonValue, depth: u32) -> (u32, u32) {
    match value {
        JsonValue::Array { items } => {
            let mut count = 1u32;
            let mut max_depth = depth;
            for item in items {
                let (c, d) = walk(item, depth + 1);
                count += c;
                max_depth = max_depth.max(d);
            }
            (count, max_depth)
        }
        JsonValue::Object { members } => {
            let mut count = 1u32;
            let mut max_depth = depth;
            for member in members {
                let (c, d) = walk(&member.value, depth + 1);
                count += c;
                max_depth = max_depth.max(d);
            }
            (count, max_depth)
        }
        _ => (1, depth),
    }
}

impl JsonOutline {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compute(snapshot: &JsonSnapshot) -> Self {
        let (node_count, max_depth) = walk(&snapshot.value, 1);
        Self { node_count, max_depth, root_kind: root_kind_name(&snapshot.value).to_string() }
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
