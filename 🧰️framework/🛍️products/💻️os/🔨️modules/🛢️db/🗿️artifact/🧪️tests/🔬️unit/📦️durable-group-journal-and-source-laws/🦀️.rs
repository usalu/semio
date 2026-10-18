// 📦️ Durable group journal and source laws: the durable group journal commits, the committed group decision and recovery, the journal cancellation, the finite pool turns, the group commit close drain and the runner source laws.

//#region 🔖️Actor
#[semio_framework_async_macros::async_test]
async fn document_authority_durable_group_journal_commits_one_exact_fsync_event() {
    let (authority, storage, pool) = journal_authority().await;
    let record = store::durable_group::durable_owned_group_journal_test_record(include_str!("../../../../../🏪️store/🧩️composition/🗄️durable-group/🧪️tests/🧫️fixtures/🔣️.json"));
    let canonical_pack = record.canonical_pack().to_vec();
    let decision_sha256 = record.decision_sha256().to_string();
    let anchor_sha256 = record.anchor_sha256().to_string();
    let durable_head_edit_id = protocol::MutationId(record.parent_edit_id().to_string());
    let durable_chain_hash = record.parent_post_revision();
    let mut sink = authority.durable_group_journal_sink(7);
    let mut commit = sink.begin_commit(canonical_pack.clone(), decision_sha256.clone());
    let receipt = loop {
        match commit.advance(journal_grant()).unwrap() {
            store::durable_group::DurableOwnedGroupJournalAdvanceV1::Pending => semio_framework_async::yield_once().await,
            store::durable_group::DurableOwnedGroupJournalAdvanceV1::Committed(receipt) => break receipt,
            other => panic!("exact durable journal was not committed: {other:?}"),
        }
    };
    assert_eq!(receipt.anchor_sha256, anchor_sha256);
    assert_eq!(receipt.decision_sha256, decision_sha256);
    let durable_snapshot = authority.checkpoint_publication_snapshot().await.unwrap();
    assert_eq!(durable_snapshot.frontier.head_seq, 1);
    assert_eq!(durable_snapshot.frontier.commit_seq, 1);
    assert_eq!(durable_snapshot.frontier.chain_hash, durable_chain_hash);
    assert_eq!(durable_snapshot.head_edit_id, Some(durable_head_edit_id.clone()));
    close_journal_commit(commit.as_mut());
    drop(commit);
    drop(sink);
    let mut ordinary = envelope("ordinary-after-durable-decision", &[], "map-owner", &[("/ordinary", serde_json::json!(true))]).await;
    ordinary.document_id = protocol::ArtifactId("map-a".to_string());
    authority.submit(CommandBatch::new(vec![ordinary]).await.unwrap(), SubmitOptions { durability: DurabilityClass::Fsync, ..SubmitOptions::default() }, 8).await.unwrap();
    let ordinary_snapshot = authority.checkpoint_publication_snapshot().await.unwrap();
    assert_eq!(ordinary_snapshot.frontier.head_seq, 2);
    assert_eq!(ordinary_snapshot.frontier.commit_seq, 2);
    assert_eq!(ordinary_snapshot.head_edit_id, Some(protocol::MutationId("ordinary-after-durable-decision".to_string())));
    shutdown_journal_authority(&authority, &pool).await;

    let (mut reopened, report) = ArtifactEngine::open_retained(protocol::ArtifactId("map-a".to_string()), storage.clone(), ArtifactEngineConfig::default(), 9).await.unwrap();
    assert_eq!(report.commands_replayed, 1);
    let reopened_snapshot = reopened.checkpoint_publication_snapshot().await;
    assert_eq!(reopened_snapshot, ordinary_snapshot, "reopen must reconstruct the exact Event-plus-Command publication frontier");
    reopened.close().await.unwrap();
    drop(reopened);

    let wal_facet = storage.wal().await;
    let mut replay = db_wal_log::replay_committed_document(
        &wal_facet,
        &ArtifactId::from("map-a"),
        db_wal_log::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap(),
    )
    .await
    .unwrap();
    let replay_document = ArtifactId::from("map-a");
    let mut witnesses = Vec::new();
    let mut committed_transactions = 0;
    loop {
        let transaction = match replay.next_transaction_step().await.unwrap() {
            db_wal_log::WalCommittedStep::Transaction(transaction) => transaction,
            db_wal_log::WalCommittedStep::Yield => continue,
            db_wal_log::WalCommittedStep::Done => break,
        };
        committed_transactions += 1;
        if let Some(witness) = committed_durable_group_decision_from_transaction(&replay_document, transaction).await.unwrap() {
            witnesses.push(witness);
        }
    }
    while replay.close_owner_step().unwrap() {}
    assert_eq!(committed_transactions, 2);
    assert_eq!(witnesses.len(), 1);
    let witness = witnesses.pop().unwrap();
    assert_eq!(witness.document(), &replay_document);
    assert_eq!(witness.transaction_id(), receipt.transaction_id);
    assert_eq!(witness.segment_index(), receipt.segment_index);
    assert_eq!(witness.receipt(), receipt);
    assert_eq!(witness.record.canonical_pack(), canonical_pack);
    assert_eq!(witness.record.decision_sha256(), decision_sha256);
    assert_eq!(receipt.transaction_id, 1);
}

