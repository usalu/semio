
use super::*;
use crate::contract::{CommandId, DeviceId, SessionId, TenantId, TraceContext};
use crate::storage::MemoryAuthorityStore;

fn key(kind: &str) -> ActorKey {
    ActorKey { tenant: TenantId("t1".into()), kind: kind.into(), id: "c1".into() }
}

fn command(target: &ActorKey, kind: &str, idempotency: Option<&str>) -> CommandEnvelope {
    CommandEnvelope {
        command_id: CommandId(format!("cmd-{kind}")),
        kind: kind.into(),
        version: 1,
        target: target.clone(),
        scope: Scope("space-1".into()),
        principal: Principal::User { id: "alice".into() },
        session: Some(SessionId("s1".into())),
        device: Some(DeviceId("d1".into())),
        payload: vec![3],
        causal_frontier: None,
        client_hlc: HybridLogicalClock { millis: 1, counter: 0 },
        expected_revision: None,
        idempotency_key: idempotency.map(|value| IdempotencyKey(value.into())),
        capability_proof: None,
        trace: TraceContext::default(),
    }
}

async fn bus() -> CommandBus<MemoryAuthorityStore> {
    allowing(Box::new(|_| PolicyDecision::Allow)).await
}

async fn allowing(hook: PolicyHook) -> CommandBus<MemoryAuthorityStore> {
    let mut bus = CommandBus::new(AuthorityDirectory::new(), MemoryAuthorityStore::default(), hook);
    bus.register(Deciders::Counter(CounterDecider)).await;
    bus
}

fn tick(millis: u64) -> HybridLogicalClock {
    HybridLogicalClock { millis, counter: 0 }
}

//#region 🔖️Admit
#[semio_framework_async_macros::async_test]
async fn a_command_for_an_unserved_actor_kind_is_rejected_as_unknown() {
    let mut bus = bus().await;
    let outcome = bus.submit(command(&key("ghost"), "ghost.poke", None), tick(1)).await;
    match outcome {
        CommandOutcome::Rejected { reason: Rejection::UnknownCommandKind { command_kind }, .. } => {
            assert_eq!(command_kind, "ghost.poke");
        }
        other => panic!("expected unknown command kind, got {other:?}"),
    }
    assert!(!bus.directory().is_active(&key("ghost")));
}
//#endregion 🔖️Admit

//#region 🔖️Deduplicate
#[semio_framework_async_macros::async_test]
async fn an_accepted_turn_emits_events_and_bumps_the_revision() {
    let mut bus = bus().await;
    let outcome = bus.submit(command(&key(COUNTER), "counter.increment", Some("k1")), tick(1)).await;
    match outcome {
        CommandOutcome::Accepted { receipt, events, .. } => {
            assert_eq!(events.len(), 1);
            assert_eq!(events[0].seq, 1);
            assert_eq!(events[0].stream, key(COUNTER));
            assert_eq!(events[0].hlc, tick(1));
            assert_eq!(receipt.revision, Revision(1));
        }
        other => panic!("expected acceptance, got {other:?}"),
    }
    let activation = bus.directory().activation(&key(COUNTER)).expect("placed");
    assert_eq!(read_counter(&activation.state.bytes), 3);
    assert_eq!(activation.mailbox_seq, 1);
}

#[semio_framework_async_macros::async_test]
async fn resubmitting_one_idempotency_key_returns_the_identical_receipt_and_no_second_event() {
    let mut bus = bus().await;
    let first = bus.submit(command(&key(COUNTER), "counter.increment", Some("k1")), tick(1)).await;
    let second = bus.submit(command(&key(COUNTER), "counter.increment", Some("k1")), tick(9)).await;

    let CommandOutcome::Accepted { receipt: original, events: appended, .. } = first else {
        panic!("first submission must be accepted");
    };
    let CommandOutcome::Accepted { receipt: replayed, events: none, .. } = second else {
        panic!("retry must be accepted");
    };
    assert_eq!(appended.len(), 1);
    assert!(none.is_empty());
    assert_eq!(replayed, original);

    let activation = bus.directory().activation(&key(COUNTER)).expect("placed");
    assert_eq!(activation.mailbox_seq, 1);
    assert_eq!(read_counter(&activation.state.bytes), 3);
    assert_eq!(bus.store().last_seq(&key(COUNTER)).await.unwrap(), 1);
}
//#endregion 🔖️Deduplicate

//#region 🔖️Authorize
#[semio_framework_async_macros::async_test]
async fn a_policy_denial_rejects_before_the_actor_is_placed() {
    let mut bus = allowing(Box::new(|_| PolicyDecision::Deny { reason: "no grant".into() })).await;
    let outcome = bus.submit(command(&key(COUNTER), "counter.increment", Some("k1")), tick(1)).await;
    match outcome {
        CommandOutcome::Rejected { reason: Rejection::Unauthorized { detail }, .. } => {
            assert_eq!(detail, "no grant");
        }
        other => panic!("expected unauthorized, got {other:?}"),
    }
    assert!(!bus.directory().is_active(&key(COUNTER)));
}
//#endregion 🔖️Authorize

