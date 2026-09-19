//! 🔬️ The storage conformance suite — the four roles' contracts written once, as functions generic
//! over the backend, so every [`ServerInstance`](crate::gateway::ServerInstance) is held against the
//! same assertions instead of against a copy of them.
//!
//! **Why this is not a test module.** The assertions used to live in
//! `🔨️modules/🗄️storage/🧪️tests/🔬️unit/🦀️.rs`, bound to the in-memory reference backends by
//! `MemoryAuthorityStore::new()` in every body. A downstream instance with a durable backend could
//! therefore only *re-type* them, and a re-typed contract is two contracts that drift. Each function
//! here takes `&mut impl <Role>Store` and is compiled into the framework only when somebody asks for
//! it — the crate's own tests (`cfg(test)`) and any instance crate enabling the `conformance`
//! feature on its dev-dependency. A default build of the product contains none of it.
//!
//! **What a backend must bring.** Every function is handed a store that is *empty*; a durable
//! backend opens a fresh directory for each. Nothing here reaches a clock, a task or the filesystem,
//! so a backend that suspends and one that does not are judged identically.

use crate::authority::{Saga, SagaRunner};
use crate::contract::{ActorKey, CommandEnvelope, CommandId, CommandReceipt, DeviceId, EventRecord, HybridLogicalClock, IdempotencyKey, Principal, Revision, Scope, SessionId, TenantId, TraceContext};
use crate::storage::{content_hash, AuthorityStore, BlobStore, Lease, OutboxEntry, ProjectionStore, SessionRecord, SessionStore, StorageError};

//#region 🔖️Fixtures
/// 🎭️ One actor of the fixed test tenant and the `artifact` kind.
pub fn actor_key(id: &str) -> ActorKey {
    ActorKey { tenant: TenantId("t1".into()), kind: "artifact".into(), id: id.into() }
}

/// 📚️ One event of `stream` at `seq`, whose payload is the sequence itself so a replayed stream is
/// distinguishable byte-for-byte from a re-emitted one.
pub fn event_record(stream: &ActorKey, seq: u64) -> EventRecord {
    EventRecord { stream: stream.clone(), seq, hlc: HybridLogicalClock { millis: seq, counter: 0 }, kind: "artifact.mutated".into(), payload: vec![seq as u8] }
}

/// 🧾️ One receipt binding a command id to `revision` of `actor`.
pub fn receipt_for(actor: &ActorKey, revision: u64) -> CommandReceipt {
    CommandReceipt { command_id: CommandId(format!("cmd-{revision}")), actor: actor.clone(), revision: Revision(revision), accepted_at: HybridLogicalClock::default() }
}

/// 🪪️ One session record of `principal`, issued at a fixed instant.
pub fn session_record(id: &str, principal: Principal) -> SessionRecord {
    SessionRecord { id: SessionId(id.into()), principal, device: Some(DeviceId("d1".into())), issued_at_millis: 1_000 }
}
//#endregion 🔖️Fixtures

//#region 🔖️Authority
/// 🔎️ A receipt answers the same command twice and refuses to be re-bound to a different one.
pub async fn receipt_round_trips_and_is_idempotent(store: &mut impl AuthorityStore) {
    let actor = actor_key("doc-1");
    let key = IdempotencyKey("k1".into());
    assert_eq!(store.receipt(&key).await.unwrap(), None);
    store.record_receipt(&key, &receipt_for(&actor, 1)).await.unwrap();
    store.record_receipt(&key, &receipt_for(&actor, 1)).await.unwrap();
    assert_eq!(store.receipt(&key).await.unwrap(), Some(receipt_for(&actor, 1)));
    assert!(matches!(store.record_receipt(&key, &receipt_for(&actor, 2)).await, Err(StorageError::Conflict(_))));
}

/// 🪜️ A non-contiguous append is refused and leaves the stream exactly where it was.
pub async fn append_events_rejects_a_sequence_gap_and_writes_nothing(store: &mut impl AuthorityStore) {
    let actor = actor_key("doc-1");
    assert_eq!(store.append_events(&actor, &[event_record(&actor, 1), event_record(&actor, 2)], &[]).await.unwrap(), 2);
    assert_eq!(store.append_events(&actor, &[event_record(&actor, 4)], &[]).await, Err(StorageError::SequenceGap { expected: 3, got: 4 }));
    assert_eq!(store.append_events(&actor, &[event_record(&actor, 3), event_record(&actor, 5)], &[]).await, Err(StorageError::SequenceGap { expected: 4, got: 5 }));
    assert_eq!(store.last_seq(&actor).await.unwrap(), 2);
}

