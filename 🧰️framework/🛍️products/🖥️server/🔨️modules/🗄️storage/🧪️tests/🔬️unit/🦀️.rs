
use super::*;
use crate::contract::{CommandId, HybridLogicalClock, TenantId};

fn actor(id: &str) -> ActorKey {
    ActorKey { tenant: TenantId("t1".into()), kind: "artifact".into(), id: id.into() }
}

fn event(stream: &ActorKey, seq: u64) -> EventRecord {
    EventRecord { stream: stream.clone(), seq, hlc: HybridLogicalClock { millis: seq, counter: 0 }, kind: "artifact.mutated".into(), payload: vec![seq as u8] }
}

fn receipt(actor: &ActorKey, revision: u64) -> CommandReceipt {
    CommandReceipt { command_id: CommandId(format!("cmd-{revision}")), actor: actor.clone(), revision: Revision(revision), accepted_at: HybridLogicalClock::default() }
}

fn session(id: &str, principal: Principal) -> SessionRecord {
    SessionRecord { id: SessionId(id.into()), principal, device: Some(DeviceId("d1".into())), issued_at_millis: 1_000 }
}

//#region 🔖️Authority
#[semio_framework_async_macros::async_test]
async fn receipt_round_trips_and_is_idempotent() {
    let mut store = MemoryAuthorityStore::new();
    let actor = actor("doc-1");
    let key = IdempotencyKey("k1".into());
    assert_eq!(store.receipt(&key).await.unwrap(), None);
    store.record_receipt(&key, &receipt(&actor, 1)).await.unwrap();
    store.record_receipt(&key, &receipt(&actor, 1)).await.unwrap();
    assert_eq!(store.receipt(&key).await.unwrap(), Some(receipt(&actor, 1)));
    assert!(matches!(store.record_receipt(&key, &receipt(&actor, 2)).await, Err(StorageError::Conflict(_))));
}

#[semio_framework_async_macros::async_test]
async fn append_events_rejects_a_sequence_gap_and_writes_nothing() {
    let mut store = MemoryAuthorityStore::new();
    let actor = actor("doc-1");
    assert_eq!(store.append_events(&actor, &[event(&actor, 1), event(&actor, 2)], &[]).await.unwrap(), 2);
    assert_eq!(store.append_events(&actor, &[event(&actor, 4)], &[]).await, Err(StorageError::SequenceGap { expected: 3, got: 4 }));
    assert_eq!(store.append_events(&actor, &[event(&actor, 3), event(&actor, 5)], &[]).await, Err(StorageError::SequenceGap { expected: 4, got: 5 }));
    assert_eq!(store.last_seq(&actor).await.unwrap(), 2);
}

#[semio_framework_async_macros::async_test]
async fn events_since_returns_only_later_events_of_that_actor() {
    let mut store = MemoryAuthorityStore::new();
    let first = actor("doc-1");
    let second = actor("doc-2");
    store.append_events(&first, &[event(&first, 1), event(&first, 2), event(&first, 3)], &[]).await.unwrap();
    store.append_events(&second, &[event(&second, 1)], &[]).await.unwrap();
    let tail: Vec<u64> = store.events_since(&first, 1).await.unwrap().iter().map(|record| record.seq).collect();
    assert_eq!(tail, vec![2, 3]);
    assert!(store.events_since(&first, 3).await.unwrap().is_empty());
    assert_eq!(store.events_since(&second, 0).await.unwrap().len(), 1);
    assert_eq!(store.last_seq(&actor("doc-3")).await.unwrap(), 0);
}

#[semio_framework_async_macros::async_test]
async fn snapshots_only_move_forward() {
    let mut store = MemoryAuthorityStore::new();
    let actor = actor("doc-1");
    assert_eq!(store.snapshot(&actor).await.unwrap(), None);
    store.put_snapshot(&actor, Revision(4), vec![1, 2]).await.unwrap();
    assert_eq!(store.snapshot(&actor).await.unwrap(), Some((Revision(4), vec![1, 2])));
    assert!(matches!(store.put_snapshot(&actor, Revision(3), vec![9]).await, Err(StorageError::Conflict(_))));
    store.put_snapshot(&actor, Revision(7), vec![3]).await.unwrap();
    assert_eq!(store.snapshot(&actor).await.unwrap(), Some((Revision(7), vec![3])));
}

#[semio_framework_async_macros::async_test]
async fn outbox_delivers_each_entry_exactly_once() {
    let mut store = MemoryAuthorityStore::new();
    let actor = actor("doc-1");
    store.enqueue_outbox(vec![OutboxEntry::pending(actor.clone(), event(&actor, 1)), OutboxEntry::pending(actor.clone(), event(&actor, 2))]).await.unwrap();
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

#[semio_framework_async_macros::async_test]
async fn lease_epoch_bump_fences_out_the_previous_holder() {
    let mut store = MemoryAuthorityStore::new();
    let other = actor("doc-2");
    let actor = actor("doc-1");
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
#[semio_framework_async_macros::async_test]
async fn projection_list_is_prefix_scoped_and_key_ordered() {
    let mut store = MemoryProjectionStore::new();
    store.put("members", "space/b", vec![2]).await;
    store.put("members", "space/a", vec![1]).await;
    store.put("members", "tenant/a", vec![3]).await;
    store.put("documents", "space/a", vec![9]).await;
    let keys: Vec<String> = store.list("members", "space/").await.into_iter().map(|(key, _)| key).collect();
    assert_eq!(keys, vec!["space/a".to_string(), "space/b".to_string()]);
    assert_eq!(store.list("members", "").await.len(), 3);
    assert!(store.list("nothing", "").await.is_empty());
    assert_eq!(store.get("members", "space/a").await, Some(vec![1]));
    assert_eq!(store.get("members", "space/z").await, None);
}

#[semio_framework_async_macros::async_test]
async fn clearing_a_projection_resets_it_for_rebuild() {
    let mut store = MemoryProjectionStore::new();
    store.put("members", "space/a", vec![1]).await;
    store.set_checkpoint("members", 42).await;
    assert_eq!(store.checkpoint("members").await, 42);
    store.clear("members").await;
    assert_eq!(store.checkpoint("members").await, 0);
    assert_eq!(store.get("members", "space/a").await, None);
    store.put("members", "space/a", vec![7]).await;
    store.set_checkpoint("members", 43).await;
    assert_eq!(store.get("members", "space/a").await, Some(vec![7]));
    assert_eq!(store.checkpoint("members").await, 43);
}
//#endregion 🔖️Projection

//#region 🔖️Blob
#[semio_framework_async_macros::async_test]
async fn blob_put_get_and_has_are_content_addressed() {
    let mut store = MemoryBlobStore::new();
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
#[semio_framework_async_macros::async_test]
async fn revoke_principal_removes_every_session_of_that_principal() {
    let mut store = MemorySessionStore::new();
    let alice = Principal::User { id: "alice".into() };
    let bob = Principal::User { id: "bob".into() };
    store.create(session("s1", alice.clone())).await;
    store.create(session("s2", alice.clone())).await;
    store.create(session("s3", bob.clone())).await;
    assert_eq!(store.get(&SessionId("s1".into())).await.map(|record| record.principal), Some(alice.clone()));
    store.delete(&SessionId("s2".into())).await;
    assert_eq!(store.revoke_principal(&alice).await, 1);
    assert_eq!(store.revoke_principal(&alice).await, 0);
    assert_eq!(store.get(&SessionId("s1".into())).await, None);
    assert_eq!(store.get(&SessionId("s3".into())).await.map(|record| record.principal), Some(bob));
}
//#endregion 🔖️Session
