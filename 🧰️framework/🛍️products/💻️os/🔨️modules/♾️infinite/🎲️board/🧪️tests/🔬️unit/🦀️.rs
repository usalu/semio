
use super::*;
#[test]
fn port_directed_engine_round_trip() {
    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.create_node(1, 0.0, 0.0, 40.0, true);
    engine.create_handle(10, 1, 0.0);
    engine.create_handle(11, 1, std::f64::consts::PI);
    engine.create_edge(100, 10, 11);
    let snap = engine.render_snapshot();
    assert_eq!(snap.nodes.len(), 1);
    assert_eq!(snap.handles.len(), 2);
    assert_eq!(snap.edges.len(), 1);
}

#[test]
fn normal_directed_node_edges() {
    let mut engine = GraphEngine::<Normal, Directed>::new();
    engine.create_node(1, 0.0, 0.0, 40.0, true);
    engine.create_node(2, 120.0, 0.0, 40.0, true);
    engine.create_edge(100, 1, 2);
    let snap = engine.render_snapshot();
    assert_eq!(snap.nodes.len(), 2);
    assert!(snap.handles.is_empty());
    assert_eq!(snap.edges.len(), 1);
}

#[test]
fn undirected_normalizes_endpoints() {
    let mut engine = GraphEngine::<Normal, Undirected>::new();
    engine.create_node(1, 0.0, 0.0, 40.0, true);
    engine.create_node(2, 120.0, 0.0, 40.0, true);
    engine.create_edge(100, 2, 1);
    let edge = engine.edges.get(&100).unwrap();
    assert_eq!(edge.source, 1);
    assert_eq!(edge.target, 2);
}

#[test]
fn hit_test_pick_targets_collects_node_and_handle() {
    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.create_rect_node(1, 0.0, 0.0, 80.0, 56.0, true);
    engine.create_handle(10, 1, 0.0);
    let node = engine.nodes.get(&1).unwrap().clone();
    let handle = engine.handles.get(&10).unwrap().clone();
    let point = handle_position(&node, &handle);
    let targets = engine.hit_test_pick_targets(point);
    assert!(targets.iter().any(|row| row.domain == "node"));
    assert!(targets.iter().any(|row| row.domain == "handle"));
}

#[test]
fn selection_union_drag_starts_inside_bounds_without_node_hit() {
    use canvas::Point;

    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.create_rect_node(1, 0.0, 0.0, 80.0, 56.0, true);
    engine.create_rect_node(2, 200.0, 0.0, 80.0, 56.0, true);
    engine.selection.node_ids.insert(1);
    engine.selection.node_ids.insert(2);
    let gap = Point::new(100.0, 0.0);
    assert!(engine.try_begin_selection_union_drag_at(gap, 0.0));
    assert!(matches!(engine.interaction, InteractionMode::DragNodes { .. }));
    engine.pointer_move(150.0, 30.0);
    engine.pointer_up(150.0, 30.0);
    let a = engine.nodes.get(&1).unwrap().center;
    let b = engine.nodes.get(&2).unwrap().center;
    assert!((a.x - 50.0).abs() < 1e-3 && (a.y - 30.0).abs() < 1e-3);
    assert!((b.x - 250.0).abs() < 1e-3 && (b.y - 30.0).abs() < 1e-3);
}

#[test]
fn selection_drag_enclosing_lasso_uses_first_horizontal_step() {
    use canvas::Point;

    let start = Point::new(100.0, 100.0);
    let left_first = vec![start, Point::new(80.0, 100.0), Point::new(120.0, 100.0)];
    assert!(!selection_drag_enclosing("lasso", start, &left_first));
    let right_first = vec![start, Point::new(120.0, 100.0), Point::new(80.0, 100.0)];
    assert!(selection_drag_enclosing("lasso", start, &right_first));
    let rectangle = vec![start, Point::new(80.0, 100.0)];
    assert!(!selection_drag_enclosing("rectangle", start, &rectangle));
    assert!(!selection_drag_enclosing_rectangle(start, Point::new(80.0, 100.0)));
}

