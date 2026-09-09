use super::*;
use std::sync::Mutex;

/// 🧪️ [`LoopbackTransport`] (module level, above — moved there so [`ShardTransports`] can name
/// it) hands this back from `paired()`; `pub(super)` so the parent `shard` module's own
/// `LoopbackTransport::paired` can construct it.
pub(super) struct LoopbackProbe {
    pub(super) inbound: Arc<Mutex<Vec<Vec<u8>>>>,
    pub(super) outbound: Arc<Mutex<Vec<Vec<u8>>>>,
}

impl LoopbackProbe {
    async fn push_inbound(&self, bytes: Vec<u8>) {
        self.inbound.lock().expect("loopback lock").push(bytes);
    }
    async fn take_outbound(&self) -> Vec<Vec<u8>> {
        std::mem::take(&mut *self.outbound.lock().expect("loopback lock"))
    }
}

/// 👶️ host-dedyn: every `ShardLoop::pump()`/`pump_primed()` call below is wrapped in
/// `semio_framework_async::block_on` — a `#[test] fn` body is a sanctioned executor entry point
/// (R4 clause 5); every `GuestRuntime`/`ShardTransport` impl these tests drive resolves on its
/// first poll, so `block_on` never actually parks.
async fn pump(shard: &mut ShardLoop) -> Result<usize, PluginHostError> {
    semio_framework_async::block_on(shard.pump())
}

async fn decode_outcome(bytes: &[u8]) -> ShardOutcome {
    let mut pos = 0usize;
    let outcome = ShardOutcome::pack_decode(bytes, &mut pos).await.expect("decode outcome");
    assert_eq!(pos, bytes.len());
    outcome
}

async fn decode_outcomes(bytes: &[Vec<u8>]) -> Vec<ShardOutcome> {
    let mut outcomes = Vec::with_capacity(bytes.len());
    for bytes in bytes {
        outcomes.push(decode_outcome(bytes).await);
    }
    outcomes
}

async fn drain_replay_lifecycle(shard: &mut ShardLoop, actor: u64) {
    for _ in 0..8_192 {
        if shard.replay_seeds.iter().all(Option::is_none) && shard.replay_seed_refusals.iter().all(Option::is_none) {
            return;
        }
        shard.granted_budgets.insert(actor, semio_framework_actor::Budget { fuel: 1, wall_ms: 1, ..semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance) });
        if !shard.drive_replay_refusal().await.expect("replay refusal close") {
            let _ = shard.drive_replay_seed().await;
        }
    }
    panic!("bounded replay lifecycle drain exhausted");
}

async fn retain_replay_seed(shard: &mut ShardLoop, actor: ActorId, job: u64) -> JobTurn {
    for _ in 0..64 {
        let seed = shard.replay_seeds.iter().flatten().find(|seed| seed.actor == actor.0 && seed.job == job).expect("mounted replay seed");
        if matches!(seed.phase, ReplaySeedPhase::Retained) {
            return seed.authority;
        }
        shard.granted_budgets.insert(actor.0, semio_framework_actor::Budget { fuel: 1, wall_ms: 1, ..semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance) });
        assert!(shard.drive_replay_seed().await.expect("replay retention opportunity"));
    }
    panic!("bounded replay retention exhausted");
}

async fn encode_event_envelope(to: ActorId, seq: u64, event: &Event) -> Vec<u8> {
    encode_payload_envelope(to, seq, Payload::Event { bytes: serde_json::to_vec(event).expect("encode event") }).await
}

#[test]
fn instance_close_event_matches_the_shared_first_party_fixture_and_serde_structure() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧫️fixtures/🔣️.json")).expect("shared actor lifecycle fixture");
    let mut independent = fixture["vectors"].as_array().expect("lifecycle vectors").iter().find(|row| row["value"]["kind"] == "close").expect("close vector")["value"].clone();
    independent.as_object_mut().expect("close value object").remove("kind");
    let first_party: serde_json::Value = serde_json::from_str(&dsl::os_pack::json::to_json_string(&fixture_instance_close_request())).expect("first-party close JSON");
    assert_eq!(first_party, independent);
    let event = fixture_instance_close_event();
    let serde_wire = serde_json::to_vec(&event).expect("serde event JSON");
    assert_eq!(serde_json::from_slice::<Event>(&serde_wire).expect("decode serde event JSON"), event);
}

/// ✉️ Generic envelope builder for `Suspend`/`Resume`/`Cancel` payload tests —
/// `encode_event_envelope` above stays as a thin wrapper over this so existing tests are
/// untouched. terra-shard-grants: wraps in `ShardFrame::Envelope` — the transport now carries
/// `ShardFrame`, not raw `Envelope` bytes, so every test-side encoder must wrap here too.
async fn encode_payload_envelope(to: ActorId, seq: u64, payload: Payload) -> Vec<u8> {
    let envelope = Envelope { to, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Interactive, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload };
    let mut bytes = Vec::new();
    ShardFrame::Envelope(envelope).pack_encode(&mut bytes).await;
    bytes
}

fn test_job_operation(actor: ActorId, job: u64, preview_sequence: u64) -> JobOperation {
    JobOperation { operation: actor.0 ^ job.rotate_left(17), base_revision: 7, generation: 3, preview_sequence, seed: 0x5eed ^ job }
}

fn test_job_turn(actor: ActorId, job: u64, step_sequence: u64, preview_sequence: u64) -> JobTurn {
    JobTurn { job, operation: test_job_operation(actor, job, preview_sequence), step_sequence }
}

