//! 🎲️ Puzzle 2d artifact engine — the themed board hosts: `BoardHost` constructors wired to this
//! plugin's metabolism icon table, plus the scene-sync, camera/LOD, hit-test, selection and
//! area-select laws they must satisfy.

use crate::artifacts::puzzle2d::engine::icons::puzzle_themed_icon_lookup;
use crate::artifacts::puzzle2d::engine::BoardHost;

/// 🎲️ A `BoardHost` for the directed port graph, painting icons from this plugin's metabolism table.
pub fn puzzle_board_host() -> BoardHost {
    let mut h = BoardHost::new();
    h.icon_paint_cache.themed_icon_lookup = puzzle_themed_icon_lookup;
    h
}

/// 🎲️ The undirected ("normal") variant of [`puzzle_board_host`].
pub fn puzzle_board_host_normal() -> BoardHost {
    let mut h = BoardHost::new_normal();
    h.icon_paint_cache.themed_icon_lookup = puzzle_themed_icon_lookup;
    h
}

//#region 🧪️Tests
#[cfg(test)]
pub(crate) mod testkit {
    //! 🧪️ The one board-scene test harness — `🦀️linking.rs` and `🦀️brush.rs` build on it instead of
    //! re-deriving a camera/LOD/scene scaffold of their own.
    use crate::artifacts::puzzle2d::engine::{BoardHost, EdgeDescJson, HandleDescJson, NodeDescJson, SceneDescriptorJson};
    use serde_json::json;

    pub fn set_detail_lod(h: &mut BoardHost) {
        h.set_camera(0.0, 0.0, 2.0);
    }

    /// 🗂️ Board kind-catalog JSON for a compile-time manifest id — the catalogs live in the manifest
    /// registry (`mathematical_graph_manifest`), not in fixture `meta.kindCatalogs`, so tests that
    /// need real node/handle kinds read them from there. Each catalog row is the manifest row's
    /// `id`/`name` merged with its flattened `presentation` object.
    pub fn catalogs_json_from_manifest_id(manifest_id: &str) -> String {
        let manifest = mathematical_graph_manifest::manifest_by_id(manifest_id).unwrap_or_else(|| panic!("unknown manifest id {manifest_id}"));
        let rows = |kinds: &[mathematical_graph_manifest::KindDef]| -> Vec<serde_json::Value> {
            kinds
                .iter()
                .map(|kind| {
                    let mut row = serde_json::Map::new();
                    row.insert("id".to_string(), json!(kind.id));
                    row.insert("name".to_string(), json!(kind.name));
                    if let Some(serde_json::Value::Object(presentation)) = kind.presentation.as_ref() {
                        for (key, value) in presentation {
                            row.insert(key.clone(), value.clone());
                        }
                    }
                    serde_json::Value::Object(row)
                })
                .collect()
        };
        let visual_port_kinds: Vec<mathematical_graph_manifest::KindDef> = manifest.port_kinds.iter().filter(|kind| kind.presentation.as_ref().is_some_and(|p| p.get("color").is_some())).cloned().collect();
        json!({ "handleKinds": rows(&visual_port_kinds), "nodeKinds": rows(&manifest.node_kinds) }).to_string()
    }

    pub fn set_micro_lod(h: &mut BoardHost) {
        h.set_camera(0.0, 60.0, 4.5);
    }

    pub fn set_overview_lod(h: &mut BoardHost) {
        h.set_camera(0.0, 0.0, 0.25);
    }

    pub fn sample_scene() -> SceneDescriptorJson {
        SceneDescriptorJson {
            nodes: vec![NodeDescJson {
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
                root: None,
                shape: Some("circle".into()),
                radius: Some(40.0),
                width: None,
                height: None,
                scale: None,
            }],
            handles: vec![
                HandleDescJson {
                    id: "a:h0".into(),
                    node_id: "a".into(),
                    angle: 0.0,
                    radius: None,
                    selected: None,
                    style: None,
                    handle_kind: Some("port".into()),
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
                    angle: std::f64::consts::PI,
                    radius: None,
                    selected: None,
                    style: None,
                    handle_kind: Some("port".into()),
                    color: None,
                    icon_kind: None,
                    user_data: None,
                    visible: None,
                    locked: None,
                    scale: None,
                },
            ],
            edges: vec![EdgeDescJson { id: "e1".into(), source: "a:h0".into(), target: "b:h0".into(), edge_kind: None, source_tip: None, target_tip: None, selected: None, style: None, user_data: None, visible: None, locked: None }],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        }
    }

