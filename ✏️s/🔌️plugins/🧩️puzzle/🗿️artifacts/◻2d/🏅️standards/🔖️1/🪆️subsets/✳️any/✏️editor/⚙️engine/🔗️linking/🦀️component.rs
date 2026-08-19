//! 🔗️ Puzzle 2d app engine — the handle-to-handle wiring laws: link-drag snapping, proximity
//! connect, the indirect ring pick, kind-compatibility filtering and the hidden/locked guards.
//! Behaviour lives in the `infinite-board` kernel; this node pins the puzzle-2d contract on it.

//#region 🧪️Tests
#[cfg(test)]
#[allow(clippy::approx_constant, reason = "3.14159 is verbatim fixture data (a handle angle in a scene JSON literal), carried over unchanged from the pre-consolidation engine crate; swapping in std::f64::consts::PI would alter the recorded test input.")]
mod tests {
    use crate::editor::puzzle2d::engine::board_host::testkit::*;
    
    use crate::editor::puzzle2d::engine::canvas::Point;
    use crate::editor::puzzle2d::engine::{
        distance_between, handle_position_on_circle, handle_position_on_rectangle, BoardHost, EdgeDescJson, EdgeStrokePattern, EdgeTipGeometry, GraphPortMode, HandleDescJson, Interaction, NodeDescJson, NodeShape, SceneDescriptorJson,
    };
    use serde_json::json;

