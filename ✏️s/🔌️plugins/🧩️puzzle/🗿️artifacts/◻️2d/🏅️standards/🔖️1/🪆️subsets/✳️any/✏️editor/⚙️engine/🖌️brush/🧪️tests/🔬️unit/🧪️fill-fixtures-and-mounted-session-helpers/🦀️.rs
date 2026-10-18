    /// 🧪️ Fill fixtures and mounted session helpers: the fill test constants and clock, the placement witness, the capture, session, checkpoint and job helpers and the frontier host.

    const FILL_TEST_PUMP_LIMIT: usize = 4_000_000;

    static DEADLINE_CLOCK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    fn deadline_now_us() -> Option<u64> {
        Some(DEADLINE_CLOCK.fetch_add(8_000, std::sync::atomic::Ordering::AcqRel))
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

    fn capture_fill_snapshot(host: &BoardHost) -> crate::editor::puzzle2d::engine::BoardFillSnapshot {
        let mut capture = begin_board_fill_snapshot(&host.style, &host.brush);
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            match capture.step(host) {
                BoardFillCaptureStep::Pending => {}
                BoardFillCaptureStep::Complete => return capture.take_snapshot().expect("complete fill capture owns snapshot"),
                BoardFillCaptureStep::Fault(fault) => panic!("fill capture faulted: {fault:?}"),
            }
        }
        panic!("fill capture exceeded bounded cursor opportunities");
    }

    type FillSession = semio_framework_job_worker::MountedWorkerJobSession<BoardFillJob>;

    const FILL_TEST_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 4096, maximum_copy_bytes: 8 << 20, maximum_capacity_bytes: 8 << 20, maximum_release_bytes: 8 << 20, maximum_depth: 256 };

    #[derive(Debug, PartialEq)]
    enum FillSeen {
        Yield,
        Preview,
        Checkpoint,
        Complete(Option<BoardFillCommitCandidate>),
        Cancelled,
        Fault,
    }

    fn self_funded(demand: RetirementDemand) -> RetainedCloneGrant {
        RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
    }

    fn fill_params_with(operation: Operation, generation: semio_framework_job::Generation, cancel: semio_framework_job::CancelToken, site: &'static str, now_us: fn() -> Option<u64>) -> BatchJobParams {
        BatchJobParams {
            operation: operation.operation,
            generation,
            cancel,
            config: BatchDriveConfig { retained: FILL_TEST_GRANT, site, stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 7000 },
            now_us,
        }
    }

    fn fill_params(operation: Operation, site: &'static str) -> BatchJobParams {
        fill_params_with(operation, operation.generation, semio_framework_job::root_cancel_token(), site, semio_framework_job::default_now_us)
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
    fn close_fill_job(mut job: BoardFillJob) {
        InteractiveJob::begin_close(&mut job);
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            if InteractiveJob::terminal_is_empty(&job) {
                return;
            }
            let demand = RetirementDemand {
                copy_bytes: InteractiveJob::next_close_copy_byte_demand(&job).expect("a locally owned fill job quotes its copy demand"),
                capacity_bytes: InteractiveJob::next_close_capacity_byte_demand(&job, FILL_TEST_GRANT.maximum_copy_bytes).expect("a locally owned fill job quotes its capacity demand"),
                release_bytes: InteractiveJob::next_close_release_byte_demand(&job).expect("a locally owned fill job quotes its release demand"),
                depth: InteractiveJob::next_close_depth_demand(&job).expect("a locally owned fill job quotes its depth demand"),
            };
            let grant = self_funded(demand);
            let step = InteractiveJob::close_step(&mut job, grant);
            assert!(step.progress().fits(grant), "a fill job close receipt must fit its own quoted grant");
            assert!(!matches!(step, InteractiveJobCloseStep::Blocked | InteractiveJobCloseStep::Refused { .. }), "a locally owned fill job close has no external owner and no refusal");
        }
        panic!("detached fill close exceeded bounded opportunities");
    }

    /// 🧹 A placement retires through its own quoted release demand; it asserts terminal-empty when dropped.
    fn retire_fill_placement(mut placement: BoardFillPlacement) {
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            if placement.terminal_is_empty() {
                return;
            }
            let grant = self_funded(RetirementDemand { release_bytes: placement.next_close_release_byte_demand(), ..Default::default() });
            let step = placement.close_step(grant);
            assert!(step.progress().fits(grant), "a fill placement close receipt must fit its own quoted grant");
        }
        panic!("fill placement close exceeded bounded opportunities");
    }

    /// 🧹 A capture retires through its own quoted release demand.
    fn retire_fill_capture(capture: &mut crate::editor::puzzle2d::engine::BoardFillSnapshotCapture) {
        capture.begin_close();
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            if capture.terminal_is_empty() {
                return;
            }
            let grant = self_funded(RetirementDemand { release_bytes: capture.next_close_release_byte_demand(), ..Default::default() });
            let step = capture.close_step(grant);
            assert!(step.progress().fits(grant), "a fill capture close receipt must fit its own quoted grant");
        }
        panic!("fill capture close exceeded bounded opportunities");
    }

    fn mount_fill_session(job: BoardFillJob, params: BatchJobParams) -> FillSession {
        let (operation, generation) = (params.operation, params.generation);
        let mut job_slot = Some(job);
        let mut params_slot = Some(params);
        let mut recipient = RetainedCloneProgress::default();
        let admitted = match WorkerJobAdmissionContext::new(operation, generation, StepBudget::new(1, u64::MAX, FILL_TEST_GRANT), semio_framework_job::default_now_us, &mut recipient) {
            Ok(mut control) => FillSession::try_admit_owned(&mut job_slot, &mut params_slot, &mut control),
            Err(error) => Err(error),
        };
        match admitted {
            Ok(Some((session, _admission))) => session,
            Ok(None) | Err(_) => {
                if let Some(rejected) = job_slot.take() {
                    close_fill_job(rejected);
                }
                panic!("mounted fill session admission rejected");
            }
        }
    }

    fn close_fill_session(session: &mut FillSession) {
        session.begin_close();
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            if session.terminal_is_empty() {
                return;
            }
            let demand = session.retirement_demands(FILL_TEST_GRANT.maximum_copy_bytes).expect("a mounted fill session quotes its close demand");
            let grant = self_funded(demand);
            match session.close_step(grant) {
                WorkerJobCloseStep::Pending { progress } | WorkerJobCloseStep::Complete { progress } => assert!(progress.fits(grant), "a mounted fill session close receipt must fit its own quoted grant"),
                WorkerJobCloseStep::Blocked => std::thread::yield_now(),
                WorkerJobCloseStep::Refused { kind, .. } => panic!("mounted fill close was refused: {kind:?}"),
            }
        }
        panic!("mounted fill close exceeded bounded opportunities");
    }

    /// 🔁️ `WorkerPool::try_submit` is a TRY: it answers `Contended` whenever the live worker thread
    /// holds its own lane queue, and `Saturated` when that queue is full. Both are back-pressure the
    /// owner is expected to retry — `submit_retained_timer_job` reschedules on exactly these two —
    /// so treating the first refusal as a fault made every mounted fill law flaky-by-construction.
    fn pump_fill_session(session: &mut FillSession, pool: &semio_framework_async::WorkerPool) -> WorkerJobPoll {
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            match session.pump_one(pool, semio_framework_async::Lane::Background, FILL_TEST_GRANT) {
                Ok(poll) => return poll,
                Err(semio_framework_job_worker::MountedWorkerJobPumpFault::Submit(semio_framework_job_worker::WorkerJobSubmitFault::Pool(
                    semio_framework_async::WorkerSubmitErrorKind::Contended | semio_framework_async::WorkerSubmitErrorKind::Saturated,
                ))) => std::thread::yield_now(),
                Err(semio_framework_job_worker::MountedWorkerJobPumpFault::Submit(semio_framework_job_worker::WorkerJobSubmitFault::Contention(_))) => std::thread::yield_now(),
                Err(semio_framework_job_worker::MountedWorkerJobPumpFault::Submit(semio_framework_job_worker::WorkerJobSubmitFault::Pool(kind))) => panic!("mounted fill pump fault: the worker pool refused the step submission ({kind:?})"),
                Err(semio_framework_job_worker::MountedWorkerJobPumpFault::Submit(semio_framework_job_worker::WorkerJobSubmitFault::SequenceExhausted)) => panic!("mounted fill pump fault: the step sequence is exhausted"),
                Err(semio_framework_job_worker::MountedWorkerJobPumpFault::Take(_)) => panic!("mounted fill pump fault: the finished step could not be taken back"),
                Err(semio_framework_job_worker::MountedWorkerJobPumpFault::MissingTicket) => panic!("mounted fill pump fault: the session holds no ticket"),
                Err(semio_framework_job_worker::MountedWorkerJobPumpFault::CheckedOut) => panic!("mounted fill pump fault: an outcome is still checked out"),
            }
        }
        panic!("mounted fill pump never won admission within its bounded opportunities")
    }

    /// 🔎️ Settles the checked-out step receipt and classifies the retained outcome without taking it.
    fn seen_fill_outcome(session: &mut FillSession) -> FillSeen {
        if let Some((issued, progress)) = session.take_checked_out_retained_step_receipt() {
            assert!(progress.fits(issued), "a fill step receipt must fit the grant it was issued under");
        }
        match session.checked_out_outcome().expect("checked-out fill outcome view") {
            Some(JobOutcomeView::Yield { .. }) => FillSeen::Yield,
            Some(JobOutcomeView::PreviewReady { .. }) => FillSeen::Preview,
            Some(JobOutcomeView::CheckpointReady { .. }) => FillSeen::Checkpoint,
            Some(JobOutcomeView::Complete { state, output, .. }) => FillSeen::Complete(BoardFillCommitCandidate::from_complete(state, output)),
            Some(JobOutcomeView::Cancelled { .. }) => FillSeen::Cancelled,
            Some(JobOutcomeView::Fault { .. }) => FillSeen::Fault,
            None => panic!("a checked-out fill outcome must carry a retained outcome"),
        }
    }

    fn acknowledge_fill_outcome(session: &mut FillSession) {
        let acknowledgement = RetainedCloneGrant { maximum_items: 1, maximum_depth: 1, ..Default::default() };
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            if matches!(session.acknowledge_checked_out_outcome(acknowledgement), RetainedCloneStep::Complete(_)) {
                return;
            }
        }
        panic!("fill outcome acknowledgement exceeded bounded opportunities");
    }

    fn adopt_fill_checkpoint(session: &mut FillSession, checkpoint: BoardFillCheckpoint) {
        let Some(job) = session.checked_out_job_mut() else {
            close_fill_job(checkpoint.into_closing_job());
            panic!("checkpoint job owner missing");
        };
        if let Err(checkpoint) = job.adopt_checkpoint(checkpoint) {
            close_fill_job(checkpoint.into_closing_job());
            panic!("checkpoint handback rejected");
        }
    }

    /// 🧹 Retires the large fixed-page checkpoint in a frame disjoint from mounted session admission.
    fn retire_checked_out_fill_checkpoint(session: &mut FillSession) {
        let mut checkpoint = session.checked_out_job_mut().and_then(BoardFillJob::take_checkpoint).expect("field cursor checkpoint");
        if let Some(placement) = checkpoint.take_pending_placement() {
            retire_fill_placement(placement);
        }
        close_fill_job(checkpoint.into_closing_job());
    }

    fn run_fill_job(host: &BoardHost, count: u32, operation: Operation, worker_count: usize) -> (Vec<FillPlacementWitness>, Vec<u64>, crate::editor::puzzle2d::engine::BoardFillResult) {
        let job = BoardFillJob::with_operation(capture_fill_snapshot(host), count, operation);
        run_mounted_fill_job(job, worker_count)
    }

    fn run_mounted_fill_job(job: BoardFillJob, worker_count: usize) -> (Vec<FillPlacementWitness>, Vec<u64>, crate::editor::puzzle2d::engine::BoardFillResult) {
        let operation = job.operation();
        let mut session = mount_fill_session(job, fill_params(operation, "puzzle2d.fill.test"));
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
                    assert!(!session.callback_verdict().is_some_and(|verdict| verdict.is_fault()), "exact fill session callback exceeded its clock authority");
                    match seen_fill_outcome(&mut session) {
                        FillSeen::Preview => {
                            let preview = session.checked_out_job_mut().and_then(BoardFillJob::take_preview).expect("typed fill preview");
                            previews.push(preview.sequence);
                            acknowledge_fill_outcome(&mut session);
                            session.resume().expect("preview handback");
                        }
                        FillSeen::Checkpoint => {
                            let mut checkpoint = session.checked_out_job_mut().and_then(BoardFillJob::take_checkpoint).expect("typed fill checkpoint");
                            let placement = checkpoint.take_pending_placement().expect("checkpoint placement");
                            placements.push(fill_placement_witness(&placement));
                            retire_fill_placement(placement);
                            adopt_fill_checkpoint(&mut session, checkpoint);
                            acknowledge_fill_outcome(&mut session);
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
                            acknowledge_fill_outcome(&mut session);
                            break;
                        }
                        FillSeen::Yield => {
                            acknowledge_fill_outcome(&mut session);
                            session.resume().expect("yield resume");
                        }
                        FillSeen::Cancelled => {
                            acknowledge_fill_outcome(&mut session);
                            panic!("fill job unexpectedly cancelled");
                        }
                        FillSeen::Fault => {
                            let code = session.checked_out_job_mut().map(|job| (job.stage(), job.take_fault()));
                            acknowledge_fill_outcome(&mut session);
                            panic!("fill job faulted: (stage, code)={code:?} previews={} placements={}", previews.len(), placements.len());
                        }
                    }
                }
                WorkerJobPoll::Closing | WorkerJobPoll::TerminalEmpty => panic!("fill session closed before terminal result"),
                WorkerJobPoll::CheckedOut => panic!("fill outcome remained checked out"),
            }
        }
        let result = result.expect("fill completion within bounded opportunities");
        close_fill_session(&mut session);
        pool.shutdown().expect("bounded snapshot operation succeeds");
        (placements, previews, result)
    }

    /// 💾️ Runs `job` to its first checkpoint and claims the placement that checkpoint published — the owner's
    /// hand-off every resume requires — answering the resumable checkpoint and the claimed placement's witness.
    fn take_first_fill_checkpoint(job: BoardFillJob) -> (BoardFillCheckpoint, FillPlacementWitness) {
        let mut checkpoint = first_fill_checkpoint(job);
        let placement = checkpoint.take_pending_placement().expect("a checkpoint publishes its placement");
        let witness = fill_placement_witness(&placement);
        retire_fill_placement(placement);
        (checkpoint, witness)
    }

    /// 💾️ Runs `job` to its first checkpoint, the placement it published still inside.
    fn first_fill_checkpoint(job: BoardFillJob) -> BoardFillCheckpoint {
        let operation = job.operation();
        let mut session = mount_fill_session(job, fill_params(operation, "puzzle2d.fill.checkpoint"));
        let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
        let mut checkpoint = None;
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            match pump_fill_session(&mut session, &pool) {
                WorkerJobPoll::Submitted | WorkerJobPoll::Rejected | WorkerJobPoll::Idle => std::thread::yield_now(),
                WorkerJobPoll::Outcome | WorkerJobPoll::Terminal => {
                    if seen_fill_outcome(&mut session) == FillSeen::Checkpoint {
                        checkpoint = session.checked_out_job_mut().and_then(BoardFillJob::take_checkpoint);
                        acknowledge_fill_outcome(&mut session);
                        break;
                    }
                    acknowledge_fill_outcome(&mut session);
                    session.resume().expect("checkpoint search resume");
                }
                WorkerJobPoll::Closing | WorkerJobPoll::TerminalEmpty => panic!("fill closed before first checkpoint"),
                WorkerJobPoll::CheckedOut => panic!("checkpoint outcome remained checked out"),
            }
        }
        let checkpoint = checkpoint.expect("checkpoint within bounded opportunities");
        close_fill_session(&mut session);
        pool.shutdown().expect("bounded snapshot operation succeeds");
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
