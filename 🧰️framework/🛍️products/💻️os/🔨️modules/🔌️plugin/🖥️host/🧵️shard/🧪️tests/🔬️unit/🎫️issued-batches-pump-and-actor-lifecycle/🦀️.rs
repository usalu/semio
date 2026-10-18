/// 🎫️ Issued batches, pump and actor lifecycle: the issued shard batch ledger, the grant transport of the original retained authority, the pump helpers, the shared instance close event fixture and the pump, unregister and unregistered-actor laws.

#[semio_framework_async_macros::async_test]
async fn issued_shard_batch_has_one_original_ledger_and_one_terminal_return(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/📦️batch/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let input:semio_framework_actor::RetainedTurnInput=serde_json::from_value(law["input"].clone()).unwrap();let budget=semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);let mut turn=IssuedShardTurn::new(budget,input,law["envelopes"].as_u64().unwrap()as usize).unwrap();let mut completions=0;let mut original=input;
 for (index,spent) in law["spent"].as_array().unwrap().iter().enumerate(){
  let issued=turn.issue_envelope(index).unwrap();assert_eq!(issued,original);let progress=serde_json::from_value(spent.clone()).unwrap();let receipt=issued.return_original(progress).unwrap();let mut forged=receipt;forged.input.epoch+=1;
  let(refused,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||turn.settle_envelope(index,forged));assert!(refused.is_err());assert_eq!(serde_json::to_value([physical.requested_bytes,physical.released_bytes]).unwrap(),law["refusalHeap"]);
  let accepted=turn.settle_envelope(index,receipt).unwrap();original=semio_framework_actor::RetainedTurnInput{grant:receipt.remaining,..issued};if let Some(whole)=accepted{completions+=1;assert_eq!(index+1,law["envelopes"].as_u64().unwrap()as usize);whole.validate_for(input).unwrap();assert_eq!(serde_json::to_value(whole.remaining).unwrap(),law["remaining"])}else{assert!(index+1<law["envelopes"].as_u64().unwrap()as usize)}
  let(duplicate,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||turn.settle_envelope(index,receipt));assert_eq!(serde_json::to_value(duplicate.is_err()).unwrap(),law["duplicateRefused"]);assert_eq!(serde_json::to_value([physical.requested_bytes,physical.released_bytes]).unwrap(),law["refusalHeap"]);
 }
 assert_eq!(serde_json::to_value(completions).unwrap(),law["completionCount"]);assert!(turn.issue_envelope(3).is_err());
}

#[semio_framework_async_macros::async_test]
async fn shard_grant_transports_original_retained_authority_independently_of_scaled_scheduling() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/📃️policy/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let original: semio_framework_actor::RetainedTurnInput = serde_json::from_value(law["input"].clone()).unwrap();
    let budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive).scaled(0.125).await;
    let frame = ShardFrame::Grant { retained: original, actor: ActorId(71), budget, envelopes: Vec::new() };
    let mut bytes = Vec::with_capacity(256);
    let backing = bytes.as_ptr();
    let (_, encoded) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_async::poll::resolve_ready(frame.pack_encode(&mut bytes)));
    assert_eq!(encoded.requested_bytes, 0);
    assert_eq!(encoded.released_bytes, 0);
    assert_eq!(bytes.as_ptr(), backing);
    let mut cursor = 0;
    let (decoded, physical) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_async::poll::resolve_ready(ShardFrame::pack_decode(&bytes, &mut cursor)));
    assert_eq!(physical.requested_bytes, 0);
    assert_eq!(physical.released_bytes, 0);
    let decoded = decoded.unwrap();
    assert_eq!(decoded, frame);
    let ShardFrame::Grant { budget: decoded_budget, retained: decoded_retained, .. } = decoded else { panic!("original grant frame") };
    assert_eq!(serde_json::to_value(decoded_retained).unwrap(), law["input"]);
    assert_eq!(decoded_retained, original);
    assert_eq!(decoded_budget, budget);
    assert_eq!(cursor, bytes.len());
}

#[semio_framework_async_macros::async_test]
async fn shard_grant_invalid_original_operation_preserves_wire_backing_without_heap() {
    let mut original = original_retained_turn();original.operation = 0;
    let frame = ShardFrame::Grant { retained: original, actor: ActorId(71), budget: semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive), envelopes: Vec::new() };
    let mut bytes = Vec::with_capacity(256);bytes.extend_from_slice(&[11,13]);let pointer=bytes.as_ptr();let capacity=bytes.capacity();
    let(result,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_async::poll::resolve_ready(frame.pack_encode(&mut bytes)));
    assert_eq!(result.unwrap_err(),semio_framework_actor::pack::PackError::InvalidRetainedTurn("operation identity is absent"));assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert_eq!(bytes.as_ptr(),pointer);assert_eq!(bytes.capacity(),capacity);assert_eq!(bytes,[11,13]);let ShardFrame::Grant{retained,..}=frame else{unreachable!()};assert_eq!(retained,original);
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
        shard.granted_budgets.insert(actor, IssuedShardTurn::new(semio_framework_actor::Budget { fuel: 1, wall_ms: 1, ..semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance) }, semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, 1).expect("declared replay authority"));
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
        shard.granted_budgets.insert(actor.0, IssuedShardTurn::new(semio_framework_actor::Budget { fuel: 1, wall_ms: 1, ..semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance) }, semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, 1).expect("declared replay authority"));
        assert!(shard.drive_replay_seed().await.expect("replay retention opportunity"));
    }
    panic!("bounded replay retention exhausted");
}

async fn encode_event_envelope(to: ActorId, seq: u64, event: &Event) -> Vec<u8> {
    encode_payload_envelope(to, seq, Payload::Event { bytes: serde_json::to_vec(event).expect("encode event") }).await
}

#[test]
fn instance_close_event_matches_the_shared_first_party_fixture_and_serde_structure() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️tests/🧫️fixtures/🔣️.json")).expect("shared actor lifecycle fixture");
    let mut independent = fixture["vectors"].as_array().expect("lifecycle vectors").iter().find(|row| row["value"]["kind"] == "close").expect("close vector")["value"].clone();
    independent.as_object_mut().expect("close value object").remove("kind");
    let first_party: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&fixture_instance_close_request())).expect("first-party close JSON");
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
    ShardFrame::Envelope(envelope).pack_encode(&mut bytes).await.expect("declared original frame authority");
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
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { retained: semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
    let mut scripted = MockGuestRuntime::idle_turn(original_retained_turn()).await;
    scripted.fuel_used = 42;
    mock.script_turn(actor, scripted).await;

    let (transport, probe) = LoopbackTransport::paired().await;
    probe.push_inbound(encode_event_envelope(actor, 1, &fixture_instance_close_event()).await).await;

    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    shard.register(actor, instance);
    assert!(shard.is_registered(actor).await);

    let driven = pump(&mut shard).await.expect("pump succeeds");
    assert_eq!(driven, 1, "exactly one actor had a buffered envelope");

    let outbound = probe.take_outbound().await;
    assert_eq!(outbound.len(), 1, "one ShardOutcome sent back");
    let outcome = decode_outcome(&outbound[0]).await;
    match outcome {
        ShardOutcome::Turn { actor: reported, result, .. } => {
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

    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
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
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { retained: semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("mock instantiate");
    let (transport, _probe) = LoopbackTransport::paired().await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    shard.register(actor, instance);
    assert_eq!(shard.actor_count().await, 1);
    shard.unregister(actor).await;
    assert_eq!(shard.actor_count().await, 0);
    assert!(!shard.is_registered(actor).await);
}
