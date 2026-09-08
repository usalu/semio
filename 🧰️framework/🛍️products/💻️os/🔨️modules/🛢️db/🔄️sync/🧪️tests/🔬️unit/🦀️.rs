
use super::*;

#[semio_framework_async_macros::async_test]
async fn sync_retained_reads_resume_neutral_varints_without_renewing_overall_deadline() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../📝️wal/🧪️fixtures/📖️retained-decoder/🔣️.json")).unwrap();
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let expired = std::sync::atomic::AtomicBool::new(false);
    for row in fixture["varints"].as_array().unwrap().iter().filter(|row| !row["value"].is_null()) {
        let hex = row["hex"].as_str().unwrap();
        let input: Vec<_> = (0..hex.len()).step_by(2).map(|offset| u8::from_str_radix(&hex[offset..offset + 2], 16).unwrap()).collect();
        let mut control = db_wal::WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
        let mut bytes = db_wal::WalBytes::try_admit(input, 16, &mut control).await.unwrap();
        let mut cursor = bytes.cursor();
        control.replenish(std::time::Instant::now(), 1).unwrap();
        let mut attempts = 0;
        let actual = database_sync_hello_read(&mut control, &cancelled, &expired, |control| {
            attempts += 1;
            cursor.varint(control)
        })
        .await
        .unwrap();
        assert!(attempts >= 2);
        assert_eq!(actual, row["value"].as_str().unwrap().parse::<u64>().unwrap());
        while bytes.close_step().unwrap().is_some() {}
    }
    let mut control = db_wal::WalCursorControl::new(cancelled.clone(), std::time::Instant::now(), 1).unwrap();
    expired.store(true, std::sync::atomic::Ordering::Release);
    assert!(matches!(database_sync_hello_read(&mut control, &cancelled, &expired, |_| Ok(())).await, Err(DbError::Timeout(_))));
    expired.store(false, std::sync::atomic::Ordering::Release);
    cancelled.store(true, std::sync::atomic::Ordering::Release);
    assert!(matches!(database_sync_hello_read(&mut control, &cancelled, &expired, |_| Ok(())).await, Err(DbError::Closed)));
}

#[semio_framework_async_macros::async_test]
async fn sync_replay_ignores_neutral_aborted_command_snapshot_and_cas() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../📝️wal/🧪️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == "aborted-commands-snapshot-cas-have-no-effects").unwrap();
    let document = ArtifactId::from("committed-sync");
    let storage = db_wal::tests::committed_fixture_storage(row, &document).await;
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
    let mut retained = replay_sync_state_retained(&storage, document, cancelled, expired, &mut ledger, &progress).await.unwrap();
    assert!(retained.commands.is_empty());
    assert_eq!(retained.floor_head_seq, 0);
    assert_eq!(retained.frontier, ordinary.frontier);
    database_sync_hello_retire_vec(&mut retained.commands, &mut ledger).unwrap();
    assert_eq!(ledger.items, 0);
    assert_eq!(ledger.bytes, 0);
}

use ArtifactId;
use db_storage::MemoryStorage;
use db_wal::{ArtifactWal, GroupCommitPolicy, WalRecord};

//#region 🧸️Fixtures
async fn sample_envelope(id: &str, seq: u64) -> protocol::MutationEnvelope {
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(id.to_string()),
        document_id: protocol::ArtifactId("doc-1".to_string()),
        actor: protocol::ActorId("actor-1".to_string()),
        dependencies: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId("diff.v1".to_string()), payload: seq.to_le_bytes().to_vec() },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId("diff.v1".to_string()), payload: Vec::new() },
        timestamp: protocol::HybridLogicalTimestamp::new(1, seq),
    }
}

async fn submit_record(storage: &MemoryStorage, wal: &mut ArtifactWal, record: WalRecord, now_ms: u64) {
    let mut records = db_wal::WalRecordBatch::new();
    assert!(records.push(record).is_ok());
    wal.submit(storage, &records, DurabilityClass::Fsync, now_ms).await.unwrap();
    while records.close_step().unwrap() {}
}

async fn command_record(envelope: &protocol::MutationEnvelope) -> WalRecord {
    let mut control = db_wal::WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
    let bytes = db_wal::WalBytes::try_admit(encode_command_envelope(envelope).await, 1024 * 1024, &mut control).await.unwrap();
    WalRecord::Command(bytes)
}

/// @emoji 🧸️ Creates `document`'s WAL in `storage` and submits `count` sample commands
/// (ids `"op-0".."op-{count-1}"`), each `Fsync`-durable so replay sees them immediately.
async fn seed_wal(storage: &MemoryStorage, document: &ArtifactId, count: u64) {
    let mut wal = db_actor::block_on(ArtifactWal::create(storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
    for i in 0..count {
        let envelope = sample_envelope(&format!("op-{i}"), i).await;
        submit_record(storage, &mut wal, command_record(&envelope).await, i).await;
    }
}

/// @emoji 🧸️ Reopens `document`'s WAL and appends one `SnapshotPub` marker covering `frontier`.
async fn publish_snapshot_marker(storage: &MemoryStorage, document: &ArtifactId, generation: u64, frontier: Frontier) {
    let (mut wal, _report) = db_actor::block_on(ArtifactWal::open(storage, document.clone(), GroupCommitPolicy::default(), 1000)).unwrap();
    submit_record(storage, &mut wal, WalRecord::SnapshotPub { generation, frontier }, 1000).await;
}
//#endregion 🧸️Fixtures

//#region 🔖️Codec
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trips_through_encode_decode() {
    let envelope = sample_envelope("op-1", 7).await;
    let bytes = encode_command_envelope(&envelope).await;
    assert_eq!(decode_command_envelope(&bytes).await.unwrap(), envelope);
}

#[semio_framework_async_macros::async_test]
async fn decode_command_envelope_rejects_malformed_bytes_without_panicking() {
    assert!(matches!(decode_command_envelope(b"not json").await, Err(DbError::Corrupt(_))));
}
//#endregion 🔖️Codec

//#region 🔖️ReplicaState
#[semio_framework_async_macros::async_test]
async fn replay_sync_state_derives_frontier_and_ordered_commands() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 3).await;

    let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();
    assert_eq!(state.frontier.head_seq, 3);
    assert_eq!(state.frontier.commit_seq, 3);
    assert_eq!(state.floor_head_seq, 0);
    assert_eq!(state.commands.len(), 3);
    assert_eq!(state.commands[0].mutation_id.0, "op-0");
    assert_eq!(state.commands[2].mutation_id.0, "op-2");
}

