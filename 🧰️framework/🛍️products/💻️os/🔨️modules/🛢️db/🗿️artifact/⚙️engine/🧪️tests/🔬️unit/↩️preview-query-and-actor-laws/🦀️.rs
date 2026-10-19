// ↩️ Preview, query and actor laws: the preview transitions, the security gate, the query laws, the index backlog and receipt window, the actor history phases and the source laws.

//#region 🔖️Undo
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
        protocol::history_transition_envelope(&transition, &document, &protocol::ActorId(actor.to_string().into()), Vec::new(), protocol::HybridLogicalTimestamp::new(at, 0))
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
    for (name, source) in [("db_artifact_engine", ARTIFACT_ENGINE_SOURCE)] {
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

/// 🧾️ The authored source of the artifact engine crate: every child module and then the root, in module order, as the one text the source laws scan.
const ARTIFACT_ENGINE_SOURCE: &str = concat!(
    include_str!("../../../🧾️wal-batch-state-retirement-and-document-state/🦀️.rs"),
    include_str!("../../../⚙️config-index-backlog-and-open-rejection/🦀️.rs"),
    include_str!("../../../🌱️engine-lifecycle/🦀️.rs"),
    include_str!("../../../🚦️engine-submit-pipeline/🦀️.rs"),
    include_str!("../../../↩️engine-undo-durable-group-query-and-preview/🦀️.rs"),
    include_str!("../../../📖️state-query-source-and-pages/🦀️.rs"),
    include_str!("../../../📈️ledger-tail/🦀️.rs"),
    include_str!("../../../🦀️.rs"),
);