#[semio_framework_async_macros::async_test]
async fn committed_durable_group_decision_accepts_only_one_exact_event_transaction() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📓️durable-group-journal/🔣️.json")).unwrap();
    let document = ArtifactId::from(fixture["record"]["document"].as_str().unwrap());
    let record = store::durable_group::durable_owned_group_journal_test_record(include_str!("../../../../../🏪️store/🧩️composition/🗄️durable-group/🧪️tests/🧫️fixtures/🔣️.json"));
    let canonical_pack = record.canonical_pack().to_vec();
    let backing = storage().await;
    let wal_storage = backing.wal().await;
    let mut wal = db_wal::ArtifactWal::create(&wal_storage, document.clone(), db_wal_log::GroupCommitPolicy::default(), 0).await.unwrap();
    for (ordinal, row) in fixture["committedDecisionWitnessCases"].as_array().unwrap().iter().filter(|row| row["transaction"] == "committed").enumerate() {
        let mut records = durable_group_witness_batch(row["recordKinds"].as_array().unwrap(), &canonical_pack).await;
        let receipt = wal.submit(&wal_storage, &[], &records, DurabilityClass::Fsync, ordinal as u64 + 1).await.unwrap();
        assert!(receipt.committed);
        close_wal_record_batch(&mut records).await.unwrap();
    }
    wal.close().await.unwrap();

    let mut replay =
        db_wal_log::replay_committed_document(&wal_storage, &document, db_wal_log::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap())
            .await
            .unwrap();
    for row in fixture["committedDecisionWitnessCases"].as_array().unwrap().iter().filter(|row| row["transaction"] == "committed") {
        let transaction = loop {
            match replay.next_transaction_step().await.unwrap() {
                db_wal_log::WalCommittedStep::Transaction(transaction) => break transaction,
                db_wal_log::WalCommittedStep::Yield => continue,
                db_wal_log::WalCommittedStep::Done => panic!("durable witness fixture lost a committed transaction"),
            }
        };
        let replay_document = if row["replayDocument"] == "foreign" { ArtifactId::from("foreign-map") } else { document.clone() };
        let outcome = committed_durable_group_decision_from_transaction(&replay_document, transaction).await;
        match row["expected"].as_str().unwrap() {
            "witness" => {
                let witness = outcome.unwrap().expect("one exact Event must produce a committed decision witness");
                assert_eq!(witness.document(), &replay_document);
                assert_eq!(witness.record.canonical_pack(), canonical_pack);
            }
            "ignored" => assert!(outcome.unwrap().is_none()),
            "rejected" => assert!(matches!(outcome, Err(DbError::Corrupt(_)))),
            other => panic!("unknown durable witness expectation {other}"),
        }
    }
    loop {
        match replay.next_transaction_step().await.unwrap() {
            db_wal_log::WalCommittedStep::Yield => continue,
            db_wal_log::WalCommittedStep::Done => break,
            db_wal_log::WalCommittedStep::Transaction(_) => panic!("durable witness fixture replayed an unexpected committed transaction"),
        }
    }
    while replay.close_owner_step().unwrap() {}
    assert!(replay.terminal_is_empty());

    let aborted = db_wal::testkit::aborted_event_fixture_storage(&document, &canonical_pack).await;
    let mut replay =
        db_wal_log::replay_committed_document(&aborted, &document, db_wal_log::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap()).await.unwrap();
    loop {
        match replay.next_transaction_step().await.unwrap() {
            db_wal_log::WalCommittedStep::Yield => continue,
            db_wal_log::WalCommittedStep::Done => break,
            db_wal_log::WalCommittedStep::Transaction(_) => panic!("aborted durable witness fixture exposed a committed transaction"),
        }
    }
    while replay.close_owner_step().unwrap() {}
    assert!(replay.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn committed_durable_group_recovery_consumes_wal_witness_and_returns_exact_three_stores_on_pre_mutation_rejection() {
    let document = ArtifactId::from("map-a");
    let record = store::durable_group::durable_owned_group_journal_test_record(include_str!("../../../../../🏪️store/🧩️composition/🗄️durable-group/🧪️tests/🧫️fixtures/🔣️.json"));
    let anchor_sha256 = record.anchor_sha256().to_string();
    let decision_sha256 = record.decision_sha256().to_string();
    let canonical_pack = record.into_canonical_pack();
    let backing = storage().await;
    let wal_storage = backing.wal().await;
    let mut wal = db_wal::ArtifactWal::create(&wal_storage, document.clone(), db_wal_log::GroupCommitPolicy::default(), 0).await.unwrap();
    let mut records = durable_group_witness_batch(&[serde_json::Value::String("event".to_string())], &canonical_pack).await;
    wal.submit(&wal_storage, &[], &records, DurabilityClass::Fsync, 1).await.unwrap();
    close_wal_record_batch(&mut records).await.unwrap();
    wal.close().await.unwrap();
    let mut replay =
        db_wal_log::replay_committed_document(&wal_storage, &document, db_wal_log::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap())
            .await
            .unwrap();
    let transaction = loop {
        match replay.next_transaction_step().await.unwrap() {
            db_wal_log::WalCommittedStep::Transaction(transaction) => break transaction,
            db_wal_log::WalCommittedStep::Yield => continue,
            db_wal_log::WalCommittedStep::Done => panic!("committed recovery fixture lost its decision transaction"),
        }
    };
    let witness = committed_durable_group_decision_from_transaction(&document, transaction).await.unwrap().expect("one committed Event produces one opaque DB witness");
    while replay.close_owner_step().unwrap() {}

    let parent_dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.gis.gismap".into(), standard: "1".into(), subset: "*".into() };
    let child_dialect = |subset: &str| semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() };
    let parent_reference = semio_framework_artifact_reference::ArtifactRef { artifact_id: "map-a".into(), dialect: parent_dialect.clone() };
    let parent = committed_recovery_hash_store("map-a", parent_dialect, None).await;
    let drawing = committed_recovery_hash_store("gismap-drawing", child_dialect("drawing"), Some(store::OwnerRef { parent: parent_reference.clone(), slot: "drawing".into(), child_id: "gismap-drawing".into() })).await;
    let value = committed_recovery_hash_store("gismap-value", child_dialect("value"), Some(store::OwnerRef { parent: parent_reference, slot: "value".into(), child_id: "gismap-value".into() })).await;
    let parent_before = (parent.snapshot_pack().await.unwrap(), store::SpaceMember::artifact_ref(&parent), store::SpaceMember::owner_ref(&parent));
    let drawing_before = (drawing.snapshot_pack().await.unwrap(), store::SpaceMember::artifact_ref(&drawing), store::SpaceMember::owner_ref(&drawing));
    let value_before = (value.snapshot_pack().await.unwrap(), store::SpaceMember::artifact_ref(&value), store::SpaceMember::owner_ref(&value));
    let admission = store::durable_group::DurableOwnedMapRecoveryAdmissionV1::new(parent, drawing, value);
    let mut recovery = witness.into_store_owned_recovery(admission);
    assert_eq!(recovery.advance(journal_grant()), ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault(store::durable_group::DurableOwnedGroupDecisionError::InvalidFrontier));
    let mut terminal = recovery.take_rejected_terminal().expect("pre-mutation rejection consumes the witness and returns all three exact Store owners");
    assert!(recovery.terminal_is_empty());
    assert_eq!(terminal.document, document);
    assert_eq!(terminal.receipt.transaction_id, 1);
    assert_eq!(terminal.receipt.anchor_sha256, anchor_sha256);
    assert_eq!(terminal.receipt.decision_sha256, decision_sha256);
    assert_eq!(terminal.error, store::durable_group::DurableOwnedGroupDecisionError::InvalidFrontier);
    assert_eq!([terminal.owners.parent.generation(), terminal.owners.drawing.generation(), terminal.owners.value.generation()], [0, 0, 0]);
    let parent_after = terminal.owners.parent.snapshot_pack().await.unwrap();
    let drawing_after = terminal.owners.drawing.snapshot_pack().await.unwrap();
    let value_after = terminal.owners.value.snapshot_pack().await.unwrap();
    assert_eq!((parent_after.pack, parent_after.spr, store::SpaceMember::artifact_ref(&terminal.owners.parent), store::SpaceMember::owner_ref(&terminal.owners.parent)), (parent_before.0.pack, parent_before.0.spr, parent_before.1, parent_before.2));
    assert_eq!(
        (drawing_after.pack, drawing_after.spr, store::SpaceMember::artifact_ref(&terminal.owners.drawing), store::SpaceMember::owner_ref(&terminal.owners.drawing)),
        (drawing_before.0.pack, drawing_before.0.spr, drawing_before.1, drawing_before.2)
    );
    assert_eq!((value_after.pack, value_after.spr, store::SpaceMember::artifact_ref(&terminal.owners.value), store::SpaceMember::owner_ref(&terminal.owners.value)), (value_before.0.pack, value_before.0.spr, value_before.1, value_before.2));
    close_committed_recovery_hash_store(&mut terminal.owners.value);
    close_committed_recovery_hash_store(&mut terminal.owners.drawing);
    close_committed_recovery_hash_store(&mut terminal.owners.parent);
}

