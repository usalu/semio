/// 🔗️ Reconnect, proximity and LOD hit laws: edge reconnect, node drag proximity preview and connect, wired cut inputs, same-named handles, proximity disable and the silhouette and row hit picking laws.

#[test]
fn dag_host_reconnects_edge_endpoint() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    let in_w = handle_world(&host, "combine@b");
    let out_w = handle_world(&host, "scale@out");
    let (in_sx, in_sy) = world_to_screen_px(&host, in_w);
    let (out_sx, out_sy) = world_to_screen_px(&host, out_w);
    host.pointer_down(in_sx, in_sy, false);
    assert!(matches!(host.graph.engine.interaction, InteractionMode::DrawEdge { .. }));
    host.pointer_move(out_sx, out_sy);
    host.pointer_up(out_sx, out_sy);
    let e3 = host.graph.host_snapshot.edges.iter().find(|e| e.id == "e3").expect("e3");
    assert_eq!(e3.source, "scale@out");
}

#[test]
fn dag_host_node_drag_proximity_preview_and_connects() {
    let inputs = vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let src_w = computation_node_width("Src", &[], &outputs);
    let tgt_w = computation_node_width("Tgt", &inputs, &outputs);
    let src_h = computation_node_height(0, 1, false, false);
    let tgt_h = computation_node_height(1, 1, false, false);
    let mut host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![
            DagNodeSpec::computation("src".into(), "Src", "Src", "emoji:🔢️".into(), vec![], outputs.clone(), false, false, 0.0, 0.0, src_w, src_h),
            DagNodeSpec::computation("tgt".into(), "Tgt", "Tgt", "emoji:🔢️".into(), inputs, outputs, false, false, 220.0, 0.0, tgt_w, tgt_h),
        ],
        edges: vec![],
    });
    host.viewport.set_viewport(1280, 800, 1.0);
    host.graph.set_proximity_distance(120.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("normal");
    let src_center = canvas::Point::new(0.0, 0.0);
    let (sx, sy) = world_to_screen_px(&host, src_center);
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx + 200.0, sy, false, false, false);
    assert!(host.graph.engine.render_snapshot().pending_edge.is_some(), "proximity drag should preview edge");
    host.pointer_up_screen(sx + 200.0, sy, false, false, false);
    assert!(host.graph.host_snapshot.edges.iter().any(|edge| edge.source == "src@out" && edge.target == "tgt@in"), "proximity drag should commit edge");
}

#[test]
fn dag_host_node_drag_skips_wired_cut_inputs() {
    let inputs = vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }, IoPortSpec { id: "b".into(), label: "b".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let src_w = computation_node_width("Src", &[], &outputs);
    let cut_w = computation_node_width("Cut", &inputs, &outputs);
    let src_h = computation_node_height(0, 1, false, false);
    let cut_h = computation_node_height(2, 1, false, false);
    let mut host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![
            DagNodeSpec::computation("sphere".into(), "Sphere", "Sphere", "emoji:🔵️".into(), vec![], outputs.clone(), false, false, 0.0, -60.0, src_w, src_h),
            DagNodeSpec::computation("torus".into(), "Torus", "Torus", "emoji:🍩️".into(), vec![], outputs.clone(), false, false, 0.0, 60.0, src_w, src_h),
            DagNodeSpec::computation("cut".into(), "Cut", "Cut", "emoji:✂️".into(), inputs, outputs, false, false, 240.0, 0.0, cut_w, cut_h),
        ],
        edges: vec![DagHostSnapshotEdge { id: "e1".into(), source: "sphere@out".into(), target: "cut@a".into(), ..Default::default() }, DagHostSnapshotEdge { id: "e2".into(), source: "torus@out".into(), target: "cut@b".into(), ..Default::default() }],
    });
    assert_eq!(host.graph.engine.edges.len(), 2, "fixture edges should load into engine");
    host.viewport.set_viewport(1280, 800, 1.0);
    host.graph.set_proximity_distance(160.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("normal");
    let cut_center = canvas::Point::new(240.0, 0.0);
    let (sx, sy) = world_to_screen_px(&host, cut_center);
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx - 180.0, sy, false, false, false);
    assert!(host.graph.engine.render_snapshot().pending_edge.is_none(), "dragging wired cut near sources must not preview proximity edges to occupied inputs");
    host.pointer_up_screen(sx - 180.0, sy, false, false, false);
    assert_eq!(host.graph.engine.edges.len(), 2);
    assert_eq!(host.graph.host_snapshot.edges.len(), 2);
}

