use super::*;

#[semio_framework_async_macros::async_test]
async fn cli_verify_checks_neutral_logical_commit_boundaries() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../📝️wal/🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    for (name, expected) in [("aborted-commands-snapshot-cas-have-no-effects", Some(0)), ("two-commands-only-after-logical-commit", Some(2)), ("wrong-commit-count", None), ("active-incomplete-needs-durable-abort", None)] {
        let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == name).expect("registered neutral CLI boundary");
        let document = db::db_ids::ArtifactId::from("committed-cli");
        let memory = db::wal::tests::committed_fixture_storage(row, &document).await;
        let root = tempdir(name).await;
        let storage = db::storage::FsStorage::open(db::storage::db_io_test_pool(), &root).await.unwrap();
        let writer_permit = storage.acquire_writer(&document).await.unwrap();
        storage.create_segment(&writer_permit, 0).await.unwrap();
        let len = memory.segment_len(&document, 0).await.unwrap();
        let mut pages = memory.read(&document, 0, pack::ByteRange { offset: 0, len }).await.unwrap();
        let mut writer = db::storage::DbIoPageWriter::try_reserve(pages.len().div_ceil(db::storage::DB_IO_PAGE_BYTES)).unwrap();
        for fragment in pages.fragments() {
            let mut remaining = fragment;
            while !remaining.is_empty() {
                let copied = writer.write_fragment(remaining).unwrap();
                assert_ne!(copied, 0);
                remaining = &remaining[copied..];
            }
        }
        while pages.close_step().unwrap().is_some() {}
        assert_eq!(storage.append(&writer_permit, 0, writer.seal_retained().await.unwrap()).await.unwrap(), len);
        storage.sync(&writer_permit, 0, db::DurabilityClass::Fsync).await.unwrap();
        writer_permit.release().await.unwrap();
        let result = verify_document(&storage, &document).await;
        match expected {
            Some(count) => assert_eq!(result.unwrap(), format!("wal committed records={count} snapshot=none")),
            None => assert!(matches!(result, Err(db::DbError::Corrupt(_))), "{name}"),
        }
        assert_eq!(storage.segment_len(&document, 0).await.unwrap(), len);
    }
}

//#region 🧸️Fixtures
async fn tempdir(name: &str) -> std::path::PathBuf {
    let mut dir = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
    dir.push(format!("db_cli-test-{name}-{}-{}", std::process::id(), now_ms().await));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

async fn test_envelope(id: &str, document: &protocol::ArtifactId) -> protocol::MutationEnvelope {
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(id.to_string()),
        document_id: document.clone(),
        actor: protocol::ActorId("tester".to_string()),
        dependencies: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()), payload: db::document::encode_pathmap_json(&serde_json::json!({"greeting": "hello"})).await.unwrap() },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()), payload: db::document::encode_pathmap_json(&serde_json::json!({"greeting": null})).await.unwrap() },
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
    }
}

/// 🌱️ Seeds `doc-1` at `root` with one committed, `Fsync`-durable transaction through the real
/// `Database::create_document`/`ArtifactHandle::submit` round trip, then cleanly shuts down.
async fn seed_document(root: &Path) {
    let mut database = open_database(root, db::Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    let handle = database.create_document(db::ArtifactSpec::new(document.clone()).await).await.unwrap();
    let batch = db::document::CommandBatch::new(vec![test_envelope("op-1", &document).await]).await.unwrap();
    db::actor::block_on(handle.submit(batch, db::document::SubmitOptions { durability: db::DurabilityClass::Fsync, ..Default::default() })).unwrap().unwrap();
    drop(handle);
    database.shutdown(&db::DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(1))).await.unwrap();
}
//#endregion 🧸️Fixtures

//#region 🔖️Inspect
#[semio_framework_async_macros::async_test]
async fn cli_inspect_reports_an_empty_catalog_and_healthy_status_on_a_fresh_root() {
    let root = tempdir("inspect-fresh").await;
    assert_eq!(main_impl(&[String::from("inspect"), root.to_string_lossy().to_string()]).await, 0);
}
//#endregion 🔖️Inspect

