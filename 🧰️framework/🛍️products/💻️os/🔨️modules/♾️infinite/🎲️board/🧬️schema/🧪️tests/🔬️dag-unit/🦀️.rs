//! 🧪️ schema tests (🔬️dag-unit): compiled against the `testkit` surface of the board crates.

use semio_framework_os_infinite_board_schema::DagScreenHit;
use semio_framework_pack_json::JsonMemberPolicy;
use semio_framework_os_infinite_directed_presentation::*;

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.
fn read_static(path_from_manifest: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path_from_manifest);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("test input {}: {error}", path.display())).into_boxed_str())
}

#[test]
fn dag_node_spec_serde_round_trip_kinds() {
    let nodes = vec![
        DagNodeSpec::computation(
            "c".into(),
            "C",
            "C",
            "emoji:🔷️".into(),
            vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }],
            vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }],
            false,
            false,
            0.0,
            0.0,
            160.0,
            56.0,
        ),
        DagNodeSpec {
            id: "s".into(),
            name: "S".into(),
            abbreviation: "S".into(),
            icon: "emoji:🎚️".into(),
            x: 1.0,
            y: 2.0,
            width: 180.0,
            height: 80.0,
            kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 3.0, output: IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() } },
            ..Default::default()
        },
        DagNodeSpec {
            id: "m".into(),
            name: "M".into(),
            abbreviation: "M".into(),
            icon: "emoji:📋️".into(),
            x: 0.0,
            y: 0.0,
            width: 180.0,
            height: 80.0,
            kind: DagNodeKind::Select { options: vec!["A".into(), "B".into()], selected: 1, output: IoPortSpec { id: "out".into(), label: "mode".into(), ..Default::default() } },
            ..Default::default()
        },
        DagNodeSpec {
            id: "p".into(),
            name: "P".into(),
            abbreviation: "P".into(),
            icon: "emoji:🖥️".into(),
            x: 0.0,
            y: 0.0,
            width: 200.0,
            height: 140.0,
            kind: DagNodeKind::Screen { media: Some(DagMedia { kind: DagMediaKind::Svg, src: "data:image/svg+xml,test".into() }), input: IoPortSpec { id: "in".into(), label: "result".into(), ..Default::default() } },
            ..Default::default()
        },
    ];
    for node in nodes {
        let json = semio_framework_pack_json::to_json_string(&node);
        let back: DagNodeSpec = semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(node, back);
    }
}

#[test]
fn dag_node_spec_port_accessors_per_kind() {
    let slider = DagNodeSpec {
        id: "s".into(),
        name: "S".into(),
        abbreviation: "S".into(),
        icon: "emoji:🎚️".into(),
        x: 0.0,
        y: 0.0,
        width: 180.0,
        height: 80.0,
        kind: DagNodeKind::Slider { min: 0.0, max: 1.0, step: 0.1, value: 0.5, output: IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() } },
        ..Default::default()
    };
    assert!(slider.inputs().is_empty());
    assert_eq!(slider.outputs().len(), 1);
    let screen = DagNodeSpec {
        id: "p".into(),
        name: "P".into(),
        abbreviation: "P".into(),
        icon: "emoji:🖥️".into(),
        x: 0.0,
        y: 0.0,
        width: 200.0,
        height: 140.0,
        kind: DagNodeKind::Screen { media: None, input: IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() } },
        ..Default::default()
    };
    assert_eq!(screen.inputs().len(), 1);
    assert!(screen.outputs().is_empty());
}

#[test]
fn io_port_label_with_cardinality_prefixes_symbol() {
    let mut port = IoPortSpec::named("S", "Sld", "solid", "ExtrudedSolid");
    port.cardinality = "*".into();
    assert_eq!(port.label_with_cardinality(DagDrawLod::Normal), "* Sld");
}

#[test]
fn io_port_label_with_cardinality_marks_unresolved_blocked_inputs() {
    let mut port = IoPortSpec::named("R", "Rad", "radius", "Radius");
    port.resolved = Some(false);
    assert_eq!(port.label_with_cardinality(DagDrawLod::Normal), "? Rad");
    assert_eq!(port.label_with_cardinality(DagDrawLod::Detail), "? radius");
}

#[test]
fn io_port_display_label_follows_draw_lod() {
    let port = IoPortSpec::named("S", "Sld", "solid", "ExtrudedSolid");
    assert_eq!(port.display_label(DagDrawLod::Normal), "Sld");
    assert_eq!(port.display_label(DagDrawLod::Detail), "solid");
    assert_eq!(port.display_label(DagDrawLod::Micro), "ExtrudedSolid");
}

