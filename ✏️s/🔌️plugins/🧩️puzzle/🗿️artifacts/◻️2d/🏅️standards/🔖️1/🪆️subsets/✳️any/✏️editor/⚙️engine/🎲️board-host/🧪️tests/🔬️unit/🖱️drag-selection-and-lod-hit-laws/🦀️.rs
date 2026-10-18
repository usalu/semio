/// 🖱️ Drag, selection and lod hit laws: the node drag, the compact and bounded drags, the detail lod handle hit and the multi select and edge selection laws.

#[test]
fn board_host_drag_emits_node_move() {
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
    let _ = h.outbox.drain_events_json();
    let w = Point::new(0.0, 0.0);
    let s = h.scene.world_to_screen(w);
    h.pointer_down_screen(s.x, s.y, 0, false, false);
    h.pointer_move_screen(s.x + 50.0, s.y + 30.0, false, false, false);
    h.pointer_up_screen(s.x + 50.0, s.y + 30.0, false, false, false);
    let ev = h.outbox.drain_events_json();
    assert!(ev.contains("nodeMove"));
}

#[test]
fn board_host_compact_discrete_hit_selects_and_drags_node() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    board_set_camera(&mut h.scene, &mut h.outbox, 0.0, 0.0, 0.5);
    let mut desc = sample_scene();
    desc.handles.clear();
    desc.edges.clear();
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
    assert_eq!(resolve_hit_world(&h.scene, &h.focus, &h.style, Point::new(0.0, 0.0)).as_deref(), Some("a"));
    assert!(resolve_hit_world(&h.scene, &h.focus, &h.style, Point::new(150.0, 0.0)).is_none());
    let s = h.scene.world_to_screen(Point::new(0.0, 0.0));
    h.pointer_down_screen(s.x, s.y, 0, false, false);
    h.pointer_move_screen(s.x + 50.0, s.y + 30.0, false, false, false);
    h.pointer_up_screen(s.x + 50.0, s.y + 30.0, false, false, false);
    let ev = h.outbox.drain_events_json();
    assert!(ev.contains("nodeMove"), "compact discrete node hit should drag, got: {ev}");
}

#[test]
fn board_host_minimap_bounded_drag_moves_selection_inside_union_bounds() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.style.set_automatic_lod(false);
    h.style.set_forced_draw_lod_label("minimap");
    board_set_camera(&mut h.scene, &mut h.outbox, 0.0, 0.0, 0.1);
    let mut desc = sample_scene();
    desc.handles.clear();
    desc.edges.clear();
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
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into(), "b".into()]);
    let _ = h.outbox.drain_events_json();
    let gap = Point::new(150.0, 0.0);
    assert!(resolve_hit_world(&h.scene, &h.focus, &h.style, gap).is_none());
    let s = h.scene.world_to_screen(gap);
    h.pointer_down_screen(s.x, s.y, 0, false, false);
    h.pointer_move_screen(s.x + 50.0, s.y + 30.0, false, false, false);
    h.pointer_up_screen(s.x + 50.0, s.y + 30.0, false, false, false);
    let ev = h.outbox.drain_events_json();
    assert!(ev.contains("nodeMove"), "expected bounded drag nodeMove, got: {ev}");
    let zoom = 0.1;
    let dx = 50.0 / zoom;
    let dy = 30.0 / zoom;
    let a = h.scene.nodes.get("a").unwrap();
    let b = h.scene.nodes.get("b").unwrap();
    assert!((a.x - dx).abs() < 1e-3 && (a.y - dy).abs() < 1e-3);
    assert!((b.x - (300.0 + dx)).abs() < 1e-3 && (b.y - dy).abs() < 1e-3);
}

#[test]
fn board_host_overview_bounded_drag_moves_selection_inside_union_bounds() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.style.set_automatic_lod(false);
    h.style.set_forced_draw_lod_label("overview");
    set_overview_lod(&mut h);
    let mut desc = sample_scene();
    desc.handles.clear();
    desc.edges.clear();
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
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into(), "b".into()]);
    let _ = h.outbox.drain_events_json();
    let gap = Point::new(150.0, 0.0);
    assert!(resolve_hit_world(&h.scene, &h.focus, &h.style, gap).is_none());
    let s = h.scene.world_to_screen(gap);
    h.pointer_down_screen(s.x, s.y, 0, false, false);
    h.pointer_move_screen(s.x + 40.0, s.y + 20.0, false, false, false);
    h.pointer_up_screen(s.x + 40.0, s.y + 20.0, false, false, false);
    let ev = h.outbox.drain_events_json();
    assert!(ev.contains("nodeMove"), "expected overview bounded drag, got: {ev}");
}

#[test]
fn board_host_detail_lod_resolves_direct_handle_hit() {
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
    let probe = Point::new(hp.x + 2.0, hp.y);
    assert_eq!(resolve_hit_world(&h.scene, &h.focus, &h.style, probe).as_deref(), Some("a:h0"));
}

#[test]
fn board_host_multi_select_drag_moves_each_selected_node() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    let mut desc = sample_scene();
    desc.nodes.push(NodeDescriptor {
        id: "b".into(),
        x: 100.0,
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
    h.focus.set_selection_options("rectangle", "additive", true, true, true);
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into(), "b".into()]);
    let _ = h.outbox.drain_events_json();
    let w = Point::new(0.0, 0.0);
    let s = h.scene.world_to_screen(w);
    h.pointer_down_screen(s.x, s.y, 0, false, false);
    h.pointer_move_screen(s.x + 10.0, s.y + 5.0, false, false, false);
    h.pointer_up_screen(s.x + 10.0, s.y + 5.0, false, false, false);
    let a = h.scene.nodes.get("a").expect("node a");
    let b = h.scene.nodes.get("b").expect("node b");
    assert!((a.x - 10.0).abs() < 1e-6);
    assert!((a.y - 5.0).abs() < 1e-6);
    assert!((b.x - 110.0).abs() < 1e-6);
    assert!((b.y - 5.0).abs() < 1e-6);
    let sorted: Vec<_> = h.focus.selection.iter().cloned().collect();
    assert_eq!(sorted, vec!["a".to_string(), "b".to_string()]);
}

#[test]
fn board_host_selection_target_edges_skips_node_geometry() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.focus.set_selection_options("rectangle", "invertive", false, true, false);
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
    let inside_node_a = Point::new(0.0, 0.0);
    assert!(resolve_hit_world(&h.scene, &h.focus, &h.style, inside_node_a).is_none());
    let on_edge = Point::new(150.0, 0.0);
    assert_eq!(resolve_hit_world(&h.scene, &h.focus, &h.style, on_edge).as_deref(), Some("e1"));
}

#[test]
fn board_host_additive_click_merges_edge_into_existing_selection() {
    let mut h = BoardHost::new();
    h.scene.set_size(800, 600, 1.0);
    h.focus.set_selection_options("rectangle", "additive", true, true, true);
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
    set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into()]);
    let _ = h.outbox.drain_events_json();
    let on_edge = Point::new(150.0, 0.0);
    let s = h.scene.world_to_screen(on_edge);
    h.pointer_down_screen(s.x, s.y, 0, false, false);
    let mut got: Vec<_> = h.focus.selection.iter().cloned().collect();
    got.sort();
    assert_eq!(got, vec!["a".to_string(), "e1".to_string()]);
}
