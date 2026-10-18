/// 🔁️ WAL command decoder and replay laws: the admitted wal bytes, the command decoder agreement, the retained hello replay, the missing tail stream, the fifty thousand edit stream and the neutral aborted replay.

async fn admitted_wal_bytes(mut bytes: Vec<u8>) -> db_wal_log::WalBytes {
    bytes.shrink_to_fit();
    let mut control = db_wal_log::WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap();
    db_wal_log::WalBytes::try_admit(bytes, (db_io_core::DB_IO_OPERATION_PAGES * db_io_core::DB_IO_PAGE_BYTES) as u64, &mut control).await.unwrap()
}

fn decode_control(cancelled: bool, stall: std::time::Duration) -> db_wal_log::WalCursorControl {
    db_wal_log::WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(cancelled)), std::time::Instant::now() + stall, 1_000_000).unwrap()
}

async fn decoded_wal_command(bytes: Vec<u8>, control: &mut db_wal_log::WalCursorControl) -> Result<protocol::MutationEnvelope, DbError> {
    let mut admitted = admitted_wal_bytes(bytes).await;
    let decoded = decode_wal_command(&admitted, control);
    while admitted.close_step().unwrap().is_some() {}
    while db_io_driver::db_io_maintenance_step().unwrap() {}
    decoded
}

/// 📜️ The one WAL command decoder reads, in one step, exactly what the protocol codec (the reference
/// implementation) reads from the same bytes — one-page and gathered multi-page records, many
/// dependencies, non-ASCII ids — reads a record's id alone, and refuses a cancelled or expired
/// control, trailing bytes, a truncated record and a count the record cannot hold.
#[semio_framework_async_macros::async_test]
async fn wal_command_decoder_agrees_with_the_protocol_codec_and_refuses_what_a_record_cannot_hold() {
    let stall = std::time::Duration::from_secs(30);
    let mut pages = sample_envelope("wal-real-path", 1).await;
    pages.dependencies = vec![protocol::MutationId("dependency-a".to_string()), protocol::MutationId("dependency-b".to_string())];
    pages.diff.payload = vec![0x4d; db_io_core::DB_IO_PAGE_BYTES + 17];
    pages.inverse.payload = vec![0x2a; db_io_core::DB_IO_PAGE_BYTES + 1];
    let mut many = sample_envelope("übergänge-✓", 2).await;
    many.dependencies = (0..512).map(|index| protocol::MutationId(format!("dependency-{index}"))).collect();
    many.observed = Some(protocol::MutationId("dependency-7".to_string()));
    many.target = vec!["features".to_string(), "ü".to_string()];
    let mut maximum = sample_envelope("maximum", 3).await;
    maximum.diff.payload = vec![0x6d; (db_io_core::DB_IO_OPERATION_PAGES - 1) * db_io_core::DB_IO_PAGE_BYTES];
    for envelope in [sample_envelope("plain", 0).await, pages.clone(), many, maximum] {
        let encoded = encode_command_envelope(&envelope).await;
        let reference = protocol::decode_envelope(&encoded, &mut 0).unwrap();
        let decoded = decoded_wal_command(encoded, &mut decode_control(false, stall)).await.unwrap();
        assert_eq!(decoded, reference);
        assert_eq!(decoded, envelope);
        let mut admitted = admitted_wal_bytes(encode_command_envelope(&envelope).await).await;
        assert_eq!(wal_command_id(&admitted, &mut decode_control(false, stall)).unwrap(), envelope.mutation_id.0);
        while admitted.close_step().unwrap().is_some() {}
    }
    let encoded = encode_command_envelope(&pages).await;
    assert!(matches!(decoded_wal_command(encoded.clone(), &mut decode_control(true, stall)).await, Err(DbError::Unavailable(message)) if message == "wal cursor cancelled"));
    assert!(matches!(decoded_wal_command(encoded.clone(), &mut decode_control(false, std::time::Duration::ZERO)).await, Err(DbError::Unavailable(message)) if message == "wal cursor deadline reached"));
    let mut trailing = encoded.clone();
    trailing.push(0xff);
    assert!(matches!(decoded_wal_command(trailing, &mut decode_control(false, stall)).await, Err(DbError::Corrupt(message)) if message == "wal command envelope has trailing bytes"));
    assert!(matches!(decoded_wal_command(encoded[..encoded.len() - 5].to_vec(), &mut decode_control(false, stall)).await, Err(DbError::Corrupt(_))));
    let overcounted = vec![1, b'm', 1, b'd', 1, b'a', 0xff, 0xff, 0x03];
    assert!(matches!(decoded_wal_command(overcounted, &mut decode_control(false, stall)).await, Err(DbError::Corrupt(_))));
}