//#region 🔖️Fence
#[semio_framework_async_macros::async_test]
async fn a_stale_expected_revision_conflicts_and_reports_the_actual_one() {
    let mut bus = bus().await;
    bus.submit(command(&key(COUNTER), "counter.increment", Some("k1")), tick(1)).await;

    let mut stale = command(&key(COUNTER), "counter.increment", Some("k2"));
    stale.expected_revision = Some(Revision(0));
    match bus.submit(stale, tick(2)).await {
        CommandOutcome::Rejected { reason: Rejection::RevisionConflict { expected, actual }, .. } => {
            assert_eq!(expected, Revision(0));
            assert_eq!(actual, Revision(1));
        }
        other => panic!("expected revision conflict, got {other:?}"),
    }
    assert_eq!(bus.store().last_seq(&key(COUNTER)).await.unwrap(), 1);
}

#[semio_framework_async_macros::async_test]
async fn a_matching_expected_revision_passes_the_fence() {
    let mut bus = bus().await;
    bus.submit(command(&key(COUNTER), "counter.increment", Some("k1")), tick(1)).await;

    let mut fresh = command(&key(COUNTER), "counter.increment", Some("k2"));
    fresh.expected_revision = Some(Revision(1));
    assert!(matches!(bus.submit(fresh, tick(2)).await, CommandOutcome::Accepted { .. }));
    assert_eq!(read_counter(&bus.directory().activation(&key(COUNTER)).unwrap().state.bytes), 6);
}
//#endregion 🔖️Fence

//#region 🔖️Decide
#[semio_framework_async_macros::async_test]
async fn a_decider_rejection_becomes_a_rejected_outcome() {
    let mut bus = bus().await;
    match bus.submit(command(&key(COUNTER), "counter.forbid", Some("k1")), tick(1)).await {
        CommandOutcome::Rejected { reason: Rejection::Invalid { detail }, .. } => {
            assert_eq!(detail, "counter refuses");
        }
        other => panic!("expected invalid rejection, got {other:?}"),
    }
    assert_eq!(bus.store().last_seq(&key(COUNTER)).await.unwrap(), 0);
}

#[semio_framework_async_macros::async_test]
async fn a_decider_deferral_becomes_a_pending_outcome() {
    let mut bus = bus().await;
    match bus.submit(command(&key(COUNTER), "counter.rebuild", Some("k1")), tick(1)).await {
        CommandOutcome::Pending { receipt, process } => {
            assert_eq!(process, ProcessId("rebuild-1".into()));
            assert_eq!(receipt.revision, Revision(0));
        }
        other => panic!("expected pending, got {other:?}"),
    }
    assert_eq!(bus.store().last_seq(&key(COUNTER)).await.unwrap(), 0);
}
//#endregion 🔖️Decide

//#region 🔖️Saga
#[semio_framework_async_macros::async_test]
async fn the_outbox_drains_exactly_once_across_two_calls() {
    let mut bus = bus().await;
    bus.submit(command(&key(COUNTER), "counter.increment", Some("k1")), tick(1)).await;
    bus.submit(command(&key(COUNTER), "counter.increment", Some("k2")), tick(2)).await;

    let mut runner = SagaRunner::new();
    runner.register(Sagas::Echo(EchoSaga));

    let first = runner.drain_outbox(bus.store_mut(), 64).await;
    assert_eq!(first.len(), 2);
    assert!(first.iter().all(|follow_up| follow_up.kind == "counter.increment"));

    let second = runner.drain_outbox(bus.store_mut(), 64).await;
    assert!(second.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn effects_reach_the_outbox_without_an_event() {
    let mut bus = bus().await;
    bus.submit(command(&key(COUNTER), "counter.audit", Some("k1")), tick(1)).await;
    let pending = bus.store().pending_outbox(64).await.unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].kind, "counter.audited");
    assert!(pending[0].event.is_none());
}
//#endregion 🔖️Saga

//#region 🔖️Directory
#[semio_framework_async_macros::async_test]
async fn passivation_fences_the_next_activation_with_a_higher_epoch() {
    let mut directory = AuthorityDirectory::new();
    directory.activate(key(COUNTER), "authority").unwrap();
    let first = directory.activation_epoch(&key(COUNTER)).unwrap();
    directory.passivate(&key(COUNTER));
    assert!(!directory.is_active(&key(COUNTER)));
    directory.activate(key(COUNTER), "authority").unwrap();
    assert!(directory.activation_epoch(&key(COUNTER)).unwrap() > first);
}
//#endregion 🔖️Directory