#[semio_framework_async_macros::async_test]
async fn pump_drives_one_turn_per_actor_and_reports_it_as_a_shard_outcome() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(7);
    let package = PackageRef { package: PackageId("stdio".to_string()), hash: PackageHash([1u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
    let mut scripted = MockGuestRuntime::idle_turn().await;
    scripted.fuel_used = 42;
    mock.script_turn(actor, scripted).await;

    let (transport, probe) = LoopbackTransport::paired().await;
    probe.push_inbound(encode_event_envelope(actor, 1, &fixture_instance_close_event()).await).await;

    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport)).await;
    shard.register(actor, instance);
    assert!(shard.is_registered(actor).await);

    let driven = pump(&mut shard).await.expect("pump succeeds");
    assert_eq!(driven, 1, "exactly one actor had a buffered envelope");

    let outbound = probe.take_outbound().await;
    assert_eq!(outbound.len(), 1, "one ShardOutcome sent back");
    let outcome = decode_outcome(&outbound[0]).await;
    match outcome {
        ShardOutcome::Turn { actor: reported, result } => {
            assert_eq!(reported, 7);
            assert_eq!(result.usage.fuel, 42, "the scripted turn's own fuel_used must round-trip through ShardOutcome");
        }
        other => panic!("expected ShardOutcome::Turn, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn pump_reports_an_envelope_for_an_unregistered_actor_as_a_fault_not_a_silent_drop() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let (transport, probe) = LoopbackTransport::paired().await;
    let stranger = ActorId(99);
    probe.push_inbound(encode_event_envelope(stranger, 1, &fixture_instance_close_event()).await).await;

    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport)).await;
    let driven = pump(&mut shard).await.expect("pump succeeds even with an unknown actor");
    assert_eq!(driven, 1, "the rejected envelope consumes one bounded authority opportunity");

    let outbound = probe.take_outbound().await;
    assert_eq!(outbound.len(), 1);
    let outcome = decode_outcome(&outbound[0]).await;
    assert!(matches!(outcome, ShardOutcome::Fault { actor, .. } if actor == 99), "must surface as a Fault naming the actor, not vanish");
}

#[semio_framework_async_macros::async_test]
async fn unregister_drops_the_instance_and_shrinks_actor_count() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(3);
    let package = PackageRef { package: PackageId("gif".to_string()), hash: PackageHash([2u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("mock instantiate");
    let (transport, _probe) = LoopbackTransport::paired().await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport)).await;
    shard.register(actor, instance);
    assert_eq!(shard.actor_count().await, 1);
    shard.unregister(actor).await;
    assert_eq!(shard.actor_count().await, 0);
    assert!(!shard.is_registered(actor).await);
}

/// 🎯️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME J1's headline acceptance test — the mechanism
/// `📓️terra-M5-report.md` §4(a) found entirely missing: "no code anywhere reads a
/// `TurnResult.effects` entry matching `Effect::SpawnJob{kind, ...}` and spawns/drives a job
/// for it". Spawns a job from a scripted turn's own emitted effect, steps it across THREE
/// separate `pump()` calls (`Running`, `Running`, `Done` — never a single-shot call, which is
/// what actually proves the `JobBudget` mechanism resumes rather than just completing once),
/// and observes the completion reach the ORIGINATING actor as a real `Event::JobCompleted` on
/// a LATER `execute_turn` call — not merely that `step_job` returned `Done` in isolation.
#[semio_framework_async_macros::async_test]
async fn spawn_job_effect_is_admitted_stepped_across_multiple_pumps_and_completion_reaches_the_originating_actor() {
    let _replay_authority = replay_test_authority();
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(21);
    let package = PackageRef { package: PackageId("remodel".to_string()), hash: PackageHash([9u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");

    let job_id = 777u64;
    let mut spawning_turn = MockGuestRuntime::idle_turn().await;
    spawning_turn.effects.push(Effect::SpawnJob { job: job_id, kind: "remodel.reconstruct".to_string(), input: b"seed-frames".to_vec(), placement: JobPlacement::Isolated });
    mock.script_turn(actor, spawning_turn).await;
    // 🔀️ `run_job_to_completion`'s own two-arm shape (Running.../Done) but with TWO `Running`
    // steps first — the resumability proof: a job that finished on step 1 would not
    // distinguish "the budget mechanism resumed it" from "it happened to be a one-shot call".
    mock.script_job_step(actor, JobStep::Running { progress: None }).await;
    mock.script_job_step(actor, JobStep::Running { progress: Some(b"halfway".to_vec()) }).await;
    mock.script_job_step(actor, JobStep::Done { output: b"reconstruction-complete".to_vec() }).await;
    // 🔚️ Whatever `execute_turn` call eventually receives the `Event::JobCompleted` (pump 4,
    // below) still needs a scripted outcome to return — an ordinary idle turn is enough since
    // this test only asserts what EVENTS that call was given, not its own output.
    mock.script_turn(actor, MockGuestRuntime::idle_turn().await).await;

    let (transport, probe) = LoopbackTransport::paired().await;
    probe.push_inbound(encode_event_envelope(actor, 1, &fixture_instance_close_event()).await).await;

    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport)).await;
    shard.register(actor, instance);

    // Pump 1 owns only the spawning turn. The seed then advances through its finite replay
    // admission opportunities before the first explicit job-step authority is accepted.
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

    // Pump 4: still no new envelope — but the Done step queued an `Event::JobCompleted` for
    // delivery, so the actor is driven ONE more time purely to receive it.
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

    // 🎯️ The actual end-to-end proof: the ORIGINATING actor's `execute_turn` was, at some
    // point, handed a real `Event::JobCompleted{job: 777, result: Ok(..)}` — not merely that
    // `step_job` internally returned `Done` (`job_outcomes` above already showed that; this is
    // the part M5 found completely missing: nothing delivered it back).
    let completed = mock.observed_events(actor).await.into_iter().find(|event| matches!(event, Event::JobCompleted { job, .. } if *job == job_id));
    match completed {
        Some(Event::JobCompleted { result: RequestOutcome::Ok(bytes), .. }) => {
            assert_eq!(bytes, b"reconstruction-complete", "the Done step's own bytes must round-trip into the delivered completion event");
        }
        other => panic!("expected Event::JobCompleted{{job: 777, result: Ok(..)}} to have reached the originating actor's execute_turn, got {other:?}"),
    }
}

/// 🛑️ A successful `Effect::CancelJob` removes the job in the same turn, before step.
#[semio_framework_async_macros::async_test]
async fn cancel_job_effect_stops_a_job_before_it_is_ever_stepped() {
    let process_pages_before = JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire);
    let abi_bytes_before = JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire);
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(22);
    let package = PackageRef { package: PackageId("remodel".to_string()), hash: PackageHash([10u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");

    let job_id = 888u64;
    let mut turn = MockGuestRuntime::idle_turn().await;
    turn.effects.push(Effect::SpawnJob { job: job_id, kind: "remodel.reconstruct".to_string(), input: Vec::new(), placement: JobPlacement::Inline });
    turn.effects.push(Effect::CancelJob { job: job_id });
    mock.script_turn(actor, turn).await;

    let (transport, probe) = LoopbackTransport::paired().await;
    probe.push_inbound(encode_event_envelope(actor, 1, &fixture_instance_close_event()).await).await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport)).await;
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
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
    let job_id = 889u64;
    let mut spawn = MockGuestRuntime::idle_turn().await;
    spawn.effects.push(Effect::SpawnJob { job: job_id, kind: "remodel.reconstruct".to_string(), input: Vec::new(), placement: JobPlacement::Inline });
    mock.script_turn(actor, spawn).await;
    let mut cancel = MockGuestRuntime::idle_turn().await;
    cancel.effects.push(Effect::CancelJob { job: job_id });
    mock.script_turn(actor, cancel).await;

    let (transport, probe) = LoopbackTransport::paired().await;
    probe.push_inbound(encode_event_envelope(actor, 1, &fixture_instance_close_event()).await).await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport)).await;
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

