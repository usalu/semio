use super::*;
use crate::standards::v1::subsets::value::io::import::deserializers::artifacts::json::v_rfc8259::any::semio_value_from_json;
use crate::standards::v1::subsets::value::schema::snapshot::{SemioValueEntry, SemioValueNode};

#[semio_framework_async_macros::async_test]
async fn int_and_float_lexemes_reemit_as_a_plain_number_verbatim() {
    let nodes = HashMap::new();
    let mut visiting = HashSet::new();
    assert_eq!(json_value_from_semio(&SemioValue::Int { lexeme: "9007199254740993".into() }, &nodes, &mut visiting).unwrap(), JsonValue::Number { lexeme: "9007199254740993".into() });
    assert_eq!(json_value_from_semio(&SemioValue::Float { lexeme: "1.2300".into() }, &nodes, &mut visiting).unwrap(), JsonValue::Number { lexeme: "1.2300".into() });
}

#[semio_framework_async_macros::async_test]
async fn bytes_become_a_base64_string() {
    let nodes = HashMap::new();
    let mut visiting = HashSet::new();
    let out = json_value_from_semio(&SemioValue::Bytes { value: vec![0, 1, 2, 255] }, &nodes, &mut visiting).unwrap();
    assert_eq!(out, JsonValue::String { value: base64_encode(&[0, 1, 2, 255]) });
}

#[semio_framework_async_macros::async_test]
async fn ref_is_dereferenced_inline() {
    let target_id = ValueId::new("n1");
    let target_value = SemioValue::Str { value: "leaf".into() };
    let mut nodes_owned: HashMap<&ValueId, &SemioValue> = HashMap::new();
    nodes_owned.insert(&target_id, &target_value);
    let mut visiting = HashSet::new();
    let out = json_value_from_semio(&SemioValue::Ref { id: target_id.clone() }, &nodes_owned, &mut visiting).unwrap();
    assert_eq!(out, JsonValue::String { value: "leaf".into() });
}

#[semio_framework_async_macros::async_test]
async fn dangling_ref_is_a_hard_error() {
    let nodes: HashMap<&ValueId, &SemioValue> = HashMap::new();
    let mut visiting = HashSet::new();
    let err = json_value_from_semio(&SemioValue::Ref { id: ValueId::new("missing") }, &nodes, &mut visiting);
    assert!(err.is_err());
}

#[semio_framework_async_macros::async_test]
async fn self_cycle_is_a_hard_error() {
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
#[semio_framework_async_macros::async_test]
async fn json_to_value_to_json_to_value_round_trips() {
    let json = JsonValue::Object {
        members: vec![
            JsonMember { key: "name".into(), value: JsonValue::String { value: "semio".into() } },
            JsonMember { key: "count".into(), value: JsonValue::Number { lexeme: "42".into() } },
            JsonMember { key: "ratio".into(), value: JsonValue::Number { lexeme: "3.500".into() } },
            JsonMember { key: "tags".into(), value: JsonValue::Array { items: vec![JsonValue::Bool { value: true }, JsonValue::Null] } },
        ],
    };
    let s1_value = semio_value_from_json(&json);
    let s1 = SemioValueSnapshot { schema: crate::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root: s1_value, nodes: Vec::new() };
    let json_x = semio_framework_plugin::resolve_ready(SemioValueToJson::serialize(&s1)).expect("serialize");
    let s2_value = semio_value_from_json(&json_x.value);
    assert_eq!(s1.root, s2_value);
}

#[semio_framework_async_macros::async_test]
async fn nodes_graph_round_trips_through_dereferenced_json() {
    let s1 = SemioValueSnapshot {
        schema: crate::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(),
        root: SemioValue::Map { entries: vec![SemioValueEntry { key: "linked".into(), value: SemioValue::Ref { id: ValueId::new("n1") } }] },
        nodes: vec![SemioValueNode { id: ValueId::new("n1"), value: SemioValue::Int { lexeme: "7".into() } }],
    };
    let json_x = semio_framework_plugin::resolve_ready(SemioValueToJson::serialize(&s1)).expect("serialize");
    match &json_x.value {
        JsonValue::Object { members } => {
            assert_eq!(members[0].key, "linked");
            assert_eq!(members[0].value, JsonValue::Number { lexeme: "7".into() }, "Ref dereferenced inline since json has no graph");
        }
        other => panic!("expected value, got {other:?}"),
    }
}