    pub fn link_test_scene_no_edge() -> SceneDescriptorJson {
        SceneDescriptorJson {
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
                    root: None,
                    shape: Some("circle".into()),
                    radius: Some(40.0),
                    width: None,
                    height: None,
                    scale: None,
                },
                NodeDescJson {
                    id: "b".into(),
                    x: 280.0,
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
                },
            ],
            handles: vec![
                HandleDescJson {
                    id: "a:h0".into(),
                    node_id: "a".into(),
                    angle: 0.0,
                    radius: None,
                    selected: None,
                    style: None,
                    handle_kind: Some("parent".into()),
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
                    angle: std::f64::consts::PI,
                    radius: None,
                    selected: None,
                    style: None,
                    handle_kind: Some("child".into()),
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
        }
    }

    pub fn link_test_scene_no_edge_non_draggable_nodes() -> SceneDescriptorJson {
        let mut s = link_test_scene_no_edge();
        for n in &mut s.nodes {
            n.draggable = Some(false);
        }
        s
    }

    pub fn link_test_scene_node_a_two_free_handles() -> SceneDescriptorJson {
        let mut s = link_test_scene_no_edge();
        s.handles.push(HandleDescJson {
            id: "a:h1".into(),
            node_id: "a".into(),
            angle: std::f64::consts::FRAC_PI_2,
            radius: None,
            selected: None,
            style: None,
            handle_kind: Some("parent".into()),
            color: None,
            icon_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            scale: None,
        });
        s
    }

    pub fn link_test_scene_b_two_free_child_handles() -> SceneDescriptorJson {
        let mut s = link_test_scene_no_edge();
        s.handles.push(HandleDescJson {
            id: "b:h1".into(),
            node_id: "b".into(),
            angle: 0.0,
            radius: None,
            selected: None,
            style: None,
            handle_kind: Some("child".into()),
            color: None,
            icon_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            scale: None,
        });
        s
    }

