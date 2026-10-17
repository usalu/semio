/// 🧱️ Product claim and raw backing: the claim fault checkpoint, the effect moved into the fixed authority and the raw allocation identity through refusal, claim, drop and backing retirement.

#[test]
fn product_claim_fault_retains_exact_request_cursor_checkpoint_and_generation() {
    let request = admitted_product_request(83, 89, "action-bus.claim", b"claim-input", JobPlacement::Exclusive);
    let token = request.recovery.expect("pre-reserved claim handback");
    MountedProductReplayClaim { request: Some(request), actor: ActorId(97), record_cursor: 101, checkpoint: Some(MountedProductReplayCheckpoint { ordinal: 103, digest: 107, pages: 2, applied_progress: 109 }) }.retire();
    let mut registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
    assert_eq!(registry.slots[token.index].generation, token.generation);
    assert!(matches!(
        registry.slots[token.index].owner.as_ref(),
        Some(MountedProductReplayRecoveryOwner::Claim {
            request,
            actor: ActorId(97),
            record_cursor: 101,
            checkpoint: Some(MountedProductReplayCheckpoint { ordinal: 103, digest: 107, pages: 2, applied_progress: 109 }),
        }) if request.instance == 83 && request.raw.job == 89 && request.placement == JobPlacement::Exclusive
    ));
    while registry.has_close_work() {
        assert!(registry.close_one());
    }
    assert!(!registry.has_close_work());
}

#[test]
fn product_effect_after_normal_exchange_is_moved_into_fixed_authority_without_json_escape_hatch() {
    let mut outcome = ExchangeOutcome {
        frames: Vec::new(),
        surfaces: UiFixedList::default(),
        effects: vec![
            Effect::Notify { message: "before".to_string() },
            Effect::SpawnJob { job: 73, kind: "action-bus.production".to_string(), input: vec![79, 83], placement: JobPlacement::Inline },
            Effect::Notify { message: "after".to_string() },
        ],
        command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        typed_results: Vec::new(),
    };
    let (kind_pointer, input_pointer) = match &outcome.effects[1] {
        Effect::SpawnJob { kind, input, .. } => (kind.as_ptr(), input.as_ptr()),
        _ => panic!("selected product source remains SpawnJob"),
    };
    let authority = match outcome.take_product_replay_authority(89, product_admission_permit(89)) {
        MountedProductReplayAdmission::Admitted(authority) => authority,
        MountedProductReplayAdmission::None | MountedProductReplayAdmission::Refused(_) => panic!("fixed product exchange must admit"),
    };
    assert_eq!((authority.instance, authority.job, authority.job_kind(), authority.placement), (89, 73, b"action-bus.production".as_slice(), JobPlacement::Inline));
    assert_eq!(authority.request, JobReplayRequest::from_spawn("action-bus.production", &[79, 83]));
    assert_eq!((authority.raw().kind().as_ptr(), authority.raw().input().as_ptr(), authority.raw().selected_index), (kind_pointer, input_pointer, 1));
    assert!(matches!(&outcome.effects[..], [Effect::Notify { message: before }, Effect::Notify { message: after }] if before == "before" && after == "after"));
    authority.retire();
    assert_eq!(drain_product_recovery(), 2);
}

#[test]
fn admitted_product_mount_refusal_returns_exact_raw_allocation_then_closes_one_backing_per_grant() {
    let mut outcome = ExchangeOutcome {
        frames: Vec::new(),
        surfaces: UiFixedList::default(),
        effects: vec![Effect::SpawnJob { job: 91, kind: "raw.mount-refusal".to_string(), input: vec![2, 3, 5, 7], placement: JobPlacement::Exclusive }],
        command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        typed_results: Vec::new(),
    };
    let (kind_pointer, input_pointer) = match &outcome.effects[0] {
        Effect::SpawnJob { kind, input, .. } => (kind.as_ptr(), input.as_ptr()),
        _ => panic!("mount-refusal source remains SpawnJob"),
    };
    let request = match outcome.take_product_replay_authority(93, product_admission_permit(93)) {
        MountedProductReplayAdmission::Admitted(owner) => owner,
        MountedProductReplayAdmission::None | MountedProductReplayAdmission::Refused(_) => panic!("bounded raw request must admit"),
    };
    let token = request.recovery.expect("admitted request keeps exact recovery");
    let rejected: Result<(), MountedProductReplayRequest> = Err(request);
    let request = rejected.expect_err("unregistered mount returns the same request owner");
    assert_eq!((request.raw().kind().as_ptr(), request.raw().input().as_ptr(), request.raw().selected_index), (kind_pointer, input_pointer, 0));
    request.retire();
    {
        let registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
        assert!(
            matches!(registry.slots[token.index].owner.as_ref(), Some(MountedProductReplayRecoveryOwner::Request(request)) if request.raw.kind().as_ptr() == kind_pointer && request.raw.input().as_ptr() == input_pointer && request.raw.disposition == RawSpawnJobDisposition::Rejected)
        );
    }
    assert_eq!(drain_product_recovery(), 2, "raw input and kind backing close on separate grants");
}

