
use super::*;

#[test]
fn puzzle2d_edge_connection_params_default_to_zero() {
    let edge = Puzzle2dEdge::default();
    assert_eq!(edge.gap, 0.0);
    assert_eq!(edge.shift, 0.0);
    assert_eq!(edge.rise, 0.0);
    assert_eq!(edge.rotation, 0.0);
    assert_eq!(edge.turn, 0.0);
    assert_eq!(edge.tilt, 0.0);
    assert_eq!(edge.x, 0.0);
    assert_eq!(edge.y, 0.0);
}

#[test]
fn puzzle2d_node_anchor_defaults_to_fixed() {
    let node = Puzzle2dNode::default();
    assert_eq!(node.anchor, Puzzle2dNodeAnchor::Fixed);
}

#[test]
fn puzzle2d_edge_serde_roundtrips_connection_params() {
    let edge = Puzzle2dEdge { id: "e1".into(), source: "a".into(), target: "b".into(), gap: 1.0, shift: 2.0, rise: 3.0, rotation: 10.0, turn: 20.0, tilt: 30.0, x: 4.0, y: 5.0, ..Default::default() };
    let json = serde_json::to_string(&edge).expect("serialize");
    let back: Puzzle2dEdge = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, edge);
    assert!(json.contains("\"gap\":1.0") || json.contains("\"gap\":1"));
    assert!(json.contains("\"rotation\":10"));
}

#[test]
fn puzzle2d_kind_compatibility_includes_important() {
    let row = Puzzle2dKindCompatibility { source: "a".into(), target: "b".into(), bidirectional: true, important: true, specificity: Puzzle2dCompatSpecificity::Handle };
    let json = serde_json::to_value(&row).expect("serialize");
    assert_eq!(json["important"], true);
    assert_eq!(json["bidirectional"], true);
    assert_eq!(json["specificity"], "handle");
}

#[test]
fn puzzle2d_kind_catalogs_serde_roundtrip() {
    let catalogs = Puzzle2dKindCatalogs {
        nodes: vec![Puzzle2dCatalogNodeKind {
            id: "capsule".into(),
            name: "Capsule".into(),
            label: "Capsule".into(),
            description: "d".into(),
            icon: "i".into(),
            image: "img".into(),
            unit: "m".into(),
            is_abstract: false,
            base_kinds: vec!["base".into()],
            representations: vec![Puzzle2dRepresentation { id: "r1".into(), name: "mesh".into(), url: "u".into(), mime: "model/gltf-binary".into(), tags: vec!["lod0".into()], lod: Some("0".into()), description: "rep".into() }],
            handles: vec![Puzzle2dHandleTemplate {
                id: "h0".into(),
                name: "bottom".into(),
                label: "Bottom".into(),
                description: "".into(),
                icon: "".into(),
                handle_kind: Some("core.rect.bottom".into()),
                angle: 0.0,
                t: Some(0.5),
                mandatory: Some(true),
                radius: Some(3.0),
            }],
            attributes: vec![Puzzle2dAttribute { id: "a1".into(), key: "k".into(), value: "v".into(), definition: None }],
            authors: vec![Puzzle2dAuthor { id: "u1".into(), name: "Ada".into(), email: "a@b.c".into(), role: Some("author".into()), rank: Some(1) }],
        }],
        handles: vec![Puzzle2dCatalogHandleKind {
            id: "core.rect.bottom".into(),
            code: Some("B".into()),
            label: Some("Bottom".into()),
            order: Some(0),
            compatible_with: vec!["core.rect.top".into()],
            description: "".into(),
            icon: "".into(),
            color: "#112233".into(),
            default_wire_kind: "link.w".into(),
        }],
        edges: vec![Puzzle2dCatalogEdgeKind { id: "link.e".into(), name: "Link".into(), label: "Link".into(), description: "".into(), icon: "".into(), color: "#000".into() }],
        wires: vec![Puzzle2dCatalogWireKind { id: "link.w".into(), name: "W".into(), label: "W".into(), description: "".into(), icon: "".into(), color: "#111".into(), default_edge_kind: "link.e".into() }],
    };
    let json = serde_json::to_value(&catalogs).expect("serialize");
    assert_eq!(json["nodes"][0]["abstract"], false);
    let back: Puzzle2dKindCatalogs = serde_json::from_value(json).expect("deserialize");
    assert_eq!(back, catalogs);
}