#[semio_framework_async_macros::async_test]
async fn replay_sync_state_on_empty_document_is_genesis() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 0).await;

    let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();
    assert_eq!(state.frontier.head_seq, 0);
    assert_eq!(state.frontier.chain_hash, [0u8; 32]);
    assert!(state.commands.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn replay_sync_state_tracks_the_latest_snapshot_pub_as_the_floor() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 5).await;
    let floor_frontier = Frontier { document: document.clone(), head_seq: 2, commit_seq: 2, chain_hash: [1u8; 32], epoch: 0 };
    publish_snapshot_marker(&storage, &document, 1, floor_frontier).await;

    let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();
    assert_eq!(state.floor_head_seq, 2);
    assert_eq!(state.frontier.head_seq, 5, "the marker itself carries no commands");
}
//#endregion 🔖️ReplicaState

//#region 🔖️Frontier
#[semio_framework_async_macros::async_test]
async fn frontier_delta_reports_the_command_gap_and_rejects_backwards() {
    let document: ArtifactId = "doc-1".into();
    let from = Frontier { document: document.clone(), head_seq: 2, commit_seq: 2, chain_hash: [0u8; 32], epoch: 0 };
    let to = Frontier { document, head_seq: 5, commit_seq: 5, chain_hash: [9u8; 32], epoch: 0 };

    let delta = frontier_delta(&from, &to).await.unwrap();
    assert_eq!(delta.commands, 3);
    assert!(!delta.is_empty().await);
    assert!(frontier_delta(&to, &from).await.is_err(), "a delta only ever moves forward");
}

#[semio_framework_async_macros::async_test]
async fn frontier_summary_bridges_round_trip() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 2).await;
    let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();

    let summary = state_frontier_summary(&state).await;
    assert_eq!(summary.head_edit_ordinal, 2);
    assert_eq!(summary.head_edit_id, "op-1");
    assert_eq!(summary.last_commit_seq, state.frontier.commit_seq);
    assert_eq!(summary.chain_hash, state.frontier.chain_hash);

    let bridged_back = from_frontier_summary(&summary);
    assert_eq!(bridged_back.head_seq, state.frontier.head_seq);
    assert_eq!(bridged_back.commit_seq, state.frontier.commit_seq);
    assert_eq!(bridged_back.chain_hash, state.frontier.chain_hash);
    assert_eq!(bridged_back.document, state.frontier.document);
}
//#endregion 🔖️Frontier

//#region 🔖️MissingCommands
#[semio_framework_async_macros::async_test]
async fn missing_commands_transfer_round_trip_from_genesis() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 4).await;
    let state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();

    let replica_frontier = Frontier::genesis(document);
    let missing = missing_commands(&state, &replica_frontier).await.unwrap();
    assert_eq!(missing, state.commands, "a genesis replica is missing every command");

    // "Applying" the transfer catches the replica up to the server's frontier exactly.
    let caught_up = state.frontier.clone();
    assert!(missing_commands(&state, &caught_up).await.unwrap().is_empty());
}

#[semio_framework_async_macros::async_test]
async fn missing_commands_transfer_round_trip_for_a_partially_caught_up_replica() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 3).await;
    let first_state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();
    let replica_frontier = first_state.frontier;

    // More commands land on the server after the replica already caught up once.
    {
        let (mut wal, _report) = db_actor::block_on(ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 100)).unwrap();
        for i in 3..6u64 {
            let envelope = sample_envelope(&format!("op-{i}"), i).await;
            submit_record(&storage, &mut wal, command_record(&envelope).await, i).await;
        }
    }

    let second_state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();
    let missing = missing_commands(&second_state, &replica_frontier).await.unwrap();
    assert_eq!(missing.len(), 3);
    assert_eq!(missing[0].mutation_id.0, "op-3");
    assert_eq!(missing[2].mutation_id.0, "op-5");
}

#[semio_framework_async_macros::async_test]
async fn missing_commands_rejects_document_mismatch_and_a_replica_ahead_of_server() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 2).await;
    let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();

    let other_document = Frontier::genesis("doc-2".into());
    assert!(matches!(missing_commands(&state, &other_document).await, Err(DbError::InvalidArgument(_))));

    let ahead = Frontier { head_seq: 99, ..state.frontier.clone() };
    assert!(matches!(missing_commands(&state, &ahead).await, Err(DbError::InvalidArgument(_))));
}

#[semio_framework_async_macros::async_test]
async fn missing_commands_rejects_a_replica_behind_the_retained_floor() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 5).await;
    let floor_frontier = Frontier { document: document.clone(), head_seq: 3, commit_seq: 3, chain_hash: [2u8; 32], epoch: 0 };
    publish_snapshot_marker(&storage, &document, 1, floor_frontier).await;
    let state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();

    let too_far_behind = Frontier { document, head_seq: 1, commit_seq: 1, chain_hash: [0u8; 32], epoch: 0 };
    assert!(matches!(missing_commands(&state, &too_far_behind).await, Err(DbError::Unavailable(_))));
}
//#endregion 🔖️MissingCommands

//#region 🔖️Bootstrap
#[semio_framework_async_macros::async_test]
async fn decide_bootstrap_serves_tail_for_a_fresh_replica_within_the_floor() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 3).await;
    let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();

    let plan = db_actor::block_on(decide_bootstrap(&state, &storage, None)).unwrap();
    assert_eq!(plan, BootstrapPlan::Tail { envelopes: state.commands });
}

