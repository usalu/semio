use crate::JackWorkingScene;
use super::*;
use crate::{Camera, Manifest, Port, PortDirection, PropertyBag};

//#region 🧸️Fixtures
fn mini_fixture() -> JackSnapshot {
    JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "mini".into(), Some("nakagin".into()), Manifest::nakagin_default(), Camera::default(), JackWorkingScene { nodes: vec![
            Node {
                id: "root".into(),
                kind: "Piece".into(),
                name: "core".into(),
                x: 0.0,
                y: 0.0,
                width: 80.0,
                height: 40.0,
                properties: PropertyBag::new(),
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
        ], edges: vec![Edge {
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
        }] }, Some("root".into()))
}
//#endregion 🧸️Fixtures

//#region 🧪️FlatPositionLaws
#[semio_framework_async_macros::async_test]
async fn flat_position_bfs_walks_from_root() {
    let flat = compute_flat_position(&mini_fixture());
    assert_eq!(flat.positions.get("root"), Some(&JackFlatPositionUv { u: 0.0, v: 0.0 }));
    assert_eq!(flat.positions.get("child"), Some(&JackFlatPositionUv { u: 1.2, v: -0.6 }));
}

#[semio_framework_async_macros::async_test]
async fn flat_position_covers_disconnected_components() {
    let fixture = JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "disconnected".into(), Some("nakagin".into()), Manifest::nakagin_default(), Camera::default(), JackWorkingScene { nodes: vec![
            Node {
                id: "root-a".into(),
                kind: "Piece".into(),
                name: "a".into(),
                x: 0.0,
                y: 0.0,
                width: 80.0,
                height: 40.0,
                properties: PropertyBag::new(),
                ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
            },
            Node {
                id: "child-a".into(),
                kind: "Piece".into(),
                name: "a-child".into(),
                x: 100.0,
                y: 0.0,
                width: 80.0,
                height: 40.0,
                properties: PropertyBag::new(),
                ports: vec![Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
            },
            Node {
                id: "root-b".into(),
                kind: "Piece".into(),
                name: "b".into(),
                x: 300.0,
                y: 200.0,
                width: 80.0,
                height: 40.0,
                properties: PropertyBag::new(),
                ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
            },
            Node {
                id: "child-b".into(),
                kind: "Piece".into(),
                name: "b-child".into(),
                x: 400.0,
                y: 200.0,
                width: 80.0,
                height: 40.0,
                properties: PropertyBag::new(),
                ports: vec![Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
            },
        ], edges: vec![
            Edge {
                id: "e-a".into(),
                kind: "Connection".into(),
                source: "root-a@out".into(),
                target: "child-a@in".into(),
                properties: {
                    let mut p = PropertyBag::new();
                    p.insert("u".into(), PropertyValue::Number(2.0));
                    p.insert("v".into(), PropertyValue::Number(1.0));
                    p
                },
            },
            Edge {
                id: "e-b".into(),
                kind: "Connection".into(),
                source: "root-b@out".into(),
                target: "child-b@in".into(),
                properties: {
                    let mut p = PropertyBag::new();
                    p.insert("u".into(), PropertyValue::Number(3.0));
                    p.insert("v".into(), PropertyValue::Number(-1.0));
                    p
                },
            },
        ] }, Some("root-a".into()));
    let flat = compute_flat_position(&fixture);
    assert_eq!(flat.positions.get("child-a"), Some(&JackFlatPositionUv { u: 2.0, v: 1.0 }));
    assert_eq!(flat.positions.get("child-b"), Some(&JackFlatPositionUv { u: 3.0, v: -1.0 }));
}

#[semio_framework_async_macros::async_test]
async fn flat_position_handles_cycles_without_looping() {
    let fixture = JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "cycle".into(), Some("nakagin".into()), Manifest::nakagin_default(), Camera::default(), JackWorkingScene { nodes: vec![
            Node {
                id: "a".into(),
                kind: "Piece".into(),
                name: "a".into(),
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 10.0,
                properties: PropertyBag::new(),
                ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
            },
            Node {
                id: "b".into(),
                kind: "Piece".into(),
                name: "b".into(),
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 10.0,
                properties: PropertyBag::new(),
                ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
            },
        ], edges: vec![
            Edge {
                id: "ab".into(),
                kind: "Connection".into(),
                source: "a@out".into(),
                target: "b@out".into(),
                properties: {
                    let mut p = PropertyBag::new();
                    p.insert("u".into(), PropertyValue::Number(1.0));
                    p.insert("v".into(), PropertyValue::Number(0.0));
                    p
                },
            },
            Edge { id: "ba".into(), kind: "Connection".into(), source: "b@out".into(), target: "a@out".into(), properties: PropertyBag::new() },
        ] }, Some("a".into()));
    let flat = compute_flat_position(&fixture);
    assert!(flat.positions.contains_key("a"));
    assert!(flat.positions.contains_key("b"));
}

#[semio_framework_async_macros::async_test]
async fn flat_position_empty_snapshot_yields_default() {
    assert_eq!(compute_flat_position(&JackSnapshot::default()), JackFlatPosition::default());
}
//#endregion 🧪️FlatPositionLaws
