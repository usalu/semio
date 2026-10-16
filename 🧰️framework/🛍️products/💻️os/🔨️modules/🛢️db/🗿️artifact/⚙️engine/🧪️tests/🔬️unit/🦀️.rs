use super::*;

use db_artifact_contract::{CommandBatch, CommandReceipt, DB_PATHMAP_SCHEMA, FOREIGN_HISTORY_TRANSITION_CODE, SubmitOptions, decode_pathmap_json, encode_pathmap, to_core_document_id};
use db_artifact_history::{HistoryEnvelopeCursor, HistoryEnvelopeField, HistoryFrontierCursor, HistoryFrontierField, HistoryReplayPhase, HistoryReplayReservation, HistoryReplayTransition, history_capacity_test_lock};
use std::sync::Arc as StdArc;

async fn rejected_engine_open_error(mut rejected: ArtifactEngineOpenRejected) -> DbError {
    loop {
        match rejected.retry_close().await {
            Ok(cause) => return cause,
            Err(retained) => rejected = retained,
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_open_ignores_neutral_aborted_command_snapshot_and_cas() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📝️wal/🧪️tests/🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == "aborted-commands-snapshot-cas-have-no-effects").unwrap();
    let document = ArtifactId::from("committed-artifact");
    let memory = db_wal::testkit::committed_fixture_storage(row, &document).await;
    let storage = StdArc::new(db_storage::DbBackend::Memory(memory));
    let (mut engine, report) = ArtifactEngine::open_retained(protocol::ArtifactId(document.0.clone()), storage, ArtifactEngineConfig::default(), 1).await.unwrap();
    assert_eq!(report.commands_replayed, 0);
    assert!(!report.from_snapshot);
    assert_eq!(engine.frontier.head_seq, 0);
    assert!(engine.state.values.is_empty());
    assert!(engine.applied.is_empty());
    engine.wal.close().await.unwrap();
    while engine.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_engine_create_rejection_propagates_exact_wal_release_owner() {
    let inner = StdArc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let fault = db_storage::FaultStorage::new(inner).await;
    fault.set_script(db_storage::FaultScript { fail_nth_write: Some(1), ..db_storage::FaultScript::default() }).await;
    let storage = StdArc::new(db_storage::DbBackend::Fault(Box::new(fault)));
    let document = document_id().await;
    let core_document = to_core_document_id(&document).await;
    let rejected = match ArtifactEngine::create_retained(document, storage.clone(), ArtifactEngineConfig::default(), 0).await {
        Err(rejected) => rejected,
        Ok(mut engine) => {
            engine.wal.close().await.unwrap();
            panic!("faulted engine create was admitted");
        }
    };
    assert!(matches!(rejected.error(), DbError::Io(_)));
    assert!(rejected.has_retained_writer());
    assert!(matches!(storage.wal().await.acquire_writer(&core_document).await, Err(DbError::Conflict(_))));
    assert!(matches!(rejected_engine_open_error(rejected).await, DbError::Io(_)));
    storage.wal().await.acquire_writer(&core_document).await.unwrap().release().await.unwrap();
}

fn history_construction_test_lock() -> std::sync::MutexGuard<'static, ()> {
    history_capacity_test_lock()
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_replay_projects_real_committed_batch_and_cancels_owned_sources() {
    if !db_io_driver::process_isolated_law("tests::artifact_history_replay_projects_real_committed_batch_and_cancels_owned_sources") {
        return;
    }
    let _history_capacity = history_construction_test_lock();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📝️wal/🧪️tests/🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    let projection = &fixture["historyProjection"];
    let mut engine = ArtifactEngine::create_retained(document_id().await, storage().await, ArtifactEngineConfig::default(), 0).await.unwrap();
    let mut commands = Vec::new();
    for row in projection["commands"].as_array().unwrap() {
        commands.push(envelope(row["id"].as_str().unwrap(), &[], "history-author", &[(row["path"].as_str().unwrap(), row["value"].clone())]).await);
    }
    let receipt = engine.submit(CommandBatch::new(commands).await.unwrap(), SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, 1).await.unwrap();
    let mut replay = engine.history_replay(1, StdArc::new(std::sync::atomic::AtomicBool::new(false)), HistoryReplayReservation::try_new().unwrap());
    let mut view = (&mut replay).await.unwrap();
    assert!(replay.terminal_is_empty());
    assert_eq!(view.entries.len() as u64, projection["expected"]["entries"].as_u64().unwrap());
    let entry = view.entries[0];
    assert_eq!(entry.operation_count, 2);
    for (index, id) in projection["expected"]["operationIds"].as_array().unwrap().iter().enumerate() {
        assert!(view.operation_id_eq(0, index, id.as_str().unwrap()));
    }
    assert_eq!(entry.head_seq, projection["expected"]["headSeq"].as_u64().unwrap());
    assert_eq!(entry.commit_seq, projection["expected"]["commitSeq"].as_u64().unwrap());
    assert_eq!((entry.head_seq, entry.commit_seq, entry.chain_hash, entry.epoch), (receipt.frontier.head_seq, receipt.frontier.commit_seq, receipt.frontier.chain_hash, receipt.frontier.epoch));
    while view.close_step() {}
    assert!(view.terminal_is_empty());
    for checkpoint in ["verify", "committed", "copied", "published"] {
        let cancelled = StdArc::new(std::sync::atomic::AtomicBool::new(false));
        let mut replay = engine.history_replay(1, cancelled.clone(), HistoryReplayReservation::try_new().unwrap());
        let mut turns = 0;
        let reached = std::future::poll_fn(|context| {
            let target = match checkpoint {
                "verify" => matches!(replay.phase.as_ref(), Some(HistoryReplayPhase::Verify { .. })),
                "committed" => matches!(replay.phase.as_ref(), Some(HistoryReplayPhase::CommittedBody { .. })),
                "copied" => replay.result_len > 0 && replay.reservation.as_ref().is_some_and(|owner| owner.operation_ids.len() == 1),
                "published" => replay.reservation.as_ref().is_some_and(|owner| owner.entries.len() == 1),
                _ => unreachable!(),
            };
            if target {
                return std::task::Poll::Ready(true);
            }
            turns += 1;
            assert!(turns < 100_000, "history cancellation checkpoint stopped progressing");
            match Pin::new(&mut replay).poll(context) {
                std::task::Poll::Pending => std::task::Poll::Pending,
                std::task::Poll::Ready(result) => {
                    if let Ok(mut view) = result {
                        while view.close_step() {}
                    }
                    std::task::Poll::Ready(false)
                }
            }
        })
        .await;
        assert!(reached);
        assert!(replay.authenticated.is_some());
        assert_eq!(replay.reservation.as_ref().unwrap().entries.len(), usize::from(checkpoint == "published"));
        cancelled.store(true, std::sync::atomic::Ordering::Release);
        assert!(matches!((&mut replay).await, Err(DbError::Closed)));
        assert!(replay.terminal_is_empty());
    }
    engine.wal.close().await.unwrap();
    while engine.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_and_opener_reject_neutral_inner_documents_and_frontier_order() {
    if !db_io_driver::process_isolated_law("tests::artifact_history_and_opener_reject_neutral_inner_documents_and_frontier_order") {
        return;
    }
    let _history_capacity = history_construction_test_lock();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📝️wal/🧪️tests/🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    for row in fixture["historyProjection"]["rejections"].as_array().unwrap() {
        let backing = storage().await;
        let mut engine = ArtifactEngine::create_retained(document_id().await, backing.clone(), ArtifactEngineConfig::default(), 0).await.unwrap();
        let mut records = db_wal_log::WalRecordBatch::new();
        for record in row["records"].as_array().unwrap() {
            let document = if record["document"] == "foreign" { "foreign-history-document".to_string() } else { engine.document.0.clone() };
            let record = match record["kind"].as_str().unwrap() {
                "command" => {
                    let mut command = envelope("hostile-history", &[], "history-author", &[("entry", serde_json::json!("value"))]).await;
                    command.document_id = protocol::ArtifactId(document);
                    db_wal_log::WalRecord::Command(retained_wal_envelope(&command).await)
                }
                "frontier" => db_wal_log::WalRecord::Frontier(Frontier { document: ArtifactId(document), head_seq: 1, commit_seq: 1, chain_hash: [7; 32], epoch: 3 }),
                _ => unreachable!(),
            };
            records.push(record).unwrap_or_else(|mut record| {
                while record.close_step().unwrap() {}
                panic!("hostile history fixture exceeded record capacity");
            });
        }
        let facet = backing.wal().await;
        engine.wal.submit(&facet, &[], &records, DurabilityClass::Fsync, 1).await.unwrap();
        while records.close_step().unwrap() {
            semio_framework_async::yield_once().await;
        }
        drop(records);
        drop(facet);
        let mut replay = engine.history_replay(1, StdArc::new(std::sync::atomic::AtomicBool::new(false)), HistoryReplayReservation::try_new().unwrap());
        let rejected = match (&mut replay).await {
            Err(DbError::Corrupt(_)) => true,
            Ok(mut view) => {
                while view.close_step() {}
                false
            }
            Err(_) => false,
        };
        assert!(replay.terminal_is_empty());
        engine.wal.close().await.unwrap();
        while engine.state.values.close_step().unwrap() {
            semio_framework_async::yield_once().await;
        }
        drop(engine);
        assert!(rejected, "history admitted {}", row["name"]);
        let rejected = match ArtifactEngine::open_retained(document_id().await, backing, ArtifactEngineConfig::default(), 2).await {
            Err(rejected) => matches!(rejected_engine_open_error(rejected).await, DbError::Corrupt(_)),
            Ok((mut engine, _)) => {
                engine.wal.close().await.unwrap();
                while engine.state.values.close_step().unwrap() {
                    semio_framework_async::yield_once().await;
                }
                false
            }
        };
        assert!(rejected, "opener admitted {}", row["name"]);
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_staging_retirement_success_refusal_cancel_stale_fault_drop_interrupted_close_and_max_plus_one_are_lossless() {
    if !db_io_driver::process_isolated_law("tests::artifact_staging_retirement_success_refusal_cancel_stale_fault_drop_interrupted_close_and_max_plus_one_are_lossless") {
        return;
    }
    while artifact_state_retirement_maintenance_step().unwrap() {}
    let accepted_cancel = StdArc::new(std::sync::atomic::AtomicBool::new(false));
    let mut accepted_control = db_state::StateCursorControl::new(accepted_cancel, std::time::Instant::now() + std::time::Duration::from_secs(30), 8).unwrap();
    let mut accepted = db_state::StateEntry::try_admit("accepted", vec![0x41], 1, &mut accepted_control).await.unwrap();
    assert!(accepted.close_step().unwrap());
    assert!(accepted.close_step().unwrap());
    assert!(accepted.close_step().unwrap());
    assert!(!accepted.close_step().unwrap());
    assert!(accepted.terminal_is_empty());

    let cancelled = StdArc::new(std::sync::atomic::AtomicBool::new(true));
    let mut control = db_state::StateCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 8).unwrap();
    let rejected = db_state::StateEntry::try_admit("path", vec![0x42], 1, &mut control).await.unwrap_err();
    assert!(matches!(rejected.error(), DbError::Unavailable(message) if message == "state cursor cancelled"));
    let source = rejected.source().expect("exact refused source").as_ptr();
    let mut second_control = db_state::StateCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(true)), std::time::Instant::now() + std::time::Duration::from_secs(30), 8).unwrap();
    let second_rejected = db_state::StateEntry::try_admit("second-path", vec![0x44], 1, &mut second_control).await.unwrap_err();
    let second_source = second_rejected.source().expect("second exact refused source").as_ptr();
    let initial_retirement = reserve_artifact_state_retirement().expect("artifact retirement preflight");
    let mut cursor = ArtifactStateRetirementCursor::rejected(initial_retirement, rejected, std::array::from_fn(|_| None));
    assert!(cursor.close_step().unwrap());
    assert_eq!(cursor.rejected.as_ref().and_then(db_state::StateEntryRejected::source).map(Vec::as_ptr), Some(source));
    release_artifact_state_retirement(&mut cursor.retirement);
    {
        let mut retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in retired.iter_mut() {
            *slot = Some(ArtifactStateRetirementCursor::empty());
        }
    }
    {
        let mut overflow = ARTIFACT_STATE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in overflow.iter_mut() {
            *slot = Some(ArtifactStateRetirementCursor::empty());
        }
    }
    ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT.store(false, std::sync::atomic::Ordering::Release);
    cursor.retirement = Some(reserve_artifact_state_retirement().expect("artifact quarantine preflight"));
    let second_retirement = reserve_artifact_state_retirement().expect("second artifact quarantine preflight");
    assert!(retire_artifact_state_owner(cursor).is_ok());
    assert!(retire_artifact_state_owner(ArtifactStateRetirementCursor::rejected(second_retirement, second_rejected, std::array::from_fn(|_| None))).is_ok());
    assert!(ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT.load(std::sync::atomic::Ordering::Acquire));
    {
        let quarantine = ARTIFACT_STATE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(quarantine.iter().flatten().find_map(|owner| owner.rejected.as_ref()?.source().map(Vec::as_ptr)), Some(source));
        assert!(quarantine.iter().flatten().any(|owner| owner.rejected.as_ref().and_then(db_state::StateEntryRejected::source).map(Vec::as_ptr) == Some(second_source)));
    }
    {
        let mut retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in retired.iter_mut() {
            *slot = None;
        }
    }
    for _ in 0..ARTIFACT_STATE_RETIREMENT_SLOTS * 2 {
        assert!(artifact_state_retirement_maintenance_step().unwrap());
    }
    assert!(artifact_state_retirement_maintenance_step().unwrap());
    {
        let retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(retired.iter().flatten().find_map(|owner| owner.rejected.as_ref()?.source().map(Vec::as_ptr)), Some(source));
    }
    while artifact_state_retirement_maintenance_step().unwrap() {}

    let mut deadline_control = db_state::StateCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now(), 1).unwrap();
    let deadline = db_state::StateEntry::try_admit("deadline", vec![0x43], 1, &mut deadline_control).await.unwrap_err();
    assert!(matches!(deadline.error(), DbError::Unavailable(message) if message == "state cursor deadline reached"));
    let deadline_retirement = reserve_artifact_state_retirement().expect("deadline artifact retirement preflight");
    assert!(retire_artifact_state_owner(ArtifactStateRetirementCursor::rejected(deadline_retirement, deadline, std::array::from_fn(|_| None))).is_ok());
    while artifact_state_retirement_maintenance_step().unwrap() {}

    for tier in [&ARTIFACT_STATE_RETIREMENT, &ARTIFACT_STATE_RETIREMENT_OVERFLOW, &ARTIFACT_STATE_RETIREMENT_QUARANTINE] {
        let mut owners = tier.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in owners.iter_mut() {
            *slot = Some(ArtifactStateRetirementCursor::empty());
        }
    }
    let mut state = DocumentState::new();
    let entries = vec![("parked-owners-retire-first".to_string(), None)];
    state.apply_entries(&protocol::MutationId("parked-owners-retire-first".to_string()), entries).await.expect("an apply first retires every parked owner, so parked owners alone never refuse it");
    for reservations in &ARTIFACT_STATE_RETIREMENT_RESERVATIONS {
        reservations.store(u64::MAX, std::sync::atomic::Ordering::Release);
    }
    let entries = vec![("exact-all-tier-state-refusal".to_string(), None)];
    let refusal = match state.apply_entries(&protocol::MutationId("exact-all-tier-state-refusal".to_string()), entries).await {
        Err(error) => error,
        Ok(_) => panic!("all-tier artifact retirement saturation admitted a state mutation"),
    };
    assert!(matches!(refusal, DbError::Unavailable(message) if message == "artifact state retirement pressure refused admission"));
    for reservations in &ARTIFACT_STATE_RETIREMENT_RESERVATIONS {
        reservations.store(0, std::sync::atomic::Ordering::Release);
    }
}

struct HistoryReplayTestWake;

impl std::task::Wake for HistoryReplayTestWake {
    fn wake(self: StdArc<Self>) {}
}

async fn storage() -> StdArc<db_storage::DbBackend> {
    StdArc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()))
}