//#region 🔖️K1SuspendResumePlacement
/// 📸️ `Payload::Suspend` dispatches to `GuestRuntime::checkpoint` and surfaces its bytes
/// with the explicit operation identity and applied progress boundary.
#[semio_framework_async_macros::async_test]
async fn suspend_with_checkpoint_true_surfaces_checkpoint_bytes_in_the_outcome() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(31);
    let package = PackageRef { package: PackageId("suspend".to_string()), hash: PackageHash([5u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("mock instantiate");

    let (transport, probe) = LoopbackTransport::paired().await;
    let operation = test_job_operation(actor, 0, 0);
    probe.push_inbound(encode_payload_envelope(actor, 1, Payload::Suspend { operation, applied_progress: 73 }).await).await;

    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport)).await;
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
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("mock instantiate");

    let (transport, probe) = LoopbackTransport::paired().await;
    let operation = test_job_operation(actor, 0, 0);
    probe.push_inbound(encode_payload_envelope(actor, 1, Payload::Suspend { operation, applied_progress: 19 }).await).await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport)).await;
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
#[semio_framework_async_macros::async_test]
async fn cancel_unregisters_the_instance_and_no_further_step_job_happens() {
    let _replay_authority = replay_test_authority();
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(41);
    let package = PackageRef { package: PackageId("cancel-payload".to_string()), hash: PackageHash([7u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");

    let job_id = 555u64;
    let mut turn = MockGuestRuntime::idle_turn().await;
    turn.effects.push(Effect::SpawnJob { job: job_id, kind: "remodel.reconstruct".to_string(), input: Vec::new(), placement: JobPlacement::Inline });
    mock.script_turn(actor, turn).await;
    mock.script_job_step(actor, JobStep::Running { progress: None }).await;

    let (transport, probe) = LoopbackTransport::paired().await;
    probe.push_inbound(encode_event_envelope(actor, 1, &fixture_instance_close_event()).await).await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport)).await;
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

    // 🎯️ A third pump proves the job is truly dead: if `running_jobs` still held it, `step_job`
    // would be called again with an EMPTY scripted queue and fault loudly (`TurnFault::
    // Exhausted`) rather than silently succeeding — no such outcome appears.
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
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
    let job_id = 556u64;
    let mut turn = MockGuestRuntime::idle_turn().await;
    turn.effects.push(Effect::SpawnJob { job: job_id, kind: "remodel.reconstruct".to_string(), input: Vec::new(), placement: JobPlacement::Inline });
    mock.script_turn(actor, turn).await;
    mock.script_job_step(actor, JobStep::Running { progress: None }).await;

    let (transport, probe) = LoopbackTransport::paired().await;
    probe.push_inbound(encode_event_envelope(actor, 1, &fixture_instance_close_event()).await).await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport)).await;
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
#[semio_framework_async_macros::async_test]
async fn exclusive_placement_is_stepped_before_inline_placement_admitted_the_same_pump() {
    let _replay_authority = replay_test_authority();
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(51);
    let package = PackageRef { package: PackageId("placement".to_string()), hash: PackageHash([8u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");

    let inline_job = 61u64;
    let exclusive_job = 62u64;
    let mut turn = MockGuestRuntime::idle_turn().await;
    // 🔀️ Inline is pushed FIRST in spawn order, so a passing test proves the sort actually
    // reorders by placement rather than merely preserving admission order by coincidence.
    turn.effects.push(Effect::SpawnJob { job: inline_job, kind: "a".to_string(), input: Vec::new(), placement: JobPlacement::Inline });
    turn.effects.push(Effect::SpawnJob { job: exclusive_job, kind: "b".to_string(), input: Vec::new(), placement: JobPlacement::Exclusive });
    mock.script_turn(actor, turn).await;
    // 🧲️ The terminal exclusive step frees the actor slot so the inline step runs next.
    mock.script_job_step(actor, JobStep::Done { output: vec![6, 2] }).await;
    mock.script_job_step(actor, JobStep::Running { progress: None }).await;

    let (transport, probe) = LoopbackTransport::paired().await;
    probe.push_inbound(encode_event_envelope(actor, 1, &fixture_instance_close_event()).await).await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport)).await;
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
    ShardFrame::Grant { actor, budget: semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance), envelopes }.pack_encode(&mut grant).await;
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

//#region 🔖️ShardFrameRoundTrips
/// 🎯️ terra-shard-grants requirement: every `ShardFrame` variant round-trips through the pack
/// codec, INCLUDING the `Envelope` passthrough — the variant the packet brief explicitly says
/// must not be removed as redundant.
macro_rules! shard_frame_round_trip {
    ($name:ident, $value:expr) => {
        #[semio_framework_async_macros::async_test]
        async fn $name() {
            let value: ShardFrame = $value;
            let mut bytes = Vec::new();
            value.pack_encode(&mut bytes).await;
            let mut pos = 0usize;
            let decoded = ShardFrame::pack_decode(&bytes, &mut pos).await.expect("pack_decode");
            assert_eq!(pos, bytes.len(), "pack_decode must consume exactly what pack_encode wrote");
            assert_eq!(decoded, value);
        }
    };
}

shard_frame_round_trip!(shard_frame_round_trip_register, ShardFrame::Register { actor: ActorId(7) });
shard_frame_round_trip!(shard_frame_round_trip_unregister, ShardFrame::Unregister { actor: ActorId(9) });
shard_frame_round_trip!(
    shard_frame_round_trip_grant,
    ShardFrame::Grant {
        actor: ActorId(11),
        budget: semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive),
        envelopes: vec![Envelope {
            to: ActorId(11),
            from: semio_framework_actor::Origin::Kernel,
            lane: semio_framework_actor::Lane::Interactive,
            seq: 1,
            deadline_ms: None,
            coalesce: None,
            cancel_of: None,
            payload: Payload::Event { bytes: vec![1, 2, 3] },
        }],
    }
);
shard_frame_round_trip!(
    shard_frame_round_trip_envelope_passthrough,
    ShardFrame::Envelope(Envelope { to: ActorId(13), from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Background, seq: 2, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Cancel { seq: 4 } })
);

