/// 📜️ History replay and staging laws: the rejected open error, the neutral aborted open, the create rejection release owner, the real committed batch replay, the neutral inner documents and the staging retirement.

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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../📝️wal/🧪️tests/🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../📝️wal/🧪️tests/🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../📝️wal/🧪️tests/🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
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