async fn document_id() -> protocol::ArtifactId {
    protocol::ArtifactId("doc-1".to_string())
}

async fn stored_json(mut bytes: db_query::QueryBytes) -> serde_json::Value {
    let mut raw = Vec::with_capacity(bytes.len());
    for fragment in bytes.fragments() {
        raw.extend_from_slice(fragment);
    }
    while bytes.close_step().unwrap().is_some() {}
    decode_pathmap_json(&raw).await.expect("stored json value")
}

async fn envelope(id: &str, deps: &[&str], actor: &str, entries: &[(&str, serde_json::Value)]) -> protocol::MutationEnvelope {
    let object: Vec<(String, DslValue)> = entries.iter().map(|(path, value)| (path.to_string(), DslValue::from(value))).collect();
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(id.to_string()),
        document_id: document_id().await,
        actor: protocol::ActorId(actor.to_string()),
        dependencies: deps.iter().map(|dep| protocol::MutationId((*dep).to_string())).collect(),
        observed: None,
        target: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::Object(object)).await },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::Object(vec![])).await },
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        transaction: None, verb: None, line: None,
    }
}

async fn retained_wal_envelope(envelope: &protocol::MutationEnvelope) -> db_wal_log::WalBytes {
    let mut encoded = Vec::new();
    protocol::encode_envelope(envelope, &mut encoded);
    encoded.shrink_to_fit();
    let mut control = db_wal_log::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap();
    db_wal_log::WalBytes::try_admit(encoded, (db_io_core::DB_IO_OPERATION_PAGES * db_io_core::DB_IO_PAGE_BYTES) as u64, &mut control).await.unwrap()
}
//#region 🔖️Engine submit + materialize + WAL replay
#[semio_framework_async_macros::async_test]
async fn submit_persists_to_wal_and_updates_materialized_state_and_frontier() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();

    let batch = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("name", serde_json::json!("hello"))]).await]).await.unwrap();
    let receipt = engine.submit(batch, SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, 1).await.unwrap();

    assert_eq!(receipt.command_id, protocol::MutationId("op-1".to_string()));
    assert_eq!(receipt.frontier.head_seq, 1);
    assert_eq!(receipt.frontier.commit_seq, 1);
    assert!(receipt.conflicts.is_empty());
    assert!(receipt.state_hash.is_some());

    let stored = engine.get("name").await.unwrap().unwrap();
    let value: serde_json::Value = stored_json(stored).await;
    assert_eq!(value, serde_json::json!("hello"));
    assert_eq!(engine.frontier().await.head_seq, 1);
    assert_eq!(engine.checkpoint_publication_snapshot().await.head_edit_id, Some(protocol::MutationId("op-1".to_string())));
}

