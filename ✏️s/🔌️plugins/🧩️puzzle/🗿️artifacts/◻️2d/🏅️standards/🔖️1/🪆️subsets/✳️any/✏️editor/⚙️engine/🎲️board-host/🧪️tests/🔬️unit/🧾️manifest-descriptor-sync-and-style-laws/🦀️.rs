/// 🧾️ Manifest, descriptor sync and style laws: the default manifest validation, the descriptor sync deferral while panning and dragging, the cached content and the element style colors.

/// 🔗️ Keeps the runtime kind-catalog JSON shape in sync with the compile-time `puzzle2d-default` manifest.
#[test]
fn puzzle2d_default_manifest_satisfies_board_host_validation() {
    let manifest: semio_framework_pack_json::Value = semio_framework_pack_json::from_json_str(fixture_text!("../../../../../../../../../🛂️manifest.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let handle_kinds: Vec<semio_framework_pack_json::Value> =
        manifest["portKinds"].as_array().unwrap().iter().map(|row| json!({ "id": row["id"], "name": row["name"], "color": row["presentation"]["color"], "defaultWireKind": row["presentation"]["defaultWireKind"] })).collect();
    let wire_kinds: Vec<semio_framework_pack_json::Value> = manifest["wireKinds"].as_array().unwrap().iter().map(|row| json!({ "id": row["id"], "name": row["name"], "defaultEdgeKind": row["presentation"]["defaultEdgeKind"] })).collect();
    let edge_kinds: Vec<semio_framework_pack_json::Value> = manifest["edgeKinds"].as_array().unwrap().iter().map(|row| json!({ "id": row["id"], "name": row["name"] })).collect();
    let catalogs_json = json!({ "handleKinds": handle_kinds, "wireKinds": wire_kinds, "edgeKinds": edge_kinds }).to_string();

    let mut host = BoardHost::new();
    host.style.set_board_kind_catalogs_from_json(&catalogs_json).expect("catalog json derived from the manifest must be valid");
    host.style.validate_against_manifest(&crate::standards::v1::subsets::any::schema::graph_manifest::puzzle2d_default::puzzle2d_default_manifest()).expect("runtime catalog must satisfy the compile-time puzzle2d-default manifest");
}

#[test]
fn board_host_defers_descriptor_sync_while_panning() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.sync_descriptor(&sample_scene()).unwrap();
    let _ = h.outbox.drain_events_json();
    h.pointer_down_screen(10.0, 10.0, 1, false, false);
    assert!(defers_descriptor_sync_from_js(&h.focus, &h.gestures));
    h.pointer_move_screen(80.0, 60.0, false, false, false);
    assert!(defers_descriptor_sync_from_js(&h.focus, &h.gestures));
    let _ = h.outbox.drain_events_json();
    h.pointer_up_screen(80.0, 60.0, false, false, false);
    assert!(!defers_descriptor_sync_from_js(&h.focus, &h.gestures));
}

#[test]
fn board_host_defers_descriptor_sync_while_dragging_nodes() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.sync_descriptor(&sample_scene()).unwrap();
    let _ = h.outbox.drain_events_json();
    let start = h.scene.world_to_screen(Point::new(0.0, 0.0));
    h.pointer_down_screen(start.x, start.y, 0, false, false);
    assert!(matches!(h.focus.interaction, Interaction::DragNodes { .. }));
    h.pointer_move_screen(start.x + 40.0, start.y, false, false, false);
    assert!(defers_descriptor_sync_from_js(&h.focus, &h.gestures));
    let ev = h.outbox.drain_events_json();
    assert!(ev.contains("nodeMove"));
    h.pointer_up_screen(start.x + 40.0, start.y, false, false, false);
    assert!(!defers_descriptor_sync_from_js(&h.focus, &h.gestures));
    let end: Vec<semio_framework_pack_json::Value> = semio_framework_pack_json::from_json_str(&h.outbox.drain_events_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("release rows");
    let record = end.iter().find(|row| row["name"] == "gesture").expect("the release publishes one gesture record");
    let reached = h.scene.screen_to_world(Point::new(start.x + 40.0, start.y));
    let (dx, dy) = (record["payload"]["dx"].as_f64().expect("dx"), record["payload"]["dy"].as_f64().expect("dy"));
    assert_eq!(record["payload"]["kind"], "drag");
    assert!((dx - reached.x).abs() < 1e-6 && (dy - reached.y).abs() < 1e-6, "the record carries the grab-to-release offset: ({dx}, {dy}) vs {reached:?}");
}

#[test]
fn board_host_set_node_positions_updates_existing_nodes_only() {
    let mut h = BoardHost::new();
    h.scene.set_size(400, 300, 1.0);
    h.sync_descriptor(&sample_scene()).unwrap();
    let gen_before = h.scene.test_content_scene_generation();
    h.scene.set_node_positions(&[("a".into(), 12.0, 34.0), ("missing".into(), 1.0, 2.0), ("a".into(), f64::NAN, 0.0)]);
    let node = h.scene.nodes.get("a").expect("node a should remain");
    assert!((node.x - 12.0).abs() < 0.001);
    assert!((node.y - 34.0).abs() < 0.001);
    assert!(h.scene.test_content_scene_generation() > gen_before, "moving nodes must invalidate cached world content");
    h.scene.set_node_positions_json(r#"[{"id":"a","x":90.0,"y":110.0}]"#).unwrap();
    let node = h.scene.nodes.get("a").expect("node a should remain");
    assert!((node.x - 90.0).abs() < 0.001);
    assert!((node.y - 110.0).abs() < 0.001);
}

#[test]
fn board_host_overlay_paint_state_json_matches_host_camera_lod_and_node_centers() {
    let mut h = BoardHost::new();
    h.scene.set_size(640, 480, 2.0);
    h.sync_descriptor(&sample_scene()).unwrap();
    set_camera_silent(&mut h.scene, &mut h.outbox, 12.0, -8.0, 0.2);
    if let Some(n) = h.scene.nodes.get_mut("a") {
        n.x = 33.0;
        n.y = 44.0;
    }
    let raw: semio_framework_pack_json::Value = semio_framework_pack_json::from_json_str(&overlay_paint_state_json(&h.scene, &h.style), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("overlay paint state json");
    assert!((raw["camera"]["x"].as_f64().unwrap() - 12.0).abs() < 1e-9);
    assert!((raw["camera"]["y"].as_f64().unwrap() - (-8.0)).abs() < 1e-9);
    assert!((raw["camera"]["zoom"].as_f64().unwrap() - 0.2).abs() < 1e-9);
    assert_eq!(raw["lod"].as_str(), Some("overview"));
    let nodes = raw["nodes"].as_array().expect("nodes array");
    let a = nodes.iter().find(|row| row["id"].as_str() == Some("a")).expect("node a row");
    assert!((a["x"].as_f64().unwrap() - 33.0).abs() < 1e-9);
    assert!((a["y"].as_f64().unwrap() - 44.0).abs() < 1e-9);
}

#[test]
fn board_host_node_drag_invalidates_cached_world_content() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.sync_descriptor(&sample_scene()).unwrap();
    let gen_before = h.scene.test_content_scene_generation();
    let s = h.scene.world_to_screen(Point::new(0.0, 0.0));
    h.pointer_down_screen(s.x, s.y, 0, false, false);
    h.pointer_move_screen(s.x + 80.0, s.y + 40.0, false, false, false);
    assert!(h.scene.test_content_scene_generation() > gen_before, "node drag must rebuild cached nodes/handles, not only edges");
    let node = h.scene.nodes.get("a").expect("dragged node");
    assert!(node.x.abs() > 1.0 || node.y.abs() > 1.0, "pointer move should translate node a away from origin");
}

#[test]
fn board_host_manual_lod_follow_zoom_still_encodes_graph() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.sync_descriptor(&sample_scene()).unwrap();
    let with_automatic = h.encoded_scene_hint();
    assert!(with_automatic > 0, "sample scene should encode vector paths");
    h.style.set_automatic_lod(false);
    h.style.set_forced_draw_lod_label("");
    let manual_follow_zoom = h.encoded_scene_hint();
    assert!(manual_follow_zoom > 0, "manual follow-zoom LOD must still draw nodes/edges (hint={manual_follow_zoom})");
    h.style.set_forced_draw_lod_label("overview");
    let pinned_overview = h.encoded_scene_hint();
    assert!(pinned_overview > 0, "pinned overview LOD must still draw graph");
    h.style.set_automatic_lod(true);
    let automatic_restored = h.encoded_scene_hint();
    assert_eq!(with_automatic, automatic_restored);
}

#[test]
fn board_host_pick_selection_never_sets_exit_highlight() {
    let mut h = BoardHost::new();
    h.scene.set_size(400, 300, 1.0);
    let mut d = sample_scene();
    d.selection_exit_highlight_ids = vec!["a".into(), "ghost".into()];
    h.sync_descriptor(&d).unwrap();
    let _ = h.outbox.drain_events_json();
    assert!(h.focus.selection_exit_highlight.is_empty());
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into(), "e1".into()]);
    let ev = h.outbox.drain_events_json();
    assert!(h.focus.selection_exit_highlight.is_empty());
    assert!(ev.contains("\"exitHighlightIds\":[]"));
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["e1".into()]);
    let ev2 = h.outbox.drain_events_json();
    assert!(h.focus.selection_exit_highlight.is_empty());
    assert!(ev2.contains("\"exitHighlightIds\":[]"));
}

