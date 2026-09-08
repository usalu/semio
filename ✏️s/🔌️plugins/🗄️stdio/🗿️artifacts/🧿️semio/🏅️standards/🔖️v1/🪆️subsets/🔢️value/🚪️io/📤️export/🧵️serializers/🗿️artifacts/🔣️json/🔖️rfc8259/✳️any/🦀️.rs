//! 📤️ `SemioValueToJson` — mirror of `SemioValueFromJson`. `Null`/`Bool`/`Str`/`List`/`Map` map
//! directly back onto `Null`/`Bool`/`String`/`Array`/`Value`; `Int{lexeme}`/`Float{lexeme}` both
//! re-emit as a plain `Number{lexeme}` verbatim (the split is reversible by construction — an
//! `Int`'s lexeme never contains `.`/`e`/`E`, a `Float`'s always does, so re-classifying the
//! resulting `Number` on the way back reproduces the same variant).
//!
//! Two REAL, honest, one-directional gaps (json has no binary or graph-reference primitive):
//! - `Bytes{value}` has no JSON type — encoded as a plain base64 `String`. This is asymmetric: the
//!   deserializer never produces `Bytes` (a JSON string is always `Str`), so `Bytes` -> json ->
//!   value always becomes `Str`, never round-trips back to `Bytes`. Documented, never silently
//!   "fixed" by inventing a JSON binary convention this codec doesn't otherwise use.
//! - `Ref{id}` is DEREFERENCED: the referenced `nodes` entry's value is walked and inlined
//!   recursively (json has no graph — every `JsonValue` is a tree). A `Ref` that does not resolve
//!   in `nodes`, or a reference cycle (a node reachable from itself through one or more `Ref`
//!   hops), is a hard `PackError` — never silently dropped or truncated.

use semio_s_artifact_stdio_json::schema::snapshot::{JsonMember, JsonValue};
use semio_s_artifact_stdio_json::JsonSnapshot;
use semio_s_artifact_stdio_json::STDIO_JSON_DOCUMENT_SCHEMA;
use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueSnapshot, ValueId};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use std::collections::{HashMap, HashSet};

//#region 🔖️Serializer
pub struct SemioValueToJson;

impl ArtifactSerializer for SemioValueToJson {
    type From = SemioValueSnapshot;
    type Into = JsonSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("value") };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let nodes: HashMap<&ValueId, &SemioValue> = from.nodes.iter().map(|n| (&n.id, &n.value)).collect();
        let mut visiting: HashSet<ValueId> = HashSet::new();
        let value = json_value_from_semio(&from.root, &nodes, &mut visiting)?;
        Ok(JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
//#endregion 🔖️Serializer

//#region 🔖️Base64
const B64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn base64_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        out.push(B64_TABLE[(b0 >> 2) as usize] as char);
        out.push(B64_TABLE[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        out.push(if chunk.len() > 1 { B64_TABLE[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { B64_TABLE[(b2 & 0x3f) as usize] as char } else { '=' });
    }
    out
}
//#endregion 🔖️Base64

//#region 🔖️Convert
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_value_from_semio(v: &SemioValue, nodes: &HashMap<&ValueId, &SemioValue>, visiting: &mut HashSet<ValueId>) -> Result<JsonValue, store::PackError> {
    match v {
        SemioValue::Null => Ok(JsonValue::Null),
        SemioValue::Bool { value } => Ok(JsonValue::Bool { value: *value }),
        SemioValue::Int { lexeme } | SemioValue::Float { lexeme } => Ok(JsonValue::Number { lexeme: lexeme.clone() }),
        SemioValue::Str { value } => Ok(JsonValue::String { value: value.clone() }),
        SemioValue::Bytes { value } => Ok(JsonValue::String { value: base64_encode(value) }),
        SemioValue::List { items } => {
            let items = items.iter().map(|item| json_value_from_semio(item, nodes, visiting)).collect::<Result<Vec<_>, _>>()?;
            Ok(JsonValue::Array { items })
        }
        SemioValue::Map { entries } => {
            let members = entries.iter().map(|e| Ok(JsonMember { key: e.key.clone(), value: json_value_from_semio(&e.value, nodes, visiting)? })).collect::<Result<Vec<_>, store::PackError>>()?;
            Ok(JsonValue::Object { members })
        }
        SemioValue::Ref { id } => {
            if !visiting.insert(id.clone()) {
                return Err(store::PackError::Schema(format!("value->json: reference cycle detected at id {:?} (json has no graph, cannot represent a cycle)", id.value)));
            }
            let target = nodes.get(id).ok_or_else(|| store::PackError::Schema(format!("value->json: dangling Ref{{id: {:?}}} — not found in `nodes`", id.value)))?;
            let result = json_value_from_semio(target, nodes, visiting);
            visiting.remove(id);
            result
        }
    }
}
//#endregion 🔖️Convert

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