#[semio_framework_async_macros::async_test]
async fn open_replays_the_wal_and_reconstructs_state_and_frontier_identically() {
    let storage = storage().await;
    let (before_count, before_frontier) = {
        let mut engine = ArtifactEngine::create(document_id().await, storage.clone(), ArtifactEngineConfig::default(), 0).unwrap();
        let batch1 = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("name", serde_json::json!("hello"))]).await]).await.unwrap();
        engine.submit(batch1, SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, 1).await.unwrap();
        let batch2 = CommandBatch::new(vec![envelope("op-2", &["op-1"], "alice", &[("count", serde_json::json!(2))]).await]).await.unwrap();
        engine.submit(batch2, SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, 2).await.unwrap();
        let count = stored_json(engine.get("count").await.unwrap().unwrap()).await;
        assert!(store::pack_rt::json_values_equal(&count, &serde_json::json!(2)));
        let frontier = engine.frontier().await;
        engine.wal.close().await.unwrap();
        while engine.state.values.close_step().unwrap() {
            semio_framework_async::yield_once().await;
        }
        (count, frontier)
    };

    let (mut reopened, report) = ArtifactEngine::open(document_id().await, &storage, ArtifactEngineConfig::default(), 3).unwrap();
    assert_eq!(report.torn_tail_bytes, 0);
    assert_eq!(reopened.frontier().await.head_seq, 2);
    assert_eq!(reopened.frontier().await.commit_seq, 2);
    assert_eq!(reopened.frontier().await, before_frontier);
    assert_eq!(reopened.checkpoint_publication_snapshot().await.head_edit_id, Some(protocol::MutationId("op-2".to_string())));

    let name: serde_json::Value = stored_json(reopened.get("name").await.unwrap().unwrap()).await;
    assert_eq!(name, serde_json::json!("hello"));
    let count: serde_json::Value = stored_json(reopened.get("count").await.unwrap().unwrap()).await;
    assert_eq!(count, before_count);
    reopened.wal.close().await.unwrap();
    while reopened.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
}