#[test]
fn secondary_pointer_down_on_node_selects_without_dragging() {
    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.create_rect_node(1, 100.0, 50.0, 160.0, 72.0, true);
    let before = engine.nodes.get(&1).unwrap().center;
    engine.pointer_down_screen(100.0, 50.0, 100.0, 50.0, 2, false, false, false);
    assert!(engine.selection.node_ids.contains(&1), "secondary click should select the node");
    assert!(matches!(engine.interaction, InteractionMode::Idle), "secondary click must not start a drag");
    engine.pointer_move_screen(140.0, 80.0, 140.0, 80.0, false, false, false);
    engine.pointer_up_screen(140.0, 80.0, 140.0, 80.0, false, false, false);
    let after = engine.nodes.get(&1).unwrap().center;
    assert!((after.x - before.x).abs() < 1e-9 && (after.y - before.y).abs() < 1e-9, "secondary click must not move the node");
}

#[test]
fn secondary_pointer_down_on_empty_keeps_selection() {
    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.create_rect_node(1, 100.0, 50.0, 160.0, 72.0, true);
    engine.selection.node_ids.insert(1);
    engine.pointer_down_screen(400.0, 400.0, 400.0, 400.0, 2, false, false, false);
    assert!(engine.selection.node_ids.contains(&1), "secondary empty click must keep selection for context menus");
    assert!(matches!(engine.interaction, InteractionMode::Idle));
}

#[test]
fn rect_node_drags_from_center() {
    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.create_rect_node(1, 100.0, 50.0, 160.0, 72.0, true);
    engine.pointer_down(100.0, 50.0, false);
    assert!(matches!(engine.interaction, InteractionMode::DragNode { .. }));
    engine.pointer_move(140.0, 80.0);
    engine.pointer_up(140.0, 80.0);
    let c = engine.nodes.get(&1).unwrap().center;
    assert!((c.x - 140.0).abs() < 0.01);
    assert!((c.y - 80.0).abs() < 0.01);
}

#[test]
fn rect_node_hit_and_wire_connect() {
    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.enforce_acyclic = true;
    engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
    engine.create_rect_node(2, 220.0, 0.0, 160.0, 72.0, true);
    engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
    engine.set_handle_role(10, HandleRole::Source);
    engine.set_handle_role(11, HandleRole::Target);
    engine.pointer_down(80.0, 0.0, false);
    engine.pointer_up(140.0, 0.0);
    assert_eq!(engine.edges.len(), 1);
    assert!(engine.render_snapshot().pending_edge.is_none());
}