    pub fn link_test_scene_target_b_handle_busy() -> SceneDescriptorJson {
        let mut s = link_test_scene_no_edge();
        s.nodes.push(NodeDescJson {
            id: "c".into(),
            x: 560.0,
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
        s.handles.push(HandleDescJson {
            id: "c:h0".into(),
            node_id: "c".into(),
            angle: std::f64::consts::PI,
            radius: None,
            selected: None,
            style: None,
            handle_kind: Some("child".into()),
            color: None,
            icon_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            scale: None,
        });
        s.edges.push(EdgeDescJson { id: "e-bc".into(), source: "b:h0".into(), target: "c:h0".into(), edge_kind: None, source_tip: None, target_tip: None, selected: None, style: None, user_data: None, visible: None, locked: None });
        s
    }

    pub fn link_test_scene_a_to_b_linked() -> SceneDescriptorJson {
        let mut s = link_test_scene_no_edge();
        s.edges.push(EdgeDescJson { id: "e-ab".into(), source: "a:h0".into(), target: "b:h0".into(), edge_kind: None, source_tip: None, target_tip: None, selected: None, style: None, user_data: None, visible: None, locked: None });
        s
    }

    pub fn link_test_scene_node_a_two_handles_one_busy() -> SceneDescriptorJson {
        let mut s = link_test_scene_a_to_b_linked();
        s.handles.push(HandleDescJson {
            id: "a:h1".into(),
            node_id: "a".into(),
            angle: std::f64::consts::FRAC_PI_2,
            radius: None,
            selected: None,
            style: None,
            handle_kind: Some("parent".into()),
            color: None,
            icon_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            scale: None,
        });
        s
    }
}

#[cfg(test)]
mod tests {
    use super::testkit::*;
    use crate::artifacts::puzzle2d::engine::canvas;
    use crate::artifacts::puzzle2d::engine::canvas::geom_sel::cubic_bezier_point;
    use crate::artifacts::puzzle2d::engine::canvas::Point;
    use crate::artifacts::puzzle2d::engine::{
        compute_edge_bezier_points, handle_position_on_circle, BoardElementStyleKind, BoardHost, Interaction,
        NodeDescJson, WireDescJson,
    };
    use serde_json::json;

    /// 🔗️ Keeps the runtime kind-catalog JSON shape in sync with the compile-time `puzzle2d-default` manifest.
    #[test]
    fn puzzle2d_default_manifest_satisfies_board_host_validation() {
        let manifest: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/◻2d/🛂️manifest.jsondefault.manifest.json")).unwrap();
        let handle_kinds: Vec<serde_json::Value> =
            manifest["portKinds"].as_array().unwrap().iter().map(|row| json!({ "id": row["id"], "name": row["name"], "color": row["presentation"]["color"], "defaultWireKind": row["presentation"]["defaultWireKind"] })).collect();
        let wire_kinds: Vec<serde_json::Value> = manifest["wireKinds"].as_array().unwrap().iter().map(|row| json!({ "id": row["id"], "name": row["name"], "defaultEdgeKind": row["presentation"]["defaultEdgeKind"] })).collect();
        let edge_kinds: Vec<serde_json::Value> = manifest["edgeKinds"].as_array().unwrap().iter().map(|row| json!({ "id": row["id"], "name": row["name"] })).collect();
        let catalogs_json = json!({ "handleKinds": handle_kinds, "wireKinds": wire_kinds, "edgeKinds": edge_kinds }).to_string();

        let mut host = BoardHost::new();
        host.set_board_kind_catalogs_from_json(&catalogs_json).expect("catalog json derived from the manifest must be valid");
        host.validate_against_manifest_id("puzzle2d-default").expect("runtime catalog must satisfy the compile-time puzzle2d-default manifest");
    }

    #[test]
    fn board_host_defers_descriptor_sync_while_panning() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let _ = h.drain_events_json();
        h.pointer_down_screen(10.0, 10.0, 1, false, false);
        assert!(h.defers_descriptor_sync_from_js());
        h.pointer_move_screen(80.0, 60.0, false, false, false);
        assert!(h.defers_descriptor_sync_from_js());
        let _ = h.drain_events_json();
        h.pointer_up_screen(80.0, 60.0, false, false, false);
        assert!(!h.defers_descriptor_sync_from_js());
    }

    #[test]
    fn board_host_defers_descriptor_sync_while_dragging_nodes() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let _ = h.drain_events_json();
        let start = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(start.x, start.y, 0, false, false);
        assert!(matches!(h.interaction, Interaction::DragNodes { .. }));
        h.pointer_move_screen(start.x + 40.0, start.y, false, false, false);
        assert!(h.defers_descriptor_sync_from_js());
        let ev = h.drain_events_json();
        assert!(ev.contains("nodeMove"));
        h.pointer_up_screen(start.x + 40.0, start.y, false, false, false);
        assert!(!h.defers_descriptor_sync_from_js());
        let end = h.drain_events_json();
        assert!(end.contains("nodeDragEnd"));
    }

