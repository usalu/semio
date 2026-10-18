/// 📐️ Generation, area select and minimap laws: the scene generation stability, the background click deselect, the area select, the minimap hover and preselect and the board naming vocabulary.

#[test]
fn board_host_selection_change_does_not_bump_content_scene_generation() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.sync_descriptor(&sample_scene()).unwrap();
    let gen = h.scene.test_content_scene_generation();
    let neutral_hint = h.encoded_scene_hint();
    set_selection_ids_silent(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into()]);
    assert_eq!(h.scene.test_content_scene_generation(), gen);
    let selected_hint = h.encoded_scene_hint();
    assert_ne!(selected_hint, neutral_hint);
    set_selection_ids_silent(&mut h.scene, &mut h.focus, &mut h.outbox, &[]);
    assert_eq!(h.scene.test_content_scene_generation(), gen);
    assert_eq!(h.encoded_scene_hint(), neutral_hint);
}

#[test]
fn board_host_hover_change_does_not_bump_content_scene_generation() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.sync_descriptor(&sample_scene()).unwrap();
    let gen = h.scene.test_content_scene_generation();
    let neutral_hint = h.encoded_scene_hint();
    h.focus.set_hovered_id_silent(Some("a".into()));
    assert_eq!(h.scene.test_content_scene_generation(), gen, "hover must paint via dynamic overlay chrome without rebuilding cached icons");
    let hovered_hint = h.encoded_scene_hint();
    assert_ne!(hovered_hint, neutral_hint);
    h.focus.set_hovered_id_silent(None);
    assert_eq!(h.scene.test_content_scene_generation(), gen);
    assert_eq!(h.encoded_scene_hint(), neutral_hint);
}

#[test]
fn board_host_background_click_deselect_skips_preselect_events() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    let desc = sample_scene();
    h.sync_descriptor(&desc).unwrap();
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into(), "e1".into()]);
    let _ = h.outbox.drain_events_json();
    let away = Point::new(5000.0, 5000.0);
    let s = h.scene.world_to_screen(away);
    h.pointer_down_screen(s.x, s.y, 0, false, false);
    assert!(!h.focus.is_dragging_area_select());
    h.pointer_move_screen(s.x + 1.0, s.y, false, false, false);
    let mid = h.outbox.drain_events_json();
    assert!(!mid.contains("preselect"), "background click path must not emit preselect");
    assert!(h.focus.preselect_removed.is_empty());
    assert!(h.focus.selection_exit_highlight.is_empty());
    assert!(h.focus.selection.contains("a"));
    h.pointer_up_screen(s.x, s.y, false, false, false);
    assert!(h.focus.selection.is_empty());
    assert!(h.focus.selection_exit_highlight.is_empty());
    let fin = h.outbox.drain_events_json();
    assert!(fin.contains("select"));
    assert!(!fin.contains("preselect"));
    assert!(fin.contains("\"exitHighlightIds\":[]"));
}

#[test]
fn board_host_background_click_without_drag_clears_selection() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
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
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into(), "e1".into()]);
    let away = Point::new(5000.0, 5000.0);
    let s = h.scene.world_to_screen(away);
    h.pointer_down_screen(s.x, s.y, 0, false, false);
    h.pointer_up_screen(s.x, s.y, false, false, false);
    assert!(h.focus.selection.is_empty());
}

#[test]
fn board_host_rectangle_area_select_includes_handles_with_nodes() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.focus.set_selection_options("rectangle", "invertive", true, true, true);
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
    let _ = h.outbox.drain_events_json();
    let w0 = Point::new(-90.0, -70.0);
    let w1 = Point::new(90.0, 90.0);
    let s0 = h.scene.world_to_screen(w0);
    let s1 = h.scene.world_to_screen(w1);
    h.pointer_down_screen(s0.x, s0.y, 0, false, false);
    h.pointer_move_screen(s1.x, s1.y, false, false, false);
    h.pointer_up_screen(s1.x, s1.y, false, false, false);
    let mut got: Vec<_> = h.focus.selection.iter().cloned().collect();
    got.sort();
    assert!(got.contains(&"a".to_string()));
    assert!(got.contains(&"a:h0".to_string()));
}

