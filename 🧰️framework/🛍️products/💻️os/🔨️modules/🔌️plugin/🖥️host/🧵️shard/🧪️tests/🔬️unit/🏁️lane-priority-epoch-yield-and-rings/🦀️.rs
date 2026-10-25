// 🏁️ Lane priority, epoch yield and rings: the interactive grant before background grants, the epoch deadline yield, the fixed owner ring boundaries and the malformed frame terminal.

//#region 🔖️LanePriorityAndEpochYield
/// 🎯️ A single admitted grant can carry mixed-lane authorities. One bounded pump selects
/// the interactive authority first, then later pumps preserve background arrival order.
#[semio_framework_async_macros::async_test]
async fn an_interactive_grant_is_executed_before_background_grants_queued_the_same_pump() {
    const BACKGROUND_ACTORS: u64 = 5;
    let mock = Arc::new(MockGuestRuntime::new().await);
    let (transport, probe) = LoopbackTransport::paired().await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;

    let package = PackageRef { package: PackageId("lane-priority".to_string()), hash: PackageHash([90u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let mut envelopes = Vec::new();
    for offset in 0..BACKGROUND_ACTORS {
        let actor = ActorId::new(0, 0, u32::try_from(200 + offset).expect("actor ordinal"), 0).await;
        let instance = mock.instantiate(&compiled, actor, &[], &Budget::original(semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }, 1_000, semio_framework_job::default_now_us().unwrap_or(0), 4, 8, 4096, 1)).await.expect("mock instantiate");
        shard.register(actor, instance);
        mock.script_turn(actor, MockGuestRuntime::idle_turn(original_retained_turn()).await).await;
        envelopes.push(Envelope {
            to: actor,
            from: semio_framework_actor::Origin::Kernel,
            lane: semio_framework_actor::Lane::Background,
            seq: 1,
            deadline_ms: None,
            coalesce: None,
            cancel_of: None,
            payload: Payload::Event { bytes: serde_json::to_vec(&fixture_instance_close_event()).expect("encode") },
        });
    }

    let interactive_actor = ActorId::new(0, 0, 999, 0).await;
    let interactive_instance = mock.instantiate(&compiled, interactive_actor, &[], &Budget::original(semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }, 1_000, semio_framework_job::default_now_us().unwrap_or(0), 4, 8, 4096, 1)).await.expect("mock instantiate");
    shard.register(interactive_actor, interactive_instance);
    let mut interactive_turn = MockGuestRuntime::idle_turn(original_retained_turn()).await;
    interactive_turn.fuel_used = 4242;
    mock.script_turn(interactive_actor, interactive_turn).await;
    let interactive_budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);
    envelopes.push(Envelope {
        to: interactive_actor,
        from: semio_framework_actor::Origin::Kernel,
        lane: semio_framework_actor::Lane::Interactive,
        seq: 1,
        deadline_ms: None,
        coalesce: None,
        cancel_of: None,
        payload: Payload::Event { bytes: serde_json::to_vec(&fixture_instance_close_event()).expect("encode") },
    });
    let mut bytes = Vec::new();
    ShardFrame::Grant { retained: original_retained_turn(), retained_policy: original_retained_turn().grant, work_fuel: original_retained_turn().work.fuel, deadline_us: original_retained_turn().work.absolute_deadline_us, actor: interactive_actor, budget: interactive_budget, envelopes }.pack_encode(&mut bytes).await.expect("declared original frame authority");
    probe.push_inbound(bytes).await;

    let driven = pump(&mut shard).await.expect("pump");
    assert_eq!(driven, 1, "one bounded pump grants one authority");

    let mut outbound = probe.take_outbound().await;
    for _ in 0..BACKGROUND_ACTORS {
        assert_eq!(pump(&mut shard).await.expect("background pump"), 1);
        outbound.extend(probe.take_outbound().await);
    }
    assert_eq!(outbound.len(), BACKGROUND_ACTORS as usize + 1);
    let outcomes = decode_outcomes(&outbound).await;
    match &outcomes[0] {
        ShardOutcome::Turn { actor, result, .. } => {
            assert_eq!(*actor, interactive_actor.0, "the Interactive-lane grant must be the FIRST ShardOutcome sent, despite every Background-lane grant having been queued on the wire BEFORE it");
            assert_eq!(result.usage.fuel, 4242, "must be the interactive actor's own scripted turn, not a background one that happens to share a position");
        }
        other => panic!("expected the first outcome to be the interactive actor's Turn, got {other:?}"),
    }
}

