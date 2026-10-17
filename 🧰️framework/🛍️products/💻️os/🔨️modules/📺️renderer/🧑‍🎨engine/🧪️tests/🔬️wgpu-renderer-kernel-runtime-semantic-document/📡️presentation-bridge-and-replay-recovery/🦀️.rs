/// 📡️ Presentation bridge and replay recovery: the fixed FIFO job progress bridge, the cancelled head, the replay recovery capacity and the rejected production submit retry.

#[test]
fn mounted_job_progress_presentation_bridge_is_fixed_fifo_and_generation_checked() {
    let mut bridge = JobProgressPresentationBridge::new();
    let identity = |ordinal: usize| JobProgressIdentity { actor: ActorId(ordinal as u64 + 1), job: 7, operation: ordinal as u64 + 11, base_revision: 13, generation: 17, step_sequence: 0, preview_sequence: 1 };
    let mut tokens = Vec::new();
    for ordinal in 0..JOB_PROGRESS_PRESENTATION_CAPACITY {
        let token = bridge.reserve(identity(ordinal), JobProgressKind::Preview, ordinal as u64).expect("fixed presentation slot");
        assert!(bridge.publish(token));
        tokens.push(token);
    }
    assert!(bridge.reserve(identity(999), JobProgressKind::Preview, 0).is_none(), "capacity +1 must reject before publication ownership moves");
    for (ordinal, token) in tokens.into_iter().enumerate() {
        let mut lease = bridge.take().expect("FIFO presentation lease");
        assert_eq!(bridge.slots[lease.token.index].identity, identity(ordinal));
        assert_eq!(lease.token, token);
        assert!(bridge.presented(token));
        lease.terminal = true;
        assert!(bridge.release_presented(token));
        assert!(!bridge.release_presented(token), "duplicate generation release is rejected");
    }
    assert!(bridge.take().is_none());
    assert!(bridge.terminal_is_empty(), "realm witness requires every fixed presentation slot to be vacant");
}

#[test]
fn cancelled_head_does_not_strand_later_ready_presentations() {
    let mut bridge = JobProgressPresentationBridge::new();
    let identity = |actor: u64| JobProgressIdentity { actor: ActorId(actor), job: 7, operation: actor + 11, base_revision: 13, generation: 17, step_sequence: 0, preview_sequence: 1 };
    let head = bridge.reserve(identity(1), JobProgressKind::Preview, 1).expect("head");
    let middle = bridge.reserve(identity(2), JobProgressKind::Preview, 2).expect("middle");
    let tail = bridge.reserve(identity(3), JobProgressKind::Preview, 3).expect("tail");
    assert!(bridge.publish(head));
    assert!(bridge.publish(middle));
    assert!(bridge.publish(tail));
    assert!(bridge.cancel(head));

    let mut returned = bridge.take().expect("cancelled head is skipped");
    assert_eq!(returned.token, middle);
    assert!(bridge.return_lease(middle));
    returned.terminal = true;
    let mut middle_lease = bridge.take().expect("returned oldest lease retains admitted order");
    assert_eq!(middle_lease.token, middle);
    assert!(bridge.presented(middle));
    middle_lease.terminal = true;
    assert!(bridge.release_presented(middle));

    let mut tail_lease = bridge.take().expect("tail remains reachable after cancelled hole");
    assert_eq!(tail_lease.token, tail);
    assert!(bridge.presented(tail));
    tail_lease.terminal = true;
    assert!(bridge.release_presented(tail));
    assert!(bridge.terminal_is_empty());
}

#[test]
fn mounted_replay_recovery_capacity_plus_one_refuses_without_touching_reserved_identity() {
    let mut registry = MountedReplayRecoveryRegistry::new();
    let tokens: [_; JOB_PROGRESS_ACTIVE_CAPACITY] = std::array::from_fn(|_| registry.reserve().expect("fixed recovery reservation"));
    assert!(registry.reserve().is_none());
    for token in tokens {
        assert!(registry.release(token));
        assert!(!registry.release(token));
    }
    assert!(!registry.has_close_work());
}

#[test]
fn rejected_production_replay_submit_retries_exact_restore_start_identity_before_job_step() {
    let recovery = mounted_replay_recovery_registry().lock().expect("replay recovery lock").reserve().expect("pre-reserved submit recovery");
    let route = JobReplayRoute { plugin: [1; 32], package: [2; 32], controller: [3; 32], tool: [4; 32], window: 5, artifact: [6; 32], request_schema: [7; 32], request_version: 1, request_digest: [8; 32] };
    let authority = JobTurn { job: 19, operation: JobOperation { operation: 23, base_revision: 29, generation: 31, preview_sequence: 0, seed: 37 }, step_sequence: 0 };
    let request = JobReplayRequest::from_spawn("fixture.production-retry", b"retained-input");
    let mut mounted = MountedJobReplay {
        actor: ActorId(41),
        authority,
        request,
        placement: JobPlacement::Isolated,
        log: ManuallyDrop::new(JobReplayLog::new(route, 31).expect("generation-qualified replay")),
        captured: None,
        policy: None,
        terminal_seen: true,
        replay_requested: true,
        replay_started: false,
        replay_submit_sequence: None,
        accepted_replay_sequence: None,
        replay_worker_count: 4,
        replay_worker_slot: 3,
        recovery: Some(recovery),
    };
    let first = mounted.replay_submission_sequence().expect("first retained restore/start packet");
    let rejected_identity = (mounted.actor, mounted.authority, mounted.request, mounted.replay_worker_count, mounted.replay_worker_slot, first);
    assert!(!mounted.replay_started);
    assert!(mounted.pending_replay_start_is_exact(4, 3));
    assert!(!mounted.pending_replay_start_is_exact(2, 3));
    assert!(!mounted.pending_replay_start_is_exact(4, 2));
    let retry = mounted.replay_submission_sequence().expect("backpressure retries retained restore/start packet");
    assert_eq!((mounted.actor, mounted.authority, mounted.request, mounted.replay_worker_count, mounted.replay_worker_slot, retry), rejected_identity);
    assert!(!mounted.replay_started, "a rejected runtime submit cannot make the next opportunity a JobStep");
    mounted.accept_replay_submission(retry).expect("accepted exact restore/start packet");
    assert!(mounted.replay_started);
    assert_eq!(mounted.replay_submit_sequence, None);
    drop(mounted);
    while mounted_replay_recovery_registry().lock().expect("replay recovery lock").has_close_work() {
        assert!(mounted_replay_recovery_registry().lock().expect("replay recovery lock").close_one());
    }
}
