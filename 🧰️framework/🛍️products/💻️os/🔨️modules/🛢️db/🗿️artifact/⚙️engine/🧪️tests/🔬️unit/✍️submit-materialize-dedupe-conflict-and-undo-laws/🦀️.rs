// ✍️ Submit, materialize, dedupe, conflict and undo laws: the submit and replay of the wal, the ledger tail, the snapshot suffix materialization, the dependency and dedupe laws, the conflict detection and the undo.

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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🪪️mutation-id-collision/🔣️.json")).unwrap();
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

    let receipt = engine.submit(CommandBatch::new(vec![envelope("op-2", &[], "bob", &[("x", serde_json::json!(2))]).await]).await.unwrap(), SubmitOptions::default(), 1).await.unwrap();
    assert_eq!(receipt.conflicts.len(), 1);
    assert_eq!(receipt.conflicts[0].conflicting_with, protocol::MutationId("op-1".to_string()));
    assert_eq!(receipt.conflicts[0].path, "x");
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
        actor: protocol::ActorId("alice".to_string().into()),
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

    let receipt = engine.undo(&protocol::MutationId("op-1".to_string()), protocol::MutationId("op-1-undo".to_string()), protocol::ActorId("alice".to_string().into()), 1).await.unwrap();
    assert_eq!(receipt.frontier.head_seq, 2);
    assert!(engine.get("x").await.unwrap().is_none(), "undo must have applied the recorded inverse (delete x)");
}

#[semio_framework_async_macros::async_test]
async fn undo_of_an_unknown_operation_errs_not_found() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    let never_applied = protocol::MutationId("never-applied".to_string());
    let result = engine.undo(&never_applied, protocol::MutationId("undo-1".to_string()), protocol::ActorId("alice".to_string().into()), 0);
    assert!(matches!(result.await, Err(DbError::NotFound(_))));
}
//#endregion 🔖️Undo