#[semio_framework_async_macros::async_test]
async fn ledger_tail_returns_exact_committed_batches_between_two_published_points() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage.clone(), ArtifactEngineConfig::default(), 0).unwrap();
    let options = || SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() };
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("a", serde_json::json!(1))]).await]).await.unwrap(), options(), 1).await.unwrap();
    let first = ArtifactLedgerPoint::of_snapshot(&engine.checkpoint_publication_snapshot().await);
    engine
        .submit(CommandBatch::new(vec![envelope("op-2", &["op-1"], "bob", &[("b", serde_json::json!(2))]).await, envelope("op-3", &["op-2"], "bob", &[("c", serde_json::json!(3))]).await]).await.unwrap(), options(), 2)
        .await
        .unwrap();
    let second = ArtifactLedgerPoint::of_snapshot(&engine.checkpoint_publication_snapshot().await);
    engine.submit(CommandBatch::new(vec![envelope("op-4", &["op-3"], "alice", &[("a", serde_json::json!(4))]).await]).await.unwrap(), options(), 3).await.unwrap();
    let head = ArtifactLedgerPoint::of_snapshot(&engine.checkpoint_publication_snapshot().await);
    let core = ArtifactId(document_id().await.0);
    let wal = storage.wal().await;
    let cancelled = || StdArc::new(std::sync::atomic::AtomicBool::new(false));
    let ids = |commits: &[ArtifactLedgerCommit]| commits.iter().map(|commit| commit.envelopes.iter().map(|envelope| envelope.mutation_id.0.clone()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let whole = artifact_ledger_tail(&wal, &core, &ArtifactLedgerPoint::genesis(), &head, cancelled()).await.unwrap();
    assert_eq!(ids(&whole), vec![vec!["op-1".to_string()], vec!["op-2".to_string(), "op-3".to_string()], vec!["op-4".to_string()]]);
    assert_eq!(whole.iter().map(|commit| commit.point.clone()).collect::<Vec<_>>(), vec![first.clone(), second.clone(), head.clone()]);
    assert_eq!(whole[1].point.head_seq, 3);
    assert_eq!(whole[1].point.head_edit_id, Some(protocol::MutationId("op-3".to_string())));

    let tail = artifact_ledger_tail(&wal, &core, &first, &second, cancelled()).await.unwrap();
    assert_eq!(ids(&tail), vec![vec!["op-2".to_string(), "op-3".to_string()]]);
    assert_eq!(tail[0].envelopes[1], envelope("op-3", &["op-2"], "bob", &[("c", serde_json::json!(3))]).await);

    let mut forged = second.clone();
    forged.chain_hash[0] ^= 1;
    assert!(matches!(artifact_ledger_tail(&wal, &core, &first, &forged, cancelled()).await, Err(DbError::NotFound(_))));
    assert!(matches!(artifact_ledger_tail(&wal, &core, &forged, &head, cancelled()).await, Err(DbError::NotFound(_))));
    assert!(matches!(artifact_ledger_tail(&wal, &core, &head, &head, cancelled()).await, Err(DbError::InvalidArgument(_))));
    assert!(matches!(artifact_ledger_tail(&wal, &core, &second, &first, cancelled()).await, Err(DbError::InvalidArgument(_))));
    let stopped = StdArc::new(std::sync::atomic::AtomicBool::new(true));
    assert!(matches!(artifact_ledger_tail(&wal, &core, &ArtifactLedgerPoint::genesis(), &head, stopped).await, Err(DbError::Unavailable(_))));
    drop(wal);
    engine.wal.close().await.unwrap();
    while engine.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
}

#[semio_framework_async_macros::async_test]
async fn materialize_from_snapshot_plus_wal_suffix_matches_full_replay() {
    let storage = storage().await;
    {
        let mut engine = ArtifactEngine::create(document_id().await, storage.clone(), ArtifactEngineConfig::default(), 0).unwrap();
        for i in 0..3 {
            let key = format!("path-{i}");
            let value = format!("value-{i}");
            let batch = CommandBatch::new(vec![envelope(&format!("op-{i}"), &[], "alice", &[(&key, serde_json::json!(value))]).await]).await.unwrap();
            engine.submit(batch, SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, i).await.unwrap();
        }
        engine.snapshot_now(10).await.unwrap();
        for i in 3..6 {
            let key = format!("path-{i}");
            let value = format!("value-{i}");
            let batch = CommandBatch::new(vec![envelope(&format!("op-{i}"), &[], "alice", &[(&key, serde_json::json!(value))]).await]).await.unwrap();
            engine.submit(batch, SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, i).await.unwrap();
        }
    }

    let (reopened, report) = ArtifactEngine::open(document_id().await, &storage, ArtifactEngineConfig::default(), 20).unwrap();
    assert!(report.from_snapshot);
    assert_eq!(report.commands_replayed, 3);
    assert_eq!(reopened.frontier().await.head_seq, 6);
    for i in 0..6 {
        let value: serde_json::Value = stored_json(reopened.get(&format!("path-{i}")).await.unwrap().unwrap()).await;
        assert_eq!(value, serde_json::json!(format!("value-{i}")));
    }
}

#[semio_framework_async_macros::async_test]
async fn deletion_via_json_null_tombstones_a_path() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();
    assert!(engine.get("x").await.unwrap().is_some());
    engine.submit(CommandBatch::new(vec![envelope("op-2", &["op-1"], "alice", &[("x", serde_json::Value::Null)]).await]).await.unwrap(), SubmitOptions::default(), 1).await.unwrap();
    assert!(engine.get("x").await.unwrap().is_none());
}
//#endregion 🔖️Engine submit + materialize + WAL replay
//#region 🔖️Deps + Dedupe
#[semio_framework_async_macros::async_test]
async fn submit_rejects_an_envelope_whose_dependency_was_never_applied() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    let batch = CommandBatch::new(vec![envelope("op-2", &["op-1-never-applied"], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap();
    let result = engine.submit(batch, SubmitOptions::default(), 0);
    assert!(matches!(result.await, Err(DbError::InvalidArgument(_))));
    assert!(engine.get("x").await.unwrap().is_none(), "a rejected batch must not have partially applied");
}

#[semio_framework_async_macros::async_test]
async fn resubmitting_the_same_batch_returns_the_cached_receipt_without_advancing_the_frontier() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    let batch = || db_actor::block_on(CommandBatch::new(vec![db_actor::block_on(envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]))])).unwrap();

    let first = engine.submit(batch(), SubmitOptions::default(), 0).await.unwrap();
    let frontier_after_first = engine.frontier().await;
    let second = engine.submit(batch(), SubmitOptions::default(), 1).await.unwrap();

    assert_eq!(first, second);
    assert_eq!(engine.frontier().await, frontier_after_first, "a deduped resubmit must not move the frontier");
}

/// 🪪️ Fixture law `🧫️fixtures/🪪️mutation-id-collision`: a replayed id is idempotent only while its
/// content matches the committed envelope; a colliding id is refused instead of silently dropped.
#[semio_framework_async_macros::async_test]
async fn a_committed_mutation_id_replays_idempotently_but_refuses_colliding_content() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🪪️mutation-id-collision/🔣️.json")).unwrap();
    let row_envelope = |row: &serde_json::Value| db_actor::block_on(envelope(row["id"].as_str().unwrap(), &[], row["actor"].as_str().unwrap(), &[(row["path"].as_str().unwrap(), row["value"].clone())]));
    let mut engine = ArtifactEngine::create(document_id().await, storage().await, ArtifactEngineConfig::default(), 0).unwrap();
    engine.submit(CommandBatch::new(vec![row_envelope(&fixture["committed"])]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();
    for (tick, case) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let name = case["name"].as_str().unwrap();
        let before = engine.frontier().await;
        let outcome = engine.submit(CommandBatch::new(vec![row_envelope(case)]).await.unwrap(), SubmitOptions::default(), tick as u64 + 1).await;
        let after = engine.frontier().await;
        match case["outcome"].as_str().unwrap() {
            "replayed" => assert!(outcome.is_ok() && after == before, "{name}"),
            "conflict" => assert!(matches!(outcome, Err(DbError::Conflict(_))) && after == before, "{name}: {outcome:?}"),
            "committed" => assert!(outcome.is_ok() && after.commit_seq == before.commit_seq + 1, "{name}"),
            other => panic!("unknown fixture outcome {other}"),
        }
        assert_eq!(after.head_seq, case["headSeq"].as_u64().unwrap(), "{name}");
    }
}