#[semio_framework_async_macros::async_test]
async fn decide_bootstrap_reports_none_for_an_already_caught_up_replica() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 3).await;
    let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();

    let plan = db_actor::block_on(decide_bootstrap(&state, &storage, Some(&state.frontier))).unwrap();
    assert_eq!(plan, BootstrapPlan::None);
}

#[semio_framework_async_macros::async_test]
async fn decide_bootstrap_serves_snapshot_when_a_generation_is_available_below_the_floor() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 5).await;
    let floor_frontier = Frontier { document: document.clone(), head_seq: 4, commit_seq: 4, chain_hash: [3u8; 32], epoch: 0 };
    publish_snapshot_marker(&storage, &document, 7, floor_frontier).await;
    let pages = db_storage::db_io_copy_pages(b"snapshot-bytes").unwrap().await.unwrap();
    SnapshotStorage::write_generation(&storage, &document, 7, pages).await.unwrap();
    let state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();

    let stale_replica = Frontier { document, head_seq: 0, commit_seq: 0, chain_hash: [0u8; 32], epoch: 0 };
    let plan = db_actor::block_on(decide_bootstrap(&state, &storage, Some(&stale_replica))).unwrap();
    match plan {
        BootstrapPlan::Snapshot { generation, pages, pack_hash } => {
            assert_eq!(generation, 7);
            assert_eq!(pages, b"snapshot-bytes");
            assert_eq!(pack_hash, *semio_framework_hash::hash(b"snapshot-bytes").as_bytes());
        }
        other => panic!("expected a Snapshot plan, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn decide_bootstrap_reports_unavailable_when_below_floor_with_no_snapshot() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 5).await;
    let floor_frontier = Frontier { document: document.clone(), head_seq: 4, commit_seq: 4, chain_hash: [3u8; 32], epoch: 0 };
    publish_snapshot_marker(&storage, &document, 7, floor_frontier).await;
    let state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();

    let stale_replica = Frontier { document, head_seq: 0, commit_seq: 0, chain_hash: [0u8; 32], epoch: 0 };
    assert!(matches!(db_actor::block_on(decide_bootstrap(&state, &storage, Some(&stale_replica))), Err(DbError::Unavailable(_))));
}
//#endregion 🔖️Bootstrap

//#region 🔖️ResumeToken
#[semio_framework_async_macros::async_test]
async fn issue_resume_token_produces_the_documented_v1_wire_format() {
    let document: ArtifactId = "doc-1".into();
    let frontier = Frontier { document, head_seq: 4, commit_seq: 4, chain_hash: [5u8; 32], epoch: 0 };
    let token = issue_resume_token(&frontier).await.unwrap();
    assert!(token.starts_with("v1|doc-1|4|4|0|"));
}
//#endregion 🔖️ResumeToken

//#region 🔖️Hello
#[semio_framework_async_macros::async_test]
async fn handle_hello_bootstraps_a_fresh_replica_via_tail_and_issues_a_resume_token() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 3).await;
    let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(storage));

    let response = db_actor::block_on(handle_hello(db_storage::db_io_test_pool(), storage, document, None, "session-1".to_string(), protocol::ActorId("semio_hub".to_string()), 64 * 1024)).unwrap();
    let protocol::ServerFrame::Welcome { bootstrap, server_frontier, resume_token, .. } = &response.welcome else {
        panic!("expected a Welcome frame");
    };
    assert_eq!(*bootstrap, protocol::Bootstrap::Tail);
    assert_eq!(server_frontier.head_edit_ordinal, 3);
    assert!(!resume_token.is_empty());
    assert_eq!(response.follow_up.len(), 1);
    match &response.follow_up[0] {
        protocol::ServerFrame::Commands { envelopes, .. } => assert_eq!(envelopes.len(), 3),
        other => panic!("expected a Commands follow-up frame, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn handle_hello_reports_no_follow_up_for_an_already_caught_up_replica() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 2).await;
    let state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();
    let hello_frontier = state_frontier_summary(&state).await;
    let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(storage));

    let response = db_actor::block_on(handle_hello(db_storage::db_io_test_pool(), storage, document, Some(hello_frontier), "session-2".to_string(), protocol::ActorId("semio_hub".to_string()), 64 * 1024)).unwrap();
    let protocol::ServerFrame::Welcome { bootstrap, .. } = &response.welcome else {
        panic!("expected a Welcome frame");
    };
    assert_eq!(*bootstrap, protocol::Bootstrap::None);
    assert!(response.follow_up.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn handle_hello_chunks_a_snapshot_larger_than_the_requested_chunk_size() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 4).await;
    let floor_frontier = Frontier { document: document.clone(), head_seq: 4, commit_seq: 4, chain_hash: [1u8; 32], epoch: 0 };
    publish_snapshot_marker(&storage, &document, 9, floor_frontier).await;
    let big_snapshot = vec![7u8; 10];
    let pages = db_storage::db_io_copy_pages(&big_snapshot).unwrap().await.unwrap();
    SnapshotStorage::write_generation(&storage, &document, 9, pages).await.unwrap();
    let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(storage));

    let stale_hello_frontier = protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(document.0.clone()), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: [0u8; 32] };
    let response = db_actor::block_on(handle_hello(db_storage::db_io_test_pool(), storage, document, Some(stale_hello_frontier), "session-3".to_string(), protocol::ActorId("semio_hub".to_string()), 4)).unwrap();

    let protocol::ServerFrame::Welcome { bootstrap, .. } = &response.welcome else {
        panic!("expected a Welcome frame");
    };
    assert!(matches!(bootstrap, protocol::Bootstrap::Snapshot { inline: None, .. }));
    // 10 bytes chunked at 4 bytes/chunk -> 3 chunks (4, 4, 2), plus one SnapshotDone.
    assert_eq!(response.follow_up.len(), 4);
    assert!(matches!(response.follow_up[3], protocol::ServerFrame::SnapshotDone { seq_count: 3 }));
}