/// 📜️ A stream reads back in order, scoped to its own actor, and an unwritten actor is at zero.
pub async fn events_since_returns_only_later_events_of_that_actor(store: &mut impl AuthorityStore) {
    let first = actor_key("doc-1");
    let second = actor_key("doc-2");
    store.append_events(&first, &[event_record(&first, 1), event_record(&first, 2), event_record(&first, 3)], &[]).await.unwrap();
    store.append_events(&second, &[event_record(&second, 1)], &[]).await.unwrap();
    let tail: Vec<u64> = store.events_since(&first, 1).await.unwrap().iter().map(|record| record.seq).collect();
    assert_eq!(tail, vec![2, 3]);
    assert!(store.events_since(&first, 3).await.unwrap().is_empty());
    assert_eq!(store.events_since(&second, 0).await.unwrap().len(), 1);
    assert_eq!(store.last_seq(&actor_key("doc-3")).await.unwrap(), 0);
}

/// 📸️ A snapshot may only move forward, and the stored one is the newest accepted.
pub async fn snapshots_only_move_forward(store: &mut impl AuthorityStore) {
    let actor = actor_key("doc-1");
    assert_eq!(store.snapshot(&actor).await.unwrap(), None);
    store.put_snapshot(&actor, Revision(4), vec![1, 2]).await.unwrap();
    assert_eq!(store.snapshot(&actor).await.unwrap(), Some((Revision(4), vec![1, 2])));
    assert!(matches!(store.put_snapshot(&actor, Revision(3), vec![9]).await, Err(StorageError::Conflict(_))));
    store.put_snapshot(&actor, Revision(7), vec![3]).await.unwrap();
    assert_eq!(store.snapshot(&actor).await.unwrap(), Some((Revision(7), vec![3])));
}

/// 📮️ The outbox hands each entry out until it is acknowledged, then never again.
pub async fn outbox_delivers_each_entry_exactly_once(store: &mut impl AuthorityStore) {
    let actor = actor_key("doc-1");
    store.enqueue_outbox(vec![OutboxEntry::pending(actor.clone(), event_record(&actor, 1)), OutboxEntry::pending(actor.clone(), event_record(&actor, 2))]).await.unwrap();
    let pending = store.pending_outbox(10).await.unwrap();
    assert_eq!(pending.iter().map(|entry| entry.id).collect::<Vec<_>>(), vec![1, 2]);
    assert!(pending.iter().all(|entry| !entry.delivered));
    assert_eq!(store.pending_outbox(1).await.unwrap().len(), 1);
    store.mark_outbox_delivered(&[1]).await.unwrap();
    assert_eq!(store.pending_outbox(10).await.unwrap().iter().map(|entry| entry.id).collect::<Vec<_>>(), vec![2]);
    store.mark_outbox_delivered(&[1, 2]).await.unwrap();
    assert!(store.pending_outbox(10).await.unwrap().is_empty());
    assert_eq!(store.mark_outbox_delivered(&[99]).await, Err(StorageError::NotFound));
}

/// 🎟️ Taking a lease from another holder bumps the epoch and fences the previous one out for good.
pub async fn lease_epoch_bump_fences_out_the_previous_holder(store: &mut impl AuthorityStore) {
    let other = actor_key("doc-2");
    let actor = actor_key("doc-1");
    let first = store.acquire_lease(&actor, "node-a").await.unwrap();
    assert_eq!(first, Lease { epoch: 1, holder: "node-a".into() });
    assert_eq!(store.acquire_lease(&actor, "node-a").await.unwrap(), first);
    assert!(store.validate_lease(&actor, &first).await);
    let second = store.acquire_lease(&actor, "node-b").await.unwrap();
    assert_eq!(second, Lease { epoch: 2, holder: "node-b".into() });
    assert!(!store.validate_lease(&actor, &first).await);
    assert!(store.validate_lease(&actor, &second).await);
    assert!(!store.validate_lease(&other, &second).await);
}
//#endregion 🔖️Authority

//#region 🔖️Projection
/// 📋️ A listing is scoped to its prefix, ordered by key, and blind to other projections.
pub async fn projection_list_is_prefix_scoped_and_key_ordered(store: &mut impl ProjectionStore) {
    store.put("members", "space/b", vec![2]).await.unwrap();
    store.put("members", "space/a", vec![1]).await.unwrap();
    store.put("members", "tenant/a", vec![3]).await.unwrap();
    store.put("documents", "space/a", vec![9]).await.unwrap();
    let keys: Vec<String> = store.list("members", "space/").await.into_iter().map(|(key, _)| key).collect();
    assert_eq!(keys, vec!["space/a".to_string(), "space/b".to_string()]);
    assert_eq!(store.list("members", "").await.len(), 3);
    assert!(store.list("nothing", "").await.is_empty());
    assert_eq!(store.get("members", "space/a").await, Some(vec![1]));
    assert_eq!(store.get("members", "space/z").await, None);
}