#[semio_framework_async_macros::async_test]
async fn shard_outcome_owned_pack_round_trips_every_variant() {
    let operation = test_job_operation(ActorId(11), 1, 0);
    let turn = test_job_turn(ActorId(11), 44, 0, 0);
    let values = vec![
        ShardOutcome::Turn {
            actor: 11,
            result: semio_framework_actor::TurnResult {
                ui_patches: vec![1],
                effects: vec![2],
                command_ingress: vec![3],
                cold_pair_ingress: semio_framework_actor::cold_pair::ColdPairIngressStatus::Idle,
                lifecycle_receipt: None,
                ui_patch_receipt: Some(semio_framework_actor::instance_lifetime::ActorUiPatchReceipt {
                    lifetime: semio_framework_actor::instance_lifetime::ActorInstanceLifetime { activation_generation: 1, instance_id: 7, guest_lifetime: 13 },
                    patch_sequence: 1,
                }),
                next_wake: Some(3),
                status: semio_framework_actor::TurnStatus::MoreWork,
                usage: semio_framework_actor::Usage { fuel: 4, wall_us: 5, memory_bytes: 6 },
            },
        },
        ShardOutcome::Job {
            actor: 11,
            authority: turn,
            request: JobReplayRequest::from_spawn("remodel.reconstruct", b"fixture"),
            placement: JobPlacement::Isolated,
            publication: JobPublication { turn, outcome: JobStepOutcome::PreviewReady { preview: vec![7] } },
        },
        ShardOutcome::Fault { actor: 11, message: "fault".to_string() },
        ShardOutcome::Checkpoint { actor: 11, operation, checkpoint: JobCheckpoint { state: vec![8], applied_progress: 9 } },
        ShardOutcome::Resumed { actor: 11, operation },
        ShardOutcome::Cancelled { actor: 11 },
    ];
    for value in values {
        let mut bytes = Vec::new();
        value.pack_encode(&mut bytes).await;
        assert_eq!(decode_outcome(&bytes).await, value);
    }
}

/// 🎯️ A `Grant` with ZERO bundled envelopes must still record its budget — proven separately
/// from the "budget actually executes under" test below, which needs at least one envelope to
/// drive a turn.
#[semio_framework_async_macros::async_test]
async fn grant_with_no_envelopes_still_records_the_budget() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(61);
    let package = PackageRef { package: PackageId("grant-empty".to_string()), hash: PackageHash([20u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("mock instantiate");
    let (transport, probe) = LoopbackTransport::paired().await;
    let mut budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);
    budget.fuel = 123_456;
    let mut bytes = Vec::new();
    ShardFrame::Grant { actor, budget, envelopes: vec![] }.pack_encode(&mut bytes).await;
    probe.push_inbound(bytes).await;

    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport)).await;
    shard.register(actor, instance);
    let driven = pump(&mut shard).await.expect("pump");
    assert_eq!(driven, 0, "no envelopes bundled — nothing to drive yet");
    assert_eq!(shard.granted_budget(actor.0).fuel, 123_456, "the Grant's budget must be recorded even with no envelopes");
}
//#endregion 🔖️ShardFrameRoundTrips

//#region 🔖️GrantBudgetExecution
// 👶️ host-dedyn: `RecordingRuntime` moved to module level (above `mod tests`) so
// `GuestRuntimes::Recording` (`🖥️host/🦀️.rs`) can name it — see that region's own doc.

/// 🎯️ Headline property test for terra-shard-grants Part B: a `Grant`'s budget — NOT any
/// leftover constant, since `TURN_BUDGET`/`JOB_STEP_BUDGET` are deleted from this file entirely
/// — is what `execute_turn` actually receives. Two DIFFERENT `Grant`s (deliberately distinct
/// `fuel` values) prove the budget travels PER-GRANT, not a single hardcoded number that would
/// happen to match by coincidence.
#[semio_framework_async_macros::async_test]
async fn a_grants_budget_is_what_the_turn_actually_executes_under() {
    let runtime = Arc::new(RecordingRuntime::new().await);
    let actor = ActorId(71);
    let package = PackageRef { package: PackageId("grant-budget".to_string()), hash: PackageHash([21u8; 32]) };
    let compiled = runtime.compile(&package, &[]).await.expect("compile");
    let instance = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("instantiate");

    let (transport, probe) = LoopbackTransport::paired().await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Recording(runtime.clone())), ShardTransports::Loopback(transport)).await;
    shard.register(actor, instance);

    let mut first_budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);
    first_budget.fuel = 111_111;
    let envelope = Envelope {
        to: actor,
        from: semio_framework_actor::Origin::Kernel,
        lane: semio_framework_actor::Lane::Interactive,
        seq: 1,
        deadline_ms: None,
        coalesce: None,
        cancel_of: None,
        payload: Payload::Event { bytes: serde_json::to_vec(&Event::Wake).unwrap() },
    };
    let mut bytes = Vec::new();
    ShardFrame::Grant { actor, budget: first_budget, envelopes: vec![envelope] }.pack_encode(&mut bytes).await;
    probe.push_inbound(bytes).await;
    pump(&mut shard).await.expect("pump 1");
    assert_eq!(runtime.last_turn_budget.lock().unwrap().expect("execute_turn must have been called").fuel, 111_111, "the FIRST Grant's own fuel must reach execute_turn, not a constant");

    let mut second_budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Background);
    second_budget.fuel = 222_222;
    let envelope2 = Envelope {
        to: actor,
        from: semio_framework_actor::Origin::Kernel,
        lane: semio_framework_actor::Lane::Interactive,
        seq: 2,
        deadline_ms: None,
        coalesce: None,
        cancel_of: None,
        payload: Payload::Event { bytes: serde_json::to_vec(&Event::Wake).unwrap() },
    };
    let mut bytes2 = Vec::new();
    ShardFrame::Grant { actor, budget: second_budget, envelopes: vec![envelope2] }.pack_encode(&mut bytes2).await;
    probe.push_inbound(bytes2).await;
    pump(&mut shard).await.expect("pump 2");
    assert_eq!(runtime.last_turn_budget.lock().unwrap().expect("execute_turn must have been called again").fuel, 222_222, "a DIFFERENT second Grant's fuel must reach execute_turn too — proving it travels per-Grant, not a fixed constant");

    let _ = probe.take_outbound().await;
}