/// 🧾️ A retained hello decodes only the commands its replica lacks: every earlier command is
/// counted and hashed into the same frontier the whole-WAL replay derives, and the last command's
/// id is the server head, while no command is materialized.
#[semio_framework_async_macros::async_test]
async fn retained_hello_replay_derives_the_server_frontier_without_decoding_the_tail() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-tail".into();
    seed_wal(&storage, &document, 6).await;
    let ordinary = replay_sync_state(&storage, document.clone()).await.unwrap();
    let storage = db_storage::DbBackend::Memory(storage);
    let mut ledger = DatabaseSyncHelloBackingLedger::default();
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let expired = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let progress = std::sync::atomic::AtomicU8::new(0);
    let replay = replay_sync_state_retained(&storage, document.clone(), cancelled, expired, &mut ledger, &progress).await.unwrap();
    assert_eq!(replay.frontier, ordinary.frontier);
    assert_eq!(replay.head_edit_id, "op-5");
    assert_eq!((ledger.items, ledger.bytes), (1, replay.head_edit_id.capacity()), "only the server head id is held");
}

/// 🚰️ Drives one retained hello to its end and answers the welcome's server frontier with every tail frame's
/// envelopes and frontier, acknowledging each frame, plus the largest backing the hello held while a frame was out.
async fn stream_hello_tail(storage: std::sync::Arc<db_storage::DbBackend>, document: ArtifactId, replica: Option<protocol::RuntimeFrontierSummary>) -> (protocol::RuntimeFrontierSummary, Vec<(Vec<protocol::MutationEnvelope>, protocol::RuntimeFrontierSummary)>, usize, usize) {
    let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let result = DatabaseSyncHelloFuture::try_submit(pool, storage, document, replica, String::from("tail-stream-session"), protocol::ActorId(String::from("tail-stream-origin")), 64 * 1024).unwrap().await.unwrap();
    let mut session = result.close_and_take_session().unwrap();
    let welcome = session.take_welcome().unwrap();
    let server = match welcome.frame().unwrap() {
        protocol::ServerFrame::Welcome { server_frontier, .. } => server_frontier.clone(),
        other => panic!("expected Welcome, got {other:?}"),
    };
    welcome.acknowledge().unwrap();
    let (mut frames, mut peak_items, mut peak_bytes) = (Vec::new(), 0usize, 0usize);
    while let Some(frame) = session.next_frame().await.unwrap() {
        {
            let state = session.state.as_ref().unwrap();
            let core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let ledger = core.execution.as_ref().and_then(|execution| execution.ledger.as_ref()).unwrap();
            peak_items = peak_items.max(ledger.items);
            peak_bytes = peak_bytes.max(ledger.bytes);
        }
        match frame.frame().unwrap() {
            protocol::ServerFrame::Commands { envelopes, frontier, .. } => frames.push((envelopes.clone(), frontier.clone())),
            other => panic!("expected a tail Commands frame, got {other:?}"),
        }
        frame.acknowledge().unwrap();
    }
    (server, frames, peak_items, peak_bytes)
}

