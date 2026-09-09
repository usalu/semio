use super::*;
use crate::empty_trinity_graph_fixture;

#[semio_framework_async_macros::async_test]
async fn nakagin_example_dsl_round_trips() {
    let document = parse_dsl(NAKAGIN_EXAMPLE_TEXT).expect("parse nakagin example");
    ::store::os_store::test_support::assert_dsl_round_trip(&document);
}

#[semio_framework_async_macros::async_test]
async fn empty_document_dsl_round_trips() {
    ::store::os_store::test_support::assert_dsl_round_trip(&empty_trinity_graph_fixture());
}

#[semio_framework_async_macros::async_test]
async fn parse_dsl_rejects_unknown_keyword() {
    let err = JackSnapshot::parse_dsl("bogus line").expect_err("unknown keyword");
    assert!(err.message.contains("jack snapshot"));
}

#[semio_framework_async_macros::async_test]
async fn dsl_round_trip_mini_and_bundled_fixtures() {
    let nakagin = parse_dsl(NAKAGIN_EXAMPLE_TEXT).unwrap();
    ::store::os_store::test_support::assert_dsl_round_trip(&nakagin);
    ::store::os_store::test_support::assert_dsl_pack_equivalence(&nakagin);
}

/// 🧩️ A hand-built fixture (not one of the bundled `.trinity` examples) with a nested `Object`-shaped
/// node property (`position: {x,y,z}`) and `Number`-shaped edge properties (`u`/`v`) — exercises the
/// JSON-blob content codec round trip on non-trivial `PropertyBag`'s `Object`/`Number` variants.
#[semio_framework_async_macros::async_test]
async fn dsl_round_trip_mini_fixture() {
    use crate::{Camera, Edge, JackSnapshot, Manifest, Node, Port, PortDirection, PropertyBag, PropertyValue};
    use std::collections::BTreeMap;

    let fixture = JackSnapshot::with_content(
        JackSnapshot::SCHEMA.into(),
        "mini".into(),
        Some("nakagin".into()),
        Manifest::nakagin_default(),
        Camera::default(),
        vec![
            Node {
                id: "root".into(),
                kind: "Piece".into(),
                name: "core".into(),
                x: 0.0,
                y: 0.0,
                width: 80.0,
                height: 40.0,
                properties: {
                    let mut p = PropertyBag::new();
                    let mut pos = BTreeMap::new();
                    pos.insert("x".into(), PropertyValue::Number(0.0));
                    pos.insert("y".into(), PropertyValue::Number(0.0));
                    pos.insert("z".into(), PropertyValue::Number(0.0));
                    p.insert("position".into(), PropertyValue::Object(pos));
                    p
                },
                ports: vec![Port { id: "out-a".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
            },
            Node {
                id: "child".into(),
                kind: "Piece".into(),
                name: "capsule".into(),
                x: 120.0,
                y: 0.0,
                width: 80.0,
                height: 40.0,
                properties: PropertyBag::new(),
                ports: vec![Port { id: "in-a".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
            },
        ],
        vec![Edge {
            id: "e1".into(),
            kind: "Connection".into(),
            source: "root@out-a".into(),
            target: "child@in-a".into(),
            properties: {
                let mut p = PropertyBag::new();
                p.insert("u".into(), PropertyValue::Number(1.2));
                p.insert("v".into(), PropertyValue::Number(-0.6));
                p
            },
        }],
        Some("root".into()),
    );
    ::store::os_store::test_support::assert_dsl_round_trip(&fixture);
    ::store::os_store::test_support::assert_dsl_pack_equivalence(&fixture);
}