//#region 🔖️FullCycle
#[semio_framework_async_macros::async_test]
async fn cli_full_cycle_succeeds_for_a_seeded_document() {
    let root = tempdir("full-cycle").await;
    seed_document(&root).await;
    let root_str = root.to_string_lossy().to_string();

    assert_eq!(main_impl(&[String::from("doc"), root_str.clone(), String::from("doc-1")]).await, 0);
    assert_eq!(main_impl(&[String::from("query"), root_str.clone(), String::from("doc-1"), String::from("greeting")]).await, 0);
    assert_eq!(main_impl(&[String::from("wal-inspect"), root_str.clone(), String::from("doc-1")]).await, 0);
    assert_eq!(main_impl(&[String::from("replay"), root_str.clone(), String::from("doc-1")]).await, 0);
    assert_eq!(main_impl(&[String::from("verify"), root_str.clone(), String::from("doc-1")]).await, 0);
    assert_eq!(main_impl(&[String::from("verify"), root_str.clone()]).await, 0);
    assert_eq!(main_impl(&[String::from("repair"), root_str.clone(), String::from("doc-1")]).await, 0);
    assert_eq!(main_impl(&[String::from("compact"), root_str.clone(), String::from("doc-1"), String::from("--consolidate")]).await, 0);
    assert_eq!(main_impl(&[String::from("health"), root_str.clone()]).await, 0);
    assert_eq!(main_impl(&[String::from("snapshot-inspect"), root_str, String::from("doc-1")]).await, 0);
}

#[semio_framework_async_macros::async_test]
async fn cli_doc_and_query_err_cleanly_on_an_unknown_document() {
    let root = tempdir("unknown-doc").await;
    let root_str = root.to_string_lossy().to_string();
    assert_eq!(main_impl(&[String::from("doc"), root_str.clone(), String::from("never-created")]).await, 1);
    assert_eq!(main_impl(&[String::from("query"), root_str, String::from("never-created"), String::from("x")]).await, 1);
}
//#endregion 🔖️FullCycle

//#region 🔖️Verify
#[semio_framework_async_macros::async_test]
async fn cli_verify_fails_on_a_torn_wal_tail_and_repair_fixes_it() {
    let root = tempdir("torn-tail").await;
    seed_document(&root).await;

    let wal_dir = root.join("wal").join("doc-1");
    let segment_path = std::fs::read_dir(&wal_dir).unwrap().filter_map(|entry| entry.ok()).map(|entry| entry.path()).find(|path| path.extension().is_some_and(|ext| ext == "bin")).expect("expected at least one wal segment file");
    let mut bytes = std::fs::read(&segment_path).unwrap();
    assert!(bytes.len() > 16, "segment must be large enough to truncate meaningfully");
    bytes.truncate(bytes.len() - 5);
    std::fs::write(&segment_path, &bytes).unwrap();

    let root_str = root.to_string_lossy().to_string();
    assert_eq!(main_impl(&[String::from("verify"), root_str.clone(), String::from("doc-1")]).await, 1);
    assert_eq!(main_impl(&[String::from("wal-inspect"), root_str.clone(), String::from("doc-1")]).await, 1);
    assert_eq!(main_impl(&[String::from("repair"), root_str.clone(), String::from("doc-1")]).await, 0);
    assert_eq!(main_impl(&[String::from("verify"), root_str, String::from("doc-1")]).await, 0);
}
//#endregion 🔖️Verify

//#region 🔖️ConflictSimulate
#[semio_framework_async_macros::async_test]
async fn cli_conflict_simulate_detects_overlapping_writes_and_ignores_disjoint_ones() {
    assert_eq!(main_impl(&[String::from("conflict-simulate"), String::from("--touch-a"), String::from("a/name"), String::from("--touch-b"), String::from("a/name")]).await, 1);
    assert_eq!(main_impl(&[String::from("conflict-simulate"), String::from("--touch-a"), String::from("a/name"), String::from("--touch-b"), String::from("b/name")]).await, 0);
}

#[semio_framework_async_macros::async_test]
async fn cli_conflict_simulate_requires_both_touch_flags() {
    assert_eq!(main_impl(&[String::from("conflict-simulate")]).await, 2);
    assert_eq!(main_impl(&[String::from("conflict-simulate"), String::from("--touch-a"), String::from("a")]).await, 2);
}
//#endregion 🔖️ConflictSimulate

