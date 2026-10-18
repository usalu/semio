/// 🛑️ Recovery abort laws: the idempotent abort of incomplete active transactions, the abort fsync across reopens, the abort fault retry, the abort cancellation storage and the abort capacity boundary.

#[semio_framework_async_macros::async_test]
async fn wal_recovery_aborts_only_incomplete_active_transactions_idempotently() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    for name in ["active-incomplete-needs-durable-abort", "active-empty-begin-needs-durable-abort", "sealed-incomplete-is-corrupt", "cross-segment-open-transaction-is-corrupt"] {
        let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == name).unwrap();
        let document = ArtifactId::from("abort-recovery");
        let storage = committed_fixture_storage(row, &document).await;
        let mut before = Vec::new();
        for index in 0..row["segments"].as_array().unwrap().len() {
            before.push((segment_bytes(&storage, &document, index as u64).await, storage.segment_state(&document, index as u64).await.unwrap()));
        }
        let opened = ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 1).await;
        if row["expected"]["accepted"] == false {
            let rejected = match opened {
                Err(rejected) => matches!(rejected_open_error(rejected).await, DbError::Corrupt(_)),
                Ok((mut wal, _)) => {
                    wal.close().await.unwrap();
                    false
                }
            };
            assert!(rejected, "{name}");
            for (index, (bytes, state)) in before.iter().enumerate() {
                assert_eq!(segment_bytes(&storage, &document, index as u64).await, *bytes, "{name}");
                assert_eq!(storage.segment_state(&document, index as u64).await.unwrap(), *state, "{name}");
            }
            continue;
        }
        let expected_abort: u64 = row["expected"]["recoverAbort"].as_str().unwrap().parse().unwrap();
        let next: u64 = row["expected"]["nextTxId"].as_str().unwrap().parse().unwrap();
        let (mut wal, report) = opened.unwrap();
        assert_eq!(report.recovered_abort_tx_id, Some(expected_abort));
        assert_eq!(wal.next_tx_id, next);
        assert_eq!(wal.active.index, 0);
        assert_eq!(report.torn_tail_bytes, 0);
        let repaired = segment_bytes(&storage, &document, 0).await;
        assert!(repaired.starts_with(&before[0].0));
        assert_eq!(repaired.len() as u64, before[0].0.len() as u64 + wal_frame_bytes(8).unwrap() + protocol::format::COMMIT_FRAME_LEN);
        assert_eq!(replay_summaries(&storage, &document).await.iter().filter(|record| **record == ReplaySummary::Abort(expected_abort)).count(), 1);
        assert_no_committed_transaction(&storage, &document).await;
        wal.close().await.unwrap();
        let (mut reopened, report) = ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 2).await.unwrap();
        assert_eq!(report.recovered_abort_tx_id, None);
        assert_eq!(segment_bytes(&storage, &document, 0).await, repaired);
        let receipt = submit_one(&storage, &mut reopened, WalRecord::Command(retained(b"after-recovery").await), DurabilityClass::Fsync, 3).await;
        assert_eq!(receipt.tx_id, next);
        reopened.close().await.unwrap();
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn wal_recovery_abort_fsync_survives_two_independent_filesystem_reopens() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    for (ordinal, name) in ["active-incomplete-needs-durable-abort", "active-empty-begin-needs-durable-abort"].iter().enumerate() {
        let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == *name).unwrap();
        let document = ArtifactId::from("abort-filesystem");
        let expected_abort = row["expected"]["recoverAbort"].as_str().unwrap().parse::<u64>().unwrap();
        let next = row["expected"]["nextTxId"].as_str().unwrap().parse::<u64>().unwrap();
        let base = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
        let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let root = base.join(format!("wal-active-abort-{}-{nonce}-{ordinal}", std::process::id()));
        {
            let filesystem = db_storage_fs::FsStorage::open(db_io_driver::db_io_test_pool(), &root).await.unwrap();
            write_committed_fixture(&filesystem, row, &document).await;
            assert_eq!(filesystem.segment_state(&document, 0).await.unwrap(), db_io_core::WalSegmentState::Active);
            filesystem.close().await.unwrap();
        }
        let repaired = {
            let filesystem = db_storage_fs::FsStorage::open(db_io_driver::db_io_test_pool(), &root).await.unwrap();
            let (mut wal, report) = ArtifactWal::open(&filesystem, document.clone(), GroupCommitPolicy::default(), 1).await.unwrap();
            assert_eq!(report.recovered_abort_tx_id, Some(expected_abort));
            assert_eq!(report.torn_tail_bytes, 0);
            assert_eq!(wal.next_tx_id, next);
            assert_eq!(replay_summaries(&filesystem, &document).await.iter().filter(|record| **record == ReplaySummary::Abort(expected_abort)).count(), 1);
            assert_no_committed_transaction(&filesystem, &document).await;
            let bytes = segment_bytes(&filesystem, &document, 0).await;
            wal.close().await.unwrap();
            filesystem.close().await.unwrap();
            bytes
        };
        {
            let filesystem = db_storage_fs::FsStorage::open(db_io_driver::db_io_test_pool(), &root).await.unwrap();
            let (mut wal, report) = ArtifactWal::open(&filesystem, document.clone(), GroupCommitPolicy::default(), 2).await.unwrap();
            assert_eq!(report.recovered_abort_tx_id, None);
            assert_eq!(report.torn_tail_bytes, 0);
            assert_eq!(segment_bytes(&filesystem, &document, 0).await, repaired);
            assert_eq!(filesystem.segment_state(&document, 0).await.unwrap(), db_io_core::WalSegmentState::Active);
            assert_eq!(replay_summaries(&filesystem, &document).await.iter().filter(|record| **record == ReplaySummary::Abort(expected_abort)).count(), 1);
            assert_no_committed_transaction(&filesystem, &document).await;
            let receipt = submit_one(&filesystem, &mut wal, WalRecord::Command(retained(b"after-filesystem-recovery").await), DurabilityClass::Fsync, 3).await;
            assert_eq!(receipt.tx_id, next);
            wal.close().await.unwrap();
            filesystem.close().await.unwrap();
        }
        eprintln!("one durable WAL abort survived independent filesystem retirement and reopen, preserving the next transaction id: {name}");
    }
}

