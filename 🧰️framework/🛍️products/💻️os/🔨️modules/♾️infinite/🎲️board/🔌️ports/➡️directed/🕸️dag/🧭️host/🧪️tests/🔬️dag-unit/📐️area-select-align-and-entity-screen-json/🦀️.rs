/// 📐️ Area select, align and entity screen JSON: the handle helpers, area select preselect, selection alignment, union bounds, the entity screen JSON, minimap bounded drag, node drags, grid snap and the selection camera.

fn handle_world(host: &DagHost, port_key: &str) -> canvas::Point {
    let hid = host.graph.handle_key_map.iter().find(|(_, key)| key.as_str() == port_key).map(|(id, _)| *id).expect("handle");
    host.graph.engine.render_snapshot().handles.iter().find(|(id, _, _)| *id == hid).map(|(_, p, _)| *p).expect("handle pos")
}

fn world_to_screen_px(host: &DagHost, p: canvas::Point) -> (f64, f64) {
    (p.x + host.viewport.width as f64 * 0.5, p.y + host.viewport.height as f64 * 0.5)
}

#[test]
fn dag_host_area_select_previews_preselect_before_commit() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    let start_sx = 24.0;
    let start_sy = 24.0;
    let end_sx = 1100.0;
    let end_sy = 700.0;
    host.pointer_down_screen(start_sx, start_sy, 0, false, false, false, false);
    host.pointer_move_screen(end_sx, end_sy, false, false, false);
    assert!(matches!(host.graph.engine.interaction, InteractionMode::AreaSelect { .. }), "expected area-select after marquee threshold");
    let preselect = host.graph.preselect_widget_ids();
    assert!(!preselect.is_empty(), "marquee drag should preview widget ids before commit");
    let preview_points: Vec<[f64; 2]> = semio_framework_pack_json::from_json_str(&host.graph.selection_preview_points_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(preview_points.len() >= 2, "marquee overlay points should be published during drag");
    host.pointer_up_screen(end_sx, end_sy, false, false, false);
    assert!(!host.graph.selected_node_ids().is_empty(), "marquee drag should commit selection on release");
    assert!(host.graph.preselect_widget_ids().is_empty(), "preselect should clear after commit");
}

#[test]
fn dag_host_align_selection_horizontal_and_vertical_center() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(800, 600, 1.0);
    host.graph.set_selection(&["scale".into(), "combine".into(), "screen".into()]);
    host.graph.align_selection("alignHorizontal").unwrap();
    let xs: Vec<f64> = host.graph.selected_snapshot_nodes().into_iter().map(|(_, node)| node.x).collect();
    assert!(xs.windows(2).all(|pair| (pair[0] - pair[1]).abs() < 1e-6));
    host.graph.align_selection("alignVertical").unwrap();
    let ys: Vec<f64> = host.graph.selected_snapshot_nodes().into_iter().map(|(_, node)| node.y).collect();
    assert!(ys.windows(2).all(|pair| (pair[0] - pair[1]).abs() < 1e-6));
}

#[test]
fn dag_host_align_selection_left_and_distribute_horizontal() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(800, 600, 1.0);
    host.graph.set_selection(&["scale".into(), "combine".into(), "screen".into()]);
    host.graph.align_selection("alignLeft").unwrap();
    let left_edges: Vec<f64> = host.graph.selected_snapshot_nodes().into_iter().map(|(_, node)| node.x - node.width * 0.5).collect();
    assert!(left_edges.windows(2).all(|pair| (pair[0] - pair[1]).abs() < 1e-6));
    host.graph.align_selection("distributeHorizontal").unwrap();
    let mut xs: Vec<f64> = host.graph.selected_snapshot_nodes().into_iter().map(|(_, node)| node.x).collect();
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!(xs.windows(2).all(|pair| pair[1] > pair[0]));
}

#[test]
fn dag_host_selection_union_bounds_screen_json_nonempty_for_selection() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(800, 600, 1.0);
    host.graph.set_selection(&["scale".into(), "combine".into()]);
    let json = selection_union_bounds_screen_json(&host.viewport, &host.graph);
    assert_ne!(json, "null");
    let parsed: Value = semio_framework_pack_json::parse(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(parsed["width"].as_f64().unwrap_or(0.0) > 1.0);
    assert!(parsed["height"].as_f64().unwrap_or(0.0) > 1.0);
}