#[test]
fn dag_host_keeps_same_named_input_and_output_handles_distinct() {
    let solid = vec![IoPortSpec { id: "solid".into(), label: "solid".into(), ..Default::default() }];
    let brep = vec![IoPortSpec { id: "brep".into(), label: "brep".into(), ..Default::default() }];
    let list = vec![IoPortSpec { id: "list".into(), label: "list".into(), ..Default::default() }];
    let mut host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![
            DagNodeSpec::computation("extrude".into(), "Extrude", "Extrude", "emoji:⬆️".into(), vec![], solid.clone(), false, false, 0.0, 0.0, computation_node_width("Extrude", &[], &solid), computation_node_height(0, 1, false, false)),
            DagNodeSpec::computation("brep".into(), "Brep", "Brep", "emoji:🧊️".into(), brep.clone(), brep.clone(), false, false, 200.0, 0.0, computation_node_width("Brep", &brep, &brep), computation_node_height(1, 1, false, false)),
            DagNodeSpec::computation(
                "get".into(),
                "Get",
                "Get",
                "emoji:📋️".into(),
                list,
                vec![],
                false,
                false,
                400.0,
                0.0,
                computation_node_width("Get", &[IoPortSpec { id: "list".into(), label: "list".into(), ..Default::default() }], &[]),
                computation_node_height(1, 0, false, false),
            ),
        ],
        edges: vec![
            DagHostSnapshotEdge { id: "e100".into(), source: "extrude@solid".into(), target: "brep@brep".into(), ..Default::default() },
            DagHostSnapshotEdge { id: "e101".into(), source: "brep@brep".into(), target: "get@list".into(), ..Default::default() },
        ],
    });
    host.graph.sync_edges_from_engine();
    let incoming = host.graph.engine.edges.get(&100).expect("incoming brep edge");
    let outgoing = host.graph.engine.edges.get(&101).expect("outgoing brep edge");
    let incoming_target = host.graph.engine.handles.get(&incoming.target).expect("incoming target handle");
    let outgoing_source = host.graph.engine.handles.get(&outgoing.source).expect("outgoing source handle");
    assert_eq!(incoming_target.role, HandleRole::Target);
    assert_eq!(outgoing_source.role, HandleRole::Source);
    assert_eq!(host.graph.host_snapshot.edges.iter().find(|edge| edge.id == "e100").map(|edge| edge.target.as_str()), Some("brep@brep"));
    assert_eq!(host.graph.host_snapshot.edges.iter().find(|edge| edge.id == "e101").map(|edge| edge.source.as_str()), Some("brep@brep"));
}

#[test]
fn dag_host_proximity_zero_disables_node_drag_connect() {
    let inputs = vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let src_w = computation_node_width("Src", &[], &outputs);
    let tgt_w = computation_node_width("Tgt", &inputs, &outputs);
    let src_h = computation_node_height(0, 1, false, false);
    let tgt_h = computation_node_height(1, 1, false, false);
    let mut host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![
            DagNodeSpec::computation("src".into(), "Src", "Src", "emoji:🔢️".into(), vec![], outputs.clone(), false, false, 0.0, 0.0, src_w, src_h),
            DagNodeSpec::computation("tgt".into(), "Tgt", "Tgt", "emoji:🔢️".into(), inputs, outputs, false, false, 220.0, 0.0, tgt_w, tgt_h),
        ],
        edges: vec![],
    });
    host.viewport.set_viewport(1280, 800, 1.0);
    host.graph.set_proximity_distance(0.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("normal");
    let (sx, sy) = world_to_screen_px(&host, canvas::Point::new(0.0, 0.0));
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx + 200.0, sy, false, false, false);
    assert!(host.graph.engine.render_snapshot().pending_edge.is_none());
    host.pointer_up_screen(sx + 200.0, sy, false, false, false);
    assert!(host.graph.host_snapshot.edges.is_empty());
}

