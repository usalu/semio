/// ♻️ Mounted replay drop and panic recovery: the process replay sizes, the sub eight millisecond opportunities, the populated drop and the panic after capture.

#[test]
fn production_mounted_process_replay_supports_one_two_four_and_default_without_private_pool() {
    let profiles: [_; PRODUCT_REPLAY_PROFILE_COUNT] = std::array::from_fn(|cursor| mounted_product_replay_profile(7, cursor).expect("same-process logical replay profile"));
    assert_eq!(profiles, [1, 2, 4, 7]);
    assert_eq!(profiles.map(|worker_count| mounted_product_replay_worker_slot(11, worker_count).expect("deterministic logical slot")), [0, 1, 3, 4]);
    assert_eq!(mounted_product_replay_profile(7, PRODUCT_REPLAY_PROFILE_COUNT), None);
    assert_eq!(mounted_product_replay_profile(0, 3), None);
    assert_eq!(mounted_product_replay_worker_slot(u16::MAX, 4), None);
}

#[test]
fn production_product_authority_opportunities_remain_sub_eight_ms() {
    for cursor in 0..PRODUCT_REPLAY_PROFILE_COUNT {
        let started = std::time::Instant::now();
        let worker_count = mounted_product_replay_profile(7, cursor).expect("fixed production profile");
        let _worker_slot = mounted_product_replay_worker_slot(11, worker_count).expect("fixed production slot");
        assert!(started.elapsed() < Duration::from_millis(8), "one product authority control opportunity exceeded 8ms");
    }
    let started = std::time::Instant::now();
    let (authority, expected) = product_authority_fixture();
    let authority = authority.validate(expected).expect("one fixed identity opportunity");
    assert!(started.elapsed() < Duration::from_millis(8), "one product authority validation opportunity exceeded 8ms");
    authority.retire();
    assert_eq!(drain_product_recovery(), 2);
}

#[test]
fn populated_mounted_replay_drop_publishes_exact_generation_and_drains_incrementally() {
    let recovery = mounted_replay_recovery_registry().lock().expect("replay recovery lock").reserve().expect("pre-reserved recovery");
    let route = JobReplayRoute { plugin: [1; 32], package: [2; 32], controller: [3; 32], tool: [4; 32], window: 5, artifact: [6; 32], request_schema: [7; 32], request_version: 1, request_digest: [8; 32] };
    let authority = JobTurn { job: 19, operation: JobOperation { operation: 23, base_revision: 29, generation: 31, preview_sequence: 0, seed: 37 }, step_sequence: 0 };
    drop(MountedJobReplay {
        actor: ActorId(41),
        authority,
        request: JobReplayRequest::from_spawn("fixture.replay", b"seed"),
        placement: JobPlacement::Inline,
        log: ManuallyDrop::new(JobReplayLog::new(route, 31).expect("generation-qualified replay")),
        captured: None,
        policy: None,
        terminal_seen: false,
        replay_requested: false,
        replay_started: false,
        replay_submit_sequence: None,
        accepted_replay_sequence: None,
        replay_worker_count: 0,
        replay_worker_slot: u16::MAX,
        recovery: Some(recovery),
    });
    {
        let registry = mounted_replay_recovery_registry().lock().expect("replay recovery lock");
        let owner = registry.slots[recovery.index].owner.as_ref().expect("drop published recovery owner");
        assert_eq!((owner.generation, owner.actor, owner.job), (31, ActorId(41), 19));
    }
    let mut opportunities = 0;
    while mounted_replay_recovery_registry().lock().expect("replay recovery lock").has_close_work() {
        assert!(mounted_replay_recovery_registry().lock().expect("replay recovery lock").close_one());
        opportunities += 1;
        assert!(opportunities <= 4);
    }
    assert!(opportunities >= 2, "owner transfer and terminal slot release are distinct grants");
}

#[test]
fn panic_after_mounted_capture_transfers_the_exact_generation_to_incremental_recovery() {
    let recovery = mounted_replay_recovery_registry().lock().expect("replay recovery lock").reserve().expect("pre-reserved panic recovery");
    let route = JobReplayRoute { plugin: [9; 32], package: [8; 32], controller: [7; 32], tool: [6; 32], window: 5, artifact: [4; 32], request_schema: [3; 32], request_version: 1, request_digest: [2; 32] };
    let authority = JobTurn { job: 43, operation: JobOperation { operation: 47, base_revision: 53, generation: 59, preview_sequence: 0, seed: 61 }, step_sequence: 0 };
    let caught = std::panic::catch_unwind(|| {
        let _mounted = MountedJobReplay {
            actor: ActorId(67),
            authority,
            request: JobReplayRequest::from_spawn("fixture.panic", b"owner"),
            placement: JobPlacement::Exclusive,
            log: ManuallyDrop::new(JobReplayLog::new(route, 59).expect("generation-qualified panic replay")),
            captured: None,
            policy: None,
            terminal_seen: false,
            replay_requested: false,
            replay_started: false,
            replay_submit_sequence: None,
            accepted_replay_sequence: None,
            replay_worker_count: 0,
            replay_worker_slot: u16::MAX,
            recovery: Some(recovery),
        };
        panic!("mounted replay panic fixture");
    });
    assert!(caught.is_err());
    {
        let registry = mounted_replay_recovery_registry().lock().expect("replay recovery lock");
        let owner = registry.slots[recovery.index].owner.as_ref().expect("panic published exact owner");
        assert_eq!((owner.generation, owner.actor, owner.job), (59, ActorId(67), 43));
    }
    let mut opportunities = 0;
    while mounted_replay_recovery_registry().lock().expect("replay recovery lock").slots[recovery.index].reserved {
        assert!(mounted_replay_recovery_registry().lock().expect("replay recovery lock").close_one());
        opportunities += 1;
        assert!(opportunities <= JOB_PROGRESS_ACTIVE_CAPACITY * 4);
    }
    assert!(opportunities >= 2);
}
