
use super::*;
use crate::contract::{CommandId, DeviceId, IdempotencyKey, SessionId, TenantId, TraceContext};
use crate::test_instance::{read_counter, CounterDecider, EchoSaga, MemoryAuthorityStore, MirrorDecider, SilentSaga, TestDeciders, TestSagas, COUNTER, MIRROR};
use std::sync::atomic::{AtomicUsize, Ordering};

fn key(kind: &str) -> ActorKey {
    actor(kind, "c1")
}

fn actor(kind: &str, id: &str) -> ActorKey {
    ActorKey { tenant: TenantId("t1".into()), kind: kind.into(), id: id.into() }
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

fn sent_by(principal: Principal, mut envelope: CommandEnvelope) -> CommandEnvelope {
    envelope.principal = principal;
    envelope
}

async fn bus() -> CommandBus<MemoryAuthorityStore, TestDeciders> {
    allowing(Box::new(|_| PolicyDecision::Allow)).await
}

async fn allowing(hook: PolicyHook) -> CommandBus<MemoryAuthorityStore, TestDeciders> {
    over(MemoryAuthorityStore::default(), AuthorityDirectory::new(), hook).await
}

async fn over(store: MemoryAuthorityStore, directory: AuthorityDirectory, hook: PolicyHook) -> CommandBus<MemoryAuthorityStore, TestDeciders> {
    let mut bus = CommandBus::new(directory, store, hook);
    bus.register(TestDeciders::Counter(CounterDecider)).await;
    bus.register(TestDeciders::Mirror(MirrorDecider)).await;
    bus
}

fn tick(millis: u64) -> HybridLogicalClock {
    HybridLogicalClock { millis, counter: 0 }
}

async fn last_seq(bus: &CommandBus<MemoryAuthorityStore, TestDeciders>, actor: &ActorKey) -> u64 {
    bus.store().read().await.last_seq(actor).await.unwrap()
}

fn history(actor: &ActorKey, events: u64) -> Vec<EventRecord> {
    (1..=events).map(|seq| EventRecord { stream: actor.clone(), seq, hlc: tick(seq), kind: "counter.incremented".into(), payload: vec![3] }).collect()
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

#[semio_framework_async_macros::async_test]
async fn an_envelope_the_admission_hook_refuses_is_neither_read_nor_placed() {
    let asked = Arc::new(AtomicUsize::new(0));
    let policy = Arc::clone(&asked);
    let hook: PolicyHook = Box::new(move |_| {
        policy.fetch_add(1, Ordering::Relaxed);
        PolicyDecision::Allow
    });
    let mut bus = allowing(hook).await.admitting(Box::new(|envelope| if envelope.target.id.len() == 2 { Ok(()) } else { Err(Rejection::Invalid { detail: "target-malformed".into() }) }));
    let malformed = actor(COUNTER, &"x".repeat(4096));
    match bus.submit(command(&malformed, "counter.increment", Some("k1")), tick(1)).await {
        CommandOutcome::Rejected { receipt, reason: Rejection::Invalid { detail }, .. } => {
            assert_eq!(detail, "target-malformed");
            assert_eq!(receipt.revision, Revision(0));
        }
        other => panic!("expected the admission refusal, got {other:?}"),
    }
    assert_eq!(asked.load(Ordering::Relaxed), 0, "admission comes before policy");
    assert_eq!(bus.directory().placed(), 0);
    assert_eq!(last_seq(&bus, &malformed).await, 0);
    assert!(matches!(bus.submit(command(&key(COUNTER), "counter.increment", Some("k1")), tick(2)).await, CommandOutcome::Accepted { .. }), "a well-formed target passes");
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
    assert_eq!(last_seq(&bus, &key(COUNTER)).await, 1);
}

#[test]
fn a_receipt_key_is_unambiguous_in_both_halves() {
    let alice = Principal::User { id: "alice".into() };
    assert_eq!(receipt_key(&alice, &IdempotencyKey("k1".into())), IdempotencyKey("10:user:alice:k1".into()));
    assert_eq!(receipt_key(&Principal::Anonymous, &IdempotencyKey("k1".into())), IdempotencyKey("9:anonymous:k1".into()));
    let split = receipt_key(&Principal::User { id: "a".into() }, &IdempotencyKey("b:c".into()));
    let joined = receipt_key(&Principal::User { id: "a:b".into() }, &IdempotencyKey("c".into()));
    assert_ne!(split, joined, "a separator inside an id cannot move the boundary");
}

#[semio_framework_async_macros::async_test]
async fn a_receipt_is_answered_to_the_principal_that_earned_it_and_to_nobody_else() {
    let mut bus = bus().await;
    let service = Principal::ServiceAccount { id: "workflow".into() };
    let secret = actor(COUNTER, "learner-secret-id");
    let earned = bus.submit(sent_by(service.clone(), command(&secret, "counter.increment", Some("workflow:1"))), tick(1)).await;
    let CommandOutcome::Accepted { receipt: original, .. } = earned else { panic!("the workflow's command is accepted") };

    let probe = sent_by(Principal::Anonymous, command(&key(COUNTER), "counter.forbid", Some("workflow:1")));
    let answer = bus.submit(probe, tick(2)).await;
    assert!(matches!(answer, CommandOutcome::Rejected { reason: Rejection::Invalid { ref detail }, .. } if detail == "counter refuses"), "another principal's key is no receipt of this caller: {answer:?}");
    assert!(!format!("{answer:?}").contains("learner-secret-id"), "{answer:?}");

    let replayed = bus.submit(sent_by(service, command(&secret, "counter.increment", Some("workflow:1"))), tick(3)).await;
    assert!(matches!(replayed, CommandOutcome::Accepted { ref receipt, ref events, .. } if *receipt == original && events.is_empty()), "the workflow still deduplicates: {replayed:?}");
    assert_eq!(last_seq(&bus, &secret).await, 1);
}

#[semio_framework_async_macros::async_test]
async fn a_key_another_principal_occupied_first_does_not_swallow_the_real_command() {
    let mut bus = bus().await;
    let poisoned = bus.submit(sent_by(Principal::Anonymous, command(&key(COUNTER), "counter.increment", Some("workflow:7"))), tick(1)).await;
    assert!(matches!(poisoned, CommandOutcome::Accepted { .. }));
    let service = Principal::ServiceAccount { id: "workflow".into() };
    let learner = actor(COUNTER, "learner-7");
    match bus.submit(sent_by(service, command(&learner, "counter.increment", Some("workflow:7"))), tick(2)).await {
        CommandOutcome::Accepted { events, receipt, .. } => {
            assert_eq!(events.len(), 1, "the real command runs its turn");
            assert_eq!(receipt.actor, learner);
        }
        other => panic!("expected the workflow's turn, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn a_key_reused_for_another_actor_is_refused_without_the_stored_receipt() {
    let mut bus = bus().await;
    let first = actor(COUNTER, "first");
    bus.submit(command(&first, "counter.increment", Some("k1")), tick(1)).await;
    let other = actor(COUNTER, "other");
    match bus.submit(command(&other, "counter.increment", Some("k1")), tick(2)).await {
        CommandOutcome::Rejected { receipt, reason, notices } => {
            assert_eq!(reason, Rejection::Invalid { detail: IDEMPOTENCY_CONFLICT.into() });
            assert_eq!(receipt, CommandReceipt { command_id: CommandId("cmd-counter.increment".into()), actor: other.clone(), revision: Revision(0), accepted_at: tick(2) }, "the answer is built from the caller's own envelope");
            assert!(notices.is_empty());
        }
        other => panic!("expected the conflict, got {other:?}"),
    }
    assert!(!bus.directory().is_active(&other));
    assert_eq!(last_seq(&bus, &other).await, 0);
}
//#endregion 🔖️Deduplicate

//#region 🔖️Authorize
#[semio_framework_async_macros::async_test]
async fn a_policy_denial_rejects_before_the_actor_is_placed_and_says_nothing_of_the_policy() {
    let mut bus = allowing(Box::new(|_| PolicyDecision::Deny { reason: "no grant lets user:alice do 'counter.increment'".into() })).await;
    let outcome = bus.submit(command(&key(COUNTER), "counter.increment", Some("k1")), tick(1)).await;
    match outcome {
        CommandOutcome::Rejected { reason: Rejection::Unauthorized { detail }, .. } => {
            assert_eq!(detail, FORBIDDEN);
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
    assert_eq!(last_seq(&bus, &key(COUNTER)).await, 1);
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
    assert_eq!(last_seq(&bus, &key(COUNTER)).await, 0);
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
    assert_eq!(last_seq(&bus, &key(COUNTER)).await, 0);
}
#[semio_framework_async_macros::async_test]
async fn a_second_variant_of_the_instance_closed_decider_set_serves_its_own_actor_kind() {
    let mut bus = bus().await;
    match bus.submit(command(&key(MIRROR), "mirror.reflect", Some("k1")), tick(1)).await {
        CommandOutcome::Accepted { events, .. } => {
            assert_eq!(events.len(), 1);
            assert_eq!(events[0].kind, "mirror.reflected");
        }
        other => panic!("expected acceptance, got {other:?}"),
    }
    assert_eq!(bus.directory().activation(&key(MIRROR)).unwrap().state.bytes, vec![3]);
    assert!(matches!(bus.submit(command(&key(COUNTER), "counter.increment", Some("k2")), tick(2)).await, CommandOutcome::Accepted { .. }));
    assert_eq!(read_counter(&bus.directory().activation(&key(COUNTER)).unwrap().state.bytes), 3);
}
//#endregion 🔖️Decide

//#region 🔖️Saga
#[semio_framework_async_macros::async_test]
async fn the_outbox_drains_exactly_once_across_two_calls() {
    let mut bus = bus().await;
    bus.submit(command(&key(COUNTER), "counter.increment", Some("k1")), tick(1)).await;
    bus.submit(command(&key(COUNTER), "counter.increment", Some("k2")), tick(2)).await;

    let mut runner = SagaRunner::new();
    runner.register(TestSagas::Echo(EchoSaga));
    runner.register(TestSagas::Silent(SilentSaga));

    let first = runner.drain_outbox(&mut *bus.store().write().await, 64).await;
    assert_eq!(first.len(), 2);
    assert!(first.iter().all(|follow_up| follow_up.kind == "counter.increment"));

    let second = runner.drain_outbox(&mut *bus.store().write().await, 64).await;
    assert!(second.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn effects_reach_the_outbox_without_an_event() {
    let mut bus = bus().await;
    bus.submit(command(&key(COUNTER), "counter.audit", Some("k1")), tick(1)).await;
    let pending = bus.store().read().await.pending_outbox(64).await.unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].kind, "counter.audited");
    assert!(pending[0].event.is_none());
}
//#endregion 🔖️Saga

//#region 🔖️Directory
#[semio_framework_async_macros::async_test]
async fn a_fresh_activation_rehydrates_from_the_durable_stream_before_deciding() {
    let mut store = MemoryAuthorityStore::default();
    store.append_events(&key(COUNTER), &history(&key(COUNTER), 2), &[]).await.unwrap();
    let mut restarted = over(store, AuthorityDirectory::new(), Box::new(|_| PolicyDecision::Allow)).await;
    match restarted.submit(command(&key(COUNTER), "counter.increment", Some("k3")), tick(3)).await {
        CommandOutcome::Accepted { receipt, events, .. } => {
            assert_eq!(events[0].seq, 3);
            assert_eq!(receipt.revision, Revision(3));
        }
        other => panic!("expected acceptance after rehydration, got {other:?}"),
    }
    assert_eq!(read_counter(&restarted.directory().activation(&key(COUNTER)).unwrap().state.bytes), 9);
}

#[semio_framework_async_macros::async_test]
async fn passivation_fences_the_next_activation_with_a_higher_epoch() {
    let mut directory = AuthorityDirectory::new();
    directory.activate(&key(COUNTER), "authority").unwrap();
    let first = directory.activation_epoch(&key(COUNTER)).unwrap();
    directory.passivate(&key(COUNTER));
    assert!(!directory.is_active(&key(COUNTER)));
    directory.activate(&key(COUNTER), "authority").unwrap();
    assert!(directory.activation_epoch(&key(COUNTER)).unwrap() > first);
}

#[semio_framework_async_macros::async_test]
async fn an_actor_without_history_is_not_kept_placed() {
    let mut bus = bus().await;
    for id in 0..100 {
        let never = actor(COUNTER, &format!("never-{id}"));
        assert!(matches!(bus.submit(command(&never, "counter.forbid", Some(&format!("k{id}"))), tick(id)).await, CommandOutcome::Rejected { .. }));
    }
    assert_eq!(bus.directory().placed(), 0, "a refused first command leaves no activation behind");
    let mut conflicting = command(&key(COUNTER), "counter.increment", Some("stale"));
    conflicting.expected_revision = Some(Revision(4));
    assert!(matches!(bus.submit(conflicting, tick(200)).await, CommandOutcome::Rejected { reason: Rejection::RevisionConflict { .. }, .. }));
    assert!(matches!(bus.submit(command(&key(COUNTER), "counter.rebuild", Some("deferred")), tick(201)).await, CommandOutcome::Pending { .. }));
    assert_eq!(bus.directory().placed(), 0);
    bus.submit(command(&key(COUNTER), "counter.increment", Some("first")), tick(202)).await;
    assert!(matches!(bus.submit(command(&key(COUNTER), "counter.forbid", Some("later")), tick(203)).await, CommandOutcome::Rejected { .. }));
    assert_eq!(bus.directory().placed(), 1, "an actor with history stays placed through a refusal");
}

#[semio_framework_async_macros::async_test]
async fn a_bounded_directory_passivates_the_least_recently_used_actor_and_rehydrates_it_on_return() {
    let mut bus = over(MemoryAuthorityStore::default(), AuthorityDirectory::bounded(2), Box::new(|_| PolicyDecision::Allow)).await;
    let (first, second, third) = (actor(COUNTER, "a"), actor(COUNTER, "b"), actor(COUNTER, "c"));
    bus.submit(command(&first, "counter.increment", Some("a1")), tick(1)).await;
    bus.submit(command(&second, "counter.increment", Some("b1")), tick(2)).await;
    bus.submit(command(&first, "counter.increment", Some("a2")), tick(3)).await;
    bus.submit(command(&third, "counter.increment", Some("c1")), tick(4)).await;
    assert_eq!(bus.directory().placed(), 2);
    assert!(bus.directory().is_active(&first) && bus.directory().is_active(&third) && !bus.directory().is_active(&second), "the actor used longest ago made room");
    match bus.submit(command(&second, "counter.increment", Some("b2")), tick(5)).await {
        CommandOutcome::Accepted { receipt, .. } => assert_eq!(receipt.revision, Revision(2), "the returning actor appends after its history"),
        other => panic!("expected acceptance, got {other:?}"),
    }
    assert_eq!(read_counter(&bus.directory().activation(&second).unwrap().state.bytes), 6);
    assert_eq!(bus.directory().placed(), 2);
}
//#endregion 🔖️Directory

//#region 🔖️Snapshot
#[semio_framework_async_macros::async_test]
async fn a_snapshot_is_written_every_interval_and_never_in_between() {
    let mut bus = bus().await.snapshotting(4);
    for turn in 1..=3 {
        bus.submit(command(&key(COUNTER), "counter.increment", Some(&format!("k{turn}"))), tick(turn)).await;
    }
    assert_eq!(bus.store().read().await.snapshot(&key(COUNTER)).await.unwrap(), None);
    bus.submit(command(&key(COUNTER), "counter.increment", Some("k4")), tick(4)).await;
    assert_eq!(bus.store().read().await.snapshot(&key(COUNTER)).await.unwrap(), Some((Revision(4), frame(0, &12u64.to_le_bytes()))));
    for turn in 5..=7 {
        bus.submit(command(&key(COUNTER), "counter.increment", Some(&format!("k{turn}"))), tick(turn)).await;
    }
    assert_eq!(bus.store().read().await.snapshot(&key(COUNTER)).await.unwrap().map(|(revision, _)| revision), Some(Revision(4)));
    bus.submit(command(&key(COUNTER), "counter.increment", Some("k8")), tick(8)).await;
    assert_eq!(bus.store().read().await.snapshot(&key(COUNTER)).await.unwrap().map(|(revision, _)| revision), Some(Revision(8)));

    let mut never = self::bus().await.snapshotting(0);
    for turn in 1..=10 {
        never.submit(command(&key(COUNTER), "counter.increment", Some(&format!("k{turn}"))), tick(turn)).await;
    }
    assert_eq!(never.store().read().await.snapshot(&key(COUNTER)).await.unwrap(), None);
}

#[semio_framework_async_macros::async_test]
async fn a_fresh_activation_starts_from_its_snapshot_and_folds_only_what_came_after() {
    let mut store = MemoryAuthorityStore::default();
    store.append_events(&key(COUNTER), &history(&key(COUNTER), 3), &[]).await.unwrap();
    store.put_snapshot(&key(COUNTER), Revision(2), frame(0, &100u64.to_le_bytes())).await.unwrap();
    let mut restarted = over(store, AuthorityDirectory::new(), Box::new(|_| PolicyDecision::Allow)).await;
    match restarted.submit(command(&key(COUNTER), "counter.increment", Some("k4")), tick(4)).await {
        CommandOutcome::Accepted { receipt, .. } => assert_eq!(receipt.revision, Revision(4)),
        other => panic!("expected acceptance, got {other:?}"),
    }
    let activation = restarted.directory().activation(&key(COUNTER)).unwrap();
    assert_eq!(read_counter(&activation.state.bytes), 106, "the snapshot's state plus the two events after it");
    assert_eq!(activation.snapshot_version, 2);
}

#[semio_framework_async_macros::async_test]
async fn a_snapshot_of_another_state_format_is_ignored_and_replaced() {
    let mut store = MemoryAuthorityStore::default();
    store.append_events(&key(COUNTER), &history(&key(COUNTER), 3), &[]).await.unwrap();
    store.put_snapshot(&key(COUNTER), Revision(2), frame(7, &100u64.to_le_bytes())).await.unwrap();
    let mut restarted = over(store, AuthorityDirectory::new(), Box::new(|_| PolicyDecision::Allow)).await.snapshotting(4);
    restarted.submit(command(&key(COUNTER), "counter.increment", Some("k4")), tick(4)).await;
    assert_eq!(read_counter(&restarted.directory().activation(&key(COUNTER)).unwrap().state.bytes), 12, "the whole stream is replayed");
    assert_eq!(restarted.store().read().await.snapshot(&key(COUNTER)).await.unwrap(), Some((Revision(4), frame(0, &12u64.to_le_bytes()))), "the next snapshot is of the current format");
    assert_eq!(unframe(0, &[1, 2]), None, "a snapshot too short to carry its format is no snapshot");
}
//#endregion 🔖️Snapshot

//#region 🔖️Batch
/// 🧾️ The reference store behind a recorder: how many turns each commit carried, and a switch that
/// makes every commit fail whole, the way a full disk fails a transaction.
struct Recording {
    inner: MemoryAuthorityStore,
    commits: Arc<std::sync::Mutex<Vec<usize>>>,
    broken: Arc<std::sync::atomic::AtomicBool>,
}

impl AuthorityStore for Recording {
    async fn receipt(&self, key: &IdempotencyKey) -> Result<Option<CommandReceipt>, StorageError> {
        self.inner.receipt(key).await
    }

    async fn record_receipt(&mut self, key: &IdempotencyKey, receipt: &CommandReceipt) -> Result<(), StorageError> {
        self.inner.record_receipt(key, receipt).await
    }

    async fn append_events(&mut self, actor: &ActorKey, events: &[EventRecord], outbox: &[OutboxEntry]) -> Result<u64, StorageError> {
        self.inner.append_events(actor, events, outbox).await
    }

    async fn commit(&mut self, turns: &[TurnCommit<'_>]) -> Vec<Result<(), StorageError>> {
        self.commits.lock().unwrap().push(turns.len());
        if self.broken.load(Ordering::Acquire) {
            return turns.iter().map(|_| Err(StorageError::Backend("disk full at /srv/data".into()))).collect();
        }
        let mut fates = Vec::new();
        for turn in turns {
            self.inner.append_events(turn.actor, turn.events, turn.outbox).await.unwrap();
            if let Some((key, receipt)) = turn.receipt {
                self.inner.record_receipt(key, receipt).await.unwrap();
            }
            if let Some((revision, bytes)) = turn.snapshot {
                self.inner.put_snapshot(turn.actor, revision, bytes.to_vec()).await.unwrap();
            }
            fates.push(Ok(()));
        }
        fates
    }

    async fn events_since(&self, actor: &ActorKey, since: u64) -> Result<Vec<EventRecord>, StorageError> {
        self.inner.events_since(actor, since).await
    }

    async fn last_seq(&self, actor: &ActorKey) -> Result<u64, StorageError> {
        self.inner.last_seq(actor).await
    }

    async fn put_snapshot(&mut self, actor: &ActorKey, revision: Revision, bytes: Vec<u8>) -> Result<(), StorageError> {
        self.inner.put_snapshot(actor, revision, bytes).await
    }

    async fn snapshot(&self, actor: &ActorKey) -> Result<Option<(Revision, Vec<u8>)>, StorageError> {
        self.inner.snapshot(actor).await
    }

    async fn enqueue_outbox(&mut self, entries: Vec<OutboxEntry>) -> Result<(), StorageError> {
        self.inner.enqueue_outbox(entries).await
    }

    async fn pending_outbox(&self, limit: usize) -> Result<Vec<OutboxEntry>, StorageError> {
        self.inner.pending_outbox(limit).await
    }

    async fn mark_outbox_delivered(&mut self, ids: &[u64]) -> Result<(), StorageError> {
        self.inner.mark_outbox_delivered(ids).await
    }

    async fn acquire_lease(&mut self, actor: &ActorKey, holder: &str) -> Result<Lease, StorageError> {
        self.inner.acquire_lease(actor, holder).await
    }

    async fn validate_lease(&self, actor: &ActorKey, lease: &Lease) -> bool {
        self.inner.validate_lease(actor, lease).await
    }
}

type Recorded = (CommandBus<Recording, TestDeciders>, Arc<std::sync::Mutex<Vec<usize>>>, Arc<std::sync::atomic::AtomicBool>);

async fn recording(directory: AuthorityDirectory) -> Recorded {
    let (commits, broken) = (Arc::new(std::sync::Mutex::new(Vec::new())), Arc::new(std::sync::atomic::AtomicBool::new(false)));
    let mut bus = CommandBus::new(directory, Recording { inner: MemoryAuthorityStore::default(), commits: Arc::clone(&commits), broken: Arc::clone(&broken) }, Box::new(|_| PolicyDecision::Allow));
    bus.register(TestDeciders::Counter(CounterDecider)).await;
    (bus, commits, broken)
}

fn revision_of(outcome: &CommandOutcome) -> Option<(u64, usize)> {
    match outcome {
        CommandOutcome::Accepted { receipt, events, .. } => Some((receipt.revision.0, events.len())),
        _ => None,
    }
}

#[semio_framework_async_macros::async_test]
async fn a_batch_decides_every_command_in_order_and_commits_once() {
    let (mut bus, commits, _) = recording(AuthorityDirectory::new()).await;
    let (first, second) = (actor(COUNTER, "a"), actor(COUNTER, "b"));
    let batch = vec![
        (command(&first, "counter.increment", Some("k1")), tick(1)),
        (command(&first, "counter.increment", Some("k2")), tick(2)),
        (command(&second, "counter.increment", Some("k3")), tick(3)),
        (command(&first, "counter.increment", Some("k1")), tick(4)),
        (command(&first, "counter.forbid", Some("k5")), tick(5)),
        (command(&second, "counter.increment", Some("k1")), tick(6)),
        (command(&first, "counter.increment", Some("k7")), tick(7)),
    ];
    let outcomes = bus.submit_batch(batch).await;
    assert_eq!(outcomes.iter().map(revision_of).collect::<Vec<_>>(), [Some((1, 1)), Some((2, 1)), Some((1, 1)), Some((1, 0)), None, None, Some((3, 1))]);
    assert_eq!(outcomes[3], CommandOutcome::Accepted { receipt: CommandReceipt { command_id: CommandId("cmd-counter.increment".into()), actor: first.clone(), revision: Revision(1), accepted_at: tick(1) }, events: Vec::new(), frontier: None }, "a key replayed inside the batch is answered with the staged receipt");
    assert!(matches!(&outcomes[4], CommandOutcome::Rejected { reason: Rejection::Invalid { detail }, receipt, .. } if detail == "counter refuses" && receipt.revision == Revision(2)), "a refusal sees what the batch decided before it");
    assert!(matches!(&outcomes[5], CommandOutcome::Rejected { reason: Rejection::Invalid { detail }, .. } if detail == IDEMPOTENCY_CONFLICT), "a staged key of another actor is a conflict");
    assert_eq!(*commits.lock().unwrap(), [4], "four accepted turns, one commit");
    assert_eq!(read_counter(&bus.directory().activation(&first).unwrap().state.bytes), 9);
    assert_eq!(bus.store().read().await.last_seq(&first).await.unwrap(), 3);
    assert_eq!(bus.store().read().await.receipt(&receipt_key(&Principal::User { id: "alice".into() }, &IdempotencyKey("k7".into()))).await.unwrap().map(|receipt| receipt.revision), Some(Revision(3)));
}

#[semio_framework_async_macros::async_test]
async fn a_turn_the_store_refuses_is_unavailable_and_its_actor_forgets_what_it_decided() {
    let (mut bus, commits, broken) = recording(AuthorityDirectory::new()).await;
    bus.submit(command(&key(COUNTER), "counter.increment", Some("k0")), tick(1)).await;
    broken.store(true, Ordering::Release);
    let outcomes = bus.submit_batch(vec![(command(&key(COUNTER), "counter.increment", Some("k1")), tick(2)), (command(&key(COUNTER), "counter.increment", Some("k2")), tick(3)), (command(&key(COUNTER), "counter.increment", Some("k1")), tick(4))]).await;
    for outcome in &outcomes {
        match outcome {
            CommandOutcome::Rejected { reason: Rejection::ActorUnavailable { detail }, notices, .. } => {
                assert_eq!(detail, UNAVAILABLE);
                assert_eq!(notices, &[Notice { code: "authority.retryable".into(), message: UNAVAILABLE.into() }], "the cause stays with the operator");
            }
            other => panic!("expected the retryable refusal, got {other:?}"),
        }
    }
    assert_eq!(outcomes.len(), 3);
    assert!(!bus.directory().is_active(&key(COUNTER)), "what was decided ahead of the store is discarded");
    assert_eq!(*commits.lock().unwrap(), [1, 2]);

    broken.store(false, Ordering::Release);
    match bus.submit(command(&key(COUNTER), "counter.increment", Some("k1")), tick(5)).await {
        CommandOutcome::Accepted { receipt, events, .. } => assert_eq!((receipt.revision, events.len()), (Revision(2), 1), "the retry decides against what is durable"),
        other => panic!("expected the retry to be accepted, got {other:?}"),
    }
    assert_eq!(read_counter(&bus.directory().activation(&key(COUNTER)).unwrap().state.bytes), 6);
}

#[semio_framework_async_macros::async_test]
async fn a_batch_larger_than_the_directory_commits_before_it_makes_room() {
    let (mut bus, commits, _) = recording(AuthorityDirectory::bounded(2)).await;
    let actors: Vec<ActorKey> = ["a", "b", "c"].iter().map(|id| actor(COUNTER, id)).collect();
    let batch: Vec<_> = (0..9u64).map(|turn| (command(&actors[(turn % 3) as usize], "counter.increment", Some(&format!("k{turn}"))), tick(turn + 1))).collect();
    let outcomes = bus.submit_batch(batch).await;
    assert_eq!(outcomes.iter().map(revision_of).collect::<Vec<_>>(), [Some((1, 1)), Some((1, 1)), Some((1, 1)), Some((2, 1)), Some((2, 1)), Some((2, 1)), Some((3, 1)), Some((3, 1)), Some((3, 1))], "an actor that made room is rehydrated from what the batch already committed");
    assert_eq!(commits.lock().unwrap().iter().sum::<usize>(), 9);
    assert!(commits.lock().unwrap().iter().all(|turns| *turns <= 2), "no commit carries more turns than the directory keeps actors: {:?}", commits.lock().unwrap());
    for actor in &actors {
        assert_eq!(bus.store().read().await.last_seq(actor).await.unwrap(), 3);
    }
}
//#endregion 🔖️Batch
