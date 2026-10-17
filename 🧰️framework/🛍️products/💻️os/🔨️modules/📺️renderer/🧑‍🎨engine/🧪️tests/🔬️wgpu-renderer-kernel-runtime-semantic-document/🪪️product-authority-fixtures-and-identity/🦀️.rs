/// 🪪️ Product authority fixtures and identity: the admitted request, permit, recovery drain and authority fixtures and the complete identity and field qualification laws.

fn admitted_product_request(instance: u32, job: u64, kind: &str, input: &[u8], placement: JobPlacement) -> MountedProductReplayRequest {
    let recovery = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").reserve().expect("fixed product replay request recovery");
    MountedProductReplayRequest::from_admitted_effect(instance, 0, Effect::SpawnJob { job, kind: kind.to_string(), input: input.to_vec(), placement }, recovery)
}

fn product_admission_permit(instance: u32) -> MountedProductReplayAdmissionPermit {
    let token = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").reserve(instance).expect("fixed product refusal permit");
    MountedProductReplayAdmissionPermit { token: Some(token) }
}

fn drain_product_recovery() -> usize {
    let mut opportunities = 0;
    while mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").has_close_work() {
        assert!(mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").close_one());
        opportunities += 1;
    }
    opportunities
}

fn product_authority_fixture() -> (MountedProductReplayAuthority, MountedProductReplayExpected) {
    let kind = "action-bus.production";
    let mut request_owner = admitted_product_request(59, 7, kind, b"fixed-input", JobPlacement::Isolated);
    let request = request_owner.request;
    let route =
        JobReplayRoute { plugin: [1; 32], package: [2; 32], controller: request.controller, tool: request.tool, window: 5, artifact: [6; 32], request_schema: request.schema, request_version: request.version, request_digest: request.digest };
    let operation = JobOperation { operation: 11, base_revision: 13, generation: 17, preview_sequence: 0, seed: 19 };
    let turn = JobTurn { job: 7, operation, step_sequence: 23 };
    let terminal = JobReplayRecordHeader {
        actor: ActorId(29),
        turn,
        ordinal: 31,
        granted_fuel: 37,
        deadline_class_ms: 4,
        worker_count: 7,
        worker_slot: 3,
        cancellation_observed: false,
        kind: JobReplayPublicationKind::Commit,
        policy: JobReplayPublicationPolicy::Accepted,
        applied_progress: 41,
        payload_items: 2,
        payload_bytes: 43,
        payload_digest: 47,
        prefix_digest: 53,
    };
    let checkpoint = Some(MountedProductReplayCheckpoint { ordinal: 61, digest: 67, pages: 2, applied_progress: 69 });
    request_owner.raw_mut().acknowledge_mount();
    request_owner.raw_mut().acknowledge_qualification(checkpoint);
    let expected = MountedProductReplayExpected {
        instance: 59,
        actor: terminal.actor,
        turn,
        request,
        placement: JobPlacement::Isolated,
        route,
        checkpoint,
        terminal,
        process_worker_count: 7,
        process_worker_slot: 3,
        logical_worker_count: 7,
        logical_worker_slot: 3,
        begin: true,
        restore_start_ordinal: Some(71),
    };
    (
        MountedProductReplayAuthority {
            request_owner: Some(request_owner),
            actor: terminal.actor,
            route,
            operation,
            generation: operation.generation,
            seed: operation.seed,
            checkpoint,
            terminal,
            worker_count: 7,
            worker_slot: 3,
            begin: true,
            restore_start_ordinal: Some(71),
            profile_cursor: 0,
            profile_started: false,
        },
        expected,
    )
}

#[test]
fn production_action_bus_product_authority_carries_complete_identity_and_returns_exact_owner_on_wrong_field() {
    let (authority, expected) = product_authority_fixture();
    let authority = authority.validate(expected).expect("complete typed authority");
    assert_eq!(authority.request_owner().job_kind(), b"action-bus.production");
    assert_eq!((authority.instance(), authority.job(), authority.operation, authority.generation, authority.seed), (59, 7, expected.turn.operation, 17, 19));
    assert_eq!(authority.request_owner().placement, JobPlacement::Isolated);
    assert_eq!(authority.checkpoint, Some(MountedProductReplayCheckpoint { ordinal: 61, digest: 67, pages: 2, applied_progress: 69 }));
    assert_eq!((authority.worker_count, authority.worker_slot, authority.begin, authority.restore_start_ordinal), (7, 3, true, Some(71)));
    authority.retire();

    let (authority, mut wrong) = product_authority_fixture();
    wrong.checkpoint.as_mut().expect("checkpoint witness").digest ^= 1;
    let token = authority.request_owner().recovery.expect("pre-reserved exact handback");
    let rejected = authority.validate(wrong).expect_err("wrong checkpoint returns exact authority");
    assert_eq!((rejected.instance(), rejected.job(), rejected.generation, rejected.seed, rejected.terminal.prefix_digest), (59, 7, 17, 19, 53));
    rejected.retire();
    {
        let registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
        assert_eq!(registry.slots[token.index].generation, token.generation);
        assert_eq!(registry.slots[token.index].owner.as_ref().map(recovery_job), Some(7));
    }
    while mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").has_close_work() {
        assert!(mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").close_one());
    }
}