#[semio_framework_async_macros::async_test]
async fn document_authority_durable_group_journal_cancellation_before_handoff_is_absent() {
    let (authority, _storage, pool) = journal_authority().await;
    let record = store::durable_group::durable_owned_group_journal_test_record(include_str!("../../../../../🏪️store/🧩️composition/🗄️durable-group/🧪️tests/🧫️fixtures/🔣️.json"));
    let mut sink = authority.durable_group_journal_sink(9);
    let mut commit = sink.begin_commit(record.canonical_pack().to_vec(), record.decision_sha256().to_string());
    commit.cancel();
    assert_eq!(commit.advance(journal_grant()).unwrap(), store::durable_group::DurableOwnedGroupJournalAdvanceV1::Absent);
    close_journal_commit(commit.as_mut());
    drop(commit);
    drop(sink);
    shutdown_journal_authority(&authority, &pool).await;
}

#[semio_framework_async_macros::async_test]
async fn document_authority_durable_group_journal_rejects_hash_before_mailbox() {
    let (authority, _storage, pool) = journal_authority().await;
    let record = store::durable_group::durable_owned_group_journal_test_record(include_str!("../../../../../🏪️store/🧩️composition/🗄️durable-group/🧪️tests/🧫️fixtures/🔣️.json"));
    let mut sink = authority.durable_group_journal_sink(11);
    let mut commit = sink.begin_commit(record.canonical_pack().to_vec(), "0".repeat(64));
    assert!(matches!(commit.advance(journal_grant()).unwrap(), store::durable_group::DurableOwnedGroupJournalAdvanceV1::Rejected(_)));
    close_journal_commit(commit.as_mut());
    drop(commit);
    drop(sink);
    shutdown_journal_authority(&authority, &pool).await;
}

