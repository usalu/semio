
use super::*;
use crate::schema::snapshot::JsonMember;

#[semio_framework_async_macros::async_test]
async fn counts_nodes_and_depth_over_nested_structure() {
    let snapshot = JsonSnapshot {
        schema: "stdio.json".into(),
        value: JsonValue::Object { members: vec![JsonMember { key: "a".into(), value: JsonValue::Array { items: vec![JsonValue::Number { lexeme: "1".into() }, JsonValue::Number { lexeme: "2".into() }] } }] },
    };
    let outline = JsonOutline::compute(&snapshot);
    // root object(1) + array(1) + two numbers(2) = 4 nodes; depth: object(1) -> array(2) -> number(3)
    assert_eq!(outline.node_count, 4);
    assert_eq!(outline.max_depth, 3);
    assert_eq!(outline.root_kind, "object");
}

#[semio_framework_async_macros::async_test]
async fn scalar_root_has_depth_one() {
    let snapshot = JsonSnapshot { schema: "stdio.json".into(), value: JsonValue::Null };
    let outline = JsonOutline::compute(&snapshot);
    assert_eq!(outline.node_count, 1);
    assert_eq!(outline.max_depth, 1);
    assert_eq!(outline.root_kind, "null");
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = JsonSnapshot::default();
    assert_eq!(JsonOutline::compute(&snapshot), JsonOutline::compute(&snapshot));
}