/// 🔌️ Only the whole-graph SILHOUETTE withholds ports: every tier that draws a node draws its port
/// rows and publishes their connector rects, so a press on one wires there. The minimap tier owns the
/// bounded selection-AABB drag instead.
#[test]
fn only_the_silhouette_lod_withholds_port_hit_picking() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    let handle = handle_world(&host, "combine@b");
    let (hsx, hsy) = world_to_screen_px(&host, handle);
    host.viewport.set_forced_draw_lod_label("minimap");
    assert!(!draw_lod_for_frame(&host.viewport, &host.graph).allows_connection_hit_picking(), "minimap");
    host.pointer_down(hsx, hsy, false);
    assert!(!matches!(host.graph.engine.interaction, InteractionMode::DrawEdge { .. }), "minimap must not start an edge draw");
    host.pointer_up(hsx, hsy);
    for lod in ["overview", "compact", "normal", "detail", "micro"] {
        host.viewport.set_forced_draw_lod_label(lod);
        assert!(draw_lod_for_frame(&host.viewport, &host.graph).allows_connection_hit_picking(), "{lod}");
        host.pointer_down(hsx, hsy, false);
        assert!(matches!(host.graph.engine.interaction, InteractionMode::DrawEdge { .. }), "{lod} must wire from a port");
        host.pointer_up(hsx, hsy);
    }
}

/// 🔌️ The row splits in two: its outer CONNECTOR share wires, its interior drags the node. Both the
/// anchor the cap is painted at and the rect the host publishes fall in the connector share.
#[test]
fn normal_lod_input_row_drags_node_handle_anchor_starts_edge_draw() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("normal");
    let combine = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "combine").expect("combine");
    let port_idx = combine.inputs().iter().position(|p| p.id == "b").expect("port b");
    let (x0, y0, x1, y1) = input_port_row_hit_bounds(combine, port_idx).expect("row bounds");
    let row_center = canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
    let handle = handle_world(&host, "combine@b");
    assert!((row_center.x - handle.x).abs() > 4.0, "row center should sit away from the painted handle anchor");
    let (sx, sy) = world_to_screen_px(&host, row_center);
    host.pointer_down(sx, sy, false);
    assert!(matches!(host.graph.engine.interaction, InteractionMode::DragNode { .. }), "interior rectangle drag should move the node");
    host.pointer_up(sx, sy);
    let (hsx, hsy) = world_to_screen_px(&host, handle);
    host.pointer_down(hsx, hsy, false);
    assert!(matches!(host.graph.engine.interaction, InteractionMode::DrawEdge { .. }));
}

#[test]
fn set_hover_channel_targets_port_handle_at_detail_lod() {
    let mut host = DagHost::default_demo();
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("detail");
    set_hover_channel(&mut host.viewport, &mut host.graph, Some("combine"), Some("b"));
    assert_eq!(host.graph.hovered_channel(), Some(DagChannelRef { widget_id: "combine".into(), port: "b".into(), direction: DagChannelDirection::In }));
    set_hover_channel(&mut host.viewport, &mut host.graph, None, None);
    assert!(host.graph.hovered_channel().is_none());
}

#[test]
fn set_hover_channel_falls_back_to_node_at_compact_lod() {
    let mut host = DagHost::default_demo();
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("compact");
    set_hover_channel(&mut host.viewport, &mut host.graph, Some("combine"), Some("b"));
    assert_eq!(hovered_node_id(&host.viewport, &host.graph).as_deref(), Some("combine"));
    assert!(host.graph.hovered_channel().is_none());
}

#[test]
fn hovered_channel_decodes_port_handle_at_detail_lod() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("detail");
    let combine = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "combine").expect("combine").clone();
    let port_idx = combine.inputs().iter().position(|p| p.id == "b").expect("port b");
    let (x0, y0, x1, y1) = input_port_row_hit_bounds(&combine, port_idx).expect("row bounds");
    let row_center = canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
    let (sx, sy) = world_to_screen_px(&host, row_center);
    host.pointer_move_screen(sx, sy, false, false, false);
    assert_eq!(host.graph.hovered_channel(), Some(DagChannelRef { widget_id: "combine".into(), port: "b".into(), direction: DagChannelDirection::In }));
}