    #[semio_framework_async_macros::async_test]
    async fn board_host_node_drag_proximity_connect_overlapping_compatible_handles() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let center_b = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_down_screen(center_b.x, center_b.y, 0, false, false);
        let overlap = h.world_to_screen(Point::new(60.0, 0.0));
        h.pointer_move_screen(overlap.x, overlap.y, false, false, false);
        assert!(matches!(h.interaction, Interaction::DragNodes { proximity_pair: Some(_), .. }), "expected proximity preview wire while overlapping compatible nodes");
        h.pointer_up_screen(overlap.x, overlap.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"), "expected edgeCreate, got: {ev}");
        assert!(ev.contains("proximityConnect"), "expected proximityConnect, got: {ev}");
        assert!(ev.contains("b:h0"));
        assert!(ev.contains("a:h0"));
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_node_drag_skips_proximity_when_moving_node_has_incident_edge() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_a_to_b_linked()).unwrap();
        let _ = h.drain_events_json();
        let center_b = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_down_screen(center_b.x, center_b.y, 0, false, false);
        let overlap = h.world_to_screen(Point::new(60.0, 0.0));
        h.pointer_move_screen(overlap.x, overlap.y, false, false, false);
        assert!(matches!(h.interaction, Interaction::DragNodes { proximity_pair: None, .. }), "connected moving node must not preview node-drag proximity");
        h.pointer_up_screen(overlap.x, overlap.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("proximityConnect"), "expected no proximityConnect, got: {ev}");
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_link_drag_snap_emits_edge_create() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"));
        assert!(ev.contains("proximityConnect"));
        assert!(ev.contains("a:h0"));
        assert!(ev.contains("b:h0"));
        let created: Vec<_> = h.edges.keys().filter(|k| k.starts_with("edge-link-")).cloned().collect();
        assert_eq!(created.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_link_drag_snap_micro_zoom_rectangle_compatible_handles() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_micro_lod(&mut h);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id":"core.rect.bottom","name":"B","color":"#112233","defaultWireKind":"link.w"},
                    {"id":"core.rect.top","name":"T","color":"#112233","defaultWireKind":"link.w"}
                ],
                "wireKinds": [{"id":"link.w","name":"W","defaultEdgeKind":"link.e"}],
            })
            .to_string(),
        )
        .unwrap();
        h.set_handle_link_compat_from_json(r#"[{"source":"core.rect.bottom","target":"core.rect.top","specificity":"handle"}]"#).unwrap();
        let desc = SceneDescriptorJson {
            nodes: vec![
                NodeDescJson {
                    id: "a".into(),
                    x: 0.0,
                    y: 100.0,
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
                    shape: Some("rectangle".into()),
                    radius: None,
                    width: Some(100.0),
                    height: Some(56.0),
                    scale: None,
                },
                NodeDescJson {
                    id: "b".into(),
                    x: 0.0,
                    y: 20.0,
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
                    shape: Some("rectangle".into()),
                    radius: None,
                    width: Some(100.0),
                    height: Some(56.0),
                    scale: None,
                },
            ],
            handles: vec![
                HandleDescJson {
                    id: "a:h0".into(),
                    node_id: "a".into(),
                    angle: std::f64::consts::PI,
                    radius: None,
                    selected: None,
                    style: None,
                    handle_kind: Some("core.rect.bottom".into()),
                    color: None,
                    icon_kind: None,
                    user_data: None,
                    visible: None,
                    locked: None,
                    scale: None,
                },
                HandleDescJson {
                    id: "b:h0".into(),
                    node_id: "b".into(),
                    angle: 0.0,
                    radius: None,
                    selected: None,
                    style: None,
                    handle_kind: Some("core.rect.top".into()),
                    color: None,
                    icon_kind: None,
                    user_data: None,
                    visible: None,
                    locked: None,
                    scale: None,
                },
            ],
            edges: vec![],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let pa = handle_position_on_rectangle(Point::new(0.0, 100.0), 100.0, 56.0, std::f64::consts::PI);
        let pb = handle_position_on_rectangle(Point::new(0.0, 20.0), 100.0, 56.0, 0.0);
        let s0 = h.world_to_screen(pa);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let mid = Point::new(0.0, 60.0);
        let s_mid = h.world_to_screen(mid);
        h.pointer_move_screen(s_mid.x, s_mid.y, false, false, false);
        let s1 = h.world_to_screen(pb);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        assert!(matches!(h.interaction, Interaction::LinkDragSnap { ref target_id, .. } if target_id.as_deref() == Some("b:h0")), "expected drag snap onto b:h0 at micro zoom");
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"), "expected edgeCreate, got: {ev}");
        assert!(ev.contains("proximityConnect"), "expected proximityConnect, got: {ev}");
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_link_drag_snap_proximity_connect_in_overview_lod() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_overview_lod(&mut h);
        h.sync_descriptor(&link_test_scene_no_edge_non_draggable_nodes()).unwrap();
        let _ = h.drain_events_json();
        let center_a = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(center_a.x, center_a.y, 0, false, false);
        h.pointer_up_screen(center_a.x, center_a.y, false, false, false);
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"), "expected edgeCreate at overview LOD, got: {ev}");
        assert!(ev.contains("proximityConnect") || ev.contains("indirectConnect"), "expected proximityConnect or indirectConnect, got: {ev}");
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_parses_mindmap_fixture_without_handles() {
        let mut h = BoardHost::new_normal();
        let fixture = json!({
            "schema": "reasoning.mindmap.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "a", "x": 0.0, "y": 0.0, "width": 48.0, "height": 48.0, "shape": "rectangle", "root": true },
                { "id": "b", "x": 120.0, "y": 0.0, "width": 40.0, "height": 40.0, "shape": "rectangle" }
            ],
            "edges": [
                { "id": "e1", "source": "a", "target": "b", "edgeKind": "wires.owns" }
            ]
        });
        assert!(h.parse_fixture_v1(&fixture));
        assert_eq!(h.port_mode, GraphPortMode::Normal);
        assert!(h.handles.is_empty());
        assert_eq!(h.edges.len(), 1);
        assert_eq!(h.edges.get("e1").unwrap().source, "a");
        assert_eq!(h.edges.get("e1").unwrap().target, "b");
        h.set_size(800, 600, 1.0);
        let scene = h.build_vector_scene();
        assert!(scene.path_count() > 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_ingests_edge_and_node_kind_catalog_visual_fields() {
        let mut h = BoardHost::new_normal();
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "edgeKinds": [
                    {"id":"wires.owns","name":"Owns","color":"#ff0000","stroke":"3","pattern":"dashed","targetTip":"filled-diamond","directed":false},
                    {"id":"wires.is","name":"Is","color":"#00ff00","pattern":"dotted","targetTip":"filled-arrow","directed":false}
                ],
                "nodeKinds": [
                    {"id":"capsule","name":"Capsule","shape":"circle","color":"#aabbcc"}
                ]
            })
            .to_string(),
        )
        .unwrap();
        let owns = h.edge_kinds.get("wires.owns").expect("owns edge kind");
        assert_eq!(owns.stroke_width, 3.0);
        assert_eq!(owns.pattern, EdgeStrokePattern::Dashed);
        assert_eq!(owns.target_tip.as_deref(), Some("filled-diamond"));
        assert!(!owns.directed);
        assert!(owns.color.is_some());
        let is = h.edge_kinds.get("wires.is").expect("is edge kind");
        assert_eq!(is.pattern, EdgeStrokePattern::Dotted);
        assert_eq!(is.target_tip.as_deref(), Some("filled-arrow"));
        assert!(!is.directed);
        let diamond = h.edge_tips.get("filled-diamond").expect("filled-diamond tip");
        assert_eq!(diamond.geometry, EdgeTipGeometry::Diamond);
        assert!(diamond.filled);
        let capsule = h.node_kinds.get("capsule").expect("capsule node kind");
        assert_eq!(capsule.shape, NodeShape::Circle);
        assert!(capsule.color_fill.is_some());
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_sync_descriptor_normal_graph_node_id_edges() {
        let mut h = BoardHost::new_normal();
        let desc = SceneDescriptorJson {
            nodes: vec![
                NodeDescJson {
                    id: "a".into(),
                    x: 0.0,
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
                    root: Some(true),
                    shape: Some("rectangle".into()),
                    radius: None,
                    width: Some(48.0),
                    height: Some(48.0),
                    scale: None,
                },
                NodeDescJson {
                    id: "b".into(),
                    x: 120.0,
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
                    shape: Some("rectangle".into()),
                    radius: None,
                    width: Some(40.0),
                    height: Some(40.0),
                    scale: None,
                },
            ],
            handles: vec![],
            edges: vec![EdgeDescJson { id: "e1".into(), source: "a".into(), target: "b".into(), edge_kind: Some("wires.owns".into()), source_tip: None, target_tip: None, selected: None, style: None, user_data: None, visible: None, locked: None }],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        assert!(h.handles.is_empty());
        assert_eq!(h.edges.get("e1").unwrap().source, "a");
        assert_eq!(h.edges.get("e1").unwrap().target, "b");
        h.set_size(800, 600, 1.0);
        let scene = h.build_vector_scene();
        assert!(scene.path_count() > 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_hidden_handle_blocks_proximity_connect() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 280.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port", "hidden": true }]
                }
            ],
            "edges": []
        });
        assert!(h.parse_fixture_v1(&fixture));
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("edgeCreate"), "hidden handle should block connect, got: {ev}");
        assert!(h.edges.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_hidden_node_blocks_indirect_connect() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "parent" }]
                },
                {
                    "id": "b",
                    "x": 280.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "hidden": true,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "child" }]
                }
            ],
            "edges": []
        });
        assert!(h.parse_fixture_v1(&fixture));
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let inside_a = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(inside_a.x, inside_a.y, 0, false, false);
        let inside_b = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_move_screen(inside_b.x, inside_b.y, false, false, false);
        h.pointer_up_screen(inside_b.x, inside_b.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("edgeCreate"), "hidden node should block indirect connect, got: {ev}");
        assert!(matches!(h.interaction, Interaction::None));
        assert!(h.edges.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_locked_node_blocks_hit_select() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let mut desc = sample_scene();
        desc.nodes[0].locked = Some(true);
        h.sync_descriptor(&desc).unwrap();
        assert!(h.resolve_hit_world(Point::new(0.0, 0.0)).is_none());
        h.update_hover_from_world(Point::new(0.0, 0.0));
        assert_ne!(h.hovered_id.as_deref(), Some("a"));
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_locked_handle_blocks_proximity_connect() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 280.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port", "locked": true }]
                }
            ],
            "edges": []
        });
        assert!(h.parse_fixture_v1(&fixture));
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("edgeCreate"), "locked handle should block connect, got: {ev}");
        assert!(h.edges.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_overview_lod_omits_direct_handle_resolve_hit() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_overview_lod(&mut h);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let probe = Point::new(hp.x + 3.0, hp.y);
        assert_ne!(h.resolve_hit_world(probe).as_deref(), Some("a:h0"));
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_link_rejects_incompatible_handle_kind_pairs() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_handle_link_compat_from_json(r#"[{"source":"child","target":"parent"}]"#).unwrap();
        let desc = link_test_scene_no_edge();
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("edgeCreate"));
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_link_accepts_matching_handle_kind_pair() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        let desc = link_test_scene_no_edge();
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"));
        assert!(ev.contains("proximityConnect"));
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_normal_lod_prefers_node_at_center_and_handle_off_rim() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        assert_eq!(h.resolve_hit_world(Point::new(0.0, 0.0)).as_deref(), Some("a"));
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let probe = Point::new(hp.x + 2.0, hp.y);
        assert_eq!(h.resolve_hit_world(probe).as_deref(), Some("a:h0"));
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_indirect_ring_resolve_skips_connected_handles() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_node_a_two_handles_one_busy()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let ha0 = h.handles.get("a:h0").unwrap();
        let ring_busy = h.indirect_handle_world_pos(ha0).unwrap();
        assert_ne!(h.resolve_hit_world(ring_busy).as_deref(), Some("a:h0"));
        assert_eq!(h.resolve_hit_world(Point::new(0.0, 0.0)).as_deref(), Some("a:h1"));
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_indirect_sole_compatible_drop_creates_edge_immediately() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let inside_a = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(inside_a.x, inside_a.y, 0, false, false);
        assert!(matches!(
            h.interaction,
            Interaction::LinkAtSourceHandle { ref source_id, .. } if source_id == "a:h0"
        ));
        let inside_b = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_move_screen(inside_b.x, inside_b.y, false, false, false);
        assert!(matches!(h.interaction, Interaction::LinkDragSnap { .. }));
        h.pointer_up_screen(inside_b.x, inside_b.y, false, false, false);
        assert!(matches!(h.interaction, Interaction::None));
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"));
        assert!(ev.contains("indirectConnect"));
        assert!(ev.contains("a:h0"));
        assert!(ev.contains("b:h0"));
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_indirect_two_compatible_child_handles_on_target_require_ring_pick() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_b_two_free_child_handles()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let sa = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(sa.x, sa.y, 0, false, false);
        let sb = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_move_screen(sb.x, sb.y, false, false, false);
        h.pointer_up_screen(sb.x, sb.y, false, false, false);
        assert!(matches!(
            h.interaction,
            Interaction::LinkTargetNode { ref target_node_id, .. } if target_node_id == "b"
        ));
        let b0 = h.handles.get("b:h0").unwrap();
        let ring0 = h.indirect_handle_world_pos(b0).unwrap();
        let s1 = h.world_to_screen(ring0);
        h.pointer_down_screen(s1.x, s1.y, 0, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"));
        assert!(ev.contains("indirectConnect"));
        assert!(ev.contains("a:h0"));
        assert!(ev.contains("b:h0"));
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_indirect_target_click_elsewhere_stops_wire() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_b_two_free_child_handles()).unwrap();
        h.set_selection_ids(&["a".into()]);
        let sa = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(sa.x, sa.y, 0, false, false);
        let target_center = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_move_screen(target_center.x, target_center.y, false, false, false);
        h.pointer_up_screen(target_center.x, target_center.y, false, false, false);
        assert!(matches!(h.interaction, Interaction::LinkTargetNode { .. }));
        h.pointer_down_screen(20.0, 20.0, 0, false, false);
        assert!(matches!(h.interaction, Interaction::None));
        assert!(h.edges.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_indirect_ring_shown_when_node_has_two_free_handles() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_node_a_two_free_handles()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let ha0 = h.handles.get("a:h0").unwrap();
        let ring = h.indirect_handle_world_pos(ha0).unwrap();
        assert_eq!(h.resolve_hit_world(ring).as_deref(), Some("a:h0"));
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_indirect_ring_paints_without_rebuilding_world_cache() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_node_a_two_free_handles()).unwrap();
        let gen = h.test_content_scene_generation();
        let neutral_hint = h.encoded_scene_hint();
        h.set_selection_ids_silent(&["a".into()]);
        assert_eq!(h.test_content_scene_generation(), gen);
        let ha0 = h.handles.get("a:h0").unwrap();
        let ring = h.indirect_handle_world_pos(ha0).unwrap();
        assert_eq!(h.resolve_hit_world(ring).as_deref(), Some("a:h0"));
        assert!(h.encoded_scene_hint() > neutral_hint, "indirect ring must paint in the live overlay, not only in stale cached geometry");
        h.set_selection_ids_silent(&[]);
        assert_eq!(h.encoded_scene_hint(), neutral_hint);
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_link_drag_emits_compatible_nodes_and_target_ring_events() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_b_two_free_child_handles()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let sa = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(sa.x, sa.y, 0, false, false);
        let sb = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_move_screen(sb.x, sb.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("linkCompatibleNodes"), "got: {ev}");
        assert!(ev.contains(r#""nodeIds":["b"]"#) || ev.contains(r#""nodeIds": ["b"]"#), "got: {ev}");
        assert!(ev.contains("linkTargetRing"), "got: {ev}");
        assert!(ev.contains("b:h0") && ev.contains("b:h1"), "got: {ev}");
        let ring = h.indirect_handle_world_pos(h.handles.get("b:h1").unwrap()).unwrap();
        assert_eq!(h.resolve_hit_world(ring).as_deref(), Some("b:h1"));
        h.pointer_up_screen(20.0, 20.0, false, false, false);
        let ev_end = h.drain_events_json();
        assert!(ev_end.contains("linkCompatibleNodes"));
        assert!(ev_end.contains(r#""nodeIds":[]"#) || ev_end.contains(r#""nodeIds": []"#));
        assert!(ev_end.contains("linkTargetRing"));
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_indirect_ring_gap_scales_with_node_across_zoom() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let ha = h.handles.get("a:h0").unwrap().clone();
        let node_r = 40.0_f64;
        let body = || handle_position_on_circle(Point::new(0.0, 0.0), node_r, 0.0);
        let gap_ratio = |host: &BoardHost| {
            let ring = host.indirect_handle_world_pos(&ha).unwrap();
            let gap_px = distance_between(host.world_to_screen(ring), host.world_to_screen(body()));
            gap_px / (node_r * host.camera.zoom)
        };
        h.set_camera(0.0, 0.0, 1.0);
        let ratio_z1 = gap_ratio(&h);
        let gap_px_z1 = node_r * ratio_z1;
        h.set_camera(0.0, 0.0, 4.25);
        let ratio_z2 = gap_ratio(&h);
        let gap_px_z2 = node_r * 4.25 * ratio_z2;
        assert!((ratio_z1 - ratio_z2).abs() < 1e-6, "rim-to-ring ratios differ: {ratio_z1} vs {ratio_z2}");
        assert!((ratio_z1 - 0.7).abs() < 1e-6);
        assert!((gap_px_z2 - gap_px_z1 * 4.25).abs() < 0.6, "screen gap should scale with zoom: {gap_px_z1} vs {gap_px_z2}");
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_indirect_handle_marker_radius_scales_with_node_extent() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let ha = h.handles.get("a:h0").unwrap();
        assert!((h.indirect_handle_marker_radius_world(ha) - 32.0).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_handle_scale_combines_node_and_kind_scales() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [{"id":"slot-a","name":"Slot A","color":"#112233","scale":2.0}],
                "nodeKinds": [{"id":"kind-a","name":"Kind A","scale":1.5}],
            })
            .to_string(),
        )
        .unwrap();
        let mut desc = link_test_scene_no_edge();
        desc.nodes[0].node_kind = Some("kind-a".into());
        desc.nodes[0].scale = Some(2.0);
        desc.handles[0].handle_kind = Some("slot-a".into());
        desc.handles[0].scale = Some(0.5);
        h.sync_descriptor(&desc).unwrap();
        let ha = h.handles.get("a:h0").unwrap();
        assert_eq!(h.resolve_hit_world(Point::new(120.0, 0.0)).as_deref(), Some("a:h0"));
        assert!((h.indirect_handle_marker_radius_world(ha) - 96.0).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_link_wire_specificity_allows_when_handle_row_absent() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [{"id":"parent","name":"P","color":"#112233","defaultWireKind":"flow.wire"}],
                "wireKinds": [{"id":"flow.wire","name":"W","defaultEdgeKind":"flow.edge"}],
            })
            .to_string(),
        )
        .unwrap();
        h.set_handle_link_compat_from_json(r#"[{"source":"flow.wire","target":"child","specificity":"wire"}]"#).unwrap();
        let desc = link_test_scene_no_edge();
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"));
        assert!(ev.contains("proximityConnect"));
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_kind_catalog_accepts_modern_hsl_handle_colors() {
        let mut h = BoardHost::new();
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id":"space","name":"S","color":"hsl(206 52% 48%)"},
                    {"id":"comma","name":"C","color":"hsl(206, 52%, 48%)"},
                    {"id":"slash","name":"Sl","color":"hsl(206 52% 48% / 0.5)"},
                ],
            })
            .to_string(),
        )
        .unwrap();
        let c_space = h.handle_kinds.get("space").expect("space").color;
        let c_comma = h.handle_kinds.get("comma").expect("comma").color;
        let c_slash = h.handle_kinds.get("slash").expect("slash").color;
        assert_eq!(c_space, c_comma);
        assert_ne!(c_space, c_slash);
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_rejects_kind_catalog_rows_with_legacy_label() {
        let mut h = BoardHost::new();
        let err = h.set_board_kind_catalogs_from_json(&serde_json::json!({"handleKinds":[{"id":"h","label":"legacy","color":"#112233"}]}).to_string()).unwrap_err();
        assert!(err.to_string().contains("legacy label"));
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_link_important_pair_overrides_lower_specificity_filter() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [{"id":"parent","name":"P","color":"#112233","defaultWireKind":"flow.wire"}],
                "wireKinds": [{"id":"flow.wire","name":"W"}],
            })
            .to_string(),
        )
        .unwrap();
        h.set_handle_link_compat_from_json(
            r#"[
				{"source":"flow.wire","target":"nope","specificity":"wire"},
				{"source":"parent","target":"child","specificity":"general","important":true}
			]"#,
        )
        .unwrap();
        let desc = link_test_scene_no_edge();
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"));
        assert!(ev.contains("proximityConnect"));
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_link_drag_does_not_snap_when_target_handle_busy() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_target_b_handle_busy()).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("edgeCreate"));
        assert_eq!(h.edges.len(), 1);
        assert!(h.edges.contains_key("e-bc"));
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_link_does_not_start_from_busy_source_handle() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.sync_descriptor(&link_test_scene_a_to_b_linked()).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        assert!(matches!(h.interaction, Interaction::None));
        assert!(!h.drain_events_json().contains("edgeCreate"));
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_indirect_does_not_commit_on_busy_target_handle() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_target_b_handle_busy()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let sa = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(sa.x, sa.y, 0, false, false);
        let target_center = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_move_screen(target_center.x, target_center.y, false, false, false);
        h.pointer_up_screen(target_center.x, target_center.y, false, false, false);
        assert!(matches!(
            h.interaction,
            Interaction::LinkTargetNode {
                ref source_id,
                ref target_node_id
            } if source_id == "a:h0" && target_node_id == "b"
        ));
        let _ = h.drain_events_json();
        let sb = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_down_screen(sb.x, sb.y, 0, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("edgeCreate"));
        assert_eq!(h.edges.len(), 1);
        assert!(matches!(h.interaction, Interaction::None));
    }

    #[semio_framework_async_macros::async_test]
    async fn board_host_link_short_drag_does_not_emit_edge_create() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        h.pointer_move_screen(s0.x + 2.0, s0.y, false, false, false);
        h.pointer_up_screen(s0.x + 2.0, s0.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("edgeCreate"));
    }
}
//#endregion 🧪️Tests
