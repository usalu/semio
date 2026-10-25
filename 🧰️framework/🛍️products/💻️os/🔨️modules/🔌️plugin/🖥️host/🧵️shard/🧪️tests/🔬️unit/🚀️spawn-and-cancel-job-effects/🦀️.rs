/// 🚀️ Spawn and cancel job effects: the spawn job effect across pumps, the framework reserved spawn, the cancel job effect before its first step and the cancel failure that retires the actor.

/// 🎯️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME J1's headline acceptance test — the mechanism
/// `📓️terra-M5-report.md` §4(a) found entirely missing: "no code anywhere reads a
/// `TurnResult.effects` entry matching `Effect::SpawnJob{kind, ...}` and spawns/drives a job
/// for it". Spawns a job from a scripted turn's own emitted effect, steps it across THREE
/// separate `pump()` calls (`Running`, `Running`, `Done` — never a single-shot call, which is
/// what actually proves the `JobBudget` mechanism resumes rather than just completing once),
/// and observes the completion reach the ORIGINATING actor as a real `Event::JobCompleted` on
/// a LATER `execute_turn` call — not merely that `step_job` returned `Done` in isolation.
///
/// 🔀️ `run_job_to_completion`'s own two-arm shape (Running.../Done) but with TWO `Running`
/// steps first — the resumability proof: a job that finished on step 1 would not
/// distinguish "the budget mechanism resumed it" from "it happened to be a one-shot call".
///
/// 🔚️ Whatever `execute_turn` call eventually receives the `Event::JobCompleted` (pump 4,
/// below) still needs a scripted outcome to return — an ordinary idle turn is enough since
/// this test only asserts what EVENTS that call was given, not its own output.
///
/// Pump 1 owns only the spawning turn. The seed then advances through its finite replay
/// admission opportunities before the first explicit job-step authority is accepted.
///
/// Pump 4: still no new envelope — but the Done step queued an `Event::JobCompleted` for
/// delivery, so the actor is driven ONE more time purely to receive it.
///
/// 🎯️ The actual end-to-end proof: the ORIGINATING actor's `execute_turn` was, at some
/// point, handed a real `Event::JobCompleted{job: 777, result: Ok(..)}` — not merely that
/// `step_job` internally returned `Done` (`job_outcomes` above already showed that; this is
/// the part M5 found completely missing: nothing delivered it back).
#[semio_framework_async_macros::async_test]
async fn spawn_job_effect_is_admitted_stepped_across_multiple_pumps_and_completion_reaches_the_originating_actor() {
    let _replay_authority = replay_test_authority();
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(21);
    let package = PackageRef { package: PackageId("remodel".to_string()), hash: PackageHash([9u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget::original(semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }, 1_000, semio_framework_job::default_now_us().unwrap_or(0), 4, 8, 4096, 1)).await.expect("mock instantiate");

    let job_id = 777u64;
    let mut spawning_turn = MockGuestRuntime::idle_turn(original_retained_turn()).await;
    spawning_turn.effects.push(Effect::SpawnJob { job: job_id, kind: "remodel.reconstruct".to_string(), input: b"seed-frames".to_vec(), placement: JobPlacement::Isolated });
    mock.script_turn(actor, spawning_turn).await;
    mock.script_job_step(actor, JobStep::Running { progress: None }).await;
    mock.script_job_step(actor, JobStep::Running { progress: Some(b"halfway".to_vec()) }).await;
    mock.script_job_step(actor, JobStep::Done { output: b"reconstruction-complete".to_vec() }).await;
    mock.script_turn(actor, MockGuestRuntime::idle_turn(original_retained_turn()).await).await;

    let (transport, probe) = LoopbackTransport::paired().await;
    probe.push_inbound(encode_event_envelope(actor, 1, &fixture_instance_close_event()).await).await;

    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    shard.register(actor, instance);

    let driven1 = pump(&mut shard).await.expect("pump 1");
    assert_eq!(driven1, 1, "the spawn turn does not bypass replay admission");
    let authority = retain_replay_seed(&mut shard, actor, job_id).await;
    probe.push_inbound(encode_payload_envelope(actor, 2, Payload::JobStep { turn: authority }).await).await;
    assert_eq!(pump(&mut shard).await.expect("first retained step"), 1);

    probe.push_inbound(encode_payload_envelope(actor, 3, Payload::JobStep { turn: JobTurn { step_sequence: 1, ..authority } }).await).await;
    let driven2 = pump(&mut shard).await.expect("pump 2");
    assert_eq!(driven2, 1, "only the second Running step — no envelope, so no turn this pump");
    probe.push_inbound(encode_payload_envelope(actor, 4, Payload::JobStep { turn: JobTurn { operation: JobOperation { preview_sequence: 1, ..authority.operation }, step_sequence: 2, ..authority } }).await).await;
    let driven3 = pump(&mut shard).await.expect("pump 3");
    assert_eq!(driven3, 1, "the terminal Done step");

    let driven4 = pump(&mut shard).await.expect("pump 4");
    assert_eq!(driven4, 1, "the queued completion drives one more turn, with no job left to step");

    let outbound = probe.take_outbound().await;
    let outcomes = decode_outcomes(&outbound).await;
    let job_outcomes: Vec<&JobStepOutcome> = outcomes
        .iter()
        .filter_map(|outcome| match outcome {
            ShardOutcome::Job { publication, .. } if publication.turn.job == job_id => Some(&publication.outcome),
            _ => None,
        })
        .collect();
    let authorities: Vec<(JobTurn, JobTurn)> = outcomes
        .iter()
        .filter_map(|outcome| match outcome {
            ShardOutcome::Job { authority, publication, .. } if publication.turn.job == job_id => Some((*authority, publication.turn)),
            _ => None,
        })
        .collect();
    assert_eq!(authorities.len(), 3);
    assert!(authorities.iter().all(|(authority, publication)| {
        authority.job == job_id
            && authority.step_sequence == 0
            && authority.operation.preview_sequence == 0
            && authority.operation.operation == authorities[0].0.operation.operation
            && authority.operation.operation == publication.operation.operation
            && authority.operation.base_revision == publication.operation.base_revision
            && authority.operation.generation == publication.operation.generation
    }));
    assert_eq!(job_outcomes.len(), 3, "exactly three step_job calls were made — the resumability proof");
    assert!(matches!(job_outcomes[0], JobStepOutcome::Yield));
    assert!(matches!(job_outcomes[1], JobStepOutcome::PreviewReady { preview } if preview == b"halfway"));
    assert!(matches!(job_outcomes[2], JobStepOutcome::Complete { candidate } if candidate.output == b"reconstruction-complete"));

    let completed = mock.observed_events(actor).await.into_iter().find(|event| matches!(event, Event::JobCompleted { job, .. } if *job == job_id));
    match completed {
        Some(Event::JobCompleted { result: RequestOutcome::Ok(bytes), .. }) => {
            assert_eq!(bytes, b"reconstruction-complete", "the Done step's own bytes must round-trip into the delivered completion event");
        }
        other => panic!("expected Event::JobCompleted{{job: 777, result: Ok(..)}} to have reached the originating actor's execute_turn, got {other:?}"),
    }
}

/// 🧰️ A framework reserved tool job (`semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND`: undo, redo,
/// the selection and clipboard verbs) is live-only: the spawning turn's outcome hands the host the job's
/// exact shard-minted `JobTurn`, the seed starts the job without ever checkpointing the guest (a whole
/// block2d checkpoint outgrew the fixed checkpoint pages and closed the seed, so native undo never
/// applied — ticket 26/09/23 slice WG8 §1.4), a replay request for it is refused, and the host's steps
/// with that turn run it to `Done` and deliver `Event::JobCompleted` to the spawning actor. Its end takes
/// no guest checkpoint either: nothing commits a live-only job's state, and a block2d terminal checkpoint
/// cut at its 1 s deadline left the guest mid-flight, so the deferred `Event::JobCompleted` trapped
/// (WG8 session 12, native undo/redo under load).
#[semio_framework_async_macros::async_test]
async fn a_framework_reserved_spawn_starts_live_hands_its_turn_to_the_host_and_refuses_replay() {
    let _replay_authority = replay_test_authority();
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(23);
    let package = PackageRef { package: PackageId("block".to_string()), hash: PackageHash([7u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget::original(semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }, 1_000, semio_framework_job::default_now_us().unwrap_or(0), 4, 8, 4096, 1)).await.expect("mock instantiate");
    let job_id = 71u64;
    let mut spawning_turn = MockGuestRuntime::idle_turn(original_retained_turn()).await;
    spawning_turn.effects.push(Effect::SpawnJob { job: job_id, kind: semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND.to_string(), input: b"undo".to_vec(), placement: JobPlacement::Isolated });
    mock.script_turn(actor, spawning_turn).await;
    mock.script_job_step(actor, JobStep::Running { progress: None }).await;
    mock.script_job_step(actor, JobStep::Done { output: b"undone".to_vec() }).await;
    mock.script_turn(actor, MockGuestRuntime::idle_turn(original_retained_turn()).await).await;

    let (transport, probe) = LoopbackTransport::paired().await;
    probe.push_inbound(encode_event_envelope(actor, 1, &fixture_instance_close_event()).await).await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    shard.register(actor, instance);
    assert_eq!(pump(&mut shard).await.expect("spawning turn"), 1);
    let reported = decode_outcomes(&probe.take_outbound().await).await.into_iter().find_map(|outcome| match outcome {
        ShardOutcome::Turn { actor: turn_actor, jobs, .. } if turn_actor == actor.0 => Some(jobs),
        _ => None,
    });
    let reported = reported.expect("the spawning turn's outcome");
    assert_eq!(reported.len(), 1, "the host learns the live job's turn with the spawning turn");
    assert_eq!(reported[0].job, job_id);

    let authority = retain_replay_seed(&mut shard, actor, job_id).await;
    assert_eq!(authority, reported[0], "the reported turn is the seed's exact step authority");
    let GuestInstanceState::Mock(state) = &shard.instances.get(&actor.0).expect("registered instance").state else { panic!("mock instance") };
    assert_eq!(state.checkpoint, None, "a reserved tool job never checkpoints the guest before it starts");
    let request = shard.replay_seeds.iter().flatten().find(|seed| seed.job == job_id).and_then(|seed| seed.seed.as_ref()).map(|seed| seed.request).expect("retained seed request");
    let refusal = shard.validate_replay_request(actor.0, authority, request).expect_err("a live-only reserved job has nothing to replay from");
    assert!(refusal.to_string().contains("live-only"), "{refusal}");

    probe.push_inbound(encode_payload_envelope(actor, 2, Payload::JobStep { turn: authority }).await).await;
    assert_eq!(pump(&mut shard).await.expect("first step"), 1);
    probe.push_inbound(encode_payload_envelope(actor, 3, Payload::JobStep { turn: JobTurn { step_sequence: 1, ..authority } }).await).await;
    assert_eq!(pump(&mut shard).await.expect("terminal step"), 1);
    assert_eq!(pump(&mut shard).await.expect("deferred completion turn"), 1);
    let steps: Vec<JobStepOutcome> = decode_outcomes(&probe.take_outbound().await)
        .await
        .into_iter()
        .filter_map(|outcome| match outcome {
            ShardOutcome::Job { publication, .. } if publication.turn.job == job_id => Some(publication.outcome),
            _ => None,
        })
        .collect();
    assert!(matches!(steps.as_slice(), [JobStepOutcome::Yield, JobStepOutcome::Complete { candidate }] if candidate.output == b"undone" && candidate.state.is_empty()), "{steps:?}");
    let GuestInstanceState::Mock(state) = &shard.instances.get(&actor.0).expect("registered instance").state else { panic!("mock instance") };
    assert_eq!(state.checkpoint, None, "a live-only reserved job commits no restore state, so its end never checkpoints the guest either");
    let completed = mock.observed_events(actor).await.into_iter().find(|event| matches!(event, Event::JobCompleted { job, .. } if *job == job_id));
    assert!(matches!(&completed, Some(Event::JobCompleted { result: RequestOutcome::Ok(bytes), .. }) if bytes == b"undone"), "{completed:?}");
}

/// 🛑️ A successful `Effect::CancelJob` removes the job in the same turn, before step.
#[semio_framework_async_macros::async_test]
async fn cancel_job_effect_stops_a_job_before_it_is_ever_stepped() {
    let _replay_authority = replay_test_authority();
    let process_pages_before = JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire);
    let abi_bytes_before = JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire);
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(22);
    let package = PackageRef { package: PackageId("remodel".to_string()), hash: PackageHash([10u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget::original(semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }, 1_000, semio_framework_job::default_now_us().unwrap_or(0), 4, 8, 4096, 1)).await.expect("mock instantiate");

    let job_id = 888u64;
    let mut turn = MockGuestRuntime::idle_turn(original_retained_turn()).await;
    turn.effects.push(Effect::SpawnJob { job: job_id, kind: "remodel.reconstruct".to_string(), input: Vec::new(), placement: JobPlacement::Inline });
    turn.effects.push(Effect::CancelJob { job: job_id });
    mock.script_turn(actor, turn).await;

    let (transport, probe) = LoopbackTransport::paired().await;
    probe.push_inbound(encode_event_envelope(actor, 1, &fixture_instance_close_event()).await).await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    shard.register(actor, instance);

    let driven = pump(&mut shard).await.expect("pump");
    assert_eq!(driven, 1, "only the turn itself — the job was cancelled before the step phase, so no step_job call happened");

    let outbound = probe.take_outbound().await;
    let outcomes = decode_outcomes(&outbound).await;
    assert!(!outcomes.iter().any(|outcome| matches!(outcome, ShardOutcome::Job { .. })), "a cancelled-before-first-step job must never produce a ShardOutcome::Job");
    assert!(shard.is_registered(actor).await, "local pre-retained cancellation does not retire the actor");
    assert_eq!(mock.cancel_admissions(), 0, "no guest job was admitted to cancel");
    assert_eq!(mock.step_admissions(), 0);
    let seed = shard.replay_seeds.iter().flatten().find(|seed| seed.actor == actor.0 && seed.job == job_id).expect("closing local seed");
    assert_eq!(seed.phase, ReplaySeedPhase::Closing);
    assert_eq!(seed.close_reason, Some(ReplaySeedCloseReason::Cancelled));
    drain_replay_lifecycle(&mut shard, actor.0).await;
    assert_eq!(JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire), process_pages_before);
    assert_eq!(JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire), abi_bytes_before);
}