#[test]
fn board_host_area_select_preselect_matches_selected_chrome() {
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
    let _ = h.outbox.drain_events_json();
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into()]);
    let _ = h.outbox.drain_events_json();
    assert!(h.focus.preselect_removed.is_empty());
    assert!(h.focus.selection_exit_highlight.is_empty());
    let w_down = Point::new(350.0, -50.0);
    let w_mid = Point::new(270.0, 50.0);
    let w_end = Point::new(265.0, 48.0);
    let s_down = h.scene.world_to_screen(w_down);
    h.pointer_down_screen(s_down.x, s_down.y, 0, false, false);
    assert!(!h.focus.is_dragging_area_select());
    let _ = h.outbox.drain_events_json();
    let s_mid = h.scene.world_to_screen(w_mid);
    let s_end = h.scene.world_to_screen(w_end);
    h.pointer_move_screen(s_mid.x, s_mid.y, false, false, false);
    assert!(h.focus.is_dragging_area_select());
    let _ = h.outbox.drain_events_json();
    assert!(h.focus.preselect.contains("b"), "preview should include node b");
    assert!(h.focus.preselect_removed.contains("a"));
    assert!(h.focus.selection_exit_highlight.is_empty());
    assert!(!h.focus.selection.contains("b"), "committed selection unchanged during preselect");
    let frozen = h.focus.preselect_removed.clone();
    h.pointer_move_screen(s_end.x, s_end.y, false, false, false);
    let _ = h.outbox.drain_events_json();
    assert_eq!(frozen, h.focus.preselect_removed);
    h.pointer_up_screen(s_end.x, s_end.y, false, false, false);
    let _ = h.outbox.drain_events_json();
    assert!(h.focus.selection.contains("b"));
    assert!(!h.focus.selection.contains("a"));
    assert!(h.focus.preselect_removed.is_empty());
    assert!(h.focus.selection_exit_highlight.is_empty());
}

#[test]
fn board_host_area_select_from_empty_keeps_selection_until_commit() {
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
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &[]);
    let _ = h.outbox.drain_events_json();
    let w_down = Point::new(350.0, -50.0);
    let w_mid = Point::new(270.0, 50.0);
    let s_down = h.scene.world_to_screen(w_down);
    let s_mid = h.scene.world_to_screen(w_mid);
    h.pointer_down_screen(s_down.x, s_down.y, 0, false, false);
    h.pointer_move_screen(s_mid.x, s_mid.y, false, false, false);
    let _ = h.outbox.drain_events_json();
    assert!(h.focus.is_dragging_area_select());
    assert!(h.focus.preselect.contains("b"));
    assert!(h.focus.preselect_removed.is_empty());
    assert!(h.focus.selection.is_empty());
    h.pointer_up_screen(s_mid.x, s_mid.y, false, false, false);
    let _ = h.outbox.drain_events_json();
    assert!(h.focus.selection.contains("b"));
    assert!(h.focus.preselect.is_empty());
}

#[test]
fn board_host_minimap_pointer_move_hovers_node_under_cursor() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    board_set_camera(&mut h.scene, &mut h.outbox, 0.0, 0.0, 0.1);
    h.style.set_forced_draw_lod_label("minimap");
    h.sync_descriptor(&sample_scene()).unwrap();
    let center = h.scene.world_to_screen(Point::new(0.0, 0.0));
    h.pointer_move_screen(center.x, center.y, false, false, false);
    assert_eq!(h.focus.hovered_id.as_deref(), Some("a"));
    let away = h.scene.world_to_screen(Point::new(5000.0, 5000.0));
    h.pointer_move_screen(away.x, away.y, false, false, false);
    assert!(h.focus.hovered_id.is_none());
}

#[test]
fn board_host_minimap_preselect_matches_selected_chrome() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    board_set_camera(&mut h.scene, &mut h.outbox, 0.0, 0.0, 0.1);
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
    let neutral_hint = h.encoded_scene_hint();
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["b".into()]);
    let _ = h.outbox.drain_events_json();
    let selected_hint = h.encoded_scene_hint();
    assert!(selected_hint > neutral_hint, "minimap selected chrome should add visible vector encoding over neutral state");
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into()]);
    let _ = h.outbox.drain_events_json();
    let w_down = Point::new(350.0, -50.0);
    let w_end = Point::new(265.0, 48.0);
    let s_down = h.scene.world_to_screen(w_down);
    let s_end = h.scene.world_to_screen(w_end);
    set_transform_flags(&mut h.scene, &mut h.gestures, true, false);
    h.pointer_down_screen(s_down.x, s_down.y, 0, false, false);
    h.pointer_move_screen(s_end.x, s_end.y, false, false, false);
    assert!(h.focus.is_dragging_area_select(), "with the rotate ring off, a press beside the selection opens the marquee");
    assert!(h.focus.preselect.contains("b"));
    h.focus.set_selection_screen_preview(None);
    let preselect_hint = h.encoded_scene_hint();
    assert!(preselect_hint > neutral_hint, "minimap preselect should add visible selected chrome over neutral minimap rendering");
}

#[test]
fn board_host_silent_preselect_applies_selected_chrome_without_area_drag() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    board_set_camera(&mut h.scene, &mut h.outbox, 0.0, 0.0, 0.1);
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
    let neutral_hint = h.encoded_scene_hint();
    assert!(!matches!(h.focus.interaction, Interaction::Selection { .. }));
    set_preselect_state_silent(&mut h.scene, &mut h.focus, &["b".into()], &[]);
    assert!(h.scene.nodes.get("b").is_some_and(|n| n.selected));
    assert!(h.scene.nodes.get("a").is_some_and(|n| !n.selected));
    let preselect_hint = h.encoded_scene_hint();
    assert!(preselect_hint > neutral_hint, "silent minimap preselect should paint selected chrome without an active area-select interaction");
}

