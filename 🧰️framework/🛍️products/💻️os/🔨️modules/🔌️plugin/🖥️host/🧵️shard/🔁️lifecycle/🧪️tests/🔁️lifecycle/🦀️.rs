use super::*;

fn lane() -> semio_framework_actor::Lane {
    semio_framework_actor::Lane::Interactive
}
fn grant() -> semio_framework_actor::Budget {
    semio_framework_actor::lane_defaults::budget_for(lane())
}
fn event(sequence: u64) -> Event {
    let Event::InstanceClose(mut request) = fixture_instance_close_event() else { unreachable!() };
    request.request_sequence = sequence;
    Event::InstanceClose(request)
}
fn deadline() -> semio_framework::Fault {
    semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.reactor-turn-deadline"), "retained test verdict").with_retryable(true)
}
async fn setup() -> (ShardLoop, Arc<MockGuestRuntime>, Arc<Mutex<Vec<Vec<u8>>>>) {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let transport = LoopbackTransport::default();
    let outbound = Arc::clone(&transport.outbound);
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(Arc::clone(&mock))), ShardTransports::Loopback(transport)).await;
    let package = PackageRef { package: PackageId("lifecycle-retry".into()), hash: PackageHash([0; 32]) };
    let compiled = mock.compile(&package, &[]).await.unwrap();
    for actor in [ActorId(1), ActorId(2)] {
        let instance = mock.instantiate(&compiled, actor, &[], &turn_budget_from_grant(grant()).await).await.unwrap();
        assert!(shard.register(actor, instance).is_ok());
        shard.granted_budgets.insert(actor.0, grant());
    }
    (shard, mock, outbound)
}
fn enqueue(shard: &mut ShardLoop, actor: u64, sequence: u64, bytes: usize) {
    shard.enqueue_authority(lane(), DeferredAuthority::Event { actor, event: event(sequence) }, bytes).unwrap();
}

#[semio_framework_async_macros::async_test]
async fn neutral_retry_order_uses_the_production_selector() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for trace in fixture["traces"].as_array().unwrap() {
        let (mut shard, _, _) = setup().await;
        for label in trace["queued"].as_array().unwrap() {
            let label = label.as_str().unwrap();
            enqueue(&mut shard, if label.starts_with('A') { 1 } else { 2 }, u64::from(label.as_bytes()[1] - b'0'), 37);
            if label == trace["retry"].as_str().unwrap() {
                let ring = &mut shard.pending_interactive;
                let slot = ring.order[ring.len - 1];
                ring.slots[slot].as_mut().unwrap().owner.retry = LifecycleRetry::YieldPeer;
            }
        }
        let mut actual = Vec::new();
        while let Some(owner) = shard.select_pending_authority() {
            let DeferredAuthority::Event { actor, event: Event::InstanceClose(request) } = owner.authority else { unreachable!() };
            actual.push(format!("{}{}", if actor == 1 { 'A' } else { 'B' }, request.request_sequence));
        }
        assert_eq!(serde_json::json!(actual), trace["expected"], "{}", trace["id"]);
        assert_eq!(shard.pending_interactive.bytes, 0);
    }
    eprintln!("[DEBUG] retained lifecycle neutral selector traces=4");
}

#[semio_framework_async_macros::async_test]
async fn retry_keeps_exact_event_budget_credit_and_one_peer_order() {
    let (mut shard, mock, outbound) = setup().await;
    mock.script_guest_fault(ActorId(1), deadline()).await;
    for actor in [ActorId(1), ActorId(1), ActorId(2)] {
        mock.script_turn(actor, MockGuestRuntime::idle_turn().await).await;
    }
    enqueue(&mut shard, 1, 1, 101);
    enqueue(&mut shard, 1, 2, 202);
    enqueue(&mut shard, 2, 1, 303);
    assert_eq!(shard.pump().await.unwrap(), 1);
    assert!(outbound.lock().unwrap().is_empty());
    assert!(shard.has_lifecycle_retry() && shard.can_accept_primed_frame());
    assert_eq!((shard.pending_interactive.len, shard.pending_interactive.bytes), (3, 606));
    shard.granted_budgets.insert(1, semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance));
    for _ in 0..3 {
        shard.pump().await.unwrap();
    }
    let turns = mock.observed_turns.lock().unwrap();
    assert_eq!(turns.iter().map(|(actor, _, _)| *actor).collect::<Vec<_>>(), vec![1, 2, 1, 1]);
    assert_eq!(turns[0].1, turns[2].1);
    assert_eq!(turns[0].2, turns[2].2);
    assert_eq!(turns[0].2, turns[3].2);
    assert_eq!(outbound.lock().unwrap().len(), 3);
    assert_eq!((shard.pending_interactive.len, shard.pending_interactive.bytes), (0, 0));
    eprintln!("[DEBUG] retained lifecycle order=A1,B1,A1,A2 exact-input=1 exact-budget=1 credit=606->0");
}

