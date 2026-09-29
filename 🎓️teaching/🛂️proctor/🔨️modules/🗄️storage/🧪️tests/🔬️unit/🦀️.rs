use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_SCRATCH: AtomicU64 = AtomicU64::new(0);

/// 🧹️ A fresh directory under the system temp dir, removed when dropped.
pub(crate) struct Scratch(pub PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// 🧪️ A fresh scratch directory named after `label`.
pub(crate) fn scratch(label: &str) -> Scratch {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_nanos());
    let path = std::env::temp_dir().join(format!("teaching-proctor-{label}-{}-{nanos}-{}", std::process::id(), NEXT_SCRATCH.fetch_add(1, Ordering::Relaxed)));
    std::fs::create_dir_all(&path).expect("scratch directory");
    Scratch(path)
}

fn actor(id: &str) -> ActorKey {
    ActorKey { tenant: TenantId("t1".into()), kind: "quiz-learner".into(), id: id.into() }
}

fn event(stream: &ActorKey, seq: u64) -> EventRecord {
    EventRecord { stream: stream.clone(), seq, hlc: HybridLogicalClock { millis: 1_000 + seq, counter: 2 }, kind: "quiz.answer-recorded".into(), payload: format!("{{\"seq\":{seq}}}").into_bytes() }
}

#[tokio::test]
async fn a_database_file_is_stamped_and_a_foreign_format_is_refused() {
    let directory = scratch("format");
    drop(Database::open(&directory.0).expect("open"));
    assert!(directory.0.join(DATABASE_FILE).is_file());
    let raw = Connection::open(directory.0.join(DATABASE_FILE)).expect("raw");
    raw.execute("UPDATE proctor_format SET version = 99", []).expect("tamper");
    drop(raw);
    let Err(StorageError::Backend(detail)) = Database::open(&directory.0) else { panic!("a foreign format must be refused") };
    assert!(detail.contains("v99") && detail.contains(FORMAT_SCHEMA), "{detail}");
}

#[tokio::test]
async fn the_log_orders_events_of_every_stream_by_commit() {
    let mut store = SqliteAuthorityStore::new(Database::memory().expect("memory"));
    let (first, second) = (actor("a"), actor("b"));
    store.append_events(&first, &[event(&first, 1)], &[]).await.unwrap();
    store.append_events(&second, &[event(&second, 1), event(&second, 2)], &[]).await.unwrap();
    store.append_events(&first, &[event(&first, 2)], &[]).await.unwrap();
    assert_eq!(store.log_head().unwrap(), 4);
    let order: Vec<(u64, String, u64)> = store.log_after(0, 10).unwrap().into_iter().map(|logged| (logged.position, logged.event.stream.id, logged.event.seq)).collect();
    assert_eq!(order, vec![(1, "a".into(), 1), (2, "b".into(), 1), (3, "b".into(), 2), (4, "a".into(), 2)]);
    assert_eq!(store.log_after(2, 1).unwrap()[0].event, event(&second, 2));
}

#[tokio::test]
async fn an_outbox_row_carries_its_event_back_and_facts_survive_a_reopen() {
    let directory = scratch("reopen");
    let learner = actor("a");
    let recorded = event(&learner, 1);
    {
        let mut store = SqliteAuthorityStore::new(Database::open(&directory.0).expect("open"));
        store.append_events(&learner, std::slice::from_ref(&recorded), &[OutboxEntry::pending(learner.clone(), recorded.clone())]).await.unwrap();
        let receipt = CommandReceipt { command_id: CommandId("c1".into()), actor: learner.clone(), revision: Revision(1), accepted_at: HybridLogicalClock { millis: 5, counter: 1 } };
        store.record_receipt(&IdempotencyKey("c1".into()), &receipt).await.unwrap();
    }
    let store = SqliteAuthorityStore::new(Database::open(&directory.0).expect("reopen"));
    assert_eq!(store.events_since(&learner, 0).await.unwrap(), vec![recorded.clone()]);
    let pending = store.pending_outbox(10).await.unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].event.as_ref(), Some(&recorded));
    assert_eq!(pending[0].kind, recorded.kind);
    assert_eq!(store.receipt(&IdempotencyKey("c1".into())).await.unwrap().map(|receipt| receipt.revision), Some(Revision(1)));
}

#[tokio::test]
async fn a_commit_lands_whole_or_not_at_all() {
    let directory = scratch("commit");
    {
        let mut projections = SqliteProjectionStore::new(Database::open(&directory.0).expect("open"));
        let writes = vec![ProjectionWrite { projection: "quiz.learner".into(), key: "a".into(), value: b"1".to_vec() }, ProjectionWrite { projection: "quiz.run".into(), key: "r".into(), value: b"2".to_vec() }];
        projections.commit(&writes, ("quiz", 7)).unwrap();
        assert_eq!(projections.get("quiz.run", "r").await, Some(b"2".to_vec()));
        assert_eq!(projections.checkpoint("quiz").await, 7);
    }
    let mut refusing = SqliteProjectionStore::new(Database::open_read_only(&directory.0).expect("read-only"));
    assert!(matches!(refusing.commit(&[ProjectionWrite { projection: "quiz.run".into(), key: "r".into(), value: b"3".to_vec() }], ("quiz", 8)), Err(StorageError::Backend(_))));
    assert_eq!(refusing.get("quiz.run", "r").await, Some(b"2".to_vec()));
    assert_eq!(refusing.checkpoint("quiz").await, 7);
}