#[semio_framework_async_macros::async_test]
async fn document_authority_submits_and_queries_over_finite_pool_turns() {
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)));
    let storage = storage().await;
    let document = document_id().await;
    let authority = ArtifactAuthority::spawn(pool.clone(), move || async move { ArtifactEngine::create_retained(document, storage, ArtifactEngineConfig::default(), 0).await.map(Box::new) }, MailboxCapacities::uniform(16)).await.unwrap();

    let batch = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("name", serde_json::json!("hi"))]).await]).await.unwrap();
    let receipt = authority.submit(batch, SubmitOptions::default(), 0).await.unwrap();
    assert_eq!(receipt.frontier.head_seq, 1);

    let queried: serde_json::Value = stored_json(authority.query("name").await.unwrap().unwrap()).await;
    assert_eq!(queried, serde_json::json!("hi"));

    let frontier = authority.frontier().await.unwrap();
    assert_eq!(frontier.head_seq, 1);

    let generation = authority.snapshot_now(1).await.unwrap();
    assert_eq!(generation, 0);

    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while !authority.shutdown_step() {
        assert!(std::time::Instant::now() < deadline, "authority close parked: {}", authority.shutdown_debug_witness());
        semio_framework_async::yield_once().await;
    }
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn document_authority_close_drains_pending_group_commit_for_every_durability_class() {
    for durability in [DurabilityClass::Memory, DurabilityClass::Os, DurabilityClass::Fsync] {
        let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)));
        let storage = storage().await;
        let reopen_storage = storage.clone();
        let document = document_id().await;
        let reopen_document = document.clone();
        let authority = ArtifactAuthority::spawn(pool.clone(), move || async move { ArtifactEngine::create_retained(document, storage, ArtifactEngineConfig::default(), 0).await.map(Box::new) }, MailboxCapacities::uniform(16)).await.unwrap();
        let batch = CommandBatch::new(vec![envelope("close-drain", &[], "alice", &[("name", serde_json::json!("kept"))]).await]).await.unwrap();
        let receipt = authority.submit(batch, SubmitOptions { durability, ..Default::default() }, 0).await.unwrap();
        assert_eq!(receipt.frontier.head_seq, 1);
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while !authority.shutdown_step() {
            assert!(std::time::Instant::now() < deadline, "{durability:?} close parked: {}", authority.shutdown_debug_witness());
            semio_framework_async::yield_once().await;
        }
        assert!(authority.handoff.close_error.lock().unwrap().is_none());
        drop(authority);
        let (engine, report) = ArtifactEngine::open_retained(reopen_document, reopen_storage, ArtifactEngineConfig::default(), 1).await.unwrap();
        assert_eq!(report.torn_tail_bytes, 0);
        assert_eq!(engine.frontier.head_seq, 1, "{durability:?} close must make the acknowledged transaction durable");
        let mut engine = engine;
        engine.close().await.unwrap();
        assert_eq!(pool.shutdown(), Ok(()));
    }
}