    #[test]
    fn board_host_set_node_positions_updates_existing_nodes_only() {
        let mut h = BoardHost::new();
        h.set_size(400, 300, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let gen_before = h.test_content_scene_generation();
        h.set_node_positions(&[("a".into(), 12.0, 34.0), ("missing".into(), 1.0, 2.0), ("a".into(), f64::NAN, 0.0)]);
        let node = h.nodes.get("a").expect("node a should remain");
        assert!((node.x - 12.0).abs() < 0.001);
        assert!((node.y - 34.0).abs() < 0.001);
        assert!(h.test_content_scene_generation() > gen_before, "moving nodes must invalidate cached world content");
        h.set_node_positions_json(r#"[{"id":"a","x":90.0,"y":110.0}]"#).unwrap();
        let node = h.nodes.get("a").expect("node a should remain");
        assert!((node.x - 90.0).abs() < 0.001);
        assert!((node.y - 110.0).abs() < 0.001);
    }

    #[test]
    fn board_host_overlay_paint_state_json_matches_host_camera_lod_and_node_centers() {
        let mut h = BoardHost::new();
        h.set_size(640, 480, 2.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        h.set_camera_silent(12.0, -8.0, 0.2);
        if let Some(n) = h.nodes.get_mut("a") {
            n.x = 33.0;
            n.y = 44.0;
        }
        let raw: serde_json::Value = serde_json::from_str(&h.overlay_paint_state_json()).expect("overlay paint state json");
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
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let gen_before = h.test_content_scene_generation();
        let s = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_move_screen(s.x + 80.0, s.y + 40.0, false, false, false);
        assert!(h.test_content_scene_generation() > gen_before, "node drag must rebuild cached nodes/handles, not only edges");
        let node = h.nodes.get("a").expect("dragged node");
        assert!(node.x.abs() > 1.0 || node.y.abs() > 1.0, "pointer move should translate node a away from origin");
    }

    #[test]
    fn board_host_manual_lod_follow_zoom_still_encodes_graph() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let with_automatic = h.encoded_scene_hint();
        assert!(with_automatic > 0, "sample scene should encode vector paths");
        h.set_automatic_lod(false);
        h.set_forced_draw_lod_label("");
        let manual_follow_zoom = h.encoded_scene_hint();
        assert!(manual_follow_zoom > 0, "manual follow-zoom LOD must still draw nodes/edges (hint={manual_follow_zoom})");
        h.set_forced_draw_lod_label("overview");
        let pinned_overview = h.encoded_scene_hint();
        assert!(pinned_overview > 0, "pinned overview LOD must still draw graph");
        h.set_automatic_lod(true);
        let automatic_restored = h.encoded_scene_hint();
        assert_eq!(with_automatic, automatic_restored);
    }

    #[test]
    fn board_host_pick_selection_never_sets_exit_highlight() {
        let mut h = BoardHost::new();
        h.set_size(400, 300, 1.0);
        let mut d = sample_scene();
        d.selection_exit_highlight_ids = vec!["a".into(), "ghost".into()];
        h.sync_descriptor(&d).unwrap();
        let _ = h.drain_events_json();
        assert!(h.selection_exit_highlight.is_empty());
        h.set_selection_ids(&["a".into(), "e1".into()]);
        let ev = h.drain_events_json();
        assert!(h.selection_exit_highlight.is_empty());
        assert!(ev.contains("\"exitHighlightIds\":[]"));
        h.set_selection_ids(&["e1".into()]);
        let ev2 = h.drain_events_json();
        assert!(h.selection_exit_highlight.is_empty());
        assert!(ev2.contains("\"exitHighlightIds\":[]"));
    }

    #[test]
    fn board_host_canvas_theme_keeps_explicit_element_state_colors() {
        let mut h = BoardHost::new();
        h.set_canvas_theme_from_json(
            r#"{
				"nodeStrokeHovered": [1, 2, 3, 255],
				"edgeStrokeHovered": [4, 5, 6, 255],
				"handleStrokeHovered": [7, 8, 9, 255],
				"wireStrokeHovered": [10, 11, 12, 255]
			}"#,
        )
        .unwrap();
        assert_eq!(h.canvas_theme.node_stroke_hovered.to_rgba8(), canvas::Color::from_rgba8(1, 2, 3, 255).to_rgba8());
        assert_eq!(h.canvas_theme.edge_stroke_hovered.to_rgba8(), canvas::Color::from_rgba8(4, 5, 6, 255).to_rgba8());
        assert_eq!(h.canvas_theme.handle_stroke_hovered.to_rgba8(), canvas::Color::from_rgba8(7, 8, 9, 255).to_rgba8());
        assert_eq!(h.canvas_theme.wire_stroke_hovered.to_rgba8(), canvas::Color::from_rgba8(10, 11, 12, 255).to_rgba8());
    }

    #[test]
    fn board_host_cancel_area_select_restores_initial_selection() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into(), "b".into()]);
        let _ = h.drain_events_json();
        h.pointer_down_screen(5.0, 5.0, 0, false, false);
        assert!(!h.is_dragging_area_select());
        h.pointer_move_screen(20.0, 5.0, false, false, false);
        assert!(h.is_dragging_area_select());
        let _ = h.drain_events_json();
        assert!(h.cancel_area_select());
        assert!(!h.is_dragging_area_select());
        let ev = h.drain_events_json();
        assert!(ev.contains("preselectCancel"));
        assert!(!ev.contains("\"select\""));
        assert_eq!(h.selection.len(), 2);
        assert!(h.selection.contains("a") && h.selection.contains("b"));
        assert!(h.preselect.is_empty());
    }

    #[test]
    fn board_host_syncs_descriptor_and_hit_tests_handle_before_node() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
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
        let hit = h.resolve_hit_world(hp);
        assert_eq!(hit.as_deref(), Some("a:h0"));
        assert!(h.encoded_scene_hint() > 10);
    }

    #[test]
    fn board_host_cached_content_includes_edge_vector_paths_at_overview_zoom() {
        let mut h = BoardHost::new();
        h.set_size(1200, 800, 1.0);
        h.sync_descriptor(&link_test_scene_a_to_b_linked()).unwrap();
        h.set_camera_silent(0.0, 0.0, 0.21);
        let with_edges = h.encoded_scene_hint();
        let without = link_test_scene_no_edge();
        h.sync_descriptor(&without).unwrap();
        let without_edges = h.encoded_scene_hint();
        assert!(with_edges > without_edges, "overview cached draw must encode edges (with={with_edges}, without={without_edges})");
    }

    #[test]
    fn board_host_world_clip_changes_vector_encoding() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
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
        h.set_world_raster_tiling("none");
        let monolithic = h.encoded_scene_hint();
        h.set_world_raster_tiling("world-clip");
        let tiled = h.encoded_scene_hint();
        assert!(tiled >= monolithic);
    }

    #[test]
    fn board_host_silent_selection_keeps_cached_world_content_warm() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let gen_before = h.test_content_scene_generation();
        let neutral_hint = h.encoded_scene_hint();
        h.set_selection_ids_silent(&["a".into()]);
        assert_eq!(h.test_content_scene_generation(), gen_before, "selection chrome must paint via dynamic fill/stroke layers without rebuilding cached icons");
        assert_ne!(h.encoded_scene_hint(), neutral_hint, "selected node fill appears in overlay fill layer at normal LOD");
        assert_eq!(h.test_resolve_node_style_kind("a"), Some(BoardElementStyleKind::Selected));
    }

    #[test]
    fn board_host_selected_node_keeps_selected_style_when_hovered() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.sync_descriptor(&sample_scene()).unwrap();
        h.set_selection_ids(&["a".into()]);
        h.set_hovered_id_silent(Some("a".into()));
        assert_eq!(h.test_resolve_node_style_kind("a"), Some(BoardElementStyleKind::Selected), "committed selection chrome should beat hover while pointer is over the node");
        h.set_selection_ids(&[]);
        h.set_hovered_id_silent(Some("a".into()));
        assert_eq!(h.test_resolve_node_style_kind("a"), Some(BoardElementStyleKind::Hovered), "unselected nodes should still use hover chrome");
    }

    #[test]
    fn board_host_dragging_selected_node_keeps_selected_style_at_detail_lod() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.sync_descriptor(&sample_scene()).unwrap();
        h.set_selection_ids(&["a".into()]);
        let s = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        assert!(matches!(h.interaction, Interaction::DragNodes { .. }));
        assert_eq!(h.hovered_id.as_deref(), Some("a"));
        assert_eq!(h.test_resolve_node_style_kind("a"), Some(BoardElementStyleKind::Selected), "node drag should keep primary selected paint at detail LOD");
    }

    #[test]
    fn board_host_drag_emits_node_move() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
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
        let _ = h.drain_events_json();
        let w = Point::new(0.0, 0.0);
        let s = h.world_to_screen(w);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_move_screen(s.x + 50.0, s.y + 30.0, false, false, false);
        h.pointer_up_screen(s.x + 50.0, s.y + 30.0, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("nodeMove"));
    }

    #[test]
    fn board_host_compact_discrete_hit_selects_and_drags_node() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 0.5);
        let mut desc = sample_scene();
        desc.handles.clear();
        desc.edges.clear();
        desc.nodes.push(NodeDescJson {
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
        let _ = h.drain_events_json();
        assert_eq!(h.resolve_hit_world(Point::new(0.0, 0.0)).as_deref(), Some("a"));
        assert!(h.resolve_hit_world(Point::new(150.0, 0.0)).is_none());
        let s = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_move_screen(s.x + 50.0, s.y + 30.0, false, false, false);
        h.pointer_up_screen(s.x + 50.0, s.y + 30.0, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("nodeMove"), "compact discrete node hit should drag, got: {ev}");
    }

    #[test]
    fn board_host_minimap_bounded_drag_moves_selection_inside_union_bounds() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_automatic_lod(false);
        h.set_forced_draw_lod_label("minimap");
        h.set_camera(0.0, 0.0, 0.1);
        let mut desc = sample_scene();
        desc.handles.clear();
        desc.edges.clear();
        desc.nodes.push(NodeDescJson {
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
        h.set_selection_ids(&["a".into(), "b".into()]);
        let _ = h.drain_events_json();
        let gap = Point::new(150.0, 0.0);
        assert!(h.resolve_hit_world(gap).is_none());
        let s = h.world_to_screen(gap);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_move_screen(s.x + 50.0, s.y + 30.0, false, false, false);
        h.pointer_up_screen(s.x + 50.0, s.y + 30.0, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("nodeMove"), "expected bounded drag nodeMove, got: {ev}");
        let zoom = 0.1;
        let dx = 50.0 / zoom;
        let dy = 30.0 / zoom;
        let a = h.nodes.get("a").unwrap();
        let b = h.nodes.get("b").unwrap();
        assert!((a.x - dx).abs() < 1e-3 && (a.y - dy).abs() < 1e-3);
        assert!((b.x - (300.0 + dx)).abs() < 1e-3 && (b.y - dy).abs() < 1e-3);
    }

    #[test]
    fn board_host_overview_bounded_drag_moves_selection_inside_union_bounds() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_automatic_lod(false);
        h.set_forced_draw_lod_label("overview");
        set_overview_lod(&mut h);
        let mut desc = sample_scene();
        desc.handles.clear();
        desc.edges.clear();
        desc.nodes.push(NodeDescJson {
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
        h.set_selection_ids(&["a".into(), "b".into()]);
        let _ = h.drain_events_json();
        let gap = Point::new(150.0, 0.0);
        assert!(h.resolve_hit_world(gap).is_none());
        let s = h.world_to_screen(gap);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_move_screen(s.x + 40.0, s.y + 20.0, false, false, false);
        h.pointer_up_screen(s.x + 40.0, s.y + 20.0, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("nodeMove"), "expected overview bounded drag, got: {ev}");
    }

    #[test]
    fn board_host_detail_lod_resolves_direct_handle_hit() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
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
        assert_eq!(h.resolve_hit_world(probe).as_deref(), Some("a:h0"));
    }

    #[test]
    fn board_host_multi_select_drag_moves_each_selected_node() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
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
        h.set_selection_options("rectangle", "additive", true, true, true);
        h.set_selection_ids(&["a".into(), "b".into()]);
        let _ = h.drain_events_json();
        let w = Point::new(0.0, 0.0);
        let s = h.world_to_screen(w);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_move_screen(s.x + 10.0, s.y + 5.0, false, false, false);
        h.pointer_up_screen(s.x + 10.0, s.y + 5.0, false, false, false);
        let a = h.nodes.get("a").expect("node a");
        let b = h.nodes.get("b").expect("node b");
        assert!((a.x - 10.0).abs() < 1e-6);
        assert!((a.y - 5.0).abs() < 1e-6);
        assert!((b.x - 110.0).abs() < 1e-6);
        assert!((b.y - 5.0).abs() < 1e-6);
        let sorted: Vec<_> = h.selection.iter().cloned().collect();
        assert_eq!(sorted, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn board_host_selection_target_edges_skips_node_geometry() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_selection_options("rectangle", "invertive", false, true, false);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
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
        assert!(h.resolve_hit_world(inside_node_a).is_none());
        let on_edge = Point::new(150.0, 0.0);
        assert_eq!(h.resolve_hit_world(on_edge).as_deref(), Some("e1"));
    }

    #[test]
    fn board_host_additive_click_merges_edge_into_existing_selection() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_selection_options("rectangle", "additive", true, true, true);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
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
        h.set_selection_ids(&["a".into()]);
        let _ = h.drain_events_json();
        let on_edge = Point::new(150.0, 0.0);
        let s = h.world_to_screen(on_edge);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        let mut got: Vec<_> = h.selection.iter().cloned().collect();
        got.sort();
        assert_eq!(got, vec!["a".to_string(), "e1".to_string()]);
    }

    #[test]
    fn board_host_selection_change_does_not_bump_content_scene_generation() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let gen = h.test_content_scene_generation();
        let neutral_hint = h.encoded_scene_hint();
        h.set_selection_ids_silent(&["a".into()]);
        assert_eq!(h.test_content_scene_generation(), gen);
        let selected_hint = h.encoded_scene_hint();
        assert_ne!(selected_hint, neutral_hint);
        h.set_selection_ids_silent(&[]);
        assert_eq!(h.test_content_scene_generation(), gen);
        assert_eq!(h.encoded_scene_hint(), neutral_hint);
    }

    #[test]
    fn board_host_hover_change_does_not_bump_content_scene_generation() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let gen = h.test_content_scene_generation();
        let neutral_hint = h.encoded_scene_hint();
        h.set_hovered_id_silent(Some("a".into()));
        assert_eq!(h.test_content_scene_generation(), gen, "hover must paint via dynamic overlay chrome without rebuilding cached icons");
        let hovered_hint = h.encoded_scene_hint();
        assert_ne!(hovered_hint, neutral_hint);
        h.set_hovered_id_silent(None);
        assert_eq!(h.test_content_scene_generation(), gen);
        assert_eq!(h.encoded_scene_hint(), neutral_hint);
    }

    #[test]
    fn board_host_background_click_deselect_skips_preselect_events() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        let desc = sample_scene();
        h.sync_descriptor(&desc).unwrap();
        h.set_selection_ids(&["a".into(), "e1".into()]);
        let _ = h.drain_events_json();
        let away = Point::new(5000.0, 5000.0);
        let s = h.world_to_screen(away);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        assert!(!h.is_dragging_area_select());
        h.pointer_move_screen(s.x + 1.0, s.y, false, false, false);
        let mid = h.drain_events_json();
        assert!(!mid.contains("preselect"), "background click path must not emit preselect");
        assert!(h.preselect_removed.is_empty());
        assert!(h.selection_exit_highlight.is_empty());
        assert!(h.selection.contains("a"));
        h.pointer_up_screen(s.x, s.y, false, false, false);
        assert!(h.selection.is_empty());
        assert!(h.selection_exit_highlight.is_empty());
        let fin = h.drain_events_json();
        assert!(fin.contains("select"));
        assert!(!fin.contains("preselect"));
        assert!(fin.contains("\"exitHighlightIds\":[]"));
    }

    #[test]
    fn board_host_background_click_without_drag_clears_selection() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
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
        h.set_selection_ids(&["a".into(), "e1".into()]);
        let away = Point::new(5000.0, 5000.0);
        let s = h.world_to_screen(away);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_up_screen(s.x, s.y, false, false, false);
        assert!(h.selection.is_empty());
    }

    #[test]
    fn board_host_rectangle_area_select_includes_handles_with_nodes() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_selection_options("rectangle", "invertive", true, true, true);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
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
        let _ = h.drain_events_json();
        let w0 = Point::new(-90.0, -70.0);
        let w1 = Point::new(90.0, 90.0);
        let s0 = h.world_to_screen(w0);
        let s1 = h.world_to_screen(w1);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let mut got: Vec<_> = h.selection.iter().cloned().collect();
        got.sort();
        assert!(got.contains(&"a".to_string()));
        assert!(got.contains(&"a:h0".to_string()));
    }

    #[test]
    fn board_host_area_select_preselect_matches_selected_chrome() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
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
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let _ = h.drain_events_json();
        assert!(h.preselect_removed.is_empty());
        assert!(h.selection_exit_highlight.is_empty());
        let w_down = Point::new(350.0, -50.0);
        let w_mid = Point::new(270.0, 50.0);
        let w_end = Point::new(265.0, 48.0);
        let s_down = h.world_to_screen(w_down);
        h.pointer_down_screen(s_down.x, s_down.y, 0, false, false);
        assert!(!h.is_dragging_area_select());
        let _ = h.drain_events_json();
        let s_mid = h.world_to_screen(w_mid);
        let s_end = h.world_to_screen(w_end);
        h.pointer_move_screen(s_mid.x, s_mid.y, false, false, false);
        assert!(h.is_dragging_area_select());
        let _ = h.drain_events_json();
        assert!(h.preselect.contains("b"), "preview should include node b");
        assert!(h.preselect_removed.contains("a"));
        assert!(h.selection_exit_highlight.is_empty());
        assert!(!h.selection.contains("b"), "committed selection unchanged during preselect");
        let frozen = h.preselect_removed.clone();
        h.pointer_move_screen(s_end.x, s_end.y, false, false, false);
        let _ = h.drain_events_json();
        assert_eq!(frozen, h.preselect_removed);
        h.pointer_up_screen(s_end.x, s_end.y, false, false, false);
        let _ = h.drain_events_json();
        assert!(h.selection.contains("b"));
        assert!(!h.selection.contains("a"));
        assert!(h.preselect_removed.is_empty());
        assert!(h.selection_exit_highlight.is_empty());
    }

    #[test]
    fn board_host_area_select_from_empty_keeps_selection_until_commit() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
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
        h.set_selection_ids(&[]);
        let _ = h.drain_events_json();
        let w_down = Point::new(350.0, -50.0);
        let w_mid = Point::new(270.0, 50.0);
        let s_down = h.world_to_screen(w_down);
        let s_mid = h.world_to_screen(w_mid);
        h.pointer_down_screen(s_down.x, s_down.y, 0, false, false);
        h.pointer_move_screen(s_mid.x, s_mid.y, false, false, false);
        let _ = h.drain_events_json();
        assert!(h.is_dragging_area_select());
        assert!(h.preselect.contains("b"));
        assert!(h.preselect_removed.is_empty());
        assert!(h.selection.is_empty());
        h.pointer_up_screen(s_mid.x, s_mid.y, false, false, false);
        let _ = h.drain_events_json();
        assert!(h.selection.contains("b"));
        assert!(h.preselect.is_empty());
    }

    #[test]
    fn board_host_minimap_pointer_move_hovers_node_under_cursor() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 0.1);
        h.set_forced_draw_lod_label("minimap");
        h.sync_descriptor(&sample_scene()).unwrap();
        let center = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_move_screen(center.x, center.y, false, false, false);
        assert_eq!(h.hovered_id.as_deref(), Some("a"));
        let away = h.world_to_screen(Point::new(5000.0, 5000.0));
        h.pointer_move_screen(away.x, away.y, false, false, false);
        assert!(h.hovered_id.is_none());
    }

    #[test]
    fn board_host_minimap_preselect_matches_selected_chrome() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 0.1);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
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
        h.set_selection_ids(&["b".into()]);
        let _ = h.drain_events_json();
        let selected_hint = h.encoded_scene_hint();
        assert!(selected_hint > neutral_hint, "minimap selected chrome should add visible vector encoding over neutral state");
        h.set_selection_ids(&["a".into()]);
        let _ = h.drain_events_json();
        let w_down = Point::new(350.0, -50.0);
        let w_end = Point::new(265.0, 48.0);
        let s_down = h.world_to_screen(w_down);
        let s_end = h.world_to_screen(w_end);
        h.pointer_down_screen(s_down.x, s_down.y, 0, false, false);
        h.pointer_move_screen(s_end.x, s_end.y, false, false, false);
        assert!(h.is_dragging_area_select());
        assert!(h.preselect.contains("b"));
        h.set_selection_screen_preview(None);
        let preselect_hint = h.encoded_scene_hint();
        assert!(preselect_hint > neutral_hint, "minimap preselect should add visible selected chrome over neutral minimap rendering");
    }

    #[test]
    fn board_host_silent_preselect_applies_selected_chrome_without_area_drag() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 0.1);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
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
        assert!(!matches!(h.interaction, Interaction::Selection { .. }));
        h.set_preselect_state_silent(&["b".into()], &[]);
        assert!(h.nodes.get("b").is_some_and(|n| n.selected));
        assert!(h.nodes.get("a").is_some_and(|n| !n.selected));
        let preselect_hint = h.encoded_scene_hint();
        assert!(preselect_hint > neutral_hint, "silent minimap preselect should paint selected chrome without an active area-select interaction");
    }

    #[test]
    fn board_host_hover_tracks_visible_wires() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let mut desc = sample_scene();
        desc.edges.clear();
        desc.wires.push(WireDescJson { id: "w1".into(), source: "a:h0".into(), target: None, end_x: Some(220.0), end_y: Some(0.0), selected: None, style: None, wire_kind: None, user_data: None, visible: None, locked: None });
        h.sync_descriptor(&desc).unwrap();
        let source = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let curve = compute_edge_bezier_points(source, Point::new(220.0, 0.0), Point::new(0.0, 0.0), Point::new(220.0, 0.0));
        let probe = cubic_bezier_point(curve, 0.5);
        h.update_hover_from_world(probe);
        assert_eq!(h.hovered_id.as_deref(), Some("w1"));
    }
}
//#endregion 🧪️Tests