/// 🧹️ Clearing a projection drops its entries and its checkpoint, so the next fold rebuilds it.
pub async fn clearing_a_projection_resets_it_for_rebuild(store: &mut impl ProjectionStore) {
    store.put("members", "space/a", vec![1]).await.unwrap();
    store.set_checkpoint("members", 42).await.unwrap();
    assert_eq!(store.checkpoint("members").await, 42);
    store.clear("members").await.unwrap();
    assert_eq!(store.checkpoint("members").await, 0);
    assert_eq!(store.get("members", "space/a").await, None);
    store.put("members", "space/a", vec![7]).await.unwrap();
    store.set_checkpoint("members", 43).await.unwrap();
    assert_eq!(store.get("members", "space/a").await, Some(vec![7]));
    assert_eq!(store.checkpoint("members").await, 43);
}

/// 🌱️ The state [`a_projection_write_reports_a_failing_sink`] expects its store to arrive in.
/// Written through the port, so a caller seeds a backend exactly as the law will read it back.
pub async fn seed_projection_fault_fixture(store: &mut impl ProjectionStore) {
    store.put("members", "space/a", vec![1]).await.unwrap();
    store.set_checkpoint("members", 5).await.unwrap();
}

/// 💥️ A read model whose sink has failed refuses every write and keeps answering with the last
/// state it durably holds.
///
/// **Precondition**: the store arrives seeded by [`seed_projection_fault_fixture`] and its sink has
/// since failed. How a backend is brought into that state is the backend's own business — a
/// read-only handle on its journal for a journalling one, a refusing sink for an in-memory
/// reference — which is why the law is written against the port instead of against a fault
/// injection hook the port would then have to carry into production.
///
/// What it pins is the reason `put`/`set_checkpoint`/`clear` return `Result` at all. A write that
/// only *counted* its journal failure still landed in memory, so the read model answered queries
/// with state the next open would not reproduce and no caller could learn of it. Here the refusal
/// is reported and the visible state is still the durable one: `space/a` is the seeded value, not
/// the refused one; the checkpoint has not moved; and a refused `clear` has not dropped anything.
pub async fn a_projection_write_reports_a_failing_sink(store: &mut impl ProjectionStore) {
    assert!(matches!(store.put("members", "space/a", vec![2]).await, Err(StorageError::Backend(_))));
    assert_eq!(store.get("members", "space/a").await, Some(vec![1]));
    assert!(matches!(store.set_checkpoint("members", 42).await, Err(StorageError::Backend(_))));
    assert_eq!(store.checkpoint("members").await, 5);
    assert!(matches!(store.clear("members").await, Err(StorageError::Backend(_))));
    assert_eq!(store.list("members", "space/").await, vec![("space/a".to_string(), vec![1])]);
    assert_eq!(store.checkpoint("members").await, 5);
}
//#endregion 🔖️Projection

//#region 🔖️Blob
/// 🧱️ Blobs deduplicate by hash and refuse to re-bind a hash to different bytes.
pub async fn blob_put_get_and_has_are_content_addressed(store: &mut impl BlobStore) {
    let hash = content_hash(b"hello");
    assert!(!store.has(&hash).await);
    assert_eq!(store.get(&hash).await, None);
    store.put(hash, b"hello").await.unwrap();
    store.put(hash, b"hello").await.unwrap();
    assert!(store.has(&hash).await);
    assert_eq!(store.get(&hash).await, Some(b"hello".to_vec()));
    assert_ne!(content_hash(b"hello"), content_hash(b"world"));
    assert!(matches!(store.put(hash, b"world").await, Err(StorageError::Conflict(_))));
}
//#endregion 🔖️Blob

//#region 🔖️Session
/// 🚪️ Revoking a principal removes every one of its sessions and leaves the others untouched.
pub async fn revoke_principal_removes_every_session_of_that_principal(store: &mut impl SessionStore) {
    let alice = Principal::User { id: "alice".into() };
    let bob = Principal::User { id: "bob".into() };
    store.create(session_record("s1", alice.clone())).await.unwrap();
    store.create(session_record("s2", alice.clone())).await.unwrap();
    store.create(session_record("s3", bob.clone())).await.unwrap();
    assert_eq!(store.get(&SessionId("s1".into())).await.map(|record| record.principal), Some(alice.clone()));
    store.delete(&SessionId("s2".into())).await.unwrap();
    assert_eq!(store.revoke_principal(&alice).await.unwrap(), 1);
    assert_eq!(store.revoke_principal(&alice).await.unwrap(), 0);
    assert_eq!(store.get(&SessionId("s1".into())).await, None);
    assert_eq!(store.get(&SessionId("s3".into())).await.map(|record| record.principal), Some(bob));
}