/// ⚖️ A retained hello streams exactly the commands a replica lacks — for a fresh replica, one mid-way and one caught up —
/// in order, and its last frame's frontier is the welcome's server frontier, commit sequence included although a
/// transaction without a command (a snapshot marker at floor 0) follows the last command.
#[semio_framework_async_macros::async_test]
async fn a_retained_hello_streams_exactly_the_missing_tail() {
    let memory = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-tail-stream".into();
    seed_wal(&memory, &document, 6).await;
    publish_snapshot_marker(&memory, &document, 1, Frontier { document: document.clone(), head_seq: 0, commit_seq: 0, chain_hash: [0; 32], epoch: 0 }).await;
    let ordinary = replay_sync_state(&memory, document.clone()).await.unwrap();
    assert_eq!(ordinary.frontier.commit_seq, 7, "the marker transaction counts in the server's commit sequence");
    let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(memory));
    for replica_head in [0u64, 4, 6] {
        let replica = (replica_head > 0).then(|| protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(document.0.clone()), head_edit_ordinal: replica_head, head_edit_id: format!("op-{}", replica_head - 1), last_commit_seq: replica_head, chain_hash: [0; 32] });
        let (server, frames, _, _) = stream_hello_tail(storage.clone(), document.clone(), replica).await;
        let streamed: Vec<protocol::MutationEnvelope> = frames.iter().flat_map(|(envelopes, _)| envelopes.clone()).collect();
        assert_eq!(streamed, ordinary.commands[replica_head as usize..].to_vec(), "replica head {replica_head}");
        if let Some((_, last)) = frames.last() {
            assert_eq!(*last, server);
        } else {
            assert_eq!(replica_head, 6);
        }
    }
}

/// ⚖️ A document of 50 000 single-command commits opens for a fresh replica: its tail streams as 196 frames of at most
/// `DATABASE_SYNC_HELLO_TAIL_FRAME_ENVELOPES` envelopes on commit boundaries, each frontier exact (head and commit counts)
/// and the last equal to the welcome's, while the hello never holds more than about one frame — the old hello decoded the
/// whole tail first and refused at ~8 000 envelopes (`cumulative envelope backing`). A replica at 40 000 receives exactly
/// the last 10 000.
#[semio_framework_async_macros::async_test]
async fn a_fifty_thousand_edit_document_streams_to_a_fresh_replica_within_one_frame_of_backing() {
    const EDITS: u64 = 50_000;
    let dir = std::env::temp_dir().join(format!("db_sync_fifty_thousand_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let fs = db_storage_fs::FsStorage::open(db_io_driver::db_io_test_pool(), &dir).await.unwrap();
    let document: ArtifactId = "doc-fifty-thousand".into();
    let mut wal = db_actor::block_on(ArtifactWal::create(&fs, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
    for i in 0..EDITS {
        let envelope = sample_envelope(&format!("op-{i}"), i).await;
        let mut records = db_wal_log::WalRecordBatch::new();
        assert!(records.push(command_record(&envelope).await).is_ok());
        wal.submit(&fs, &[], &records, DurabilityClass::Os, i).await.unwrap();
        while records.close_step().unwrap() {}
    }
    wal.close().await.unwrap();
    let storage = std::sync::Arc::new(db_storage::DbBackend::Fs(fs));
    let (server, frames, peak_items, peak_bytes) = stream_hello_tail(storage.clone(), document.clone(), None).await;
    assert_eq!(server.head_edit_ordinal, EDITS);
    assert_eq!(frames.len(), EDITS.div_ceil(DATABASE_SYNC_HELLO_TAIL_FRAME_ENVELOPES as u64) as usize);
    let mut expected = 0u64;
    for (envelopes, frontier) in &frames {
        assert!(!envelopes.is_empty() && envelopes.len() <= DATABASE_SYNC_HELLO_TAIL_FRAME_ENVELOPES);
        for envelope in envelopes {
            assert_eq!(envelope.mutation_id.0, format!("op-{expected}"));
            expected += 1;
        }
        assert_eq!((frontier.head_edit_ordinal, frontier.last_commit_seq), (expected, expected), "each frame ends on the commit of its last command");
        assert_eq!(frontier.head_edit_id, format!("op-{}", expected - 1));
    }
    assert_eq!(expected, EDITS);
    assert_eq!(frames.last().unwrap().1, server, "the last frame reaches the welcome's frontier, chain hash included");
    assert!(peak_items <= 16 * DATABASE_SYNC_HELLO_TAIL_FRAME_ENVELOPES, "the hello held {peak_items} items while a frame was out");
    assert!(peak_bytes <= 4 * 1024 * 1024, "the hello held {peak_bytes} bytes while a frame was out");
    let replica = protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(document.0.clone()), head_edit_ordinal: 40_000, head_edit_id: String::from("op-39999"), last_commit_seq: 40_000, chain_hash: [0; 32] };
    let (_, frames, _, _) = stream_hello_tail(storage.clone(), document, Some(replica)).await;
    let streamed: Vec<String> = frames.iter().flat_map(|(envelopes, _)| envelopes.iter().map(|envelope| envelope.mutation_id.0.clone())).collect();
    assert_eq!(streamed, (40_000..EDITS).map(|i| format!("op-{i}")).collect::<Vec<_>>());
    drop(storage);
    let _ = std::fs::remove_dir_all(&dir);
}

#[semio_framework_async_macros::async_test]
async fn sync_replay_ignores_neutral_aborted_command_snapshot_and_cas() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../📝️wal/🧪️tests/🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == "aborted-commands-snapshot-cas-have-no-effects").unwrap();
    let document = ArtifactId::from("committed-sync");
    let storage = db_wal::testkit::committed_fixture_storage(row, &document).await;
    let ordinary = replay_sync_state(&storage, document.clone()).await.unwrap();
    assert!(ordinary.commands.is_empty());
    assert_eq!(ordinary.floor_head_seq, 0);
    assert_eq!(ordinary.frontier.head_seq, 0);
    assert_eq!(ordinary.frontier.commit_seq, 0);
    assert_eq!(ordinary.frontier.chain_hash, [0; 32]);
    let storage = db_storage::DbBackend::Memory(storage);
    let mut ledger = DatabaseSyncHelloBackingLedger::default();
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let expired = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let progress = std::sync::atomic::AtomicU8::new(0);
    let retained = replay_sync_state_retained(&storage, document, cancelled, expired, &mut ledger, &progress).await.unwrap();
    assert_eq!(retained.floor_head_seq, 0);
    assert_eq!(retained.frontier, ordinary.frontier);
    assert_eq!(ledger.items, 0);
    assert_eq!(ledger.bytes, 0);
}

use db_storage_memory::MemoryStorage;
use db_wal::ArtifactWal;
use db_wal_log::{GroupCommitPolicy, WalRecord};
use db_ids::ArtifactId;

//#region 🧸️Fixtures
async fn sample_envelope(id: &str, seq: u64) -> protocol::MutationEnvelope {
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(id.to_string()),
        document_id: protocol::ArtifactId("doc-1".to_string()),
        actor: protocol::ActorId("actor-1".to_string()),
        dependencies: Vec::new(),
        observed: None,
        target: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId("diff.v1".to_string()), payload: seq.to_le_bytes().to_vec() },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId("diff.v1".to_string()), payload: Vec::new() },
        timestamp: protocol::HybridLogicalTimestamp::new(1, seq),
        transaction: None, verb: None, line: None,
    }
}

