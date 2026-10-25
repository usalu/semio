// 🔁️ Terminal overflow, credit and lifecycle authorities: terminal capacity plus one, the bounded overflow handoff, the transient frame rejection, the exact raw credit, the suspend and resume authorities and the replay seed maximum.

//#region 🔖️LanePriorityAndEpochYield
#[semio_framework_async_macros::async_test]
async fn terminal_capacity_plus_one_parks_then_rearms_once_at_the_fifo_tail() {
    let (transport, probe) = LoopbackTransport::paired().await;
    let runtime = Arc::new(GuestRuntimes::Mock(Arc::new(MockGuestRuntime::new().await)));
    let mut shard = ShardLoop::new(runtime, ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    for epoch in 1..=SHARD_DEFERRED_ITEMS as u64 {
        let raw = vec![0xff, (epoch >> 8) as u8, epoch as u8];
        probe.push_inbound(raw).await;
        assert!(matches!(shard.drive_one().await, ShardDrive::Fault { consumed_epoch: Some(consumed), terminal_overflow: false, .. } if consumed == epoch));
    }
    let overflow_raw = vec![0xfe, 0x01, 0x01];
    probe.push_inbound(overflow_raw.clone()).await;
    match shard.drive_one().await {
        ShardDrive::Fault { consumed_epoch: None, work_remains: false, terminal_frame: true, terminal_overflow: true, .. } => {}
        _ => panic!("terminal capacity plus one must park without acknowledgement or readiness"),
    }
    assert!(!shard.has_pending_work(), "no retrieval means no self-resubmit readiness");
    assert_eq!(shard.terminal_frame_overflow.len(), 1, "one exact overflow owner is retained");

    let (oldest, rearmed_epoch) = shard.take_terminal_frame_and_rearm();
    assert_eq!(oldest, Some(vec![0xff, 0, 1]), "retrieval preserves the older FIFO head");
    assert_eq!(rearmed_epoch, Some(SHARD_DEFERRED_ITEMS as u64 + 1), "one freed slot re-arms the original overflow epoch exactly once");
    assert!(shard.terminal_frame_overflow.is_empty());
    assert_eq!(shard.terminal_frames.len(), SHARD_DEFERRED_ITEMS, "re-arm fills only the one freed terminal slot");
    for _ in 1..SHARD_DEFERRED_ITEMS {
        assert!(shard.take_terminal_frame().is_some());
    }
    assert_eq!(shard.take_terminal_frame(), Some(overflow_raw), "overflow appends after every older terminal owner");
    assert!(shard.take_terminal_frame().is_none());
}

#[semio_framework_async_macros::async_test]
async fn permanently_over_capacity_frame_uses_the_same_bounded_overflow_handoff() {
    let actor = ActorId(54);
    let (transport, probe) = LoopbackTransport::paired().await;
    let runtime = Arc::new(GuestRuntimes::Mock(Arc::new(MockGuestRuntime::new().await)));
    let mut shard = ShardLoop::new(runtime, ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    for index in 0..SHARD_DEFERRED_ITEMS {
        let raw = vec![0xfd, index as u8];
        shard.terminal_frames.try_push(raw.clone(), raw.len()).expect("fill terminal ring exactly");
    }
    let envelopes = (0..=SHARD_DEFERRED_ITEMS)
        .map(|seq| Envelope { to: actor, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Background, seq: seq as u64, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Cancel { seq: seq as u64 } })
        .collect();
    let mut raw = Vec::new();
    ShardFrame::Grant { retained: original_retained_turn(), retained_policy: original_retained_turn().grant, work_fuel: original_retained_turn().work.fuel, deadline_us: original_retained_turn().work.absolute_deadline_us, actor, budget: semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Background), envelopes }.pack_encode(&mut raw).await.expect("declared original frame authority");
    probe.push_inbound(raw.clone()).await;

    assert!(matches!(shard.drive_one().await, ShardDrive::Fault { consumed_epoch: None, terminal_overflow: true, work_remains: false, .. }));
    let (_, rearmed_epoch) = shard.take_terminal_frame_and_rearm();
    assert_eq!(rearmed_epoch, Some(1));
    for _ in 1..SHARD_DEFERRED_ITEMS {
        assert!(shard.take_terminal_frame().is_some());
    }
    assert_eq!(shard.take_terminal_frame(), Some(raw), "permanent-cap raw owner is retained without re-encoding");
}

#[test]
fn terminal_overflow_slot_rejects_an_aba_key_after_rearm() {
    let mut overflow = FixedOwnerRing::<TerminalFrameOverflow, 1>::new(usize::MAX);
    let stale = overflow.try_push(TerminalFrameOverflow { epoch: 1, bytes: vec![1] }, 1).expect("first overflow generation");
    assert_eq!(overflow.pop_front().map(|(_, owner)| owner.epoch), Some(1));
    let current = overflow.try_push(TerminalFrameOverflow { epoch: 2, bytes: vec![2] }, 1).expect("rearmed overflow generation");
    assert!(!overflow.contains(stale), "an ABA-stale overflow generation cannot address the rearmed owner");
    assert!(overflow.contains(current));
}

#[semio_framework_async_macros::async_test]
async fn transient_frame_rejection_keeps_its_original_epoch_until_admission() {
    let actor = ActorId(51);
    let mut bytes = Vec::new();
    ShardFrame::Register { actor }.pack_encode(&mut bytes).await.expect("declared original frame authority");
    let runtime = Arc::new(GuestRuntimes::Mock(Arc::new(MockGuestRuntime::new().await)));
    let mut shard = ShardLoop::new(runtime, ShardTransports::Loopback(LoopbackTransport::default()),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    shard.rejected_frame = Some((9, bytes));

    match shard.drive_one().await {
        ShardDrive::Idle { consumed_epoch: Some(9) } => {}
        _ => panic!("a retained frame must acknowledge its original epoch only after admission"),
    }
}

#[test]
fn grant_raw_credit_is_exact_at_bytes_plus_one() {
    let raw_bytes = 17usize;
    let credits = [split_frame_credit(raw_bytes, 3, 0), split_frame_credit(raw_bytes, 3, 1), split_frame_credit(raw_bytes, 3, 2)];
    assert_eq!(credits.iter().sum::<usize>(), raw_bytes, "nested authorities split the admitted Grant bytes exactly");
    let mut ring = FixedOwnerRing::<u8, 4>::new(raw_bytes);
    for (owner, bytes) in credits.into_iter().enumerate() {
        ring.try_push(owner as u8, bytes).expect("exact Grant byte credit");
    }
    let rejected = ring.try_push(4, 1).expect_err("Grant raw bytes plus one");
    assert_eq!(rejected.limit, AdmissionLimit::Bytes);
    assert_eq!(rejected.owner, 4);
}

#[test]
fn suspend_and_resume_authorities_have_exact_item_and_byte_handback() {
    let actor = ActorId(52);
    let operation = test_job_operation(actor, 7, 0);
    let mut items = FixedOwnerRing::<DeferredAuthority, 1>::new(32);
    items.try_push(DeferredAuthority::Suspend { actor: actor.0, operation, applied_progress: 3 }, 16).expect("Suspend exact item cap");
    let rejected = items.try_push(DeferredAuthority::Resume { actor: actor.0, operation, checkpoint: JobCheckpoint { state: vec![1, 2], applied_progress: 3 } }, 16).expect_err("Resume item cap plus one");
    assert_eq!(rejected.limit, AdmissionLimit::Items);
    assert!(matches!(rejected.owner, DeferredAuthority::Resume { actor: owner, .. } if owner == actor.0));

    let mut bytes = FixedOwnerRing::<DeferredAuthority, 2>::new(16);
    bytes.try_push(DeferredAuthority::Resume { actor: actor.0, operation, checkpoint: JobCheckpoint { state: vec![3], applied_progress: 4 } }, 16).expect("Resume exact byte cap");
    let rejected = bytes.try_push(DeferredAuthority::Suspend { actor: actor.0, operation, applied_progress: 4 }, 1).expect_err("Suspend bytes plus one");
    assert_eq!(rejected.limit, AdmissionLimit::Bytes);
    assert!(matches!(rejected.owner, DeferredAuthority::Suspend { actor: owner, .. } if owner == actor.0));
}

#[test]
fn mixed_lifecycle_authorities_remain_fifo_and_pop_one_per_grant() {
    let actor = ActorId(53);
    let operation = test_job_operation(actor, 8, 0);
    let mut ring = FixedOwnerRing::<DeferredAuthority, 3>::new(3);
    ring.try_push(DeferredAuthority::Register { actor }, 1).expect("Register");
    ring.try_push(DeferredAuthority::Suspend { actor: actor.0, operation, applied_progress: 5 }, 1).expect("Suspend");
    ring.try_push(DeferredAuthority::Resume { actor: actor.0, operation, checkpoint: JobCheckpoint { state: vec![], applied_progress: 5 } }, 1).expect("Resume");
    assert!(matches!(ring.pop_front(), Some((_, DeferredAuthority::Register { actor: owner })) if owner == actor));
    assert_eq!(ring.len(), 2, "one grant advances one mixed lifecycle authority");
    assert!(matches!(ring.pop_front(), Some((_, DeferredAuthority::Suspend { actor: owner, .. })) if owner == actor.0));
    assert!(matches!(ring.pop_front(), Some((_, DeferredAuthority::Resume { actor: owner, .. })) if owner == actor.0));
}

#[test]
fn replay_seed_max_plus_one_returns_the_exact_spawn_owners_unchanged() {
    let _replay_authority = replay_test_authority();
    let kind = "k".repeat(JOB_REPLAY_KIND_PAGE_CAPACITY * JOB_REPLAY_SEED_PAGE_BYTES + 1);
    let input = vec![7; JOB_REPLAY_SEED_PAGE_BYTES];
    let kind_identity = kind.as_ptr();
    let input_identity = input.as_ptr();
    let turn = test_job_turn(ActorId(61), 9, 0, 0);
    let request = JobReplayRequest::from_spawn(&kind, &input);
    let (kind, input) = match MountedReplaySeed::new(61, 9, turn, request, JobPlacement::Inline, kind, input) {
        Ok(_) => panic!("page maximum plus one must refuse"),
        Err(owners) => owners,
    };
    assert_eq!(kind.as_ptr(), kind_identity);
    assert_eq!(input.as_ptr(), input_identity);
}

#[test]
fn replay_owners_drop_safely_from_every_owned_frontier_and_balance_accounting() {
    let _replay_authority = replay_test_authority();
    let process_pages_before = JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire);
    let abi_bytes_before = JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire);
    for (ordinal, phase) in [ReplaySeedPhase::CaptureInput, ReplaySeedPhase::Retained, ReplaySeedPhase::Closing].into_iter().enumerate() {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let actor = ActorId(70 + ordinal as u64);
            let job = 20 + ordinal as u64;
            let turn = test_job_turn(actor, job, 0, 0);
            let kind = "lifecycle.drop".to_string();
            let input = vec![ordinal as u8; JOB_REPLAY_SEED_PAGE_BYTES + 1];
            let request = JobReplayRequest::from_spawn(&kind, &input);
            let mut mounted = MountedReplaySeed::new(actor.0, job, turn, request, JobPlacement::Inline, kind, input).expect("drop-safe replay seed");
            let seed = mounted.seed.as_mut().expect("fixed replay seed");
            while !seed.copy_kind_page(mounted.kind_owner.as_ref().expect("kind owner").as_bytes(), &mut mounted.kind_cursor).expect("kind page admission") {}
            while !seed.copy_input_page(mounted.input_owner.as_ref().expect("input owner"), &mut mounted.input_cursor).expect("input page admission") {}
            if !matches!(phase, ReplaySeedPhase::CaptureInput) {
                let bytes = seed.kind_length;
                mounted.materialized_kind = Some(try_replay_abi_buffer(bytes).expect("ABI buffer admission"));
                mounted.abi_reserved += bytes;
            }
            mounted.phase = phase;
            drop(mounted);
        }));
        assert!(result.is_ok(), "owned replay phase {phase:?} must be drop-safe");
        assert_eq!(JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire), process_pages_before);
        assert_eq!(JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire), abi_bytes_before);
    }
    assert!(std::panic::catch_unwind(|| drop(ReplaySpawnRefusal::new(73, 23, b"drop-safe refusal", "kind".into(), vec![1, 2, 3]))).is_ok());
}
//#endregion 🔖️LanePriorityAndEpochYield
