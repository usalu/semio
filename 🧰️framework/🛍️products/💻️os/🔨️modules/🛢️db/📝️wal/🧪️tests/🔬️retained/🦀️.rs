
use super::*;
use crate::db_storage::WalStorage;

fn fail_stop_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🛑️fail-stop/🔣️.json")).unwrap()
}

async fn fail_stop_segment_bytes(storage: &impl WalStorage, document: &ArtifactId) -> Vec<u8> {
    let len = storage.segment_len(document, 0).await.unwrap();
    let mut pages = storage.read(document, 0, pack::ByteRange { offset: 0, len }).await.unwrap();
    let mut bytes = Vec::with_capacity(len as usize);
    for fragment in pages.fragments() {
        bytes.extend_from_slice(fragment);
    }
    while pages.close_step().unwrap().is_some() {
        semio_framework_async::yield_once().await;
    }
    bytes
}

async fn assert_artifact_wal_fail_stop_case(name: &str) {
    let fixture = fail_stop_fixture();
    let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["name"] == name).unwrap();
    let inner = std::sync::Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
    let storage = crate::db_testkit::FaultStorage::new(inner.clone()).await;
    let document = ArtifactId::from(format!("retained-artifact-wal-fail-stop-{name}"));
    let mut wal = ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).await.unwrap();
    let baseline = fail_stop_segment_bytes(&storage, &document).await;
    let append_boundary = storage.append_calls().await + 1;
    let sync_boundary = storage.sync_calls().await + 1;
    let mut script = crate::db_testkit::FaultScript::default();
    match case["fault"].as_str().unwrap() {
        "shortAppend" => script.torn_write_at = Some((append_boundary, case["keepBytes"].as_u64().unwrap())),
        "appendError" => script.fail_nth_write = Some(append_boundary),
        "syncError" => script.fail_nth_sync = Some(sync_boundary),
        fault => panic!("unknown WAL fail-stop fixture fault {fault}"),
    }
    storage.set_script(script).await;

    let mut admission = WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 128).unwrap();
    let command = WalBytes::try_admit(format!("fault-{name}").into_bytes(), 64, &mut admission).await.unwrap();
    let mut batch = WalRecordBatch::new();
    assert!(batch.push(WalRecord::Command(command)).is_ok());
    let error = wal.submit(&storage, &batch, DurabilityClass::Fsync, 1).await.unwrap_err();
    match case["expectedError"].as_str().unwrap() {
        "Corrupt" => assert!(matches!(error, DbError::Corrupt(message) if message.contains("append returned segment length"))),
        "Io" => assert!(matches!(error, DbError::Io(_))),
        expected => panic!("unknown WAL fail-stop fixture error {expected}"),
    }
    let append_calls = storage.append_calls().await;
    let sync_calls = storage.sync_calls().await;
    assert!(matches!(wal.submit(&storage, &batch, DurabilityClass::Fsync, 2).await, Err(DbError::Closed)));
    assert!(matches!(wal.force_flush(&storage).await, Err(DbError::Closed)));
    assert!(matches!(wal.rotate(&storage, 2).await, Err(DbError::Closed)));
    assert_eq!(storage.append_calls().await, append_calls, "a poisoned writer must never retry its uncertain suffix");
    assert_eq!(storage.sync_calls().await, sync_calls, "a poisoned writer must never retry sync");
    while batch.close_step().unwrap() {}

    let after_failure = fail_stop_segment_bytes(&storage, &document).await;
    match case["expectedPhysicalSuffix"].as_str().unwrap() {
        "absent" => assert_eq!(after_failure, baseline),
        "torn" => {
            assert!(after_failure.starts_with(&baseline));
            assert_eq!(after_failure.len(), baseline.len() + case["keepBytes"].as_u64().unwrap() as usize);
        }
        "complete" => {
            assert!(after_failure.starts_with(&baseline));
            assert!(after_failure.len() > baseline.len());
        }
        suffix => panic!("unknown WAL fail-stop fixture suffix {suffix}"),
    }
    while wal.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    assert!(wal.terminal_is_empty());

    let inner_wal = inner.wal().await;
    let (mut reopened, report) = ArtifactWal::open(&inner_wal, document.clone(), GroupCommitPolicy::default(), 3).await.unwrap();
    let recovered = fail_stop_segment_bytes(&inner_wal, &document).await;
    if case["expectedPhysicalSuffix"] == "complete" {
        assert_eq!(recovered, after_failure, "a sync error must not duplicate its already-appended complete commit");
        assert_eq!(report.torn_tail_bytes, 0);
    } else {
        assert_eq!(recovered, baseline, "reopen must retain exactly the last complete prefix");
        assert_eq!(report.torn_tail_bytes, after_failure.len() as u64 - baseline.len() as u64);
    }
    while reopened.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    assert!(reopened.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn wal_bytes_exact_backing_handback_cancel_and_close_are_one_owner() {
    let _pool = crate::db_storage::db_io_test_pool();
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let mut control = WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 64).unwrap();
    let mut source = Vec::with_capacity(32);
    source.push(0xA5);
    let mut retained = WalBytes::try_admit(source, 32, &mut control).await.unwrap();
    assert_eq!(retained.len(), 1);
    while retained.close_step().unwrap().is_some() {}
    assert!(retained.terminal_is_empty());

    let mut source = Vec::with_capacity(33);
    source.push(0x5A);
    let pointer = source.as_ptr();
    let rejected = WalBytes::try_admit(source, 32, &mut control).await.unwrap_err();
    let returned = rejected.into_source().unwrap();
    assert_eq!(returned.as_ptr(), pointer);
    assert_eq!(returned.capacity(), 33);

    cancelled.store(true, std::sync::atomic::Ordering::Release);
    let mut source = Vec::with_capacity(8);
    source.push(1);
    let pointer = source.as_ptr();
    let mut rejected = WalBytes::try_admit(source, 8, &mut control).await.unwrap_err();
    while rejected.close_step().unwrap() {}
    let returned = rejected.into_source().unwrap();
    assert_eq!(returned.as_ptr(), pointer);

    cancelled.store(false, std::sync::atomic::Ordering::Release);
    let mut deadline_control = WalCursorControl::new(cancelled, std::time::Instant::now(), 16).unwrap();
    let mut source = Vec::with_capacity(2);
    source.push(0x33);
    let pointer = source.as_ptr();
    let mut rejected = WalBytes::try_admit(source, 2, &mut deadline_control).await.unwrap_err();
    assert!(matches!(rejected.error(), DbError::Unavailable(message) if message == "wal cursor deadline reached"));
    while rejected.close_step().unwrap() {}
    assert_eq!(rejected.into_source().unwrap().as_ptr(), pointer);
}

