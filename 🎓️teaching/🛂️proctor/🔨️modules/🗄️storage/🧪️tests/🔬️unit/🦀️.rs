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

/// 🗂️ The file names in `directory`, ascending.
fn files(directory: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(directory).expect("listing").filter_map(Result::ok).map(|entry| entry.file_name().to_string_lossy().into_owned()).collect();
    names.sort();
    names
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
    let raw = Connection::open(directory.0.join(DATABASE_FILE)).expect("raw");
    raw.execute("UPDATE proctor_format SET version = 2", []).expect("an earlier format");
    drop(raw);
    let Err(StorageError::Backend(earlier)) = Database::open(&directory.0) else { panic!("the format without challenges must be refused") };
    assert!(FORMAT_VERSION == 3 && earlier.contains("v2;") && earlier.contains("v3"), "{earlier}");
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

#[tokio::test]
async fn a_snapshot_taken_while_serving_is_a_whole_database_another_directory_adopts() {
    let (serving, other) = (scratch("snapshot"), scratch("adopt"));
    let learner = actor("a");
    let mut store = SqliteAuthorityStore::new(Database::open(&serving.0).expect("open"));
    store.append_events(&learner, &[event(&learner, 1), event(&learner, 2)], &[]).await.unwrap();
    let copy = other.0.join("copy.sqlite");
    let mut heard = Vec::new();
    assert_eq!(Database::snapshot(&serving.0, &copy, &CancelToken::root_now(), |written, size| heard.push((written, size))).expect("snapshot"), Some(Inspection { events: 2, head: 2 }));
    assert!(heard.last().is_some_and(|(written, size)| written == size && *size > 0), "the copy reports its progress up to the whole: {heard:?}");
    assert_eq!(files(&other.0), ["copy.sqlite"], "nothing but the named copy is left");
    assert_eq!(&std::fs::read(&copy).expect("copy")[..16], b"SQLite format 3\0");
    assert!(!other.0.join("copy.sqlite-wal").exists(), "the copy needs no WAL beside it");
    let taken = std::fs::read(&copy).expect("copy");
    assert!(matches!(Database::snapshot(&serving.0, &copy, &CancelToken::root_now(), |_, _| {}), Err(StorageError::Backend(detail)) if detail.contains("never overwritten")));
    assert_eq!(std::fs::read(&copy).expect("copy"), taken, "an existing target is never overwritten");
    store.append_events(&learner, &[event(&learner, 3)], &[]).await.unwrap();
    assert_eq!(Database::inspect(&copy).expect("inspect"), Inspection { events: 2, head: 2 }, "the copy is the moment it was taken");
    let cancelled = CancelToken::root_now();
    cancelled.cancel_now();
    assert_eq!(Database::snapshot(&serving.0, &other.0.join("never.sqlite"), &cancelled, |_, _| {}).expect("a cancellation is no failure"), None);
    assert_eq!(files(&other.0), ["copy.sqlite"], "a cancelled copy leaves neither its name nor a partial file");

    let Err(StorageError::Backend(detail)) = Database::adopt(&serving.0, &copy) else { panic!("a database in use must not be replaced") };
    assert!(detail.contains("in use"), "{detail}");
    assert_eq!(store.log_head().unwrap(), 3);

    let staged = other.0.join("staged.sqlite");
    std::fs::copy(&copy, &staged).expect("stage");
    assert_eq!(Database::adopt(&other.0, &staged).expect("adopt"), 2);
    assert!(!staged.exists());
    let adopted = SqliteAuthorityStore::new(Database::open(&other.0).expect("adopted"));
    assert_eq!(adopted.events_since(&learner, 0).await.unwrap(), vec![event(&learner, 1), event(&learner, 2)]);
    drop(adopted);

    std::fs::copy(&copy, &staged).expect("stage again");
    std::fs::write(other.0.join(format!("{DATABASE_FILE}-wal")), b"stale").expect("stale wal");
    assert_eq!(Database::adopt(&other.0, &staged).expect("replace a stopped database"), 2);
    assert!(!other.0.join(format!("{DATABASE_FILE}-wal")).exists(), "a stale WAL never outlives the database it belonged to");
}

#[test]
fn only_a_whole_proctor_database_is_adopted() {
    let directory = scratch("refuse");
    let garbage = directory.0.join("garbage.sqlite");
    std::fs::write(&garbage, b"not a database").expect("garbage");
    assert!(matches!(Database::adopt(&directory.0, &garbage), Err(StorageError::Backend(_))));
    let foreign = directory.0.join("foreign.sqlite");
    Connection::open(&foreign).expect("foreign").execute_batch("CREATE TABLE other (id INTEGER)").expect("schema");
    let Err(StorageError::Backend(detail)) = Database::adopt(&directory.0, &foreign) else { panic!("a foreign database must be refused") };
    assert!(detail.contains("is not a proctor database"), "{detail}");
    assert!(!directory.0.join(DATABASE_FILE).exists());
}

#[tokio::test]
async fn an_operator_opens_only_a_stopped_proctors_database() {
    let directory = scratch("offline");
    let Err(StorageError::Backend(missing)) = Database::open_offline(&directory.0) else { panic!("there is nothing to open") };
    assert!(missing.contains("does not exist") && !directory.0.join(DATABASE_FILE).exists(), "{missing}");
    let serving = Database::open(&directory.0).expect("serving");
    let asked = std::time::Instant::now();
    let Err(StorageError::Backend(detail)) = Database::open_offline(&directory.0) else { panic!("a served database must be refused") };
    assert!(detail.contains("in use") && detail.contains("stop the proctor"), "{detail}");
    assert!(asked.elapsed() < BUSY_TIMEOUT / 2, "the refusal does not wait out the writer's busy timeout: {:?}", asked.elapsed());
    drop(serving);
    assert!(Database::open_offline(&directory.0).is_ok());
}

#[tokio::test]
async fn an_erasure_removes_every_trace_of_its_actors_from_the_file_and_nothing_of_anybody_else() {
    const NAME: &str = "Ada Lovelace of Ockham";
    let directory = scratch("erase");
    let (ada, bob) = (actor("a"), actor("b"));
    let handle = ActorKey { tenant: TenantId("t1".into()), kind: "quiz-handle".into(), id: "616461".into() };
    let named = |stream: &ActorKey, seq: u64, name: &str| EventRecord { payload: format!("{{\"handle\":\"{name}\",\"seq\":{seq}}}").into_bytes(), ..event(stream, seq) };
    let receipt = |command: &str, actor: &ActorKey| CommandReceipt { command_id: CommandId(command.into()), actor: actor.clone(), revision: Revision(1), accepted_at: HybridLogicalClock { millis: 5, counter: 1 } };
    {
        let database = Database::open(&directory.0).expect("open");
        let mut store = SqliteAuthorityStore::new(database.clone());
        let claimed = named(&handle, 1, NAME);
        store.append_events(&handle, std::slice::from_ref(&claimed), &[OutboxEntry::pending(handle.clone(), claimed.clone())]).await.unwrap();
        let (first, second) = (named(&ada, 1, NAME), named(&ada, 2, NAME));
        store.append_events(&ada, &[first.clone(), second.clone()], &[OutboxEntry::pending(ada.clone(), first), OutboxEntry::pending(ada.clone(), second)]).await.unwrap();
        let other = named(&bob, 1, "Bob");
        store.append_events(&bob, std::slice::from_ref(&other), &[OutboxEntry::pending(bob.clone(), other.clone())]).await.unwrap();
        let delivered: Vec<u64> = store.pending_outbox(2).await.unwrap().iter().map(|entry| entry.id).collect();
        store.mark_outbox_delivered(&delivered).await.unwrap();
        store.record_receipt(&IdempotencyKey("claim".into()), &receipt("claim", &handle)).await.unwrap();
        store.record_receipt(&IdempotencyKey(format!("enroll:{NAME}")), &receipt("enroll", &ada)).await.unwrap();
        store.record_receipt(&IdempotencyKey("bob".into()), &receipt("bob", &bob)).await.unwrap();
        store.put_snapshot(&ada, Revision(2), NAME.as_bytes().to_vec()).await.unwrap();
        store.put_snapshot(&bob, Revision(1), b"Bob".to_vec()).await.unwrap();
        store.acquire_lease(&ada, "node").await.unwrap();
        let mut projections = SqliteProjectionStore::new(database);
        projections.commit(&[ProjectionWrite { projection: "quiz.learner".into(), key: "a".into(), value: NAME.as_bytes().to_vec() }, ProjectionWrite { projection: "quiz.handle".into(), key: NAME.to_lowercase(), value: b"a".to_vec() }], ("quiz", 4)).unwrap();
    }
    let holds = |needle: &str| files(&directory.0).iter().any(|file| std::fs::read(directory.0.join(file)).expect("file").windows(needle.len()).any(|window| window == needle.as_bytes()));
    assert!(holds(NAME) && holds(&NAME.to_lowercase()), "the name is in the file before it is erased");

    let database = Database::open_offline(&directory.0).expect("stopped");
    let store = SqliteAuthorityStore::new(database.clone());
    let actors = [ada.clone(), handle.clone()];
    let preview = database.erasure(&actors).expect("dry run");
    assert_eq!(preview, Erasure { actors: vec![ErasedActor { actor: ada.clone(), events: 2, receipts: 1, outbox: 2, snapshots: 1, leases: 1 }, ErasedActor { actor: handle.clone(), events: 1, receipts: 1, outbox: 1, snapshots: 0, leases: 0 }], projections: 2 });
    assert_eq!((store.log_head().unwrap(), store.events_since(&ada, 0).await.unwrap().len(), holds(NAME)), (4, 2, true), "a dry run changes nothing");

    assert_eq!(database.erase(&actors).expect("erase"), preview);
    assert_eq!(database.erasure(&actors).expect("again"), Erasure { actors: actors.iter().map(|actor| ErasedActor { actor: actor.clone(), events: 0, receipts: 0, outbox: 0, snapshots: 0, leases: 0 }).collect(), projections: 0 });
    assert!(store.events_since(&ada, 0).await.unwrap().is_empty() && store.events_since(&handle, 0).await.unwrap().is_empty());
    assert_eq!((store.receipt(&IdempotencyKey("claim".into())).await.unwrap(), store.receipt(&IdempotencyKey(format!("enroll:{NAME}"))).await.unwrap(), store.snapshot(&ada).await.unwrap()), (None, None, None));
    assert_eq!(store.events_since(&bob, 0).await.unwrap(), vec![named(&bob, 1, "Bob")], "another learner keeps its facts");
    assert_eq!(store.receipt(&IdempotencyKey("bob".into())).await.unwrap(), Some(receipt("bob", &bob)));
    assert_eq!(store.snapshot(&bob).await.unwrap(), Some((Revision(1), b"Bob".to_vec())));
    assert_eq!(store.pending_outbox(10).await.unwrap().iter().map(|entry| entry.actor.clone()).collect::<Vec<_>>(), vec![bob.clone()]);
    assert_eq!((store.streams("t1", "quiz-learner").unwrap(), store.streams("t1", "quiz-handle").unwrap(), store.tenants().unwrap()), (vec!["b".to_string()], Vec::new(), vec!["t1".to_string()]));
    let projections = SqliteProjectionStore::new(database.clone());
    assert_eq!((projections.get("quiz.learner", "a").await, projections.checkpoint("quiz").await), (None, 0), "every read model is dropped and refolded by the next start");
    assert!(!holds(NAME) && !holds(&NAME.to_lowercase()) && holds("Bob"), "the bytes of the name are in no file of the directory while the handle is still open");
    drop((store, projections, database));
    assert!(!holds(NAME) && !holds(&NAME.to_lowercase()) && holds("Bob"), "nor after it is closed");
    assert_eq!(Database::inspect(&directory.0.join(DATABASE_FILE)).expect("whole"), Inspection { events: 1, head: 4 });
    let mut reopened = SqliteAuthorityStore::new(Database::open(&directory.0).expect("reopen"));
    let again = named(&handle, 1, "Somebody else");
    assert_eq!(reopened.append_events(&handle, std::slice::from_ref(&again), &[]).await.unwrap(), 1, "the erased stream starts over");
}

#[tokio::test]
async fn streams_are_seen_at_a_glance_and_removed_as_one_set() {
    let directory = scratch("remove");
    let database = Database::open(&directory.0).expect("open");
    let mut store = SqliteAuthorityStore::new(database.clone());
    let at = |stream: &ActorKey, seq: u64, millis: u64, kind: &str| EventRecord { hlc: HybridLogicalClock { millis, counter: 0 }, kind: kind.into(), ..event(stream, seq) };
    let learners: Vec<ActorKey> = (0..300).map(|learner| actor(&format!("{learner:032x}"))).collect();
    for (index, learner) in learners.iter().enumerate() {
        let mut events = vec![at(learner, 1, 500 + index as u64, "quiz.learner-registered"), at(learner, 2, 9_000, "quiz.run-started")];
        if index % 3 == 0 {
            events.extend([at(learner, 3, 9_500, "quiz.run-submitted"), at(learner, 4, 9_600, "quiz.run-submitted")]);
        }
        let queued: Vec<OutboxEntry> = events.iter().map(|event| OutboxEntry::pending(learner.clone(), event.clone())).collect();
        store.append_events(learner, &events, &queued).await.unwrap();
        store.record_receipt(&IdempotencyKey(format!("register-{index}")), &CommandReceipt { command_id: CommandId(format!("register-{index}")), actor: learner.clone(), revision: Revision(1), accepted_at: HybridLogicalClock { millis: 500, counter: 0 } }).await.unwrap();
    }
    let delivered: Vec<u64> = store.pending_outbox(100).await.unwrap().iter().map(|entry| entry.id).collect();
    store.mark_outbox_delivered(&delivered).await.unwrap();
    store.put_snapshot(&learners[1], Revision(2), b"state".to_vec()).await.unwrap();
    store.acquire_lease(&learners[1], "node").await.unwrap();
    store.acquire_lease(&learners[0], "node").await.unwrap();
    SqliteProjectionStore::new(database.clone()).commit(&[ProjectionWrite { projection: "quiz.learner".into(), key: "a".into(), value: b"view".to_vec() }], ("quiz", 7)).unwrap();

    let spans = store.stream_spans("t1", "quiz-learner", "quiz.run-submitted").unwrap();
    assert_eq!(spans.len(), 300);
    assert_eq!((&spans[0], &spans[1], &spans[299]), (&StreamSpan { id: learners[0].id.clone(), since: 500, events: 4, marked: 2 }, &StreamSpan { id: learners[1].id.clone(), since: 501, events: 2, marked: 0 }, &StreamSpan { id: learners[299].id.clone(), since: 799, events: 2, marked: 0 }));
    assert!(store.stream_spans("t1", "quiz-handle", "quiz.run-submitted").unwrap().is_empty() && store.stream_spans("t2", "quiz-learner", "x").unwrap().is_empty());
    let first = store.first_events("t1", "quiz-learner").unwrap();
    assert_eq!((first.len(), &first[7]), (300, &at(&learners[7], 1, 507, "quiz.learner-registered")), "the first event of every stream, by stream");

    let relay = IdempotencyKey("relay-of-a-removed-stream".into());
    store.record_receipt(&relay, &CommandReceipt { command_id: CommandId("relay".into()), actor: learners[0].clone(), revision: Revision(4), accepted_at: HybridLogicalClock { millis: 600, counter: 0 } }).await.unwrap();
    let idle: Vec<ActorKey> = learners.iter().enumerate().filter(|(index, _)| index % 3 != 0).map(|(_, learner)| learner.clone()).collect();
    let removed = database.remove(&idle, &[relay.clone(), IdempotencyKey("no such key".into())]).expect("removed");
    assert_eq!(removed, Removed { events: 400, receipts: 201, outbox: 400, snapshots: 1, leases: 1, projections: 1 }, "two hundred streams as one set, and the one receipt a stream that stays held for one of them");
    assert_eq!((store.receipt(&relay).await.unwrap(), store.receipt(&IdempotencyKey("register-0".into())).await.unwrap().map(|receipt| receipt.actor)), (None, Some(learners[0].clone())));
    assert_eq!(database.remove(&idle, std::slice::from_ref(&relay)).expect("again"), Removed::default(), "nothing of them is left to remove");
    let left = store.stream_spans("t1", "quiz-learner", "quiz.run-submitted").unwrap();
    assert!(left.len() == 100 && left.iter().all(|span| span.events == 4 && span.marked == 2), "every other stream is whole");
    assert_eq!(store.receipt(&IdempotencyKey("register-1".into())).await.unwrap(), None);
    assert!(store.receipt(&IdempotencyKey("register-3".into())).await.unwrap().is_some());
    assert_eq!((delivered.len(), store.pending_outbox(10_000).await.unwrap().len()), (100, 348), "of the four hundred rows of the streams that stay, the fifty-two delivered before are still delivered");
    assert!(store.validate_lease(&learners[0], &Lease { epoch: 1, holder: "node".into() }).await);
    let mut total = Removed::default();
    total += removed;
    total += Removed { events: 1, ..Removed::default() };
    assert_eq!(total, Removed { events: 401, ..removed });
    database.scrub().expect("scrubbed");
    assert_eq!(Database::inspect(&directory.0.join(DATABASE_FILE)).expect("whole").events, 400);
}

#[tokio::test]
async fn a_batch_of_turns_is_one_transaction() {
    let mut store = SqliteAuthorityStore::new(Database::memory().expect("memory"));
    let (first, second) = (actor("a"), actor("b"));
    let receipt = |command: &str, actor: &ActorKey, revision: u64| CommandReceipt { command_id: CommandId(command.into()), actor: actor.clone(), revision: Revision(revision), accepted_at: HybridLogicalClock { millis: 5, counter: 1 } };
    let (one, two, gap) = ([event(&first, 1)], [event(&second, 1)], [event(&second, 7)]);
    let (queued_one, queued_two) = ([OutboxEntry::pending(first.clone(), one[0].clone())], [OutboxEntry::pending(second.clone(), two[0].clone())]);
    let (one_receipt, two_receipt, gap_receipt) = (receipt("c1", &first, 1), receipt("c2", &second, 1), receipt("c3", &second, 7));
    let (one_key, two_key, gap_key) = (IdempotencyKey("c1".into()), IdempotencyKey("c2".into()), IdempotencyKey("c3".into()));

    let refused = store
        .commit(&[
            TurnCommit { actor: &first, events: &one, outbox: &queued_one, receipt: Some((&one_key, &one_receipt)), snapshot: Some((Revision(1), b"state")) },
            TurnCommit { actor: &second, events: &gap, outbox: &[], receipt: Some((&gap_key, &gap_receipt)), snapshot: None },
        ])
        .await;
    assert_eq!(refused, [Err(StorageError::SequenceGap { expected: 1, got: 7 }), Err(StorageError::SequenceGap { expected: 1, got: 7 })], "one turn the store cannot write refuses the whole batch");
    assert_eq!((store.log_head().unwrap(), store.receipt(&one_key).await.unwrap(), store.snapshot(&first).await.unwrap(), store.pending_outbox(10).await.unwrap().len()), (0, None, None, 0), "and nothing of any turn is left");

    let committed = store
        .commit(&[
            TurnCommit { actor: &first, events: &one, outbox: &queued_one, receipt: Some((&one_key, &one_receipt)), snapshot: Some((Revision(1), b"state")) },
            TurnCommit { actor: &second, events: &two, outbox: &queued_two, receipt: Some((&two_key, &two_receipt)), snapshot: None },
        ])
        .await;
    assert_eq!(committed, [Ok(()), Ok(())]);
    assert_eq!((store.log_head().unwrap(), store.receipt(&one_key).await.unwrap(), store.receipt(&two_key).await.unwrap()), (2, Some(one_receipt), Some(two_receipt)));
    assert_eq!(store.snapshot(&first).await.unwrap(), Some((Revision(1), b"state".to_vec())));
    assert_eq!(store.pending_outbox(10).await.unwrap().iter().map(|entry| entry.actor.id.clone()).collect::<Vec<_>>(), ["a", "b"]);
    let late = [event(&first, 2)];
    let older = store.commit(&[TurnCommit { actor: &first, events: &late, outbox: &[], receipt: None, snapshot: Some((Revision(0), b"older")) }]).await;
    assert_eq!((older, store.snapshot(&first).await.unwrap()), (vec![Ok(())], Some((Revision(1), b"state".to_vec()))), "a snapshot older than the stored one is skipped and fails nothing");
}

#[tokio::test]
async fn pending_outbox_rows_are_looked_for_after_everything_delivered() {
    let mut store = SqliteAuthorityStore::new(Database::memory().expect("memory"));
    let learner = actor("a");
    let rows: Vec<OutboxEntry> = (1..=6).map(|seq| OutboxEntry::pending(learner.clone(), event(&learner, seq))).collect();
    store.enqueue_outbox(rows).await.unwrap();
    let cursor = |store: &SqliteAuthorityStore| store.database.with(|connection| outbox_cursor(connection)).unwrap();
    let pending = |entries: Vec<OutboxEntry>| entries.iter().map(|entry| entry.id).collect::<Vec<_>>();
    assert_eq!((cursor(&store), pending(store.pending_outbox(10).await.unwrap())), (0, vec![1, 2, 3, 4, 5, 6]));
    store.mark_outbox_delivered(&[2, 3]).await.unwrap();
    assert_eq!((cursor(&store), pending(store.pending_outbox(10).await.unwrap())), (0, vec![1, 4, 5, 6]), "a row delivered out of order does not hide the one before it");
    store.mark_outbox_delivered(&[1]).await.unwrap();
    assert_eq!((cursor(&store), pending(store.pending_outbox(10).await.unwrap())), (3, vec![4, 5, 6]));
    store.mark_outbox_delivered(&[]).await.unwrap();
    store.mark_outbox_delivered(&[4, 5, 6]).await.unwrap();
    assert_eq!((cursor(&store), pending(store.pending_outbox(10).await.unwrap())), (6, Vec::new()), "with nothing pending the cursor is the newest row");
    store.enqueue_outbox(vec![OutboxEntry::pending(learner.clone(), event(&learner, 7))]).await.unwrap();
    assert_eq!(pending(store.pending_outbox(10).await.unwrap()), [7]);
    assert_eq!(store.mark_outbox_delivered(&[99]).await, Err(StorageError::NotFound));
    assert_eq!(cursor(&store), 6, "a refused acknowledgement moves nothing");
}

#[tokio::test]
async fn only_what_is_made_again_commits_without_waiting_for_the_disk() {
    const FULL: i64 = 2;
    const NORMAL: i64 = 1;
    let directory = scratch("remakeable");
    let database = Database::open(&directory.0).expect("opened");
    let waits = |database: &Database| database.with(|connection| connection.query_row("PRAGMA synchronous", [], |row| row.get::<_, i64>(0)).map_err(backend)).unwrap();
    let within = database.remakeable(|transaction| transaction.query_row("PRAGMA synchronous", [], |row| row.get::<_, i64>(0)).map_err(backend)).unwrap();
    assert_eq!((within, waits(&database)), (NORMAL, FULL), "a remakeable write does not wait, and facts wait again right after it");
    assert_eq!(database.transaction(|transaction| transaction.query_row("PRAGMA synchronous", [], |row| row.get::<_, i64>(0)).map_err(backend)).unwrap(), FULL);

    let mut projections = SqliteProjectionStore::new(database.clone());
    let mut authority = SqliteAuthorityStore::new(database.clone());
    let learner = actor("a");
    authority.append_events(&learner, &[event(&learner, 1)], &[OutboxEntry::pending(learner.clone(), event(&learner, 1))]).await.unwrap();
    projections.commit(&[ProjectionWrite { projection: "p".into(), key: "k".into(), value: vec![1] }], ("p", 1)).unwrap();
    projections.put("p", "l", vec![2]).await.unwrap();
    projections.set_checkpoint("p", 2).await.unwrap();
    authority.mark_outbox_delivered(&[1]).await.unwrap();
    assert_eq!(database.remakeable(|_| Err::<(), _>(StorageError::NotFound)), Err(StorageError::NotFound));
    assert_eq!(waits(&database), FULL, "whatever a remakeable write ends in, facts wait for the disk after it");

    drop((projections, authority, database));
    let reopened = Database::open(&directory.0).expect("reopened");
    let projections = SqliteProjectionStore::new(reopened.clone());
    assert_eq!((projections.get("p", "k").await, projections.get("p", "l").await, projections.checkpoint("p").await), (Some(vec![1]), Some(vec![2]), 2), "a process that ends loses none of it");
    assert!(SqliteAuthorityStore::new(reopened).pending_outbox(8).await.unwrap().is_empty());
}
