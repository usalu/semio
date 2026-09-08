
use super::*;

#[test]
fn fastener_defaults_include_diagram_xy() {
    let fastener: Puzzle5dFastener = serde_json::from_value(serde_json::json!({
        "id": "f1",
        "source": "p1:g0",
        "target": "p2:g0"
    }))
    .unwrap();
    assert_eq!(fastener.gap, 0.0);
    assert_eq!(fastener.x, 0.0);
    assert_eq!(fastener.y, 0.0);
    assert_eq!(fastener.rotation, 0.0);
}

#[test]
fn fastener_round_trips_eight_transform_params() {
    let fastener = Puzzle5dFastener { id: "f1".into(), source: "p1:g0".into(), target: "p2:g0".into(), fastener_kind: Some("fk".into()), gap: 1.0, shift: 2.0, rise: 3.0, rotation: 4.0, turn: 5.0, tilt: 6.0, x: 7.0, y: 8.0 };
    let value = serde_json::to_value(&fastener).unwrap();
    assert_eq!(value["x"], 7.0);
    assert_eq!(value["y"], 8.0);
    let back: Puzzle5dFastener = serde_json::from_value(value).unwrap();
    assert_eq!(back, fastener);
}

#[test]
fn part_anchor_defaults_to_fixed() {
    let part: Puzzle5dPart = serde_json::from_value(serde_json::json!({ "id": "p1" })).unwrap();
    assert_eq!(part.anchor, Puzzle5dPartAnchor::Fixed);
    let derived: Puzzle5dPart = serde_json::from_value(serde_json::json!({ "id": "p2", "anchor": "derived" })).unwrap();
    assert_eq!(derived.anchor, Puzzle5dPartAnchor::Derived);
}

#[test]
fn kind_compatibility_unifies_important_and_specificity() {
    let row: Puzzle5dKindCompatibility = serde_json::from_value(serde_json::json!({
        "source": "a",
        "target": "b",
        "bidirectional": true,
        "important": true,
        "specificity": "grip"
    }))
    .unwrap();
    assert!(row.important);
    assert_eq!(row.specificity, Puzzle5dCompatSpecificity::Grip);
    let sparse: Puzzle5dKindCompatibility = serde_json::from_value(serde_json::json!({
        "source": "a",
        "target": "b"
    }))
    .unwrap();
    assert!(!sparse.important);
    assert_eq!(sparse.specificity, Puzzle5dCompatSpecificity::General);
}

#[test]
fn catalog_part_kind_carries_representations_and_grip_templates() {
    let kind = Puzzle5dCatalogPartKind {
        id: "hex".into(),
        name: "Hex".into(),
        label: "Hex".into(),
        description: "cut".into(),
        icon: "hexagon".into(),
        image: "".into(),
        unit: "m".into(),
        is_abstract: false,
        base_kinds: vec!["solid".into()],
        representations: vec![Puzzle5dRepresentation { id: "lod0".into(), name: "mesh".into(), url: "/mesh/hex.glb".into(), mime: "model/gltf-binary".into(), tags: vec!["mesh".into()], lod: Some("0".into()), description: "".into() }],
        grips: vec![Puzzle5dGripTemplate {
            id: "g0".into(),
            name: "north".into(),
            label: "N".into(),
            grip_kind: Some("b-l".into()),
            point: [1.0, 2.0, 3.0],
            direction: [0.0, 1.0, 0.0],
            t: Some(0.25),
            mandatory: Some(true),
            radius: Some(0.36),
            ..Default::default()
        }],
        attributes: vec![Puzzle5dAttribute { id: "a1".into(), key: "material".into(), value: "concrete".into(), definition: None }],
        authors: vec![Puzzle5dAuthor { id: "u1".into(), name: "Ada".into(), email: "ada@semio.tech".into(), role: Some("author".into()), rank: Some(1) }],
    };
    let value = serde_json::to_value(&kind).unwrap();
    assert_eq!(value["abstract"], false);
    assert_eq!(value["representations"][0]["url"], "/mesh/hex.glb");
    assert_eq!(value["grips"][0]["point"], serde_json::json!([1.0, 2.0, 3.0]));
    let back: Puzzle5dCatalogPartKind = serde_json::from_value(value).unwrap();
    assert_eq!(back.grips[0].direction, [0.0, 1.0, 0.0]);
    assert_eq!(back.authors[0].name, "Ada");
}

#[test]
fn grip_template_direction_defaults_to_positive_z() {
    let template: Puzzle5dGripTemplate = serde_json::from_value(serde_json::json!({ "id": "g0" })).unwrap();
    assert_eq!(template.direction, [0.0, 0.0, 1.0]);
}

#[test]
fn catalog_grip_kind_is_port_like() {
    let kind = Puzzle5dCatalogGripKind {
        id: "b-l".into(),
        code: Some("BL".into()),
        label: Some("Long".into()),
        order: Some(1),
        compatible_with: vec!["b-l".into(), "b-s".into()],
        description: "long bond".into(),
        icon: "link".into(),
        color: "hsl(206 52% 48%)".into(),
        default_rope_kind: "cable.link".into(),
    };
    let value = serde_json::to_value(&kind).unwrap();
    assert_eq!(value["compatibleWith"], serde_json::json!(["b-l", "b-s"]));
    let back: Puzzle5dCatalogGripKind = serde_json::from_value(value).unwrap();
    assert_eq!(back.order, Some(1));
}