#[semio_framework_async_macros::async_test]
async fn handle_hello_rejects_zero_snapshot_chunk_bytes() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 1).await;
    let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(storage));
    assert!(matches!(db_actor::block_on(handle_hello(db_storage::db_io_test_pool(), storage, document, None, "s".to_string(), protocol::ActorId("semio_hub".to_string()), 0)), Err(DbError::InvalidArgument(_))));
}

#[semio_framework_async_macros::async_test]
async fn handle_frontier_advertise_relays_missing_commands_and_none_when_caught_up() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 2).await;
    let first_state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();
    let replica_summary = state_frontier_summary(&first_state).await;

    {
        let (mut wal, _report) = db_actor::block_on(ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 100)).unwrap();
        let envelope = sample_envelope("op-2", 2).await;
        submit_record(&storage, &mut wal, command_record(&envelope).await, 100).await;
    }

    let frame = db_actor::block_on(handle_frontier_advertise(&storage, document.clone(), &replica_summary, protocol::ActorId("semio_hub".to_string()))).unwrap();
    match frame {
        Some(protocol::ServerFrame::Commands { envelopes, .. }) => {
            assert_eq!(envelopes.len(), 1);
            assert_eq!(envelopes[0].mutation_id.0, "op-2");
        }
        other => panic!("expected a Commands frame, got {other:?}"),
    }

    let up_to_date_state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();
    let up_to_date_summary = state_frontier_summary(&up_to_date_state).await;
    assert!(db_actor::block_on(handle_frontier_advertise(&storage, document, &up_to_date_summary, protocol::ActorId("semio_hub".to_string()))).unwrap().is_none());
}

fn held_sync_hello_pool() -> (std::sync::Arc<semio_framework_async::WorkerPool>, std::sync::Arc<std::sync::atomic::AtomicBool>) {
    let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let entered = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let held = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let worker_entered = entered.clone();
    let worker_held = held.clone();
    pool.try_submit(
        semio_framework_async::Lane::Maintenance,
        Box::new(move || {
            worker_entered.store(true, std::sync::atomic::Ordering::Release);
            while worker_held.load(std::sync::atomic::Ordering::Acquire) {
                std::thread::yield_now();
            }
        }),
    )
    .ok()
    .expect("sync hello blocker admission");
    while !entered.load(std::sync::atomic::Ordering::Acquire) {
        std::thread::yield_now();
    }
    (pool, held)
}

fn replenishing_sync_hello_io_job(pool: std::sync::Arc<semio_framework_async::WorkerPool>, active: std::sync::Arc<std::sync::atomic::AtomicBool>) -> semio_framework_async::Job {
    Box::new(move || {
        if active.load(std::sync::atomic::Ordering::Acquire) {
            let next = replenishing_sync_hello_io_job(pool.clone(), active.clone());
            if let Err(error) = pool.try_submit(semio_framework_async::Lane::Io, next) {
                drop(error.into_job());
            }
        }
    })
}

fn replenishing_held_sync_hello_pool() -> (std::sync::Arc<semio_framework_async::WorkerPool>, std::sync::Arc<std::sync::atomic::AtomicBool>, std::sync::Arc<std::sync::atomic::AtomicBool>) {
    let (pool, held) = held_sync_hello_pool();
    let active = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    loop {
        let job = replenishing_sync_hello_io_job(pool.clone(), active.clone());
        if let Err(error) = pool.try_submit(semio_framework_async::Lane::Io, job) {
            drop(error.into_job());
            break;
        }
    }
    (pool, held, active)
}

async fn retained_sync_hello_storage() -> std::sync::Arc<db_storage::DbBackend> {
    std::sync::Arc::new(db_storage::DbBackend::Memory(MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()))
}

#[semio_framework_async_macros::async_test]
async fn retained_sync_hello_handoff_first_poll_cancel_preserves_exact_owner_and_io_lane() {
    let (pool, held) = held_sync_hello_pool();
    let storage = retained_sync_hello_storage().await;
    let storage_identity = std::sync::Arc::as_ptr(&storage);
    let document = ArtifactId(String::from("p1z-handoff-cancel"));
    let document_identity = document.0.as_ptr();
    let future = DatabaseSyncHelloFuture::try_submit(pool, storage, document, None, String::from("p1z-session"), protocol::ActorId(String::from("p1z-origin")), 4096).unwrap();
    let state = future.state.as_ref().unwrap().clone();
    assert_eq!(state.driver.load(std::sync::atomic::Ordering::Acquire), DatabaseSyncHelloDriverAuthority::Queued as u8);
    future.cancel();
    held.store(false, std::sync::atomic::Ordering::Release);
    let result = future.await.unwrap();
    let core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owners = core.execution.as_ref().unwrap().owners.as_ref().unwrap();
    assert_eq!(std::sync::Arc::as_ptr(owners.storage.as_ref().unwrap()), storage_identity);
    assert_eq!(owners.document.0.as_ptr(), document_identity);
    assert!(matches!(core.execution.as_ref().unwrap().prepared, Err(DbError::Closed)));
}

#[semio_framework_async_macros::async_test]
async fn retained_sync_hello_max_plus_one_refusal_keeps_storage_document_frontier_session_origin_identity() {
    let (pool, held) = held_sync_hello_pool();
    let storage = retained_sync_hello_storage().await;
    let mut futures = Vec::new();
    for index in 0..DATABASE_SYNC_HELLO_SLOTS {
        futures.push(DatabaseSyncHelloFuture::try_submit(pool.clone(), storage.clone(), ArtifactId(format!("p1z-max-{index}")), None, format!("session-{index}"), protocol::ActorId(format!("origin-{index}")), 4096).unwrap());
    }
    let document = ArtifactId(String::from("p1z-max-plus-one"));
    let identity = document.0.as_ptr();
    let rejected = DatabaseSyncHelloFuture::try_submit(pool, storage, document, None, String::from("retained-session"), protocol::ActorId(String::from("retained-origin")), 4096).unwrap_err();
    let owners = rejected.close.owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    assert_eq!(owners.as_ref().unwrap().document.0.as_ptr(), identity);
    assert_eq!(owners.as_ref().unwrap().session_id, "retained-session");
    assert_eq!(owners.as_ref().unwrap().origin.0, "retained-origin");
    drop(owners);
    held.store(false, std::sync::atomic::Ordering::Release);
    for future in futures {
        future.cancel();
    }
}