fn assert_product_authority_field_rejected(mutate: fn(&mut MountedProductReplayAuthority, &mut MountedProductReplayExpected)) {
    let (mut authority, mut expected) = product_authority_fixture();
    let token = authority.request_owner().recovery.expect("pre-reserved field rejection");
    mutate(&mut authority, &mut expected);
    let rejected = authority.validate(expected).expect_err("wrong product authority field must return the exact shell");
    assert_eq!((rejected.instance(), rejected.job()), (59, 7));
    rejected.retire();
    let mut registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
    assert_eq!((registry.slots[token.index].generation, registry.slots[token.index].owner.as_ref().map(recovery_job)), (token.generation, Some(7)));
    while registry.has_close_work() {
        assert!(registry.close_one());
    }
}

#[test]
fn production_product_authority_qualifies_every_minted_field_before_replay_work() {
    assert_product_authority_field_rejected(|authority, _| authority.request_owner.as_mut().expect("request").job_kind[0] ^= 1);
    assert_product_authority_field_rejected(|_, expected| expected.request.digest[0] ^= 1);
    assert_product_authority_field_rejected(|_, expected| expected.placement = JobPlacement::Exclusive);
    assert_product_authority_field_rejected(|_, expected| expected.route.request_digest[0] ^= 1);
    assert_product_authority_field_rejected(|_, expected| expected.turn.operation.operation ^= 1);
    assert_product_authority_field_rejected(|_, expected| expected.turn.operation.generation ^= 1);
    assert_product_authority_field_rejected(|_, expected| expected.turn.operation.seed ^= 1);
    assert_product_authority_field_rejected(|_, expected| expected.checkpoint.as_mut().expect("checkpoint").ordinal ^= 1);
    assert_product_authority_field_rejected(|_, expected| expected.checkpoint.as_mut().expect("checkpoint").digest ^= 1);
    assert_product_authority_field_rejected(|_, expected| expected.checkpoint.as_mut().expect("checkpoint").pages ^= 1);
    assert_product_authority_field_rejected(|_, expected| expected.checkpoint.as_mut().expect("checkpoint").applied_progress ^= 1);
    assert_product_authority_field_rejected(|_, expected| expected.terminal.ordinal ^= 1);
    assert_product_authority_field_rejected(|_, expected| expected.terminal.payload_digest ^= 1);
    assert_product_authority_field_rejected(|_, expected| expected.terminal.prefix_digest ^= 1);
    assert_product_authority_field_rejected(|_, expected| expected.process_worker_count ^= 1);
    assert_product_authority_field_rejected(|_, expected| expected.process_worker_slot ^= 1);
    assert_product_authority_field_rejected(|_, expected| expected.logical_worker_count ^= 1);
    assert_product_authority_field_rejected(|_, expected| expected.logical_worker_slot ^= 1);
    assert_product_authority_field_rejected(|_, expected| expected.begin = false);
    assert_product_authority_field_rejected(|_, expected| expected.restore_start_ordinal = Some(72));
}

#[test]
fn product_authority_fault_abort_and_close_rediscover_exact_generation_once() {
    let (authority, _) = product_authority_fixture();
    let token = authority.request_owner().recovery.expect("pre-reserved abort handback");
    let mut authorities: [Option<MountedProductReplayAuthority>; JOB_PROGRESS_ACTIVE_CAPACITY] = std::array::from_fn(|_| None);
    authorities[0] = Some(authority);
    authorities[0].take().expect("fault retains exact authority").retire();
    let mut registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
    assert_eq!((registry.slots[token.index].generation, registry.slots[token.index].owner.as_ref().map(recovery_job)), (token.generation, Some(7)));
    while registry.has_close_work() {
        assert!(registry.close_one());
    }
    assert!(!registry.has_close_work());
}