/// 🎯️ Same property for job stepping: `step_job`'s `JobBudget` comes from the SAME actor's last
/// granted budget (point 2 of the brief: "job steps take the owning actor's last granted budget
/// on the Maintenance lane").
#[semio_framework_async_macros::async_test]
async fn job_step_uses_the_owning_actors_last_granted_budget() {
    let _replay_authority = replay_test_authority();
    let runtime = Arc::new(RecordingRuntime::new().await);
    let actor = ActorId(72);
    let package = PackageRef { package: PackageId("grant-job-budget".to_string()), hash: PackageHash([22u8; 32]) };
    let compiled = runtime.compile(&package, &[]).await.expect("compile");
    let instance = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("instantiate");

    let (transport, probe) = LoopbackTransport::paired().await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Recording(runtime.clone())), ShardTransports::Loopback(transport)).await;
    shard.register(actor, instance);

    let mut budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance);
    budget.fuel = 333_333;
    let mut bytes = Vec::new();
    ShardFrame::Grant { actor, budget, envelopes: vec![] }.pack_encode(&mut bytes).await;
    probe.push_inbound(bytes).await;
    assert_eq!(pump(&mut shard).await.expect("budget grant pump"), 0, "an empty Grant updates authority without driving a guest");
    let turn = test_job_turn(actor, 999, 0, 0);
    let request = JobReplayRequest::from_spawn("grant-budget", &[]);
    shard.running_jobs.insert((actor.0, turn.job));
    shard.job_turns.insert((actor.0, turn.job), turn);
    shard.job_authorities.insert((actor.0, turn.job), JobAuthority { turn, request });
    shard.job_placement.insert((actor.0, turn.job), JobPlacement::Inline);
    let job_bytes = {
        let envelope = Envelope { to: actor, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Maintenance, seq: 2, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::JobStep { turn } };
        let mut out = Vec::new();
        ShardFrame::Envelope(envelope).pack_encode(&mut out).await;
        out
    };
    probe.push_inbound(job_bytes).await;
    assert_eq!(pump(&mut shard).await.expect("job step pump"), 1, "the following pump drives the retained job authority");
    assert_eq!(runtime.last_job_budget.lock().unwrap().expect("step_job must have been called").fuel, 333_333, "step_job must run under the SAME actor's last granted budget, not a deleted JOB_STEP_BUDGET constant");
    let _ = probe.take_outbound().await;
}

/// 🎯️ An actor that was NEVER granted a budget still gets a real, principled one (the
/// Maintenance lane's default) — never a panic, never a zeroed budget.
#[semio_framework_async_macros::async_test]
async fn an_actor_never_granted_a_budget_falls_back_to_the_maintenance_lane_default() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(73);
    let shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(LoopbackTransport::default())).await;
    let expected = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance);
    assert_eq!(shard.granted_budget(actor.0), expected);
}
//#endregion 🔖️GrantBudgetExecution

//#region 🔖️RegisterUnregisterFrames
/// 🛑️ An incoming `ShardFrame::Unregister` must unregister the actor exactly like calling
/// [`ShardLoop::unregister`] directly — real behavior, unlike `Register`'s wire-symmetry-only
/// role (see that variant's own doc).
#[semio_framework_async_macros::async_test]
async fn unregister_frame_drops_the_instance_exactly_like_the_direct_call() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(81);
    let package = PackageRef { package: PackageId("unreg-frame".to_string()), hash: PackageHash([23u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("mock instantiate");
    let (transport, probe) = LoopbackTransport::paired().await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport)).await;
    shard.register(actor, instance);
    assert!(shard.is_registered(actor).await);

    let mut bytes = Vec::new();
    ShardFrame::Unregister { actor }.pack_encode(&mut bytes).await;
    probe.push_inbound(bytes).await;
    let driven = pump(&mut shard).await.expect("pump");
    assert_eq!(driven, 1, "Unregister consumes one bounded authority opportunity");
    assert!(!shard.is_registered(actor).await, "an incoming Unregister frame must drop the instance");
}

/// 📌️ `Register` is decoded without error but has no LOCAL side effect (see its own doc) — this
/// proves `pump` does not choke on it or mistake it for anything else.
#[semio_framework_async_macros::async_test]
async fn register_frame_is_accepted_without_error_and_has_no_local_side_effect() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(82);
    let (transport, probe) = LoopbackTransport::paired().await;
    let mut bytes = Vec::new();
    ShardFrame::Register { actor }.pack_encode(&mut bytes).await;
    probe.push_inbound(bytes).await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport)).await;
    let driven = pump(&mut shard).await.expect("pump must not error on a Register frame");
    assert_eq!(driven, 1, "Register consumes one bounded authority opportunity without instantiating locally");
    assert!(!shard.is_registered(actor).await, "Register never instantiates locally — see its own doc");
}
//#endregion 🔖️RegisterUnregisterFrames

//#region 🔖️BudgetBridge
#[semio_framework_async_macros::async_test]
async fn to_actor_turn_result_maps_status_and_carries_host_measured_usage() {
    let kernel_result = TurnResult {
        ui_patches: semio_framework::kernel::UiTurnPatches::default(),
        effects: vec![],
        presence: vec![],
        next_wake: Some(42),
        status: semio_framework::kernel::TurnStatus::Faulted(b"trap".to_vec()),
        fuel_used: 999,
        command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        cold_pair_ingress: semio_framework::kernel::ColdPairIngressStatus::Idle,
        lifecycle_receipt: None,
        ui_patch_receipt: None,
    };
    let bridged = to_actor_turn_result(kernel_result, 1, 1234, 5678).await.expect("fixed turn encoding");
    assert_eq!(bridged.next_wake, Some(42));
    assert_eq!(bridged.status, semio_framework_actor::TurnStatus::Faulted { detail: b"trap".to_vec() }, "status must map 1:1, and the actor crate's struct-variant Faulted (Part A) must be what this bridge constructs");
    assert_eq!(bridged.usage.fuel, 999, "usage.fuel comes from the kernel TurnResult's own fuel_used");
    assert_eq!(bridged.usage.wall_us, 1234, "wall_us is host-measured, passed straight through");
    assert_eq!(bridged.usage.memory_bytes, 5678, "memory_bytes is host-measured, passed straight through");
    assert!(bridged.ui_patches.is_empty());
    assert_eq!(bridged.ui_patch_receipt, None);
}