#[semio_framework_async_macros::async_test]
async fn wal_recovery_abort_faults_retry_without_duplicate_abort() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == "active-incomplete-needs-durable-abort").unwrap();
    let faults: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛑️fail-stop/🔣️.json")).unwrap();
    for case in faults["cases"].as_array().unwrap().iter().filter(|case| case["fault"] != "successorAppendError") {
        for (tail, fail_tail_sync) in [(false, false), (true, false), (true, true)] {
            if fail_tail_sync && case["fault"] != "syncError" {
                continue;
            }
            let document = ArtifactId::from("abort-fault");
            let inner = std::sync::Arc::new(db_storage::DbBackend::Memory(committed_fixture_storage(row, &document).await));
            let storage = db_storage::FaultStorage::new(inner.clone()).await;
            let baseline = segment_bytes(&storage, &document, 0).await;
            if tail {
                let writer = storage.acquire_writer(&document).await.unwrap();
                storage.append(&writer, 0, pages(b"uncommitted-tail")).await.unwrap();
                writer.release().await.unwrap();
            }
            let append_boundary = storage.append_calls().await + 1;
            let sync_boundary = storage.sync_calls().await + if tail && !fail_tail_sync { 2 } else { 1 };
            let mut script = db_storage::FaultScript::default();
            match case["fault"].as_str().unwrap() {
                "shortAppend" => script.torn_write_at = Some((append_boundary, case["keepBytes"].as_u64().unwrap())),
                "appendError" => script.fail_nth_write = Some(append_boundary),
                "syncError" => script.fail_nth_sync = Some(sync_boundary),
                _ => unreachable!(),
            }
            storage.set_script(script).await;
            let error = match ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 1).await {
                Err(rejected) => rejected_open_error(rejected).await,
                Ok((mut wal, _)) => {
                    wal.close().await.unwrap();
                    panic!("abort recovery ignored injected fault");
                }
            };
            match case["expectedError"].as_str().unwrap() {
                "Corrupt" => assert!(matches!(error, DbError::Corrupt(_))),
                "Io" => assert!(matches!(error, DbError::Io(_))),
                _ => unreachable!(),
            }
            let failed = segment_bytes(&storage, &document, 0).await;
            let suffix = if fail_tail_sync { "absent" } else { case["expectedPhysicalSuffix"].as_str().unwrap() };
            match suffix {
                "absent" => assert_eq!(failed, baseline),
                "torn" => {
                    assert!(failed.starts_with(&baseline));
                    assert_eq!(failed.len(), baseline.len() + case["keepBytes"].as_u64().unwrap() as usize);
                }
                "complete" => assert!(failed.len() > baseline.len() && failed.starts_with(&baseline)),
                _ => unreachable!(),
            }
            let facet = inner.wal().await;
            let (mut wal, report) = ArtifactWal::open(&facet, document.clone(), GroupCommitPolicy::default(), 2).await.unwrap();
            let repaired = segment_bytes(&facet, &document, 0).await;
            if suffix == "complete" {
                assert_eq!(repaired, failed);
                assert_eq!(report.recovered_abort_tx_id, None);
            } else {
                assert_eq!(report.recovered_abort_tx_id, Some(7));
            }
            assert_no_committed_transaction(&facet, &document).await;
            assert_eq!(replay_summaries(&facet, &document).await.iter().filter(|record| **record == ReplaySummary::Abort(7)).count(), 1);
            wal.close().await.unwrap();
            let (mut wal, report) = ArtifactWal::open(&facet, document.clone(), GroupCommitPolicy::default(), 3).await.unwrap();
            assert_eq!(report.recovered_abort_tx_id, None);
            assert_eq!(segment_bytes(&facet, &document, 0).await, repaired);
            wal.close().await.unwrap();
            eprintln!("WAL abort fault retired owners and reopened idempotently: {}, tail={tail}, fail_tail_sync={fail_tail_sync}", case["name"]);
        }
    }
}

