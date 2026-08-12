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

use crate::artifacts::json::JsonSnapshot;
use crate::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;
use crate::artifacts::json::schema::snapshot::{JsonMember, JsonValue};
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{ValueId, SemioValueSnapshot, SemioValue};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use std::collections::{HashMap, HashSet};

//#region 🔖️Serializer
pub struct SemioValueToJson;

impl ArtifactSerializer for SemioValueToJson {
    type From = SemioValueSnapshot;
    type Into = JsonSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("value") };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

    fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let nodes: HashMap<&ValueId, &SemioValue> = from.nodes.iter().map(|n| (&n.id, &n.value)).collect();
        let mut visiting: HashSet<ValueId> = HashSet::new();
        let value = json_value_from_semio(&from.root, &nodes, &mut visiting)?;
        Ok(JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
    }
}

pub fn register() {}
//#endregion 🔖️Serializer

//#region 🔖️Base64
const B64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn base64_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity((bytes.len() + 2) / 3 * 4);
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
            let members = entries
                .iter()
                .map(|e| Ok(JsonMember { key: e.key.clone(), value: json_value_from_semio(&e.value, nodes, visiting)? }))
                .collect::<Result<Vec<_>, store::PackError>>()?;
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
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::value::io::import::deserializers::artifacts::json::v_rfc8259::any::semio_value_from_json;
    use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValueEntry, SemioValueNode};

    #[test]
    fn int_and_float_lexemes_reemit_as_a_plain_number_verbatim() {
        let nodes = HashMap::new();
        let mut visiting = HashSet::new();
        assert_eq!(json_value_from_semio(&SemioValue::Int { lexeme: "9007199254740993".into() }, &nodes, &mut visiting).unwrap(), JsonValue::Number { lexeme: "9007199254740993".into() });
        assert_eq!(json_value_from_semio(&SemioValue::Float { lexeme: "1.2300".into() }, &nodes, &mut visiting).unwrap(), JsonValue::Number { lexeme: "1.2300".into() });
    }

    #[test]
    fn bytes_become_a_base64_string() {
        let nodes = HashMap::new();
        let mut visiting = HashSet::new();
        let out = json_value_from_semio(&SemioValue::Bytes { value: vec![0, 1, 2, 255] }, &nodes, &mut visiting).unwrap();
        assert_eq!(out, JsonValue::String { value: base64_encode(&[0, 1, 2, 255]) });
    }

    #[test]
    fn ref_is_dereferenced_inline() {
        let target_id = ValueId::new("n1");
        let target_value = SemioValue::Str { value: "leaf".into() };
        let mut nodes_owned: HashMap<&ValueId, &SemioValue> = HashMap::new();
        nodes_owned.insert(&target_id, &target_value);
        let mut visiting = HashSet::new();
        let out = json_value_from_semio(&SemioValue::Ref { id: target_id.clone() }, &nodes_owned, &mut visiting).unwrap();
        assert_eq!(out, JsonValue::String { value: "leaf".into() });
    }

    #[test]
    fn dangling_ref_is_a_hard_error() {
        let nodes: HashMap<&ValueId, &SemioValue> = HashMap::new();
        let mut visiting = HashSet::new();
        let err = json_value_from_semio(&SemioValue::Ref { id: ValueId::new("missing") }, &nodes, &mut visiting);
        assert!(err.is_err());
    }

    #[test]
    fn self_cycle_is_a_hard_error() {
        let id = ValueId::new("n1");
        let value = SemioValue::Ref { id: id.clone() };
        let mut nodes_owned: HashMap<&ValueId, &SemioValue> = HashMap::new();
        nodes_owned.insert(&id, &value);
        let mut visiting = HashSet::new();
        let err = json_value_from_semio(&SemioValue::Ref { id: id.clone() }, &nodes_owned, &mut visiting);
        assert!(err.is_err(), "a Ref pointing to itself must error, not infinitely recurse");
    }

    /// 🧪️ Required proof: json -> value -> json -> value round trip preserves everything the
    /// value subset can represent (Bytes/Ref excepted — documented one-directional gaps, proven
    /// separately above).
    #[test]
    fn json_to_value_to_json_to_value_round_trips() {
        let json = JsonValue::Object {
            members: vec![
                JsonMember { key: "name".into(), value: JsonValue::String { value: "semio".into() } },
                JsonMember { key: "count".into(), value: JsonValue::Number { lexeme: "42".into() } },
                JsonMember { key: "ratio".into(), value: JsonValue::Number { lexeme: "3.500".into() } },
                JsonMember { key: "tags".into(), value: JsonValue::Array { items: vec![JsonValue::Bool { value: true }, JsonValue::Null] } },
            ],
        };
        let s1_value = semio_value_from_json(&json);
        let s1 = SemioValueSnapshot { schema: crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root: s1_value, nodes: Vec::new() };
        let json_x = SemioValueToJson::serialize(&s1).expect("serialize");
        let s2_value = semio_value_from_json(&json_x.value);
        assert_eq!(s1.root, s2_value);
    }

    #[test]
    fn nodes_graph_round_trips_through_dereferenced_json() {
        let s1 = SemioValueSnapshot {
            schema: crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(),
            root: SemioValue::Map { entries: vec![SemioValueEntry { key: "linked".into(), value: SemioValue::Ref { id: ValueId::new("n1") } }] },
            nodes: vec![SemioValueNode { id: ValueId::new("n1"), value: SemioValue::Int { lexeme: "7".into() } }],
        };
        let json_x = SemioValueToJson::serialize(&s1).expect("serialize");
        match &json_x.value {
            JsonValue::Object { members } => {
                assert_eq!(members[0].key, "linked");
                assert_eq!(members[0].value, JsonValue::Number { lexeme: "7".into() }, "Ref dereferenced inline since json has no graph");
            }
            other => panic!("expected value, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