#[semio_framework_async_macros::async_test]
async fn document_authority_spawn_propagates_a_build_failure_synchronously() {
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)));
    let result = ArtifactAuthority::spawn(
        pool.clone(),
        || async { Err::<Box<ArtifactEngine>, ArtifactEngineOpenRejected>(ArtifactEngineOpenRejected::BeforeWal(DbError::InvalidArgument("boom".to_string()))) },
        MailboxCapacities::uniform(4),
    );
    let rejected = match result.await {
        Err(rejected) => rejected,
        Ok(_) => panic!("artifact authority admitted a rejected builder"),
    };
    assert!(matches!(rejected_engine_open_error(rejected).await, DbError::InvalidArgument(_)));
    pool.shutdown();
}

//#endregion 🔖️Actor

//#region 🔖️Source laws
#[test]
fn artifact_runner_one_grant_polls_one_turn_and_never_blocks_on() {
    let source = ARTIFACT_SOURCE;
    let runner = &source[source.find("type ArtifactBuildFuture").unwrap()..source.find("//#region 🧪️Tests").unwrap()];
    assert!(!runner.contains("block_on("));
    assert!(!runner.contains("ask_blocking"));
    assert!(runner.contains("future.as_mut().poll(&mut context)"));
    assert!(runner.contains("Self::start_turn(engine, envelope.payload)"));
    assert!(runner.contains("let closed ="));
    assert!(runner.contains("self.clone().run_turn(generation, true);"));
}

#[test]
fn artifact_history_runner_close_mid_turn_retains_replay_until_terminal_empty() {
    let artifact = ARTIFACT_SOURCE;
    let runner = &artifact[artifact.find("enum ArtifactTurn").unwrap()..artifact.find("//#region 🧪️Tests").unwrap()];
    assert!(runner.contains("ArtifactTurn::History"));
    assert!(runner.contains("replay.request_close(DbError::Closed)"));
    assert!(runner.contains("!replay.terminal_is_empty()"));
    assert!(runner.contains("Pin::new(&mut *replay).poll(&mut context)"));
    let panicked = runner.find("history replay cursor panicked").unwrap();
    let panic_close = &runner[runner[..panicked].rfind("Err(_) =>").unwrap()..];
    assert!(panic_close.contains("replay.request_close"));
    assert!(panic_close.contains("self.schedule()"));
    assert!(!runner.contains("turn.take();\n                    drop(turn);\n                    self.address.close();"));
}

#[test]
fn retained_retry_owners_release_their_guard_before_resubmission() {
    for (name, source) in [("db_artifact", ARTIFACT_SOURCE)] {
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

#[test]
fn artifact_runner_polls_the_history_replay_future_once_per_grant() {
    assert!(ARTIFACT_SOURCE.contains("future.as_mut().poll(&mut context)"));
}
//#endregion 🔖️Source laws

/// 🧾️ The authored source of the artifact crate: every child module and then the root, in module order, as the one text the source laws scan.
const ARTIFACT_SOURCE: &str = concat!(
    include_str!("../../../🧾️authority-and-journal-sink/🦀️.rs"),
    include_str!("../../../🔁️close-retry-and-runner-handoff/🦀️.rs"),
    include_str!("../../../🧹️runner-retirement-and-poll/🦀️.rs"),
    include_str!("../../../🏃️runner-turns/🦀️.rs"),
    include_str!("../../../🎭️authority-impl-and-terminal-job/🦀️.rs"),
    include_str!("../../../🦀️.rs"),
);