#[test]
fn board_host_hover_tracks_visible_wires() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    set_detail_lod(&mut h);
    let mut desc = sample_scene();
    desc.edges.clear();
    desc.wires.push(WireDescriptor { id: "w1".into(), source: "a:h0".into(), target: None, end_x: Some(220.0), end_y: Some(0.0), selected: None, style: None, wire_kind: None, user_data: None, visible: None, locked: None });
    h.sync_descriptor(&desc).unwrap();
    let source = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
    let curve = compute_edge_bezier_points(source, Point::new(220.0, 0.0), Point::new(0.0, 0.0), Point::new(220.0, 0.0));
    let probe = cubic_bezier_point(curve, 0.5);
    h.update_hover_from_world(probe);
    assert_eq!(h.focus.hovered_id.as_deref(), Some("w1"));
}

/// 🎫️ The puzzle board naming is the plugin's wire vocabulary, byte for byte: every fill stage keeps its
/// `puzzle2d-fill-*` progress label, placements keep their `puzzle2d.brush.` / `puzzle2d.fill.` prefixes, and both
/// board constructors install it.
#[test]
fn puzzle_board_naming_declares_the_plugin_wire_vocabulary() {
    use crate::editor::puzzle2d::engine::board_host::{puzzle_board_host, puzzle_board_host_normal, PUZZLE_2D_BOARD_NAMING};
    use crate::editor::puzzle2d::engine::BoardFillStage;
    let labels = [
        (BoardFillStage::ResetSources, "puzzle2d-fill-reset-sources"),
        (BoardFillStage::PrepareSources, "puzzle2d-fill-prepare-sources"),
        (BoardFillStage::SelectTarget, "puzzle2d-fill-select-target"),
        (BoardFillStage::ResetCandidates, "puzzle2d-fill-reset-candidates"),
        (BoardFillStage::PrepareCandidates, "puzzle2d-fill-prepare-candidates"),
        (BoardFillStage::ScanCompatibility, "puzzle2d-fill-scan-compatibility"),
        (BoardFillStage::SelectCandidate, "puzzle2d-fill-select-candidate"),
        (BoardFillStage::ConstructPreview, "puzzle2d-fill-construct-preview"),
        (BoardFillStage::ScanHostCollision, "puzzle2d-fill-scan-host-collision"),
        (BoardFillStage::ScanVirtualCollision, "puzzle2d-fill-scan-virtual-collision"),
        (BoardFillStage::AcceptCandidate, "puzzle2d-fill-accept-candidate"),
        (BoardFillStage::AcceptNodeId, "puzzle2d-fill-accept-node-id"),
        (BoardFillStage::AcceptEdgeId, "puzzle2d-fill-accept-edge-id"),
        (BoardFillStage::AcceptEdgeKind, "puzzle2d-fill-accept-edge-kind"),
        (BoardFillStage::AcceptNodeKind, "puzzle2d-fill-accept-node-kind"),
        (BoardFillStage::AcceptSourceHandle, "puzzle2d-fill-accept-source-handle"),
        (BoardFillStage::AcceptTargetHandle, "puzzle2d-fill-accept-target-handle"),
        (BoardFillStage::AcceptIcon, "puzzle2d-fill-accept-icon"),
        (BoardFillStage::AcceptVirtualNode, "puzzle2d-fill-accept-virtual-node"),
        (BoardFillStage::AcceptSourceConnection, "puzzle2d-fill-accept-source-connection"),
        (BoardFillStage::AcceptHandles, "puzzle2d-fill-accept-handles"),
        (BoardFillStage::AcceptHandleId, "puzzle2d-fill-accept-handle-id"),
        (BoardFillStage::AcceptHandleVirtual, "puzzle2d-fill-accept-handle-virtual"),
        (BoardFillStage::AcceptHandlePublish, "puzzle2d-fill-accept-handle-publish"),
        (BoardFillStage::PublishPlanPrefix, "puzzle2d-fill-publish-prefix"),
        (BoardFillStage::Complete, "puzzle2d-fill-complete"),
    ];
    for (stage, label) in labels {
        assert_eq!((PUZZLE_2D_BOARD_NAMING.fill_stage_label)(stage), label);
    }
    assert_eq!(
        (PUZZLE_2D_BOARD_NAMING.brush_node_prefix, PUZZLE_2D_BOARD_NAMING.brush_edge_prefix, PUZZLE_2D_BOARD_NAMING.fill_node_prefix, PUZZLE_2D_BOARD_NAMING.fill_edge_prefix, PUZZLE_2D_BOARD_NAMING.fill_stale_operation_fault),
        ("puzzle2d.brush.", "puzzle2d.brush.edge.", "puzzle2d.fill.", "puzzle2d.fill.edge.", "stale-puzzle2d-fill-operation")
    );
    assert_eq!(PUZZLE_2D_BOARD_NAMING.placement_naming().to_json(), r#"{"brushNodePrefix":"puzzle2d.brush.","brushEdgePrefix":"puzzle2d.brush.edge."}"#, "the scene wire form other hosts mint with");
    assert!(std::ptr::eq(puzzle_board_host().style.naming(), &PUZZLE_2D_BOARD_NAMING));
    assert!(std::ptr::eq(puzzle_board_host_normal().style.naming(), &PUZZLE_2D_BOARD_NAMING));
}