/// 🌱️ The state [`a_session_write_reports_a_failing_sink`] expects its store to arrive in: one live
/// session `s1` of `alice`.
pub async fn seed_session_fault_fixture(store: &mut impl SessionStore) {
    store.create(session_record("s1", Principal::User { id: "alice".into() })).await.unwrap();
}

/// 💥️ A session store whose sink has failed refuses to mint a session, and refuses to claim a
/// revocation it did not carry out.
///
/// **Precondition**: the store arrives seeded by [`seed_session_fault_fixture`] with its sink since
/// failed — the same shape as [`a_projection_write_reports_a_failing_sink`], and for the same
/// reason.
///
/// The stakes are higher here than for a read model, which is why this law exists separately: a
/// `revoke_principal` that answered a plain `usize` through a failed removal would tell a caller
/// "signed out everywhere" while the record that opens the door is still on disk. So the law
/// asserts the refusal *and* that `s1` is still readable afterwards — the door really is still
/// open, and the caller was told rather than reassured.
pub async fn a_session_write_reports_a_failing_sink(store: &mut impl SessionStore) {
    let alice = Principal::User { id: "alice".into() };
    assert!(matches!(store.create(session_record("s2", alice.clone())).await, Err(StorageError::Backend(_))));
    assert_eq!(store.get(&SessionId("s2".into())).await, None);
    assert!(matches!(store.delete(&SessionId("s1".into())).await, Err(StorageError::Backend(_))));
    assert!(matches!(store.revoke_principal(&alice).await, Err(StorageError::Backend(_))));
    assert_eq!(store.get(&SessionId("s1".into())).await.map(|record| record.principal), Some(alice));
}
//#endregion 🔖️Session

//#region 🔖️Saga
/// 🧵️ A workflow that answers every committed event with exactly one follow-up command, so the
/// length of a drain's result *is* the number of reactions and no counter has to be threaded
/// through the store under test.
pub struct OneCommandPerEvent;

impl Saga for OneCommandPerEvent {
    async fn on_event(&self, event: &EventRecord) -> Vec<CommandEnvelope> {
        vec![CommandEnvelope {
            command_id: CommandId(format!("saga-{}-{}", event.stream.id, event.seq)),
            kind: "conformance.reaction".into(),
            version: 1,
            target: event.stream.clone(),
            scope: Scope("space-1".into()),
            principal: Principal::Anonymous,
            session: None,
            device: None,
            payload: event.payload.clone(),
            causal_frontier: None,
            client_hlc: event.hlc,
            expected_revision: None,
            idempotency_key: Some(IdempotencyKey(format!("saga-{}-{}", event.stream.id, event.seq))),
            capability_proof: None,
            trace: TraceContext::default(),
        }]
    }
}

/// 🔁️ A committed event reaches a workflow **exactly once across a restart** — the outbox law.
///
/// `reopen` is how this instance's process comes back: a second open of the same directory for a
/// durable backend, a fresh store for an ephemeral one. It is a factory rather than a second
/// argument because the restart has to happen *after* the commit and the drain — handing the law
/// an already-open second store would let it observe a journal that had not been written yet.
/// The law is the same for both shapes and that is the point: a durable backend must not
/// re-deliver a row it already acknowledged, and an ephemeral one must not resurrect a queue it
/// never kept.
///
/// The four facts it pins, in order: an append commits the event and its outbox row in one write;
/// a drain hands the row to every workflow once, with the idempotency key that makes the follow-up
/// command safe to retry; the row leaves the pending queue behind it; and the restarted store hands
/// the workflow nothing, so one committed event produces exactly one reaction in total.
pub async fn a_saga_reacts_to_a_committed_event_exactly_once_across_a_restart<S: AuthorityStore, R: core::future::Future<Output = S>>(before: &mut S, reopen: impl FnOnce() -> R) {
    let actor = actor_key("doc-1");
    let event = event_record(&actor, 1);
    before.append_events(&actor, &[event.clone()], &[OutboxEntry::pending(actor.clone(), event)]).await.unwrap();

    let mut runner: SagaRunner<OneCommandPerEvent> = SagaRunner::new();
    runner.register(OneCommandPerEvent);

    let first = runner.drain_outbox(before, 10).await;
    assert_eq!(first.len(), 1, "one committed event must reach the workflow once");
    assert_eq!(first[0].idempotency_key, Some(IdempotencyKey("saga-doc-1-1".into())));
    assert!(before.pending_outbox(10).await.unwrap().is_empty(), "a drained row must leave the pending queue");

    let mut restarted = reopen().await;
    assert!(runner.drain_outbox(&mut restarted, 10).await.is_empty(), "a restarted store must not re-deliver an acknowledged row");
}
//#endregion 🔖️Saga
