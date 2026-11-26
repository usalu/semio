    /// 🧪️ Fill fixtures and mounted session helpers: the fill test constants and clock, the placement witness, the capture, session, checkpoint and job helpers and the frontier host.

    const FILL_TEST_PUMP_LIMIT: usize = 4_000_000;

    fn deadline_now_us() -> Option<u64> {
        Some(7_000)
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

    fn capture_fill_snapshot(original: &mut OriginalFixture, host: &BoardHost) -> crate::editor::puzzle2d::engine::BoardFillSnapshot {
        let mut construction = semio_framework_job::OriginalConstructionAdmission::default();
        let mut capture = begin_fill_capture(original, host, &mut construction);
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            match original.turn(FILL_TEST_GRANT, |cx| capture.step(&mut construction, &host.scene, &host.style, &host.brush, cx)).expect("original captured descriptor turn") {
                BoardFillCaptureStep::Pending => {}
                BoardFillCaptureStep::Complete => {
                    let snapshot = capture.take_snapshot().expect("complete fill capture owns snapshot");
                    retire_fill_capture(original, &mut capture);
                    retire_fill_construction(original, &mut construction);
                    return snapshot;
                }
                BoardFillCaptureStep::Fault(fault) => panic!("fill capture faulted: {fault:?}"),
            }
        }
        panic!("fill capture exceeded bounded cursor opportunities");
    }

    fn fixture(clock: fn() -> Option<u64>) -> OriginalFixture {
        OriginalFixture::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), fixture_stock(), clock, u64::MAX, 1 << 48)
    }

    fn begin_fill_capture(original: &mut OriginalFixture, host: &BoardHost, construction: &mut semio_framework_job::OriginalConstructionAdmission) -> crate::editor::puzzle2d::engine::BoardFillSnapshotCapture {
        let mut output = None;
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            if original.turn(FILL_TEST_GRANT, |cx| host.begin_board_fill_snapshot(construction, &mut output, cx)).expect("original capture admission") {
                return output.expect("admitted capture stays in caller custody");
            }
        }
        panic!("original capture admission remains pending");
    }

    fn new_fill_job(original: &mut OriginalFixture, host: &BoardHost, count: u32, operation: Operation) -> BoardFillJob {
        let mut snapshot = Some(capture_fill_snapshot(original, host));
        let mut output = None;
        let mut construction = semio_framework_job::OriginalConstructionAdmission::default();
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            if original.turn(FILL_TEST_GRANT, |cx| BoardFillJob::admit_original(&mut construction, &mut snapshot, count, operation, &mut output, cx)).expect("original fill job admission") {
                assert!(snapshot.is_none());
                retire_fill_construction(original, &mut construction);
                return output.expect("admitted fill job stays in caller custody");
            }
        }
        panic!("original fill job admission remains pending");
    }

    type FillSession = semio_framework_job_worker::MountedWorkerJobSession<BoardFillJob>;

    const FILL_TEST_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 4096, maximum_copy_bytes: 8 << 20, maximum_capacity_bytes: 8 << 20, maximum_release_bytes: 8 << 20, maximum_depth: 256 };

    #[derive(Debug, PartialEq)]
    enum FillSeen {
        Pending,
        Yield,
        Preview,
        Checkpoint,
        Complete(Option<BoardFillCommitCandidate>),
        Cancelled,
        Fault,
    }

    fn fill_config(site: &'static str) -> BatchDriveConfig {
        BatchDriveConfig { retained: FILL_TEST_GRANT, site, stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 7000 }
    }





    fn fill_placement_witness(placement: &BoardFillPlacement) -> FillPlacementWitness {
        FillPlacementWitness {
            node_kind: placement.node_kind.as_str().to_string(),
            edge_kind: placement.edge_kind.as_str().to_string(),
            source: placement.source_handle_id.as_str().to_string(),
            target: placement.target_handle_id.as_str().to_string(),
            x: placement.x,
            y: placement.y,
        }
    }

    /// 🚪️ A fill job closes through its own quoted demands; every job must reach terminal-empty before it drops.
    fn close_fill_job(original: &mut OriginalFixture, mut job: BoardFillJob) {
        original.close_job(&mut job);
    }

    /// 🧹 A settled construction admission retires its control metadata through the caller's funded turns.
    fn retire_fill_construction(original: &mut OriginalFixture, construction: &mut semio_framework_job::OriginalConstructionAdmission) {
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            if construction.terminal_is_empty() { return; }
            original.turn(FILL_TEST_GRANT, |cx| Ok(construction.close_step(cx)?)).expect("original fill construction retirement");
        }
        panic!("fill construction retirement exceeded bounded opportunities");
    }

    /// 🧹 A placement retires through its own quoted release demand; it asserts terminal-empty when dropped.
    fn retire_fill_placement(original: &mut OriginalFixture, mut placement: BoardFillPlacement) {
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            if placement.terminal_is_empty() { return; }
            original.turn(FILL_TEST_GRANT, |cx| {
                let demand = RetirementDemand { release_bytes: placement.next_close_release_byte_demand(), depth: 1, ..Default::default() };
                semio_framework_job::close_original_retained_step(cx, demand, |grant| semio_framework_value::RetainedWorkOutcome { result: Ok(match placement.close_step(grant) {
                    InteractiveJobCloseStep::Complete { progress } => RetainedCloneStep::Complete(progress),
                    InteractiveJobCloseStep::Pending { progress } => RetainedCloneStep::Progress(progress),
                    InteractiveJobCloseStep::Blocked => RetainedCloneStep::Progress(Default::default()),
                }), work: 1 })?;
                Ok(())
            }).expect("original fill placement retirement");
        }
        panic!("fill placement close exceeded bounded opportunities");
    }

    /// 🧹 A capture retires through its own quoted release demand.
    fn retire_fill_capture(original: &mut OriginalFixture, capture: &mut crate::editor::puzzle2d::engine::BoardFillSnapshotCapture) {
        capture.begin_close();
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            if capture.terminal_is_empty() { return; }
            original.turn(FILL_TEST_GRANT, |cx| {
                let demand = RetirementDemand { release_bytes: capture.next_close_release_byte_demand(), depth: 1, ..Default::default() };
                semio_framework_job::close_original_retained_step(cx, demand, |grant| semio_framework_value::RetainedWorkOutcome { result: Ok(match capture.close_step(grant) {
                    InteractiveJobCloseStep::Complete { progress } => RetainedCloneStep::Complete(progress),
                    InteractiveJobCloseStep::Pending { progress } => RetainedCloneStep::Progress(progress),
                    InteractiveJobCloseStep::Blocked => RetainedCloneStep::Progress(Default::default()),
                }), work: 1 })?;
                Ok(())
            }).expect("original fill capture retirement");
        }
        panic!("fill capture close exceeded bounded opportunities");
    }

    fn mount_fill_session(original: &mut OriginalFixture, job: BoardFillJob, config: BatchDriveConfig) -> FillSession {
        let mut pending = original.turn(FILL_TEST_GRANT, |cx| Ok(semio_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(job, (cx.operation(), cx.generation())))).expect("same original pending source");
        let mut output = None;
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            if original.turn(FILL_TEST_GRANT, |cx| pending.admit_pending(config, &mut output, cx)).expect("same original mounted admission") {
                assert!(pending.terminal_is_empty());
                return output.expect("admitted mounted session stays in caller custody");
            }
        }
        panic!("original mounted fill admission remains pending");
    }

    fn close_fill_session(original: &mut OriginalFixture, session: &mut FillSession) {
        session.begin_close();
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            if session.terminal_is_empty() { return; }
            original.turn(FILL_TEST_GRANT, |cx| {
                if session.has_original_cancel_alias_witness(cx.original_cancel_token()).map_err(|error| match error {
                    semio_framework_job_worker::WorkerJobDemandError::Refused(error) => semio_framework_job::OriginalTurnFailure::from(error),
                    semio_framework_job_worker::WorkerJobDemandError::Contention(_) => semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "original fill session remains contended").into(),
                })? {
                    let turn = cx.with_original_effect(cx.retained_grant(), |cx, effect| session.return_original_cancel_alias_step(cx.original_cancel_token(), effect))?.map_or(semio_framework_value::RetainedWorkOutcome { result: Ok(None), work: 0 }, |(turn, _)| turn);
                    let result = turn.result.map(|step| step.expect("same original witnessed cancellation alias")).map_err(|error| match error {
                        semio_framework_job_worker::WorkerJobDemandError::Refused(error) => error,
                        semio_framework_job_worker::WorkerJobDemandError::Contention(_) => semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "original fill session alias remains checked out"),
                    });
                    semio_framework_job::receive_original_retained_work(cx, semio_framework_value::RetainedWorkOutcome { result, work: turn.work })?;
                } else {
                    session.close_step(cx)?;
                }
                Ok(())
            }).expect("same original mounted fill close");
            std::thread::yield_now();
        }
        panic!("mounted fill close exceeded bounded opportunities");
    }

    /// 🔁️ One original-caller turn of the mounted session: the business producer runs on the caller's own funded turn.
    fn pump_fill_session(original: &mut OriginalFixture, session: &mut FillSession) -> WorkerJobPoll {
        original.turn(FILL_TEST_GRANT, |cx| session.pump_one_for_interactive_turn(cx)).expect("original mounted fill pump turn")
    }

    /// 🔎️ Settles the checked-out step receipt and classifies the retained outcome without taking it.
    fn seen_fill_outcome(original: &mut OriginalFixture, session: &mut FillSession) -> FillSeen {
        if let Some((issued, progress)) = session.take_checked_out_retained_step_receipt() {
            assert!(progress.fits(issued), "a fill step receipt must fit its original issued grant");
            original.turn(issued, |cx| { cx.consume_retained(progress)?; cx.consume_fuel(progress.copied_items as u64); Ok(()) }).expect("original fill callback receipt acceptance");
        }
        match session.checked_out_outcome().expect("checked-out fill outcome view") {
            Some(JobOutcomeView::Yield { .. }) => FillSeen::Yield,
            Some(JobOutcomeView::PreviewReady { .. }) => FillSeen::Preview,
            Some(JobOutcomeView::CheckpointReady { .. }) => FillSeen::Checkpoint,
            Some(JobOutcomeView::Complete { state, output, .. }) => FillSeen::Complete(BoardFillCommitCandidate::from_complete(state, output)),
            Some(JobOutcomeView::Cancelled { .. }) => FillSeen::Cancelled,
            Some(JobOutcomeView::Fault { .. }) => FillSeen::Fault,
            None => {
                assert!(session.checked_out_continuation().is_none(), "fill must deliver a semantic outcome through its original descriptor");
                FillSeen::Pending
            }
        }
    }

    fn acknowledge_fill_outcome(original: &mut OriginalFixture, session: &mut FillSession) {
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            if original.turn(FILL_TEST_GRANT, |cx| session.acknowledge_checked_out_outcome(cx)).expect("original fill outcome acknowledgement") { return; }
        }
        panic!("fill outcome acknowledgement exceeded bounded opportunities");
    }

    fn adopt_fill_checkpoint(original: &mut OriginalFixture, session: &mut FillSession, checkpoint: BoardFillCheckpoint) {
        let Some(job) = session.checked_out_job_mut() else {
            close_fill_job(original, checkpoint.into_closing_job());
            panic!("checkpoint job owner missing");
        };
        if let Err(checkpoint) = job.adopt_checkpoint(checkpoint) {
            close_fill_job(original, checkpoint.into_closing_job());
            panic!("checkpoint handback rejected");
        }
    }

    /// 🧹 Retires the large fixed-page checkpoint in a frame disjoint from mounted session admission.
    fn retire_checked_out_fill_checkpoint(original: &mut OriginalFixture, session: &mut FillSession) {
        let mut checkpoint = session.checked_out_job_mut().and_then(BoardFillJob::take_checkpoint).expect("field cursor checkpoint");
        if let Some(placement) = checkpoint.take_pending_placement() {
            retire_fill_placement(original, placement);
        }
        close_fill_job(original, checkpoint.into_closing_job());
    }

    fn run_fill_job(host: &BoardHost, count: u32, operation: Operation, worker_count: usize) -> (Vec<FillPlacementWitness>, Vec<u64>, crate::editor::puzzle2d::engine::BoardFillResult) {
        let mut original = fixture(semio_framework_job::default_now_us);
        let job = new_fill_job(&mut original, host, count, operation);
        let result = run_mounted_fill_job(&mut original, job, worker_count);
        original.finish();
        result
    }

    fn run_mounted_fill_job(original: &mut OriginalFixture, job: BoardFillJob, worker_count: usize) -> (Vec<FillPlacementWitness>, Vec<u64>, crate::editor::puzzle2d::engine::BoardFillResult) {
        let operation = job.operation();
        let mut session = mount_fill_session(original, job, fill_config("puzzle2d.fill.test"));
        let mut placements = Vec::new();
        let mut previews = Vec::new();
        let mut result = None;
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            match pump_fill_session(original, &mut session) {
                WorkerJobPoll::Submitted | WorkerJobPoll::Rejected | WorkerJobPoll::Idle => {
                    std::thread::yield_now();
                }
                WorkerJobPoll::Outcome | WorkerJobPoll::Terminal => {
                    assert!(!session.callback_verdict().is_some_and(|verdict| verdict.is_fault()), "exact fill session callback exceeded its clock authority");
                    let outcome = seen_fill_outcome(original, &mut session);
                    acknowledge_fill_outcome(original, &mut session);
                    match outcome {
                        FillSeen::Preview => {
                            let preview = session.checked_out_job_mut().and_then(BoardFillJob::take_preview).expect("typed fill preview");
                            previews.push(preview.sequence);
                            session.resume().expect("preview handback");
                        }
                        FillSeen::Checkpoint => {
                            let mut checkpoint = session.checked_out_job_mut().and_then(BoardFillJob::take_checkpoint).expect("typed fill checkpoint");
                            let placement = checkpoint.take_pending_placement().expect("checkpoint placement");
                            placements.push(fill_placement_witness(&placement));
                            retire_fill_placement(original, placement);
                            adopt_fill_checkpoint(original, &mut session, checkpoint);
                            session.resume().expect("checkpoint resume");
                        }
                        FillSeen::Complete(candidate) => {
                            let candidate = candidate.expect("typed full fill candidate");
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
                            break;
                        }
                        FillSeen::Pending | FillSeen::Yield => {
                            session.resume().expect("yield resume");
                        }
                        FillSeen::Cancelled => {
                            panic!("fill job unexpectedly cancelled");
                        }
                        FillSeen::Fault => {
                            let code = session.checked_out_job_mut().map(|job| (job.stage(), job.take_fault()));
                            panic!("fill job faulted: (stage, code)={code:?} previews={} placements={}", previews.len(), placements.len());
                        }
                    }
                }
                WorkerJobPoll::Closing | WorkerJobPoll::TerminalEmpty => panic!("fill session closed before terminal result"),
                WorkerJobPoll::CheckedOut => panic!("fill outcome remained checked out"),
            }
        }
        let result = result.expect("fill completion within bounded opportunities");
        close_fill_session(original, &mut session);
        (placements, previews, result)
    }

    /// 💾️ Runs `job` to its first checkpoint and claims the placement that checkpoint published — the owner's
    /// hand-off every resume requires — answering the resumable checkpoint and the claimed placement's witness.
    fn take_first_fill_checkpoint(original: &mut OriginalFixture, job: BoardFillJob) -> (BoardFillCheckpoint, FillPlacementWitness) {
        let mut checkpoint = first_fill_checkpoint(original, job);
        let placement = checkpoint.take_pending_placement().expect("a checkpoint publishes its placement");
        let witness = fill_placement_witness(&placement);
        retire_fill_placement(original, placement);
        (checkpoint, witness)
    }

    /// 💾️ Runs `job` to its first checkpoint, the placement it published still inside.
    fn first_fill_checkpoint(original: &mut OriginalFixture, job: BoardFillJob) -> BoardFillCheckpoint {
        let operation = job.operation();
        let mut session = mount_fill_session(original, job, fill_config("puzzle2d.fill.checkpoint"));
        let mut checkpoint = None;
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            match pump_fill_session(original, &mut session) {
                WorkerJobPoll::Submitted | WorkerJobPoll::Rejected | WorkerJobPoll::Idle => std::thread::yield_now(),
                WorkerJobPoll::Outcome | WorkerJobPoll::Terminal => {
                    let outcome = seen_fill_outcome(original, &mut session);
                    acknowledge_fill_outcome(original, &mut session);
                    if outcome == FillSeen::Checkpoint {
                        checkpoint = session.checked_out_job_mut().and_then(BoardFillJob::take_checkpoint);
                        break;
                    }
                    session.resume().expect("checkpoint search resume");
                }
                WorkerJobPoll::Closing | WorkerJobPoll::TerminalEmpty => panic!("fill closed before first checkpoint"),
                WorkerJobPoll::CheckedOut => panic!("checkpoint outcome remained checked out"),
            }
        }
        let checkpoint = checkpoint.expect("checkpoint within bounded opportunities");
        close_fill_session(original, &mut session);
        checkpoint
    }

    fn frontier_fill_host() -> BoardHost {
        let mut host = BoardHost::new();
        host.scene.set_size(800, 600, 1.0);
        host.set_suggestion_offset(40.0);
        host.set_brush_node_size(40.0);
        host.style.set_board_kind_catalogs_from_json(
            &semio_framework_pack_json::json!({
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
