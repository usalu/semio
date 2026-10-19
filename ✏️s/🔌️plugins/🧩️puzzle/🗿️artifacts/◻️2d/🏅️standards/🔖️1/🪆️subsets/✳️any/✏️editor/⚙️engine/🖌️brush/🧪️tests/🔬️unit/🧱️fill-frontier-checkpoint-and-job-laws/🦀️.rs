    /// 🧱️ Fill frontier, checkpoint and job laws: the deterministic frontier, the checkpoint restore, the cancel, supersession, deadline, refusal and saturation closes, the worker count identity and the capture maximum.

    #[test]
    fn board_host_brush_fill_frontier_deterministic_and_collision_limited() {
        let mut h = BoardHost::new();
        h.scene.set_size(800, 600, 1.0);
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
        let many_operation = Operation::new(semio_framework_job::OperationId(99), semio_framework_job::RevisionId(9), semio_framework_job::Generation(1), 99);
        let (many, _, _) = run_fill_job(&h, 1000, many_operation, 1);
        assert!(many.len() < 1000, "collision should cap fill before 1000 on a tight scene");
    }

    #[test]
    fn board_host_brush_fill_checkpoint_restore_matches_uninterrupted_replay() {
        let mut original = fixture(semio_framework_job::default_now_us);
        let mut h = BoardHost::new();
        h.scene.set_size(800, 600, 1.0);
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
        let job = new_fill_job(&mut original, &h, 12, operation);
        let (checkpoint, first) = take_first_fill_checkpoint(&mut original, job);
        let stale_operation = Operation::new(operation.operation, operation.base_revision, semio_framework_job::Generation(4), operation.seed);
        let checkpoint = match BoardFillJob::restore(checkpoint, stale_operation) {
            Ok(job) => {
                close_fill_job(&mut original, job);
                panic!("stale checkpoint was accepted");
            }
            Err(checkpoint) => checkpoint,
        };
        let resumed = match BoardFillJob::restore(checkpoint, operation) {
            Ok(job) => job,
            Err(checkpoint) => {
                close_fill_job(&mut original, checkpoint.into_closing_job());
                panic!("exact checkpoint restore rejected");
            }
        };
        let (resumed_placements, previews, _) = run_mounted_fill_job(&mut original, resumed, 1);
        let (expected, _, _) = run_fill_job(&h, 12, operation, 1);
        let actual: Vec<FillPlacementWitness> = std::iter::once(first).chain(resumed_placements).collect();
        assert_eq!(actual, expected, "the claimed first placement plus the resumed run replay the uninterrupted run");
        assert!(!previews.is_empty());
        original.finish();
    }

    /// 🙅️ A checkpoint still carrying the placement it published never resumes: restore refuses it with the exact
    /// checkpoint back, so the placement can be claimed and the resume retried — a resumed job never holds, and never
    /// overwrites, a placement nobody claimed.
    #[test]
    fn a_checkpoint_with_an_unclaimed_placement_never_resumes() {
        let mut original = fixture(semio_framework_job::default_now_us);
        let mut h = BoardHost::new();
        h.scene.set_size(800, 600, 1.0);
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        h.style.set_board_kind_catalogs_from_json(&semio_framework_pack_json::json!({ "handleKinds": [{ "id": "child", "name": "Child", "color": "#888888" }], "nodeKinds": [{ "id": "brush.kind", "name": "Brush Kind", "handles": [{ "handleKind": "child", "angle": 0.0 }] }] }).to_string()).unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(9), semio_framework_job::Generation(2), 9);
        let job = new_fill_job(&mut original, &h, 4, operation);
        let checkpoint = first_fill_checkpoint(&mut original, job);
        assert!(checkpoint.pending_placement().is_some(), "the checkpoint publishes its placement");
        let mut checkpoint = match BoardFillJob::restore(checkpoint, operation) {
            Ok(job) => {
                close_fill_job(&mut original, job);
                panic!("a checkpoint with an unclaimed placement resumed");
            }
            Err(checkpoint) => checkpoint,
        };
        retire_fill_placement(&mut original, checkpoint.take_pending_placement().expect("the refused checkpoint still owns its placement"));
        match BoardFillJob::restore(checkpoint, operation) {
            Ok(job) => close_fill_job(&mut original, job),
            Err(checkpoint) => {
                close_fill_job(&mut original, checkpoint.into_closing_job());
                panic!("a claimed checkpoint must resume");
            }
        }
        original.finish();
    }

    #[test]
    fn board_fill_job_cancel_and_supersession_close_exact_mounted_owner() {
        let mut original = fixture(semio_framework_job::default_now_us);
        let host = frontier_fill_host();
        let operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(11), semio_framework_job::Generation(4), 9);
        let job = new_fill_job(&mut original, &host, 32, operation);
        let params = fill_config("puzzle2d.fill.cancel");
        let mut session = mount_fill_session(&mut original, job, params);
        original.cancel().cancel_now();
        let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
        let mut cancelled = false;
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            match pump_fill_session(&mut session, &pool) {
                WorkerJobPoll::Submitted | WorkerJobPoll::Rejected | WorkerJobPoll::Idle => std::thread::yield_now(),
                WorkerJobPoll::Terminal => {
                    assert_eq!(seen_fill_outcome(&mut original, &mut session), FillSeen::Cancelled);
                    acknowledge_fill_outcome(&mut original, &mut session);
                    cancelled = true;
                    break;
                }
                WorkerJobPoll::Outcome => panic!("cancelled job published a nonterminal outcome"),
                WorkerJobPoll::Closing | WorkerJobPoll::TerminalEmpty | WorkerJobPoll::CheckedOut => panic!("cancelled owner entered invalid phase"),
            }
        }
        assert!(cancelled, "cancel terminal exceeded bounded opportunities");
        close_fill_session(&mut original, &mut session);
        pool.shutdown().expect("bounded snapshot operation succeeds");

        original.finish();
        let mut source = fixture(semio_framework_job::default_now_us);
        let stale_operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(11), semio_framework_job::Generation(4), 9);
        let mut stale_job = new_fill_job(&mut source, &host, 32, stale_operation);
        let mut foreign = OriginalFixture::new(semio_framework_job::OperationId(2), semio_framework_job::Generation(5), fixture_stock(), semio_framework_job::default_now_us, u64::MAX, 1 << 48);
        assert!(foreign.turn(FILL_TEST_GRANT, |cx| stale_job.step(cx).map(|_| ())).is_err(), "different actual source roots must not execute the original job");
        source.close_job(&mut stale_job);
        source.finish();
        foreign.finish();
    }

    #[test]
    fn board_fill_job_deadline_yields_before_semantic_unit_and_closes_exactly() {
        let mut original = fixture(deadline_now_us);
        let host = frontier_fill_host();
        let operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(13), semio_framework_job::Generation(6), 17);
        let job = new_fill_job(&mut original, &host, 8, operation);
        DEADLINE_CLOCK.store(0, std::sync::atomic::Ordering::Release);
        let params = fill_config("puzzle2d.fill.deadline");
        let mut session = mount_fill_session(&mut original, job, params);
        let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
        let mut yielded = false;
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            match pump_fill_session(&mut session, &pool) {
                WorkerJobPoll::Submitted | WorkerJobPoll::Rejected | WorkerJobPoll::Idle => std::thread::yield_now(),
                WorkerJobPoll::Outcome => {
                    assert_eq!(seen_fill_outcome(&mut original, &mut session), FillSeen::Yield);
                    acknowledge_fill_outcome(&mut original, &mut session);
                    yielded = true;
                    break;
                }
                WorkerJobPoll::Terminal => panic!("deadline yielded terminal outcome"),
                WorkerJobPoll::Closing | WorkerJobPoll::TerminalEmpty | WorkerJobPoll::CheckedOut => panic!("deadline owner entered invalid phase"),
            }
        }
        assert!(yielded, "deadline yield exceeded bounded opportunities");
        close_fill_session(&mut original, &mut session);
        pool.shutdown().expect("bounded snapshot operation succeeds");
    }

    #[test]
    fn board_fill_worker_refusal_and_unclaimed_complete_close_exact_owners() {
        let mut original = fixture(semio_framework_job::default_now_us);
        let host = frontier_fill_host();
        let refused_operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(17), semio_framework_job::Generation(7), 23);
        let refused_params = fill_config("puzzle2d.fill.refusal");
        let refused_job = new_fill_job(&mut original, &host, 4, refused_operation);
        let mut refused = mount_fill_session(&mut original, refused_job, refused_params);
        let unavailable = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
        unavailable.shutdown().expect("bounded snapshot operation succeeds");
        assert!(matches!(refused.pump_one(&unavailable, semio_framework_async::Lane::Background, FILL_TEST_GRANT), Err(semio_framework_job_worker::MountedWorkerJobPumpFault::Submit(_))));
        close_fill_session(&mut original, &mut refused);

        let complete_operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(19), semio_framework_job::Generation(8), 29);
        let complete_params = fill_config("puzzle2d.fill.unclaimed-complete");
        let complete_job = new_fill_job(&mut original, &host, 0, complete_operation);
        let mut complete = mount_fill_session(&mut original, complete_job, complete_params);
        let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
        let mut terminal = false;
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            match pump_fill_session(&mut complete, &pool) {
                WorkerJobPoll::Submitted | WorkerJobPoll::Rejected | WorkerJobPoll::Idle => std::thread::yield_now(),
                WorkerJobPoll::Terminal => {
                    let _ = seen_fill_outcome(&mut original, &mut complete);
                    terminal = true;
                    break;
                }
                WorkerJobPoll::Outcome => {
                    let yielded = seen_fill_outcome(&mut original, &mut complete) == FillSeen::Yield;
                    acknowledge_fill_outcome(&mut original, &mut complete);
                    assert!(yielded, "a zero-count fill may only YIELD while it streams its empty commit, never publish a preview or checkpoint");
                    complete.resume().expect("zero-count fill yield resume");
                }
                WorkerJobPoll::Closing | WorkerJobPoll::TerminalEmpty | WorkerJobPoll::CheckedOut => {
                    panic!("unclaimed completion entered invalid phase")
                }
            }
        }
        assert!(terminal, "unclaimed completion exceeded bounded opportunities");
        close_fill_session(&mut original, &mut complete);
        pool.shutdown().expect("bounded snapshot operation succeeds");
        original.finish();
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn board_fill_worker_saturation_returns_exact_session_for_close() {
        let mut original = fixture(semio_framework_job::default_now_us);
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
        let params = fill_config("puzzle2d.fill.saturation");
        let job = new_fill_job(&mut original, &host, 4, operation);
        let mut session = mount_fill_session(&mut original, job, params);
        let saturated = matches!(
            session.pump_one(&pool, semio_framework_async::Lane::Background, FILL_TEST_GRANT),
            Err(semio_framework_job_worker::MountedWorkerJobPumpFault::Submit(semio_framework_job_worker::WorkerJobSubmitFault::Pool(semio_framework_async::WorkerSubmitErrorKind::Saturated)))
        );
        gate.store(true, std::sync::atomic::Ordering::Release);
        close_fill_session(&mut original, &mut session);
        pool.shutdown().expect("bounded snapshot operation succeeds");
        assert!(queue_filled);
        assert!(saturated);
        original.finish();
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
        let remaining_capacity = crate::editor::puzzle2d::engine::BOARD_FILL_NODE_CAPACITY - descriptor.nodes.len();
        for index in 0..remaining_capacity {
            let node_id = format!("stress.{index}");
            descriptor.nodes.push(NodeDescriptor {
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
            descriptor.handles.push(HandleDescriptor {
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
    }

    #[test]
    fn board_fill_capture_max_plus_one_refuses_without_partial_snapshot() {
        let mut original = fixture(semio_framework_job::default_now_us);
        let mut host = BoardHost::new();
        let mut descriptor = link_test_scene_no_edge();
        let prototype = descriptor.nodes[0].clone();
        while descriptor.nodes.len() <= crate::editor::puzzle2d::engine::BOARD_FILL_NODE_CAPACITY {
            let index = descriptor.nodes.len();
            let mut node = prototype.clone();
            node.id = format!("capture-capacity.{index}");
            node.x = index as f64 * 100.0;
            descriptor.nodes.push(node);
        }
        host.sync_descriptor(&descriptor).unwrap();
        let mut capture = begin_fill_capture(&mut original, &host);
        let mut fault = None;
        let capture_opportunities = crate::editor::puzzle2d::engine::BOARD_FILL_NODE_CAPACITY.saturating_mul(3).saturating_add(3);
        for _ in 0..capture_opportunities {
            if let BoardFillCaptureStep::Fault(found) = original.turn(FILL_TEST_GRANT, |cx| capture.step(&host, cx)).expect("original bounded capture refusal") {
                fault = Some(found);
                break;
            }
        }
        assert_eq!(fault, Some(crate::editor::puzzle2d::engine::BoardFillCaptureFault::NodeCapacity));
        retire_fill_capture(&mut original, &mut capture);
        assert!(capture.terminal_is_empty());
        original.finish();
    }