#[test]
fn board_host_canvas_theme_keeps_explicit_element_state_colors() {
    let mut h = BoardHost::new();
    h.style.set_canvas_palette(&BoardPaletteOverlay {
        node_stroke_hovered:Some([1,2,3,255]),edge_stroke_hovered:Some([4,5,6,255]),
        handle_stroke_hovered:Some([7,8,9,255]),wire_stroke_hovered:Some([10,11,12,255]),
        ..BoardPaletteOverlay::default()
    });
    assert_eq!(h.style.canvas_theme.node_stroke_hovered.to_rgba8(), canvas::Color::from_rgba8(1, 2, 3, 255).to_rgba8());
    assert_eq!(h.style.canvas_theme.edge_stroke_hovered.to_rgba8(), canvas::Color::from_rgba8(4, 5, 6, 255).to_rgba8());
    assert_eq!(h.style.canvas_theme.handle_stroke_hovered.to_rgba8(), canvas::Color::from_rgba8(7, 8, 9, 255).to_rgba8());
    assert_eq!(h.style.canvas_theme.wire_stroke_hovered.to_rgba8(), canvas::Color::from_rgba8(10, 11, 12, 255).to_rgba8());
}

#[test]
fn board_host_cancel_area_select_restores_initial_selection() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    set_detail_lod(&mut h);
    h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
    let _ = h.outbox.drain_events_json();
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into(), "b".into()]);
    let _ = h.outbox.drain_events_json();
    h.pointer_down_screen(5.0, 5.0, 0, false, false);
    assert!(!h.focus.is_dragging_area_select());
    h.pointer_move_screen(20.0, 5.0, false, false, false);
    assert!(h.focus.is_dragging_area_select());
    let _ = h.outbox.drain_events_json();
    assert!(h.cancel_area_select());
    assert!(!h.focus.is_dragging_area_select());
    let ev = h.outbox.drain_events_json();
    assert!(ev.contains("preselectCancel"));
    assert!(!ev.contains("\"select\""));
    assert_eq!(h.focus.selection.len(), 2);
    assert!(h.focus.selection.contains("a") && h.focus.selection.contains("b"));
    assert!(h.focus.preselect.is_empty());
}

