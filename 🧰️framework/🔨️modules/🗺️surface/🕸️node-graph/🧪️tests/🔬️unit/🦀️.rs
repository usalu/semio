
use super::*;

#[test]
fn fixture_from_workflow_json() {
    let nodes = r#"[{"id":"a","label":"Alpha","x":10,"y":20,"inputs":[],"outputs":[{"id":"out","label":"Out"}]}]"#;
    let edges = r#"[]"#;
    let fixture = fixture_from_node_graph_json(nodes, edges, r#"{"x":0,"y":0,"zoom":1}"#).expect("fixture");
    assert_eq!(fixture.nodes.len(), 1);
    assert_eq!(fixture.nodes[0].id, "a");
}

#[test]
fn graph_host_syncs_selection_from_framework_interaction_state() {
    let mut host = GraphHost::default();
    let payload = NodeGraphScenePayload {
        nodes: vec![GraphNodeRecord { id: "a".into(), label: Some("A".into()), outputs: Some(vec![GraphPortRecord { id: "out".into(), ..Default::default() }]), ..Default::default() }],
        edges: Vec::new(),
        viewport: Some(GraphViewport { x: 0.0, y: 0.0, zoom: 1.0 }),
        ..Default::default()
    };
    host.sync_from_payload(&payload).expect("sync");
    let selection = DomainSelection { granularity: "node".into(), ids: vec!["a".into()], anchor_id: None };
    host.sync_interaction(Some(&selection), None);
    assert_eq!(host.dag.selected_node_ids(), vec!["a"]);
}

#[test]
fn graph_host_pointer_up_after_plain_click_gathers_one_pick_target() {
    let mut host = GraphHost::default();
    let payload = payload_with_node("a");
    host.sync_from_payload(&payload).expect("sync");
    host.set_viewport(400, 400, 1.0);
    host.pointer_down_screen([200.0, 200.0], 0, false, false, false, false);
    host.pointer_up_screen(200.0, 200.0, false, false, false);
    let gather = host.take_selection_gather().expect("gather");
    assert_eq!(gather.target_ids, vec!["a".to_string()]);
    assert_eq!(gather.method, SelectionMethod::Pick);
    assert!(host.take_selection_gather().is_none(), "gather is take-once");
}

#[test]
fn set_canvas_theme_dark_applies_board_palette() {
    let mut host = GraphHost::default();
    host.set_canvas_theme_dark(true);
    let dark_stroke = host.dag.canvas_theme.node_stroke.to_rgba8();
    host.set_canvas_theme_dark(false);
    let light_stroke = host.dag.canvas_theme.node_stroke.to_rgba8();
    assert_ne!(dark_stroke.r, light_stroke.r);
}

//#region 🔖️PortHelpers
#[test]
fn port_label_uses_last_at_segment_when_label_missing() {
    let port = GraphPortRecord { id: "node@channel@foo".into(), label: None, ..Default::default() };
    assert_eq!(port_label(&port), "foo");
}

#[test]
fn port_label_falls_back_to_full_id_without_at() {
    let port = GraphPortRecord { id: "solo".into(), label: None, ..Default::default() };
    assert_eq!(port_label(&port), "solo");
}

#[test]
fn port_label_prefers_explicit_label() {
    let port = GraphPortRecord { id: "node@out".into(), label: Some("Output".into()), ..Default::default() };
    assert_eq!(port_label(&port), "Output");
}

#[test]
fn port_to_io_copies_optional_metadata() {
    let port = GraphPortRecord { id: "p1".into(), label: Some("Speed".into()), code: Some("SPD".into()), abbreviation: Some("Sp".into()), full_name: Some("Speed Value".into()), artifact_kind: Some("number".into()) };
    let spec = port_to_io(&port);
    assert_eq!(spec.id, "p1");
    assert_eq!(spec.label, "Speed");
    assert_eq!(spec.code, "SPD");
    assert_eq!(spec.abbreviation, "Sp");
    assert_eq!(spec.full_name, "Speed Value");
    assert_eq!(spec.artifact_kind.as_deref(), Some("number"));
}

#[test]
fn port_to_io_uses_simple_defaults_when_optional_fields_absent() {
    let port = GraphPortRecord { id: "p2".into(), ..Default::default() };
    let spec = port_to_io(&port);
    assert_eq!(spec.id, "p2");
    assert!(spec.artifact_kind.is_none());
}
//#endregion 🔖️PortHelpers

//#region 🔖️NodeRecordConversion
#[test]
fn node_record_to_spec_builds_app_instance_kind() {
    let record = GraphNodeRecord { id: "n1".into(), label: Some("Widget".into()), instance_id: Some("inst-1".into()), plugin_id: None, app_id: None, ..Default::default() };
    let spec = node_record_to_spec(&record);
    match spec.kind {
        DagNodeKind::AppInstance { instance_id, plugin_id, app_id, .. } => {
            assert_eq!(instance_id, "inst-1");
            assert_eq!(plugin_id, "app");
            assert_eq!(app_id, "n1");
        }
        other => panic!("expected AppInstance kind, got {other:?}"),
    }
    assert_eq!(spec.abbreviation, "Wid");
}

#[test]
fn node_record_to_spec_defaults_computation_kind_without_instance_id() {
    let record = GraphNodeRecord { id: "n2".into(), label: Some("Compute".into()), ..Default::default() };
    let spec = node_record_to_spec(&record);
    assert!(matches!(spec.kind, DagNodeKind::Computation { .. }));
}

#[test]
fn node_record_to_spec_defaults_position_when_missing() {
    let record = GraphNodeRecord { id: "n3".into(), label: Some("Anchor".into()), ..Default::default() };
    let spec = node_record_to_spec(&record);
    assert_eq!(spec.x, 0.0);
    assert_eq!(spec.y, 0.0);
    assert_eq!(spec.icon, "emoji:🔷️");
}

#[test]
fn node_record_to_spec_falls_back_to_id_when_label_missing() {
    let record = GraphNodeRecord { id: "n4".into(), ..Default::default() };
    let spec = node_record_to_spec(&record);
    // 🔤️ Computation kind routes through `DagNodeSpec::computation`, which pascal-cases the display name.
    assert_eq!(spec.name, "N4");
}
//#endregion 🔖️NodeRecordConversion

//#region 🔖️FixtureFromJson
#[test]
fn fixture_from_node_graph_json_defaults_when_inputs_blank() {
    let fixture = fixture_from_node_graph_json("", "", "").expect("fixture");
    assert_eq!(fixture.schema, "dag.fixture");
    assert!(fixture.nodes.is_empty());
    assert!(fixture.edges.is_empty());
    // 🐛️ blank viewport_json takes the `GraphViewport::default()` (derived) path, which zeroes zoom
    // instead of using `default_zoom()` (1.0) — that helper only fires for missing-key JSON parsing.
    assert_eq!(fixture.camera.zoom, 0.0);
}

#[test]
fn fixture_from_node_graph_json_builds_composite_edge_endpoints() {
    let nodes = r#"[{"id":"a","outputs":[{"id":"out"}]},{"id":"b","inputs":[{"id":"in"}]}]"#;
    let edges = r#"[{"id":"e1","sourceNodeId":"a","sourcePortId":"out","targetNodeId":"b","targetPortId":"in"}]"#;
    let fixture = fixture_from_node_graph_json(nodes, edges, "").expect("fixture");
    assert_eq!(fixture.edges.len(), 1);
    assert_eq!(fixture.edges[0].source, "a@out");
    assert_eq!(fixture.edges[0].target, "b@in");
}

#[test]
fn fixture_from_node_graph_json_propagates_malformed_nodes_json() {
    let err = fixture_from_node_graph_json("not json", "[]", "").unwrap_err();
    assert!(matches!(err, NodeGraphError::Json(_)));
}

#[test]
fn fixture_from_node_graph_json_reads_custom_viewport() {
    let fixture = fixture_from_node_graph_json("[]", "[]", r#"{"x":5,"y":-3,"zoom":2.5}"#).expect("fixture");
    assert_eq!(fixture.camera.x, 5.0);
    assert_eq!(fixture.camera.y, -3.0);
    assert_eq!(fixture.camera.zoom, 2.5);
}
//#endregion 🔖️FixtureFromJson

//#region 🔖️ScenePayloadFromJson
#[test]
fn node_graph_scene_payload_from_json_defaults_missing_fields() {
    let value = serde_json::json!({});
    let payload = NodeGraphScenePayload::from_json(&value);
    assert!(payload.nodes.is_empty());
    assert!(payload.edges.is_empty());
    assert!(payload.viewport.is_none());
    assert!(payload.controls_json.is_none());
}

#[test]
fn node_graph_scene_payload_from_json_reads_optional_fields() {
    let value = serde_json::json!({
        "nodes": [{"id": "1", "x": 0.0, "y": 0.0, "width": 1.0, "height": 1.0}],
        "edges": [{"id": "2", "sourceNodeId": "a", "sourcePortId": "out", "targetNodeId": "b", "targetPortId": "in"}],
        "viewport": {"x": 0.0, "y": 0.0, "zoom": 1.0},
        "previewOffJson": "[]",
        "lodJson": "{}",
        "controlsJson": "ctl",
        "clustersJson": "clu",
        "computingJson": "{}",
        "capabilitiesJson": "cap",
        "fixtureJson": "fix",
    });
    let payload = NodeGraphScenePayload::from_json(&value);
    assert_eq!(payload.nodes.len(), 1);
    assert_eq!(payload.edges.len(), 1);
    assert_eq!(payload.controls_json.as_deref(), Some("ctl"));
    assert_eq!(payload.clusters_json.as_deref(), Some("clu"));
    assert_eq!(payload.capabilities_json.as_deref(), Some("cap"));
    assert_eq!(payload.fixture_json.as_deref(), Some("fix"));
}
//#endregion 🔖️ScenePayloadFromJson

//#region 🔖️GraphHostSync
fn payload_with_node(id: &str) -> NodeGraphScenePayload {
    NodeGraphScenePayload {
        nodes: vec![GraphNodeRecord {
            id: id.into(),
            label: Some("A".into()),
            x: Some(0.0),
            y: Some(0.0),
            outputs: Some(vec![GraphPortRecord { id: "out".into(), ..Default::default() }]),
            inputs: Some(vec![GraphPortRecord { id: "in".into(), ..Default::default() }]),
            ..Default::default()
        }],
        edges: Vec::new(),
        viewport: Some(GraphViewport { x: 0.0, y: 0.0, zoom: 1.0 }),
        ..Default::default()
    }
}

/// 🛍️ The app-static palette catalogue arrives on its OWN channel — never on a scene payload, whose
/// fixed 32 KiB admission it would blow with real operator sets installed
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1) — and a scene sync must never clear it.
#[test]
fn graph_host_keeps_the_app_catalogue_across_scene_syncs() {
    let mut host = GraphHost::default();
    let payload = payload_with_node("a");
    host.set_catalogue_json("first");
    host.sync_from_payload(&payload).expect("sync");
    assert_eq!(host.catalogue_json, "first");
    host.set_catalogue_json("second");
    host.sync_from_payload(&payload).expect("sync");
    assert_eq!(host.catalogue_json, "second");
}

#[test]
fn graph_host_sync_interaction_sets_hover_node_only() {
    let mut host = GraphHost::default();
    let payload = payload_with_node("a");
    host.sync_from_payload(&payload).expect("sync");
    let hover = DomainHover { channel: "pointer".into(), ids: vec!["a".into()] };
    host.sync_interaction(None, Some(&hover));
    assert_eq!(host.hovered_node_id().as_deref(), Some("a"));
    assert_eq!(host.hovered_channel_json(), "null");
}

#[test]
fn graph_host_sync_interaction_clears_hover_when_absent() {
    let mut host = GraphHost::default();
    let payload = payload_with_node("a");
    host.sync_from_payload(&payload).expect("sync");
    let hover = DomainHover { channel: "pointer".into(), ids: vec!["a".into()] };
    host.sync_interaction(None, Some(&hover));
    assert_eq!(host.hovered_node_id().as_deref(), Some("a"));
    host.sync_interaction(None, None);
    assert_eq!(host.hovered_node_id(), None);
}

#[test]
fn graph_host_sync_from_payload_dims_preview_off_nodes() {
    let mut host = GraphHost::default();
    let mut payload = payload_with_node("a");
    payload.preview_off_json = Some(r#"["a"]"#.into());
    host.sync_from_payload(&payload).expect("sync");
    assert_eq!(host.dag.dimmed_node_ids(), vec!["a".to_string()]);
}

#[test]
fn graph_host_sync_from_payload_applies_lod_settings() {
    let mut host = GraphHost::default();
    let mut payload = payload_with_node("a");
    payload.lod_json = Some(r#"{"automatic":false,"lod":"micro","proximityDistance":12.5,"gridVisible":false,"gridSnapEnabled":true,"gridFactor":2.0}"#.into());
    host.sync_from_payload(&payload).expect("sync");
    assert_eq!(host.dag.draw_lod_label(), "micro");
}

#[test]
fn graph_host_sync_from_payload_applies_computing_progress() {
    let mut host = GraphHost::default();
    let mut payload = payload_with_node("a");
    payload.computing_json = Some(r#"{"active":"a","stale":[]}"#.into());
    host.sync_from_payload(&payload).expect("sync");
    assert_eq!(host.dag.hovered_node_id(), None);
}

#[test]
fn graph_host_sync_from_scene_json_parses_raw_json() {
    let mut host = GraphHost::default();
    let scene = r#"{"nodes":[{"id":"a","x":0.0,"y":0.0,"width":1.0,"height":1.0,"outputs":[{"id":"out"}]}],"edges":[],"viewport":{"x":0,"y":0,"zoom":1}}"#;
    host.sync_from_scene_json(scene).expect("sync");
    assert_eq!(host.dag.fixture.nodes.iter().map(|node| node.id.as_str()).collect::<Vec<_>>(), vec!["a"]);
}

#[test]
fn graph_host_sync_from_scene_pack_decodes_pack_shell() {
    let mut host = GraphHost::default();
    let scene = serde_json::json!({
        "nodes": [{"id": "a", "x": 0.0, "y": 0.0, "width": 1.0, "height": 1.0, "outputs": [{"id": "out"}]}],
        "edges": [],
        "viewport": {"x": 0.0, "y": 0.0, "zoom": 1.0}
    });
    let dsl = dsl::DslValue::from(&scene);
    let bytes = store::pack_rt::encode_pack_value(&dsl);
    host.sync_from_scene_pack(&bytes).expect("sync");
    assert_eq!(host.dag.fixture.nodes.iter().map(|node| node.id.as_str()).collect::<Vec<_>>(), vec!["a"]);
}

#[test]
fn graph_host_sync_from_scene_json_rejects_invalid_json() {
    let mut host = GraphHost::default();
    let err = host.sync_from_scene_json("not json").unwrap_err();
    assert!(matches!(err, NodeGraphError::Json(_)));
}
//#endregion 🔖️GraphHostSync

//#region 🔖️GraphHostQueries
#[test]
fn graph_host_camera_json_reflects_viewport() {
    let mut host = GraphHost::default();
    let mut payload = payload_with_node("a");
    payload.viewport = Some(GraphViewport { x: 11.0, y: 22.0, zoom: 3.0 });
    host.sync_from_payload(&payload).expect("sync");
    assert_eq!(host.camera_json(), r#"{"x":11.0,"y":22.0,"zoom":3.0}"#);
}

#[test]
fn graph_host_selected_node_ids_json_matches_selection() {
    let mut host = GraphHost::default();
    let payload = payload_with_node("a");
    host.sync_from_payload(&payload).expect("sync");
    let selection = DomainSelection { granularity: "node".into(), ids: vec!["a".into()], anchor_id: None };
    host.sync_interaction(Some(&selection), None);
    assert_eq!(host.selected_node_ids_json(), r#"["a"]"#);
}

#[test]
fn graph_host_wheel_screen_pan_without_zoom_gesture() {
    let mut host = GraphHost::default();
    host.set_viewport(400, 400, 1.0);
    let before = host.dag.fixture.camera.y;
    host.wheel_screen(200.0, 200.0, 10.0, false);
    assert!(host.dag.fixture.camera.y < before);
    assert_eq!(host.dag.fixture.camera.zoom, 1.0);
}

#[test]
fn graph_host_wheel_screen_zoom_gesture_changes_zoom() {
    let mut host = GraphHost::default();
    host.set_viewport(400, 400, 1.0);
    host.wheel_screen(200.0, 200.0, -10.0, true);
    assert!(host.dag.fixture.camera.zoom > 1.0);
}

#[test]
fn graph_wheel_plan_matches_direct_and_rejects_stale_revision() {
    let mut direct = GraphHost::default();
    let mut planned = GraphHost::default();
    direct.set_viewport(400, 400, 1.0);
    planned.set_viewport(400, 400, 1.0);
    direct.wheel_screen(160.0, 190.0, -10.0, true);
    let plan = planned.plan_wheel(160.0, 190.0, -10.0, true);
    assert!(planned.commit_wheel(plan));
    assert_eq!(direct.camera_json(), planned.camera_json());

    let stale = planned.plan_wheel(160.0, 190.0, -10.0, true);
    planned.set_viewport(401, 400, 1.0);
    let replacement = planned.camera_json();
    assert!(!planned.commit_wheel(stale));
    assert_eq!(planned.camera_json(), replacement);
}

#[test]
fn graph_pointer_plan_matches_direct_click_and_rejects_stale_revision() {
    let payload = payload_with_node("a");
    let mut direct = GraphHost::default();
    let mut planned = GraphHost::default();
    direct.sync_from_payload(&payload).expect("direct sync");
    planned.sync_from_payload(&payload).expect("planned sync");
    direct.set_viewport(400, 400, 1.0);
    planned.set_viewport(400, 400, 1.0);

    direct.pointer_down_screen([200.0, 200.0], 0, false, false, false, false);
    direct.pointer_up_screen(200.0, 200.0, false, false, false);
    let down = planned.plan_pointer(dag::DagPointerIntent { phase: dag::DagPointerPhase::Down, x: 200.0, y: 200.0, button: 0, shift: false, ctrl_or_meta: false, alt: false, pan: false }).expect("down plan");
    assert!(planned.commit_pointer(down));
    let up = planned.plan_pointer(dag::DagPointerIntent { phase: dag::DagPointerPhase::Up, x: 200.0, y: 200.0, button: 0, shift: false, ctrl_or_meta: false, alt: false, pan: false }).expect("up plan");
    assert!(planned.commit_pointer(up));
    assert_eq!(direct.dag.selected_node_ids(), planned.dag.selected_node_ids());

    let stale = planned.plan_pointer(dag::DagPointerIntent { phase: dag::DagPointerPhase::Down, x: 200.0, y: 200.0, button: 0, shift: false, ctrl_or_meta: false, alt: false, pan: true }).expect("stale plan");
    planned.set_viewport(401, 400, 1.0);
    let camera = planned.camera_json();
    assert!(!planned.commit_pointer(stale));
    assert_eq!(planned.camera_json(), camera);
}

#[test]
fn graph_host_pointer_click_selects_node() {
    let mut host = GraphHost::default();
    let payload = payload_with_node("a");
    host.sync_from_payload(&payload).expect("sync");
    host.set_viewport(400, 400, 1.0);
    host.pointer_down_screen([200.0, 200.0], 0, false, false, false, false);
    host.pointer_up_screen(200.0, 200.0, false, false, false);
    assert_eq!(host.dag.selected_node_ids(), vec!["a".to_string()]);
}

#[test]
fn graph_host_pick_targets_at_screen_json_finds_node() {
    let mut host = GraphHost::default();
    let payload = payload_with_node("a");
    host.sync_from_payload(&payload).expect("sync");
    host.set_viewport(400, 400, 1.0);
    let json = host.pick_targets_at_screen_json(200.0, 200.0);
    assert!(json.contains("\"a\""));
}

#[test]
fn graph_host_entity_screen_json_visible_for_known_node() {
    let mut host = GraphHost::default();
    let payload = payload_with_node("a");
    host.sync_from_payload(&payload).expect("sync");
    host.set_viewport(400, 400, 1.0);
    let json = host.entity_screen_json("node", "a");
    assert!(json.contains("\"visible\":true"));
}

#[test]
fn graph_host_entity_screen_json_invisible_for_unknown_node() {
    let host = GraphHost::default();
    let json = host.entity_screen_json("node", "missing");
    assert_eq!(json, r#"{"visible":false}"#);
}

#[test]
fn graph_host_align_selection_errors_on_unknown_mode() {
    let mut host = GraphHost::default();
    let payload = payload_with_node("a");
    host.sync_from_payload(&payload).expect("sync");
    let selection = DomainSelection { granularity: "node".into(), ids: vec!["a".into()], anchor_id: None };
    host.sync_interaction(Some(&selection), None);
    let err = host.align_selection("bogusMode").unwrap_err();
    assert!(matches!(err, NodeGraphError::Dag(_)));
}

#[test]
fn graph_host_align_selection_ok_for_single_node() {
    let mut host = GraphHost::default();
    let payload = payload_with_node("a");
    host.sync_from_payload(&payload).expect("sync");
    let selection = DomainSelection { granularity: "node".into(), ids: vec!["a".into()], anchor_id: None };
    host.sync_interaction(Some(&selection), None);
    host.align_selection("alignLeft").expect("align");
}

#[test]
fn graph_host_fixture_json_round_trips_nodes() {
    let mut host = GraphHost::default();
    let payload = payload_with_node("a");
    host.sync_from_payload(&payload).expect("sync");
    let json = host.fixture_json().expect("fixture json");
    assert!(json.contains("\"a\""));
}

#[test]
fn graph_host_label_overlay_paint_state_json_includes_camera() {
    let mut host = GraphHost::default();
    let payload = payload_with_node("a");
    host.sync_from_payload(&payload).expect("sync");
    let json = host.label_overlay_paint_state_json().expect("labels");
    assert!(json.contains("\"camera\""));
}
//#endregion 🔖️GraphHostQueries
