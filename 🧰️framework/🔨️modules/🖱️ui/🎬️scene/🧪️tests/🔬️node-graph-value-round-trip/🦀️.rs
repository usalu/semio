//! 🔁️ Value-codec round trips of the node-graph scene, its records and its viewport.

use super::*;

/// 🕸️ `NodeGraphScene` is the deepest nesting in this crate — it embeds `NodeGraphNodeRecord`
/// (itself embedding `NodeGraphPortRecord`), `NodeGraphEdgeRecord`, shared `Viewport2d` and
/// `NodeGraphHover` — one round trip here exercises every nested `NodeGraph*Record` the SCENE still
/// carries. `scene.operators` stays DOCUMENT-derived (see its own docstring); the registered operator
/// catalogue is app-static and rides the reserved `framework.section.catalogue` surface, which is why
/// the record family also has its own round-trip law below (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn node_graph_scene_round_trips_through_every_nested_record_type() {
    let port = NodeGraphPortRecord { id: "a".into(), label: Some("A".into()), code: None, abbreviation: None, full_name: None, artifact_kind: None, value_type: Some("geometry".into()) };
    let node = NodeGraphNodeRecord { id: "n1".into(), label: Some("Node".into()), x: 1.0, y: 2.0, width: 100.0, height: 50.0, inputs: vec![port], outputs: Vec::new(), instance_id: Some("i1".into()), plugin_id: None, app_id: None, icon: None };
    let edge = NodeGraphEdgeRecord { id: "e1".into(), source_node_id: "n1".into(), source_port_id: "a".into(), target_node_id: "n1".into(), target_port_id: "a".into(), label: None };
    let mut scene = NodeGraphScene::base(vec![node], vec![edge], semio_framework_ui_viewport::Viewport2d { x: 1.0, y: 2.0, zoom: 1.5 });
    scene.hover = Some(NodeGraphHover { node_id: Some("n1".into()), port_id: Some("a".into()) });
    scene.find_items = vec![NodeGraphFindItem { id: "f1".into(), label: "Find".into(), category: "cat".into() }];
    scene.highlighted = vec!["n1".into()];

    assert_eq!(NodeGraphScene::from_value(scene.to_value()), Ok(scene));
}

/// 🛍️ The operator catalogue record family — `NodeGraphOperatorRecord` over
/// `NodeGraphOperatorChannelRecord`/`NodeGraphOperatorVariadicRecord` — is the app-static catalogue
/// payload's own vocabulary, carried once per app instance on the reserved catalogue surface rather
/// than on every scene. Its codec still has to round-trip exactly.
#[test]
fn node_graph_operator_catalogue_records_round_trip() {
    let variadic = NodeGraphOperatorVariadicRecord { slot_key: "vs".into(), min: 1, max: Some(4) };
    let channel =
        NodeGraphOperatorChannelRecord { code: "c".into(), abbreviation: "C".into(), name: "Chan".into(), full_name: "Channel".into(), operators: vec!["op".into()], value_types: vec!["geometry".into()], item_types: Vec::new(), default_json: Some("null".into()), label: None, cardinality: "one".into() };
    let operator = NodeGraphOperatorRecord {
        id: "op1".into(),
        extension: "core".into(),
        name: "Op".into(),
        abbreviation: "O".into(),
        icon: "icon".into(),
        summary: "sums".into(),
        inputs: vec![channel.clone()],
        outputs: vec![channel],
        variadic_input: Some(variadic.clone()),
        variadic_output: Some(variadic),
        group: vec!["g".into()],
    };
    assert_eq!(NodeGraphOperatorRecord::from_value(operator.to_value()), Ok(operator));
}

#[test]
fn node_graph_viewport_and_hover_round_trip_including_all_none() {
    let viewport = semio_framework_ui_viewport::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 };
    assert_eq!(semio_framework_ui_viewport::Viewport2d::from_value(viewport.to_value()), Ok(viewport));
    let hover = NodeGraphHover { node_id: None, port_id: None };
    assert_eq!(NodeGraphHover::from_value(hover.to_value()), Ok(hover.clone()));
    assert_eq!(hover.to_value(), DslValue::object([]));
}
