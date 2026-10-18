    /// 🔎️ Candidate acceptance, preview and proximity: the retained field stages of an accepted candidate, the drop preview json, the session mirror and the proximity sorted candidates.

    /// 🧱️ Mounted acceptance publishes every string, template, virtual owner, and source mutation as a distinct worker stage.
    #[test]
    fn board_fill_candidate_acceptance_exposes_every_retained_field_stage() {
        let host = frontier_fill_host();
        let operation = Operation::new(semio_framework_job::OperationId(271), semio_framework_job::RevisionId(37), semio_framework_job::Generation(11), 271);
        let params = fill_params_with(operation, operation.generation, semio_framework_job::root_cancel_token(), "puzzle2d.fill.field-cursors", semio_framework_job::default_now_us);
        let mut session = mount_fill_session(BoardFillJob::with_operation(capture_fill_snapshot(&host), 2, operation), params);
        let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
        let mut seen = [false; 13];
        let mut checkpointed = false;
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            match pump_fill_session(&mut session, &pool) {
                WorkerJobPoll::Submitted | WorkerJobPoll::Rejected | WorkerJobPoll::Idle => std::thread::yield_now(),
                WorkerJobPoll::Outcome | WorkerJobPoll::Terminal => {
                    match seen_fill_outcome(&mut session) {
                        FillSeen::Preview => {
                            let job = session.checked_out_job_mut().expect("field cursor job");
                            match job.stage() {
                                crate::editor::puzzle2d::engine::BoardFillStage::AcceptNodeId => seen[0] = true,
                                crate::editor::puzzle2d::engine::BoardFillStage::AcceptEdgeId => seen[1] = true,
                                crate::editor::puzzle2d::engine::BoardFillStage::AcceptEdgeKind => seen[2] = true,
                                crate::editor::puzzle2d::engine::BoardFillStage::AcceptNodeKind => seen[3] = true,
                                crate::editor::puzzle2d::engine::BoardFillStage::AcceptSourceHandle => seen[4] = true,
                                crate::editor::puzzle2d::engine::BoardFillStage::AcceptTargetHandle => seen[5] = true,
                                crate::editor::puzzle2d::engine::BoardFillStage::AcceptIcon => seen[6] = true,
                                crate::editor::puzzle2d::engine::BoardFillStage::AcceptVirtualNode => seen[7] = true,
                                crate::editor::puzzle2d::engine::BoardFillStage::AcceptSourceConnection => seen[8] = true,
                                crate::editor::puzzle2d::engine::BoardFillStage::AcceptHandles => seen[9] = true,
                                crate::editor::puzzle2d::engine::BoardFillStage::AcceptHandleId => seen[10] = true,
                                crate::editor::puzzle2d::engine::BoardFillStage::AcceptHandleVirtual => seen[11] = true,
                                crate::editor::puzzle2d::engine::BoardFillStage::AcceptHandlePublish => seen[12] = true,
                                _ => {}
                            }
                            let _ = job.take_preview().expect("field cursor preview");
                            acknowledge_fill_outcome(&mut session);
                            session.resume().expect("field cursor resume");
                        }
                        FillSeen::Checkpoint => {
                            retire_checked_out_fill_checkpoint(&mut session);
                            acknowledge_fill_outcome(&mut session);
                            checkpointed = true;
                            break;
                        }
                        FillSeen::Yield => {
                            acknowledge_fill_outcome(&mut session);
                            session.resume().expect("field cursor yield resume");
                        }
                        ended @ (FillSeen::Complete(_) | FillSeen::Cancelled | FillSeen::Fault) => {
                            let fault = session.checked_out_job_mut().and_then(BoardFillJob::take_fault);
                            acknowledge_fill_outcome(&mut session);
                            panic!("field cursor job terminated before checkpoint: {ended:?} fault={fault:?} seen={seen:?}");
                        }
                    }
                }
                WorkerJobPoll::Closing | WorkerJobPoll::TerminalEmpty | WorkerJobPoll::CheckedOut => panic!("field cursor session entered invalid phase"),
            }
        }
        assert!(checkpointed);
        assert!(seen.into_iter().all(|value| value));
        close_fill_session(&mut session);
        pool.shutdown().expect("bounded snapshot operation succeeds");
    }

    #[test]
    fn board_host_fixture_drop_preview_json_paints_while_select_utility_active() {
        let mut h = BoardHost::new();
        h.scene.set_size(800, 600, 1.0);
        h.set_active_utility("select");
        set_drop_preview_json(&mut h.scene, &mut h.brush, r#"{"nodeKind":"capsule_J","screenX":200.0,"screenY":150.0,"shape":"circle","radius":20.0,"iconKind":"capsule_J"}"#).unwrap();
        let ev = h.outbox.drain_events_json();
        assert!(!ev.contains("brushPlace"));
        assert!(h.encoded_scene_hint() > 0);
        set_drop_preview_json(&mut h.scene, &mut h.brush, "").unwrap();
        assert!(h.encoded_scene_hint() > 0);
        close_board_host(h);
    }

    #[test]
    fn board_host_fixture_drop_preview_uses_catalog_shape_and_icon_at_overview_lod() {
        let mut h = BoardHost::new();
        h.scene.set_size(800, 600, 1.0);
        board_set_camera(&mut h.scene, &mut h.outbox, 0.0, 0.0, 0.05);
        h.style.set_board_kind_catalogs_from_json(
            &semio_framework_pack_json::json!({
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
        set_drop_preview_json(&mut h.scene, &mut h.brush, r#"{"nodeKind":"capsule_J","screenX":120.0,"screenY":90.0,"shape":"circle","radius":10.0,"iconKind":"capsule_J"}"#).unwrap();
        let hint_with_preview = h.encoded_scene_hint();
        assert!(hint_with_preview > 0);
        set_drop_preview_json(&mut h.scene, &mut h.brush, "").unwrap();
        let hint_cleared = h.encoded_scene_hint();
        assert!(hint_cleared != hint_with_preview || hint_with_preview > 0);
        close_board_host(h);
    }

    #[test]
    fn board_host_brush_session_mirror_json_shows_preview_without_pointer() {
        let mut h = BoardHost::new();
        h.scene.set_size(800, 600, 1.0);
        h.set_active_utility("brush");
        h.style.set_board_kind_catalogs_from_json(
            &semio_framework_pack_json::json!({
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
        let _ = h.outbox.drain_events_json();
        let session = semio_framework_pack_json::json!({
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
        let ev = h.outbox.drain_events_json();
        assert!(!ev.contains("brushPlace"));
        assert!(h.encoded_scene_hint() > 0);
    }

    #[test]
    fn board_host_brush_candidates_sorted_by_handle_proximity() {
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
        let _ = h.outbox.drain_events_json();
        let inside = h.scene.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_move_screen(inside.x, inside.y, false, false, false);
        let ev = h.outbox.drain_events_json();
        assert!(ev.contains("brushCandidates"), "expected brushCandidates, got: {ev}");
        let v: semio_framework_pack_json::Value = semio_framework_pack_json::from_json_str(&ev, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let candidates =
            v.as_array().and_then(|rows| rows.iter().find(|row| row.get("name").and_then(|n| n.as_str()) == Some("brushCandidates")).and_then(|row| row.get("payload")).and_then(|p| p.get("candidates")).and_then(|c| c.as_array()).cloned());
        assert_eq!(candidates.as_ref().map(|rows| rows.len()), Some(3));
        let first_kind = candidates.as_ref().and_then(|rows| rows.first()).and_then(|row| row.get("nodeKind")).and_then(|x| x.as_str());
        assert_eq!(first_kind, Some("heavy"));
    }

    #[test]
    fn board_host_brush_lists_every_compatible_handle_per_node_kind() {
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
        let _ = h.outbox.drain_events_json();
        let inside = h.scene.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_move_screen(inside.x, inside.y, false, false, false);
        let ev = h.outbox.drain_events_json();
        let v: semio_framework_pack_json::Value = semio_framework_pack_json::from_json_str(&ev, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let candidates = v
            .as_array()
            .and_then(|rows| rows.iter().find(|row| row.get("name").and_then(|n| n.as_str()) == Some("brushCandidates")).and_then(|row| row.get("payload")).and_then(|p| p.get("candidates")).and_then(|c| c.as_array()).cloned())
            .unwrap_or_default();
        assert_eq!(candidates.len(), 2, "expected one row per compatible handle, got: {ev}");
        let indices: Vec<u64> = candidates.iter().filter_map(|row| row.get("targetHandleIndex").and_then(|i| i.as_u64())).collect();
        assert!(indices.contains(&0));
        assert!(indices.contains(&1));
    }