#[semio_framework_async_macros::async_test]
async fn wal_replay_cancel_resume_close_and_fragment_crc_are_deterministic() {
    let storage = db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
    let document = ArtifactId::from("retained-replay");
    let mut wal = ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).await.unwrap();
    let mut admission = WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_024).unwrap();
    let bytes = WalBytes::try_admit(vec![0xA5; db_storage::DB_IO_PAGE_BYTES + 1], (db_storage::DB_IO_PAGE_BYTES + 1) as u64, &mut admission).await.unwrap();
    let mut batch = WalRecordBatch::new();
    assert!(batch.push(WalRecord::Command(bytes)).is_ok());
    wal.submit(&storage, &batch, DurabilityClass::Fsync, 0).await.unwrap();
    while batch.close_step().unwrap() {}
    wal.close().await.unwrap();

    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let control = WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
    let mut replay = replay_document(&storage, &document, control).await.unwrap();
    cancelled.store(true, std::sync::atomic::Ordering::Release);
    assert!(matches!(replay.next().await, Err(DbError::Unavailable(_))));
    cancelled.store(false, std::sync::atomic::Ordering::Release);
    let mut seen = 0usize;
    let mut boundary_yields = 0usize;
    while boundary_yields < 2 {
        match replay.next_step().await.unwrap() {
            WalReplayStep::Record(mut record) => {
                seen += 1;
                while record.close_step().unwrap() {}
            }
            WalReplayStep::Yield => {
                if seen != 0 {
                    boundary_yields += 1;
                }
            }
            WalReplayStep::Done => panic!("retained replay closed without resumable segment retirement"),
        }
    }
    assert!(seen >= 1);
    while replay.close_step().await.unwrap() {}
    assert!(replay.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn wal_replay_cancellation_remains_set_while_close_reaches_terminal_empty() {
    let storage = db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
    let document = ArtifactId::from("retained-replay-cancel-close");
    let mut wal = ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).await.unwrap();
    let mut admission = WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_024).unwrap();
    let bytes = WalBytes::try_admit(vec![0x5A; db_storage::DB_IO_PAGE_BYTES + 1], (db_storage::DB_IO_PAGE_BYTES + 1) as u64, &mut admission).await.unwrap();
    let mut batch = WalRecordBatch::new();
    assert!(batch.push(WalRecord::Command(bytes)).is_ok());
    wal.submit(&storage, &batch, DurabilityClass::Fsync, 0).await.unwrap();
    while batch.close_step().unwrap() {}
    wal.close().await.unwrap();

    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let control = WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
    let mut replay = replay_document(&storage, &document, control).await.unwrap();
    assert!(matches!(replay.next_step().await.unwrap(), WalReplayStep::Yield));
    assert!(replay.pages.is_some());
    cancelled.store(true, std::sync::atomic::Ordering::Release);
    assert!(matches!(replay.next_step().await, Err(DbError::Unavailable(message)) if message == "wal cursor cancelled"));
    while replay.close_step().await.unwrap() {
        assert!(cancelled.load(std::sync::atomic::Ordering::Acquire));
    }
    assert!(cancelled.load(std::sync::atomic::Ordering::Acquire));
    assert!(replay.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn artifact_wal_repeated_open_close_is_page_budget_neutral() {
    let storage = db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
    let document = ArtifactId::from("retained-artifact-wal-close-budget");
    let mut wal = ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).await.unwrap();
    while wal.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    assert!(wal.terminal_is_empty());

    for turn in 0..18 {
        let (mut wal, _) = ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), turn as u64 + 1).await.unwrap();
        while wal.close_step().unwrap() {
            semio_framework_async::yield_once().await;
        }
        assert!(wal.terminal_is_empty());
    }

    let (mut wal, _) = ArtifactWal::open(&storage, document, GroupCommitPolicy::default(), 100).await.unwrap();
    let mut admission = WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 128).unwrap();
    let command = WalBytes::try_admit(b"after-repeated-close".to_vec(), 64, &mut admission).await.unwrap();
    let mut batch = WalRecordBatch::new();
    assert!(batch.push(WalRecord::Command(command)).is_ok());
    assert!(wal.submit(&storage, &batch, DurabilityClass::Fsync, 101).await.unwrap().committed);
    while batch.close_step().unwrap() {}
    while wal.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    assert!(wal.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn artifact_wal_close_rejects_pending_records_and_closed_writes() {
    let storage = db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
    let document = ArtifactId::from("retained-artifact-wal-pending-close");
    let policy = GroupCommitPolicy { max_delay_ms: u64::MAX, max_bytes: u64::MAX, max_records: u32::MAX };
    let mut wal = ArtifactWal::create(&storage, document, policy, 0).await.unwrap();
    let mut admission = WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 128).unwrap();
    let command = WalBytes::try_admit(b"pending".to_vec(), 64, &mut admission).await.unwrap();
    let mut batch = WalRecordBatch::new();
    assert!(batch.push(WalRecord::Command(command)).is_ok());
    assert!(!wal.submit(&storage, &batch, DurabilityClass::Memory, 1).await.unwrap().committed);
    while batch.close_step().unwrap() {}
    assert!(matches!(wal.close_step(), Err(DbError::InvalidArgument(message)) if message.contains("force_flush")));
    assert!(!wal.terminal_is_empty());
    assert!(wal.force_flush(&storage).await.unwrap());
    while wal.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    assert!(wal.terminal_is_empty());

    let empty = WalRecordBatch::new();
    assert!(matches!(wal.submit(&storage, &empty, DurabilityClass::Memory, 2).await, Err(DbError::Closed)));
    assert!(matches!(wal.force_flush(&storage).await, Err(DbError::Closed)));
    assert!(matches!(wal.rotate(&storage, 2).await, Err(DbError::Closed)));
}

