
use super::*;

#[test]
fn canvas2d_scene_round_trips_with_and_without_snapshot() {
    let bare = Canvas2dScene { camera_x: 1.5, camera_y: -2.0, zoom: 1.0, layers_json: "[]".into(), snapshot: None };
    assert_eq!(Canvas2dScene::from_value(bare.to_value()), Ok(bare.clone()));
    assert!(matches!(bare.to_value(), DslValue::Object(entries) if !entries.iter().any(|(k, _)| k == "snapshot")));

    let leased = Canvas2dScene { snapshot: Some(crate::Canvas2dSnapshotLease { slot: 1, epoch: 2, revision: 3, generation: 4, page_count: 1, byte_count: 16 }), ..bare };
    assert_eq!(Canvas2dScene::from_value(leased.to_value()), Ok(leased));
}

#[test]
fn world3d_scene_round_trips_dense_and_bare_and_keeps_integers_as_integers() {
    let bare = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
    assert_eq!(World3dScene::from_value(bare.to_value()), Ok(bare.clone()));

    let mut dense = bare.clone();
    dense.domain_id = Some("cad".into());
    dense.domain_granularity_id = Some("handle".into());
    dense.snapshot = Some(crate::World3dSnapshotLease { slot: 2, epoch: 3, revision: 4, generation: 5, page_count: 6, item_count: 7, byte_count: 8 });
    let encoded = dense.to_value();
    assert_eq!(World3dScene::from_value(encoded.clone()), Ok(dense));
    let DslValue::Object(entries) = &encoded else { panic!("expected an object") };
    let snapshot_entries = match entries.iter().find(|(k, _)| k == "snapshot").map(|(_, v)| v) {
        Some(DslValue::Object(entries)) => entries,
        other => panic!("expected the nested snapshot lease to encode as an object, found {other:?}"),
    };
    let slot = snapshot_entries.iter().find(|(k, _)| k == "slot").map(|(_, v)| v.clone());
    assert!(matches!(slot, Some(DslValue::Number(protocol::value::Number::UInt(2)))), "u8 field must stay an integer, found {slot:?}");
}

#[test]
fn missing_required_field_reports_the_field_name() {
    let empty = DslValue::object([]);
    assert_eq!(TableScene::from_value(empty), Err(ValueError::new("missing field `columnsJson`")));
}

/// 🕸️ `NodeGraphScene` is the deepest nesting in this crate — it embeds `NodeGraphNodeRecord`
/// (itself embedding `NodeGraphPortRecord`), `NodeGraphEdgeRecord`, `NodeGraphViewport`,
/// `NodeGraphHover`, and `NodeGraphOperatorRecord` (itself embedding
/// `NodeGraphOperatorChannelRecord`/`NodeGraphOperatorVariadicRecord`) — one round trip here
/// exercises every nested `NodeGraph*Record` codec added in this pass at once.
#[test]
fn node_graph_scene_round_trips_through_every_nested_record_type() {
    let port = NodeGraphPortRecord { id: "a".into(), label: Some("A".into()), code: None, abbreviation: None, full_name: None, artifact_kind: None };
    let node = NodeGraphNodeRecord { id: "n1".into(), label: Some("Node".into()), x: 1.0, y: 2.0, width: 100.0, height: 50.0, inputs: vec![port], outputs: Vec::new(), instance_id: Some("i1".into()), plugin_id: None, app_id: None, icon: None };
    let edge = NodeGraphEdgeRecord { id: "e1".into(), source_node_id: "n1".into(), source_port_id: "a".into(), target_node_id: "n1".into(), target_port_id: "a".into(), label: None };
    let variadic = NodeGraphOperatorVariadicRecord { slot_key: "vs".into(), min: 1, max: Some(4) };
    let channel =
        NodeGraphOperatorChannelRecord { code: "c".into(), abbreviation: "C".into(), name: "Chan".into(), full_name: "Channel".into(), operators: vec!["op".into()], default_json: Some("null".into()), label: None, cardinality: "one".into() };
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
    let mut scene = NodeGraphScene::base(vec![node], vec![edge], NodeGraphViewport { x: 1.0, y: 2.0, zoom: 1.5 });
    scene.hover = Some(NodeGraphHover { node_id: Some("n1".into()), port_id: Some("a".into()) });
    scene.operators = vec![operator];
    scene.find_items = vec![NodeGraphFindItem { id: "f1".into(), label: "Find".into(), category: "cat".into() }];
    scene.highlighted = vec!["n1".into()];

    assert_eq!(NodeGraphScene::from_value(scene.to_value()), Ok(scene));
}

#[test]
fn node_graph_viewport_and_hover_round_trip_including_all_none() {
    let viewport = NodeGraphViewport { x: 0.0, y: 0.0, zoom: 1.0 };
    assert_eq!(NodeGraphViewport::from_value(viewport.to_value()), Ok(viewport));
    let hover = NodeGraphHover { node_id: None, port_id: None };
    assert_eq!(NodeGraphHover::from_value(hover.to_value()), Ok(hover.clone()));
    assert_eq!(hover.to_value(), DslValue::object([]));
}

#[test]
fn table_scene_and_tiled_map_scene_round_trip() {
    let table = TableScene { columns_json: "[]".into(), rows_json: "[]".into(), selection_json: Some("{}".into()), row_drag_mime: None, drop_action_json: None, sort_json: None, domain_id: Some("d".into()) };
    assert_eq!(TableScene::from_value(table.to_value()), Ok(table));

    let tiled = TiledMapScene::base("{}".into(), "{}".into());
    assert_eq!(TiledMapScene::from_value(tiled.to_value()), Ok(tiled.clone()));
    // 🕳️ Every field but `mapFixtureJson`/`cameraJson` carries a `#[serde(default = ...)]`
    // fallback — omitting just the defaulted ones must reproduce `base`'s own defaults exactly,
    // matching `serde`'s behaviour for a missing key on a `#[serde(default = "fn")]` field.
    assert_eq!(TiledMapScene::from_value(DslValue::object([("mapFixtureJson".to_string(), DslValue::String("{}".into())), ("cameraJson".to_string(), DslValue::String("{}".into()))])), Ok(tiled));
}

#[test]
fn board2d_scene_and_block_list_scene_round_trip() {
    let board = Board2dScene::base("{}".into(), "{}".into(), true);
    assert_eq!(Board2dScene::from_value(board.to_value()), Ok(board));

    let blocks = BlockListScene { steps_json: "[]".into(), palette_json: "[]".into(), selected_id: Some("s1".into()), dragging_id: None, domain_id: None };
    assert_eq!(BlockListScene::from_value(blocks.to_value()), Ok(blocks));
}
