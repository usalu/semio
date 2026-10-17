/// 📥️ Product ingress and refusal slots: kind and input max plus one, incremental recovery of the remainder and the refusal slot overflow.

#[test]
fn product_ingress_kind_and_input_max_plus_one_return_exact_spawn_and_remainder() {
    let kind_max = "k".repeat(PRODUCT_REPLAY_KIND_BYTES);
    let mut kind_max_outcome = ExchangeOutcome {
        frames: Vec::new(),
        surfaces: UiFixedList::default(),
        effects: vec![Effect::SpawnJob { job: 101, kind: kind_max, input: vec![1], placement: JobPlacement::Inline }],
        command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        typed_results: Vec::new(),
    };
    let admitted = match kind_max_outcome.take_product_replay_authority(103, product_admission_permit(103)) {
        MountedProductReplayAdmission::Admitted(owner) => owner,
        MountedProductReplayAdmission::None | MountedProductReplayAdmission::Refused(_) => panic!("kind MAX must admit"),
    };
    admitted.retire();
    assert_eq!(drain_product_recovery(), 2);

    let input_max = vec![2; PRODUCT_REPLAY_INPUT_BYTES];
    let mut input_max_outcome = ExchangeOutcome {
        frames: Vec::new(),
        surfaces: UiFixedList::default(),
        effects: vec![Effect::SpawnJob { job: 107, kind: "input.max".to_string(), input: input_max, placement: JobPlacement::Isolated }],
        command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        typed_results: Vec::new(),
    };
    let admitted = match input_max_outcome.take_product_replay_authority(109, product_admission_permit(109)) {
        MountedProductReplayAdmission::Admitted(owner) => owner,
        MountedProductReplayAdmission::None | MountedProductReplayAdmission::Refused(_) => panic!("input MAX must admit"),
    };
    admitted.retire();
    assert_eq!(drain_product_recovery(), 2);

    let kind_max_plus_one = "z".repeat(PRODUCT_REPLAY_KIND_BYTES + 1);
    let mut kind_refusal_outcome = ExchangeOutcome {
        frames: Vec::new(),
        surfaces: UiFixedList::default(),
        effects: vec![
            Effect::Notify { message: "before-kind".to_string() },
            Effect::SpawnJob { job: 113, kind: kind_max_plus_one.clone(), input: vec![3, 5], placement: JobPlacement::Exclusive },
            Effect::Notify { message: "after-kind".to_string() },
        ],
        command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        typed_results: Vec::new(),
    };
    let refusal = match kind_refusal_outcome.take_product_replay_authority(127, product_admission_permit(127)) {
        MountedProductReplayAdmission::Refused(owner) => owner,
        MountedProductReplayAdmission::None | MountedProductReplayAdmission::Admitted(_) => panic!("kind MAX+1 must return typed refusal"),
    };
    assert_eq!((refusal.instance, refusal.cause, refusal.selected_index), (127, MountedProductReplayRefusalCause::KindCapacity, 1));
    assert!(matches!(refusal.spawn.as_ref(), Some(Effect::SpawnJob { job: 113, kind, input, placement: JobPlacement::Exclusive }) if kind == &kind_max_plus_one && input == &[3, 5]));
    assert!(matches!(refusal.remaining_effects.as_deref(), Some([Effect::Notify { message: before }, Effect::Notify { message: after }]) if before == "before-kind" && after == "after-kind"));
    assert!(kind_refusal_outcome.effects.is_empty());
    refusal.retire();
    let mut kind_close_opportunities = 0;
    while mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_work() {
        assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(false).progressed);
        kind_close_opportunities += 1;
        assert!(kind_close_opportunities <= 8);
    }
    assert!(kind_close_opportunities >= 6);

    let input_max_plus_one = vec![11; PRODUCT_REPLAY_INPUT_BYTES + 1];
    let mut input_refusal_outcome = ExchangeOutcome {
        frames: Vec::new(),
        surfaces: UiFixedList::default(),
        effects: vec![Effect::SpawnJob { job: 131, kind: "input.max-plus-one".to_string(), input: input_max_plus_one, placement: JobPlacement::Inline }],
        command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        typed_results: Vec::new(),
    };
    let refusal = match input_refusal_outcome.take_product_replay_authority(137, product_admission_permit(137)) {
        MountedProductReplayAdmission::Refused(owner) => owner,
        MountedProductReplayAdmission::None | MountedProductReplayAdmission::Admitted(_) => panic!("input MAX+1 must return typed refusal"),
    };
    assert_eq!(refusal.cause, MountedProductReplayRefusalCause::InputCapacity);
    assert!(matches!(refusal.spawn.as_ref(), Some(Effect::SpawnJob { job: 131, input, .. }) if input.len() == PRODUCT_REPLAY_INPUT_BYTES + 1));
    refusal.retire();
    while mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_work() {
        assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(false).progressed);
    }
}