#[semio_framework_async_macros::async_test]
async fn artifact_wal_short_append_is_fail_stop_until_reopen() {
    assert_artifact_wal_fail_stop_case("short-append").await;
}

#[semio_framework_async_macros::async_test]
async fn artifact_wal_append_error_is_fail_stop_until_reopen() {
    assert_artifact_wal_fail_stop_case("append-error").await;
}

#[semio_framework_async_macros::async_test]
async fn artifact_wal_sync_error_is_fail_stop_until_reopen() {
    assert_artifact_wal_fail_stop_case("sync-error").await;
}

#[semio_framework_async_macros::async_test]
async fn artifact_wal_successor_failure_after_seal_is_fail_stop_until_reopen() {
    assert!(fail_stop_fixture()["cases"].as_array().unwrap().iter().any(|case| case["fault"] == "successorAppendError"));
    let inner = std::sync::Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
    let storage = crate::db_testkit::FaultStorage::new(inner.clone()).await;
    let document = ArtifactId::from("retained-artifact-wal-successor-fail-stop");
    let mut wal = ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).await.unwrap();
    wal.max_segment_bytes = 1;
    storage.set_script(crate::db_testkit::FaultScript { fail_nth_write: Some(storage.append_calls().await + 2), ..crate::db_testkit::FaultScript::default() }).await;
    let mut admission = WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 128).unwrap();
    let command = WalBytes::try_admit(b"committed-before-successor-failure".to_vec(), 64, &mut admission).await.unwrap();
    let mut batch = WalRecordBatch::new();
    assert!(batch.push(WalRecord::Command(command)).is_ok());
    assert!(matches!(wal.submit(&storage, &batch, DurabilityClass::Fsync, 1).await, Err(DbError::Io(_))));
    while batch.close_step().unwrap() {}
    assert_eq!(storage.segment_state(&document, 0).await.unwrap(), db_storage::WalSegmentState::Sealed);
    let sealed = fail_stop_segment_bytes(&storage, &document).await;
    let append_calls = storage.append_calls().await;
    assert!(matches!(wal.force_flush(&storage).await, Err(DbError::Closed)));
    assert!(matches!(wal.rotate(&storage, 2).await, Err(DbError::Closed)));
    assert_eq!(storage.append_calls().await, append_calls);
    while wal.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    assert!(wal.terminal_is_empty());

    let inner_wal = inner.wal().await;
    let (mut reopened, _) = ArtifactWal::open(&inner_wal, document.clone(), GroupCommitPolicy::default(), 3).await.unwrap();
    assert_eq!(reopened.active_segment_index().await, 1);
    assert_eq!(fail_stop_segment_bytes(&inner_wal, &document).await, sealed);
    let control = WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
    let mut replay = replay_document(&inner_wal, &document, control).await.unwrap();
    let mut commands = 0;
    while let Some(mut record) = replay.next().await.unwrap() {
        if matches!(&record, WalRecord::Command(_)) {
            commands += 1;
        }
        while record.close_step().unwrap() {
            semio_framework_async::yield_once().await;
        }
    }
    assert_eq!(commands, 1, "reopen must expose the pre-seal transaction exactly once");
    while replay.close_step().await.unwrap() {}
    while reopened.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    assert!(reopened.terminal_is_empty());
}