#[test]
fn reconnect_replaces_incoming_edge() {
    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.create_rect_node(1, 0.0, 0.0, 80.0, 56.0, true);
    engine.create_rect_node(2, 160.0, 0.0, 80.0, 56.0, true);
    engine.create_rect_node(3, 320.0, 0.0, 80.0, 56.0, true);
    engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.create_handle(11, 2, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.create_handle(12, 3, std::f64::consts::FRAC_PI_2);
    engine.set_handle_role(10, HandleRole::Source);
    engine.set_handle_role(11, HandleRole::Source);
    engine.set_handle_role(12, HandleRole::Target);
    engine.create_edge(4, 11, 12);
    use canvas::Point;
    let tgt = handle_position_on_rectangle(Point::new(320.0, 0.0), 80.0, 56.0, std::f64::consts::FRAC_PI_2);
    let src = handle_position_on_rectangle(Point::new(0.0, 0.0), 80.0, 56.0, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.pointer_down(tgt.x, tgt.y, false);
    engine.pointer_move(src.x, src.y);
    engine.pointer_up(src.x, src.y);
    assert_eq!(engine.edges.len(), 1);
    let edge = engine.edges.values().next().unwrap();
    assert_eq!(edge.source, 10);
    assert_eq!(edge.target, 12);
}

#[test]
fn draw_edge_preview_uses_bezier_from_source_and_target() {
    use canvas::Point;

    fn midpoint_bulge(curve: CubicBez) -> f64 {
        let p0 = curve.p0();
        let p3 = curve.p3();
        let mid = curve.eval(0.5);
        let chord = p3 - p0;
        let len = chord.hypot();
        if len <= f64::EPSILON {
            return 0.0;
        }
        let t = ((mid - p0).dot(chord)) / (len * len);
        let proj = p0 + chord * t;
        (mid - proj).hypot()
    }

    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.proximity_distance_world = 0.0;
    engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
    engine.create_rect_node(2, 280.0, 0.0, 160.0, 72.0, true);
    engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
    engine.set_handle_role(10, HandleRole::Source);
    engine.set_handle_role(11, HandleRole::Target);
    let out = handle_position_on_rectangle(Point::new(0.0, 0.0), 160.0, 72.0, 3.0 * std::f64::consts::FRAC_PI_2);
    let cursor = Point::new(220.0, 40.0);
    engine.pointer_down(out.x, out.y, false);
    engine.pointer_move(cursor.x, cursor.y);
    let from_source = engine.render_snapshot().pending_edge.expect("source drag preview");
    assert!(midpoint_bulge(from_source) > 1.0, "source-anchored preview should bow away from chord");
    engine.pointer_up(cursor.x, cursor.y);

    let inp = handle_position_on_rectangle(Point::new(280.0, 0.0), 160.0, 72.0, std::f64::consts::FRAC_PI_2);
    engine.pointer_down(inp.x, inp.y, false);
    engine.pointer_move(cursor.x, cursor.y);
    let from_target = engine.render_snapshot().pending_edge.expect("target drag preview");
    assert!(midpoint_bulge(from_target) > 1.0, "target-anchored preview should bow away from chord");
}

#[test]
fn wire_snaps_preview_and_connects_to_compatible_handle() {
    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.enforce_acyclic = true;
    engine.set_camera(0.0, 0.0, 1.0);
    engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
    engine.create_rect_node(2, 280.0, 0.0, 160.0, 72.0, true);
    engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
    engine.set_handle_role(10, HandleRole::Source);
    engine.set_handle_role(11, HandleRole::Target);
    let out = handle_position_on_rectangle(Point::new(0.0, 0.0), 160.0, 72.0, 3.0 * std::f64::consts::FRAC_PI_2);
    let inp = handle_position_on_rectangle(Point::new(280.0, 0.0), 160.0, 72.0, std::f64::consts::FRAC_PI_2);
    engine.pointer_down(out.x, out.y, false);
    let near = Point::new(inp.x + 24.0, inp.y);
    engine.pointer_move(near.x, near.y);
    let InteractionMode::DrawEdge { snap_target, .. } = engine.interaction else {
        panic!("expected draw-edge interaction");
    };
    assert_eq!(snap_target, Some(11));
    let preview = engine.render_snapshot().pending_edge.expect("preview");
    let target_node = engine.nodes.get(&2).expect("target node");
    let target_handle = engine.handles.get(&11).expect("target handle");
    let outward = handle_outward_at_node_rim(inp, target_node.center, target_node.shape, target_node.radius, target_node.width, target_node.height).expect("target outward");
    let peak = handle_exterior_cap_peak(inp, outward, target_handle.radius);
    assert!((preview.p3.x - peak.x).abs() < 0.01);
    assert!((preview.p3.y - peak.y).abs() < 0.01);
    engine.pointer_up(near.x, near.y);
    assert_eq!(engine.edges.len(), 1);
    let edge = engine.edges.values().next().expect("edge");
    assert_eq!(Ported::endpoint_as_u64(edge.source), 10);
    assert_eq!(Ported::endpoint_as_u64(edge.target), 11);
}

#[test]
fn wire_snap_replaces_occupied_compatible_target() {
    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.enforce_acyclic = true;
    engine.set_camera(0.0, 0.0, 1.0);
    engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
    engine.create_rect_node(2, 280.0, 0.0, 160.0, 72.0, true);
    engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
    engine.create_handle(12, 2, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.set_handle_role(10, HandleRole::Source);
    engine.set_handle_role(11, HandleRole::Target);
    engine.set_handle_role(12, HandleRole::Source);
    engine.create_edge(100, 12, 11);
    let out = handle_position_on_rectangle(Point::new(0.0, 0.0), 160.0, 72.0, 3.0 * std::f64::consts::FRAC_PI_2);
    let occupied = handle_position_on_rectangle(Point::new(280.0, 0.0), 160.0, 72.0, std::f64::consts::FRAC_PI_2);
    engine.pointer_down(out.x, out.y, false);
    engine.pointer_move(occupied.x + 8.0, occupied.y);
    let InteractionMode::DrawEdge { snap_target, .. } = engine.interaction else {
        panic!("expected draw-edge interaction");
    };
    assert_eq!(snap_target, Some(11));
    engine.pointer_up(occupied.x + 8.0, occupied.y);
    assert_eq!(engine.edges.len(), 1);
    let edge = engine.edges.values().next().expect("edge");
    assert_eq!(Ported::endpoint_as_u64(edge.source), 10);
    assert_eq!(Ported::endpoint_as_u64(edge.target), 11);
}

#[test]
fn wire_snap_ignores_incompatible_handles() {
    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.enforce_acyclic = true;
    engine.set_camera(0.0, 0.0, 1.0);
    engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
    engine.create_rect_node(2, 280.0, 0.0, 160.0, 72.0, true);
    engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.create_handle(12, 2, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.set_handle_role(10, HandleRole::Source);
    engine.set_handle_role(12, HandleRole::Source);
    let out = handle_position_on_rectangle(Point::new(0.0, 0.0), 160.0, 72.0, 3.0 * std::f64::consts::FRAC_PI_2);
    let other_out = handle_position_on_rectangle(Point::new(280.0, 0.0), 160.0, 72.0, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.pointer_down(out.x, out.y, false);
    engine.pointer_move(other_out.x + 8.0, other_out.y);
    let InteractionMode::DrawEdge { snap_target, .. } = engine.interaction else {
        panic!("expected draw-edge interaction");
    };
    assert!(snap_target.is_none());
}

#[test]
fn proximity_zero_disables_wire_snap() {
    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.proximity_distance_world = 0.0;
    engine.enforce_acyclic = true;
    engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
    engine.create_rect_node(2, 280.0, 0.0, 160.0, 72.0, true);
    engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
    engine.set_handle_role(10, HandleRole::Source);
    engine.set_handle_role(11, HandleRole::Target);
    let out = handle_position_on_rectangle(Point::new(0.0, 0.0), 160.0, 72.0, 3.0 * std::f64::consts::FRAC_PI_2);
    let inp = handle_position_on_rectangle(Point::new(280.0, 0.0), 160.0, 72.0, std::f64::consts::FRAC_PI_2);
    engine.pointer_down(out.x, out.y, false);
    engine.pointer_move(inp.x + 8.0, inp.y);
    let InteractionMode::DrawEdge { snap_target, .. } = engine.interaction else {
        panic!("expected draw-edge interaction");
    };
    assert!(snap_target.is_none());
}

#[test]
fn node_drag_proximity_connects_compatible_channels() {
    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.enforce_acyclic = true;
    engine.proximity_distance_world = 80.0;
    engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
    engine.create_rect_node(2, 200.0, 0.0, 160.0, 72.0, true);
    engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
    engine.set_handle_role(10, HandleRole::Source);
    engine.set_handle_role(11, HandleRole::Target);
    engine.pointer_down(0.0, 0.0, false);
    engine.pointer_move(40.0, 0.0);
    assert!(engine.render_snapshot().pending_edge.is_some());
    engine.pointer_up(40.0, 0.0);
    assert_eq!(engine.edges.len(), 1);
    let edge = engine.edges.values().next().expect("edge");
    assert_eq!(Ported::endpoint_as_u64(edge.source), 10);
    assert_eq!(Ported::endpoint_as_u64(edge.target), 11);
}

#[test]
fn node_drag_proximity_skips_wired_handles_on_dragged_node() {
    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.enforce_acyclic = true;
    engine.proximity_distance_world = 120.0;
    engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
    engine.create_rect_node(2, 220.0, 0.0, 160.0, 72.0, true);
    engine.create_rect_node(3, 440.0, 0.0, 160.0, 72.0, true);
    engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
    engine.create_handle(12, 2, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.create_handle(13, 3, std::f64::consts::FRAC_PI_2);
    engine.set_handle_role(10, HandleRole::Source);
    engine.set_handle_role(11, HandleRole::Target);
    engine.set_handle_role(12, HandleRole::Source);
    engine.set_handle_role(13, HandleRole::Target);
    engine.create_edge(100, 10, 11);
    engine.create_edge(101, 12, 13);
    engine.pointer_down(220.0, 0.0, false);
    engine.pointer_move(260.0, 0.0);
    assert!(engine.render_snapshot().pending_edge.is_none(), "wired mid-node drag must not flip proximity preview between input and output");
    engine.pointer_up(260.0, 0.0);
    assert_eq!(engine.edges.len(), 2);
}

#[test]
fn node_drag_proximity_skips_occupied_input() {
    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.enforce_acyclic = true;
    engine.proximity_distance_world = 80.0;
    engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
    engine.create_rect_node(2, 200.0, 0.0, 160.0, 72.0, true);
    engine.create_rect_node(3, 400.0, 0.0, 160.0, 72.0, true);
    engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
    engine.create_handle(12, 3, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.set_handle_role(10, HandleRole::Source);
    engine.set_handle_role(11, HandleRole::Target);
    engine.set_handle_role(12, HandleRole::Source);
    engine.create_edge(100, 10, 11);
    engine.pointer_down(400.0, 0.0, false);
    engine.pointer_move(220.0, 0.0);
    assert!(engine.render_snapshot().pending_edge.is_none());
    engine.pointer_up(220.0, 0.0);
    assert_eq!(engine.edges.len(), 1);
    let edge = engine.edges.values().next().expect("edge");
    assert_eq!(Ported::endpoint_as_u64(edge.source), 10);
    assert_eq!(Ported::endpoint_as_u64(edge.target), 11);
}

#[test]
fn node_drag_alt_suppresses_proximity_preview_and_connect() {
    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.enforce_acyclic = true;
    engine.proximity_distance_world = 80.0;
    engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
    engine.create_rect_node(2, 200.0, 0.0, 160.0, 72.0, true);
    engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
    engine.set_handle_role(10, HandleRole::Source);
    engine.set_handle_role(11, HandleRole::Target);
    engine.pointer_down(0.0, 0.0, false);
    engine.pointer_move_screen(40.0, 0.0, 40.0, 0.0, false, false, true);
    assert!(engine.render_snapshot().pending_edge.is_none());
    engine.pointer_up_screen(40.0, 0.0, 40.0, 0.0, false, false, true);
    assert_eq!(engine.edges.len(), 0);
    assert_eq!(engine.proximity_distance_world, 80.0);
}

#[test]
fn node_drag_alt_restores_proximity_when_released() {
    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.enforce_acyclic = true;
    engine.proximity_distance_world = 80.0;
    engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
    engine.create_rect_node(2, 200.0, 0.0, 160.0, 72.0, true);
    engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
    engine.set_handle_role(10, HandleRole::Source);
    engine.set_handle_role(11, HandleRole::Target);
    engine.pointer_down(0.0, 0.0, false);
    engine.pointer_move_screen(40.0, 0.0, 40.0, 0.0, false, false, true);
    assert!(engine.render_snapshot().pending_edge.is_none());
    engine.pointer_move_screen(40.0, 0.0, 40.0, 0.0, false, false, false);
    assert!(engine.render_snapshot().pending_edge.is_some());
    assert_eq!(engine.proximity_distance_world, 80.0);
}

#[test]
fn wire_snap_connects_fan_out_from_same_output() {
    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.enforce_acyclic = true;
    engine.set_camera(0.0, 0.0, 1.0);
    engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
    engine.create_rect_node(2, 280.0, -60.0, 160.0, 72.0, true);
    engine.create_rect_node(3, 280.0, 60.0, 160.0, 72.0, true);
    engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
    engine.create_handle(12, 3, std::f64::consts::FRAC_PI_2);
    engine.set_handle_role(10, HandleRole::Source);
    engine.set_handle_role(11, HandleRole::Target);
    engine.set_handle_role(12, HandleRole::Target);
    engine.create_edge(100, 10, 11);
    let out = handle_position_on_rectangle(Point::new(0.0, 0.0), 160.0, 72.0, 3.0 * std::f64::consts::FRAC_PI_2);
    let second = handle_position_on_rectangle(Point::new(280.0, 60.0), 160.0, 72.0, std::f64::consts::FRAC_PI_2);
    engine.pointer_down(out.x, out.y, false);
    engine.pointer_move(second.x + 8.0, second.y);
    let InteractionMode::DrawEdge { snap_target, .. } = engine.interaction else {
        panic!("expected draw-edge interaction");
    };
    assert_eq!(snap_target, Some(12));
    engine.pointer_up(second.x + 8.0, second.y);
    assert_eq!(engine.edges.len(), 2);
    assert!(engine.edges.values().any(|edge| Ported::endpoint_as_u64(edge.source) == 10 && Ported::endpoint_as_u64(edge.target) == 11));
    assert!(engine.edges.values().any(|edge| Ported::endpoint_as_u64(edge.source) == 10 && Ported::endpoint_as_u64(edge.target) == 12));
}

#[test]
fn node_drag_proximity_allows_fan_out_from_occupied_source() {
    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.enforce_acyclic = true;
    engine.proximity_distance_world = 120.0;
    engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
    engine.create_rect_node(2, 220.0, 0.0, 160.0, 72.0, true);
    engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
    engine.create_handle(12, 2, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.set_handle_role(10, HandleRole::Source);
    engine.set_handle_role(11, HandleRole::Target);
    engine.set_handle_role(12, HandleRole::Target);
    engine.create_edge(100, 10, 11);
    engine.pointer_down(220.0, 0.0, false);
    engine.pointer_move(40.0, 0.0);
    assert!(engine.render_snapshot().pending_edge.is_some());
    engine.pointer_up(40.0, 0.0);
    assert_eq!(engine.edges.len(), 2);
    assert!(engine.edges.values().any(|edge| Ported::endpoint_as_u64(edge.source) == 10 && Ported::endpoint_as_u64(edge.target) == 12));
}

#[test]
fn pick_merge_mode_for_modifiers_matches_puzzle() {
    assert_eq!(pick_merge_mode_for_modifiers(false, false, "replace"), "replace");
    assert_eq!(pick_merge_mode_for_modifiers(false, true, "replace"), "additive");
    assert_eq!(pick_merge_mode_for_modifiers(true, false, "replace"), "subtractive");
    assert_eq!(pick_merge_mode_for_modifiers(true, true, "replace"), "invertive");
}

#[test]
fn acyclic_rejects_back_edge() {
    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.enforce_acyclic = true;
    engine.create_rect_node(1, 0.0, 0.0, 80.0, 56.0, true);
    engine.create_rect_node(2, 160.0, 0.0, 80.0, 56.0, true);
    engine.create_handle(10, 1, std::f64::consts::FRAC_PI_2);
    engine.create_handle(11, 2, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.set_handle_role(10, HandleRole::Source);
    engine.set_handle_role(11, HandleRole::Target);
    engine.create_edge(100, 10, 11);
    engine.pointer_down(160.0, 0.0, false);
    engine.pointer_up(0.0, 0.0);
    assert_eq!(engine.edges.len(), 1);
}

/// 🚫️ A wire whose two ends declare disjoint value schemas never snaps, never connects, and never
/// displaces the wire already on that target port — the board half of the port-type law
/// (`🌊️flow/🧫️fixtures/🔌️port-types/🔣️.json`). The pair is the defect's own:
/// `extrusion-axis@vectorOut` (a `vector`) dropped on `extrude@wire` (a `geometry`).
#[test]
fn a_wire_whose_ends_declare_disjoint_value_types_never_snaps_and_never_connects() {
    let mut engine = GraphEngine::<Ported, Directed>::new();
    engine.enforce_acyclic = true;
    engine.set_camera(0.0, 0.0, 1.0);
    engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
    engine.create_rect_node(2, 280.0, 0.0, 160.0, 72.0, true);
    engine.create_rect_node(3, 0.0, 200.0, 160.0, 72.0, true);
    engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
    engine.create_handle(12, 3, 3.0 * std::f64::consts::FRAC_PI_2);
    engine.set_handle_role(10, HandleRole::Source);
    engine.set_handle_role(11, HandleRole::Target);
    engine.set_handle_role(12, HandleRole::Source);
    engine.set_handle_value_types(10, vec!["vector".into()]);
    engine.set_handle_value_types(11, vec!["geometry".into()]);
    engine.set_handle_value_types(12, vec!["geometry".into()]);
    engine.create_edge(100, 12, 11);

    let out = handle_position_on_rectangle(Point::new(0.0, 0.0), 160.0, 72.0, 3.0 * std::f64::consts::FRAC_PI_2);
    let inp = handle_position_on_rectangle(Point::new(280.0, 0.0), 160.0, 72.0, std::f64::consts::FRAC_PI_2);
    engine.pointer_down(out.x, out.y, false);
    engine.pointer_move(inp.x, inp.y);
    let InteractionMode::DrawEdge { snap_target, .. } = engine.interaction else {
        panic!("expected draw-edge interaction");
    };
    assert_eq!(snap_target, None, "an incompatible port must never be a snap target");
    assert_eq!(engine.wire_drag_type_refusal(), Some((10, 11)), "the drag must name the pair it is refusing");
    engine.pointer_up(inp.x, inp.y);
    assert_eq!(engine.edges.len(), 1, "a refused drop adds no edge");
    let edge = engine.edges.values().next().expect("edge");
    assert_eq!(Ported::endpoint_as_u64(edge.source), 12, "the wire already on that port survives");
    assert_eq!(Ported::endpoint_as_u64(edge.target), 11);

    engine.set_handle_value_types(10, vec!["geometry".into()]);
    engine.pointer_down(out.x, out.y, false);
    engine.pointer_move(inp.x, inp.y);
    assert_eq!(engine.wire_drag_type_refusal(), None, "a compatible pair is not a refusal");
    engine.pointer_up(inp.x, inp.y);
    assert_eq!(engine.edges.len(), 1, "the compatible drop replaces the incoming wire");
    let edge = engine.edges.values().next().expect("edge");
    assert_eq!(Ported::endpoint_as_u64(edge.source), 10, "and it is the drawn wire that stands");
}

/// 🕹️ The gumball's pivot is the arithmetic mean of the SELECTED node centres — the same definition
/// the guest's `rotateSelection` reducer uses, so the in-canvas preview and the committed document
/// turn about the same point. Locked members still count toward it; only their positions stay.
#[test]
fn transform_pivot_is_the_selection_centroid() {
    assert_eq!(transform_pivot_of(&[]), None, "an empty selection has no pivot");
    assert_eq!(transform_pivot_of(&[Point::new(-40.0, 0.0), Point::new(40.0, 0.0)]), Some(Point::new(0.0, 0.0)));
    let pivot = transform_pivot_of(&[Point::new(0.0, 0.0), Point::new(30.0, 60.0), Point::new(-30.0, 0.0)]).expect("three centres have a pivot");
    assert!((pivot.x - 0.0).abs() < 1e-9 && (pivot.y - 20.0).abs() < 1e-9, "pivot {pivot:?}");
}

/// 🔄️ Rotation is counter-clockwise about the pivot and exact on the quarter turns.
#[test]
fn rotate_point_about_turns_counter_clockwise() {
    let turned = rotate_point_about(Point::new(0.0, 0.0), Point::new(40.0, 0.0), std::f64::consts::FRAC_PI_2);
    assert!(turned.x.abs() < 1e-9 && (turned.y - 40.0).abs() < 1e-9, "a quarter turn of (40,0) is (0,40), got {turned:?}");
    let about = rotate_point_about(Point::new(10.0, 10.0), Point::new(20.0, 10.0), std::f64::consts::PI);
    assert!((about.x - 0.0).abs() < 1e-9 && (about.y - 10.0).abs() < 1e-9, "a half turn reflects through the pivot, got {about:?}");
}

/// ⭕️ The ring keeps a constant SCREEN clearance: doubling the zoom halves its world radius gap, and
/// an empty-extent selection still gets a grabbable minimum.
#[test]
fn transform_ring_radius_keeps_a_constant_screen_gap() {
    let pivot = Point::new(0.0, 0.0);
    let corners = [Point::new(-100.0, -100.0), Point::new(100.0, 100.0)];
    let at_one = transform_ring_radius_world(pivot, &corners, 1.0);
    let at_two = transform_ring_radius_world(pivot, &corners, 2.0);
    let reach = (100.0_f64 * 100.0 + 100.0 * 100.0).sqrt();
    assert!((at_one - reach - TRANSFORM_RING_GAP_PX).abs() < 1e-9, "at zoom 1 the gap is one screen pixel per world unit, got {at_one}");
    assert!((at_two - reach - TRANSFORM_RING_GAP_PX / 2.0).abs() < 1e-9, "at zoom 2 the world gap halves, got {at_two}");
    let tiny = transform_ring_radius_world(pivot, &[Point::new(0.0, 0.0)], 1.0);
    assert!(tiny >= TRANSFORM_RING_MIN_RADIUS_PX, "a point-sized selection still gets a grabbable ring, got {tiny}");
    assert!(transform_ring_radius_world(pivot, &corners, 0.0) > 0.0, "a degenerate zoom must not produce a zero ring");
}

/// 🎯️ The ring is a band, not a disc: the pivot itself and the far outside both miss it.
#[test]
fn transform_ring_hit_is_a_band_around_the_radius() {
    let pivot = Point::new(0.0, 0.0);
    assert!(transform_ring_hit(pivot, 100.0, 1.0, Point::new(100.0, 0.0)), "dead on the ring is a hit");
    assert!(transform_ring_hit(pivot, 100.0, 1.0, Point::new(100.0 + TRANSFORM_RING_HIT_TOLERANCE_PX - 0.1, 0.0)), "inside the tolerance is a hit");
    assert!(!transform_ring_hit(pivot, 100.0, 1.0, Point::new(100.0 + TRANSFORM_RING_HIT_TOLERANCE_PX + 0.1, 0.0)), "past the tolerance is a miss");
    assert!(!transform_ring_hit(pivot, 100.0, 1.0, pivot), "the pivot is never on the ring");
}

/// 📐️ A drag across the ±π seam reports the SHORT way round, never a full extra turn.
#[test]
fn transform_ring_angle_delta_takes_the_short_way_round() {
    let pivot = Point::new(0.0, 0.0);
    let quarter = transform_ring_angle_delta(pivot, Point::new(10.0, 0.0), Point::new(0.0, 10.0));
    assert!((quarter - std::f64::consts::FRAC_PI_2).abs() < 1e-9, "got {quarter}");
    let seam = transform_ring_angle_delta(pivot, Point::new(-10.0, 0.001), Point::new(-10.0, -0.001));
    assert!(seam.abs() < 0.01, "crossing the seam must not jump a whole turn, got {seam}");
    let back = transform_ring_angle_delta(pivot, Point::new(0.0, 10.0), Point::new(10.0, 0.0));
    assert!((back + std::f64::consts::FRAC_PI_2).abs() < 1e-9, "the reverse drag is the negative delta, got {back}");
}

/// 🧲️ The grid-snap modifier quantizes the rotation; without it the angle passes through untouched.
#[test]
fn snap_transform_angle_quantizes_only_under_the_snap_modifier() {
    let raw = 20.0_f64.to_radians();
    assert!((snap_transform_angle(raw, false) - raw).abs() < 1e-12, "snap off leaves the angle alone");
    let snapped = snap_transform_angle(raw, true);
    assert!((snapped - 15.0_f64.to_radians()).abs() < 1e-9, "20° snaps to the 15° step, got {}", snapped.to_degrees());
    assert!(snap_transform_angle(0.0, true).abs() < 1e-12, "a zero drag stays a zero rotation");
    assert!((snap_transform_angle(-20.0_f64.to_radians(), true) + 15.0_f64.to_radians()).abs() < 1e-9, "negative drags snap symmetrically");
}