//#region 🔖️ReplicaSimulate
#[semio_framework_async_macros::async_test]
async fn cli_replica_simulate_copies_missing_commands_to_a_fresh_follower() {
    let leader_root = tempdir("replica-leader").await;
    let follower_root = tempdir("replica-follower").await;
    seed_document(&leader_root).await;

    let leader_str = leader_root.to_string_lossy().to_string();
    let follower_str = follower_root.to_string_lossy().to_string();
    assert_eq!(main_impl(&[String::from("replica-simulate"), leader_str, follower_str.clone(), String::from("doc-1")]).await, 0);
    assert_eq!(main_impl(&[String::from("verify"), follower_str, String::from("doc-1")]).await, 0);
}
//#endregion 🔖️ReplicaSimulate

//#region 🔖️Migrate
#[semio_framework_async_macros::async_test]
async fn cli_migrate_appends_a_migration_record_visible_to_wal_inspect() {
    let root = tempdir("migrate").await;
    let root_str = root.to_string_lossy().to_string();
    assert_eq!(main_impl(&[String::from("migrate"), root_str.clone(), String::from("doc-1"), String::from("rename-field"), String::from("--payload"), String::from("old->new")]).await, 0);
    assert_eq!(main_impl(&[String::from("wal-inspect"), root_str, String::from("doc-1")]).await, 0);
}

#[semio_framework_async_macros::async_test]
async fn cli_migrate_reports_a_usage_error_with_too_few_args() {
    assert_eq!(main_impl(&[String::from("migrate"), String::from("root-only")]).await, 2);
}
//#endregion 🔖️Migrate

//#region 🔖️Profile
#[semio_framework_async_macros::async_test]
async fn cli_profile_reports_throughput_for_n_commands_on_a_fresh_document() {
    let root = tempdir("profile").await;
    let root_str = root.to_string_lossy().to_string();
    assert_eq!(main_impl(&[String::from("profile"), root_str.clone(), String::from("doc-1"), String::from("--commands"), String::from("5"), String::from("--durability"), String::from("memory")]).await, 0);
    // 🎯️ The 5 profiled commands are real, durable commits — verified via the query subcommand
    // rather than parsing this test's own stdout (`println!` isn't easily captured in-process).
    assert_eq!(main_impl(&[String::from("query"), root_str, String::from("doc-1"), String::from("cli/profile/counter")]).await, 0);
}

#[semio_framework_async_macros::async_test]
async fn cli_profile_rejects_a_bad_durability_flag() {
    let root = tempdir("profile-bad-durability").await;
    let root_str = root.to_string_lossy().to_string();
    assert_eq!(main_impl(&[String::from("profile"), root_str, String::from("doc-1"), String::from("--durability"), String::from("bogus")]).await, 2);
}
//#endregion 🔖️Profile

//#region 🔖️Cli
#[semio_framework_async_macros::async_test]
async fn cli_help_and_unknown_subcommand() {
    assert_eq!(main_impl(&[]).await, 2);
    assert_eq!(main_impl(&[String::from("help")]).await, 0);
    assert_eq!(main_impl(&[String::from("--help")]).await, 0);
    assert_eq!(main_impl(&[String::from("bogus-subcommand")]).await, 2);
}