#[semio_framework_async_macros::async_test]
async fn to_actor_turn_result_status_maps_idle_more_work_and_checkpoint_ready() {
    let checkpoint = JobCheckpoint { state: vec![4, 2], applied_progress: 9 };
    for (kernel_status, expected) in [
        (semio_framework::kernel::TurnStatus::Idle, semio_framework_actor::TurnStatus::Idle),
        (semio_framework::kernel::TurnStatus::MoreWork, semio_framework_actor::TurnStatus::MoreWork),
        (semio_framework::kernel::TurnStatus::CheckpointReady { checkpoint: checkpoint.clone() }, semio_framework_actor::TurnStatus::CheckpointReady { checkpoint: checkpoint.clone() }),
    ] {
        let kernel_result = TurnResult {
            ui_patches: semio_framework::kernel::UiTurnPatches::default(),
            effects: vec![],
            presence: vec![],
            next_wake: None,
            status: kernel_status,
            fuel_used: 0,
            command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
            cold_pair_ingress: semio_framework::kernel::ColdPairIngressStatus::Idle,
            lifecycle_receipt: None,
            ui_patch_receipt: None,
        };
        let bridged = to_actor_turn_result(kernel_result, 1, 0, 0).await.expect("fixed turn encoding");
        assert_eq!(bridged.status, expected);
        assert!(bridged.ui_patches.is_empty());
        assert_eq!(bridged.ui_patch_receipt, None);
    }
}

/// 📤️ Repeated validated empty turns use the canonical empty actor representation without
/// reserving transport slots, while one populated owner retains its single-claim lifecycle.
#[semio_framework_async_macros::async_test]
async fn empty_turns_bypass_patch_transport_while_one_populated_owner_claims_once() {
    for session in 1..=semio_framework::kernel::UI_TURN_PATCH_TRANSPORT_SLOTS * 2 {
        let result = TurnResult {
            ui_patches: semio_framework::kernel::UiTurnPatches::default(),
            effects: vec![],
            presence: vec![],
            next_wake: None,
            status: semio_framework::kernel::TurnStatus::Idle,
            fuel_used: 0,
            command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
            cold_pair_ingress: semio_framework::kernel::ColdPairIngressStatus::Idle,
            lifecycle_receipt: None,
            ui_patch_receipt: None,
        };
        let bridged = to_actor_turn_result(result, session as u64, 0, 0).await.expect("validated empty turn");
        assert!(bridged.ui_patches.is_empty());
        assert_eq!(bridged.ui_patch_receipt, None);
    }

    let mut patches = semio_framework::kernel::UiTurnPatches::default();
    let patch = serde_json::from_value(serde_json::json!({ "surface": "stack-authority", "baseRevision": 0, "revision": 1, "ops": [] })).expect("one patch fixture");
    patches.try_push_ui_patch(patch).expect("one patch owner");
    let receipt = semio_framework::kernel::ActorUiPatchReceipt { lifetime: semio_framework::kernel::ActorInstanceLifetime { activation_generation: 1, instance_id: 7, guest_lifetime: 13 }, patch_sequence: 1 };
    let result = TurnResult {
        ui_patches: patches,
        effects: vec![],
        presence: vec![],
        next_wake: None,
        status: semio_framework::kernel::TurnStatus::Idle,
        fuel_used: 0,
        command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        cold_pair_ingress: semio_framework::kernel::ColdPairIngressStatus::Idle,
        lifecycle_receipt: None,
        ui_patch_receipt: Some(receipt),
    };
    let session = 800_001;
    let bridged = to_actor_turn_result(result, session, 0, 0).await.expect("populated turn");
    assert_eq!(bridged.ui_patch_receipt, Some(receipt));
    let lease = semio_framework::kernel::UiTurnPatchTransportLease::try_from_token(&bridged.ui_patches, session).expect("one exact claim");
    assert!(semio_framework::kernel::UiTurnPatchTransportLease::try_from_token(&bridged.ui_patches, session).is_err());
    let mut owner = lease.take_owner().unwrap_or_else(|_| panic!("one exact owner"));
    assert_eq!(owner.len(), 1);
    while !owner.close_step() {}
    assert!(semio_framework::kernel::UiTurnPatchTransportLease::try_from_token(&bridged.ui_patches, session).is_err());
}
//#endregion 🔖️BudgetBridge