/// ⚖️ C12 P1 (ticket 26/09/23 session 14): a link cut's whole outbox reaches the authority as ONE batch. The largest
/// batch of the smallest db envelope the document backbone declares legal (`DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES`,
/// admitted by the wire's own exact decoder) holds more envelopes than both former capacity refusals (the per-operation
/// DB I/O credit and the 4 096-command batch limit); it commits as one transaction and replays to the same frontier.
#[semio_framework_async_macros::async_test]
async fn a_declared_legal_byte_maximal_batch_commits_as_one_transaction_and_replays() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage.clone(), ArtifactEngineConfig::default(), 0).unwrap();
    let mut envelopes = Vec::new();
    let mut bytes = 0usize;
    loop {
        let next = envelope(&format!("{:x}", envelopes.len()), &[], "a", &[]).await;
        let mut encoded = Vec::new();
        protocol::encode_envelope(&next, &mut encoded);
        let count_bytes = if envelopes.len() + 1 < 128 { 1 } else if envelopes.len() + 1 < 16_384 { 2 } else { 3 };
        if envelopes.len() == protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES || count_bytes + bytes + encoded.len() > protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES {
            let mut over = envelopes.clone();
            over.push(next);
            assert!(protocol::decode_document_backbone_envelopes_exact(&protocol::encode_envelopes(&over)).is_err(), "one more envelope leaves the declared batch");
            break;
        }
        bytes += encoded.len();
        envelopes.push(next);
    }
    let wire = protocol::encode_envelopes(&envelopes);
    assert_eq!(protocol::decode_document_backbone_envelopes_exact(&wire).unwrap().len(), envelopes.len(), "the batch is declared legal by the wire's own decoder");
    assert!(envelopes.len() > 4_096, "the byte-maximal batch ({} envelopes, {} bytes) exceeds the former 4 096-command limit", envelopes.len(), wire.len());
    let count = envelopes.len() as u64;
    let receipt = engine.submit(CommandBatch::new(envelopes).await.unwrap(), SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, 1).await.unwrap();
    assert_eq!((receipt.frontier.head_seq, receipt.frontier.commit_seq), (count, 1), "one transaction holds the whole batch");
    engine.wal.close().await.unwrap();
    while engine.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    let (mut reopened, _) = ArtifactEngine::open(document_id().await, &storage, ArtifactEngineConfig::default(), 2).unwrap();
    assert_eq!(reopened.frontier().await, receipt.frontier);
    reopened.wal.close().await.unwrap();
    while reopened.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
}

/// ⚖️ C12 P1: a batch refused after admission (here: its second envelope names an unknown dependency) records nothing
/// in the replay guard, so its envelope commits when resent; a resend that merges an already-committed envelope with a
/// new one commits only the new one (the committed envelope is idempotent, never a replay refusal); one operation twice
/// in one batch is refused.
#[semio_framework_async_macros::async_test]
async fn an_envelope_of_a_refused_batch_commits_when_resent_and_a_merged_resend_is_idempotent() {
    let mut engine = ArtifactEngine::create(document_id().await, storage().await, ArtifactEngineConfig::default(), 0).unwrap();
    let op = |id: &str, deps: &[&str], path: &str| db_actor::block_on(envelope(id, deps, "alice", &[(path, serde_json::json!(1))]));
    let refused = engine.submit(CommandBatch::new(vec![op("op-1", &[], "a"), op("op-2", &["never"], "b")]).await.unwrap(), SubmitOptions::default(), 1).await;
    assert!(matches!(refused, Err(DbError::InvalidArgument(_))));
    assert_eq!(engine.frontier().await.head_seq, 0);
    let resent = engine.submit(CommandBatch::new(vec![op("op-1", &[], "a")]).await.unwrap(), SubmitOptions::default(), 2).await.unwrap();
    assert_eq!((resent.frontier.head_seq, resent.frontier.commit_seq), (1, 1));
    let merged = engine.submit(CommandBatch::new(vec![op("op-1", &[], "a"), op("op-3", &["op-1"], "c")]).await.unwrap(), SubmitOptions::default(), 3).await.unwrap();
    assert_eq!((merged.frontier.head_seq, merged.frontier.commit_seq), (2, 2));
    let twice = engine.submit(CommandBatch::new(vec![op("op-4", &[], "d"), op("op-4", &[], "d")]).await.unwrap(), SubmitOptions::default(), 4).await;
    assert!(matches!(twice, Err(DbError::Conflict(_))));
    assert_eq!(engine.frontier().await.head_seq, 2);
}
//#endregion 🔖️Deps + Dedupe
//#region 🔖️Conflict
#[semio_framework_async_macros::async_test]
async fn concurrent_write_to_the_same_path_without_a_dependency_is_recorded_as_a_conflict() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();

    // op-2 writes the same path but does NOT declare op-1 as a dependency: a real concurrent
    // write from `op-2`'s author's point of view.
    let receipt = engine.submit(CommandBatch::new(vec![envelope("op-2", &[], "bob", &[("x", serde_json::json!(2))]).await]).await.unwrap(), SubmitOptions::default(), 1).await.unwrap();
    assert_eq!(receipt.conflicts.len(), 1);
    assert_eq!(receipt.conflicts[0].conflicting_with, protocol::MutationId("op-1".to_string()));
    assert_eq!(receipt.conflicts[0].path, "x");
    // Last-writer-wins: the conflicting write still applies.
    let x: serde_json::Value = stored_json(engine.get("x").await.unwrap().unwrap()).await;
    assert_eq!(x, serde_json::json!(2));
}

#[semio_framework_async_macros::async_test]
async fn declaring_the_prior_writer_as_a_dependency_avoids_the_conflict() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();
    let receipt = engine.submit(CommandBatch::new(vec![envelope("op-2", &["op-1"], "bob", &[("x", serde_json::json!(2))]).await]).await.unwrap(), SubmitOptions::default(), 1).await.unwrap();
    assert!(receipt.conflicts.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn preview_conflicts_uses_real_db_conflict_detector_against_recent_history() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();

    let probe = CommandBatch::new(vec![envelope("op-2", &[], "bob", &[("x", serde_json::json!(2))]).await]).await.unwrap();
    let conflicts = engine.preview_conflicts(&probe).await.unwrap();
    assert!(!conflicts.is_empty(), "db_conflict must detect the same-path intersection against recent history");
}
//#endregion 🔖️Conflict
//#region 🔖️Undo
#[semio_framework_async_macros::async_test]
async fn undo_applies_the_recorded_inverse_and_produces_a_fresh_commit() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    let original = protocol::MutationEnvelope {
        mutation_id: protocol::MutationId("op-1".to_string()),
        document_id: document_id().await,
        actor: protocol::ActorId("alice".to_string()),
        dependencies: Vec::new(),
        observed: None,
        target: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::from(&serde_json::json!({ "x": 1 }))).await },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::from(&serde_json::json!({ "x": null }))).await },
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        transaction: None, verb: None, line: None,
    };
    engine.submit(CommandBatch::new(vec![original]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();
    assert!(engine.get("x").await.unwrap().is_some());

    let receipt = engine.undo(&protocol::MutationId("op-1".to_string()), protocol::MutationId("op-1-undo".to_string()), protocol::ActorId("alice".to_string()), 1).await.unwrap();
    assert_eq!(receipt.frontier.head_seq, 2);
    assert!(engine.get("x").await.unwrap().is_none(), "undo must have applied the recorded inverse (delete x)");
}

#[semio_framework_async_macros::async_test]
async fn undo_of_an_unknown_operation_errs_not_found() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    let never_applied = protocol::MutationId("never-applied".to_string());
    let result = engine.undo(&never_applied, protocol::MutationId("undo-1".to_string()), protocol::ActorId("alice".to_string()), 0);
    assert!(matches!(result.await, Err(DbError::NotFound(_))));
}
//#endregion 🔖️Undo
//#region 🔖️Preview
#[semio_framework_async_macros::async_test]
async fn preview_is_never_durable_and_a_conflicting_commit_supersedes_it() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();

    let preview_id = engine.publish_preview(&[("y".to_string(), Some(serde_json::json!("preview-value")))], 0).await.unwrap();
    assert_eq!(engine.preview_status(&preview_id).await.unwrap(), db_preview::PreviewState::Active);
    let preview_value: serde_json::Value = stored_json(engine.preview_get(&preview_id, "y").await.unwrap().unwrap()).await;
    assert_eq!(preview_value, serde_json::json!("preview-value"));
    assert!(engine.get("y").await.unwrap().is_none(), "a preview must never be visible in committed state");

    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "bob", &[("y", serde_json::json!("committed-value"))]).await]).await.unwrap(), SubmitOptions::default(), 1).await.unwrap();
    assert_eq!(engine.preview_status(&preview_id).await.unwrap(), db_preview::PreviewState::Superseded, "an intersecting real commit must supersede the preview");

    let committed: serde_json::Value = stored_json(engine.get("y").await.unwrap().unwrap()).await;
    assert_eq!(committed, serde_json::json!("committed-value"));
}