/// 🎯️ terra-shard-lane piece 2: a turn that raises `TurnFault::DeadlineExceeded` (what
/// `WasmtimeRuntime::execute_turn` raises when the epoch deadline armed from
/// `budget.deadline_ms` is hit — `📓️terra-shard-lane-report.md`) must surface as a graceful
/// `ShardOutcome::Turn{status: MoreWork}`, NEVER `ShardOutcome::Fault` — and the actor must stay
/// registered on the shard, ready to be re-granted on a later tick, exactly as a real epoch
/// interrupt (which lands at a wasm-bytecode safe point and leaves the `Store` usable) allows.
#[semio_framework_async_macros::async_test]
async fn a_turn_that_hits_its_epoch_deadline_yields_more_work_not_a_fault_and_stays_registered() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(91);
    let package = PackageRef { package: PackageId("epoch-yield".to_string()), hash: PackageHash([91u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget::original(semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }, 1_000, semio_framework_job::default_now_us().unwrap_or(0), 2, 8, 4096, 1)).await.expect("mock instantiate");
    mock.script_deadline_exceeded(actor).await;

    let (transport, probe) = LoopbackTransport::paired().await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    shard.register(actor, instance);
    probe.push_inbound(encode_payload_envelope(actor, 1, Payload::Event { bytes: serde_json::to_vec(&fixture_instance_close_event()).expect("encode") }).await).await;

    let driven = pump(&mut shard).await.expect("pump");
    assert_eq!(driven, 1);

    let outbound = probe.take_outbound().await;
    assert_eq!(outbound.len(), 1);
    let outcome = decode_outcome(&outbound[0]).await;
    match outcome {
        ShardOutcome::Turn { actor: reported, result, .. } => {
            assert_eq!(reported, 91);
            assert_eq!(result.status, semio_framework_actor::TurnStatus::MoreWork, "a deadline-exceeded turn must yield MoreWork, not surface as a Fault the kernel would escalate/quarantine the actor for");
        }
        other => panic!("expected ShardOutcome::Turn{{status: MoreWork}}, got {other:?} — DeadlineExceeded must never become a Fault"),
    }
    assert!(shard.is_registered(actor).await, "the actor's instance must stay registered — an epoch interrupt does not lose state");
}

#[test]
fn fixed_owner_ring_hands_back_items_and_bytes_at_the_exact_boundary() {
    let mut items = FixedOwnerRing::<u64, 2>::new(16);
    items.try_push(11, 8).expect("first exact owner");
    items.try_push(12, 8).expect("item and byte caps exactly full");
    let rejected = items.try_push(13, 0).expect_err("capacity plus one");
    assert_eq!(rejected.limit, AdmissionLimit::Items);
    assert_eq!(rejected.owner, 13, "the exact rejected owner is handed back");
    assert_eq!(items.pop_front().map(|(_, owner)| owner), Some(11));
    assert_eq!(items.len(), 1, "one scheduling grant pops one FIFO owner and leaves the next retained");

    let mut bytes = FixedOwnerRing::<u64, 2>::new(8);
    bytes.try_push(21, 8).expect("exact byte cap");
    let rejected = bytes.try_push(22, 1).expect_err("byte cap plus one");
    assert_eq!(rejected.limit, AdmissionLimit::Bytes);
    assert_eq!(rejected.owner, 22, "bytes plus one returns the exact authority");
}

#[test]
fn fixed_owner_ring_generation_rejects_an_aba_key_after_slot_reuse() {
    let mut ring = FixedOwnerRing::<u64, 1>::new(8);
    let stale = ring.try_push(31, 1).expect("first generation");
    assert_eq!(ring.pop_front().map(|(_, owner)| owner), Some(31));
    let current = ring.try_push(32, 1).expect("reused physical slot");
    assert!(!ring.contains(stale), "an ABA-stale generation must not address current work");
    assert!(ring.contains(current));
}

#[test]
fn interrupted_close_ring_releases_one_authority_per_grant() {
    let mut closes = FixedOwnerRing::<CancelCursor, 2>::new(2 * size_of::<CancelCursor>());
    closes.try_push(CancelCursor { actor: 41, after_job: None, owner_bytes: size_of::<CancelCursor>() }, size_of::<CancelCursor>()).expect("first close");
    closes.try_push(CancelCursor { actor: 42, after_job: None, owner_bytes: size_of::<CancelCursor>() }, size_of::<CancelCursor>()).expect("second close");
    assert!(closes.pop_front().is_some());
    assert_eq!(closes.len(), 1, "one scheduling grant releases exactly one close authority");
}

#[semio_framework_async_macros::async_test]
async fn malformed_frame_terminalizes_once_without_self_resubmission_readiness() {
    let raw = vec![0xff, 0x7f, 0x01];
    let (transport, probe) = LoopbackTransport::paired().await;
    probe.push_inbound(raw.clone()).await;
    let runtime = Arc::new(GuestRuntimes::Mock(Arc::new(MockGuestRuntime::new().await)));
    let mut shard = ShardLoop::new(runtime, ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;

    match shard.drive_one().await {
        ShardDrive::Fault { consumed_epoch: Some(1), work_remains: false, terminal_frame: true, .. } => {}
        _ => panic!("malformed ingress must consume epoch one and terminalize without retained work"),
    }
    assert_eq!(shard.take_terminal_frame(), Some(raw), "terminal retrieval transfers the exact raw frame");
    assert!(shard.take_terminal_frame().is_none(), "one terminal owner is retrieved exactly once");
    assert!(!shard.has_pending_work(), "terminal ownership is observable, not a hot-resubmit readiness condition");
}
//#endregion 🔖️LanePriorityAndEpochYield
