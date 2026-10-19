    /// 🚪️ Fill base core, tambour and pointer slot laws: the rectangular base core, the door tambour exclusion and the pointer slot acceptance on the node body and the indirect ring anchor.

    #[test]
    fn board_host_fill_base_core_rectangular_excludes_cylindric_tambour() {
        const BASE_KIND: &str = "Base";
        const CYLINDRIC_TAMBOUR_KIND: &str = "Cylindric Tambour";
        const FIRST_STOREY_KIND: &str = "First Storey Tambour";
        let mut h = BoardHost::new();
        h.set_suggestion_offset(80.0);
        h.set_brush_node_size(40.0);

        let snapshot: semio_framework_pack_json::Value = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&(<crate::Puzzle2dSnapshot as store::os_store::serialization::ArtifactDsl>::parse_dsl(crate::standards::v1::subsets::any::io::text::snapshot::PUZZLE2D_NAKAGIN_EXAMPLE_TEXT).unwrap())));
        let compat_str = snapshot.get("meta").and_then(|m| m.get("kindCompatibility")).map_or_else(|| "[]".to_string(), |v| v.to_string());
        h.style.set_handle_link_compat_from_json(&compat_str).unwrap();
        h.style.set_board_kind_catalogs_from_json(&catalogs_json_from_manifest_id("nakagin")).unwrap();
        let desc = SceneDescriptor {
            nodes: vec![NodeDescriptor {
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
            }].into(),
            handles: vec![
                HandleDescriptor {
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
                HandleDescriptor {
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
            ].into(),
            edges: vec![].into(),
            wires: vec![].into(),
            regions: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        let operation = Operation::new(semio_framework_job::OperationId(7), semio_framework_job::RevisionId(5), semio_framework_job::Generation(1), 7);
        let (placements, _, _) = run_fill_job(&h, 1, operation, 1);
        assert_eq!(placements.len(), 1, "expected one fill placement on base");
        let node_kind = placements[0].node_kind.as_str();
        assert_ne!(node_kind, CYLINDRIC_TAMBOUR_KIND, "cylindric tambour must not stack on rectangular core");
        assert_eq!(node_kind, FIRST_STOREY_KIND, "first storey tambour matches rectangular core stack");
    }

    #[test]
    fn board_host_brush_door_tambour_left_excludes_capital_with_metabolism_compat_rules() {
        const DOOR_TAMBOUR_LEFT: &str = "door tambour left";
        const CAPITAL_KIND: &str = "Capital";
        let mut h = BoardHost::new();
        h.scene.set_size(800, 600, 1.0);
        board_set_camera(&mut h.scene, &mut h.outbox, 0.0, 0.0, 1.0);
        h.set_active_utility("brush");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);

        let snapshot: semio_framework_pack_json::Value = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&(<crate::Puzzle2dSnapshot as store::os_store::serialization::ArtifactDsl>::parse_dsl(crate::standards::v1::subsets::any::io::text::snapshot::PUZZLE2D_NAKAGIN_EXAMPLE_TEXT).unwrap())));
        let compat_str = snapshot.get("meta").and_then(|m| m.get("kindCompatibility")).map_or_else(|| "[]".to_string(), |v| v.to_string());
        h.style.set_handle_link_compat_from_json(&compat_str).unwrap();
        let catalogs_str = snapshot.get("meta").and_then(|m| m.get("kindCatalogs")).map_or_else(
            || "{}".to_string(),
            |kc| {
                semio_framework_pack_json::json!({
                    "handleKinds": kc.get("handles").cloned(),
                    "nodeKinds": kc.get("nodes").cloned(),
                })
                .to_string()
            },
        );
        h.style.set_board_kind_catalogs_from_json(&catalogs_str).unwrap();
        let desc = SceneDescriptor {
            nodes: vec![NodeDescriptor {
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
            }].into(),
            handles: vec![HandleDescriptor {
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
            }].into(),
            edges: vec![].into(),
            wires: vec![].into(),
            regions: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        let _ = h.outbox.drain_events_json();
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let slot = hp + (hp - Point::new(0.0, 0.0)) * (40.0 / 40.0);
        let slot_screen = h.scene.world_to_screen(slot);
        h.pointer_move_screen(slot_screen.x, slot_screen.y, false, false, false);
        let ev = h.outbox.drain_events_json();
        assert!(ev.contains("brushCandidates"), "expected brushCandidates, got: {ev}");
        let v: semio_framework_pack_json::Value = semio_framework_pack_json::from_json_str(&ev, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let candidates = v
            .as_array()
            .and_then(|rows| rows.iter().find(|row| row.get("name").and_then(|n| n.as_str()) == Some("brushCandidates")).and_then(|row| row.get("payload")).and_then(|p| p.get("candidates")).cloned())
            .and_then(|c| c.as_array().cloned())
            .unwrap_or_default();
        let ids: Vec<String> = candidates.iter().filter_map(|x| x.as_str().map(str::to_string)).collect();
        assert!(!ids.iter().any(|id| id == CAPITAL_KIND), "door tambour left must not suggest Capital, got: {ids:?}");
    }

    #[test]
    fn board_host_brush_slot_accepts_pointer_on_node_body_at_overview_lod() {
        let mut h = BoardHost::new();
        h.scene.set_size(800, 600, 1.0);
        set_overview_lod(&mut h);
        h.set_active_utility("brush");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        h.style.set_board_kind_catalogs_from_json(
            &semio_framework_pack_json::json!({
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
        let _ = h.outbox.drain_events_json();
        let inside = h.scene.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_move_screen(inside.x, inside.y, false, false, false);
        let ev = h.outbox.drain_events_json();
        assert!(ev.contains("brushPreview"), "expected brushPreview when hovering node body at overview LOD, got: {ev}");
        assert!(ev.contains("brushCandidates"), "expected brushCandidates, got: {ev}");
    }

    #[test]
    fn board_host_brush_slot_accepts_pointer_on_indirect_ring_anchor() {
        let mut h = BoardHost::new();
        h.scene.set_size(800, 600, 1.0);
        board_set_camera(&mut h.scene, &mut h.outbox, 0.0, 0.0, 1.0);
        h.set_active_utility("brush");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        h.style.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.style.set_board_kind_catalogs_from_json(
            &semio_framework_pack_json::json!({
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
        let _ = h.outbox.drain_events_json();
        set_selection_ids(&mut h.scene, &mut h.focus, &mut h.outbox, &["a".into()]);
        let ha0 = h.scene.handles.get("a:h0").unwrap();
        let ring = indirect_handle_world_pos(&h.scene, &h.style, ha0).unwrap();
        let s = h.scene.world_to_screen(ring);
        h.pointer_move_screen(s.x, s.y, false, false, false);
        let ev = h.outbox.drain_events_json();
        assert!(ev.contains("brushPreview"), "expected brushPreview on indirect ring anchor, got: {ev}");
    }