#[semio_framework_async_macros::async_test]
async fn retry_refuses_replacement_and_stale_queue_cannot_reach_same_id_successor() {
    let (mut shard, mock, _) = setup().await;
    mock.script_guest_fault(ActorId(1), deadline()).await;
    enqueue(&mut shard, 1, 1, 101);
    enqueue(&mut shard, 1, 2, 202);
    shard.pump().await.unwrap();
    let allocation = shard.current_allocation(1).unwrap();
    let package = PackageRef { package: PackageId("replacement".into()), hash: PackageHash([0; 32]) };
    let compiled = mock.compile(&package, &[]).await.unwrap();
    let fresh = mock.instantiate(&compiled, ActorId(1), &[], &turn_budget_from_grant(grant()).await).await.unwrap();
    let rejected = match shard.register(ActorId(1), fresh) {
        Err(rejected) => rejected,
        Ok(_) => panic!("same raw actor cannot overwrite a live instance"),
    };
    assert_eq!(rejected.reason, ShardRegistrationReason::Occupied);
    shard.unregister(ActorId(1)).await;
    assert!(shard.register(ActorId(1), rejected.instance).is_ok());
    assert_ne!(shard.current_allocation(1), Some(allocation));
    mock.script_turn(ActorId(1), MockGuestRuntime::idle_turn().await).await;
    enqueue(&mut shard, 1, 3, 303);
    for _ in 0..3 {
        shard.pump().await.unwrap();
    }
    assert_eq!(mock.observed_events(ActorId(1)).await, vec![event(1), event(3)]);
    assert_eq!(shard.pending_interactive.bytes, 0);
    eprintln!("[DEBUG] retained lifecycle duplicate-refused=1 stale-successor-calls=0");
}

#[semio_framework_async_macros::async_test]
async fn cancellation_revokes_retry_before_another_guest_call() {
    let (mut shard, mock, _) = setup().await;
    mock.script_guest_fault(ActorId(1), deadline()).await;
    enqueue(&mut shard, 1, 1, 101);
    enqueue(&mut shard, 1, 2, 202);
    shard.pump().await.unwrap();
    shard.enqueue_authority(lane(), DeferredAuthority::Cancel(CancelCursor { actor: 1, after_job: None, owner_bytes: 5 }), 5).unwrap();
    shard.pump().await.unwrap();
    assert!(!shard.is_registered(ActorId(1)).await);
    for _ in 0..2 {
        shard.pump().await.unwrap();
    }
    assert_eq!(mock.observed_events(ActorId(1)).await, vec![event(1)]);
    assert_eq!((shard.pending_interactive.len, shard.pending_interactive.bytes), (0, 0));
    eprintln!("[DEBUG] retained lifecycle cancellation guest-retry-calls=0 credit=0");
}

#[semio_framework_async_macros::async_test]
async fn retry_occupies_the_original_fixed_lane_capacity() {
    let (mut shard, mock, _) = setup().await;
    mock.script_guest_fault(ActorId(1), deadline()).await;
    enqueue(&mut shard, 1, 1, SHARD_DEFERRED_BYTES - 255);
    for sequence in 2..=256 {
        enqueue(&mut shard, 1, sequence, 1);
    }
    shard.pump().await.unwrap();
    assert_eq!((shard.pending_interactive.len, shard.pending_interactive.bytes), (256, SHARD_DEFERRED_BYTES));
    assert_eq!(shard.pending_interactive.can_admit(1, 0), Err(AdmissionLimit::Items));
    assert_eq!(shard.pending_interactive.can_admit(0, 1), Err(AdmissionLimit::Bytes));
    eprintln!("[DEBUG] retained lifecycle fixed-capacity items=256 bytes=16777216");
}

#[semio_framework_async_macros::async_test]
async fn terminal_faults_never_create_a_lifecycle_retry() {
    for retryable in [false, true] {
        let (mut shard, mock, outbound) = setup().await;
        let mut fault = deadline();
        fault.retryable = retryable;
        if retryable {
            fault.code = semio_framework::FaultCode::new("plugin.other-fault");
        }
        mock.script_guest_fault(ActorId(1), fault).await;
        enqueue(&mut shard, 1, 1, 101);
        shard.pump().await.unwrap();
        assert!(!shard.has_lifecycle_retry());
        assert_eq!(outbound.lock().unwrap().len(), 1);
        assert_eq!(shard.pending_interactive.bytes, 0);
    }
    eprintln!("[DEBUG] retained lifecycle terminal-guest-faults=2 hidden-credit=0");
}

#[semio_framework_async_macros::async_test]
async fn unknown_transport_actors_never_allocate_host_bookkeeping() {
    let (mut shard, mock, outbound) = setup().await;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let count = fixture["unknownActorGrants"].as_u64().unwrap();
    let before = (shard.granted_budgets.len(), shard.actor_lanes.len(), shard.allocations.len());
    for raw in 3..3 + count {
        let mut bytes = Vec::new();
        ShardFrame::Grant { actor: ActorId(raw), budget: grant(), envelopes: Vec::new() }.pack_encode(&mut bytes).await;
        assert!(shard.consume_frame(bytes).await.is_ok());
    }
    assert_eq!((shard.granted_budgets.len(), shard.actor_lanes.len(), shard.allocations.len()), before);
    assert!(mock.observed_turns.lock().unwrap().is_empty());
    assert_eq!(outbound.lock().unwrap().len() as u64, count);
    assert_eq!(shard.pending_interactive.bytes + shard.pending_background.bytes, 0);
    eprintln!("[DEBUG] unknown actor grants=256 allocated-host-entries=0 guest-calls=0");
}