#[semio_framework_async_macros::async_test]
async fn retained_sync_hello_tail_stream_publishes_welcome_then_one_backpressured_frame() {
    let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let memory = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = ArtifactId(String::from("p1z-tail-stream"));
    seed_wal(&memory, &document, 3).await;
    let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(memory));
    let result = DatabaseSyncHelloFuture::try_submit(pool, storage, document, None, String::from("p1z-tail"), protocol::ActorId(String::from("p1z-origin")), 4096).unwrap().await.unwrap();
    let mut session = result.close_and_take_session().unwrap();
    let welcome = session.take_welcome().unwrap();
    assert!(matches!(welcome.frame().unwrap(), protocol::ServerFrame::Welcome { bootstrap: protocol::Bootstrap::Tail, .. }));
    welcome.acknowledge().unwrap();
    let frame = session.next_frame().await.unwrap().unwrap();
    assert!(matches!(frame.frame().unwrap(), protocol::ServerFrame::Commands { envelopes, .. } if envelopes.len() == 3));
    frame.acknowledge().unwrap();
    assert!(session.next_frame().await.unwrap().is_none());
}

#[semio_framework_async_macros::async_test]
async fn retained_sync_hello_snapshot_cursor_copies_at_most_one_page_fragment_per_driver_opportunity() {
    let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let memory = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = ArtifactId(String::from("p1z-snapshot-stream"));
    seed_wal(&memory, &document, 4).await;
    let floor = Frontier { document: document.clone(), head_seq: 4, commit_seq: 4, chain_hash: [1; 32], epoch: 0 };
    publish_snapshot_marker(&memory, &document, 9, floor).await;
    let pages = db_storage::db_io_copy_pages(b"0123456789").unwrap().await.unwrap();
    SnapshotStorage::write_generation(&memory, &document, 9, pages).await.unwrap();
    let stale = protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(document.0.clone()), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: [0; 32] };
    let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(memory));
    let result = DatabaseSyncHelloFuture::try_submit(pool, storage, document, Some(stale), String::from("p1z-snapshot"), protocol::ActorId(String::from("p1z-origin")), 4).unwrap().await.unwrap();
    let mut session = result.close_and_take_session().unwrap();
    let welcome = session.take_welcome().unwrap();
    assert!(matches!(welcome.frame().unwrap(), protocol::ServerFrame::Welcome { bootstrap: protocol::Bootstrap::Snapshot { inline: None, .. }, .. }));
    welcome.acknowledge().unwrap();
    for (seq, bytes) in [(0, b"0123".as_slice()), (1, b"4567".as_slice()), (2, b"89".as_slice())] {
        let frame = session.next_frame().await.unwrap().unwrap();
        assert!(matches!(frame.frame().unwrap(), protocol::ServerFrame::SnapshotChunk { seq: actual, bytes: actual_bytes } if *actual == seq && actual_bytes.as_slice() == bytes));
        frame.acknowledge().unwrap();
    }
    let done = session.next_frame().await.unwrap().unwrap();
    assert!(matches!(done.frame().unwrap(), protocol::ServerFrame::SnapshotDone { seq_count: 3 }));
    done.acknowledge().unwrap();
    assert!(session.next_frame().await.unwrap().is_none());
}

#[semio_framework_async_macros::async_test]
async fn retained_sync_hello_returned_snapshot_credit_waits_for_exact_generation_ack() {
    let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let memory = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = ArtifactId(String::from("p1z-returned-credit"));
    seed_wal(&memory, &document, 2).await;
    let floor = Frontier { document: document.clone(), head_seq: 2, commit_seq: 2, chain_hash: [2; 32], epoch: 0 };
    publish_snapshot_marker(&memory, &document, 7, floor).await;
    let pages = db_storage::db_io_copy_pages(b"abcdefgh").unwrap().await.unwrap();
    SnapshotStorage::write_generation(&memory, &document, 7, pages).await.unwrap();
    let stale = protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(document.0.clone()), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: [0; 32] };
    let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(memory));
    let result = DatabaseSyncHelloFuture::try_submit(pool, storage, document, Some(stale), String::from("p1z-returned-credit-session"), protocol::ActorId(String::from("p1z-returned-credit-origin")), 4).unwrap().await.unwrap();
    let mut session = result.close_and_take_session().unwrap();
    session.take_welcome().unwrap().acknowledge().unwrap();
    let first = session.next_frame().await.unwrap().unwrap();
    let state = session.state.as_ref().unwrap().clone();
    let (generation, items, bytes, ledger_before) = {
        let core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let lease = core.returned_frame.as_ref().unwrap();
        let ledger = core.execution.as_ref().unwrap().ledger.as_ref().unwrap();
        (lease.generation, lease.items, lease.bytes, (ledger.items, ledger.bytes))
    };
    assert!(generation != 0 && items == 1 && bytes >= 4);
    let mut next = Box::pin(session.next_frame());
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    assert!(matches!(std::future::Future::poll(next.as_mut(), &mut context), std::task::Poll::Pending));
    {
        let core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let lease = core.returned_frame.as_ref().unwrap();
        let ledger = core.execution.as_ref().unwrap().ledger.as_ref().unwrap();
        assert_eq!(lease.generation, generation);
        assert_eq!((ledger.items, ledger.bytes), ledger_before);
        assert!(core.frame.is_none());
    }
    first.acknowledge().unwrap();
    let second = next.await.unwrap().unwrap();
    assert!(matches!(second.frame().unwrap(), protocol::ServerFrame::SnapshotChunk { seq: 1, .. }));
    second.acknowledge().unwrap();
}

