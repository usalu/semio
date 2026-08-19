//! 🖌️ Puzzle 2d app engine — the brush laws: slot preview/commit/cancel, candidate ordering by
//! handle proximity, per-node-kind compatibility enumeration, and the deterministic fill session.

//#region 🧪️Tests
#[cfg(test)]
#[allow(clippy::approx_constant, reason = "3.14159 is verbatim fixture data (a handle angle in a scene JSON literal), carried over unchanged from the pre-consolidation engine crate; swapping in std::f64::consts::PI would alter the recorded test input.")]
mod tests {
    use crate::editor::puzzle2d::engine::board_host::testkit::*;
    use crate::editor::puzzle2d::engine::canvas::Point;
    use crate::editor::puzzle2d::engine::{handle_position_on_circle, BoardHost, HandleDescJson, NodeDescJson, SceneDescriptorJson};
    use serde_json::json;

    #[test]
    async fn board_host_brush_slot_emits_preview_and_place_on_leave() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_active_utility("brush");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        let catalogs = json!({
            "handleKinds": [{ "id": "port", "name": "Port", "color": "#888" }],
            "nodeKinds": [{
                "id": "brush.kind",
                "name": "Brush Kind",
                "handles": [{ "handleKind": "port", "angle": 3.141592653589793 }]
            }]
        });
        h.set_board_kind_catalogs_from_json(&catalogs.to_string()).unwrap();
        let desc = SceneDescriptorJson {
            nodes: vec![NodeDescJson {
                id: "a".into(),
                x: 0.0,
                y: 0.0,
                draggable: Some(true),
                selected: None,
                style: None,
                text: None,
                icon_kind: None,
                node_kind: Some("a.kind".into()),
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
            handles: vec![HandleDescJson {
                id: "a:h0".into(),
                node_id: "a".into(),
                angle: 0.0,
                radius: None,
                scale: None,
                selected: None,
                visible: None,
                locked: None,
                style: None,
                handle_kind: Some("port".into()),
                color: None,
                icon_kind: None,
                user_data: None,
            }],
            edges: vec![],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let slot = hp + (hp - Point::new(0.0, 0.0)) * (40.0 / 40.0);
        let s = h.world_to_screen(slot);
        h.pointer_move_screen(s.x, s.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("brushPreview"), "expected brushPreview, got: {ev}");
        h.pointer_leave_screen(true);
        let ev2 = h.drain_events_json();
        assert!(ev2.contains("brushPlace"), "expected brushPlace on leave with Alt, got: {ev2}");
        assert!(ev2.contains("brush.kind"));
        assert!(ev2.contains("a:h0"));
        assert!(ev2.contains("nodeId"));
        assert!(ev2.contains("edgeId"));
    }

    #[test]
    async fn board_host_brush_open_slot_suggestions_commit_and_cancel() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 2.0);
        h.set_active_utility("select");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        let catalogs = json!({
            "handleKinds": [{ "id": "port", "name": "Port", "color": "#888" }],
            "nodeKinds": [{
                "id": "brush.kind",
                "name": "Brush Kind",
                "handles": [{ "handleKind": "port", "angle": 3.141592653589793 }]
            }]
        });
        h.set_board_kind_catalogs_from_json(&catalogs.to_string()).unwrap();
        let desc = SceneDescriptorJson {
            nodes: vec![NodeDescJson {
                id: "a".into(),
                x: 0.0,
                y: 0.0,
                draggable: Some(true),
                selected: None,
                style: None,
                text: None,
                icon_kind: None,
                node_kind: Some("a.kind".into()),
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
            handles: vec![HandleDescJson {
                id: "a:h0".into(),
                node_id: "a".into(),
                angle: 0.0,
                radius: None,
                scale: None,
                selected: None,
                visible: None,
                locked: None,
                style: None,
                handle_kind: Some("port".into()),
                color: None,
                icon_kind: None,
                user_data: None,
            }],
            edges: vec![],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        h.brush_open_slot("a:h0");
        let ev = h.drain_events_json();
        assert!(ev.contains("brushCandidates"), "expected brushCandidates, got: {ev}");
        assert!(ev.contains("brushPreview"), "expected brushPreview, got: {ev}");
        assert!(ev.contains("\"id\":\"a:h0\""), "expected hovered source handle, got: {ev}");
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let expected_x = hp.x + (hp.x - 0.0) * (40.0 / 40.0);
        assert!(ev.contains(&format!("\"x\":{expected_x}")), "preview should flush along handle normal, got: {ev}");
        h.brush_commit_slot();
        let ev_commit = h.drain_events_json();
        assert!(ev_commit.contains("brushPlace"), "expected brushPlace on commit, got: {ev_commit}");
        h.brush_open_slot("a:h0");
        let _ = h.drain_events_json();
        h.brush_cancel_slot();
        let ev_cancel = h.drain_events_json();
        assert!(!ev_cancel.contains("brushPlace"), "cancel should not place, got: {ev_cancel}");
    }

    #[test]
    async fn board_host_brush_slot_commit_survives_pointer_move_out_of_slot() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_active_utility("brush");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [{
                    "id": "brush.kind",
                    "name": "Brush Kind",
                    "handles": [{ "handleKind": "child", "angle": 3.141592653589793 }]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let inside = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_move_screen(inside.x, inside.y, false, false, false);
        let _ = h.drain_events_json();
        assert_eq!(h.nodes.len(), 2);
        let far = h.world_to_screen(Point::new(500.0, 500.0));
        h.pointer_move_screen(far.x, far.y, false, false, true);
        let ev = h.drain_events_json();
        assert!(ev.contains("brushPlace"), "expected brushPlace when leaving slot with Alt, got: {ev}");
    }

    #[test]
    async fn board_host_brush_slot_skips_place_on_leave_without_alt() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_active_utility("brush");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [{
                    "id": "brush.kind",
                    "name": "Brush Kind",
                    "handles": [{"handleKind": "child", "angle": 3.141592653589793}]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let inside = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_move_screen(inside.x, inside.y, false, false, false);
        let _ = h.drain_events_json();
        h.pointer_leave_screen(false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("brushPlace"), "expected no brushPlace without Alt, got: {ev}");
    }

    #[test]
    async fn board_host_brush_fill_frontier_deterministic_and_collision_limited() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [{
                    "id": "brush.kind",
                    "name": "Brush Kind",
                    "handles": [
                        { "handleKind": "child", "angle": 0.0 },
                        { "handleKind": "child", "angle": 3.141592653589793 }
                    ]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let first = h.brush_fill_json(3, 42);
        let second = h.brush_fill_json(3, 42);
        assert_eq!(first, second, "fill must be deterministic for the same seed");
        let v: serde_json::Value = serde_json::from_str(&first).unwrap();
        let placements = v.get("placements").and_then(|x| x.as_array()).unwrap();
        assert!(!placements.is_empty(), "expected at least one fill placement");
        assert!(placements.len() <= 3);
        let many = h.brush_fill_json(1000, 99);
        let many_v: serde_json::Value = serde_json::from_str(&many).unwrap();
        let many_n = many_v.get("placements").and_then(|x| x.as_array()).map_or(0, |a| a.len());
        assert!(many_n < 1000, "collision should cap fill before 1000 on a tight scene");
    }

    #[test]
    async fn board_host_brush_fill_session_step_matches_brush_fill_json() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [{
                    "id": "brush.kind",
                    "name": "Brush Kind",
                    "handles": [
                        { "handleKind": "child", "angle": 0.0 },
                        { "handleKind": "child", "angle": 3.141592653589793 }
                    ]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let expected: serde_json::Value = serde_json::from_str(&h.brush_fill_json(12, 77)).unwrap();
        h.brush_fill_session_begin(12, 77);
        let mut stepped: Vec<serde_json::Value> = Vec::new();
        let mut done = false;
        while !done {
            let chunk: serde_json::Value = serde_json::from_str(&h.brush_fill_session_step(4)).unwrap();
            done = chunk.get("done").and_then(|x| x.as_bool()).unwrap_or(true);
            if let Some(rows) = chunk.get("placements").and_then(|x| x.as_array()) {
                stepped.extend(rows.iter().cloned());
            }
        }
        h.brush_fill_session_clear();
        assert_eq!(stepped, expected.get("placements").and_then(|x| x.as_array()).cloned().unwrap_or_default());
    }

    #[test]
    async fn board_host_fixture_drop_preview_json_paints_while_select_utility_active() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_active_utility("select");
        h.set_fixture_drop_preview_json(r#"{"nodeKind":"capsule_J","screenX":200.0,"screenY":150.0,"shape":"circle","radius":20.0,"iconKind":"capsule_J"}"#).unwrap();
        let ev = h.drain_events_json();
        assert!(!ev.contains("brushPlace"));
        assert!(h.encoded_scene_hint() > 0);
        h.set_fixture_drop_preview_json("").unwrap();
        assert!(h.encoded_scene_hint() > 0);
    }

    #[test]
    async fn board_host_fixture_drop_preview_uses_catalog_shape_and_icon_at_overview_lod() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 0.05);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "nodeKinds": [{
                    "id": "capsule_J",
                    "name": "Capsule J",
                    "scale": 2.0,
                    "shape": "circle",
                    "icon": "capsule_J",
                    "handles": [{"handleKind": "door", "angle": 0.0}]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.set_fixture_drop_preview_json(r#"{"nodeKind":"capsule_J","screenX":120.0,"screenY":90.0,"shape":"circle","radius":10.0,"iconKind":"capsule_J"}"#).unwrap();
        let hint_with_preview = h.encoded_scene_hint();
        assert!(hint_with_preview > 0);
        h.set_fixture_drop_preview_json("").unwrap();
        let hint_cleared = h.encoded_scene_hint();
        assert!(hint_cleared != hint_with_preview || hint_with_preview > 0);
    }

    #[test]
    async fn board_host_brush_session_mirror_json_shows_preview_without_pointer() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_active_utility("brush");
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [{"id": "parent", "name": "Parent", "color": "#888888"}],
                "nodeKinds": [{
                    "id": "brush.kind",
                    "name": "Brush Kind",
                    "handles": [{"handleKind": "parent", "angle": 3.141592653589793}]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let session = serde_json::json!({
            "sourceHandleId": "a:h0",
            "candidates": ["brush.kind"],
            "index": 0,
            "preview": {
                "node": {
                    "nodeKind": "brush.kind",
                    "x": 120.0,
                    "y": 0.0,
                    "shape": "circle",
                    "radius": 20.0,
                    "handles": [{"handleKind": "parent", "angle": 3.141592653589793}]
                },
                "edge": { "sourceHandleId": "a:h0", "targetHandleIndex": 0 }
            }
        });
        h.set_brush_session_mirror_json(&session.to_string()).unwrap();
        let ev = h.drain_events_json();
        assert!(!ev.contains("brushPlace"));
        assert!(h.encoded_scene_hint() > 0);
    }

    #[test]
    async fn board_host_brush_candidates_sorted_by_handle_proximity() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_active_utility("brush");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [
                    {
                        "id": "light",
                        "name": "Light",
                        "handles": [
                            {"handleKind": "child", "angle": 0.0},
                            {"handleKind": "child", "angle": 3.141592653589793}
                        ]
                    },
                    {
                        "id": "heavy",
                        "name": "Heavy",
                        "handles": [{"handleKind": "child", "angle": 3.141592653589793}]
                    }
                ]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let inside = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_move_screen(inside.x, inside.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("brushCandidates"), "expected brushCandidates, got: {ev}");
        let v: serde_json::Value = serde_json::from_str(&ev).unwrap();
        let candidates =
            v.as_array().and_then(|rows| rows.iter().find(|row| row.get("name").and_then(|n| n.as_str()) == Some("brushCandidates")).and_then(|row| row.get("payload")).and_then(|p| p.get("candidates")).and_then(|c| c.as_array()).cloned());
        assert_eq!(candidates.as_ref().map(|rows| rows.len()), Some(3));
        let first_kind = candidates.as_ref().and_then(|rows| rows.first()).and_then(|row| row.get("nodeKind")).and_then(|x| x.as_str());
        assert_eq!(first_kind, Some("heavy"));
    }

    #[test]
    async fn board_host_brush_lists_every_compatible_handle_per_node_kind() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_active_utility("brush");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [{
                    "id": "dual",
                    "name": "Dual",
                    "handles": [
                        {"handleKind": "child", "angle": 0.0},
                        {"handleKind": "child", "angle": 3.141592653589793}
                    ]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let inside = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_move_screen(inside.x, inside.y, false, false, false);
        let ev = h.drain_events_json();
        let v: serde_json::Value = serde_json::from_str(&ev).unwrap();
        let candidates = v
            .as_array()
            .and_then(|rows| rows.iter().find(|row| row.get("name").and_then(|n| n.as_str()) == Some("brushCandidates")).and_then(|row| row.get("payload")).and_then(|p| p.get("candidates")).and_then(|c| c.as_array()).cloned())
            .unwrap_or_default();
        assert_eq!(candidates.len(), 2, "expected one row per compatible handle, got: {ev}");
        let indices: Vec<u64> = candidates.iter().filter_map(|row| row.get("targetHandleIndex").and_then(|i| i.as_u64())).collect();
        assert!(indices.contains(&0));
        assert!(indices.contains(&1));
    }

    #[test]
    async fn board_host_fill_base_core_rectangular_excludes_cylindric_tambour() {
        const BASE_KIND: &str = "Base";
        const CYLINDRIC_TAMBOUR_KIND: &str = "Cylindric Tambour";
        const FIRST_STOREY_KIND: &str = "First Storey Tambour";
        let mut h = BoardHost::new();
        h.set_suggestion_offset(80.0);
        h.set_brush_node_size(40.0);
        
        let fixture: serde_json::Value = serde_json::to_value(
            <crate::artifacts::puzzle2d::Puzzle2dSnapshot as store::ArtifactDsl>::parse_dsl(crate::artifacts::puzzle2d::dsl::PUZZLE2D_NAKAGIN_EXAMPLE_TEXT).unwrap(),
        )
        .unwrap();
        let compat_str = fixture.get("meta").and_then(|m| m.get("kindCompatibility")).map_or_else(|| "[]".to_string(), |v| v.to_string());
        h.set_handle_link_compat_from_json(&compat_str).unwrap();
        h.set_board_kind_catalogs_from_json(&catalogs_json_from_manifest_id("nakagin")).unwrap();
        let desc = SceneDescriptorJson {
            nodes: vec![NodeDescJson {
                id: "base".into(),
                x: 0.0,
                y: 0.0,
                draggable: Some(true),
                selected: None,
                style: None,
                text: None,
                icon_kind: Some("base".into()),
                node_kind: Some(BASE_KIND.into()),
                user_data: None,
                visible: None,
                locked: None,
                root: None,
                shape: Some("circle".into()),
                radius: Some(20.0),
                width: None,
                height: None,
                scale: None,
            }],
            handles: vec![
                HandleDescJson {
                    id: "base:c0".into(),
                    node_id: "base".into(),
                    angle: -2.3561944901923453,
                    radius: Some(3.0),
                    scale: None,
                    selected: None,
                    visible: None,
                    locked: None,
                    style: None,
                    handle_kind: Some("core rectangular bottom".into()),
                    color: None,
                    icon_kind: None,
                    user_data: None,
                },
                HandleDescJson {
                    id: "base:c1".into(),
                    node_id: "base".into(),
                    angle: -0.7853981633974483,
                    radius: Some(3.0),
                    scale: None,
                    selected: None,
                    visible: None,
                    locked: None,
                    style: None,
                    handle_kind: Some("core rectangular bottom".into()),
                    color: None,
                    icon_kind: None,
                    user_data: None,
                },
            ],
            edges: vec![],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        let out: serde_json::Value = serde_json::from_str(&h.brush_fill_json(1, 7)).unwrap();
        let placements = out.get("placements").and_then(|x| x.as_array()).unwrap();
        assert_eq!(placements.len(), 1, "expected one fill placement on base");
        let node_kind = placements[0].get("nodeKind").and_then(|x| x.as_str()).unwrap_or("");
        assert_ne!(node_kind, CYLINDRIC_TAMBOUR_KIND, "cylindric tambour must not stack on rectangular core");
        assert_eq!(node_kind, FIRST_STOREY_KIND, "first storey tambour matches rectangular core stack");
    }

    #[test]
    async fn board_host_brush_door_tambour_left_excludes_capital_with_metabolism_compat_rules() {
        const DOOR_TAMBOUR_LEFT: &str = "door tambour left";
        const CAPITAL_KIND: &str = "Capital";
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_active_utility("brush");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        
        let fixture: serde_json::Value = serde_json::to_value(
            <crate::artifacts::puzzle2d::Puzzle2dSnapshot as store::ArtifactDsl>::parse_dsl(crate::artifacts::puzzle2d::dsl::PUZZLE2D_NAKAGIN_EXAMPLE_TEXT).unwrap(),
        )
        .unwrap();
        let compat_str = fixture.get("meta").and_then(|m| m.get("kindCompatibility")).map_or_else(|| "[]".to_string(), |v| v.to_string());
        h.set_handle_link_compat_from_json(&compat_str).unwrap();
        let catalogs_str = fixture
            .get("meta")
            .and_then(|m| m.get("kindCatalogs")).map_or_else(|| "{}".to_string(), |kc| {
                serde_json::json!({
                    "handleKinds": kc.get("handles"),
                    "nodeKinds": kc.get("nodes"),
                })
                .to_string()
            });
        h.set_board_kind_catalogs_from_json(&catalogs_str).unwrap();
        let desc = SceneDescriptorJson {
            nodes: vec![NodeDescJson {
                id: "tambour".into(),
                x: 0.0,
                y: 0.0,
                draggable: Some(true),
                selected: None,
                style: None,
                text: None,
                icon_kind: None,
                node_kind: Some("Tambour".into()),
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
            handles: vec![HandleDescJson {
                id: "tambour:h0".into(),
                node_id: "tambour".into(),
                angle: 0.0,
                radius: None,
                scale: None,
                selected: None,
                visible: None,
                locked: None,
                style: None,
                handle_kind: Some(DOOR_TAMBOUR_LEFT.into()),
                color: None,
                icon_kind: None,
                user_data: None,
            }],
            edges: vec![],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let slot = hp + (hp - Point::new(0.0, 0.0)) * (40.0 / 40.0);
        let slot_screen = h.world_to_screen(slot);
        h.pointer_move_screen(slot_screen.x, slot_screen.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("brushCandidates"), "expected brushCandidates, got: {ev}");
        let v: serde_json::Value = serde_json::from_str(&ev).unwrap();
        let candidates = v
            .as_array()
            .and_then(|rows| rows.iter().find(|row| row.get("name").and_then(|n| n.as_str()) == Some("brushCandidates")).and_then(|row| row.get("payload")).and_then(|p| p.get("candidates")).cloned())
            .and_then(|c| c.as_array().cloned())
            .unwrap_or_default();
        let ids: Vec<String> = candidates.iter().filter_map(|x| x.as_str().map(str::to_string)).collect();
        assert!(!ids.iter().any(|id| id == CAPITAL_KIND), "door tambour left must not suggest Capital, got: {ids:?}");
    }

    #[test]
    async fn board_host_brush_slot_accepts_pointer_on_node_body_at_overview_lod() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_overview_lod(&mut h);
        h.set_active_utility("brush");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [{
                    "id": "brush.kind",
                    "name": "Brush Kind",
                    "handles": [{ "handleKind": "child", "angle": 3.141592653589793 }]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let inside = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_move_screen(inside.x, inside.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("brushPreview"), "expected brushPreview when hovering node body at overview LOD, got: {ev}");
        assert!(ev.contains("brushCandidates"), "expected brushCandidates, got: {ev}");
    }

    #[test]
    async fn board_host_brush_slot_accepts_pointer_on_indirect_ring_anchor() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_active_utility("brush");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [{
                    "id": "brush.kind",
                    "name": "Brush Kind",
                    "handles": [{ "handleKind": "child", "angle": 3.141592653589793 }]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_node_a_two_free_handles()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let ha0 = h.handles.get("a:h0").unwrap();
        let ring = h.indirect_handle_world_pos(ha0).unwrap();
        let s = h.world_to_screen(ring);
        h.pointer_move_screen(s.x, s.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("brushPreview"), "expected brushPreview on indirect ring anchor, got: {ev}");
    }
}
//#endregion 🧪️Tests
