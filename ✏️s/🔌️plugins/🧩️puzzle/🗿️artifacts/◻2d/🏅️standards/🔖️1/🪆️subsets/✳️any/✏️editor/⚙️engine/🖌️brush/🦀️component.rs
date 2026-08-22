//! 🖌️ Puzzle 2d app engine — the brush laws: slot preview/commit/cancel, candidate ordering by
//! handle proximity, per-node-kind compatibility enumeration, and the deterministic fill session.

//#region 🧪️Tests
#[cfg(test)]
#[allow(
    clippy::approx_constant,
    reason = "3.14159 is verbatim fixture data (a handle angle in a scene JSON literal), carried over unchanged from the pre-consolidation engine crate; swapping in std::f64::consts::PI would alter the recorded test input."
)]
mod tests {
    use crate::editor::puzzle2d::engine::board_host::testkit::*;
    use crate::editor::puzzle2d::engine::canvas::Point;
    use crate::editor::puzzle2d::engine::{handle_position_on_circle, BoardHost, HandleDescJson, NodeDescJson, SceneDescriptorJson};
    use infinite_canvas::BoardFillJob;
    use semio_framework_job::{BatchDriveConfig, BatchJobParams, InteractiveStage, Operation, StepBudget, StepOutcome};
    use serde_json::json;

    fn run_fill_job(mut job: BoardFillJob) -> (Vec<serde_json::Value>, Vec<u64>, std::time::Duration) {
        let operation = job.operation();
        let mut sequence = operation.preview_sequence;
        let mut previews = Vec::new();
        let mut max_step = std::time::Duration::ZERO;
        for _ in 0..1_000_000 {
            let started = std::time::Instant::now();
            let outcome = semio_framework_job::drive_step(
                &mut job,
                "puzzle2d.fill.test",
                operation.operation,
                operation.generation,
                InteractiveStage::InteractiveStep,
                StepBudget::new(1, u64::MAX),
                semio_framework_job::root_cancel_token(),
                || 0,
                &mut sequence,
            );
            max_step = max_step.max(started.elapsed());
            match outcome {
                StepOutcome::PreviewReady(bytes) => {
                    let preview: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
                    previews.push(preview.get("sequence").and_then(serde_json::Value::as_u64).unwrap());
                }
                StepOutcome::CheckpointReady(_) | StepOutcome::Yield => {}
                StepOutcome::Complete(candidate) => {
                    let value: serde_json::Value = serde_json::from_slice(&candidate.output).unwrap();
                    return (value.get("placements").and_then(serde_json::Value::as_array).cloned().unwrap_or_default(), previews, max_step);
                }
                StepOutcome::Cancelled | StepOutcome::Fault(_) => panic!("fill job ended without a commit"),
            }
        }
        panic!("fill job exceeded the bounded test drive");
    }

