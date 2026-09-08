
use super::*;

#[test]
fn attraction_exposes_eight_connection_parameters_with_zero_defaults() {
    let attraction = Puzzle3dAttraction { id: "a".into(), attracting: "o1:v0".into(), attracted: "o2:v0".into(), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 };
    let json = serde_json::to_value(&attraction).expect("serialize");
    for key in ["gap", "shift", "rise", "rotation", "turn", "tilt", "x", "y"] {
        assert_eq!(json.get(key).and_then(|v| v.as_f64()), Some(0.0), "{key}");
    }
    let parsed: Puzzle3dAttraction = serde_json::from_value(serde_json::json!({
        "attracting": "o1:v0",
        "attracted": "o2:v0"
    }))
    .expect("sparse attraction deserializes");
    assert_eq!(parsed.x, 0.0);
    assert_eq!(parsed.y, 0.0);
    assert_eq!(parsed.gap, 0.0);
}

#[test]
fn object_anchor_defaults_to_fixed() {
    let object: Puzzle3dObject = serde_json::from_value(serde_json::json!({
        "id": "o1"
    }))
    .expect("object");
    assert_eq!(object.anchor, Puzzle3dObjectAnchor::Fixed);
    assert_eq!(serde_json::to_value(Puzzle3dObjectAnchor::Derived).unwrap(), serde_json::json!("derived"));
}

#[test]
fn object_kind_is_type_like_with_representations() {
    let kind = Puzzle3dCatalogObjectKind {
        id: "Capsule".into(),
        name: "Capsule".into(),
        label: "Capsule".into(),
        description: "demo".into(),
        icon: "".into(),
        image: "".into(),
        unit: "m".into(),
        is_abstract: false,
        base_kinds: vec!["Part".into()],
        representations: vec![Puzzle3dRepresentation { id: "mesh".into(), name: "mesh".into(), url: "/mesh/capsule.glb".into(), mime: "model/gltf-binary".into(), tags: vec!["default".into()], lod: Some("high".into()), description: "".into() }],
        vortices: vec![Puzzle3dCatalogVortexTemplate {
            id: "v0".into(),
            name: "v0".into(),
            label: "v0".into(),
            description: "".into(),
            icon: "".into(),
            vortex_kind: Some("c-t".into()),
            point: [0.0, 0.0, 3.0],
            direction: [0.0, 0.0, 1.0],
            t: Some(0.25),
            mandatory: Some(true),
            radius: Some(0.36),
        }],
        attributes: vec![Puzzle3dAttribute { id: "a1".into(), key: "material".into(), value: "concrete".into(), definition: None }],
        authors: vec![Puzzle3dAuthor { id: "u1".into(), name: "Ada".into(), email: "ada@example.com".into(), role: Some("author".into()), rank: Some(1) }],
    };
    let json = serde_json::to_value(&kind).expect("serialize");
    assert_eq!(json.get("abstract").and_then(|v| v.as_bool()), Some(false));
    assert!(json.get("meshUrl").is_none());
    assert_eq!(json["representations"][0]["url"], "/mesh/capsule.glb");
    assert_eq!(json["vortices"][0]["point"][2], 3.0);
    let round: Puzzle3dCatalogObjectKind = serde_json::from_value(json).expect("round");
    assert_eq!(round.vortices[0].point, [0.0, 0.0, 3.0]);
    assert_eq!(round.representations[0].mime, "model/gltf-binary");
}

#[test]
fn vortex_kind_is_port_like() {
    let kind = Puzzle3dCatalogVortexKind {
        id: "c-t".into(),
        code: Some("CT".into()),
        label: Some("ceiling top".into()),
        order: Some(2),
        compatible_with: vec!["c-b".into()],
        description: "ceiling".into(),
        icon: "".into(),
        color: "hsl(169 52% 48%)".into(),
        default_cable_kind: "cable.link".into(),
    };
    let json = serde_json::to_value(&kind).expect("serialize");
    assert_eq!(json["compatibleWith"][0], "c-b");
    assert_eq!(json["defaultCableKind"], "cable.link");
}

#[test]
fn kind_compatibility_uses_typed_specificity() {
    let rule = Puzzle3dKindCompatibility { source: "c-t".into(), target: "c-b".into(), bidirectional: true, important: false, specificity: Puzzle3dCompatSpecificity::Vortex };
    let json = serde_json::to_value(&rule).expect("serialize");
    assert_eq!(json["specificity"], "vortex");
    let parsed: Puzzle3dKindCompatibility = serde_json::from_value(serde_json::json!({
        "source": "a",
        "target": "b"
    }))
    .expect("defaults");
    assert_eq!(parsed.specificity, Puzzle3dCompatSpecificity::Vortex);
}