async fn submit_record(storage: &MemoryStorage, wal: &mut ArtifactWal, record: WalRecord, now_ms: u64) {
    let mut records = db_wal_log::WalRecordBatch::new();
    assert!(records.push(record).is_ok());
    wal.submit(storage, &[], &records, DurabilityClass::Fsync, now_ms).await.unwrap();
    while records.close_step().unwrap() {}
}

async fn command_record(envelope: &protocol::MutationEnvelope) -> WalRecord {
    let mut control = db_wal_log::WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
    let bytes = db_wal_log::WalBytes::try_admit(encode_command_envelope(envelope).await, 1024 * 1024, &mut control).await.unwrap();
    WalRecord::Command(bytes)
}

/// 🧸️ Creates `document`'s WAL in `storage` and submits `count` sample commands
/// (ids `"op-0".."op-{count-1}"`), each `Fsync`-durable so replay sees them immediately.
async fn seed_wal(storage: &MemoryStorage, document: &ArtifactId, count: u64) {
    let mut wal = db_actor::block_on(ArtifactWal::create(storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
    for i in 0..count {
        let envelope = sample_envelope(&format!("op-{i}"), i).await;
        submit_record(storage, &mut wal, command_record(&envelope).await, i).await;
    }
    wal.close().await.unwrap();
}

/// 🧸️ Reopens `document`'s WAL and appends one `SnapshotPub` marker covering `frontier`.
async fn publish_snapshot_marker(storage: &MemoryStorage, document: &ArtifactId, generation: u64, frontier: Frontier) {
    let (mut wal, _report) = db_actor::block_on(ArtifactWal::open(storage, document.clone(), GroupCommitPolicy::default(), 1000)).unwrap();
    submit_record(storage, &mut wal, WalRecord::SnapshotPub { generation, frontier }, 1000).await;
    wal.close().await.unwrap();
}
//#endregion 🧸️Fixtures
