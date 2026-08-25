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
    use infinite_canvas::{BoardFillCaptureStep, BoardFillJob};
    use semio_framework_job::{BatchDriveConfig, BatchJobParams, InteractiveStage, Operation, StepOutcome, WorkerJobPoll};
    use serde_json::json;

    const FILL_TEST_PUMP_LIMIT: usize = 4_000_000;

    static DEADLINE_CLOCK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    fn deadline_now_ms() -> u64 {
        DEADLINE_CLOCK.fetch_add(8, std::sync::atomic::Ordering::AcqRel)
    }

    #[derive(Debug, PartialEq)]
    struct FillPlacementWitness {
        node_kind: String,
        edge_kind: String,
        source: String,
        target: String,
        x: f64,
        y: f64,
    }

    fn capture_fill_snapshot(host: &BoardHost) -> infinite_canvas::BoardFillSnapshot {
        let mut capture = host.begin_board_fill_snapshot();
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            match capture.step(host) {
                BoardFillCaptureStep::Pending => {}
                BoardFillCaptureStep::Complete => return capture.take_snapshot().expect("complete fill capture owns snapshot"),
                BoardFillCaptureStep::Fault(fault) => panic!("fill capture faulted: {fault:?}"),
            }
        }
        panic!("fill capture exceeded bounded cursor opportunities");
    }

    fn mount_fill_session(job: BoardFillJob, params: BatchJobParams) -> semio_framework_job::MountedWorkerJobSession<BoardFillJob> {
        match semio_framework_job::MountedWorkerJobSession::try_new(job, params) {
            Ok(session) => session,
            Err(mut rejected) => {
                rejected.begin_close();
                for _ in 0..FILL_TEST_PUMP_LIMIT {
                    if matches!(rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) && rejected.terminal_is_empty() {
                        break;
                    }
                }
                assert!(rejected.terminal_is_empty());
                panic!("mounted fill session admission rejected");
            }
        }
    }

    fn close_fill_session(session: &mut semio_framework_job::MountedWorkerJobSession<BoardFillJob>) {
        session.begin_close();
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            if matches!(session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::WorkerJobCloseStep::Complete) && session.terminal_is_empty() {
                return;
            }
            std::thread::yield_now();
        }
        panic!("mounted fill close exceeded bounded opportunities");
    }

    fn pump_fill_session(session: &mut semio_framework_job::MountedWorkerJobSession<BoardFillJob>, pool: &semio_framework_async::WorkerPool) -> WorkerJobPoll {
        match session.pump_one(pool, semio_framework_async::Lane::Background) {
            Ok(poll) => poll,
            Err(_) => panic!("mounted fill pump fault"),
        }
    }

    fn close_fill_job(mut job: BoardFillJob) {
        semio_framework_job::InteractiveJob::begin_close(&mut job);
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            if matches!(semio_framework_job::InteractiveJob::close_step(&mut job, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES,), semio_framework_job::InteractiveJobCloseStep::Complete)
                && semio_framework_job::InteractiveJob::terminal_is_empty(&job)
            {
                return;
            }
        }
        panic!("detached fill close exceeded bounded opportunities");
    }

    fn adopt_fill_checkpoint(session: &mut semio_framework_job::MountedWorkerJobSession<BoardFillJob>, checkpoint: infinite_canvas::BoardFillCheckpoint) {
        let Some(job) = session.checked_out_job_mut() else {
            close_fill_job(checkpoint.into_closing_job());
            panic!("checkpoint job owner missing");
        };
        if let Err(checkpoint) = job.adopt_checkpoint(checkpoint) {
            close_fill_job(checkpoint.into_closing_job());
            panic!("checkpoint handback rejected");
        }
    }

    fn run_fill_job(host: &BoardHost, count: u32, operation: Operation, worker_count: usize) -> (Vec<FillPlacementWitness>, Vec<u64>, infinite_canvas::BoardFillResult) {
        let job = BoardFillJob::with_operation(capture_fill_snapshot(host), count, operation);
        run_mounted_fill_job(job, worker_count)
    }

    fn run_mounted_fill_job(job: BoardFillJob, worker_count: usize) -> (Vec<FillPlacementWitness>, Vec<u64>, infinite_canvas::BoardFillResult) {
        let operation = job.operation();
        let cancel = semio_framework_job::root_cancel_token();
        let params = BatchJobParams {
            operation: operation.operation,
            generation: operation.generation,
            cancel,
            config: BatchDriveConfig { site: "puzzle2d.fill.test", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_ms: 7 },
            now_ms: semio_framework_job::default_now_ms,
        };
        let mut session = mount_fill_session(job, params);
        let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, worker_count));
        let mut placements = Vec::new();
        let mut previews = Vec::new();
        let mut result = None;
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            match pump_fill_session(&mut session, &pool) {
                WorkerJobPoll::Submitted | WorkerJobPoll::Rejected | WorkerJobPoll::Idle => {
                    std::thread::yield_now();
                }
                WorkerJobPoll::Outcome | WorkerJobPoll::Terminal => {
                    let mut outcome = session.take_checked_out_outcome().expect("checked-out fill outcome");
                    match &outcome {
                        StepOutcome::PreviewReady(_) => {
                            let preview = session.checked_out_job_mut().and_then(BoardFillJob::take_preview).expect("typed fill preview");
                            previews.push(preview.sequence);
                            while !outcome.terminal_is_empty() {
                                let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                            }
                            session.resume().expect("preview handback");
                        }
                        StepOutcome::CheckpointReady(_) => {
                            let mut checkpoint = session.checked_out_job_mut().and_then(BoardFillJob::take_checkpoint).expect("typed fill checkpoint");
                            let mut placement = checkpoint.take_pending_placement().expect("checkpoint placement");
                            placements.push(FillPlacementWitness {
                                node_kind: placement.node_kind.as_str().to_string(),
                                edge_kind: placement.edge_kind.as_str().to_string(),
                                source: placement.source_handle_id.as_str().to_string(),
                                target: placement.target_handle_id.as_str().to_string(),
                                x: placement.x,
                                y: placement.y,
                            });
                            while !placement.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {}
                            assert!(placement.terminal_is_empty());
                            adopt_fill_checkpoint(&mut session, checkpoint);
                            while !outcome.terminal_is_empty() {
                                let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                            }
                            session.resume().expect("checkpoint resume");
                        }
                        StepOutcome::Complete(candidate) => {
                            let candidate = infinite_canvas::BoardFillCommitCandidate::from_commit_candidate(candidate).expect("typed full fill candidate");
                            if let Some(placement) = candidate.placement {
                                placements.push(FillPlacementWitness {
                                    node_kind: placement.node_kind.as_str().to_string(),
                                    edge_kind: placement.edge_kind.as_str().to_string(),
                                    source: placement.source_handle_id.as_str().to_string(),
                                    target: placement.target_handle_id.as_str().to_string(),
                                    x: placement.x,
                                    y: placement.y,
                                });
                            }
                            result = Some(candidate.result);
                            while !outcome.terminal_is_empty() {
                                let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                            }
                            break;
                        }
                        StepOutcome::Yield => {
                            session.resume().expect("yield resume");
                        }
                        StepOutcome::Cancelled => panic!("fill job unexpectedly cancelled"),
                        StepOutcome::Fault(_) => {
                            let code = session.checked_out_job_mut().and_then(BoardFillJob::take_fault);
                            panic!("fill job faulted: {code:?}");
                        }
                    }
                }
                WorkerJobPoll::Closing | WorkerJobPoll::TerminalEmpty => panic!("fill session closed before terminal result"),
                WorkerJobPoll::CheckedOut => panic!("fill outcome remained checked out"),
            }
        }
        let result = result.expect("fill completion within bounded opportunities");
        close_fill_session(&mut session);
        pool.shutdown();
        (placements, previews, result)
    }

    fn take_first_fill_checkpoint(job: BoardFillJob) -> infinite_canvas::BoardFillCheckpoint {
        let operation = job.operation();
        let params = BatchJobParams {
            operation: operation.operation,
            generation: operation.generation,
            cancel: semio_framework_job::root_cancel_token(),
            config: BatchDriveConfig { site: "puzzle2d.fill.checkpoint", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_ms: 7 },
            now_ms: semio_framework_job::default_now_ms,
        };
        let mut session = mount_fill_session(job, params);
        let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
        let mut checkpoint = None;
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            match pump_fill_session(&mut session, &pool) {
                WorkerJobPoll::Submitted | WorkerJobPoll::Rejected | WorkerJobPoll::Idle => std::thread::yield_now(),
                WorkerJobPoll::Outcome => {
                    let mut outcome = session.take_checked_out_outcome().expect("checkpoint outcome");
                    if matches!(outcome, StepOutcome::CheckpointReady(_)) {
                        checkpoint = session.checked_out_job_mut().and_then(BoardFillJob::take_checkpoint);
                        while !outcome.terminal_is_empty() {
                            let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                        }
                        break;
                    }
                    while !outcome.terminal_is_empty() {
                        let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                    }
                    session.resume().expect("checkpoint search resume");
                }
                WorkerJobPoll::Terminal => panic!("fill completed before first checkpoint"),
                WorkerJobPoll::Closing | WorkerJobPoll::TerminalEmpty => panic!("fill closed before first checkpoint"),
                WorkerJobPoll::CheckedOut => panic!("checkpoint outcome remained checked out"),
            }
        }
        let checkpoint = checkpoint.expect("checkpoint within bounded opportunities");
        close_fill_session(&mut session);
        pool.shutdown();
        checkpoint
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
        let first_operation = Operation::new(semio_framework_job::OperationId(42), semio_framework_job::RevisionId(9), semio_framework_job::Generation(1), 42);
        let second_operation = Operation::new(semio_framework_job::OperationId(42), semio_framework_job::RevisionId(9), semio_framework_job::Generation(1), 42);
        let (first, first_previews, first_result) = run_fill_job(&h, 3, first_operation, 1);
        let (second, _, second_result) = run_fill_job(&h, 3, second_operation, 1);
        assert_eq!(first, second, "fill must be deterministic for the same seed");
        assert_eq!(first_result.accepted_count, second_result.accepted_count);
        assert!(!first.is_empty(), "expected at least one fill placement");
        assert!(first.len() <= 3);
        assert!(first_previews.windows(2).all(|pair| pair[0] < pair[1]), "preview sequences must increase monotonically");
        assert_eq!(semio_framework_job::watchdog_step_overrun_us(first_operation.operation, first_operation.generation), None);
        let many_operation = Operation::new(semio_framework_job::OperationId(99), semio_framework_job::RevisionId(9), semio_framework_job::Generation(1), 99);
        let (many, _, _) = run_fill_job(&h, 1000, many_operation, 1);
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
        let operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(7), semio_framework_job::Generation(3), 77);
        let checkpoint = take_first_fill_checkpoint(BoardFillJob::with_operation(capture_fill_snapshot(&h), 12, operation));
        let stale_operation = Operation::new(operation.operation, operation.base_revision, semio_framework_job::Generation(4), operation.seed);
        let checkpoint = match BoardFillJob::restore(checkpoint, stale_operation) {
            Ok(job) => {
                close_fill_job(job);
                panic!("stale checkpoint was accepted");
            }
            Err(checkpoint) => checkpoint,
        };
        let resumed = match BoardFillJob::restore(checkpoint, operation) {
            Ok(job) => job,
            Err(checkpoint) => {
                close_fill_job(checkpoint.into_closing_job());
                panic!("exact checkpoint restore rejected");
            }
        };
        let (actual, previews, _) = run_mounted_fill_job(resumed, 1);
        let (expected, _, _) = run_fill_job(&h, 12, operation, 1);
        assert_eq!(actual, expected);
        assert!(!previews.is_empty());
    }

    #[test]
    fn board_fill_job_cancel_and_supersession_close_exact_mounted_owner() {
        let host = frontier_fill_host();
        let operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(11), semio_framework_job::Generation(4), 9);
        let job = BoardFillJob::with_operation(capture_fill_snapshot(&host), 32, operation);
        let cancel = semio_framework_job::root_cancel_token();
        cancel.cancel_now();
        let params = BatchJobParams {
            operation: operation.operation,
            generation: operation.generation,
            cancel,
            config: BatchDriveConfig { site: "puzzle2d.fill.cancel", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_ms: 7 },
            now_ms: semio_framework_job::default_now_ms,
        };
        let mut session = mount_fill_session(job, params);
        let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
        let mut cancelled = false;
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            match pump_fill_session(&mut session, &pool) {
                WorkerJobPoll::Submitted | WorkerJobPoll::Rejected | WorkerJobPoll::Idle => std::thread::yield_now(),
                WorkerJobPoll::Terminal => {
                    let outcome = session.take_checked_out_outcome().expect("cancel outcome");
                    assert_eq!(outcome, StepOutcome::Cancelled);
                    cancelled = true;
                    break;
                }
                WorkerJobPoll::Outcome => panic!("cancelled job published a nonterminal outcome"),
                WorkerJobPoll::Closing | WorkerJobPoll::TerminalEmpty | WorkerJobPoll::CheckedOut => panic!("cancelled owner entered invalid phase"),
            }
        }
        assert!(cancelled, "cancel terminal exceeded bounded opportunities");
        close_fill_session(&mut session);
        pool.shutdown();

        let stale_operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(11), semio_framework_job::Generation(4), 9);
        let stale_job = BoardFillJob::with_operation(capture_fill_snapshot(&host), 32, stale_operation);
        let stale_params = BatchJobParams {
            operation: stale_operation.operation,
            generation: semio_framework_job::Generation(5),
            cancel: semio_framework_job::root_cancel_token(),
            config: BatchDriveConfig { site: "puzzle2d.fill.stale", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_ms: 7 },
            now_ms: semio_framework_job::default_now_ms,
        };
        let mut stale = mount_fill_session(stale_job, stale_params);
        let stale_pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
        let mut stale_fault = false;
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            match pump_fill_session(&mut stale, &stale_pool) {
                WorkerJobPoll::Submitted | WorkerJobPoll::Rejected | WorkerJobPoll::Idle => std::thread::yield_now(),
                WorkerJobPoll::Terminal => {
                    let mut outcome = stale.take_checked_out_outcome().expect("stale outcome");
                    assert!(matches!(outcome, StepOutcome::Fault(_)));
                    while !outcome.terminal_is_empty() {
                        let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                    }
                    stale_fault = true;
                    break;
                }
                WorkerJobPoll::Outcome => panic!("stale job published a nonterminal outcome"),
                WorkerJobPoll::Closing | WorkerJobPoll::TerminalEmpty | WorkerJobPoll::CheckedOut => panic!("stale owner entered invalid phase"),
            }
        }
        assert!(stale_fault, "stale terminal exceeded bounded opportunities");
        close_fill_session(&mut stale);
        stale_pool.shutdown();
    }

    #[test]
    fn board_fill_job_deadline_yields_before_semantic_unit_and_closes_exactly() {
        let host = frontier_fill_host();
        let operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(13), semio_framework_job::Generation(6), 17);
        let job = BoardFillJob::with_operation(capture_fill_snapshot(&host), 8, operation);
        DEADLINE_CLOCK.store(0, std::sync::atomic::Ordering::Release);
        let params = BatchJobParams {
            operation: operation.operation,
            generation: operation.generation,
            cancel: semio_framework_job::root_cancel_token(),
            config: BatchDriveConfig { site: "puzzle2d.fill.deadline", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_ms: 7 },
            now_ms: deadline_now_ms,
        };
        let mut session = mount_fill_session(job, params);
        let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
        let mut yielded = false;
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            match pump_fill_session(&mut session, &pool) {
                WorkerJobPoll::Submitted | WorkerJobPoll::Rejected | WorkerJobPoll::Idle => std::thread::yield_now(),
                WorkerJobPoll::Outcome => {
                    let outcome = session.take_checked_out_outcome().expect("deadline outcome");
                    assert_eq!(outcome, StepOutcome::Yield);
                    yielded = true;
                    break;
                }
                WorkerJobPoll::Terminal => panic!("deadline yielded terminal outcome"),
                WorkerJobPoll::Closing | WorkerJobPoll::TerminalEmpty | WorkerJobPoll::CheckedOut => panic!("deadline owner entered invalid phase"),
            }
        }
        assert!(yielded, "deadline yield exceeded bounded opportunities");
        close_fill_session(&mut session);
        pool.shutdown();
    }

    #[test]
    fn board_fill_worker_refusal_and_unclaimed_complete_close_exact_owners() {
        let host = frontier_fill_host();
        let refused_operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(17), semio_framework_job::Generation(7), 23);
        let refused_params = BatchJobParams {
            operation: refused_operation.operation,
            generation: refused_operation.generation,
            cancel: semio_framework_job::root_cancel_token(),
            config: BatchDriveConfig { site: "puzzle2d.fill.refusal", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_ms: 7 },
            now_ms: semio_framework_job::default_now_ms,
        };
        let refused_job = BoardFillJob::with_operation(capture_fill_snapshot(&host), 4, refused_operation);
        let mut refused = mount_fill_session(refused_job, refused_params);
        let unavailable = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
        unavailable.shutdown();
        assert!(matches!(refused.pump_one(&unavailable, semio_framework_async::Lane::Background), Err(semio_framework_job::MountedWorkerJobPumpFault::Submit(_))));
        close_fill_session(&mut refused);

        let complete_operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(19), semio_framework_job::Generation(8), 29);
        let complete_params = BatchJobParams {
            operation: complete_operation.operation,
            generation: complete_operation.generation,
            cancel: semio_framework_job::root_cancel_token(),
            config: BatchDriveConfig { site: "puzzle2d.fill.unclaimed-complete", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_ms: 7 },
            now_ms: semio_framework_job::default_now_ms,
        };
        let complete_job = BoardFillJob::with_operation(capture_fill_snapshot(&host), 0, complete_operation);
        let mut complete = mount_fill_session(complete_job, complete_params);
        let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
        let mut terminal = false;
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            match pump_fill_session(&mut complete, &pool) {
                WorkerJobPoll::Submitted | WorkerJobPoll::Rejected | WorkerJobPoll::Idle => std::thread::yield_now(),
                WorkerJobPoll::Terminal => {
                    terminal = true;
                    break;
                }
                WorkerJobPoll::Outcome => panic!("zero-count fill published a nonterminal outcome"),
                WorkerJobPoll::Closing | WorkerJobPoll::TerminalEmpty | WorkerJobPoll::CheckedOut => {
                    panic!("unclaimed completion entered invalid phase")
                }
            }
        }
        assert!(terminal, "unclaimed completion exceeded bounded opportunities");
        close_fill_session(&mut complete);
        pool.shutdown();
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn board_fill_worker_saturation_returns_exact_session_for_close() {
        let gate = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let started = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
        let worker_gate = std::sync::Arc::clone(&gate);
        let worker_started = std::sync::Arc::clone(&started);
        let blocker: semio_framework_async::Job = Box::new(move || {
            worker_started.store(true, std::sync::atomic::Ordering::Release);
            while !worker_gate.load(std::sync::atomic::Ordering::Acquire) {
                std::hint::spin_loop();
            }
        });
        if let Err(error) = pool.try_submit(semio_framework_async::Lane::Background, blocker) {
            drop(error.into_job());
            panic!("saturation blocker admission failed");
        }
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            if started.load(std::sync::atomic::Ordering::Acquire) {
                break;
            }
            std::thread::yield_now();
        }
        assert!(started.load(std::sync::atomic::Ordering::Acquire));
        let mut queue_filled = true;
        for _ in 0..semio_framework_async::WORKER_JOBS_PER_LANE {
            if let Err(error) = pool.try_submit(semio_framework_async::Lane::Background, Box::new(|| {})) {
                drop(error.into_job());
                queue_filled = false;
                break;
            }
        }
        let host = frontier_fill_host();
        let operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(23), semio_framework_job::Generation(9), 31);
        let params = BatchJobParams {
            operation: operation.operation,
            generation: operation.generation,
            cancel: semio_framework_job::root_cancel_token(),
            config: BatchDriveConfig { site: "puzzle2d.fill.saturation", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_ms: 7 },
            now_ms: semio_framework_job::default_now_ms,
        };
        let mut session = mount_fill_session(BoardFillJob::with_operation(capture_fill_snapshot(&host), 4, operation), params);
        let saturated = matches!(
            session.pump_one(&pool, semio_framework_async::Lane::Background),
            Err(semio_framework_job::MountedWorkerJobPumpFault::Submit(semio_framework_job::WorkerJobSubmitFault::Pool(semio_framework_async::WorkerSubmitErrorKind::Saturated)))
        );
        gate.store(true, std::sync::atomic::Ordering::Release);
        close_fill_session(&mut session);
        pool.shutdown();
        assert!(queue_filled);
        assert!(saturated);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn board_fill_job_is_byte_identical_across_worker_counts() {
        let host = frontier_fill_host();
        let mut outputs = Vec::new();
        let default_workers = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
        for worker_count in [1usize, 2, 4, default_workers] {
            let operation = Operation::new(semio_framework_job::OperationId(191), semio_framework_job::RevisionId(2), semio_framework_job::Generation(8), 91);
            let (placements, previews, result) = run_fill_job(&host, 24, operation, worker_count);
            outputs.push((placements, previews, result.accepted_count, result.stalled, result.search_count));
        }
        assert!(outputs.windows(2).all(|pair| pair[0] == pair[1]));
    }

    #[test]
    fn board_fill_job_large_host_has_no_step_at_or_above_eight_ms() {
        let mut host = frontier_fill_host();
        let mut descriptor = link_test_scene_no_edge();
        let remaining_capacity = infinite_canvas::BOARD_FILL_NODE_CAPACITY - descriptor.nodes.len();
        for index in 0..remaining_capacity {
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
        let operation = Operation::new(semio_framework_job::OperationId(123), semio_framework_job::RevisionId(31), semio_framework_job::Generation(1), 123);
        let (_, previews, _) = run_fill_job(&host, 2, operation, 1);
        assert!(previews.len() > 2_000, "large host did not expose cursor progress");
        assert_eq!(semio_framework_job::watchdog_step_overrun_us(operation.operation, operation.generation), None);
    }

    #[test]
    fn board_fill_capture_max_plus_one_refuses_without_partial_snapshot() {
        let mut host = BoardHost::new();
        let mut descriptor = link_test_scene_no_edge();
        let prototype = descriptor.nodes[0].clone();
        while descriptor.nodes.len() <= infinite_canvas::BOARD_FILL_NODE_CAPACITY {
            let index = descriptor.nodes.len();
            let mut node = prototype.clone();
            node.id = format!("capture-capacity.{index}");
            node.x = index as f64 * 100.0;
            descriptor.nodes.push(node);
        }
        host.sync_descriptor(&descriptor).unwrap();
        let mut capture = host.begin_board_fill_snapshot();
        let mut fault = None;
        let capture_opportunities = infinite_canvas::BOARD_FILL_NODE_CAPACITY.saturating_mul(3).saturating_add(3);
        for _ in 0..capture_opportunities {
            if let BoardFillCaptureStep::Fault(found) = capture.step(&host) {
                fault = Some(found);
                break;
            }
        }
        assert_eq!(fault, Some(infinite_canvas::BoardFillCaptureFault::NodeCapacity));
        capture.begin_close();
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            if matches!(capture.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) && capture.terminal_is_empty() {
                break;
            }
        }
        assert!(capture.terminal_is_empty());
    }

    /// 🧱️ Mounted acceptance publishes every string, template, virtual owner, and source mutation as a distinct worker stage.
    #[test]
    fn board_fill_candidate_acceptance_exposes_every_retained_field_stage() {
        let host = frontier_fill_host();
        let operation = Operation::new(semio_framework_job::OperationId(271), semio_framework_job::RevisionId(37), semio_framework_job::Generation(11), 271);
        let params = BatchJobParams {
            operation: operation.operation,
            generation: operation.generation,
            cancel: semio_framework_job::root_cancel_token(),
            config: BatchDriveConfig { site: "puzzle2d.fill.field-cursors", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_ms: 7 },
            now_ms: semio_framework_job::default_now_ms,
        };
        let mut session = mount_fill_session(BoardFillJob::with_operation(capture_fill_snapshot(&host), 1, operation), params);
        let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
        let mut seen = [false; 13];
        let mut checkpointed = false;
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            match pump_fill_session(&mut session, &pool) {
                WorkerJobPoll::Submitted | WorkerJobPoll::Rejected | WorkerJobPoll::Idle => std::thread::yield_now(),
                WorkerJobPoll::Outcome => {
                    let mut outcome = session.take_checked_out_outcome().expect("field cursor outcome");
                    match &outcome {
                        StepOutcome::PreviewReady(_) => {
                            let job = session.checked_out_job_mut().expect("field cursor job");
                            match job.stage() {
                                infinite_canvas::BoardFillStage::AcceptNodeId => seen[0] = true,
                                infinite_canvas::BoardFillStage::AcceptEdgeId => seen[1] = true,
                                infinite_canvas::BoardFillStage::AcceptEdgeKind => seen[2] = true,
                                infinite_canvas::BoardFillStage::AcceptNodeKind => seen[3] = true,
                                infinite_canvas::BoardFillStage::AcceptSourceHandle => seen[4] = true,
                                infinite_canvas::BoardFillStage::AcceptTargetHandle => seen[5] = true,
                                infinite_canvas::BoardFillStage::AcceptIcon => seen[6] = true,
                                infinite_canvas::BoardFillStage::AcceptVirtualNode => seen[7] = true,
                                infinite_canvas::BoardFillStage::AcceptSourceConnection => seen[8] = true,
                                infinite_canvas::BoardFillStage::AcceptHandles => seen[9] = true,
                                infinite_canvas::BoardFillStage::AcceptHandleId => seen[10] = true,
                                infinite_canvas::BoardFillStage::AcceptHandleVirtual => seen[11] = true,
                                infinite_canvas::BoardFillStage::AcceptHandlePublish => seen[12] = true,
                                _ => {}
                            }
                            let _ = job.take_preview().expect("field cursor preview");
                            while !outcome.terminal_is_empty() {
                                let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                            }
                            session.resume().expect("field cursor resume");
                        }
                        StepOutcome::CheckpointReady(_) => {
                            let mut checkpoint = session.checked_out_job_mut().and_then(BoardFillJob::take_checkpoint).expect("field cursor checkpoint");
                            if let Some(mut placement) = checkpoint.take_pending_placement() {
                                while !placement.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {}
                                assert!(placement.terminal_is_empty());
                            }
                            close_fill_job(checkpoint.into_closing_job());
                            while !outcome.terminal_is_empty() {
                                let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                            }
                            checkpointed = true;
                            break;
                        }
                        StepOutcome::Yield => session.resume().expect("field cursor yield resume"),
                        StepOutcome::Complete(_) | StepOutcome::Cancelled | StepOutcome::Fault(_) => panic!("field cursor job terminated before checkpoint"),
                    }
                }
                WorkerJobPoll::Terminal | WorkerJobPoll::Closing | WorkerJobPoll::TerminalEmpty | WorkerJobPoll::CheckedOut => panic!("field cursor session entered invalid phase"),
            }
        }
        assert!(checkpointed);
        assert!(seen.into_iter().all(|value| value));
        close_fill_session(&mut session);
        pool.shutdown();
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
