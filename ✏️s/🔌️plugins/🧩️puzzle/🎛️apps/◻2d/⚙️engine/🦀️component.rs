//! ⚙️ Puzzle 2d app engine — headless compute + interactive host over the board scene, rehomed
//! app-side (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1e: an artifact is a `🧬️schema`
//! plus a `🚪️io` system, never an engine — the puzzle2d artifact's old `⚙️engine` owned a genuinely
//! stateful/interactive `BoardHost`, so it moves to the app that edits the artifact). Thin domain
//! facade over the `infinite-board` port-directed graph kernel: it re-exports that kernel's whole
//! surface (`BoardEngine`, `BoardHost`, the vector `canvas`, the force/hierarchical layouts) under
//! one name, tags it with the `puzzle.2d` canvas extension.
//!
//! 📚️ Sibling topic files: `🎲️board-host/🦀️component.rs` (the themed host constructors + their
//! scene/selection/hit-test laws), `🔗️linking/🦀️component.rs` (handle-to-handle wiring and
//! compatibility laws), `🖌️brush/🦀️component.rs` (brush slot/fill session laws),
//! `📐️layout/🦀️component.rs` (the redraw layout dispatcher), `🔣️icons/🦀️component.rs` (the
//! build-script-generated metabolism icon table and the SVG icon codec laws).
//!
//! 🌉️ Externally reachable at `puzzle::apps::puzzle2d::engine::*` — the framework OS renderer
//! (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/EngineCanvas/🧊️component.rs`)
//! holds a `BoardHost` and calls `board_host::puzzle_board_host()` directly, so this module and its
//! `board_host` child must both stay `pub`.
//!
//! 🧭️ Placement rule for helpers reaching across nodes: a helper with exactly ONE consumer lives in
//! that consumer's file; two or more consumers put it here.

pub use canvas::{CubicBez, Point, Vec2};
pub use graph::canvas;
pub use graph::{
    apply_edge_handle_snap_to_fixture_v1_json, apply_force_graph_layout_to_fixture_v1_json, apply_force_graph_layout_to_fixture_v1_value, apply_normal_undirected_redraw_layout_to_fixture_v1_json,
    apply_redraw_layout_to_fixture_v1_json as apply_ported_redraw_layout_to_fixture_v1_json, apply_undirected_force_graph_layout_to_fixture_v1_json, apply_undirected_force_graph_layout_to_fixture_v1_value, GraphExtension,
};
pub use infinite_board_port_directed_normal::{self as graph, *};
pub use semio_s_mindmap as mindmap;

//#region 🔖️Puzzle2dExtension
/// 🧩️ Puzzle 2d domain extension over the property graph canvas.
#[derive(Clone, Debug, Default)]
pub struct Puzzle2dExtension;

impl canvas::CanvasExtension for Puzzle2dExtension {
    fn extension_id(&self) -> &str {
        "puzzle.2d"
    }
}

impl GraphExtension for Puzzle2dExtension {}
//#endregion 🔖️Puzzle2dExtension

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::puzzle2d::engine::canvas::Point;

    #[test]
    fn computes_handle_positions_and_edge_curves() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 0.0, 0.0, 40.0, true);
        engine.create_node(2, 300.0, 0.0, 40.0, true);
        engine.create_handle(10, 1, 0.0);
        engine.create_handle(20, 2, std::f64::consts::PI);
        engine.create_edge(100, 10, 20);

        let curve = engine.edge_curve(100).expect("edge curve should exist");
        let p0 = curve.p0();
        let p1 = curve.p1();
        let p2 = curve.p2();
        let p3 = curve.p3();
        let cap = 8.0;
        assert!((p0.x() - (40.0 + cap)).abs() < 0.001);
        assert!(p0.y().abs() < 0.001);
        assert!((p3.x() - (260.0 - cap)).abs() < 0.001);
        assert!(p3.y().abs() < 0.001);
        let source_radial = p0 - Point::ZERO;
        let arm0 = p1 - p0;
        let align0 = normalize_or_zero(source_radial).dot(normalize_or_zero(arm0));
        let target_approach = Point::new(300.0, 0.0) - p3;
        let arm1 = p3 - p2;
        let align1 = normalize_or_zero(target_approach).dot(normalize_or_zero(arm1));
        assert!(align0 > 0.99);
        assert!(align1 > 0.99);
    }

    #[test]
    fn drags_nodes_without_rebuilding_the_scene_catalog() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 0.0, 0.0, 30.0, true);

        engine.pointer_down(0.0, 0.0, false);
        engine.pointer_move(60.0, 25.0);
        engine.pointer_up(60.0, 25.0);

        let node = engine.nodes.get(&1).expect("node should remain in the engine");
        assert_eq!(node.center, Point::new(60.0, 25.0));

        let events = engine.drain_events();
        assert!(events.iter().any(|event| matches!(event, BoardEvent::SelectionChanged { node_ids, .. } if node_ids == &vec![1])));
        assert!(events.iter().any(|event| matches!(event, BoardEvent::NodeMoved { id: 1, x, y } if (*x - 60.0).abs() < 0.001 && (*y - 25.0).abs() < 0.001)));
    }

    #[test]
    fn hit_tests_handles_before_nodes_and_edges() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 0.0, 0.0, 40.0, true);
        engine.create_node(2, 200.0, 0.0, 40.0, true);
        engine.create_handle(10, 1, 0.0);
        engine.create_handle(20, 2, std::f64::consts::PI);
        engine.create_edge(100, 10, 20);

        let handle_point = handle_position(engine.nodes.get(&1).unwrap(), engine.handles.get(&10).unwrap());
        engine.pointer_down(handle_point.x, handle_point.y, false);

        let events = engine.drain_events();
        assert!(events.iter().any(|event| matches!(event, BoardEvent::SelectionChanged { handle_ids, .. } if handle_ids == &vec![10])));
    }

    #[test]
    fn renders_snapshot_for_nodes_handles_and_edges() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 10.0, 20.0, 18.0, true);
        engine.create_node(2, 120.0, 20.0, 18.0, true);
        engine.create_handle(10, 1, 0.0);
        engine.create_handle(20, 2, std::f64::consts::PI);
        engine.create_edge(100, 10, 20);

        let snapshot = engine.render_snapshot();
        assert_eq!(snapshot.nodes.len(), 2);
        assert_eq!(snapshot.handles.len(), 2);
        assert_eq!(snapshot.edges.len(), 1);
    }

    #[test]
    fn engine_extend_pick_keeps_node_when_adding_handle() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 0.0, 0.0, 40.0, true);
        engine.create_node(2, 300.0, 0.0, 40.0, true);
        engine.create_handle(10, 1, 0.0);
        engine.create_handle(20, 2, std::f64::consts::PI);
        engine.create_edge(100, 10, 20);

        engine.pointer_down(0.0, 0.0, false);
        let _ = engine.drain_events();
        let hp = handle_position(engine.nodes.get(&1).unwrap(), engine.handles.get(&10).unwrap());
        engine.pointer_down(hp.x, hp.y, true);
        let events = engine.drain_events();
        let last = events.iter().rev().find_map(|event| match event {
            BoardEvent::SelectionChanged { node_ids, handle_ids, edge_ids } => Some((node_ids.clone(), handle_ids.clone(), edge_ids.clone())),
            _ => None,
        });
        let Some((node_ids, handle_ids, edge_ids)) = last else {
            panic!("expected SelectionChanged");
        };
        assert!(node_ids.contains(&1));
        assert!(handle_ids.contains(&10));
        assert!(edge_ids.is_empty());
    }
}
//#endregion 🧪️Tests