/// 🏷️ Zooming IN never shortens a caption: `Detail` sits above `Normal` and says the same word.
///
/// 🔌️ Ports are grabbable wherever a node is drawn — the host publishes every port's screen rect
/// at every zoom (`entity_screen_json("handle", …)`), so every tier but the silhouette must accept
/// a press on one.
#[test]
fn dag_draw_lod_progressive_disclosure_gates() {
    assert_eq!(DagDrawLod::Normal.node_label(), DagNodeLabel::Name);
    assert_eq!(DagDrawLod::Detail.node_label(), DagNodeLabel::Name);
    assert!(DagDrawLod::Normal.shows_computation_layout());
    assert!(!DagDrawLod::Compact.shows_computation_layout());
    assert!(!DagDrawLod::Normal.shows_handles());
    assert!(DagDrawLod::Detail.shows_handles());
    assert!(DagDrawLod::Detail.uses_channel_row_pick());
    assert!(DagDrawLod::Micro.uses_channel_row_pick());
    assert!(!DagDrawLod::Normal.uses_channel_row_pick());
    assert!(!DagDrawLod::Minimap.allows_connection_hit_picking());
    assert!(DagDrawLod::Overview.allows_connection_hit_picking());
    assert!(DagDrawLod::Compact.allows_connection_hit_picking());
    assert!(DagDrawLod::Normal.allows_connection_hit_picking());
    assert!(DagDrawLod::Detail.allows_connection_hit_picking());
    assert!(DagDrawLod::Micro.allows_connection_hit_picking());
    assert!(DagDrawLod::Normal.shows_port_labels());
    assert!(DagDrawLod::Detail.shows_port_labels());
    assert!(DagDrawLod::Micro.shows_port_labels());
    assert_eq!(DagDrawLod::Minimap.edge_stroke_screen_px(), DAG_EDGE_STROKE_MINIMAP_SCREEN_PX);
    assert_eq!(DagDrawLod::Normal.edge_stroke_screen_px(), DAG_EDGE_STROKE_SCREEN_PX);
}

#[test]
fn to_pascal_case_normalizes_spaced_labels() {
    assert_eq!(to_pascal_case("pass through"), "PassThrough");
    assert_eq!(to_pascal_case("Draw Rectangle"), "DrawRectangle");
}

#[test]
fn dag_node_spec_round_trips_display_fields() {
    let node = DagNodeSpec::computation("n".into(), "pass through", "pass", "emoji:➡️".into(), vec![], vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }], false, false, 0.0, 0.0, 80.0, 24.0);
    let json = semio_framework_pack_json::to_json_string(&node);
    let back: DagNodeSpec = semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(back.name, "PassThrough");
    assert_eq!(back.abbreviation, "Pass");
    assert_eq!(back.icon, "emoji:➡️");
}

#[test]
fn cluster_node_round_trips_serde() {
    let inputs = vec![IoPortSpec::simple("a", "a")];
    let outputs = vec![IoPortSpec::simple("out", "out")];
    let node = DagNodeSpec::cluster("cluster".into(), "Cluster", "Cluster", "emoji:🧩️".into(), inputs, outputs, 10.0, 20.0, 120.0, 80.0);
    let json = semio_framework_pack_json::to_json_string(&node);
    let back: DagNodeSpec = semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(matches!(back.kind, DagNodeKind::Cluster { .. }));
}

#[test]
fn screen_hit_projection_matches_neutral_and_serde_classification() {
    use DagScreenHit;
    let law:serde_json::Value=serde_json::from_str(read_static("../../../🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🧫️screen-hit.json")).unwrap();
    for case in law["cases"].as_array().unwrap(){
        let reference:DagScreenHit=serde_json::from_value(case["hit"].clone()).unwrap();
        let original:DagScreenHit=semio_framework_pack_json::from_json_str(&case["hit"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(original,reference);
        assert_eq!(original.is_screen_path(),case["screenPath"].as_bool().unwrap());
        assert_eq!(original.is_draggable_body(),case["draggable"].as_bool().unwrap());
        let emitted:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&original)).unwrap();
        assert_eq!(emitted,case["hit"]);
    }
    eprintln!("[DEBUG] all neutral screen hit domains share the serde classification");
}