struct AbortCancellationStorage<'a> {
    inner: &'a MemoryStorage,
    pub cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    cancel_on_truncate: bool,
    synced: std::sync::atomic::AtomicUsize,
}

impl WalStorage for AbortCancellationStorage<'_> {
    async fn acquire_writer(&self, document: &ArtifactId) -> Result<db_io_core::WalWriterPermit, DbError> {
        self.inner.acquire_writer(document).await
    }
    async fn create_segment(&self, writer: &db_io_core::WalWriterPermit, index: u64) -> Result<(), DbError> {
        self.inner.create_segment(writer, index).await
    }
    async fn append(&self, writer: &db_io_core::WalWriterPermit, index: u64, bytes: db_io_core::DbIoPages) -> Result<u64, DbError> {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.inner.append(writer, index, bytes).await
    }
    async fn sync(&self, writer: &db_io_core::WalWriterPermit, index: u64, class: DurabilityClass) -> Result<(), DbError> {
        let result = self.inner.sync(writer, index, class).await;
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) && class == DurabilityClass::Fsync && result.is_ok() {
            self.synced.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        }
        result
    }
    async fn seal(&self, writer: &db_io_core::WalWriterPermit, index: u64) -> Result<(), DbError> {
        self.inner.seal(writer, index).await
    }
    async fn read(&self, document: &ArtifactId, index: u64, range: pack::ByteRange) -> Result<db_io_core::DbIoPages, DbError> {
        self.inner.read(document, index, range).await
    }
    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        self.inner.segment_len(document, index).await
    }
    async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<db_io_core::WalSegmentState, DbError> {
        self.inner.segment_state(document, index).await
    }
    async fn list_segments(&self, document: &ArtifactId) -> Result<db_io_core::DbIoU64List, DbError> {
        self.inner.list_segments(document).await
    }
    async fn truncate_tail(&self, writer: &db_io_core::WalWriterPermit, index: u64, len: u64) -> Result<(), DbError> {
        if self.cancel_on_truncate {
            self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        }
        self.inner.truncate_tail(writer, index, len).await
    }
    async fn delete_segment(&self, writer: &db_io_core::WalWriterPermit, index: u64) -> Result<(), DbError> {
        self.inner.delete_segment(writer, index).await
    }
}