#[semio_framework_async_macros::async_test]
async fn preview_withdraw_and_expire_transitions() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();

    let withdrawn_id = engine.publish_preview(&[("a".to_string(), Some(serde_json::json!(1)))], 0).await.unwrap();
    engine.withdraw_preview(&withdrawn_id).await.unwrap();
    assert_eq!(engine.preview_status(&withdrawn_id).await.unwrap(), db_preview::PreviewState::Withdrawn);

    let dummy_id = db_preview::PreviewId("does-not-exist".to_string());
    assert!(matches!(engine.preview_status(&dummy_id).await, Err(DbError::NotFound(_))));
}
//#endregion 🔖️Preview
//#region 🔖️Security
#[semio_framework_async_macros::async_test]
async fn security_gate_rejects_a_principal_denied_by_its_policy() {
    // An empty `RoleBasedPolicy` (no grants at all) denies every action, per its own doc — a
    // default-deny policy, matching `db_engine`'s own equivalent test of the same gate.
    let security = db_security::SecurityGate::new(db_security::RoleBasedPolicy::new(), db_security::ReplayGuard::new(60_000, 1_024), db_security::BudgetRegistry::new(100_000, 100_000), Arc::new(NullEmit));
    let storage = storage().await;
    let config = ArtifactEngineConfig { security, ..ArtifactEngineConfig::default() };
    let mut engine = ArtifactEngine::create(document_id().await, storage, config, 0).unwrap();
    let result = engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "bob", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0);
    assert!(matches!(result.await, Err(DbError::Unauthorized(_))));
    assert!(engine.get("x").await.unwrap().is_none());
}
//#endregion 🔖️Security
//#region 🔖️Query
#[semio_framework_async_macros::async_test]
async fn query_finds_a_committed_row_by_path() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("greeting", serde_json::json!("hello"))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();

    let query = db_query::Query::new().filter(db_query::Predicate::Eq(db_query::Path::empty().push_field("path"), db_query::Value::Text("greeting".to_string())));
    let result = engine.query(query, db_query::Consistency::Canonical).await.unwrap();
    assert_eq!(result.rows.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn live_query_refresh_reports_no_further_diff_right_after_submit_already_refreshed_it() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    let id = engine.subscribe(db_query::LiveQuerySpec { query: db_query::Query::new(), consistency: db_query::Consistency::Canonical }).await;
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();
    let diffs = engine.refresh_live_queries().await;
    assert!(diffs.is_empty());
    engine.unsubscribe(id).await;
}
//#endregion 🔖️Query
//#region 🔖️IndexBacklog + Receipts
async fn single_envelope_commit(engine: &mut ArtifactEngine, index: u64, durability: DurabilityClass) -> CommandReceipt {
    let id = format!("op-{index}");
    engine.submit(CommandBatch::new(vec![envelope(&id, &[], "alice", &[]).await]).await.unwrap(), SubmitOptions { durability, ..Default::default() }, index).await.unwrap()
}