#[test]
fn product_ingress_recovery_max_plus_one_retries_exact_spawn_then_closes_remainder_incrementally() {
    let reservations: [MountedProductReplayRecoveryToken; JOB_PROGRESS_ACTIVE_CAPACITY] = {
        let mut registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
        std::array::from_fn(|_| registry.reserve().expect("fill every product recovery slot"))
    };
    let mut outcome = ExchangeOutcome {
        frames: Vec::new(),
        surfaces: UiFixedList::default(),
        effects: vec![
            Effect::Notify { message: "before-recovery".to_string() },
            Effect::SpawnJob { job: 139, kind: "recovery.max-plus-one".to_string(), input: vec![13, 17, 19], placement: JobPlacement::Isolated },
            Effect::Notify { message: "after-recovery".to_string() },
        ],
        command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        typed_results: Vec::new(),
    };
    let refusal = match outcome.take_product_replay_authority(149, product_admission_permit(149)) {
        MountedProductReplayAdmission::Refused(owner) => owner,
        MountedProductReplayAdmission::None | MountedProductReplayAdmission::Admitted(_) => panic!("recovery MAX+1 must return typed refusal"),
    };
    assert_eq!((refusal.cause, refusal.selected_index), (MountedProductReplayRefusalCause::RecoveryCapacity, 1));
    assert!(matches!(refusal.spawn.as_ref(), Some(Effect::SpawnJob { job: 139, kind, input, placement: JobPlacement::Isolated }) if kind == "recovery.max-plus-one" && input == &[13, 17, 19]));
    assert_eq!(refusal.remaining_effects.as_ref().map(Vec::len), Some(2));
    refusal.retire();
    assert!(mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").release(reservations[0]));
    let step = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(true);
    let request = step.request.expect("one recovery opportunity retries the exact fixed request");
    assert_eq!(
        (request.instance, request.job, request.job_kind(), request.request, request.placement),
        (149, 139, b"recovery.max-plus-one".as_slice(), JobReplayRequest::from_spawn("recovery.max-plus-one", &[13, 17, 19]), JobPlacement::Isolated)
    );
    request.retire();
    for token in reservations.into_iter().skip(1) {
        assert!(mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").release(token));
    }
    let raw_close_opportunities = drain_product_recovery();
    assert_eq!(raw_close_opportunities, 2);
    let mut close_opportunities = 0;
    while mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_work() {
        assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(false).progressed);
        close_opportunities += 1;
        assert!(close_opportunities <= 8);
    }
    assert!(raw_close_opportunities + close_opportunities >= 6, "raw input, kind, two effects, and remainder backing close remain distinct opportunities");
}

#[test]
fn product_refusal_slot_max_plus_one_stops_before_exchange_and_drop_publishes_exact_generation() {
    let permits: [MountedProductReplayRefusalToken; JOB_PROGRESS_ACTIVE_CAPACITY] = {
        let mut registry = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock");
        std::array::from_fn(|index| registry.reserve(index as u32 + 1).expect("fill fixed refusal permits"))
    };
    assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").reserve(u32::MAX).is_none(), "refusal MAX+1 is denied before ActionBus exchange owns a SpawnJob");
    for token in permits {
        assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").release(token));
    }

    let mut outcome = ExchangeOutcome {
        frames: Vec::new(),
        surfaces: UiFixedList::default(),
        effects: vec![
            Effect::Notify { message: "drop-before".to_string() },
            Effect::SpawnJob { job: 151, kind: "d".repeat(PRODUCT_REPLAY_KIND_BYTES + 1), input: vec![23, 29], placement: JobPlacement::Exclusive },
            Effect::Notify { message: "drop-after".to_string() },
        ],
        command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        typed_results: Vec::new(),
    };
    let refusal = match outcome.take_product_replay_authority(157, product_admission_permit(157)) {
        MountedProductReplayAdmission::Refused(owner) => owner,
        MountedProductReplayAdmission::None | MountedProductReplayAdmission::Admitted(_) => panic!("oversized kind must refuse"),
    };
    let token = refusal.token.expect("typed refusal retains its pre-reserved generation");
    drop(refusal);
    {
        let registry = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock");
        let owner = registry.slots[token.index].owner.as_ref().expect("ordinary refusal Drop publishes exact owner");
        assert_eq!((registry.slots[token.index].generation, owner.instance, owner.cause, owner.selected_index), (token.generation, 157, MountedProductReplayRefusalCause::KindCapacity, 1));
        assert!(matches!(owner.spawn.as_ref(), Some(Effect::SpawnJob { job: 151, input, placement: JobPlacement::Exclusive, .. }) if input == &[23, 29]));
        assert_eq!(owner.remaining_effects.as_ref().map(Vec::len), Some(2));
    }
    let mut opportunities = 0;
    while mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_work() {
        assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(false).progressed);
        opportunities += 1;
        assert!(opportunities <= 8);
    }
    assert!(opportunities >= 6);
}
