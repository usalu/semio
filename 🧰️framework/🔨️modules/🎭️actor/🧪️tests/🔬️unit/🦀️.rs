
mod quick {
    use crate::*;
    use std::collections::VecDeque;
    use std::sync::Arc;

    //#region 🔖️ErrorContracts
    fn assert_error_contract(error: &(dyn std::error::Error + 'static), expected: &str) {
        assert_eq!(error.to_string(), expected);
        assert!(error.source().is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn owned_errors_preserve_thiserror_display_and_source_contracts() {
        let cases: Vec<(Box<dyn std::error::Error>, &str)> = vec![
            (Box::new(pack::PackError::Truncated(7, "header")), "pack: truncated at offset 7 reading header"),
            (Box::new(pack::PackError::InvalidTag { what: "Lane", tag: 9, offset: 12 }), "pack: invalid tag 9 for Lane at offset 12"),
            (Box::new(pack::PackError::InvalidUtf8("PackageId", 23)), "pack: invalid utf8 in PackageId at offset 23"),
            (Box::new(pack::PackError::OverlongVarint(31)), "pack: overlong varint at offset 31"),
            (Box::new(JobPublicationError::OperationMismatch { active: 5, published: 8 }), "job bridge operation mismatch: active=5, published=8"),
            (Box::new(JobPublicationError::Stale { live_revision: 13, live_generation: 21 }), "job bridge stale revision/generation: live revision=13, generation=21"),
            (Box::new(JobPublicationError::StepSequence { expected: 34, published: 55 }), "job bridge step sequence mismatch: expected=34, published=55"),
            (Box::new(JobPublicationError::PreviewSequence { before: 89, after: 144 }), "job bridge preview cursor mismatch: before=89, after=144"),
            (Box::new(JobPublicationError::Terminal), "job bridge received a turn after a terminal publication"),
            (Box::new(KernelError::UnknownActor), "unknown actor"),
            (Box::new(KernelError::NoExclusiveShard), "no exclusive shard available"),
            (Box::new(KernelError::InvalidTransition), "invalid status transition"),
        ];

        for (error, expected) in cases {
            assert_error_contract(error.as_ref(), expected);
        }
    }
    //#endregion 🔖️ErrorContracts

    //#region 🔖️Helpers
    async fn env(to: ActorId, lane: Lane, seq: u64) -> Envelope {
        Envelope { to, from: Origin::Kernel, lane, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: vec![1, 2, 3] } }
    }

    async fn ok_turn() -> TurnResult {
        TurnResult {
            ui_patches: vec![],
            effects: vec![],
            command_ingress: vec![],
            cold_pair_ingress: Default::default(),
            lifecycle_receipt: None,
            ui_patch_receipt: None,
            next_wake: None,
            status: TurnStatus::Idle,
            usage: Usage { fuel: 100, wall_us: 50, memory_bytes: 1024 },
        }
    }

    fn bridge_operation() -> job::Operation {
        job::Operation::new(job::OperationId(91), job::RevisionId(7), job::Generation(3), 0x5eed)
    }

    fn bridge_turn(step_sequence: u64, preview_sequence: u64) -> JobTurn {
        let mut operation = bridge_operation();
        operation.preview_sequence = preview_sequence;
        JobTurn { job: 44, operation: JobOperation::from_job(operation), step_sequence }
    }

    const BRIDGE_NOW_US: fn() -> Option<u64> = || Some(10);

    #[derive(Default)]
    struct ScriptJob {
        outcomes: VecDeque<JobStepOutcome>,
        calls: usize,
        pending_state: Option<job::RetainedJobPayload>,
        pending_complete: Option<JobCommitCandidate>,
        closing: bool,
    }

    impl job::InteractiveJob for ScriptJob {
        fn step(&mut self, cx: &mut job::StepContext<'_>) -> job::StepOutcome {
            self.calls += 1;
            if let Some(candidate) = self.pending_complete.take() {
                let output = cx.payload_from_bytes(job::JobPayloadStream::CommitOutput, &candidate.output).expect("scripted output payload");
                let state = self.pending_state.take().expect("scripted state payload");
                return job::StepOutcome::Complete(job::CommitCandidate { state, output });
            }
            let outcome = self.outcomes.pop_front().expect("scripted bridge outcome");
            match outcome {
                JobStepOutcome::Yield => job::StepOutcome::Yield,
                JobStepOutcome::PreviewReady { preview } => {
                    cx.next_preview_sequence().expect("fixture preview sequence has capacity");
                    let preview = cx.payload_from_bytes(job::JobPayloadStream::Preview, &preview).expect("scripted preview payload");
                    job::StepOutcome::PreviewReady(preview)
                }
                JobStepOutcome::CheckpointReady { checkpoint } => {
                    let state = cx.payload_from_bytes(job::JobPayloadStream::CheckpointState, &checkpoint.state).expect("scripted checkpoint payload");
                    job::StepOutcome::CheckpointReady(job::Checkpoint { state, applied_progress: checkpoint.applied_progress })
                }
                JobStepOutcome::Complete { candidate } => {
                    self.pending_state = Some(cx.payload_from_bytes(job::JobPayloadStream::CommitState, &candidate.state).expect("scripted commit state"));
                    self.pending_complete = Some(candidate);
                    job::StepOutcome::Yield
                }
                JobStepOutcome::Cancelled => job::StepOutcome::Cancelled,
                JobStepOutcome::Fault { detail } => {
                    let detail = cx.payload_from_bytes(job::JobPayloadStream::Fault, &detail).expect("scripted fault payload");
                    job::StepOutcome::Fault(job::JobFault { detail })
                }
            }
        }

        fn begin_close(&mut self) {
            self.closing = true;
        }

        fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> job::InteractiveJobCloseStep {
            self.begin_close();
            if let Some(state) = self.pending_state.as_mut() {
                return match state.close_step(maximum_items, maximum_bytes) {
                    job::JobPayloadCloseStep::Pending { released_items, released_bytes } => job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                    job::JobPayloadCloseStep::Complete => {
                        self.pending_state = None;
                        job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                    }
                };
            }
            if maximum_items == 0 && (!self.outcomes.is_empty() || self.pending_complete.is_some()) {
                return job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            if self.pending_complete.take().is_some() || self.outcomes.pop_front().is_some() {
                return job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            job::InteractiveJobCloseStep::Complete
        }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.outcomes.is_empty() && self.pending_state.is_none() && self.pending_complete.is_none()
        }
    }

    struct UnsequencedPreviewJob;

    impl job::InteractiveJob for UnsequencedPreviewJob {
        fn step(&mut self, _cx: &mut job::StepContext<'_>) -> job::StepOutcome {
            job::StepOutcome::PreviewReady(job::RetainedJobPayload::empty(job::JobPayloadStream::Preview))
        }

        fn begin_close(&mut self) {}

        fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> job::InteractiveJobCloseStep {
            job::InteractiveJobCloseStep::Complete
        }

        fn terminal_is_empty(&self) -> bool {
            true
        }
    }
    //#endregion 🔖️Helpers

    //#region 🪪️JobBridge
    #[test]
    fn job_bridge_invokes_exactly_one_step_per_turn() {
        let operation = bridge_operation();
        let mut bridge = JobTurnBridge::new(operation);
        let mut job = ScriptJob { outcomes: VecDeque::from([JobStepOutcome::Yield, JobStepOutcome::Yield]), calls: 0, ..Default::default() };
        let publication = bridge
            .step(&mut job, bridge_turn(0, 0), operation.operation, operation.base_revision, operation.generation, "actor.job.one-step", job::InteractiveStage::InteractiveStep, job::StepBudget::new(100, 20), job::root_cancel_token(), BRIDGE_NOW_US)
            .expect("first job turn");
        assert_eq!(job.calls, 1);
        assert!(matches!(publication.outcome, JobStepOutcome::Yield));
    }

    #[test]
    fn job_bridge_preserves_checkpoint_state_and_applied_progress() {
        let operation = bridge_operation();
        let mut bridge = JobTurnBridge::new(operation);
        let checkpoint = JobCheckpoint { state: vec![4, 5, 6], applied_progress: 73 };
        let mut job = ScriptJob { outcomes: VecDeque::from([JobStepOutcome::CheckpointReady { checkpoint: checkpoint.clone() }]), calls: 0, ..Default::default() };
        let pending = bridge
            .step(&mut job, bridge_turn(0, 0), operation.operation, operation.base_revision, operation.generation, "actor.job.checkpoint", job::InteractiveStage::BackgroundStep, job::StepBudget::new(100, 20), job::root_cancel_token(), BRIDGE_NOW_US)
            .expect("checkpoint projection admission");
        assert!(matches!(pending.outcome, JobStepOutcome::Yield));
        let publication = bridge
            .step(
                &mut job,
                JobTurn { step_sequence: 1, ..pending.turn },
                operation.operation,
                operation.base_revision,
                operation.generation,
                "actor.job.checkpoint",
                job::InteractiveStage::BackgroundStep,
                job::StepBudget::new(100, 20),
                job::root_cancel_token(),
                BRIDGE_NOW_US,
            )
            .expect("checkpoint publication");
        assert_eq!(publication.outcome, JobStepOutcome::CheckpointReady { checkpoint: checkpoint.clone() });
        assert_eq!(publication.turn_status(), TurnStatus::CheckpointReady { checkpoint });
    }

    #[semio_framework_async_macros::async_test]
    async fn job_bridge_cancellation_is_terminal_and_skips_the_job() {
        let operation = bridge_operation();
        let mut bridge = JobTurnBridge::new(operation);
        let cancel = job::root_cancel_token();
        cancel.cancel().await;
        let mut job = ScriptJob { outcomes: VecDeque::from([JobStepOutcome::Yield]), calls: 0, ..Default::default() };
        let publication = bridge
            .step(&mut job, bridge_turn(0, 0), operation.operation, operation.base_revision, operation.generation, "actor.job.cancel", job::InteractiveStage::InteractiveStep, job::StepBudget::new(100, 20), cancel, BRIDGE_NOW_US)
            .expect("cancel publication");
        assert_eq!(job.calls, 0);
        assert!(matches!(publication.outcome, JobStepOutcome::Cancelled));
        assert!(matches!(
            bridge.step(
                &mut job,
                bridge_turn(1, 0),
                operation.operation,
                operation.base_revision,
                operation.generation,
                "actor.job.cancel",
                job::InteractiveStage::InteractiveStep,
                job::StepBudget::new(100, 20),
                job::root_cancel_token(),
                BRIDGE_NOW_US
            ),
            Err(JobPublicationError::Terminal)
        ));
    }

    #[test]
    fn job_bridge_rejects_stale_commit_before_work_or_publication() {
        let operation = bridge_operation();
        let mut bridge = JobTurnBridge::new(operation);
        let mut job = ScriptJob { outcomes: VecDeque::from([JobStepOutcome::Complete { candidate: JobCommitCandidate { state: vec![1], output: vec![2] } }]), calls: 0, ..Default::default() };
        let result =
            bridge.step(&mut job, bridge_turn(0, 0), operation.operation, job::RevisionId(8), operation.generation, "actor.job.stale", job::InteractiveStage::InteractiveStep, job::StepBudget::new(100, 20), job::root_cancel_token(), BRIDGE_NOW_US);
        assert!(matches!(result, Err(JobPublicationError::Stale { live_revision: 8, live_generation: 3 })));
        assert_eq!(job.calls, 0);
    }

    #[test]
    fn job_bridge_rejects_replayed_preview_identity_before_work() {
        let operation = bridge_operation();
        let mut bridge = JobTurnBridge::new(operation);
        let mut job = ScriptJob { outcomes: VecDeque::from([JobStepOutcome::Yield]), calls: 0, ..Default::default() };
        assert!(matches!(
            bridge.step(
                &mut job,
                bridge_turn(0, 1),
                operation.operation,
                operation.base_revision,
                operation.generation,
                "actor.job.replayed-preview",
                job::InteractiveStage::InteractiveStep,
                job::StepBudget::new(100, 20),
                job::root_cancel_token(),
                BRIDGE_NOW_US,
            ),
            Err(JobPublicationError::Stale { .. })
        ));
        assert_eq!(job.calls, 0);
    }

    #[test]
    fn job_bridge_rejects_a_preview_without_exactly_one_sequence_advance() {
        let operation = bridge_operation();
        let mut bridge = JobTurnBridge::new(operation);
        assert!(matches!(
            bridge.step(
                &mut UnsequencedPreviewJob,
                bridge_turn(0, 0),
                operation.operation,
                operation.base_revision,
                operation.generation,
                "actor.job.preview-order",
                job::InteractiveStage::InteractiveStep,
                job::StepBudget::new(100, 20),
                job::root_cancel_token(),
                BRIDGE_NOW_US,
            ),
            Err(JobPublicationError::PreviewSequence { before: 0, after: 0 })
        ));
    }

    #[test]
    fn mounted_fixed_replay_capture_is_deterministic_and_returns_the_exact_live_owner() {
        fn run(worker_count: u16, worker_slot: u16) -> [u64; 4] {
            let operation = bridge_operation();
            let mut bridge = JobTurnBridge::new(operation);
            let mut job = ScriptJob {
                outcomes: VecDeque::from([
                    JobStepOutcome::Yield,
                    JobStepOutcome::PreviewReady { preview: vec![9, 8] },
                    JobStepOutcome::CheckpointReady { checkpoint: JobCheckpoint { state: vec![7, 6], applied_progress: 5 } },
                    JobStepOutcome::Complete { candidate: JobCommitCandidate { state: vec![3], output: vec![2, 1] } },
                ]),
                calls: 0,
                ..Default::default()
            };
            let route = JobReplayRoute { plugin: [1; 32], package: [2; 32], controller: [3; 32], tool: [4; 32], window: 5, document: [6; 32], request_schema: [7; 32], request_version: 1, request_digest: [8; 32] };
            let mut log = JobReplayLog::new(route, operation.generation.0).expect("fixed replay authority");
            let mut turn = bridge_turn(0, 0);
            loop {
                let publication = bridge
                    .step(&mut job, turn, operation.operation, operation.base_revision, operation.generation, "actor.job.replay", job::InteractiveStage::InteractiveStep, job::StepBudget::new(100, 20), job::root_cancel_token(), BRIDGE_NOW_US)
                    .expect("replay publication");
                let terminal = matches!(&publication.outcome, JobStepOutcome::Complete { .. });
                turn = JobTurn { step_sequence: publication.turn.step_sequence + 1, ..publication.turn };
                let payload_identity = match &publication.outcome {
                    JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
                    JobStepOutcome::CheckpointReady { checkpoint } => checkpoint.state.as_ptr(),
                    JobStepOutcome::Complete { candidate } => candidate.output.as_ptr(),
                    _ => std::ptr::null(),
                };
                let mut preview_sequence = publication.turn.operation.preview_sequence;
                let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), BRIDGE_NOW_US, &mut preview_sequence);
                log.begin_capture(&mut context, ActorId(9), JobReplaySchedule { worker_count, worker_slot, granted_fuel: 1, deadline_class_ms: 4 }, publication).expect("capture admission");
                loop {
                    let mut preview_sequence = turn.operation.preview_sequence;
                    let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), BRIDGE_NOW_US, &mut preview_sequence);
                    if log.capture_step(&mut context) == JobReplayStep::PublicationReady {
                        break;
                    }
                }
                let publication = log.take_captured_publication().expect("same live publication");
                let returned_identity = match &publication.outcome {
                    JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
                    JobStepOutcome::CheckpointReady { checkpoint } => checkpoint.state.as_ptr(),
                    JobStepOutcome::Complete { candidate } => candidate.output.as_ptr(),
                    _ => std::ptr::null(),
                };
                assert_eq!(returned_identity, payload_identity);
                drop(publication);
                let now = job::default_now_us().expect("native test clock");
                let mut sequence = 0;
                let mut context = job::StepContext::new(
                    job::OperationId(operation.operation.0),
                    job::Generation(operation.generation.0),
                    job::StepBudget::from_duration(1, now, 4_000).expect("test deadline"),
                    job::root_cancel_token(),
                    job::default_now_us,
                    &mut sequence,
                );
                log.acknowledge_publication(&mut context, JobReplayPublicationPolicy::Accepted).expect("P2d ACK");
                if terminal {
                    break;
                }
            }
            log.begin_replay(operation.generation.0).expect("sealed generation replay");
            let mut replay_bridge = JobTurnBridge::new(operation);
            let mut replay_job = ScriptJob {
                outcomes: VecDeque::from([
                    JobStepOutcome::Yield,
                    JobStepOutcome::PreviewReady { preview: vec![9, 8] },
                    JobStepOutcome::CheckpointReady { checkpoint: JobCheckpoint { state: vec![7, 6], applied_progress: 5 } },
                    JobStepOutcome::Complete { candidate: JobCommitCandidate { state: vec![3], output: vec![2, 1] } },
                ]),
                calls: 0,
                ..Default::default()
            };
            let mut replay_turn = bridge_turn(0, 0);
            loop {
                let publication = replay_bridge
                    .step(
                        &mut replay_job,
                        replay_turn,
                        operation.operation,
                        operation.base_revision,
                        operation.generation,
                        "actor.job.replay-live",
                        job::InteractiveStage::InteractiveStep,
                        job::StepBudget::new(100, 20),
                        job::root_cancel_token(),
                        BRIDGE_NOW_US,
                    )
                    .expect("replayed live publication");
                assert_eq!(log.expected_turn(), Some(publication.turn));
                let terminal = matches!(&publication.outcome, JobStepOutcome::Complete { .. });
                replay_turn = JobTurn { step_sequence: publication.turn.step_sequence + 1, ..publication.turn };
                let mut preview_sequence = publication.turn.operation.preview_sequence;
                let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), BRIDGE_NOW_US, &mut preview_sequence);
                log.begin_capture(&mut context, ActorId(9), JobReplaySchedule { worker_count, worker_slot, granted_fuel: 1, deadline_class_ms: 4 }, publication).expect("replay capture admission");
                loop {
                    let mut preview_sequence = replay_turn.operation.preview_sequence;
                    let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), BRIDGE_NOW_US, &mut preview_sequence);
                    if log.capture_step(&mut context) == JobReplayStep::PublicationReady {
                        break;
                    }
                }
                drop(log.take_captured_publication().expect("matched replay publication"));
                let mut preview_sequence = replay_turn.operation.preview_sequence;
                let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), BRIDGE_NOW_US, &mut preview_sequence);
                log.acknowledge_publication(&mut context, JobReplayPublicationPolicy::Accepted).expect("matched replay policy");
                while log.has_pending_work() {
                    let mut preview_sequence = replay_turn.operation.preview_sequence;
                    let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), BRIDGE_NOW_US, &mut preview_sequence);
                    let _ = log.maintenance_step(&mut context);
                }
                if terminal {
                    break;
                }
            }
            assert!(log.expected_turn().is_none());
            let digests = std::array::from_fn(|index| log.record_header(index).expect("sealed record").prefix_digest);
            log.begin_close();
            while !log.terminal_is_empty() {
                let mut preview_sequence = 0;
                let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), BRIDGE_NOW_US, &mut preview_sequence);
                let _ = log.close_step(&mut context);
            }
            digests
        }

        let host_default = u16::try_from(std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get)).unwrap_or(u16::MAX);
        for worker_count in [1, 2, 4, host_default] {
            assert_eq!(run(worker_count, 0), run(worker_count, 0), "1/2/4/default worker replay preserves the exact ordered publication identity");
        }
    }

    #[test]
    fn mounted_replay_records_and_replays_the_exact_cancelled_terminal_classification() {
        let operation = bridge_operation();
        let route = JobReplayRoute { plugin: [11; 32], package: [13; 32], controller: [17; 32], tool: [19; 32], window: 23, document: [29; 32], request_schema: [31; 32], request_version: 1, request_digest: [37; 32] };
        let mut log = JobReplayLog::new(route, operation.generation.0).expect("fixed replay authority");
        let turn = bridge_turn(0, 0);
        for replaying in [false, true] {
            let publication = JobPublication { turn, outcome: JobStepOutcome::Cancelled };
            let mut sequence = 0;
            let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), BRIDGE_NOW_US, &mut sequence);
            log.begin_capture(&mut context, ActorId(9), JobReplaySchedule { worker_count: 1, worker_slot: 0, granted_fuel: 1, deadline_class_ms: 4 }, publication).expect("cancel capture admission");
            loop {
                let mut sequence = 0;
                let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), BRIDGE_NOW_US, &mut sequence);
                if log.capture_step(&mut context) == JobReplayStep::PublicationReady {
                    break;
                }
            }
            assert!(log.record_header(0).expect("cancel record").cancellation_observed);
            assert!(matches!(log.take_captured_publication().expect("exact cancelled owner").outcome, JobStepOutcome::Cancelled));
            let mut sequence = 0;
            let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), BRIDGE_NOW_US, &mut sequence);
            log.acknowledge_publication(&mut context, JobReplayPublicationPolicy::Accepted).expect("cancel ACK");
            if !replaying {
                log.begin_replay(operation.generation.0).expect("cancel replay generation");
            }
        }
        assert!(log.expected_turn().is_none());
        log.begin_close();
        while !log.terminal_is_empty() {
            let mut sequence = 0;
            let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), BRIDGE_NOW_US, &mut sequence);
            let _ = log.close_step(&mut context);
        }
    }

    #[test]
    fn mounted_replay_cancel_deadline_and_stale_refuse_the_exact_publication_owner_unchanged() {
        let operation = bridge_operation();
        let route = JobReplayRoute { plugin: [41; 32], package: [43; 32], controller: [47; 32], tool: [53; 32], window: 59, document: [61; 32], request_schema: [67; 32], request_version: 1, request_digest: [71; 32] };
        let mut log = JobReplayLog::new(route, operation.generation.0).expect("fixed replay authority");
        let publication = |generation: u64| JobPublication { turn: JobTurn { operation: JobOperation { generation, ..bridge_turn(0, 0).operation }, ..bridge_turn(0, 0) }, outcome: JobStepOutcome::PreviewReady { preview: vec![73, 79] } };

        let cancelled = publication(operation.generation.0);
        let cancelled_identity = match &cancelled.outcome {
            JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
            _ => unreachable!(),
        };
        let cancel = job::root_cancel_token();
        cancel.cancel_now();
        let mut sequence = 0;
        let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), cancel, BRIDGE_NOW_US, &mut sequence);
        let cancelled = log.begin_capture(&mut context, ActorId(9), JobReplaySchedule { worker_count: 1, worker_slot: 0, granted_fuel: 1, deadline_class_ms: 4 }, cancelled).expect_err("cancelled capture refuses before transfer").into_publication();
        assert!(matches!(&cancelled.outcome, JobStepOutcome::PreviewReady { preview } if preview.as_ptr() == cancelled_identity));

        let expired = publication(operation.generation.0);
        let expired_identity = match &expired.outcome {
            JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
            _ => unreachable!(),
        };
        let mut sequence = 0;
        let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 10), job::root_cancel_token(), BRIDGE_NOW_US, &mut sequence);
        let expired = log.begin_capture(&mut context, ActorId(9), JobReplaySchedule { worker_count: 1, worker_slot: 0, granted_fuel: 1, deadline_class_ms: 4 }, expired).expect_err("expired capture refuses before transfer").into_publication();
        assert!(matches!(&expired.outcome, JobStepOutcome::PreviewReady { preview } if preview.as_ptr() == expired_identity));

        let stale = publication(operation.generation.0 + 1);
        let stale_identity = match &stale.outcome {
            JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
            _ => unreachable!(),
        };
        let mut sequence = 0;
        let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), BRIDGE_NOW_US, &mut sequence);
        let stale = log.begin_capture(&mut context, ActorId(9), JobReplaySchedule { worker_count: 1, worker_slot: 0, granted_fuel: 1, deadline_class_ms: 4 }, stale).expect_err("stale capture refuses before transfer").into_publication();
        assert!(matches!(&stale.outcome, JobStepOutcome::PreviewReady { preview } if preview.as_ptr() == stale_identity));
        assert_eq!(log.sealed_records(), 0);
        assert!(!log.has_pending_work());
        log.begin_close();
        assert!(log.terminal_is_empty());
    }

    #[test]
    fn mounted_replay_preserves_the_exact_fault_payload_and_prefix_across_replay() {
        let operation = bridge_operation();
        let route = JobReplayRoute { plugin: [73; 32], package: [79; 32], controller: [83; 32], tool: [89; 32], window: 97, document: [101; 32], request_schema: [103; 32], request_version: 1, request_digest: [107; 32] };
        let mut log = JobReplayLog::new(route, operation.generation.0).expect("fixed replay authority");
        let turn = bridge_turn(0, 0);
        for replaying in [false, true] {
            let publication = JobPublication { turn, outcome: JobStepOutcome::Fault { detail: vec![109, 113, 127] } };
            let payload_identity = match &publication.outcome {
                JobStepOutcome::Fault { detail } => detail.as_ptr(),
                _ => unreachable!(),
            };
            let mut sequence = 0;
            let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), BRIDGE_NOW_US, &mut sequence);
            log.begin_capture(&mut context, ActorId(9), JobReplaySchedule { worker_count: 1, worker_slot: 0, granted_fuel: 1, deadline_class_ms: 4 }, publication).expect("fault capture admission");
            loop {
                let mut sequence = 0;
                let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), BRIDGE_NOW_US, &mut sequence);
                if log.capture_step(&mut context) == JobReplayStep::PublicationReady {
                    break;
                }
            }
            let publication = log.take_captured_publication().expect("exact fault owner");
            assert!(matches!(&publication.outcome, JobStepOutcome::Fault { detail } if detail.as_ptr() == payload_identity && detail.as_slice() == [109, 113, 127]));
            let mut sequence = 0;
            let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), BRIDGE_NOW_US, &mut sequence);
            log.acknowledge_publication(&mut context, JobReplayPublicationPolicy::Accepted).expect("fault ACK");
            if !replaying {
                log.begin_replay(operation.generation.0).expect("fault replay generation");
            }
        }
        assert!(log.expected_turn().is_none());
        log.begin_close();
        while !log.terminal_is_empty() {
            let mut sequence = 0;
            let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), BRIDGE_NOW_US, &mut sequence);
            let _ = log.close_step(&mut context);
        }
    }
    //#endregion 🪪️JobBridge

    //#region 🔖️PackRoundTrips
    macro_rules! round_trip {
        ($name:ident, $ty:ty, $value:expr) => {
            #[semio_framework_async_macros::async_test]
            async fn $name() {
                let value: $ty = $value;
                let mut bytes = Vec::new();
                value.pack_encode(&mut bytes).await;
                let mut pos = 0usize;
                let decoded = <$ty>::pack_decode(&bytes, &mut pos).await.unwrap();
                assert_eq!(pos, bytes.len());
                assert_eq!(decoded, value);
            }
        };
    }

    round_trip!(pack_round_trip_package_id, PackageId, PackageId("s.cad/extrude".into()));
    round_trip!(pack_round_trip_package_hash, PackageHash, PackageHash([7u8; 32]));
    round_trip!(pack_round_trip_actor_id, ActorId, ActorId::new(42, 1, 99, 3).await);
    round_trip!(pack_round_trip_actor_kind, ActorKind, ActorKind::PluginApp { plugin: PackageId("s.cad".into()), app_id: "s.cad.editor".into(), instance_id: 7 });
    round_trip!(pack_round_trip_lane, Lane, Lane::UserVisible);
    round_trip!(pack_round_trip_budget, Budget, lane_defaults::budget_for(Lane::Interactive));
    round_trip!(pack_round_trip_window_id, WindowId, WindowId(5));
    round_trip!(pack_round_trip_origin, Origin, Origin::Bus { topic: "os.runtime.metrics".into() });
    round_trip!(pack_round_trip_job_operation, JobOperation, JobOperation { operation: 11, base_revision: 7, generation: 3, preview_sequence: 2, seed: 99 });
    round_trip!(pack_round_trip_job_checkpoint, JobCheckpoint, JobCheckpoint { state: vec![9, 8, 7], applied_progress: 42 });
    round_trip!(pack_round_trip_job_step_outcome, JobStepOutcome, JobStepOutcome::Complete { candidate: JobCommitCandidate { state: vec![6, 5], output: vec![4, 3] } });
    round_trip!(
        pack_round_trip_job_publication,
        JobPublication,
        JobPublication {
            turn: JobTurn { job: 44, operation: JobOperation { operation: 11, base_revision: 7, generation: 3, preview_sequence: 1, seed: 99 }, step_sequence: 5 },
            outcome: JobStepOutcome::CheckpointReady { checkpoint: JobCheckpoint { state: vec![2, 1], applied_progress: 73 } },
        }
    );
    round_trip!(
        pack_round_trip_payload,
        Payload,
        Payload::Resume { operation: JobOperation { operation: 11, base_revision: 7, generation: 3, preview_sequence: 2, seed: 99 }, checkpoint: JobCheckpoint { state: vec![9, 9, 9], applied_progress: 42 } }
    );
    round_trip!(pack_round_trip_coalesce_key, CoalesceKey, CoalesceKey("pointer-move".into()));
    round_trip!(pack_round_trip_envelope, Envelope, env(ActorId::new(1, 0, 1, 0).await, Lane::Interactive, 7).await);
    round_trip!(pack_round_trip_turn_status, TurnStatus, TurnStatus::Faulted { detail: vec![1, 2] });
    round_trip!(pack_round_trip_usage, Usage, Usage { fuel: 1, wall_us: 2, memory_bytes: 3 });
    #[semio_framework_async_macros::async_test]
    async fn pack_round_trip_turn_result() {
        let value = ok_turn().await;
        let mut bytes = Vec::new();
        value.pack_encode(&mut bytes).await.expect("turn result encoding succeeds");
        let mut pos = 0;
        let decoded = TurnResult::pack_decode(&bytes, &mut pos).await.expect("turn result decoding succeeds");
        assert_eq!(pos, bytes.len());
        assert_eq!(decoded, value);
    }
    round_trip!(pack_round_trip_backpressure, Backpressure, Backpressure::Dropped { lane: Lane::Background });
    round_trip!(pack_round_trip_capability_grant, CapabilityGrant, CapabilityGrant { capability: "fs.read".into(), scope: Some(vec![1]) });
    round_trip!(pack_round_trip_failure_signal, FailureSignal, FailureSignal::HeartbeatMissed { count: 2 });
    round_trip!(pack_round_trip_failure_stage, FailureStage, FailureStage::Throttled { factor: 0.25 });
    round_trip!(pack_round_trip_failure_state, FailureState, FailureState { stage: FailureStage::Warned, clean_turns: 1, warn_count: 2, restart_count: 0, last_signal_ms: 500 });
    round_trip!(pack_round_trip_actor_status, ActorStatus, ActorStatus::Suspended { checkpoint: Some(vec![1, 2, 3]) });
    round_trip!(pack_round_trip_shard_id, ShardId, ShardId(3));
    round_trip!(pack_round_trip_shard_kind, ShardKind, ShardKind::WebWorker);
    round_trip!(pack_round_trip_decision, Decision, Decision { run: vec![TurnGrant { actor: ActorId::new(1, 0, 0, 0).await, shard: ShardId(0), budget: lane_defaults::budget_for(Lane::Background), envelopes: vec![] }], wake_at: Some(10) });
    round_trip!(
        pack_round_trip_turn_grant,
        TurnGrant,
        TurnGrant { actor: ActorId::new(2, 1, 3, 0).await, shard: ShardId(1), budget: lane_defaults::budget_for(Lane::Maintenance), envelopes: vec![env(ActorId::new(2, 1, 3, 0).await, Lane::Maintenance, 1).await] }
    );
    round_trip!(pack_round_trip_scene_snapshot, SceneSnapshot, SceneSnapshot { revision: 3, committed_ms: 12, patches: vec![9, 9], node_count: 40 });
    round_trip!(pack_round_trip_shard_metrics, ShardMetrics, ShardMetrics { actors: 3, busy_ratio: 0.5, heartbeat_age_ms: 12 });
    round_trip!(pack_round_trip_kernel_metrics, KernelMetrics, KernelMetrics { actors: 3, shards: 4, packages: 2 });
    round_trip!(
        pack_round_trip_actor_metrics_sample,
        ActorMetricsSample,
        ActorMetricsSample { id: ActorId::new(1, 0, 2, 0).await, package: PackageId("s.cad".into()), lane: Lane::UserVisible, status: ActorStatus::Active, metrics: ActorMetrics::default() }
    );
    round_trip!(pack_round_trip_shard_metrics_sample, ShardMetricsSample, ShardMetricsSample { shard: ShardId(2), metrics: ShardMetrics { actors: 5, busy_ratio: 0.4, heartbeat_age_ms: 8 } });
    round_trip!(
        pack_round_trip_runtime_metrics_snapshot,
        RuntimeMetricsSnapshot,
        RuntimeMetricsSnapshot {
            kernel: KernelMetrics { actors: 1, shards: 1, packages: 1 },
            actors: vec![ActorMetricsSample { id: ActorId::new(0, 0, 0, 0).await, package: PackageId("s.a".into()), lane: Lane::Interactive, status: ActorStatus::Active, metrics: ActorMetrics::default() }],
            shards: vec![ShardMetricsSample { shard: ShardId(0), metrics: ShardMetrics { actors: 1, busy_ratio: 1.0, heartbeat_age_ms: 0 } }],
            sampled_at_ms: 1234,
        }
    );

    #[semio_framework_async_macros::async_test]
    async fn pack_round_trip_mailbox() {
        let mut mailbox = Mailbox::new(4).await;
        mailbox.enqueue(env(ActorId::new(1, 0, 0, 0).await, Lane::Interactive, 1).await).await;
        mailbox.enqueue(env(ActorId::new(1, 0, 0, 0).await, Lane::Background, 2).await).await;
        let mut bytes = Vec::new();
        mailbox.pack_encode(&mut bytes).await;
        let mut pos = 0usize;
        let decoded = Mailbox::pack_decode(&bytes, &mut pos).await.unwrap();
        assert_eq!(pos, bytes.len());
        assert_eq!(decoded.len(), mailbox.len());
        assert_eq!(decoded.capacity, mailbox.capacity);
    }

    #[semio_framework_async_macros::async_test]
    async fn pack_round_trip_actor_record() {
        let mut kernel = Kernel::new(ShardKind::Native, 4, 1, 4).await;
        let id = kernel.activate(PackageId("s.cad".into()), 1, ActorKind::PluginApp { plugin: PackageId("s.cad".into()), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
        let record = kernel.actor_record(id).await.unwrap();
        let mut bytes = Vec::new();
        record.pack_encode(&mut bytes).await;
        let mut pos = 0usize;
        let decoded = ActorRecord::pack_decode(&bytes, &mut pos).await.unwrap();
        assert_eq!(pos, bytes.len());
        assert_eq!(decoded, record);
    }

    /// 📌️ The property the old `actor.0 % pool` silently violated: 100 actors of one plugin
    /// must SPREAD across the pool, not all land on shard 0. Every actor `activate` mints has
    /// `generation == 0`, and generation occupied the low bits, so the old modulo returned 0 for
    /// all of them — a bench-measured `perShardCounts {"0": 100}`. Asserts distribution, not
    /// mere validity: a "pin returns a shard in range" test passes when every answer is 0.
    #[semio_framework_async_macros::async_test]
    async fn pin_spreads_actors_of_one_plugin_across_the_pool() {
        let mut table = ShardTable::new(ShardKind::Native, 8, 0).await;
        let mut counts: std::collections::BTreeMap<u16, usize> = std::collections::BTreeMap::new();
        for ordinal in 0..100u32 {
            *counts.entry(table.pin(ActorId::new(7, 0, ordinal, 0).await).await.0).or_default() += 1;
        }
        assert_eq!(counts.values().sum::<usize>(), 100);
        assert_eq!(counts.len(), 8, "all 8 shards must receive actors, got {counts:?}");
        assert!(*counts.values().max().unwrap() <= 100 / 8 + 1, "no shard may exceed ceil(100/8)+1: {counts:?}");
    }

    /// 🔁️ Pinning the same actor twice is idempotent — a re-pin must not consume a second slot
    /// and skew the balance.
    #[semio_framework_async_macros::async_test]
    async fn pin_is_idempotent_for_the_same_actor() {
        let mut table = ShardTable::new(ShardKind::Native, 8, 0).await;
        let actor = ActorId::new(3, 0, 11, 0).await;
        assert_eq!(table.pin(actor).await, table.pin(actor).await);
    }

    /// 🕳️ `unpin` leaves a gap; the next `pin` must refill it rather than stride past, which is
    /// exactly what a round-robin counter would do and why placement is least-loaded.
    #[semio_framework_async_macros::async_test]
    async fn pin_refills_the_gap_left_by_unpin() {
        let mut table = ShardTable::new(ShardKind::Native, 4, 0).await;
        let mut actors: Vec<ActorId> = Vec::new();
        for ordinal in 0..8u32 {
            actors.push(ActorId::new(1, 0, ordinal, 0).await);
        }
        for actor in &actors {
            table.pin(*actor).await;
        }
        let freed = table.shard_of(actors[2]).await.unwrap();
        table.unpin(actors[2]).await;
        assert_eq!(table.pin(ActorId::new(1, 0, 99, 0).await).await, freed);
    }

    /// 🔥️ PROPERTY (terra-interactive-isolation): the mechanism this packet's mission asked for —
    /// N CPU-saturating actors sharing a shard must not receive a freshly-activated interactive
    /// actor as a co-resident. Pure/deterministic: injected `Usage`s stand in for real wall-clock
    /// turns (`ActorMetrics::is_saturating`'s whole point is never needing a clock/bench of its
    /// own), no thread/bench needed.
    #[semio_framework_async_macros::async_test]
    async fn interactive_actor_avoids_a_shard_saturated_by_cpu_bound_actors() {
        let mut kernel = Kernel::new(ShardKind::Native, 3, 0, 64).await;
        let background_package = PackageId("cpu-hog".into());
        // 🔢️ 6 Background actors round-robin 2-per-shard across 3 shards under plain least-loaded
        // `pin` (proven deterministic by `pin_spreads_actors_of_one_plugin_across_the_pool`'s own
        // reasoning) — read each one's ACTUAL shard back via `actor_record` rather than assuming
        // the exact order, so this test stays correct even if that internal tie-break ever changes.
        let mut by_shard: std::collections::BTreeMap<u16, Vec<ActorId>> = std::collections::BTreeMap::new();
        for ordinal in 0..6u32 {
            let id = kernel.activate(background_package.clone(), 1, ActorKind::PluginApp { plugin: background_package.clone(), app_id: "hog".into(), instance_id: ordinal }, Lane::Background, None, ActivationEvent::Manual).await;
            let shard = kernel.actor_record(id).await.unwrap().shard;
            by_shard.entry(shard.0).or_default().push(id);
        }
        assert_eq!(by_shard.len(), 3, "expected all 3 shards to receive background actors: {by_shard:?}");

        // 🔥️ Drive every actor on shards OTHER than the last one over its own Background budget's
        // wall_ms ceiling for 2 turns (SATURATION_MIN_TURNS) — crossing `is_saturating`'s
        // threshold. The last shard's actors are left untouched (default `ActorMetrics`, zero
        // turns), making it the one and only "clean" shard.
        let shard_ids: Vec<u16> = by_shard.keys().copied().collect();
        let (safe_shard, hot_shards) = shard_ids.split_last().unwrap();
        let hot_turn = TurnResult {
            ui_patches: vec![],
            effects: vec![],
            command_ingress: vec![],
            cold_pair_ingress: Default::default(),
            lifecycle_receipt: None,
            ui_patch_receipt: None,
            next_wake: None,
            status: TurnStatus::Idle,
            usage: Usage { fuel: 100, wall_us: 40_000, memory_bytes: 1024 },
        };
        for shard in hot_shards {
            for actor in &by_shard[shard] {
                kernel.complete(*actor, &hot_turn, 0).await.unwrap();
                kernel.complete(*actor, &hot_turn, 0).await.unwrap();
            }
        }

        let interactive_id = kernel.activate(PackageId("editor".into()), 2, ActorKind::PluginApp { plugin: PackageId("editor".into()), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
        let interactive_shard = kernel.actor_record(interactive_id).await.unwrap().shard;
        assert_eq!(interactive_shard.0, *safe_shard, "interactive actor must land on the one shard with no CPU-saturating co-resident, got {interactive_shard:?} (hot shards: {hot_shards:?})");
    }

    #[semio_framework_async_macros::async_test]
    async fn pack_round_trip_shard_table() {
        let mut table = ShardTable::new(ShardKind::Native, 4, 1).await;
        let actor = ActorId::new(1, 0, 0, 0).await;
        table.pin(actor).await;
        table.request_exclusive(ActorId::new(2, 0, 0, 0).await).await;
        let mut bytes = Vec::new();
        table.pack_encode(&mut bytes).await;
        let mut pos = 0usize;
        let decoded = ShardTable::pack_decode(&bytes, &mut pos).await.unwrap();
        assert_eq!(pos, bytes.len());
        assert_eq!(decoded.shard_of(actor).await, table.shard_of(actor).await);
    }

    #[semio_framework_async_macros::async_test]
    async fn pack_round_trip_actor_metrics() {
        let mut metrics = ActorMetrics::default();
        for i in 0..70u64 {
            metrics.record_turn(&Usage { fuel: i, wall_us: i * 3, memory_bytes: 10 }).await;
        }
        let mut bytes = Vec::new();
        metrics.pack_encode(&mut bytes).await;
        let mut pos = 0usize;
        let decoded = ActorMetrics::pack_decode(&bytes, &mut pos).await.unwrap();
        assert_eq!(pos, bytes.len());
        assert_eq!(decoded, metrics);
        assert_eq!(decoded.wall_us_p95(), metrics.wall_us_p95());
    }
    //#endregion 🔖️PackRoundTrips

    //#region 🔖️SerdeRoundTrips
    /// 🎯️ terra-shard-grants, Part A. `📌️important.md` rule 12: "after fixing one variant of a
    /// serde-shape defect, sweep every sibling" — the `JobStep::Done`/`Failed` fix recorded this
    /// instruction and nobody executed it for the six siblings sitting in this crate. Each of
    /// these SERIALIZES TO BYTES AND BACK (`serde_json`, not an in-process `assert_eq!` on the
    /// original value) — that distinction is the entire point: a plain equality check on the
    /// original Rust value would pass even if `serde_json::to_vec` panics or errors partway,
    /// exactly how the `JobStep` bug hid for a full wave (its tests only ever asserted on
    /// in-process values, never on bytes that had crossed a serde boundary).
    macro_rules! serde_round_trip {
        ($name:ident, $ty:ty, $value:expr) => {
            #[semio_framework_async_macros::async_test]
            async fn $name() {
                let value: $ty = $value;
                let bytes = serde_json::to_vec(&value).expect("serde_json::to_vec must not error — this is the exact defect this test exists to catch");
                let decoded: $ty = serde_json::from_slice(&bytes).expect("serde_json::from_slice must round-trip what to_vec produced");
                assert_eq!(decoded, value);
            }
        };
    }

    serde_round_trip!(serde_round_trip_payload_event, Payload, Payload::Event { bytes: vec![1, 2, 3] });
    serde_round_trip!(serde_round_trip_payload_cancel, Payload, Payload::Cancel { seq: 42 });
    serde_round_trip!(serde_round_trip_origin_actor, Origin, Origin::Actor { id: ActorId::new(1, 0, 2, 0).await });
    serde_round_trip!(serde_round_trip_turn_status_faulted, TurnStatus, TurnStatus::Faulted { detail: b"boom".to_vec() });
    serde_round_trip!(serde_round_trip_failure_signal_trap, FailureSignal, FailureSignal::Trap { detail: "trapped".to_string() });
    serde_round_trip!(serde_round_trip_backpressure_dropped, Backpressure, Backpressure::Dropped { lane: Lane::Background });
    //#endregion 🔖️SerdeRoundTrips

    //#region 🔖️ValueRoundTrips
    /// 🌱️ RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS (26/09/01): the additive
    /// `#[derive(ToValue, FromValue)]` pass over this crate's serde types — proves the new trait
    /// actually round-trips at runtime (not merely type-checks) for a representative spread:
    /// a `#[value(transparent)]` newtype, a plain struct, a `tag`-carrying enum, the hand-written
    /// `ActorInstanceLifecycleReceipt` impl (enum-variant `with` fields the derive rejects), and
    /// `ShardTable`'s custom stringified-key `BTreeMap` bridge.
    macro_rules! value_round_trip {
        ($name:ident, $ty:ty, $value:expr) => {
            #[semio_framework_async_macros::async_test]
            async fn $name() {
                let value: $ty = $value;
                let encoded = ::protocol::value::ToValue::to_value(&value);
                let decoded: $ty = ::protocol::value::FromValue::from_value(encoded).expect("FromValue must round-trip what ToValue produced");
                assert_eq!(decoded, value);
            }
        };
    }

    value_round_trip!(value_round_trip_actor_id, ActorId, ActorId::new(1, 0, 2, 0).await);
    value_round_trip!(value_round_trip_package_id, PackageId, PackageId("semio.demo".to_string()));
    value_round_trip!(value_round_trip_budget, Budget, lane_defaults::budget_for(Lane::Interactive));
    value_round_trip!(value_round_trip_origin_actor, Origin, Origin::Actor { id: ActorId::new(1, 0, 2, 0).await });
    value_round_trip!(value_round_trip_payload_event, Payload, Payload::Event { bytes: vec![1, 2, 3] });
    value_round_trip!(value_round_trip_failure_stage_throttled, FailureStage, FailureStage::Throttled { factor: 0.5 });

    #[semio_framework_async_macros::async_test]
    async fn value_round_trip_actor_instance_lifecycle_receipt() {
        let value = instance_lifetime::ActorInstanceLifecycleReceipt::Accepted { lifetime: instance_lifetime::ActorInstanceLifetime { activation_generation: 7, instance_id: 3, guest_lifetime: 9 }, request_sequence: 11, close_generation: 5 };
        let encoded = ::protocol::value::ToValue::to_value(&value);
        assert_eq!(encoded["kind"], ::protocol::value::DslValue::String("accepted".to_string()));
        assert_eq!(encoded["requestSequence"], ::protocol::value::DslValue::uint(11));
        assert_eq!(encoded["closeGeneration"], ::protocol::value::DslValue::String("5".to_string()));
        let decoded: instance_lifetime::ActorInstanceLifecycleReceipt = ::protocol::value::FromValue::from_value(encoded).unwrap();
        assert_eq!(decoded, value);
    }

    #[semio_framework_async_macros::async_test]
    async fn value_round_trip_shard_table_stringified_keys() {
        let mut table = ShardTable::new(ShardKind::Native, 4, 1).await;
        table.pin(ActorId::new(1, 0, 2, 0).await).await;
        let encoded = ::protocol::value::ToValue::to_value(&table);
        let ::protocol::value::DslValue::Object(entries) = &encoded else { panic!("expected object") };
        let (_, assignment) = entries.iter().find(|(k, _)| k == "assignment").expect("assignment key present");
        let ::protocol::value::DslValue::Object(assignment_entries) = assignment else { panic!("expected object") };
        assert!(assignment_entries.iter().all(|(k, _)| k.parse::<u64>().is_ok()), "keys must be stringified ActorId integers");
        let decoded: ShardTable = ::protocol::value::FromValue::from_value(encoded).unwrap();
        assert_eq!(decoded.shard_count().await, table.shard_count().await);
    }
    //#endregion 🔖️ValueRoundTrips

    //#region 🔖️ActorIdBitPacking
    #[semio_framework_async_macros::async_test]
    async fn actor_id_bit_packing_round_trips_all_fields() {
        let id = ActorId::new(0xBEEF, 2, 0xC0FFEE, 0x1234 & 0x3FFF).await;
        assert_eq!(id.plugin_ordinal(), 0xBEEF);
        assert_eq!(id.kind_tag(), 2);
        assert_eq!(id.ordinal(), 0xC0FFEE);
        assert_eq!(id.generation(), 0x1234 & 0x3FFF);
    }

    #[semio_framework_async_macros::async_test]
    async fn actor_id_next_generation_bumps_only_generation() {
        let id = ActorId::new(3, 1, 9, 5).await;
        let restarted = id.next_generation().await;
        assert_eq!(restarted.plugin_ordinal(), id.plugin_ordinal());
        assert_eq!(restarted.kind_tag(), id.kind_tag());
        assert_eq!(restarted.ordinal(), id.ordinal());
        assert_eq!(restarted.generation(), id.generation() + 1);
    }
    //#endregion 🔖️ActorIdBitPacking

    //#region 🔖️MailboxTests
    #[semio_framework_async_macros::async_test]
    async fn mailbox_coalesces_latest_wins_older_dropped() {
        let mut mailbox = Mailbox::new(10).await;
        let actor = ActorId::new(1, 0, 0, 0).await;
        for i in 0..200u64 {
            let mut e = env(actor, Lane::Interactive, i).await;
            e.coalesce = Some(CoalesceKey("pointer-move".into()));
            e.payload = Payload::Event { bytes: vec![i as u8] };
            let bp = mailbox.enqueue(e).await;
            assert!(matches!(bp, Backpressure::Accept | Backpressure::Coalesced));
        }
        assert_eq!(mailbox.len(), 1, "200 coalesced moves must never queue more than the latest");
        let popped = mailbox.pop_next().await.unwrap();
        assert_eq!(popped.payload, Payload::Event { bytes: vec![199] });
    }

    #[semio_framework_async_macros::async_test]
    async fn mailbox_backpressure_rejected_when_full_and_nothing_lower_priority() {
        let mut mailbox = Mailbox::new(2).await;
        let actor = ActorId::new(1, 0, 0, 0).await;
        assert_eq!(mailbox.enqueue(env(actor, Lane::Maintenance, 1).await).await, Backpressure::Accept);
        assert_eq!(mailbox.enqueue(env(actor, Lane::Maintenance, 2).await).await, Backpressure::Accept);
        let bp = mailbox.enqueue(env(actor, Lane::Maintenance, 3).await).await;
        assert_eq!(bp, Backpressure::Rejected, "no lower-priority lane to evict — must reject, never silently drop");
    }

    #[semio_framework_async_macros::async_test]
    async fn mailbox_backpressure_drops_lower_priority_lane_to_admit_interactive() {
        let mut mailbox = Mailbox::new(2).await;
        let actor = ActorId::new(1, 0, 0, 0).await;
        assert_eq!(mailbox.enqueue(env(actor, Lane::Maintenance, 1).await).await, Backpressure::Accept);
        assert_eq!(mailbox.enqueue(env(actor, Lane::Background, 2).await).await, Backpressure::Accept);
        let bp = mailbox.enqueue(env(actor, Lane::Interactive, 3).await).await;
        assert_eq!(bp, Backpressure::Dropped { lane: Lane::Maintenance });
        assert_eq!(mailbox.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn mailbox_pop_next_honors_lane_priority_over_fifo() {
        let mut mailbox = Mailbox::new(10).await;
        let actor = ActorId::new(1, 0, 0, 0).await;
        mailbox.enqueue(env(actor, Lane::Maintenance, 1).await).await;
        mailbox.enqueue(env(actor, Lane::Background, 2).await).await;
        mailbox.enqueue(env(actor, Lane::Interactive, 3).await).await;
        assert_eq!(mailbox.pop_next().await.unwrap().lane, Lane::Interactive);
        assert_eq!(mailbox.pop_next().await.unwrap().lane, Lane::Background);
        assert_eq!(mailbox.pop_next().await.unwrap().lane, Lane::Maintenance);
    }
    //#endregion 🔖️MailboxTests

    //#region 🔖️SchedulerFairness
    #[semio_framework_async_macros::async_test]
    async fn drr_fairness_plugin_with_50_actors_does_not_starve_plugin_with_1() {
        // 🧪️ Both plugins are given abundant backlog up front (nobody runs dry mid-test), so the
        // measured split reflects level-1 DRR's PLUGIN-level fairness, not which plugin happened
        // to have offered work left when the other ran out. Without level 1, `s.busy` (50 actors)
        // would swamp `s.quiet` (1 actor) roughly 50:1 — see design §Scheduler.
        let mut scheduler = Scheduler::new(4).await;
        let busy_package = PackageId("s.busy".into());
        let quiet_package = PackageId("s.quiet".into());
        let budget = lane_defaults::budget_for(Lane::Background);
        let mut busy_actors = Vec::new();
        for i in 0..50u32 {
            let id = ActorId::new(1, 0, i, 0).await;
            scheduler.register_actor(id, busy_package.clone(), Lane::Background, budget, ShardId(0)).await;
            busy_actors.push(id);
        }
        let quiet_actor = ActorId::new(2, 0, 0, 0).await;
        scheduler.register_actor(quiet_actor, quiet_package, Lane::Background, budget, ShardId(1)).await;

        for &id in &busy_actors {
            for seq in 0..30u64 {
                scheduler.submit(Envelope { to: id, from: Origin::Kernel, lane: Lane::Background, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: vec![] } }).await;
            }
        }
        for seq in 0..500u64 {
            scheduler.submit(Envelope { to: quiet_actor, from: Origin::Kernel, lane: Lane::Background, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: vec![] } }).await;
        }

        let mut busy_grants = 0u32;
        let mut quiet_grants = 0u32;
        for now in 0..100u64 {
            let decision = scheduler.tick(now).await;
            for grant in &decision.run {
                if grant.actor == quiet_actor {
                    quiet_grants += 1;
                } else {
                    busy_grants += 1;
                }
            }
        }
        assert!(quiet_grants > 0, "the 1-actor plugin must never starve");
        assert!(busy_grants > 0);
        let ratio = busy_grants as f64 / quiet_grants as f64;
        assert!(ratio < 10.0, "level-1 DRR must keep PLUGIN-level share roughly comparable regardless of actor count (busy={busy_grants} quiet={quiet_grants} ratio={ratio}, naive per-actor scheduling would give ~50x)");
    }

    #[semio_framework_async_macros::async_test]
    async fn deadline_preemption_runs_before_background_drr_deficit() {
        let mut scheduler = Scheduler::new(1).await;
        let package = PackageId("s.mixed".into());
        let budget = lane_defaults::budget_for(Lane::Background);
        let bg_actor = ActorId::new(1, 0, 0, 0).await;
        let interactive_actor = ActorId::new(1, 0, 1, 0).await;
        scheduler.register_actor(bg_actor, package.clone(), Lane::Background, budget, ShardId(0)).await;
        scheduler.register_actor(interactive_actor, package, Lane::Interactive, lane_defaults::budget_for(Lane::Interactive), ShardId(0)).await;
        scheduler.submit(Envelope { to: bg_actor, from: Origin::Kernel, lane: Lane::Background, seq: 1, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: vec![] } }).await;
        scheduler.submit(Envelope { to: interactive_actor, from: Origin::Kernel, lane: Lane::Interactive, seq: 2, deadline_ms: Some(5), coalesce: None, cancel_of: None, payload: Payload::Event { bytes: vec![] } }).await;
        let decision = scheduler.tick(10).await;
        assert_eq!(decision.run.len(), 1);
        assert_eq!(decision.run[0].actor, interactive_actor, "an overdue interactive deadline must preempt DRR ordering");
    }
    //#endregion 🔖️SchedulerFairness

    //#region 🔖️FailureLadder
    #[semio_framework_async_macros::async_test]
    async fn failure_ladder_escalates_and_decays_back_to_healthy() {
        let mut state = FailureState::new();
        assert_eq!(state.stage, FailureStage::Healthy);

        let esc = state.on_signal(&FailureSignal::DeadlineOverrun { ratio: 1.2 }, Lane::Interactive, 0).await;
        assert_eq!(esc, FailureEscalation::None);
        assert_eq!(state.stage, FailureStage::Warned);

        let esc = state.on_signal(&FailureSignal::DeadlineOverrun { ratio: 1.4 }, Lane::Interactive, 10).await;
        assert_eq!(esc, FailureEscalation::None);
        assert!(matches!(state.stage, FailureStage::Throttled { .. }), "second interactive warn must throttle (threshold=2)");

        let esc = state.on_signal(&FailureSignal::MailboxOverflow, Lane::Interactive, 20).await;
        assert_eq!(esc, FailureEscalation::None);
        assert!(matches!(state.stage, FailureStage::Suspended { .. }), "third interactive warn must suspend (threshold=2 -> suspend_at)");

        for i in 0..200u64 {
            state.on_clean_turn(1_000_000 + i).await;
        }
        assert_eq!(state.stage, FailureStage::Healthy, "sustained clean turns must decay all the way back to Healthy, got {:?}", state.stage);
    }

    #[semio_framework_async_macros::async_test]
    async fn failure_ladder_trap_then_quarantine_is_package_wide() {
        let mut kernel = Kernel::new(ShardKind::Native, 4, 1, 4).await;
        let package = PackageId("s.flaky".into());
        let a = kernel.activate(package.clone(), 1, ActorKind::PluginApp { plugin: package.clone(), app_id: "a".into(), instance_id: 0 }, Lane::Background, None, ActivationEvent::Manual).await;
        let b = kernel.activate(package.clone(), 1, ActorKind::PluginApp { plugin: package, app_id: "b".into(), instance_id: 1 }, Lane::Background, None, ActivationEvent::Manual).await;

        for i in 0..FAILURE_QUARANTINE_RESTART_THRESHOLD {
            let faulted = TurnResult {
                ui_patches: vec![],
                effects: vec![],
                command_ingress: vec![],
                cold_pair_ingress: Default::default(),
                lifecycle_receipt: None,
                ui_patch_receipt: None,
                next_wake: None,
                status: TurnStatus::Faulted { detail: b"boom".to_vec() },
                usage: Usage::default(),
            };
            kernel.complete(a, &faulted, (i as u64) * 100).await.unwrap();
        }
        assert_eq!(kernel.actor_status(a).await, Some(&ActorStatus::Quarantined));
        assert_eq!(kernel.actor_status(b).await, Some(&ActorStatus::Quarantined), "quarantine must be package-wide, not just the trapping actor");
    }

    #[semio_framework_async_macros::async_test]
    async fn failure_ladder_manual_reset_returns_to_healthy_immediately() {
        let mut state = FailureState::new();
        state.on_signal(&FailureSignal::FuelExhausted, Lane::Background, 0).await;
        assert_ne!(state.stage, FailureStage::Healthy);
        state.on_signal(&FailureSignal::ManualReset, Lane::Background, 1).await;
        assert_eq!(state.stage, FailureStage::Healthy);
        assert_eq!(state.warn_count, 0);
    }
    //#endregion 🔖️FailureLadder

    //#region 🔖️Scene
    #[semio_framework_async_macros::async_test]
    async fn scene_revision_is_monotonic_and_reuses_snapshot_on_empty_commit() {
        let mut store = SceneStore::new();
        let budget = lane_defaults::budget_for(Lane::Interactive);
        let actor = ActorId::new(1, 0, 0, 0).await;
        let first = store.commit_frame(0).await;
        assert_eq!(first.revision, 0, "no pending patches yet -> initial empty snapshot, revision 0");

        store.apply_patch(actor, vec![1, 2, 3], 10, &budget).await.unwrap();
        let second = store.commit_frame(16).await;
        assert_eq!(second.revision, 1);
        assert!(second.committed_ms >= first.committed_ms);

        let third = store.commit_frame(32).await;
        assert_eq!(third.revision, second.revision, "nothing pending -> previous snapshot reused, same revision");
        assert!(Arc::ptr_eq(&second, &third));
    }

    #[semio_framework_async_macros::async_test]
    async fn scene_ui_node_quota_truncates_and_signals() {
        let mut store = SceneStore::new();
        let mut budget = lane_defaults::budget_for(Lane::Interactive);
        budget.ui_nodes = 100;
        let actor = ActorId::new(1, 0, 0, 0).await;
        let err = store.apply_patch(actor, vec![1], 150, &budget).await.unwrap_err();
        assert_eq!(err, FailureSignal::UiQuota);
        let snapshot = store.commit_frame(0).await;
        assert_eq!(snapshot.node_count, 100, "node count must be truncated to the budget ceiling, never exceed it");
    }

    #[semio_framework_async_macros::async_test]
    async fn scene_max_patch_bytes_rejects_oversized_patch() {
        let mut store = SceneStore::new();
        let mut budget = lane_defaults::budget_for(Lane::Interactive);
        budget.max_patch_bytes = 4;
        let actor = ActorId::new(1, 0, 0, 0).await;
        let err = store.apply_patch(actor, vec![0; 100], 1, &budget).await.unwrap_err();
        assert_eq!(err, FailureSignal::UiQuota);
    }
    //#endregion 🔖️Scene

    //#region 🔖️ThreadTransport
    #[semio_framework_async_macros::async_test]
    #[cfg(not(target_arch = "wasm32"))]
    async fn thread_transport_duplex_send_recv_and_heartbeat() {
        let (kernel_side, shard_side) = ThreadTransport::new_pair().await;
        kernel_side.send(b"to-shard").await;
        assert_eq!(shard_side.recv().await, Some(b"to-shard".to_vec()));
        shard_side.send(b"to-kernel").await;
        assert_eq!(kernel_side.recv().await, Some(b"to-kernel".to_vec()));
        shard_side.beat(42).await;
        assert_eq!(kernel_side.heartbeat().await, 42);
    }

    #[semio_framework_async_macros::async_test]
    #[cfg(not(target_arch = "wasm32"))]
    async fn thread_transport_kill_stops_recv() {
        let (kernel_side, shard_side) = ThreadTransport::new_pair().await;
        kernel_side.send(b"queued-before-kill").await;
        kernel_side.kill().await;
        assert_eq!(shard_side.recv().await, None, "a killed transport must never yield a stale message");
    }

    /// 🎯️ terra-shard-grants requirement: `recv_deadline` must return `None` on a genuine
    /// timeout (nothing ever sent) rather than blocking forever — proven with a short, bounded
    /// deadline so the test itself cannot hang. "Spawns no thread" is a property of the
    /// IMPLEMENTATION (`mpsc::Receiver::recv_timeout`, which blocks only the calling thread —
    /// see [`ThreadTransport::recv_deadline`]'s own doc), verified by code review plus the
    /// crate-wide purity grep this ticket already runs (`std::thread` must match only the
    /// header doc comment across the whole file) — deliberately NOT re-proven here via
    /// `std::thread::current()`, which would itself add a real (non-doc-comment) `std::thread`
    /// use to this file and defeat the very grep this test is meant to keep passing.
    #[semio_framework_async_macros::async_test]
    #[cfg(not(target_arch = "wasm32"))]
    async fn recv_deadline_returns_none_on_timeout() {
        let (_kernel_side, shard_side) = ThreadTransport::new_pair().await;
        let result = shard_side.recv_deadline(std::time::Duration::from_millis(20)).await;
        assert_eq!(result, None, "nothing was ever sent — a timeout must yield None, not block forever");
    }

    /// 🎯️ The complementary case: a message sent before the deadline must still be delivered —
    /// `recv_deadline` is a bounded wait, not merely a disguised `recv()` that always returns
    /// `None` until timeout.
    #[semio_framework_async_macros::async_test]
    #[cfg(not(target_arch = "wasm32"))]
    async fn recv_deadline_returns_the_message_when_one_arrives_before_the_timeout() {
        let (kernel_side, shard_side) = ThreadTransport::new_pair().await;
        kernel_side.send(b"before-deadline").await;
        assert_eq!(shard_side.recv_deadline(std::time::Duration::from_millis(200)).await, Some(b"before-deadline".to_vec()));
    }

    /// 🛑️ Mirrors `thread_transport_kill_stops_recv` for the blocking variant — a killed
    /// transport must return `None` immediately, never wait out the full deadline.
    #[semio_framework_async_macros::async_test]
    #[cfg(not(target_arch = "wasm32"))]
    async fn recv_deadline_returns_none_immediately_on_a_killed_transport() {
        let (kernel_side, shard_side) = ThreadTransport::new_pair().await;
        kernel_side.send(b"queued-before-kill").await;
        kernel_side.kill().await;
        assert_eq!(shard_side.recv_deadline(std::time::Duration::from_millis(20)).await, None, "a killed transport must never yield a stale message");
    }
    //#endregion 🔖️ThreadTransport

    //#region 🔖️KernelFacade
    #[semio_framework_async_macros::async_test]
    async fn kernel_activate_submit_tick_complete_round_trip() {
        let mut kernel = Kernel::new(ShardKind::Native, 4, 1, 4).await;
        let window = WindowId(1);
        let actor = kernel.activate(PackageId("s.cad".into()), 1, ActorKind::PluginApp { plugin: PackageId("s.cad".into()), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, Some(window), ActivationEvent::WindowOpen { window }).await;
        let bp = kernel.submit(&env(actor, Lane::Interactive, 1).await).await;
        assert_eq!(bp, Backpressure::Accept);
        let decision = kernel.tick(0).await;
        assert_eq!(decision.run.len(), 1);
        assert_eq!(decision.run[0].actor, actor);
        let escalation = kernel.complete(actor, &ok_turn().await, 1).await.unwrap();
        assert_eq!(escalation, FailureEscalation::None);
        assert_eq!(kernel.actor_status(actor).await, Some(&ActorStatus::Active));
    }

    #[semio_framework_async_macros::async_test]
    async fn kernel_suspend_resume_round_trip() {
        let mut kernel = Kernel::new(ShardKind::Native, 4, 1, 4).await;
        let actor = kernel.activate(PackageId("s.cad".into()), 1, ActorKind::Extension { plugin: PackageId("s.cad".into()), extension_id: "e1".into() }, Lane::Background, None, ActivationEvent::Manual).await;
        kernel.suspend(actor, Some(vec![1, 2, 3])).await.unwrap();
        assert_eq!(kernel.actor_status(actor).await, Some(&ActorStatus::Suspended { checkpoint: Some(vec![1, 2, 3]) }));
        kernel.resume(actor).await.unwrap();
        assert_eq!(kernel.actor_status(actor).await, Some(&ActorStatus::Active));
    }

    #[semio_framework_async_macros::async_test]
    async fn kernel_request_exclusive_then_release() {
        let mut kernel = Kernel::new(ShardKind::Native, 4, 1, 4).await;
        let actor = kernel.activate(PackageId("s.cad".into()), 1, ActorKind::Job { owner: ActorId::new(0, 0, 0, 0).await, job_id: 1 }, Lane::Background, None, ActivationEvent::Manual).await;
        let shard = kernel.request_exclusive(actor).await.unwrap();
        assert!(shard.0 >= 3, "exclusive shards must come from the reserved tail of the pool");
        kernel.release_exclusive(actor).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn kernel_metrics_counts_actors_shards_packages() {
        let mut kernel = Kernel::new(ShardKind::Native, 4, 0, 4).await;
        kernel.activate(PackageId("s.a".into()), 1, ActorKind::Extension { plugin: PackageId("s.a".into()), extension_id: "e".into() }, Lane::Background, None, ActivationEvent::Manual).await;
        kernel.activate(PackageId("s.b".into()), 2, ActorKind::Extension { plugin: PackageId("s.b".into()), extension_id: "e".into() }, Lane::Background, None, ActivationEvent::Manual).await;
        let metrics = kernel.metrics().await;
        assert_eq!(metrics.actors, 2);
        assert_eq!(metrics.packages, 2);
        assert_eq!(metrics.shards, 4);
    }
    //#endregion 🔖️KernelFacade

    //#region 🔖️ExtensionActivation
    /// 📌️ terra-extension-activation: `activate_pinned` must place the extension actor on
    /// EXACTLY the parent's shard, not wherever the least-loaded heuristic would otherwise choose
    /// — the property `MessageEndpoint::Extension` traffic depends on to never cross a transport.
    #[semio_framework_async_macros::async_test]
    async fn activate_pinned_places_extension_on_parents_shard() {
        let mut kernel = Kernel::new(ShardKind::Native, 4, 0, 4).await;
        let plugin = PackageId("s.cad".into());
        let parent = kernel.activate(plugin.clone(), 1, ActorKind::PluginApp { plugin: plugin.clone(), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
        let parent_shard = kernel.shard_of(parent).await.expect("parent must be pinned by activate");

        let ext_pkg = PackageId("s.cad.aec-extension".into());
        let extension = kernel.activate_pinned(ext_pkg, 2, ActorKind::Extension { plugin: plugin.clone(), extension_id: "aec".into() }, Lane::Background, None, ActivationEvent::Manual, parent_shard, Some(parent), vec![]).await;

        assert_eq!(kernel.shard_of(extension).await, Some(parent_shard), "extension must land on the parent's exact shard, never wherever least-loaded would pick");
    }

    /// ✂️ terra-extension-activation: deactivating a parent must remove BOTH extensions and the
    /// parent itself, leaves-first, with zero orphans left in `kernel.metrics().actors`.
    #[semio_framework_async_macros::async_test]
    async fn deactivate_parent_cascades_leaves_first_with_zero_orphans() {
        let mut kernel = Kernel::new(ShardKind::Native, 4, 0, 4).await;
        let plugin = PackageId("s.cad".into());
        let parent = kernel.activate(plugin.clone(), 1, ActorKind::PluginApp { plugin: plugin.clone(), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
        let shard = kernel.shard_of(parent).await.unwrap();
        let e1 = kernel.activate_pinned(PackageId("s.cad.e1".into()), 2, ActorKind::Extension { plugin: plugin.clone(), extension_id: "e1".into() }, Lane::Background, None, ActivationEvent::Manual, shard, Some(parent), vec![]).await;
        let e2 = kernel.activate_pinned(PackageId("s.cad.e2".into()), 3, ActorKind::Extension { plugin: plugin.clone(), extension_id: "e2".into() }, Lane::Background, None, ActivationEvent::Manual, shard, Some(parent), vec![]).await;
        kernel.link_extension(parent, e1).await.unwrap();
        kernel.link_extension(parent, e2).await.unwrap();
        assert_eq!(kernel.metrics().await.actors, 3);

        let removed = kernel.deactivate(parent).await.unwrap();
        assert_eq!(removed.len(), 3, "parent + 2 extensions");
        assert_eq!(*removed.last().unwrap(), parent, "leaves-first: parent removed LAST");
        assert!(removed[..2].contains(&e1) && removed[..2].contains(&e2), "both extensions removed before the parent");
        assert_eq!(kernel.metrics().await.actors, 0, "zero orphans after cascade");
        assert!(kernel.actor_record(parent).await.is_none());
        assert!(kernel.actor_record(e1).await.is_none());
        assert!(kernel.actor_record(e2).await.is_none());
        assert!(kernel.children_of(parent).await.is_empty(), "link table must not resurrect a removed parent's edge");
    }

    /// 🔪️ terra-extension-activation: `kill` on the parent must cascade identically to `deactivate`
    /// — "a parent kill takes its extensions down" (design doc M6's own acceptance wording).
    #[semio_framework_async_macros::async_test]
    async fn kill_parent_takes_extensions_down() {
        let mut kernel = Kernel::new(ShardKind::Native, 4, 0, 4).await;
        let plugin = PackageId("s.flow".into());
        let parent = kernel.activate(plugin.clone(), 1, ActorKind::PluginApp { plugin: plugin.clone(), app_id: "flow".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
        let shard = kernel.shard_of(parent).await.unwrap();
        let e1 = kernel.activate_pinned(PackageId("s.flow.e1".into()), 2, ActorKind::Extension { plugin: plugin.clone(), extension_id: "e1".into() }, Lane::Background, None, ActivationEvent::Manual, shard, Some(parent), vec![]).await;
        kernel.link_extension(parent, e1).await.unwrap();

        let removed = kernel.kill(parent).await.unwrap();
        assert_eq!(removed, vec![e1, parent], "leaves-first order: extension then parent");
        assert_eq!(kernel.metrics().await.actors, 0);
    }

    /// 🚑️ terra-extension-activation: a single trap on an EXTENSION must restore/kill only that
    /// extension — the parent's own status is untouched. Also proves package isolation: giving
    /// each extension its OWN `PackageId` (the design choice the native cascade this test mirrors
    /// makes — see `📓️terra-extension-activation-report.md`) means even reaching the QUARANTINE
    /// threshold on the extension does not blast the parent's package, unlike two actors
    /// deliberately sharing one `PackageId` (`failure_ladder_trap_then_quarantine_is_package_wide`,
    /// which this test deliberately does not reproduce for the parent/extension pair).
    #[semio_framework_async_macros::async_test]
    async fn trapping_extension_never_faults_the_parent() {
        let mut kernel = Kernel::new(ShardKind::Native, 4, 0, 4).await;
        let plugin = PackageId("s.cad".into());
        let parent = kernel.activate(plugin.clone(), 1, ActorKind::PluginApp { plugin: plugin.clone(), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
        kernel.complete(parent, &ok_turn().await, 0).await.unwrap();
        assert_eq!(kernel.actor_status(parent).await, Some(&ActorStatus::Active));

        let shard = kernel.shard_of(parent).await.unwrap();
        let extension = kernel.activate_pinned(PackageId("s.cad.aec".into()), 2, ActorKind::Extension { plugin: plugin.clone(), extension_id: "aec".into() }, Lane::Background, None, ActivationEvent::Manual, shard, Some(parent), vec![]).await;
        kernel.link_extension(parent, extension).await.unwrap();

        let faulted = TurnResult {
            ui_patches: vec![],
            effects: vec![],
            command_ingress: vec![],
            cold_pair_ingress: Default::default(),
            lifecycle_receipt: None,
            ui_patch_receipt: None,
            next_wake: None,
            status: TurnStatus::Faulted { detail: b"boom".to_vec() },
            usage: Usage::default(),
        };
        let escalation = kernel.complete(extension, &faulted, 10).await.unwrap();
        assert_eq!(escalation, FailureEscalation::Restart, "one trap must only Restart, never quarantine");
        assert_eq!(kernel.actor_status(extension).await, Some(&ActorStatus::Trapped));
        assert_eq!(kernel.actor_status(parent).await, Some(&ActorStatus::Active), "the parent must be completely untouched by its extension's trap");

        // Push the SAME extension past the quarantine threshold — still must not reach the parent,
        // because this test gave the extension its own PackageId (distinct from the parent's).
        for i in 1..FAILURE_QUARANTINE_RESTART_THRESHOLD {
            let faulted = TurnResult {
                ui_patches: vec![],
                effects: vec![],
                command_ingress: vec![],
                cold_pair_ingress: Default::default(),
                lifecycle_receipt: None,
                ui_patch_receipt: None,
                next_wake: None,
                status: TurnStatus::Faulted { detail: b"boom".to_vec() },
                usage: Usage::default(),
            };
            kernel.complete(extension, &faulted, 10 + i as u64).await.unwrap();
        }
        assert_eq!(kernel.actor_status(extension).await, Some(&ActorStatus::Quarantined), "the extension itself does escalate to quarantine");
        assert_eq!(kernel.actor_status(parent).await, Some(&ActorStatus::Active), "package isolation: the parent must still be untouched, even at quarantine");
    }

    /// 🔐️ terra-extension-activation: the security property — a capability the parent was never
    /// granted must be ABSENT from the extension's grants, not silently escalated. Observable via
    /// `actor_record(extension).capabilities`, i.e. a broker denial, never a `KernelError`.
    #[semio_framework_async_macros::async_test]
    async fn extension_capability_grant_is_the_intersection_not_the_request() {
        let mut kernel = Kernel::new(ShardKind::Native, 2, 0, 4).await;
        let plugin = PackageId("s.cad".into());
        let parent = kernel.activate(plugin.clone(), 1, ActorKind::PluginApp { plugin: plugin.clone(), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
        kernel.set_capabilities(parent, vec![CapabilityGrant { capability: "fs.read".into(), scope: None }, CapabilityGrant { capability: "net.fetch".into(), scope: None }]).await.unwrap();

        let shard = kernel.shard_of(parent).await.unwrap();
        let requested = vec![CapabilityGrant { capability: "fs.read".into(), scope: None }, CapabilityGrant { capability: "fs.admin".into(), scope: None }];
        let extension = kernel.activate_pinned(PackageId("s.cad.aec".into()), 2, ActorKind::Extension { plugin: plugin.clone(), extension_id: "aec".into() }, Lane::Background, None, ActivationEvent::Manual, shard, Some(parent), requested).await;

        let record = kernel.actor_record(extension).await.expect("extension must be live");
        assert_eq!(record.capabilities.len(), 1, "only the grant the parent ALSO held may survive");
        assert_eq!(record.capabilities[0].capability, "fs.read");
        assert!(!record.capabilities.iter().any(|g| g.capability == "fs.admin"), "the parent never held fs.admin — it must be absent, not escalated");
    }

    /// 💤️▶️ terra-extension-activation: suspend cascades leaves-first (checkpoint bytes only on
    /// the root), resume cascades parent-first (the symmetric restore direction).
    #[semio_framework_async_macros::async_test]
    async fn suspend_cascade_leaves_first_resume_cascade_parent_first() {
        let mut kernel = Kernel::new(ShardKind::Native, 4, 0, 4).await;
        let plugin = PackageId("s.cad".into());
        let parent = kernel.activate(plugin.clone(), 1, ActorKind::PluginApp { plugin: plugin.clone(), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
        let shard = kernel.shard_of(parent).await.unwrap();
        let extension = kernel.activate_pinned(PackageId("s.cad.aec".into()), 2, ActorKind::Extension { plugin: plugin.clone(), extension_id: "aec".into() }, Lane::Background, None, ActivationEvent::Manual, shard, Some(parent), vec![]).await;
        kernel.link_extension(parent, extension).await.unwrap();

        let order = kernel.suspend_cascade(parent, Some(vec![9, 9])).await.unwrap();
        assert_eq!(order, vec![extension, parent], "leaves-first: extension suspended before parent");
        assert_eq!(kernel.actor_status(parent).await, Some(&ActorStatus::Suspended { checkpoint: Some(vec![9, 9]) }));
        assert_eq!(kernel.actor_status(extension).await, Some(&ActorStatus::Suspended { checkpoint: None }), "descendants carry no checkpoint bytes of their own");

        let resumed = kernel.resume_cascade(parent).await.unwrap();
        assert_eq!(resumed, vec![parent, extension], "parent-first: the symmetric restore direction");
        assert_eq!(kernel.actor_status(parent).await, Some(&ActorStatus::Active));
        assert_eq!(kernel.actor_status(extension).await, Some(&ActorStatus::Active));
    }
    //#endregion 🔖️ExtensionActivation

    //#region 🔖️RuntimeMetricsSnapshot
    /// 📈️ T1 runtime evidence: drives two real actors (different packages/lanes) through
    /// `activate`/`submit`/`tick`/`complete`, then asserts `runtime_metrics_snapshot`'s rows —
    /// package, lane, status, turns, shard — match what the kernel actually did, not a fake.
    #[semio_framework_async_macros::async_test]
    async fn runtime_metrics_snapshot_reflects_real_kernel_activity() {
        let mut kernel = Kernel::new(ShardKind::Native, 2, 0, 8).await;
        let cad = kernel.activate(PackageId("s.cad".into()), 1, ActorKind::PluginApp { plugin: PackageId("s.cad".into()), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
        let stdio = kernel.activate(PackageId("s.stdio".into()), 2, ActorKind::Extension { plugin: PackageId("s.stdio".into()), extension_id: "e".into() }, Lane::Background, None, ActivationEvent::Manual).await;

        kernel.submit(&env(cad, Lane::Interactive, 1).await).await;
        let decision = kernel.tick(0).await;
        assert_eq!(decision.run.len(), 1, "only `cad` has a pending envelope this tick");
        kernel.complete(cad, &ok_turn().await, 5).await.unwrap();

        let snapshot = kernel.runtime_metrics_snapshot(5).await;
        assert_eq!(snapshot.sampled_at_ms, 5);
        assert_eq!(snapshot.kernel.actors, 2);
        assert_eq!(snapshot.kernel.packages, 2);
        assert_eq!(snapshot.actors.len(), 2);

        let cad_row = snapshot.actors.iter().find(|row| row.id == cad).expect("cad row present");
        assert_eq!(cad_row.package, PackageId("s.cad".into()));
        assert_eq!(cad_row.lane, Lane::Interactive);
        assert_eq!(cad_row.status, ActorStatus::Active);
        assert_eq!(cad_row.metrics.turns, 1, "the completed turn must be counted");

        let stdio_row = snapshot.actors.iter().find(|row| row.id == stdio).expect("stdio row present");
        assert_eq!(stdio_row.package, PackageId("s.stdio".into()));
        assert_eq!(stdio_row.lane, Lane::Background);
        assert_eq!(stdio_row.metrics.turns, 0, "stdio never got a turn");

        assert!(!snapshot.shards.is_empty(), "at least one shard row for the two pinned actors");
        let total_shard_actors: u32 = snapshot.shards.iter().map(|row| row.metrics.actors).sum();
        assert_eq!(total_shard_actors, 2, "every actor is counted on exactly one shard");
    }

    #[semio_framework_async_macros::async_test]
    async fn runtime_metrics_due_gates_at_the_2hz_interval_and_always_fires_once() {
        assert!(runtime_metrics_due(None, 0).await, "never published yet must always be due");
        assert!(!runtime_metrics_due(Some(1_000), 1_200).await, "200ms since last publish is inside the 500ms window");
        assert!(runtime_metrics_due(Some(1_000), 1_500).await, "exactly the 500ms interval must fire");
        assert!(runtime_metrics_due(Some(1_000), 2_000).await, "well past the interval must fire");
    }
    //#endregion 🔖️RuntimeMetricsSnapshot

    //#region 🔖️ShardTable
    #[semio_framework_async_macros::async_test]
    async fn shard_sizing_policy_clamps_native_and_web() {
        assert_eq!(clamp_native_shard_count(1).await, 2);
        assert_eq!(clamp_native_shard_count(9).await, 8);
        assert_eq!(clamp_native_shard_count(5).await, 4);
        assert_eq!(clamp_web_shard_count(1).await, 1);
        assert_eq!(clamp_web_shard_count(9).await, 4);
    }
    //#endregion 🔖️ShardTable

    //#region 🔖️JobProgressOverlay
    const PROGRESS_NOW_US: fn() -> Option<u64> = || Some(0);

    fn with_progress_context<T>(operation: u64, generation: u64, cancel: job::CancelToken, run: impl FnOnce(&mut job::StepContext<'_>) -> T) -> T {
        let mut preview_sequence = 0;
        let mut context = job::StepContext::new(job::OperationId(operation), job::Generation(generation), job::StepBudget::new(8, 10), cancel, PROGRESS_NOW_US, &mut preview_sequence);
        run(&mut context)
    }

    fn progress_publication(job: u64, operation: u64, base_revision: u64, generation: u64, step_sequence: u64, preview_sequence: u64, outcome: JobStepOutcome) -> JobPublication {
        JobPublication { turn: JobTurn { job, operation: JobOperation { operation, base_revision, generation, preview_sequence, seed: 13 }, step_sequence }, outcome }
    }

    fn admit_progress(store: &mut JobProgressOverlayStore, actor: ActorId, publication: JobPublication) -> JobProgressReceipt {
        let live = store.live_authority(actor, publication.turn.job).expect("job operation was admitted before its publication");
        with_progress_context(publication.turn.operation.operation, publication.turn.operation.generation, job::root_cancel_token(), |context| {
            let admission = store.preflight(context, actor, &publication, live).expect("publication preflight");
            store.publish_reserved(context, admission, publication, live).expect("reserved publication")
        })
    }

    fn drive_progress_close(store: &mut JobProgressOverlayStore) {
        for _ in 0..(JOB_PROGRESS_TOTAL_MAXIMUM_ITEMS + JOB_PROGRESS_ACTIVE_CAPACITY * 3 + 8) {
            if store.terminal_is_empty() {
                return;
            }
            let step = with_progress_context(0, 0, job::root_cancel_token(), |context| store.close_step(context));
            assert!(matches!(step, JobProgressCloseStep::Pending { released_items: 0 | 1, .. } | JobProgressCloseStep::Complete));
        }
        panic!("job progress overlay did not reach its terminal witness");
    }

    #[test]
    fn job_progress_preview_is_distinct_owned_and_checked_out_drop_hands_back_exactly() {
        let actor = ActorId(7);
        let live = JobProgressLiveAuthority::new(11, 17, 23);
        let mut store = JobProgressOverlayStore::new();
        store.begin_operation(actor, 29, live).unwrap();
        let preview = Vec::from([1, 2, 3, 4]);
        let pointer = preview.as_ptr();
        let receipt = admit_progress(&mut store, actor, progress_publication(29, 11, 17, 23, 0, 1, JobStepOutcome::PreviewReady { preview }));
        assert_eq!(receipt.kind(), JobProgressKind::Preview);
        store.acknowledge(receipt).unwrap();
        {
            let checkout = store.take(actor, 29).unwrap();
            assert_eq!(checkout.preview().as_ptr(), pointer);
            assert_eq!(checkout.preview(), &[1, 2, 3, 4]);
        }
        assert_eq!(store.take(actor, 29).unwrap().preview().as_ptr(), pointer, "Drop must return the exact preview owner to its fixed slot");
        let replacement = admit_progress(&mut store, actor, progress_publication(29, 11, 17, 23, 1, 2, JobStepOutcome::PreviewReady { preview: vec![8, 9] }));
        store.abort(replacement).unwrap();
        assert_eq!(store.take(actor, 29).unwrap().preview().as_ptr(), pointer, "abort must restore the last valid preview owner rather than exposing the staged replacement");
        let checkpoint = admit_progress(&mut store, actor, progress_publication(29, 11, 17, 23, 1, 1, JobStepOutcome::CheckpointReady { checkpoint: JobCheckpoint { state: vec![10], applied_progress: 1 } }));
        store.abort(checkpoint).unwrap();
        assert_eq!(store.take(actor, 29).unwrap().preview().as_ptr(), pointer, "aborting scalar progress must retain the prior preview without an unnecessary owner move");
        store.begin_close_actor(actor).unwrap();
        drive_progress_close(&mut store);
    }

    #[test]
    fn job_progress_fixed_capacity_and_aba_admission_fail_closed() {
        let mut store = JobProgressOverlayStore::new();
        for ordinal in 0..JOB_PROGRESS_ACTIVE_CAPACITY {
            store.begin_operation(ActorId(ordinal as u64 + 1), 5, JobProgressLiveAuthority::new(ordinal as u64 + 10, 0, 1)).unwrap();
        }
        assert_eq!(store.begin_operation(ActorId(1000), 5, JobProgressLiveAuthority::new(1000, 0, 1)), Err(JobProgressFault::Capacity));
        assert_eq!(store.begin_operation(ActorId(1), 5, JobProgressLiveAuthority::new(10, 0, 1)), Err(JobProgressFault::Busy));
        store.begin_close_all();
        drive_progress_close(&mut store);
        store.begin_operation(ActorId(1), 5, JobProgressLiveAuthority::new(99, 0, 2)).unwrap();
        assert_eq!(store.live_authority(ActorId(1), 5), Some(JobProgressLiveAuthority::new(99, 0, 2)));
        store.begin_close_all();
        drive_progress_close(&mut store);
    }

    #[test]
    fn job_progress_mounted_aggregate_item_and_byte_caps_reject_plus_one_exactly() {
        let mut item_store = JobProgressOverlayStore::new();
        for ordinal in 0..JOB_PROGRESS_ACTIVE_CAPACITY {
            let actor = ActorId(ordinal as u64 + 1);
            let live = JobProgressLiveAuthority::new(ordinal as u64 + 10, 0, 1);
            item_store.begin_operation(actor, 5, live).unwrap();
            let receipt = admit_progress(&mut item_store, actor, progress_publication(5, live.operation, 0, 1, 0, 1, JobStepOutcome::PreviewReady { preview: Vec::new() }));
            item_store.acknowledge(receipt).unwrap();
        }
        for ordinal in 0..JOB_PROGRESS_RETIREMENT_CAPACITY {
            item_store
                .retain_rejected(JobProgressRejected::new(
                    JobProgressFault::Stale,
                    progress_publication(1_000 + ordinal as u64, 2_000 + ordinal as u64, 0, 1, 0, 0, JobStepOutcome::Complete { candidate: JobCommitCandidate { state: Vec::new(), output: Vec::new() } }),
                ))
                .unwrap();
        }
        assert_eq!(item_store.owned_items, JOB_PROGRESS_TOTAL_MAXIMUM_ITEMS);
        let rejected = JobProgressRejected::new(JobProgressFault::Stale, progress_publication(9_999, 8_888, 0, 1, 0, 0, JobStepOutcome::Yield));
        let returned = item_store.retain_rejected(rejected).unwrap_err();
        assert_eq!(returned.publication().turn.job, 9_999);
        item_store.begin_close_all();
        drive_progress_close(&mut item_store);

        let mut byte_store = JobProgressOverlayStore::new();
        for ordinal in 0..JOB_PROGRESS_ACTIVE_CAPACITY {
            let actor = ActorId(ordinal as u64 + 101);
            let live = JobProgressLiveAuthority::new(ordinal as u64 + 301, 0, 1);
            byte_store.begin_operation(actor, 7, live).unwrap();
            let receipt = admit_progress(&mut byte_store, actor, progress_publication(7, live.operation, 0, 1, 0, 1, JobStepOutcome::PreviewReady { preview: vec![3; JOB_PROGRESS_PAGE_MAXIMUM_BYTES] }));
            byte_store.acknowledge(receipt).unwrap();
        }
        for ordinal in 0..96 {
            byte_store
                .retain_rejected(JobProgressRejected::new(
                    JobProgressFault::Stale,
                    progress_publication(3_000 + ordinal, 4_000 + ordinal, 0, 1, 0, 0, JobStepOutcome::Complete { candidate: JobCommitCandidate { state: vec![4; JOB_PROGRESS_PAGE_MAXIMUM_BYTES], output: vec![5; JOB_PROGRESS_PAGE_MAXIMUM_BYTES] } }),
                ))
                .unwrap();
        }
        assert_eq!(byte_store.owned_bytes, JOB_PROGRESS_TOTAL_MAXIMUM_BYTES);
        let plus_one = vec![9];
        let pointer = plus_one.as_ptr();
        let returned = byte_store.retain_rejected(JobProgressRejected::new(JobProgressFault::Stale, progress_publication(7_777, 8_888, 0, 1, 0, 1, JobStepOutcome::PreviewReady { preview: plus_one }))).unwrap_err();
        assert!(matches!(&returned.publication().outcome, JobStepOutcome::PreviewReady { preview } if preview.as_ptr() == pointer));
        byte_store.begin_close_all();
        drive_progress_close(&mut byte_store);
    }

    #[test]
    fn job_progress_page_boundary_plus_one_stale_order_and_cancel_preserve_owner() {
        let actor = ActorId(31);
        let live = JobProgressLiveAuthority::new(37, 41, 43);
        let mut store = JobProgressOverlayStore::new();
        store.begin_operation(actor, 47, live).unwrap();
        let exact = vec![5; JOB_PROGRESS_PAGE_MAXIMUM_BYTES];
        let exact_pointer = exact.as_ptr();
        let exact_publication = progress_publication(47, 37, 41, 43, 0, 1, JobStepOutcome::PreviewReady { preview: exact });
        let admission = with_progress_context(37, 43, job::root_cancel_token(), |context| store.preflight(context, actor, &exact_publication, live)).unwrap();
        assert_eq!(
            match &exact_publication.outcome {
                JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
                _ => unreachable!(),
            },
            exact_pointer
        );
        store.cancel_admission(admission).unwrap();

        let oversized = vec![7; JOB_PROGRESS_PAGE_MAXIMUM_BYTES + 1];
        let oversized_pointer = oversized.as_ptr();
        let oversized_publication = progress_publication(47, 37, 41, 43, 0, 1, JobStepOutcome::PreviewReady { preview: oversized });
        let oversized_fault = with_progress_context(37, 43, job::root_cancel_token(), |context| store.preflight(context, actor, &oversized_publication, live)).unwrap_err();
        assert_eq!(oversized_fault, JobProgressFault::Oversized);
        assert_eq!(
            match &oversized_publication.outcome {
                JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
                _ => unreachable!(),
            },
            oversized_pointer
        );

        let stale = progress_publication(47, 37, 42, 43, 0, 1, JobStepOutcome::PreviewReady { preview: vec![1] });
        assert_eq!(with_progress_context(37, 43, job::root_cancel_token(), |context| store.preflight(context, actor, &stale, live)).unwrap_err(), JobProgressFault::Stale);
        let out_of_order = progress_publication(47, 37, 41, 43, 1, 1, JobStepOutcome::Yield);
        assert_eq!(with_progress_context(37, 43, job::root_cancel_token(), |context| store.preflight(context, actor, &out_of_order, live)).unwrap_err(), JobProgressFault::StepSequence);

        let cancelled = progress_publication(47, 37, 41, 43, 0, 1, JobStepOutcome::PreviewReady { preview: vec![9] });
        let cancelled_pointer = match &cancelled.outcome {
            JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
            _ => unreachable!(),
        };
        let cancel = job::root_cancel_token();
        cancel.cancel_now();
        assert_eq!(with_progress_context(37, 43, cancel, |context| store.preflight(context, actor, &cancelled, live)).unwrap_err(), JobProgressFault::Cancelled);
        assert_eq!(
            match &cancelled.outcome {
                JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
                _ => unreachable!(),
            },
            cancelled_pointer
        );
        store.begin_close_actor(actor).unwrap();
        drive_progress_close(&mut store);
    }

    #[test]
    fn job_progress_commit_validates_live_authority_and_rejected_close_is_incremental() {
        let actor = ActorId(53);
        let live = JobProgressLiveAuthority::new(59, 61, 67);
        let mut store = JobProgressOverlayStore::new();
        store.begin_operation(actor, 71, live).unwrap();
        let stale = progress_publication(71, 59, 61, 68, 0, 0, JobStepOutcome::Complete { candidate: JobCommitCandidate { state: vec![1], output: vec![2] } });
        assert_eq!(with_progress_context(59, 68, job::root_cancel_token(), |context| store.preflight(context, actor, &stale, JobProgressLiveAuthority::new(59, 61, 68))).unwrap_err(), JobProgressFault::Stale);

        let complete = progress_publication(71, 59, 61, 67, 0, 0, JobStepOutcome::Complete { candidate: JobCommitCandidate { state: vec![3], output: vec![4] } });
        let receipt = admit_progress(&mut store, actor, complete);
        assert_eq!(receipt.kind(), JobProgressKind::CommitValidated);
        store.acknowledge(receipt).unwrap();
        assert!(store.has_close_work(), "committed candidates retire outside the preview overlay");
        drive_progress_close(&mut store);

        let mut rejected_store = JobProgressOverlayStore::new();
        let detail = Vec::from([8, 9, 10]);
        let pointer = detail.as_ptr();
        let rejected = JobProgressRejected::new(JobProgressFault::Stale, progress_publication(73, 79, 83, 89, 0, 0, JobStepOutcome::Fault { detail }));
        assert_eq!(
            match &rejected.publication().outcome {
                JobStepOutcome::Fault { detail } => detail.as_ptr(),
                _ => unreachable!(),
            },
            pointer
        );
        rejected_store.retain_rejected(rejected).unwrap();
        let first = with_progress_context(0, 0, job::root_cancel_token(), |context| rejected_store.close_step(context));
        assert!(matches!(first, JobProgressCloseStep::Pending { released_items: 1, released_bytes } if released_bytes >= 3));
        assert!(!rejected_store.terminal_is_empty(), "publication shell retires on a distinct grant");
        drive_progress_close(&mut rejected_store);
    }

    #[test]
    fn job_progress_replay_is_deterministic_for_identical_publications() {
        let actor = ActorId(97);
        let live = JobProgressLiveAuthority::new(101, 103, 107);
        let mut left = JobProgressOverlayStore::new();
        let mut right = JobProgressOverlayStore::new();
        left.begin_operation(actor, 109, live).unwrap();
        right.begin_operation(actor, 109, live).unwrap();
        for (step, preview_sequence, byte) in [(0, 1, 11), (1, 2, 13)] {
            let left_receipt = admit_progress(&mut left, actor, progress_publication(109, 101, 103, 107, step, preview_sequence, JobStepOutcome::PreviewReady { preview: vec![byte] }));
            let right_receipt = admit_progress(&mut right, actor, progress_publication(109, 101, 103, 107, step, preview_sequence, JobStepOutcome::PreviewReady { preview: vec![byte] }));
            assert_eq!(left_receipt.identity(), right_receipt.identity());
            assert_eq!(left_receipt.kind(), right_receipt.kind());
            left.acknowledge(left_receipt).unwrap();
            right.acknowledge(right_receipt).unwrap();
        }
        assert_eq!(left.take(actor, 109).unwrap().preview(), right.take(actor, 109).unwrap().preview());
        left.begin_close_actor(actor).unwrap();
        right.begin_close_actor(actor).unwrap();
        drive_progress_close(&mut left);
        drive_progress_close(&mut right);
    }
    //#endregion 🔖️JobProgressOverlay
}

//#region 🔖️Typegen
#[cfg(feature = "typegen")]
#[semio_framework_async_macros::async_test]
async fn exports_typescript_bindings() {
    crate::schema_metadata::validate().unwrap();
    let rendered = crate::schema_metadata::render_typescript();
    if let Some(path) = std::env::var_os("SEMIO_TYPEGEN_OUT") {
        std::fs::write(path, &rendered).unwrap();
    } else {
        assert_eq!(rendered, include_str!("../../🤖️generated/🎭️actor/🟦️.ts"));
    }
}
//#endregion 🔖️Typegen