async fn fs_storage(name: &str) -> (StdArc<db_storage::DbBackend>, std::path::PathBuf) {
    let dir = std::env::temp_dir().join(format!("db_artifact_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    (StdArc::new(db_storage::DbBackend::Fs(db_storage_fs::FsStorage::open(db_io_driver::db_io_test_pool(), &dir).await.unwrap())), dir)
}

async fn close_engine(engine: &mut ArtifactEngine) {
    if engine.close_flush_pending() {
        engine.close_flush().await.unwrap();
    }
    engine.wal.close().await.unwrap();
    while engine.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
}

/// ⚖️ A commit's receipt never waits for index I/O: 300 acknowledged single-envelope commits leave no index run behind
/// (the runner writes them after replying) — the state a crash after the Ack and before the index write leaves. The
/// reopened document refills its backlog from the WAL, its maintenance writes every full run, and later commits continue
/// the same runs: every indexed seq resolves to its WAL position, nothing twice, nothing missing.
#[semio_framework_async_macros::async_test]
async fn index_runs_follow_the_receipt_and_a_reopen_refills_them_from_the_wal() {
    let (storage, dir) = fs_storage("index_follows_receipt").await;
    let before = {
        let mut engine = ArtifactEngine::create(document_id().await, storage.clone(), ArtifactEngineConfig::default(), 0).unwrap();
        for index in 1..=300 {
            single_envelope_commit(&mut engine, index, DurabilityClass::Os).await;
        }
        let frontier = engine.frontier().await;
        close_engine(&mut engine).await;
        frontier
    };
    let core = to_core_document_id(&document_id().await).await;
    let index_facet = storage.index().await;
    assert_eq!(db_index::CommandIndex::new(&index_facet, core.clone()).await.indexed_through().await.unwrap(), 0, "no receipt waited for an index run");
    assert_eq!(db_index::FrontierIndex::new(&index_facet, core.clone()).await.indexed_through().await.unwrap(), 0);

    let (mut reopened, _) = ArtifactEngine::open(document_id().await, &storage, ArtifactEngineConfig::default(), 301).unwrap();
    assert_eq!(reopened.frontier().await, before);
    reopened.maintain_index().await.unwrap();
    let commands = db_index::CommandIndex::new(&index_facet, core.clone()).await;
    assert_eq!(commands.indexed_through().await.unwrap(), 256, "four full runs from the WAL suffix; the partial tail waits");
    assert_eq!(db_index::FrontierIndex::new(&index_facet, core.clone()).await.indexed_through().await.unwrap(), 256);
    for index in 301..=400 {
        single_envelope_commit(&mut reopened, index, DurabilityClass::Os).await;
        reopened.maintain_index().await.unwrap();
    }
    assert_eq!(commands.indexed_through().await.unwrap(), 384);
    for seq in [1u64, 64, 65, 256, 257, 300, 301, 384] {
        assert_eq!(commands.lookup(seq).await.unwrap().map(|location| location.offset), Some(seq), "seq {seq}");
    }
    assert_eq!(commands.lookup(385).await.unwrap(), None);
    let mut control = db_index::IndexHandle::new(&index_facet, core.clone(), db_index::IndexKind::Command).await.operation_control(65_536).unwrap();
    db_index::IndexHandle::new(&index_facet, core, db_index::IndexKind::Command).await.verify(&mut control).await.unwrap();
    close_engine(&mut reopened).await;
    drop(reopened);
    drop(index_facet);
    drop(storage);
    let _ = std::fs::remove_dir_all(&dir);
}

/// ⚖️ The engine keeps the receipts of its newest `APPLIED_RECEIPTS_MAX` batches only; a whole-batch resend inside the window
/// answers its original receipt, an older one is answered by the per-envelope dedupe without a new commit.
#[semio_framework_async_macros::async_test]
async fn applied_receipts_keep_a_bounded_window_and_older_resends_stay_idempotent() {
    let (storage, dir) = fs_storage("receipt_window").await;
    let mut engine = ArtifactEngine::create(document_id().await, storage.clone(), ArtifactEngineConfig::default(), 0).unwrap();
    let mut last = None;
    for index in 1..=(APPLIED_RECEIPTS_MAX as u64 + 10) {
        last = Some(single_envelope_commit(&mut engine, index, DurabilityClass::Os).await);
    }
    assert_eq!(engine.applied_receipts.len(), APPLIED_RECEIPTS_MAX);
    assert_eq!(engine.applied_receipt_order.len(), APPLIED_RECEIPTS_MAX);
    let frontier = engine.frontier().await;
    let newest = single_envelope_commit(&mut engine, APPLIED_RECEIPTS_MAX as u64 + 10, DurabilityClass::Os).await;
    assert_eq!(Some(newest), last, "a resend inside the window answers its original receipt");
    let oldest = single_envelope_commit(&mut engine, 1, DurabilityClass::Os).await;
    assert_eq!(oldest.frontier, frontier, "a resend beyond the window commits nothing");
    assert_eq!(engine.frontier().await, frontier);
    assert_eq!(engine.applied_receipts.len(), APPLIED_RECEIPTS_MAX);
    close_engine(&mut engine).await;
    drop(engine);
    drop(storage);
    let _ = std::fs::remove_dir_all(&dir);
}

/// ⚖️ A document stays writable far past the old 4 096-run listing ceiling (4 runs per 64 single-envelope commits: every
/// commit failed after ~65.5 k of them): 70 016 single-envelope commits on the filesystem backend, each followed by the
/// runner's index maintenance, all commit; the four kinds hold a few hundred runs, every indexed seq resolves.
#[semio_framework_async_macros::async_test]
async fn seventy_thousand_single_envelope_commits_stay_writable_with_bounded_index_runs() {
    const COMMITS: u64 = 70_016;
    let (storage, dir) = fs_storage("seventy_thousand").await;
    let mut engine = ArtifactEngine::create(document_id().await, storage.clone(), ArtifactEngineConfig::default(), 0).unwrap();
    for index in 1..=COMMITS {
        single_envelope_commit(&mut engine, index, DurabilityClass::Memory).await;
        engine.maintain_index().await.unwrap_or_else(|error| panic!("index maintenance after commit {index}: {error}"));
    }
    assert_eq!(engine.frontier().await.head_seq, COMMITS);
    let core = to_core_document_id(&document_id().await).await;
    let index_facet = storage.index().await;
    let mut runs = 0usize;
    for kind in [db_index::IndexKind::Command, db_index::IndexKind::Inverse, db_index::IndexKind::ActorSeq, db_index::IndexKind::Frontier] {
        let handle = db_index::IndexHandle::new(&index_facet, core.clone(), kind).await;
        let mut control = handle.operation_control(1_000_000).unwrap();
        let stats = handle.stats(&mut control).await.unwrap();
        assert!(stats.entry_count >= COMMITS, "{kind:?} holds every entry");
        handle.verify(&mut control).await.unwrap();
        runs += stats.run_count;
    }
    assert!(runs <= 300, "{runs} runs for {COMMITS} commits");
    let commands = db_index::CommandIndex::new(&index_facet, core.clone()).await;
    assert_eq!(commands.indexed_through().await.unwrap(), COMMITS);
    for seq in [COMMITS - 63, COMMITS] {
        assert_eq!(commands.lookup(seq).await.unwrap().map(|location| location.offset), Some(seq), "seq {seq}");
    }
    assert_eq!(db_index::FrontierIndex::new(&index_facet, core).await.indexed_through().await.unwrap(), COMMITS);
    close_engine(&mut engine).await;
    drop(engine);
    drop(index_facet);
    drop(storage);
    let _ = std::fs::remove_dir_all(&dir);
}
//#endregion 🔖️IndexBacklog + Receipts
//#region 🔖️Actor
#[semio_framework_async_macros::async_test]
async fn artifact_engine_submit_future_stays_within_the_worker_stack_budget() {
    let mut engine = ArtifactEngine::create_retained(document_id().await, storage().await, ArtifactEngineConfig::default(), 0).await.unwrap();
    let batch = CommandBatch::new(vec![envelope("frame-budget", &[], "alice", &[("name", serde_json::json!("x"))]).await]).await.unwrap();
    let submit = engine.submit(batch, SubmitOptions::default(), 0);
    assert!(std::mem::size_of_val(&submit) <= 512 * 1024, "submit future reserves {} B; fixed-capacity owners must stay boxed off the poll stack", std::mem::size_of_val(&submit));
    drop(submit);
    assert!(size_of::<db_wal_log::WalRecordBatch>() <= 32 && size_of::<db_index::RunEntries>() <= 32);
    engine.close().await.unwrap();
}

fn history_phase_fixture_page() -> Result<db_io_core::DbIoPages, DbError> {
    let mut writer = db_io_core::DbIoPageWriter::try_reserve(1).map_err(|rejected| rejected.into_error())?;
    writer.write_fragment(&[0])?;
    writer.seal().map_err(|rejected| rejected.into_error())
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_panic_at_each_phase_transition_retains_then_fault_retires() {
    if !db_io_driver::process_isolated_law("tests::artifact_history_panic_at_each_phase_transition_retains_then_fault_retires") {
        return;
    }
    let _history_capacity = history_construction_test_lock();
    let mut engine = ArtifactEngine::create_retained(document_id().await, storage().await, ArtifactEngineConfig::default(), 0).await.unwrap();
    let phases = Vec::from([
        HistoryReplayPhase::Probe,
        HistoryReplayPhase::SegmentLen { index: 0, future: Box::pin(async { Err(DbError::NotFound("phase fixture".to_string())) }) },
        HistoryReplayPhase::PageStart { index: 0, len: 1, offset: 0 },
        HistoryReplayPhase::PageRead { index: 0, len: 1, offset: 0, requested: 1, future: Box::pin(async { history_phase_fixture_page() }) },
        HistoryReplayPhase::PageClose { index: 0, len: 1, offset: 1, retained: history_phase_fixture_page().unwrap() },
        HistoryReplayPhase::Inventory { future: Box::pin(async { Ok(db_io_core::DbIoU64List::new()) }) },
        HistoryReplayPhase::Verify { index: 0 },
        HistoryReplayPhase::Frame { index: 0 },
        HistoryReplayPhase::CommittedBody { index: 0 },
        HistoryReplayPhase::Envelope { index: 0, cursor: HistoryEnvelopeCursor { pos: 0, end: 0, field: HistoryEnvelopeField::MutationId, dependencies: 0, target_segments: 0, verb: false, line: false, mutation_id: None } },
        HistoryReplayPhase::CopyMutation { index: 0, range: 0..0, copied: 0, result_start: 0 },
        HistoryReplayPhase::Frontier { index: 0, cursor: HistoryFrontierCursor { pos: 0, end: 0, field: HistoryFrontierField::Document, head_seq: 0, commit_seq: 0, chain_hash: [0; 32] } },
        HistoryReplayPhase::Publish { index: 0, head_seq: 0, commit_seq: 0, chain_hash: [0; 32], epoch: 0 },
        HistoryReplayPhase::Retire,
        HistoryReplayPhase::InventoryClose,
        HistoryReplayPhase::FinalizeSuccess,
    ]);
    let waker = std::task::Waker::from(StdArc::new(HistoryReplayTestWake));
    let mut context = std::task::Context::from_waker(&waker);
    for phase in phases {
        let mut replay = engine.history_replay(1, StdArc::new(std::sync::atomic::AtomicBool::new(false)), HistoryReplayReservation::try_new().unwrap());
        replay.phase = Some(phase);
        replay.transition = HistoryReplayTransition::InProgress;
        replay.panic_before_transition_commit = true;
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| Pin::new(&mut replay).poll(&mut context))).is_err());
        assert!(replay.phase.is_some(), "panic must retain the exact active phase owner");
        assert!(matches!(replay.transition, HistoryReplayTransition::InProgress));
        replay.request_close(DbError::Internal("phase panic fixture".to_string()));
        let mut terminal = false;
        for _ in 0..50_000 {
            if Pin::new(&mut replay).poll(&mut context).is_ready() {
                terminal = true;
                break;
            }
        }
        assert!(terminal);
        assert!(replay.terminal_is_empty());
    }
    engine.wal.close().await.unwrap();
    while engine.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
}
//#endregion 🔖️Actor

