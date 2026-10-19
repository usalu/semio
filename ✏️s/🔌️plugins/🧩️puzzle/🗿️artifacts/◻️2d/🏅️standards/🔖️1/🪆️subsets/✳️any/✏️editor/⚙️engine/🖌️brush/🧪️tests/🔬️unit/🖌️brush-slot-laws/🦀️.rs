    /// 🖌️ Brush slot laws: the brush slot preview, place on leave, fastened handle refusal, suggestion commit and cancel and the pointer move out of the slot.

    #[test]
    fn board_host_brush_slot_emits_preview_and_place_on_leave() {
        let mut h = BoardHost::new();
        h.scene.set_size(800, 600, 1.0);
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
        h.style.set_board_kind_catalogs_from_json(&catalogs.to_string()).unwrap();
        let desc = SceneDescriptor {
            nodes: vec![NodeDescriptor {
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
            }].into(),
            handles: vec![HandleDescriptor {
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
            }].into(),
            edges: vec![].into(),
            wires: vec![].into(),
            regions: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let slot = hp + (hp - Point::new(0.0, 0.0)) * (40.0 / 40.0);
        let s = h.scene.world_to_screen(slot);
        h.pointer_move_screen(s.x, s.y, false, false, false);
        let ev = h.outbox.drain_events_json();
        assert!(ev.contains("brushPreview"), "expected brushPreview, got: {ev}");
        h.pointer_leave_screen(true);
        let ev2 = h.outbox.drain_events_json();
        assert!(ev2.contains("brushPlace"), "expected brushPlace on leave with Alt, got: {ev2}");
        assert!(ev2.contains("brush.kind"));
        assert!(ev2.contains("a:h0"));
        assert!(ev2.contains("nodeId"));
        assert!(ev2.contains("edgeId"));
    }

    /// 🔗️ LAW: "open handle" is ONE definition across all three slot entry points. The pointer path
    /// already skipped a fastened handle; `brush_open_slot` (the suggestions popup) must resolve an
    /// EMPTY candidate page on one too, so a fully fastened document refuses politely instead of
    /// placing a second node on an existing fastening.
    #[test]
    fn board_host_brush_open_slot_refuses_a_fastened_handle() {
        let mut h = BoardHost::new();
        h.scene.set_size(800, 600, 1.0);
        board_set_camera(&mut h.scene, &mut h.outbox, 0.0, 0.0, 2.0);
        h.set_active_utility("select");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        let catalogs = json!({
            "handleKinds": [{ "id": "port", "name": "Port", "color": "#888" }],
            "nodeKinds": [{ "id": "brush.kind", "name": "Brush Kind", "handles": [{ "handleKind": "port", "angle": 3.141592653589793 }] }]
        });
        h.style.set_board_kind_catalogs_from_json(&catalogs.to_string()).unwrap();
        let snapshot = json!({
            "schema": "board.ports.directed.v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "a", "nodeKind": "a.kind", "shape": "circle", "radius": 40.0, "x": 0.0, "y": 0.0, "handles": [
                    { "id": "a:h0", "handleKind": "port", "angle": 0.0 },
                    { "id": "a:h1", "handleKind": "port", "angle": 3.141592653589793 }
                ] },
                { "id": "b", "nodeKind": "a.kind", "shape": "circle", "radius": 40.0, "x": 200.0, "y": 0.0, "handles": [
                    { "id": "b:h0", "handleKind": "port", "angle": 3.141592653589793 }
                ] }
            ],
            "edges": [{ "id": "e0", "edgeKind": "link", "source": "a:h0", "target": "b:h0" }]
        });
        assert!({let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(64*1024*1024,&mut accepted);semio_framework_os_infinite::board::io::text::snapshot_assembly::load_board_snapshot_json(&mut h.scene, &mut h.focus, &mut h.style, &mut h.outbox,&snapshot.to_string(),&mut control)}, "the two-node snapshot must parse");
        let _ = h.outbox.drain_events_json();

        h.brush_open_slot("a:h0");
        let fastened = h.outbox.drain_events_json();
        assert!(fastened.contains("\"candidates\":[]"), "a fastened handle must resolve an empty candidate page, got: {fastened}");
        h.brush_commit_slot();
        assert!(!h.outbox.drain_events_json().contains("brushPlace"), "an empty page must place nothing");

        h.brush_open_slot("a:h1");
        let free = h.outbox.drain_events_json();
        assert!(free.contains("brush.kind"), "a free handle must still resolve its compatible kinds, got: {free}");
        h.brush_cancel_slot();
    }

    #[test]
    fn board_host_brush_open_slot_suggestions_commit_and_cancel() {
        let mut h = BoardHost::new();
        h.scene.set_size(800, 600, 1.0);
        board_set_camera(&mut h.scene, &mut h.outbox, 0.0, 0.0, 2.0);
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
        h.style.set_board_kind_catalogs_from_json(&catalogs.to_string()).unwrap();
        let desc = SceneDescriptor {
            nodes: vec![NodeDescriptor {
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
            }].into(),
            handles: vec![HandleDescriptor {
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
            }].into(),
            edges: vec![].into(),
            wires: vec![].into(),
            regions: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        h.brush_open_slot("a:h0");
        let ev = h.outbox.drain_events_json();
        assert!(ev.contains("brushCandidates"), "expected brushCandidates, got: {ev}");
        assert!(ev.contains("brushPreview"), "expected brushPreview, got: {ev}");
        assert!(ev.contains("\"id\":\"a:h0\""), "expected hovered source handle, got: {ev}");
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let expected_x = hp.x + (hp.x - 0.0) * (40.0 / 40.0);
        assert!(ev.contains(&format!("\"x\":{expected_x}")), "preview should flush along handle normal, got: {ev}");
        h.brush_commit_slot();
        let ev_commit = h.outbox.drain_events_json();
        assert!(ev_commit.contains("brushPlace"), "expected brushPlace on commit, got: {ev_commit}");
        h.brush_open_slot("a:h0");
        let _ = h.outbox.drain_events_json();
        h.brush_cancel_slot();
        let ev_cancel = h.outbox.drain_events_json();
        assert!(!ev_cancel.contains("brushPlace"), "cancel should not place, got: {ev_cancel}");
    }

    #[test]
    fn board_host_brush_slot_commit_survives_pointer_move_out_of_slot() {
        let mut h = BoardHost::new();
        h.scene.set_size(800, 600, 1.0);
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
        let _ = h.outbox.drain_events_json();
        assert_eq!(h.scene.nodes.len(), 2);
        let far = h.scene.world_to_screen(Point::new(500.0, 500.0));
        h.pointer_move_screen(far.x, far.y, false, false, true);
        let ev = h.outbox.drain_events_json();
        assert!(ev.contains("brushPlace"), "expected brushPlace when leaving slot with Alt, got: {ev}");
    }

    #[test]
    fn board_host_brush_slot_skips_place_on_leave_without_alt() {
        let mut h = BoardHost::new();
        h.scene.set_size(800, 600, 1.0);
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
                    "handles": [{"handleKind": "child", "angle": 3.141592653589793}]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.outbox.drain_events_json();
        let inside = h.scene.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_move_screen(inside.x, inside.y, false, false, false);
        let _ = h.outbox.drain_events_json();
        h.pointer_leave_screen(false);
        let ev = h.outbox.drain_events_json();
        assert!(!ev.contains("brushPlace"), "expected no brushPlace without Alt, got: {ev}");
    }

    /// 🔏️ A puzzle board mints its fill placements under the plugin's `puzzle2d.fill.` naming; the same scene on a
    /// bare board mints the neutral `board.fill.` ids, so the prefix is the plugin's declaration, not the board's.
    #[test]
    fn puzzle_board_naming_owns_fill_placement_ids() {
        use crate::editor::puzzle2d::engine::board_host::PUZZLE_2D_BOARD_NAMING;
        let operation = || Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(9), semio_framework_job::Generation(1), 42);
        let mut puzzle = frontier_fill_host();
        puzzle.style.set_naming(&PUZZLE_2D_BOARD_NAMING);
        let (puzzle_placements, _, _) = run_fill_job(&puzzle, 1, operation(), 1);
        let (neutral_placements, _, _) = run_fill_job(&frontier_fill_host(), 1, operation(), 1);
        assert!(!puzzle_placements.is_empty() && !neutral_placements.is_empty(), "the frontier scene admits a fill placement");
        assert!(puzzle_placements.iter().all(|placement| placement.target.starts_with("puzzle2d.fill.")), "{puzzle_placements:?}");
        assert!(neutral_placements.iter().all(|placement| placement.target.starts_with("board.fill.")), "{neutral_placements:?}");
    }