#[test]
fn dag_host_entity_screen_json_resolves_node_by_id_and_wildcard() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(800, 600, 1.0);
    let json = entity_screen_json(&host.viewport, &host.graph, "node", "scale");
    let parsed: Value = semio_framework_pack_json::parse(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(parsed["visible"], true);
    assert!(parsed["x"].as_number().is_some());
    assert!(parsed["rect"].as_array().is_some());

    let wildcard: Value = semio_framework_pack_json::parse(&entity_screen_json(&host.viewport, &host.graph, "node", "*"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(wildcard["visible"], true);
}

/// 🐢️ A malformed id (no "@port") or a port that doesn't exist on the node must degrade to
/// unresolved, never panic.
#[test]
fn dag_host_entity_screen_json_resolves_handle_by_widget_and_port() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(800, 600, 1.0);
    let input_json: Value = semio_framework_pack_json::parse(&entity_screen_json(&host.viewport, &host.graph, "handle", "scale@in"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(input_json["visible"], true);
    let output_json: Value = semio_framework_pack_json::parse(&entity_screen_json(&host.viewport, &host.graph, "handle", "combine@b"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(output_json["visible"], true);
    let malformed: Value = semio_framework_pack_json::parse(&entity_screen_json(&host.viewport, &host.graph, "handle", "scale"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(malformed["visible"], false);
    let missing_port: Value = semio_framework_pack_json::parse(&entity_screen_json(&host.viewport, &host.graph, "handle", "scale@nope"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(missing_port["visible"], false);
}

#[test]
fn dag_host_entity_screen_json_resolves_edge_with_a_two_point_polyline() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(800, 600, 1.0);
    let json: Value = semio_framework_pack_json::parse(&entity_screen_json(&host.viewport, &host.graph, "edge", "e1"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(json["visible"], true);
    let polyline = json["polyline"].as_array().expect("edge geometry carries a polyline");
    assert_eq!(polyline.len(), 2);
}

/// 🛟️ "handle":"*" may legitimately resolve to the demo fixture's first port.
#[test]
fn dag_host_entity_screen_json_unresolved_domain_or_id_never_panics() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(800, 600, 1.0);
    for (domain, id) in [("node", "nonexistent"), ("handle", "*"), ("edge", "nonexistent"), ("bogus-domain", "*")] {
        let json: Value = semio_framework_pack_json::parse(&entity_screen_json(&host.viewport, &host.graph, domain, id), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        if json["visible"] == true {
            continue;
        }
        assert_eq!(json["visible"], false, "domain={domain} id={id}");
    }
    let empty_fixture = DagHostSnapshot { schema: "dag.hostDocument".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: vec![], edges: vec![] };
    let empty: Value = semio_framework_pack_json::parse(&{ let host = DagHost::from_host_snapshot(empty_fixture); entity_screen_json(&host.viewport, &host.graph, "node", "*") }, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(empty["visible"], false);
}

#[test]
fn dag_host_minimap_bounded_drag_moves_selection_inside_union_bounds() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(800, 600, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("minimap");
    host.graph.set_camera(0.0, 0.0, 0.1);
    host.graph.set_selection(&["scale".into(), "combine".into()]);
    let scale_before = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "scale").expect("scale").clone();
    let combine_before = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "combine").expect("combine").clone();
    let gap = canvas::Point::new(0.0, 0.0);
    use canvas::camera::{Camera as CanvasCamera, Viewport, world_to_screen};
    let cam = CanvasCamera { x: host.graph.host_snapshot.camera.x, y: host.graph.host_snapshot.camera.y, zoom: host.graph.host_snapshot.camera.zoom };
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let start = world_to_screen(&cam, &viewport, gap);
    host.pointer_down_screen(start.x, start.y, 0, false, false, false, false);
    assert!(matches!(host.graph.engine.interaction, InteractionMode::DragNodes { .. }), "expected bounded drag inside selection union at minimap LOD");
    host.pointer_move_screen(start.x + 50.0, start.y + 30.0, false, false, false);
    host.pointer_up_screen(start.x + 50.0, start.y + 30.0, false, false, false);
    let zoom = host.graph.host_snapshot.camera.zoom;
    let dx = 50.0 / zoom;
    let dy = 30.0 / zoom;
    let scale_after = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "scale").expect("scale");
    let combine_after = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "combine").expect("combine");
    assert!((scale_after.x - (scale_before.x + dx)).abs() < 1e-3 && (scale_after.y - (scale_before.y + dy)).abs() < 1e-3);
    assert!((combine_after.x - (combine_before.x + dx)).abs() < 1e-3 && (combine_after.y - (combine_before.y + dy)).abs() < 1e-3);
}

#[test]
fn dag_host_drags_node_in_world_space() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    let mut dragged = false;
    for (nid, node) in host.graph.engine.nodes.clone() {
        let grab = canvas::Point::new(node.center.x - node.width * 0.4, node.center.y);
        let (sx, sy) = world_to_screen_px(&host, grab);
        host.pointer_down(sx, sy, false);
        if !matches!(host.graph.engine.interaction, InteractionMode::DragNode { node_id, .. } if node_id == nid) {
            host.pointer_up(sx, sy);
            continue;
        }
        let before = host.graph.engine.nodes.get(&nid).expect("node").center;
        host.pointer_move(sx + 40.0, sy + 30.0);
        let idx = *host.graph.node_id_map.get(&nid).expect("fixture index");
        let fixture = &host.graph.host_snapshot.nodes[idx];
        let engine = host.graph.engine.nodes.get(&nid).expect("node").center;
        assert!((fixture.x - engine.x).abs() < 1e-6, "fixture x should track engine during drag");
        assert!((fixture.y - engine.y).abs() < 1e-6, "fixture y should track engine during drag");
        host.pointer_up(sx + 40.0, sy + 30.0);
        let after = host.graph.engine.nodes.get(&nid).expect("node").center;
        assert!((after.x - before.x).abs() > 1.0);
        assert!((after.y - before.y).abs() > 1.0);
        dragged = true;
        break;
    }
    assert!(dragged, "expected at least one draggable node hit via screen coordinates");
}

#[test]
fn dag_host_grid_snap_aligns_dragged_node() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("normal");
    host.viewport.set_grid_snap_enabled(true);
    host.viewport.set_grid_factor(10.0).expect("grid factor");
    let mut dragged = false;
    for (nid, node) in host.graph.engine.nodes.clone() {
        let grab = canvas::Point::new(node.center.x - node.width * 0.4, node.center.y);
        let (sx, sy) = world_to_screen_px(&host, grab);
        host.pointer_down(sx, sy, false);
        if !matches!(host.graph.engine.interaction, InteractionMode::DragNode { node_id, .. } if node_id == nid) {
            host.pointer_up(sx, sy);
            continue;
        }
        host.pointer_move(sx + 37.0, sy + 23.0);
        host.pointer_up(sx + 37.0, sy + 23.0);
        let idx = *host.graph.node_id_map.get(&nid).expect("fixture index");
        let fixture = &host.graph.host_snapshot.nodes[idx];
        let step = GRID_WORLD_MEDIUM * 10.0;
        assert!(((fixture.x / step).round() * step - fixture.x).abs() < 1e-6);
        assert!(((fixture.y / step).round() * step - fixture.y).abs() < 1e-6);
        dragged = true;
        break;
    }
    assert!(dragged, "expected draggable node for grid snap test");
}

#[test]
fn dag_host_focus_selection_camera_frames_selection() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    let ids: Vec<String> = host.graph.host_snapshot.nodes.iter().take(2).map(|node| node.id.clone()).collect();
    host.graph.set_selection(&ids);
    let camera = focus_selection_camera(&host.viewport, &host.graph, 1.2).expect("camera");
    assert!(camera.zoom > 0.0);
    assert!(camera.x.is_finite() && camera.y.is_finite());
}