#[semio_framework_async_macros::async_test]
async fn retained_sync_hello_maximum_snapshot_request_stays_page_unit_bounded() {
    let mut page_ledger = DatabaseSyncHelloBackingLedger::default();
    let page_reservation = database_sync_hello_reserve_snapshot_pages(&mut page_ledger).unwrap();
    assert_eq!((page_ledger.items, page_ledger.bytes), (DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_ITEMS, DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_BYTES));
    let mut page_owner = db_storage::db_io_copy_pages(b"fixed-page-owner").unwrap().await.unwrap();
    let (page_items, page_bytes) = page_reservation.observed(&page_owner).unwrap();
    assert_eq!((page_items, page_bytes), (1, db_storage::DB_IO_PAGE_BYTES));
    page_reservation.settle(&mut page_ledger, page_items, page_bytes).unwrap();
    while page_owner.close_step().unwrap().is_some() {}
    page_ledger.release(page_items, page_bytes).unwrap();
    assert!(page_owner.terminal_is_empty() && page_ledger.terminal_is_empty());

    let mut fixed_ledger = DatabaseSyncHelloBackingLedger::default();
    let fixed_reservation = database_sync_hello_reserve_snapshot_chunk(&mut fixed_ledger, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES).unwrap();
    assert_eq!((fixed_ledger.items, fixed_ledger.bytes), (1, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES));
    assert_eq!(fixed_reservation.bytes, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES);
    let fixed = fixed_reservation.allocate(&mut fixed_ledger).unwrap();
    assert_eq!(fixed.backing_bytes(), DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES);
    let fixed_frame = protocol::ServerFrame::SnapshotChunk { seq: 0, bytes: fixed };
    let (fixed_items, fixed_bytes) = database_sync_hello_returned_frame_credit(&fixed_frame).unwrap();
    let mut fixed_lease = DatabaseSyncHelloReturnedFrameLease { generation: 1, items: fixed_items, bytes: fixed_bytes, close: Some(DatabaseSyncHelloFrameClose { owner: Some(fixed_frame), envelope: None }) };
    assert_eq!((fixed_lease.items, fixed_lease.bytes), (1, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES));
    assert!(fixed_lease.close.as_mut().unwrap().close_one());
    assert!(fixed_lease.close.as_mut().unwrap().close_one());
    assert!(fixed_lease.close.as_ref().unwrap().terminal_is_empty());
    fixed_ledger.release(fixed_lease.items, fixed_lease.bytes).unwrap();
    assert!(fixed_ledger.terminal_is_empty());

    let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let memory = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = ArtifactId(String::from("p1z-maximum-frame-unit"));
    seed_wal(&memory, &document, 2).await;
    let floor = Frontier { document: document.clone(), head_seq: 2, commit_seq: 2, chain_hash: [3; 32], epoch: 0 };
    publish_snapshot_marker(&memory, &document, 11, floor).await;
    let source = vec![b'x'; DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES * 2 + 1];
    let pages = db_storage::db_io_copy_pages(&source).unwrap().await.unwrap();
    SnapshotStorage::write_generation(&memory, &document, 11, pages).await.unwrap();
    let stale = protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(document.0.clone()), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: [0; 32] };
    let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(memory));
    let result = DatabaseSyncHelloFuture::try_submit(pool, storage, document, Some(stale), String::from("p1z-maximum-frame-session"), protocol::ActorId(String::from("p1z-maximum-frame-origin")), DATABASE_SYNC_HELLO_MAX_BYTES).unwrap().await.unwrap();
    let mut session = result.close_and_take_session().unwrap();
    session.take_welcome().unwrap().acknowledge().unwrap();
    for (seq, expected) in [(0, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES), (1, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES), (2, 1)] {
        let frame = session.next_frame().await.unwrap().unwrap();
        assert!(matches!(frame.frame().unwrap(), protocol::ServerFrame::SnapshotChunk { seq: actual, bytes } if *actual == seq && bytes.len() == expected && bytes.backing_bytes() == DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES));
        frame.acknowledge().unwrap();
    }
    let done = session.next_frame().await.unwrap().unwrap();
    assert!(matches!(done.frame().unwrap(), protocol::ServerFrame::SnapshotDone { seq_count: 3 }));
    done.acknowledge().unwrap();
    assert!(session.next_frame().await.unwrap().is_none());
}