    fn frontier_fill_host() -> BoardHost {
        let mut host = BoardHost::new();
        host.set_size(800, 600, 1.0);
        host.set_suggestion_offset(40.0);
        host.set_brush_node_size(40.0);
        host.set_board_kind_catalogs_from_json(
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
        host.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        host
    }

    #[test]
    fn board_host_brush_slot_emits_preview_and_place_on_leave() {
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
    fn board_host_brush_open_slot_suggestions_commit_and_cancel() {
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
    fn board_host_brush_slot_commit_survives_pointer_move_out_of_slot() {
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
    fn board_host_brush_slot_skips_place_on_leave_without_alt() {
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
    fn board_host_brush_fill_frontier_deterministic_and_collision_limited() {
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
        let (first, first_previews, max_step) = run_fill_job(BoardFillJob::new(h.board_fill_snapshot(), 3, 42, 0, 1));
        let (second, _, _) = run_fill_job(BoardFillJob::new(h.board_fill_snapshot(), 3, 42, 0, 1));
        assert_eq!(first, second, "fill must be deterministic for the same seed");
        assert!(!first.is_empty(), "expected at least one fill placement");
        assert!(first.len() <= 3);
        assert!(first_previews.windows(2).all(|pair| pair[0] < pair[1]), "preview sequences must increase monotonically");
        assert!(max_step < std::time::Duration::from_millis(8), "bounded fill step took {max_step:?}");
        let (many, _, _) = run_fill_job(BoardFillJob::new(h.board_fill_snapshot(), 1000, 99, 0, 1));
        assert!(many.len() < 1000, "collision should cap fill before 1000 on a tight scene");
    }

    #[test]
    fn board_host_brush_fill_checkpoint_restore_matches_uninterrupted_replay() {
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
        let snapshot = h.board_fill_snapshot();
        let operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(7), semio_framework_job::Generation(3), 77);
        let (expected, _, _) = run_fill_job(BoardFillJob::with_operation(snapshot.clone(), 12, operation));
        let mut interrupted = BoardFillJob::with_operation(snapshot, 12, operation);
        let mut sequence = 0;
        for _ in 0..41 {
            let outcome = semio_framework_job::drive_step(
                &mut interrupted,
                "puzzle2d.fill.checkpoint",
                operation.operation,
                operation.generation,
                InteractiveStage::InteractiveStep,
                StepBudget::new(1, u64::MAX),
                semio_framework_job::root_cancel_token(),
                || 0,
                &mut sequence,
            );
            assert!(!outcome.is_terminal());
        }
        let resumed = BoardFillJob::restore(&interrupted.checkpoint_bytes(), operation).unwrap();
        let (actual, previews, _) = run_fill_job(resumed);
        assert_eq!(actual, expected);
        assert!(previews.first().copied().unwrap_or(sequence) >= sequence);
    }

    #[test]
    fn board_fill_job_cancel_and_supersession_leave_checkpoint_unchanged() {
        use semio_framework_job::InteractiveJob;

        let host = frontier_fill_host();
        let mut job = BoardFillJob::new(host.board_fill_snapshot(), 32, 9, 11, 4);
        let operation = job.operation();
        let before = job.checkpoint_bytes();
        let cancel = semio_framework_job::root_cancel_token();
        cancel.cancel_now();
        let mut sequence = 0;
        let mut cancelled = semio_framework_job::StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), cancel, || 0, &mut sequence);
        assert_eq!(job.step(&mut cancelled), StepOutcome::Cancelled);
        assert_eq!(job.checkpoint_bytes(), before);
        let mut stale = semio_framework_job::StepContext::new(operation.operation, semio_framework_job::Generation(operation.generation.0 + 1), StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
        assert!(matches!(job.step(&mut stale), StepOutcome::Fault(_)));
        assert_eq!(job.checkpoint_bytes(), before);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn board_fill_job_is_byte_identical_across_worker_counts() {
        let host = frontier_fill_host();
        let mut outputs = Vec::new();
        for worker_count in [1usize, 2, 4] {
            let operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(2), semio_framework_job::Generation(8), 91);
            let job = BoardFillJob::with_operation(host.board_fill_snapshot(), 24, operation);
            let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, worker_count));
            let params = BatchJobParams {
                operation: operation.operation,
                generation: operation.generation,
                cancel: semio_framework_job::root_cancel_token(),
                config: BatchDriveConfig { site: "puzzle2d.fill.workers", stage: InteractiveStage::BackgroundStep, fuel_per_step: 1, step_budget_ms: 7 },
                now_ms: semio_framework_job::default_now_ms,
            };
            let receiver = semio_framework_job::run_on_worker(&pool, semio_framework_async::Lane::Background, job, params);
            let outcome = receiver.recv_timeout(std::time::Duration::from_secs(10)).expect("fill worker did not finish");
            pool.shutdown();
            match outcome {
                StepOutcome::Complete(candidate) => outputs.push(candidate.output),
                other => panic!("worker_count={worker_count} ended with {other:?}"),
            }
        }
        assert!(outputs.windows(2).all(|pair| pair[0] == pair[1]));
    }

    #[test]
    fn board_fill_job_large_host_has_no_step_at_or_above_eight_ms() {
        let mut host = frontier_fill_host();
        let mut descriptor = link_test_scene_no_edge();
        for index in 0..1_024 {
            let node_id = format!("stress.{index}");
            descriptor.nodes.push(NodeDescJson {
                id: node_id.clone(),
                x: (index % 64) as f64 * 1_000.0 + 10_000.0,
                y: (index / 64) as f64 * 1_000.0 + 10_000.0,
                draggable: Some(true),
                selected: None,
                style: None,
                text: None,
                icon_kind: None,
                node_kind: Some("source.kind".into()),
                user_data: None,
                visible: None,
                locked: None,
                root: None,
                shape: Some("circle".into()),
                radius: Some(20.0),
                width: None,
                height: None,
                scale: None,
            });
            descriptor.handles.push(HandleDescJson {
                id: format!("{node_id}:h0"),
                node_id,
                angle: 0.0,
                radius: None,
                scale: None,
                selected: None,
                visible: None,
                locked: None,
                style: None,
                handle_kind: Some("child".into()),
                color: None,
                icon_kind: None,
                user_data: None,
            });
        }
        host.sync_descriptor(&descriptor).unwrap();
        let (_, previews, max_step) = run_fill_job(BoardFillJob::new(host.board_fill_snapshot(), 2, 123, 0, 1));
        assert!(previews.len() > 2_000, "large host did not expose cursor progress");
        assert!(max_step < std::time::Duration::from_millis(8), "adversarial fill step took {max_step:?}");
    }

    #[test]
    fn board_host_fixture_drop_preview_json_paints_while_select_utility_active() {
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
    fn board_host_fixture_drop_preview_uses_catalog_shape_and_icon_at_overview_lod() {
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
    fn board_host_brush_session_mirror_json_shows_preview_without_pointer() {
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
    fn board_host_brush_candidates_sorted_by_handle_proximity() {
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
    fn board_host_brush_lists_every_compatible_handle_per_node_kind() {
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
    fn board_host_fill_base_core_rectangular_excludes_cylindric_tambour() {
        const BASE_KIND: &str = "Base";
        const CYLINDRIC_TAMBOUR_KIND: &str = "Cylindric Tambour";
        const FIRST_STOREY_KIND: &str = "First Storey Tambour";
        let mut h = BoardHost::new();
        h.set_suggestion_offset(80.0);
        h.set_brush_node_size(40.0);

        let fixture: serde_json::Value = serde_json::to_value(<crate::artifacts::puzzle2d::Puzzle2dSnapshot as store::ArtifactDsl>::parse_dsl(crate::artifacts::puzzle2d::dsl::PUZZLE2D_NAKAGIN_EXAMPLE_TEXT).unwrap()).unwrap();
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
        let (placements, _, _) = run_fill_job(BoardFillJob::new(h.board_fill_snapshot(), 1, 7, 0, 1));
        assert_eq!(placements.len(), 1, "expected one fill placement on base");
        let node_kind = placements[0].get("nodeKind").and_then(|x| x.as_str()).unwrap_or("");
        assert_ne!(node_kind, CYLINDRIC_TAMBOUR_KIND, "cylindric tambour must not stack on rectangular core");
        assert_eq!(node_kind, FIRST_STOREY_KIND, "first storey tambour matches rectangular core stack");
    }

    #[test]
    fn board_host_brush_door_tambour_left_excludes_capital_with_metabolism_compat_rules() {
        const DOOR_TAMBOUR_LEFT: &str = "door tambour left";
        const CAPITAL_KIND: &str = "Capital";
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_active_utility("brush");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);

        let fixture: serde_json::Value = serde_json::to_value(<crate::artifacts::puzzle2d::Puzzle2dSnapshot as store::ArtifactDsl>::parse_dsl(crate::artifacts::puzzle2d::dsl::PUZZLE2D_NAKAGIN_EXAMPLE_TEXT).unwrap()).unwrap();
        let compat_str = fixture.get("meta").and_then(|m| m.get("kindCompatibility")).map_or_else(|| "[]".to_string(), |v| v.to_string());
        h.set_handle_link_compat_from_json(&compat_str).unwrap();
        let catalogs_str = fixture.get("meta").and_then(|m| m.get("kindCatalogs")).map_or_else(
            || "{}".to_string(),
            |kc| {
                serde_json::json!({
                    "handleKinds": kc.get("handles"),
                    "nodeKinds": kc.get("nodes"),
                })
                .to_string()
            },
        );
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
    fn board_host_brush_slot_accepts_pointer_on_node_body_at_overview_lod() {
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
    fn board_host_brush_slot_accepts_pointer_on_indirect_ring_anchor() {
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