#[test]
fn board_host_syncs_descriptor_and_hit_tests_handle_before_node() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    set_detail_lod(&mut h);
    let mut desc = sample_scene();
    desc.nodes.push(NodeDescriptor {
        id: "b".into(),
        x: 300.0,
        y: 0.0,
        draggable: Some(true),
        selected: None,
        style: None,
        text: None,
        icon_kind: None,
        node_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        root: None,
        shape: Some("circle".into()),
        radius: Some(40.0),
        width: None,
        height: None,
        scale: None,
    });
    h.sync_descriptor(&desc).unwrap();
    let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
    let hit = resolve_hit_world(&h.scene, &h.focus, &h.style, hp);
    assert_eq!(hit.as_deref(), Some("a:h0"));
    assert!(h.encoded_scene_hint() > 10);
}

#[test]
fn board_host_cached_content_includes_edge_vector_paths_at_overview_zoom() {
    let mut h = BoardHost::new();
    h.scene.set_size(1200, 800, 1.0);
    h.sync_descriptor(&link_test_scene_a_to_b_linked()).unwrap();
    set_camera_silent(&mut h.scene, &mut h.outbox, 0.0, 0.0, 0.21);
    let with_edges = h.encoded_scene_hint();
    let without = link_test_scene_no_edge();
    h.sync_descriptor(&without).unwrap();
    let without_edges = h.encoded_scene_hint();
    assert!(with_edges > without_edges, "overview cached draw must encode edges (with={with_edges}, without={without_edges})");
}