#[semio_framework_async_macros::async_test]
async fn retained_sync_hello_grant_deadline_between_allocation_copy_and_publication_retains_credit() {
    let cancelled = std::sync::atomic::AtomicBool::new(false);
    let expired = std::sync::atomic::AtomicBool::new(false);
    let document = ArtifactId(String::from("p1z-grant-deadline"));
    let source = vec![b'g'; DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES];
    let pages = db_storage::db_io_copy_pages(&source).unwrap().await.unwrap();
    let mut follow_up = DatabaseSyncHelloFollowUp::Snapshot { pages, chunk_bytes: DATABASE_SYNC_HELLO_MAX_BYTES, offset: 0, page: 0, page_offset: 0, seq: 0, chunk: None, done: false };
    let mut ledger = DatabaseSyncHelloBackingLedger::default();
    let mut before_copy = DatabaseSyncHelloGrant::expiring_at(3);
    assert!(matches!(follow_up.drive_one_with_grant(&mut ledger, &cancelled, &expired, &mut before_copy), Err(DbError::Timeout(ref message)) if message == "database sync hello 8 ms grant"));
    assert!(matches!(&follow_up, DatabaseSyncHelloFollowUp::Snapshot { chunk: Some(chunk), offset: 0, seq: 0, .. } if chunk.is_empty() && chunk.backing_bytes() == DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES));
    assert_eq!(ledger.items, 1);
    assert!(ledger.bytes <= DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES);
    while follow_up.close_one().unwrap() {}
    while ledger.close_one_credit() {}
    assert!(follow_up.terminal_is_empty() && ledger.terminal_is_empty());

    let pages = db_storage::db_io_copy_pages(&source).unwrap().await.unwrap();
    let mut follow_up = DatabaseSyncHelloFollowUp::Snapshot { pages, chunk_bytes: DATABASE_SYNC_HELLO_MAX_BYTES, offset: 0, page: 0, page_offset: 0, seq: 0, chunk: None, done: false };
    let mut ledger = DatabaseSyncHelloBackingLedger::default();
    let mut before_publication = DatabaseSyncHelloGrant::expiring_at(5);
    assert!(matches!(follow_up.drive_one_with_grant(&mut ledger, &cancelled, &expired, &mut before_publication), Err(DbError::Timeout(ref message)) if message == "database sync hello 8 ms grant"));
    assert!(matches!(&follow_up, DatabaseSyncHelloFollowUp::Snapshot { chunk: Some(chunk), offset, seq: 0, .. } if chunk.len() == DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES && *offset == DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES));
    assert_eq!(ledger.items, 1);
    assert!(ledger.bytes <= DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES);
    while follow_up.close_one().unwrap() {}
    while ledger.close_one_credit() {}
    assert!(follow_up.terminal_is_empty() && ledger.terminal_is_empty());

    let independently_expired = std::sync::atomic::AtomicBool::new(true);
    assert!(matches!(database_sync_hello_control(&cancelled, &independently_expired), Err(DbError::Timeout(ref message)) if message == "database sync hello deadline"));
    assert_eq!(document.0, "p1z-grant-deadline");
}

#[semio_framework_async_macros::async_test]
async fn retained_sync_hello_cancel_before_stream_demand_publishes_no_new_frame() {
    let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let memory = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = ArtifactId(String::from("p1z-cancel-before-stream"));
    seed_wal(&memory, &document, 2).await;
    let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(memory));
    let result = DatabaseSyncHelloFuture::try_submit(pool, storage, document, None, String::from("p1z-cancel-stream-session"), protocol::ActorId(String::from("p1z-cancel-stream-origin")), 4_096).unwrap().await.unwrap();
    let mut session = result.close_and_take_session().unwrap();
    session.take_welcome().unwrap().acknowledge().unwrap();
    let state = session.state.as_ref().unwrap().clone();
    session.cancel();
    assert!(matches!(session.next_frame().await, Err(DbError::Closed)));
    assert!(!state.demand.load(std::sync::atomic::Ordering::Acquire));
    let core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    assert!(!matches!(core.frame.as_ref(), Some(Ok(Some(_)))));
}

#[test]
fn retained_sync_hello_cumulative_actual_backing_rejects_max_plus_one_without_mutation() {
    let mut ledger = DatabaseSyncHelloBackingLedger::default();
    ledger.observe(DATABASE_SYNC_HELLO_MAX_ITEMS, DATABASE_SYNC_HELLO_MAX_BYTES, "maximum").unwrap();
    let before = (ledger.items, ledger.bytes);
    assert!(ledger.observe(1, 0, "cumulative max plus one").is_err());
    assert_eq!((ledger.items, ledger.bytes), before);
    assert!(ledger.observe(0, 1, "cumulative max plus one").is_err());
    assert_eq!((ledger.items, ledger.bytes), before);
}

#[test]
fn retained_sync_hello_predebits_envelope_clone_and_overallocation_before_owner_construction() {
    let mut ledger = DatabaseSyncHelloBackingLedger::default();
    let mut bytes = database_sync_hello_allocate_envelope_vec::<u8>(&mut ledger, 4_096).unwrap();
    assert_eq!(ledger.items, 1);
    assert_eq!(ledger.bytes, bytes.capacity());
    database_sync_hello_retire_vec(&mut bytes, &mut ledger).unwrap();
    let source = String::from("p1z-predebit");
    let owner = database_sync_hello_clone_string(&source, &mut ledger, "database sync hello law clone backing").unwrap();
    assert_ne!(source.as_ptr(), owner.as_ptr());
    assert_eq!(ledger.bytes, owner.capacity());
    let before = (ledger.items, ledger.bytes);
    assert!(database_sync_hello_allocate_envelope_vec::<u8>(&mut ledger, DATABASE_SYNC_HELLO_MAX_BYTES).is_err());
    assert_eq!((ledger.items, ledger.bytes), before);
}

#[test]
fn retained_sync_hello_cancel_between_yield_and_resume_prevents_next_wal_backend_operation() {
    let cancelled = std::sync::atomic::AtomicBool::new(false);
    let expired = std::sync::atomic::AtomicBool::new(false);
    let mut opportunity = std::pin::pin!(database_sync_hello_opportunity(&cancelled, &expired));
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    assert!(matches!(std::future::Future::poll(opportunity.as_mut(), &mut context), std::task::Poll::Pending));
    cancelled.store(true, std::sync::atomic::Ordering::Release);
    assert!(matches!(std::future::Future::poll(opportunity.as_mut(), &mut context), std::task::Poll::Ready(Err(DbError::Closed))));
    assert!(database_sync_hello_control(&cancelled, &expired).is_err());
}

#[test]
fn retained_sync_hello_quarantine_cursor_and_byte_item_ledger_reach_zero_before_release() {
    let future: DatabaseSyncHelloExecutionFuture = Box::pin(std::future::pending());
    let bytes = size_of_val(&*future);
    let mut quarantine = DatabaseSyncHelloQuarantineClose { future: Some(future), items: 1, bytes };
    assert!(quarantine.close_one());
    assert!(quarantine.terminal_is_empty());
    let mut ledger = DatabaseSyncHelloBackingLedger::default();
    ledger.observe(2, db_storage::DB_IO_PAGE_BYTES + 1, "database sync hello quarantine law backing").unwrap();
    assert!(ledger.close_one_credit());
    assert_eq!(ledger.bytes, 1);
    while ledger.close_one_credit() {}
    assert!(ledger.terminal_is_empty());
}