//#region 🔖️LanePriorityAndEpochYield
/// 🎯️ A single admitted grant can carry mixed-lane authorities. One bounded pump selects
/// the interactive authority first, then later pumps preserve background arrival order.
#[semio_framework_async_macros::async_test]
async fn an_interactive_grant_is_executed_before_background_grants_queued_the_same_pump() {
    const BACKGROUND_ACTORS: u64 = 5;
    let mock = Arc::new(MockGuestRuntime::new().await);
    let (transport, probe) = LoopbackTransport::paired().await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport)).await;

    let package = PackageRef { package: PackageId("lane-priority".to_string()), hash: PackageHash([90u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let mut envelopes = Vec::new();
    for offset in 0..BACKGROUND_ACTORS {
        let actor = ActorId::new(0, 0, u32::try_from(200 + offset).expect("actor ordinal"), 0).await;
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
        shard.register(actor, instance);
        mock.script_turn(actor, MockGuestRuntime::idle_turn().await).await;
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
    let interactive_instance = mock.instantiate(&compiled, interactive_actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
    shard.register(interactive_actor, interactive_instance);
    let mut interactive_turn = MockGuestRuntime::idle_turn().await;
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
    ShardFrame::Grant { actor: interactive_actor, budget: interactive_budget, envelopes }.pack_encode(&mut bytes).await;
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
        ShardOutcome::Turn { actor, result } => {
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
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 2, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
    mock.script_deadline_exceeded(actor).await;

    let (transport, probe) = LoopbackTransport::paired().await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport)).await;
    shard.register(actor, instance);
    probe.push_inbound(encode_payload_envelope(actor, 1, Payload::Event { bytes: serde_json::to_vec(&fixture_instance_close_event()).expect("encode") }).await).await;

    let driven = pump(&mut shard).await.expect("pump");
    assert_eq!(driven, 1);

    let outbound = probe.take_outbound().await;
    assert_eq!(outbound.len(), 1);
    let outcome = decode_outcome(&outbound[0]).await;
    match outcome {
        ShardOutcome::Turn { actor: reported, result } => {
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
    let mut shard = ShardLoop::new(runtime, ShardTransports::Loopback(transport)).await;

    match shard.drive_one().await {
        ShardDrive::Fault { consumed_epoch: Some(1), work_remains: false, terminal_frame: true, .. } => {}
        _ => panic!("malformed ingress must consume epoch one and terminalize without retained work"),
    }
    assert_eq!(shard.take_terminal_frame(), Some(raw), "terminal retrieval transfers the exact raw frame");
    assert!(shard.take_terminal_frame().is_none(), "one terminal owner is retrieved exactly once");
    assert!(!shard.has_pending_work(), "terminal ownership is observable, not a hot-resubmit readiness condition");
}

#[semio_framework_async_macros::async_test]
async fn terminal_capacity_plus_one_parks_then_rearms_once_at_the_fifo_tail() {
    let (transport, probe) = LoopbackTransport::paired().await;
    let runtime = Arc::new(GuestRuntimes::Mock(Arc::new(MockGuestRuntime::new().await)));
    let mut shard = ShardLoop::new(runtime, ShardTransports::Loopback(transport)).await;
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
    let mut shard = ShardLoop::new(runtime, ShardTransports::Loopback(transport)).await;
    for index in 0..SHARD_DEFERRED_ITEMS {
        let raw = vec![0xfd, index as u8];
        shard.terminal_frames.try_push(raw.clone(), raw.len()).expect("fill terminal ring exactly");
    }
    let envelopes = (0..=SHARD_DEFERRED_ITEMS)
        .map(|seq| Envelope { to: actor, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Background, seq: seq as u64, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Cancel { seq: seq as u64 } })
        .collect();
    let mut raw = Vec::new();
    ShardFrame::Grant { actor, budget: semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Background), envelopes }.pack_encode(&mut raw).await;
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
    ShardFrame::Register { actor }.pack_encode(&mut bytes).await;
    let runtime = Arc::new(GuestRuntimes::Mock(Arc::new(MockGuestRuntime::new().await)));
    let mut shard = ShardLoop::new(runtime, ShardTransports::Loopback(LoopbackTransport::default())).await;
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

#[semio_framework_async_macros::async_test]
async fn replay_failure_and_actor_loss_enter_one_close_funnel_before_reporting() {
    let _replay_authority = replay_test_authority();
    let process_pages_before = JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire);
    let abi_bytes_before = JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire);
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(74);
    let package = PackageRef { package: PackageId("replay-close-funnel".into()), hash: PackageHash([74; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4_096, max_frames: 1 }).await.expect("mock instantiate");
    let (transport, probe) = LoopbackTransport::paired().await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport)).await;
    shard.register(actor, instance);
    let turn = test_job_turn(actor, 24, 0, 0);
    let kind = "close-funnel".to_string();
    let input = vec![1, 2, 3];
    let request = JobReplayRequest::from_spawn(&kind, &input);
    let mut seed = MountedReplaySeed::new(actor.0, 24, turn, request, JobPlacement::Inline, kind, input).expect("mounted seed");
    seed.seed.as_mut().expect("fixed seed").kind_pages = JOB_REPLAY_KIND_PAGE_CAPACITY;
    shard.replay_seeds[0] = Some(seed);
    shard.granted_budgets.insert(actor.0, semio_framework_actor::Budget { fuel: 1, wall_ms: 1, ..semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance) });
    let error = shard.drive_replay_seed().await.expect_err("capture admission must fault");
    assert!(error.to_string().contains("kind page admission refused"));
    let seed = shard.replay_seeds[0].as_ref().expect("faulted seed remains discoverable");
    assert_eq!(seed.phase, ReplaySeedPhase::Closing);
    assert!(matches!(seed.close_reason, Some(ReplaySeedCloseReason::Fault { stage: "capture-kind", .. })));
    shard.begin_replay_seed_close(0, ReplaySeedCloseReason::Cancelled);
    assert!(matches!(shard.replay_seeds[0].as_ref().expect("first close reason").close_reason, Some(ReplaySeedCloseReason::Fault { stage: "capture-kind", .. })));
    drain_replay_lifecycle(&mut shard, actor.0).await;

    let turn = test_job_turn(actor, 25, 0, 0);
    let kind = "actor-loss".to_string();
    let input = vec![4, 5, 6];
    let request = JobReplayRequest::from_spawn(&kind, &input);
    let mut retained = MountedReplaySeed::new(actor.0, 25, turn, request, JobPlacement::Inline, kind, input).expect("retained seed");
    retained.phase = ReplaySeedPhase::Retained;
    shard.replay_seeds[0] = Some(retained);
    shard.running_jobs.insert((actor.0, 25));
    shard.job_turns.insert((actor.0, 25), turn);
    shard.job_authorities.insert((actor.0, 25), JobAuthority { turn, request });
    shard.job_placement.insert((actor.0, 25), JobPlacement::Inline);
    shard.replay_seed_refusals[0] = Some(ReplaySpawnRefusal::new(actor.0, 26, b"refused", "kind".into(), vec![7]));
    shard.unregister(actor).await;
    assert!(matches!(shard.replay_seeds[0].as_ref().expect("unregistered seed").close_reason, Some(ReplaySeedCloseReason::ActorLost)));
    assert_eq!(shard.replay_seeds[0].as_ref().expect("unregistered seed").phase, ReplaySeedPhase::Closing);
    assert!(!shard.replay_seed_refusals[0].as_ref().expect("unregistered refusal").publish);
    assert!(!shard.running_jobs.contains(&(actor.0, 25)));
    drain_replay_lifecycle(&mut shard, actor.0).await;
    assert!(probe.take_outbound().await.is_empty(), "actor-lost refusal never publishes to a retired actor");
    assert_eq!(JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire), process_pages_before);
    assert_eq!(JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire), abi_bytes_before);
}