#[test]
fn board_host_world_clip_changes_vector_encoding() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    let mut desc = sample_scene();
    desc.nodes.push(NodeDescriptor {
        id: "b".into(),
        x: 600.0,
        y: 400.0,
        draggable: Some(true),
        selected: None,
        style: None,
        text: None,
        icon_kind: None,
        node_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        root: None,
        shape: Some("circle".into()),
        radius: Some(40.0),
        width: None,
        height: None,
        scale: None,
    });
    h.sync_descriptor(&desc).unwrap();
    h.style.set_world_raster_tiling("none");
    let monolithic = h.encoded_scene_hint();
    h.style.set_world_raster_tiling("world-clip");
    let tiled = h.encoded_scene_hint();
    assert!(tiled >= monolithic);
}

#[test]
fn board_host_silent_selection_keeps_cached_world_content_warm() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.sync_descriptor(&sample_scene()).unwrap();
    let gen_before = h.scene.test_content_scene_generation();
    let neutral_hint = h.encoded_scene_hint();
    set_selection_ids_silent(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into()]);
    assert_eq!(h.scene.test_content_scene_generation(), gen_before, "selection chrome must paint via dynamic fill/stroke layers without rebuilding cached icons");
    assert_ne!(h.encoded_scene_hint(), neutral_hint, "selected node fill appears in overlay fill layer at normal LOD");
    assert_eq!(test_resolve_node_style_kind(&h.scene, &h.focus, &h.style, "a"), Some(BoardElementStyleKind::Selected));
}

#[test]
fn board_host_selected_node_keeps_selected_style_when_hovered() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    set_detail_lod(&mut h);
    h.sync_descriptor(&sample_scene()).unwrap();
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into()]);
    h.focus.set_hovered_id_silent(Some("a".into()));
    assert_eq!(test_resolve_node_style_kind(&h.scene, &h.focus, &h.style, "a"), Some(BoardElementStyleKind::Selected), "committed selection chrome should beat hover while pointer is over the node");
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &[]);
    h.focus.set_hovered_id_silent(Some("a".into()));
    assert_eq!(test_resolve_node_style_kind(&h.scene, &h.focus, &h.style, "a"), Some(BoardElementStyleKind::Hovered), "unselected nodes should still use hover chrome");
}

#[test]
fn board_host_dragging_selected_node_keeps_selected_style_at_detail_lod() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    set_detail_lod(&mut h);
    h.sync_descriptor(&sample_scene()).unwrap();
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into()]);
    let s = h.scene.world_to_screen(Point::new(0.0, 0.0));
    h.pointer_down_screen(s.x, s.y, 0, false, false);
    assert!(matches!(h.focus.interaction, Interaction::DragNodes { .. }));
    assert_eq!(h.focus.hovered_id.as_deref(), Some("a"));
    assert_eq!(test_resolve_node_style_kind(&h.scene, &h.focus, &h.style, "a"), Some(BoardElementStyleKind::Selected), "node drag should keep primary selected paint at detail LOD");
}