#[semio_framework_async_macros::async_test]
async fn cli_command_close_success_refusal_cancel_stale_fault_drop_interrupted_and_max_plus_one_have_exact_exit_witnesses() {
    let waker = std::task::Waker::noop();
    let context = &mut std::task::Context::from_waker(waker);
    let mut record = MountedWalRecordCommandClose::new(db::wal::WalRecord::VcsRef(db::storage::DbIoText::try_from_str("retained-cli-record").unwrap()));
    assert!(matches!(std::future::Future::poll(std::pin::Pin::new(&mut record), context), std::task::Poll::Pending));
    assert!(matches!(std::future::Future::poll(std::pin::Pin::new(&mut record), context), std::task::Poll::Ready(Ok(CliCommandCloseWitness { exit: CliCommandCloseExit::Closed, opportunities: 2 }))));
    assert!(matches!(std::future::Future::poll(std::pin::Pin::new(&mut record), context), std::task::Poll::Ready(Ok(CliCommandCloseWitness { exit: CliCommandCloseExit::Closed, opportunities: 2 }))));
    drop(record);
    let mut batch = db::wal::WalRecordBatch::new();
    assert!(batch.push(db::wal::WalRecord::TxBegin { tx_id: 2 }).is_ok());
    assert!(batch.push(db::wal::WalRecord::TxCommit { tx_id: 2, record_count: 0 }).is_ok());
    let mut batch_close = MountedWalBatchCommandClose::new(batch);
    assert!(matches!(std::future::Future::poll(std::pin::Pin::new(&mut batch_close), context), std::task::Poll::Pending));
    assert!(matches!(std::future::Future::poll(std::pin::Pin::new(&mut batch_close), context), std::task::Poll::Pending));
    assert!(matches!(std::future::Future::poll(std::pin::Pin::new(&mut batch_close), context), std::task::Poll::Ready(Ok(CliCommandCloseWitness { exit: CliCommandCloseExit::Closed, opportunities: 3 }))));
    assert!(matches!(std::future::Future::poll(std::pin::Pin::new(&mut batch_close), context), std::task::Poll::Ready(Ok(CliCommandCloseWitness { exit: CliCommandCloseExit::Closed, opportunities: 3 }))));
    drop(batch_close);
    let fault = db::DbError::Internal("retained CLI fault witness".to_string());
    let mut record_fault = MountedWalRecordCommandClose { owner: None, opportunities: 7, terminal: Some(CliCommandCloseTerminal::Fault(fault.clone())) };
    assert!(matches!(std::future::Future::poll(std::pin::Pin::new(&mut record_fault), context), std::task::Poll::Ready(Err(error)) if error == fault));
    assert!(matches!(std::future::Future::poll(std::pin::Pin::new(&mut record_fault), context), std::task::Poll::Ready(Err(error)) if error == fault));
    let mut batch_fault = MountedWalBatchCommandClose { owner: None, opportunities: 7, terminal: Some(CliCommandCloseTerminal::Fault(fault.clone())) };
    assert!(matches!(std::future::Future::poll(std::pin::Pin::new(&mut batch_fault), context), std::task::Poll::Ready(Err(error)) if error == fault));
    assert!(matches!(std::future::Future::poll(std::pin::Pin::new(&mut batch_fault), context), std::task::Poll::Ready(Err(error)) if error == fault));
    let mut replay_fault: MountedWalReplayCommandClose<'static> = MountedWalReplayCommandClose { owner: None, opportunities: 7, terminal: Some(CliCommandCloseTerminal::Fault(fault.clone())) };
    assert!(matches!(std::future::Future::poll(std::pin::Pin::new(&mut replay_fault), context), std::task::Poll::Ready(Err(error)) if error == fault));
    assert!(matches!(std::future::Future::poll(std::pin::Pin::new(&mut replay_fault), context), std::task::Poll::Ready(Err(error)) if error == fault));
    let mut snapshot_fault: MountedSnapshotCommandClose<'static, 'static> = MountedSnapshotCommandClose { owner: None, opportunities: 7, terminal: Some(CliCommandCloseTerminal::Fault(fault.clone())) };
    assert!(matches!(std::future::Future::poll(std::pin::Pin::new(&mut snapshot_fault), context), std::task::Poll::Ready(Err(error)) if error == fault));
    assert!(matches!(std::future::Future::poll(std::pin::Pin::new(&mut snapshot_fault), context), std::task::Poll::Ready(Err(error)) if error == fault));
    let interrupted = CLI_COMMAND_CLOSE_INTERRUPTED.load(std::sync::atomic::Ordering::Acquire);
    let owner = MountedWalRecordCommandClose::new(db::wal::WalRecord::TxAbort { tx_id: 3 });
    drop(owner);
    assert_eq!(CLI_COMMAND_CLOSE_INTERRUPTED.load(std::sync::atomic::Ordering::Acquire), interrupted + 1);
    let mut full = db::wal::WalRecordBatch::new();
    for tx_id in 0..64 {
        assert!(full.push(db::wal::WalRecord::TxBegin { tx_id }).is_ok());
    }
    let refusal = full.push(db::wal::WalRecord::TxAbort { tx_id: 65 }).unwrap_err();
    assert_eq!(MountedWalRecordCommandClose::new(refusal).await.unwrap().exit, CliCommandCloseExit::Closed);
    assert_eq!(MountedWalBatchCommandClose::new(full).await.unwrap().opportunities, 65);
    assert_eq!(cli_command_close_witness(CliCommandCloseExit::Fault, 1).exit, CliCommandCloseExit::Fault);
    assert_eq!(db::wal::WalRecordBatch::new().len(), 0);
}
//#endregion 🔖️Cli