#[test]
fn admitted_product_request_claim_and_authority_drop_preserve_raw_allocation_identity() {
    let request = admitted_product_request(95, 97, "raw.request-drop", b"request", JobPlacement::Inline);
    let request_token = request.recovery.expect("request drop recovery");
    let request_identity = (request.raw().kind().as_ptr(), request.raw().input().as_ptr());
    drop(request);
    {
        let registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
        assert!(matches!(registry.slots[request_token.index].owner.as_ref(), Some(MountedProductReplayRecoveryOwner::Request(request)) if (request.raw.kind().as_ptr(), request.raw.input().as_ptr()) == request_identity));
    }
    assert_eq!(drain_product_recovery(), 2);

    let mut request = admitted_product_request(101, 103, "raw.claim-drop", b"claim", JobPlacement::Isolated);
    request.raw_mut().acknowledge_mount();
    let claim_token = request.recovery.expect("claim drop recovery");
    let claim_identity = (request.raw().kind().as_ptr(), request.raw().input().as_ptr());
    drop(MountedProductReplayClaim { request: Some(request), actor: ActorId(107), record_cursor: 0, checkpoint: None });
    {
        let registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
        assert!(matches!(registry.slots[claim_token.index].owner.as_ref(), Some(MountedProductReplayRecoveryOwner::Claim { request, .. }) if (request.raw.kind().as_ptr(), request.raw.input().as_ptr()) == claim_identity));
    }
    assert_eq!(drain_product_recovery(), 2);

    let (authority, _) = product_authority_fixture();
    let authority_token = authority.request_owner().recovery.expect("authority drop recovery");
    let authority_identity = (authority.request_owner().raw().kind().as_ptr(), authority.request_owner().raw().input().as_ptr());
    drop(authority);
    {
        let registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
        assert!(matches!(registry.slots[authority_token.index].owner.as_ref(), Some(MountedProductReplayRecoveryOwner::Authority { request, .. }) if (request.raw.kind().as_ptr(), request.raw.input().as_ptr()) == authority_identity));
    }
    assert_eq!(drain_product_recovery(), 2);
}

#[test]
fn admitted_raw_backing_retires_only_after_fixed_mount_qualification_and_all_replay_acks() {
    let (mut authority, _) = product_authority_fixture();
    assert!(authority.acknowledge_raw_retirement_boundary().is_err(), "raw backing cannot retire before every replay profile ACK");
    authority.profile_cursor = PRODUCT_REPLAY_PROFILE_COUNT;
    authority.profile_started = false;
    authority.restore_start_ordinal = Some(113);
    authority.acknowledge_raw_retirement_boundary().expect("all fixed, mount, qualification, terminal, and replay ACKs admit retirement");
    let raw = authority.request_owner().raw();
    assert_eq!(
        (raw.fixed_witness_acked, raw.mount_acked, raw.qualification_acked, raw.replay_acked, raw.terminal_ordinal, raw.terminal_prefix, raw.accepted_replay_ordinal, raw.disposition),
        (true, true, true, true, Some(authority.terminal.ordinal), Some(authority.terminal.prefix_digest), Some(113), RawSpawnJobDisposition::Accepted)
    );
    authority.retire();
    assert_eq!(drain_product_recovery(), 2);
}