#[test]
fn retained_sync_hello_page_close_error_is_typed_and_blocks_terminal_release() {
    let mut fault = None;
    let pending = database_sync_hello_apply_follow_up_close_result(&mut fault, false, Err(DbError::Internal("p1z retained page-close fault".to_string())));
    assert!(pending);
    let retained = fault.as_ref().unwrap();
    assert_eq!(retained.attempts, 1);
    assert!(matches!(retained.error.as_ref(), Some(DbError::Internal(detail)) if detail == "p1z retained page-close fault"));
    assert!(!retained.terminal_is_empty());
    assert!(fault.as_mut().unwrap().close_one());
    assert!(fault.as_ref().unwrap().terminal_is_empty());
}

#[test]
fn retained_sync_hello_refusal_retry_saturation_is_bounded_and_retains_terminal_job_owner() {
    let (pool, held, active) = replenishing_held_sync_hello_pool();
    let rejected = DatabaseSyncHelloRejected::new(
        pool,
        DbError::Unavailable("p1z refusal saturation law".to_string()),
        DatabaseSyncHelloOwners {
            storage: None,
            document: ArtifactId(String::from("p1z-refusal-bounded")),
            hello_frontier: None,
            session_id: String::from("p1z-refusal-session"),
            origin: protocol::ActorId(String::from("p1z-refusal-origin")),
            snapshot_chunk_bytes: 4_096,
        },
    );
    let close = rejected.close.clone();
    let generation = rejected.retirement_generation();
    assert!(matches!(rejected.close_and_take_error(), DbError::Unavailable(_)));
    let witness = database_sync_hello_rejected_terminal_witness(generation).unwrap();
    assert!(witness.owners_retained && witness.job_retained && witness.submissions == 1);
    assert_eq!(close.owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().unwrap().document.0, "p1z-refusal-bounded");
    assert!(close.submissions.load(std::sync::atomic::Ordering::Acquire) <= DATABASE_SYNC_HELLO_RETRY_LIMIT);
    active.store(false, std::sync::atomic::Ordering::Release);
    held.store(false, std::sync::atomic::Ordering::Release);
}

#[test]
fn retained_sync_hello_forever_stuck_sole_worker_guarantees_discoverable_ownership_only() {
    let (pool, held) = held_sync_hello_pool();
    loop {
        if let Err(error) = pool.try_submit(semio_framework_async::Lane::Io, Box::new(|| {})) {
            drop(error.into_job());
            break;
        }
    }
    let rejected = DatabaseSyncHelloRejected::new(
        pool,
        DbError::Unavailable("p1z forever-stuck sole worker".to_string()),
        DatabaseSyncHelloOwners {
            storage: None,
            document: ArtifactId(String::from("p1z-discoverable-only")),
            hello_frontier: None,
            session_id: String::from("p1z-discoverable-session"),
            origin: protocol::ActorId(String::from("p1z-discoverable-origin")),
            snapshot_chunk_bytes: 4_096,
        },
    );
    let generation = rejected.retirement_generation();
    rejected.close_and_take_error();
    let witness = database_sync_hello_rejected_terminal_witness(generation).unwrap();
    assert!(witness.owners_retained && witness.job_retained);
    assert!(witness.submissions <= DATABASE_SYNC_HELLO_RETRY_LIMIT);
    held.store(false, std::sync::atomic::Ordering::Release);
}

#[semio_framework_async_macros::async_test]
async fn retained_sync_hello_deadline_retry_drop_close_retains_registry_until_worker_service() {
    let (pool, held) = held_sync_hello_pool();
    let storage = retained_sync_hello_storage().await;
    let future = DatabaseSyncHelloFuture::try_submit(pool, storage, ArtifactId(String::from("p1z-deadline")), None, String::from("deadline-session"), protocol::ActorId(String::from("deadline-origin")), 4096).unwrap();
    let state = future.state.as_ref().unwrap().clone();
    state.deadline_callback();
    drop(future);
    assert!(state.expired.load(std::sync::atomic::Ordering::Acquire));
    assert!(state.cancelled.load(std::sync::atomic::Ordering::Acquire));
    assert!(matches!(database_sync_hello_control(&state.cancelled, &state.expired), Err(DbError::Timeout(ref message)) if message == "database sync hello deadline"));
    assert!(state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
    assert!(database_sync_hello_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[state.slot].is_some());
    held.store(false, std::sync::atomic::Ordering::Release);
}

#[test]
fn retained_sync_hello_ready_pending_panic_and_repeat_poll_have_typed_terminal_states() {
    let source = include_str!("../../🦀️.rs");
    assert!(source.contains("Ok(std::task::Poll::Pending)"));
    assert!(source.contains("Ok(std::task::Poll::Ready(execution))"));
    assert!(source.contains("core.quarantined = Some(DatabaseSyncHelloQuarantineClose"));
    assert!(source.contains("if self.completed"));
    assert!(source.contains("std::task::Poll::Ready(Err(DbError::Closed))"));
}

#[test]
fn retained_sync_hello_production_census_has_zero_blocking_waits_and_no_eager_follow_up() {
    let engine = include_str!("../../../../⚙️engine/🦀️.rs");
    let production = engine.split("//#region 🧪️Tests").next().unwrap();
    assert!(!production.contains("db_actor::block_on(db_sync::handle_hello"));
    assert!(production.contains("DatabaseSyncHelloFuture::try_submit"));
    let source = include_str!("../../🦀️.rs");
    assert!(source.contains("#[cfg(test)]\npub async fn handle_hello"));
    assert!(source.contains("DatabaseSyncHelloNextFuture"));
}
//#endregion 🔖️Hello
