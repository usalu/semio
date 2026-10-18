// ⏯️ Suspend, resume, cancel and placement: the checkpoint bytes of a suspend, the byte-identical suspend and resume round trip, cancellation, the actor cancel failure and the exclusive placement laws.

//#region 🔖️K1SuspendResumePlacement
/// 📸️ `Payload::Suspend` dispatches to `GuestRuntime::checkpoint` and surfaces its bytes
/// with the explicit operation identity and applied progress boundary.
#[semio_framework_async_macros::async_test]
async fn suspend_with_checkpoint_true_surfaces_checkpoint_bytes_in_the_outcome() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(31);
    let package = PackageRef { package: PackageId("suspend".to_string()), hash: PackageHash([5u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { retained: semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("mock instantiate");

    let (transport, probe) = LoopbackTransport::paired().await;
    let operation = test_job_operation(actor, 0, 0);
    probe.push_inbound(encode_payload_envelope(actor, 1, Payload::Suspend { operation, applied_progress: 73 }).await).await;

    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    shard.register(actor, instance);

    let driven = pump(&mut shard).await.expect("pump");
    assert_eq!(driven, 1, "Suspend consumes one bounded authority opportunity");

    let outbound = probe.take_outbound().await;
    assert_eq!(outbound.len(), 1);
    let outcome = decode_outcome(&outbound[0]).await;
    match outcome {
        ShardOutcome::Checkpoint { actor: reported, operation: reported_operation, checkpoint } => {
            assert_eq!(reported, 31);
            assert_eq!(reported_operation, operation);
            assert_eq!(checkpoint.state, b"mock-checkpoint:31".to_vec(), "MockGuestRuntime::checkpoint's own deterministic bytes must round-trip unmodified");
            assert_eq!(checkpoint.applied_progress, 73);
        }
        other => panic!("expected ShardOutcome::Checkpoint, got {other:?}"),
    }
}

/// 🎯️ Bench budget #7's "identical state hash" property: the EXACT bytes a `Suspend{checkpoint:
/// true}` checkpoint produced, carried through a `Resume{checkpoint: Some(bytes)}` envelope,
/// must be the bytes `GuestRuntime::restore` is called with — verified by reaching into the
/// restored `GuestInstance`'s own mock state (this file is a descendant module of the host
/// crate root that defines `GuestInstanceState`, so the private field is visible here) rather
/// than trusting `Resumed` alone, since `MockGuestRuntime::restore` would return `Ok(())` for
/// ANY bytes.
#[semio_framework_async_macros::async_test]
async fn suspend_then_resume_round_trips_byte_identical_checkpoint_state() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(32);
    let package = PackageRef { package: PackageId("suspend-resume".to_string()), hash: PackageHash([6u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { retained: semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("mock instantiate");

    let (transport, probe) = LoopbackTransport::paired().await;
    let operation = test_job_operation(actor, 0, 0);
    probe.push_inbound(encode_payload_envelope(actor, 1, Payload::Suspend { operation, applied_progress: 19 }).await).await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    shard.register(actor, instance);
    pump(&mut shard).await.expect("pump suspend");

    let suspend_outbound = probe.take_outbound().await;
    let checkpoint = match decode_outcome(&suspend_outbound[0]).await {
        ShardOutcome::Checkpoint { checkpoint, .. } => checkpoint,
        other => panic!("expected ShardOutcome::Checkpoint, got {other:?}"),
    };

    probe.push_inbound(encode_payload_envelope(actor, 2, Payload::Resume { operation, checkpoint: checkpoint.clone() }).await).await;
    pump(&mut shard).await.expect("pump resume");

    let resume_outbound = probe.take_outbound().await;
    let resume_outcome = decode_outcome(&resume_outbound[0]).await;
    assert!(matches!(resume_outcome, ShardOutcome::Resumed { actor: reported, operation: reported_operation } if reported == 32 && reported_operation == operation));

    let instance = shard.instances.get(&actor.0).expect("Resume must not drop the instance");
    let GuestInstanceState::Mock(mock_state) = &instance.state else { panic!("expected a Mock instance") };
    assert_eq!(mock_state.checkpoint.as_deref(), Some(checkpoint.state.as_slice()), "restore must have been called with the EXACT bytes checkpoint produced");
}

/// 🛑️ `Payload::Cancel` must cancel every one of the actor's `running_jobs` (via
/// `GuestRuntime::cancel_job`) and unregister its instance — after which no further `step_job`
/// call for that job can ever happen, since the (actor, job) pair no longer exists in
/// `running_jobs` and the actor itself is no longer registered.
///
/// 🎯️ The cancelled job's replay seed retires one bounded close page per opportunity; once it
/// is drained a further pump proves the job is truly dead: if `running_jobs` still held it,
/// `step_job` would be called again with an EMPTY scripted queue and fault loudly
/// (`TurnFault::Exhausted`) rather than silently succeeding — no such outcome appears.
#[semio_framework_async_macros::async_test]
async fn cancel_unregisters_the_instance_and_no_further_step_job_happens() {
    let _replay_authority = replay_test_authority();
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(41);
    let package = PackageRef { package: PackageId("cancel-payload".to_string()), hash: PackageHash([7u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { retained: semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");

    let job_id = 555u64;
    let mut turn = MockGuestRuntime::idle_turn(original_retained_turn()).await;
    turn.effects.push(Effect::SpawnJob { job: job_id, kind: "remodel.reconstruct".to_string(), input: Vec::new(), placement: JobPlacement::Inline });
    mock.script_turn(actor, turn).await;
    mock.script_job_step(actor, JobStep::Running { progress: None }).await;

    let (transport, probe) = LoopbackTransport::paired().await;
    probe.push_inbound(encode_event_envelope(actor, 1, &fixture_instance_close_event()).await).await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    shard.register(actor, instance);

    let driven1 = pump(&mut shard).await.expect("pump 1");
    assert_eq!(driven1, 1, "the spawning turn owns the first opportunity");
    let authority = retain_replay_seed(&mut shard, actor, job_id).await;
    probe.push_inbound(encode_payload_envelope(actor, 2, Payload::JobStep { turn: authority }).await).await;
    assert_eq!(pump(&mut shard).await.expect("first retained step"), 1);
    assert_eq!(mock.step_admissions(), 1);
    probe.take_outbound().await;

    probe.push_inbound(encode_payload_envelope(actor, 3, Payload::Cancel { seq: 0 }).await).await;
    assert_eq!(pump(&mut shard).await.expect("first bounded cancel opportunity"), 1);
    assert!(shard.is_registered(actor).await, "the actor remains owned until its bounded cancel cursor reaches terminal");
    let seed = shard.replay_seeds.iter().flatten().find(|seed| seed.actor == actor.0 && seed.job == job_id).expect("cancelled replay seed");
    assert_eq!(seed.phase, ReplaySeedPhase::Closing);
    assert_eq!(seed.close_reason, Some(ReplaySeedCloseReason::Cancelled));
    let driven2 = pump(&mut shard).await.expect("terminal cancel opportunity");
    assert_eq!(driven2, 1, "the re-armed cursor unregisters only after every job is cancelled");
    assert!(!shard.is_registered(actor).await, "Cancel must unregister the actor's instance");
    assert_eq!(shard.actor_count().await, 0);
    assert_eq!(mock.cancel_admissions(), 1);

    let outbound2 = probe.take_outbound().await;
    assert_eq!(outbound2.len(), 1);
    let outcome = decode_outcome(&outbound2[0]).await;
    assert!(matches!(outcome, ShardOutcome::Cancelled { actor: reported } if reported == 41));

    drain_replay_lifecycle(&mut shard, actor.0).await;
    assert!(probe.take_outbound().await.is_empty(), "retiring the cancelled seed publishes nothing");
    assert_eq!(mock.step_admissions(), 1);
    let driven3 = pump(&mut shard).await.expect("pump 3");
    assert_eq!(driven3, 0, "nothing left to drive: no envelopes, no running_jobs, no registered instance");
    assert!(probe.take_outbound().await.is_empty(), "no further outcome of any kind for the cancelled job");
}

#[semio_framework_async_macros::async_test]
async fn actor_cancel_failure_retires_the_instance_and_reports_fault_instead_of_cancelled() {
    let _replay_authority = replay_test_authority();
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(42);
    let package = PackageRef { package: PackageId("cancel-payload-failure".to_string()), hash: PackageHash([8u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { retained: semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
    let job_id = 556u64;
    let mut turn = MockGuestRuntime::idle_turn(original_retained_turn()).await;
    turn.effects.push(Effect::SpawnJob { job: job_id, kind: "remodel.reconstruct".to_string(), input: Vec::new(), placement: JobPlacement::Inline });
    mock.script_turn(actor, turn).await;
    mock.script_job_step(actor, JobStep::Running { progress: None }).await;

    let (transport, probe) = LoopbackTransport::paired().await;
    probe.push_inbound(encode_event_envelope(actor, 1, &fixture_instance_close_event()).await).await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    shard.register(actor, instance);
    assert_eq!(pump(&mut shard).await.expect("spawn turn"), 1);
    let authority = retain_replay_seed(&mut shard, actor, job_id).await;
    probe.push_inbound(encode_payload_envelope(actor, 2, Payload::JobStep { turn: authority }).await).await;
    assert_eq!(pump(&mut shard).await.expect("first retained step"), 1);
    probe.take_outbound().await;

    mock.fail_next_cancel();
    probe.push_inbound(encode_payload_envelope(actor, 3, Payload::Cancel { seq: 0 }).await).await;
    assert_eq!(pump(&mut shard).await.expect("pump failed actor cancel"), 1);
    assert!(!shard.is_registered(actor).await);
    assert_eq!(mock.cancel_admissions(), 1);
    assert_eq!(mock.step_admissions(), 1, "retirement must prevent any later guest step");
    assert!(matches!(shard.replay_seeds.iter().flatten().find(|seed| seed.actor == actor.0 && seed.job == job_id).and_then(|seed| seed.close_reason.as_ref()), Some(ReplaySeedCloseReason::ActorLost)));
    drain_replay_lifecycle(&mut shard, actor.0).await;
    let outcomes = decode_outcomes(&probe.take_outbound().await).await;
    assert!(matches!(
        outcomes.as_slice(),
        [ShardOutcome::Fault { actor: reported, message }]
            if *reported == actor.0
                && message == "ShardLoop::pump: actor cancel-job 556 failed before retirement: guest trapped: scripted cancel-job failure"
    ));
    assert!(!outcomes.iter().any(|outcome| matches!(outcome, ShardOutcome::Cancelled { .. })), "failed actor cancellation must never claim success");
}

/// 🚦 `JobPlacement::Exclusive` must be honoured rather than silently ignored: an `Exclusive`
/// job admitted in the SAME pump as an `Inline` one is stepped FIRST — the shard-local routing
/// this packet adds (see `to_step`'s own doc comment for why this is the honest in-shard-only
/// approximation, not cross-shard dedicated placement).
///
/// 🔀️ Inline is pushed FIRST in spawn order, so a passing test proves the sort actually
/// reorders by placement rather than merely preserving admission order by coincidence.
///
/// 🧲️ The terminal exclusive step frees the actor slot so the inline step runs next.
#[semio_framework_async_macros::async_test]
async fn exclusive_placement_is_stepped_before_inline_placement_admitted_the_same_pump() {
    let _replay_authority = replay_test_authority();
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(51);
    let package = PackageRef { package: PackageId("placement".to_string()), hash: PackageHash([8u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { retained: semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");

    let inline_job = 61u64;
    let exclusive_job = 62u64;
    let mut turn = MockGuestRuntime::idle_turn(original_retained_turn()).await;
    turn.effects.push(Effect::SpawnJob { job: inline_job, kind: "a".to_string(), input: Vec::new(), placement: JobPlacement::Inline });
    turn.effects.push(Effect::SpawnJob { job: exclusive_job, kind: "b".to_string(), input: Vec::new(), placement: JobPlacement::Exclusive });
    mock.script_turn(actor, turn).await;
    mock.script_job_step(actor, JobStep::Done { output: vec![6, 2] }).await;
    mock.script_job_step(actor, JobStep::Running { progress: None }).await;

    let (transport, probe) = LoopbackTransport::paired().await;
    probe.push_inbound(encode_event_envelope(actor, 1, &fixture_instance_close_event()).await).await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    shard.register(actor, instance);

    let driven = pump(&mut shard).await.expect("pump");
    assert_eq!(driven, 1, "the spawning turn owns one bounded opportunity");
    let inline_authority = retain_replay_seed(&mut shard, actor, inline_job).await;
    let exclusive_authority = retain_replay_seed(&mut shard, actor, exclusive_job).await;
    let envelopes = vec![
        Envelope { to: actor, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Maintenance, seq: 2, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::JobStep { turn: inline_authority } },
        Envelope { to: actor, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Maintenance, seq: 3, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::JobStep { turn: exclusive_authority } },
    ];
    let mut grant = Vec::new();
    ShardFrame::Grant { retained: original_retained_turn(), actor, budget: semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance), envelopes }.pack_encode(&mut grant).await.expect("declared original frame authority");
    probe.push_inbound(grant).await;
    assert_eq!(pump(&mut shard).await.expect("first placement-selected step"), 1);
    assert_eq!(pump(&mut shard).await.expect("second pump"), 1, "the second job steps on the next actor turn");

    let outbound = probe.take_outbound().await;
    let outcomes = decode_outcomes(&outbound).await;
    let job_order: Vec<u64> = outcomes
        .iter()
        .filter_map(|outcome| match outcome {
            ShardOutcome::Job { publication, .. } => Some(publication.turn.job),
            _ => None,
        })
        .collect();
    assert_eq!(job_order, vec![exclusive_job, inline_job], "the Exclusive-placed job must be stepped before the Inline one despite being spawned second");
}

#[test]
fn exclusive_selection_never_crosses_a_lifecycle_barrier() {
    let actor = ActorId(52);
    let inline = test_job_turn(actor, 71, 0, 0);
    let exclusive = test_job_turn(actor, 72, 0, 0);
    let mut placements = HashMap::new();
    placements.insert((actor.0, inline.job), JobPlacement::Inline);
    placements.insert((actor.0, exclusive.job), JobPlacement::Exclusive);
    let mut consecutive = FixedOwnerRing::<DeferredAuthority, SHARD_DEFERRED_ITEMS>::new(2);
    let inline_key = consecutive.try_push(DeferredAuthority::JobStep { actor: actor.0, turn: inline }, 1).expect("inline generation");
    let exclusive_key = consecutive.try_push(DeferredAuthority::JobStep { actor: actor.0, turn: exclusive }, 1).expect("exclusive generation");
    assert!(matches!(ShardLoop::pop_next_authority(&mut consecutive, &placements), Some((key, DeferredAuthority::JobStep { turn, .. })) if key == exclusive_key && turn.job == exclusive.job));
    assert!(consecutive.contains(inline_key), "priority removal preserves the unselected owner's physical generation key");
    assert!(!consecutive.contains(exclusive_key));
    let mut ring = FixedOwnerRing::<DeferredAuthority, SHARD_DEFERRED_ITEMS>::new(3);
    ring.try_push(DeferredAuthority::JobStep { actor: actor.0, turn: inline }, 1).expect("inline step");
    ring.try_push(DeferredAuthority::Cancel(CancelCursor { actor: actor.0, after_job: None, owner_bytes: 1 }), 1).expect("cancel barrier");
    ring.try_push(DeferredAuthority::JobStep { actor: actor.0, turn: exclusive }, 1).expect("exclusive step");
    assert!(matches!(ShardLoop::pop_next_authority(&mut ring, &placements), Some((_, DeferredAuthority::JobStep { turn, .. })) if turn.job == inline.job));
    assert!(matches!(ShardLoop::pop_next_authority(&mut ring, &placements), Some((_, DeferredAuthority::Cancel(_)))));
    assert!(matches!(ShardLoop::pop_next_authority(&mut ring, &placements), Some((_, DeferredAuthority::JobStep { turn, .. })) if turn.job == exclusive.job));
}
//#endregion 🔖️K1SuspendResumePlacement