/// 🪪️ LAW: an undo belongs to its author on the hub's event log too (`ArtifactEngine::submit`, the twin of
/// `protocol::fold_history`'s refusal, `🔗️causal/🧫️fixtures/🗄️durable-collaborative-redo-v1`): a `Revert` or `Reinstate`
/// naming any operation another actor wrote is refused with the one `history.foreign-transition` message whatever the merge
/// policy and leaves the log untouched; the author's own undo/redo, one naming an operation of the same batch, and one naming
/// an operation the log does not know are admitted (measured live on hub 7800 p33: B's crafted `Revert` of A's note edit was
/// persisted and only the replicas refused it).
#[semio_framework_async_macros::async_test]
async fn a_history_transition_naming_another_actors_operation_is_refused_before_the_log() {
    let document = protocol::ArtifactId("doc-1".to_string());
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.expect("memory storage")));
    let mut engine = ArtifactEngine::create(document.clone(), storage, ArtifactEngineConfig::default(), 0).expect("engine");
    let write = |id: &str, actor: &str, at: u64| protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(id.to_string()),
        document_id: document.clone(),
        actor: protocol::ActorId(actor.to_string()),
        dependencies: Vec::new(),
        observed: None,
        target: vec![id.to_string()],
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId("fixture.opaque.v1".into()), payload: id.as_bytes().to_vec() },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId("fixture.opaque.v1".into()), payload: Vec::new() },
        timestamp: protocol::HybridLogicalTimestamp::new(at, 0),
        transaction: None, verb: None, line: None,
    };
    let transition = |revert: bool, names: &[&str], actor: &str, at: u64| {
        let mutation_ids = names.iter().map(|name| protocol::MutationId((*name).to_string())).collect();
        let transition = if revert { protocol::HistoryTransition::Revert { mutation_ids } } else { protocol::HistoryTransition::Reinstate { mutation_ids } };
        protocol::history_transition_envelope(&transition, &document, &protocol::ActorId(actor.to_string()), Vec::new(), protocol::HybridLogicalTimestamp::new(at, 0))
    };
    let cases: Vec<(&str, Vec<protocol::MutationEnvelope>, bool)> = vec![
        ("A writes", vec![write("a1", "actor-a", 1)], true),
        ("B writes", vec![write("b1", "actor-b", 2)], true),
        ("B reverts A's operation", vec![transition(true, &["a1"], "actor-b", 3)], false),
        ("B reinstates A's operation", vec![transition(false, &["a1"], "actor-b", 4)], false),
        ("B reverts its own and A's operation", vec![transition(true, &["b1", "a1"], "actor-b", 5)], false),
        ("B reverts its own operation", vec![transition(true, &["b1"], "actor-b", 6)], true),
        ("B reinstates its own operation", vec![transition(false, &["b1"], "actor-b", 7)], true),
        ("A reverts its own operation", vec![transition(true, &["a1"], "actor-a", 8)], true),
        ("C writes and reverts in one batch", vec![write("c1", "actor-c", 9), transition(true, &["c1"], "actor-c", 10)], true),
        ("B reverts an operation the log does not know", vec![transition(true, &["ghost"], "actor-b", 11)], true),
    ];
    for (index, (label, envelopes, admitted)) in cases.into_iter().enumerate() {
        for policy in [protocol::MergePolicy::LaissezFaire, protocol::MergePolicy::Vigilant] {
            let before = engine.frontier().await;
            let batch = CommandBatch::new(envelopes.clone()).await.expect("batch");
            match engine.submit(batch, SubmitOptions { policy, ..Default::default() }, index as u64 + 1).await {
                Ok(_) => assert!(admitted, "{label} ({policy:?}) was admitted"),
                Err(DbError::Rejected { messages, .. }) => {
                    assert!(!admitted, "{label} ({policy:?}) was refused: {messages:?}");
                    assert_eq!(messages.iter().map(|message| message.code.0.as_str()).collect::<Vec<_>>(), vec![FOREIGN_HISTORY_TRANSITION_CODE], "{label}");
                    assert_eq!(engine.frontier().await.head_seq, before.head_seq, "{label}: a refused transition leaves the log untouched");
                }
                Err(other) => panic!("{label} ({policy:?}): {other:?}"),
            }
            if admitted {
                break;
            }
        }
    }
}

//#region 🔖️Source laws
#[test]
fn retained_retry_owners_release_their_guard_before_resubmission() {
    for (name, source) in [("db_artifact_engine", include_str!("../../🦀️.rs"))] {
        let lines: Vec<&str> = source.lines().collect();
        for (index, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start().trim_start_matches("} else ");
            let Some((_, scrutinee)) = trimmed.strip_prefix("if let ").or_else(|| trimmed.strip_prefix("while let ")).and_then(|rest| rest.split_once(" = ")) else { continue };
            if !(scrutinee.contains("retry_job.lock()") && !scrutinee.starts_with("{ ") && scrutinee.trim_end().ends_with('{')) {
                continue;
            }
            let mut depth = 1i64;
            for body in &lines[index + 1..] {
                assert!(!body.contains("retry_job.lock()") && !body.contains("submit_exact("), "{name}:{} holds the retry-job guard across a body that relocks or resubmits it: {line}", index + 1);
                depth += body.matches('{').count() as i64 - body.matches('}').count() as i64;
                if depth <= 0 {
                    break;
                }
            }
        }
    }
}
//#endregion 🔖️Source laws