#[semio_framework_async_macros::async_test]
async fn cancel_job_effect_failure_retires_the_actor_and_surfaces_the_typed_fault() {
    let _replay_authority = replay_test_authority();
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(23);
    let package = PackageRef { package: PackageId("remodel-cancel-failure".to_string()), hash: PackageHash([11u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget::original(semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }, 1_000, semio_framework_job::default_now_us().unwrap_or(0), 4, 8, 4096, 1)).await.expect("mock instantiate");
    let job_id = 889u64;
    let mut spawn = MockGuestRuntime::idle_turn(original_retained_turn()).await;
    spawn.effects.push(Effect::SpawnJob { job: job_id, kind: "remodel.reconstruct".to_string(), input: Vec::new(), placement: JobPlacement::Inline });
    mock.script_turn(actor, spawn).await;
    let mut cancel = MockGuestRuntime::idle_turn(original_retained_turn()).await;
    cancel.effects.push(Effect::CancelJob { job: job_id });
    mock.script_turn(actor, cancel).await;

    let (transport, probe) = LoopbackTransport::paired().await;
    probe.push_inbound(encode_event_envelope(actor, 1, &fixture_instance_close_event()).await).await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    shard.register(actor, instance);

    assert_eq!(pump(&mut shard).await.expect("spawn turn"), 1);
    let authority = retain_replay_seed(&mut shard, actor, job_id).await;
    assert!(shard.running_jobs.contains(&(actor.0, job_id)));
    assert_eq!(authority.job, job_id);
    probe.take_outbound().await;
    mock.fail_next_cancel();
    probe.push_inbound(encode_event_envelope(actor, 2, &fixture_instance_close_event()).await).await;
    assert_eq!(pump(&mut shard).await.expect("pump failed cancellation"), 1);
    assert!(!shard.is_registered(actor).await, "a failed hot-shard cancellation must retire the uncertain actor instance");
    assert!(!shard.running_jobs.contains(&(actor.0, job_id)));
    assert_eq!(mock.cancel_admissions(), 1);
    assert_eq!(mock.step_admissions(), 0, "the retired actor's job must never enter step-job");
    let outcomes = decode_outcomes(&probe.take_outbound().await).await;
    assert!(matches!(
        outcomes.as_slice(),
        [ShardOutcome::Fault { actor: reported, message }]
            if *reported == actor.0
                && message == "ShardLoop::pump: cancel-job 889 failed; actor 23 retired: guest trapped: scripted cancel-job failure"
    ));
    assert!(matches!(shard.replay_seeds.iter().flatten().find(|seed| seed.actor == actor.0 && seed.job == job_id).and_then(|seed| seed.close_reason.as_ref()), Some(ReplaySeedCloseReason::ActorLost)));
    drain_replay_lifecycle(&mut shard, actor.0).await;

    probe.push_inbound(encode_event_envelope(actor, 3, &fixture_instance_close_event()).await).await;
    assert_eq!(pump(&mut shard).await.expect("post-retirement pump"), 1, "the rejected envelope authority is consumed without re-entering the retired guest");
    assert_eq!(mock.cancel_admissions(), 1, "retirement must not retry cancellation");
    assert!(matches!(decode_outcomes(&probe.take_outbound().await).await.as_slice(), [ShardOutcome::Fault { actor: reported, .. }] if *reported == actor.0));
}