#[semio_framework_async_macros::async_test]
async fn mounted_replay_rejects_wrong_route_seed_generation_and_worker_before_work_and_closes_one_owner_per_sub_eight_ms_opportunity() {
    let _replay_authority = replay_test_authority();
    let actor = ActorId(63);
    let job = 11;
    let turn = test_job_turn(actor, job, 0, 0);
    let kind = "action-bus.fixture".to_string();
    let input = vec![1, 2, 3];
    let request = JobReplayRequest::from_spawn(&kind, &input);
    let process_pages_before = JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire);
    let abi_bytes_before = JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire);
    let runtime = Arc::new(GuestRuntimes::Mock(Arc::new(MockGuestRuntime::new().await)));
    let mut shard = ShardLoop::new(runtime, ShardTransports::Loopback(LoopbackTransport::default())).await;
    shard.replay_seeds[0] = Some(MountedReplaySeed::new(actor.0, job, turn, request, JobPlacement::Inline, kind, input).expect("fixed mounted replay seed"));

    let initial_phase = shard.replay_seeds[0].as_ref().expect("mounted seed").phase;
    shard.granted_budgets.insert(actor.0, semio_framework_actor::Budget { fuel: 0, wall_ms: 1, ..semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance) });
    assert!(!shard.drive_replay_seed().await.expect("zero fuel refuses unchanged"));
    assert_eq!(shard.replay_seeds[0].as_ref().expect("unchanged seed").phase, initial_phase);
    shard.granted_budgets.insert(actor.0, semio_framework_actor::Budget { fuel: 1, wall_ms: 0, ..semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance) });
    assert!(!shard.drive_replay_seed().await.expect("expired deadline refuses unchanged"));
    assert_eq!(shard.replay_seeds[0].as_ref().expect("unchanged seed").phase, initial_phase);

    let mut wrong_route = request;
    wrong_route.digest[0] ^= 1;
    assert!(shard.validate_replay_request(actor.0, turn, wrong_route).is_err());
    assert!(shard.validate_replay_request(actor.0, JobTurn { operation: JobOperation { seed: turn.operation.seed ^ 1, ..turn.operation }, ..turn }, request).is_err());
    assert!(shard.validate_replay_request(actor.0, JobTurn { operation: JobOperation { generation: turn.operation.generation + 1, ..turn.operation }, ..turn }, request).is_err());
    shard.replay_seeds[0].as_mut().expect("mounted seed").phase = ReplaySeedPhase::Retained;
    assert!(shard.begin_replay_seed(actor.0, turn, request, 0, 0).is_err());
    assert!(shard.begin_replay_seed(actor.0, turn, request, 1, u16::MAX).is_err());

    shard.replay_seeds[0].as_mut().expect("mounted seed").phase = ReplaySeedPhase::CaptureInput;
    shard.granted_budgets.insert(actor.0, semio_framework_actor::Budget { fuel: 1, wall_ms: 1, ..semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance) });
    let stale_started = std::time::Instant::now();
    assert!(shard.drive_replay_seed().await.expect("stale actor starts exact close"));
    assert!(stale_started.elapsed() < std::time::Duration::from_millis(8));
    assert_eq!(shard.replay_seeds[0].as_ref().expect("stale seed remains discoverable").phase, ReplaySeedPhase::Closing);
    while shard.replay_seeds[0].is_some() {
        shard.granted_budgets.insert(actor.0, semio_framework_actor::Budget { fuel: 1, wall_ms: 1, ..semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance) });
        let started = std::time::Instant::now();
        assert!(shard.drive_replay_seed().await.expect("one close opportunity"));
        assert!(started.elapsed() < std::time::Duration::from_millis(8));
    }
    assert_eq!(JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire), process_pages_before);
    assert_eq!(JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire), abi_bytes_before);
}

#[semio_framework_async_macros::async_test]
async fn mounted_cancel_marks_the_exact_replay_seed_for_incremental_close_before_another_job_step() {
    let _replay_authority = replay_test_authority();
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(65);
    let job = 13;
    let package = PackageRef { package: PackageId("mounted-replay-cancel".to_string()), hash: PackageHash([17; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
    let turn = test_job_turn(actor, job, 0, 0);
    let kind = "action-bus.cancel".to_string();
    let input = vec![2, 3, 5, 7];
    let request = JobReplayRequest::from_spawn(&kind, &input);
    let process_pages_before = JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire);
    let abi_bytes_before = JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire);
    let (transport, _) = LoopbackTransport::paired().await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport)).await;
    shard.register(actor, instance);
    let mut seed = MountedReplaySeed::new(actor.0, job, turn, request, JobPlacement::Inline, kind, input).expect("fixed mounted replay seed");
    seed.phase = ReplaySeedPhase::Retained;
    shard.replay_seeds[0] = Some(seed);
    shard.running_jobs.insert((actor.0, job));
    shard.job_turns.insert((actor.0, job), turn);
    shard.job_authorities.insert((actor.0, job), JobAuthority { turn, request });
    shard.job_placement.insert((actor.0, job), JobPlacement::Inline);
    let mut cancel = MockGuestRuntime::idle_turn().await;
    cancel.effects.push(Effect::CancelJob { job });
    mock.script_turn(actor, cancel).await;

    let budget = shard.granted_budget(actor.0);
    let lane = shard.actor_lane(actor.0);
    assert!(!shard.execute_turn_for(actor.0, &Event::Wake, budget, lane).await.expect("mounted cancel turn"));
    assert_eq!(mock.cancel_admissions(), 1);
    assert_eq!(mock.step_admissions(), 0, "cancel closes before another guest job step");
    assert_eq!(shard.replay_seeds[0].as_ref().expect("cancelled seed remains discoverable").phase, ReplaySeedPhase::Closing);
    assert!(!shard.running_jobs.contains(&(actor.0, job)));
    while shard.replay_seeds[0].is_some() {
        shard.granted_budgets.insert(actor.0, semio_framework_actor::Budget { fuel: 1, wall_ms: 1, ..semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance) });
        let started = std::time::Instant::now();
        assert!(shard.drive_replay_seed().await.expect("one cancelled-owner close opportunity"));
        assert!(started.elapsed() < std::time::Duration::from_millis(8));
    }
    assert_eq!(JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire), process_pages_before);
    assert_eq!(JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire), abi_bytes_before);
}
//#endregion 🔖️LanePriorityAndEpochYield