#[semio_framework_async_macros::async_test]
async fn wal_recovery_abort_cancellation_has_one_durable_boundary() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == "active-incomplete-needs-durable-abort").unwrap();
    let document = ArtifactId::from("abort-cancel");
    let storage = committed_fixture_storage(row, &document).await;
    let writer = storage.acquire_writer(&document).await.unwrap();
    storage.append(&writer, 0, pages(b"uncommitted-tail")).await.unwrap();
    writer.release().await.unwrap();
    let before = segment_bytes(&storage, &document, 0).await;
    for mode in ["cancelled", "expired", "fuel"] {
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(mode == "cancelled"));
        let deadline = if mode == "expired" { std::time::Instant::now() } else { std::time::Instant::now() + std::time::Duration::from_secs(30) };
        let mut control = WalCursorControl::new(cancelled, deadline, if mode == "fuel" { 1 } else { 1_000_000 }).unwrap();
        let rejected = match ArtifactWal::open_with_control(&storage, document.clone(), GroupCommitPolicy::default(), 1, &mut control).await {
            Err(rejected) => matches!(rejected_open_error(rejected).await, DbError::Unavailable(_) | DbError::LimitExceeded("wal cursor fuel")),
            Ok((mut wal, _)) => {
                wal.close().await.unwrap();
                false
            }
        };
        assert!(rejected, "{mode}");
        assert_eq!(segment_bytes(&storage, &document, 0).await, before);
    }
    for cancel_on_truncate in [false, true] {
        let storage = committed_fixture_storage(row, &document).await;
        let writer = storage.acquire_writer(&document).await.unwrap();
        storage.append(&writer, 0, pages(b"uncommitted-tail")).await.unwrap();
        writer.release().await.unwrap();
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let boundary = AbortCancellationStorage { inner: &storage, cancelled: cancelled.clone(), cancel_on_truncate, synced: std::sync::atomic::AtomicUsize::new(0) };
        let mut control = WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap();
        let (mut wal, report) = ArtifactWal::open_with_control(&boundary, document.clone(), GroupCommitPolicy::default(), 2, &mut control).await.unwrap();
        assert!(cancelled.load(std::sync::atomic::Ordering::Acquire));
        assert_eq!(boundary.synced.load(std::sync::atomic::Ordering::Acquire), if cancel_on_truncate { 2 } else { 1 });
        assert_eq!(report.recovered_abort_tx_id, Some(7));
        assert_eq!(report.torn_tail_bytes, b"uncommitted-tail".len() as u64);
        let repaired = segment_bytes(&storage, &document, 0).await;
        wal.close().await.unwrap();
        let (mut reopened, report) = ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 3).await.unwrap();
        assert_eq!(report.recovered_abort_tx_id, None);
        assert_eq!(segment_bytes(&storage, &document, 0).await, repaired);
        reopened.close().await.unwrap();
        assert_no_committed_transaction(&storage, &document).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn wal_recovery_abort_capacity_exact_and_plus_one_preserves_source() {
    if !db_io_driver::process_isolated_law("tests::wal_recovery_abort_capacity_exact_and_plus_one_preserves_source") {
        return;
    }
    let suffix = wal_frame_bytes(8).unwrap() + protocol::format::COMMIT_FRAME_LEN;
    for extra in [0, 1] {
        let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
        let document = ArtifactId::from("abort-capacity");
        let writer_permit = storage.acquire_writer(&document).await.unwrap();
        let mut writer = SegmentWriter::begin(&storage, &writer_permit, document.clone(), 0, None, 0).await.unwrap();
        writer.append_record(&WalRecord::TxBegin { tx_id: 7 }, 0).await.unwrap();
        let end = db_io_core::DB_IO_MAX_READ_BYTES - suffix + extra;
        let available = end - writer.total_len().unwrap() - protocol::format::COMMIT_FRAME_LEN;
        let payload = (0..32).map(|overhead| available as usize - overhead).find(|bytes| wal_frame_bytes(*bytes).unwrap() == available).unwrap();
        let mut command = WalRecord::Command(retained(&vec![0xa5; payload]).await);
        writer.append_record(&command, 0).await.unwrap();
        while command.close_step().unwrap() {}
        writer.commit_and_flush(&storage, &writer_permit, DurabilityClass::Fsync).await.unwrap();
        while writer.close_step().unwrap() {}
        writer_permit.release().await.unwrap();
        let before = segment_bytes(&storage, &document, 0).await;
        assert_eq!(before.len() as u64, end);
        let result = ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 1).await;
        if extra == 0 {
            let (mut wal, report) = result.unwrap();
            assert_eq!(report.recovered_abort_tx_id, Some(7));
            assert_eq!(storage.segment_len(&document, 0).await.unwrap(), db_io_core::DB_IO_MAX_READ_BYTES);
            assert_no_committed_transaction(&storage, &document).await;
            let tip = wal.active.tip_chain_hash().unwrap();
            let receipt = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"successor").await), DurabilityClass::Fsync, 2).await;
            assert_eq!((receipt.segment_index, receipt.tx_id), (1, 8));
            assert_eq!(storage.segment_state(&document, 0).await.unwrap(), db_io_core::WalSegmentState::Sealed);
            wal.close().await.unwrap();
            let (mut reopened, report) = ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 3).await.unwrap();
            assert_eq!(report.recovered_abort_tx_id, None);
            let replay = replay_summaries(&storage, &document).await;
            assert!(replay.contains(&ReplaySummary::Segment(1, Some(tip))));
            assert_eq!(replay.iter().filter(|record| **record == ReplaySummary::Abort(7)).count(), 1);
            assert!(replay.contains(&ReplaySummary::Command(b"successor".to_vec())));
            reopened.close().await.unwrap();
        } else {
            let rejected = match result {
                Err(rejected) => matches!(rejected_open_error(rejected).await, DbError::LimitExceeded("wal recovery abort exceeds retained segment budget")),
                Ok((mut wal, _)) => {
                    wal.close().await.unwrap();
                    false
                }
            };
            assert!(rejected);
            assert_eq!(segment_bytes(&storage, &document, 0).await, before);
            assert_eq!(storage.segment_state(&document, 0).await.unwrap(), db_io_core::WalSegmentState::Active);
        }
        eprintln!("WAL abort recovery exact retained capacity plus {extra} had the expected byte-preserving disposition");
    }
}
