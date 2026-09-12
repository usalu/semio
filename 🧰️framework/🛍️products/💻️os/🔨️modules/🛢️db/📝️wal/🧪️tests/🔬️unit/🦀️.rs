use super::*;
use db_storage::{MemoryStorage, WalStorage};
use {ArtifactId, DurabilityClass, Frontier};

fn pages(bytes: &[u8]) -> db_storage::DbIoPages {
    let mut writer = db_storage::DbIoPageWriter::try_reserve(bytes.len().div_ceil(db_storage::DB_IO_PAGE_BYTES)).expect("test WAL writer admitted");
    for fragment in bytes.chunks(db_storage::DB_IO_PAGE_BYTES) {
        assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len());
    }
    writer.finish().unwrap()
}

async fn doc(id: &str) -> ArtifactId {
    ArtifactId::from(id)
}

async fn sample_frontier(document: &ArtifactId) -> Frontier {
    Frontier { document: document.clone(), head_seq: 7, commit_seq: 3, chain_hash: [9u8; 32], epoch: 1 }
}

fn control() -> WalCursorControl {
    WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap()
}

async fn rejected_open_error(mut rejected: ArtifactWalOpenRejected) -> DbError {
    loop {
        match rejected.retry_close().await {
            Ok(cause) => return cause,
            Err(retained) => rejected = retained,
        }
    }
}

async fn retained(source: &[u8]) -> WalBytes {
    let mut control = control();
    WalBytes::try_admit(source.to_vec(), MAX_FIELD_BYTES, &mut control).await.unwrap()
}

async fn read_retained(bytes: &WalBytes) -> Vec<u8> {
    let mut output = Vec::with_capacity(bytes.len());
    for fragment in bytes.fragments() {
        output.extend_from_slice(fragment);
        semio_framework_async::yield_once().await;
    }
    assert_eq!(output.len(), bytes.len());
    output
}

async fn decode(kind: u8, payload: &[u8]) -> Result<WalRecord, DbError> {
    let carrier = retained(payload).await;
    let operation = carrier.operation();
    let mut control = control();
    let decoded = WalRecord::decode_retained(operation, kind, payload, &mut control).await;
    let mut carrier = carrier;
    while carrier.close_step()?.is_some() {}
    decoded
}

fn run_ids(values: &[u64]) -> db_storage::DbIoU64List {
    let mut list = db_storage::DbIoU64List::new();
    for value in values {
        list.push(*value).unwrap();
    }
    list
}

async fn submit_one(storage: &impl WalStorage, wal: &mut ArtifactWal, record: WalRecord, durability: DurabilityClass, now_ms: u64) -> WalAppendReceipt {
    let mut records = WalRecordBatch::new();
    assert!(records.push(record).is_ok());
    let receipt = wal.submit(storage, &records, durability, now_ms).await.unwrap();
    while records.close_step().unwrap() {}
    receipt
}

#[derive(Debug, PartialEq, Eq)]
enum ReplaySummary {
    Segment(u64, Option<[u8; 32]>),
    Begin(u64),
    Command(Vec<u8>),
    Commit(u64, u32),
    Abort(u64),
    Other(u8),
}

async fn replay_summaries(storage: &impl WalStorage, document: &ArtifactId) -> Vec<ReplaySummary> {
    let mut replay = replay_document(storage, document, control()).await.unwrap();
    let mut summaries = Vec::new();
    while let Some(mut record) = replay.next().await.unwrap() {
        let summary = match &record {
            WalRecord::SegmentHeader { segment_index, prev_chain_hash, .. } => ReplaySummary::Segment(*segment_index, *prev_chain_hash),
            WalRecord::TxBegin { tx_id } => ReplaySummary::Begin(*tx_id),
            WalRecord::Command(bytes) => ReplaySummary::Command(read_retained(bytes).await),
            WalRecord::TxCommit { tx_id, record_count } => ReplaySummary::Commit(*tx_id, *record_count),
            WalRecord::TxAbort { tx_id } => ReplaySummary::Abort(*tx_id),
            _ => ReplaySummary::Other(record.retained_shape().0),
        };
        summaries.push(summary);
        while record.close_step().unwrap() {}
    }
    while replay.close_step().await.unwrap() {}
    summaries
}

async fn segment_bytes(storage: &impl WalStorage, document: &ArtifactId, index: u64) -> Vec<u8> {
    let len = storage.segment_len(document, index).await.unwrap();
    let mut pages = storage.read(document, index, pack::ByteRange { offset: 0, len }).await.unwrap();
    let mut prepared = db_storage::db_io_prepare_platform(&pages).unwrap().await.unwrap();
    let output = prepared.as_slice().to_vec();
    while prepared.close_step().unwrap() {}
    while pages.close_step().unwrap().is_some() {}
    output
}

fn recovery_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🚑️recovery/🔣️.json")).unwrap()
}

pub(crate) async fn committed_fixture_storage(row: &serde_json::Value, document: &ArtifactId) -> MemoryStorage {
    let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
    write_committed_fixture(&storage, row, document).await;
    storage
}

pub(crate) async fn aborted_event_fixture_storage(document: &ArtifactId, event: &[u8]) -> MemoryStorage {
    let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
    let permit = storage.acquire_writer(document).await.unwrap();
    let mut writer = SegmentWriter::begin(&storage, &permit, document.clone(), 0, None, 0).await.unwrap();
    let mut records = [WalRecord::TxBegin { tx_id: 1 }, WalRecord::Event(retained(event).await), WalRecord::TxAbort { tx_id: 1 }];
    for record in &mut records {
        writer.append_record(record, 0).await.unwrap();
        while record.close_step().unwrap() {}
    }
    writer.commit_and_flush(&storage, &permit, DurabilityClass::Fsync).await.unwrap();
    while writer.close_step().unwrap() {}
    permit.release().await.unwrap();
    storage
}

async fn write_committed_fixture(storage: &impl WalStorage, row: &serde_json::Value, document: &ArtifactId) {
    let writer_permit = storage.acquire_writer(document).await.unwrap();
    let mut previous = None;
    for (index, segment) in row["segments"].as_array().unwrap().iter().enumerate() {
        let mut writer = SegmentWriter::begin(storage, &writer_permit, document.clone(), index as u64, previous, 0).await.unwrap();
        for (ordinal, frame) in segment["frames"].as_array().unwrap().iter().enumerate().skip(1) {
            let id = || frame["id"].as_str().unwrap().parse::<u64>().unwrap();
            let mut record = match frame["kind"].as_str().unwrap() {
                "header" => WalRecord::SegmentHeader { document: document.clone(), segment_index: index as u64, prev_chain_hash: previous },
                "begin" => WalRecord::TxBegin { tx_id: id() },
                "commit" => WalRecord::TxCommit { tx_id: id(), record_count: frame["count"].as_u64().unwrap() as u32 },
                "abort" => WalRecord::TxAbort { tx_id: id() },
                "command" => WalRecord::Command(retained(&[ordinal as u8]).await),
                "frontier" => WalRecord::Frontier(sample_frontier(document).await),
                "snapshot" => WalRecord::SnapshotPub { generation: 1, frontier: sample_frontier(document).await },
                "cas" => WalRecord::Payload(WalPayloadRef::CasRef(ContentHash([7; 32]))),
                other => panic!("unknown committed fixture kind {other}"),
            };
            writer.append_record(&record, 0).await.unwrap();
            while record.close_step().unwrap() {}
            if segment["physicalCommitsAfter"].as_array().unwrap().iter().any(|value| value.as_u64() == Some(ordinal as u64)) {
                writer.commit_and_flush(storage, &writer_permit, DurabilityClass::Fsync).await.unwrap();
            }
        }
        previous = Some(writer.tip_chain_hash().await.unwrap());
        while writer.close_step().unwrap() {}
        if segment["state"] == "sealed" {
            storage.seal(&writer_permit, index as u64).await.unwrap();
        }
    }
    writer_permit.release().await.unwrap();
}

async fn assert_no_committed_transaction(storage: &impl WalStorage, document: &ArtifactId) {
    let mut cursor = replay_committed_document(storage, document, control()).await.unwrap();
    let found = loop {
        match cursor.next_transaction_step().await.unwrap() {
            WalCommittedStep::Yield => {}
            WalCommittedStep::Done => break false,
            WalCommittedStep::Transaction(transaction) => {
                drop(transaction);
                break true;
            }
        }
    };
    while cursor.close_owner_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    assert!(cursor.terminal_is_empty());
    assert!(!found, "recovery made an incomplete transaction visible");
}

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
        eprintln!("[DEBUG] active WAL recovery appended exactly one durable abort and preserved byte identity on reopen: {name}");
    }
}

#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
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
            let filesystem = db_storage::FsStorage::open(crate::db_storage::db_io_test_pool(), &root).await.unwrap();
            write_committed_fixture(&filesystem, row, &document).await;
            assert_eq!(filesystem.segment_state(&document, 0).await.unwrap(), db_storage::WalSegmentState::Active);
            filesystem.close().await.unwrap();
        }
        let repaired = {
            let filesystem = db_storage::FsStorage::open(crate::db_storage::db_io_test_pool(), &root).await.unwrap();
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
            let filesystem = db_storage::FsStorage::open(crate::db_storage::db_io_test_pool(), &root).await.unwrap();
            let (mut wal, report) = ArtifactWal::open(&filesystem, document.clone(), GroupCommitPolicy::default(), 2).await.unwrap();
            assert_eq!(report.recovered_abort_tx_id, None);
            assert_eq!(report.torn_tail_bytes, 0);
            assert_eq!(segment_bytes(&filesystem, &document, 0).await, repaired);
            assert_eq!(filesystem.segment_state(&document, 0).await.unwrap(), db_storage::WalSegmentState::Active);
            assert_eq!(replay_summaries(&filesystem, &document).await.iter().filter(|record| **record == ReplaySummary::Abort(expected_abort)).count(), 1);
            assert_no_committed_transaction(&filesystem, &document).await;
            let receipt = submit_one(&filesystem, &mut wal, WalRecord::Command(retained(b"after-filesystem-recovery").await), DurabilityClass::Fsync, 3).await;
            assert_eq!(receipt.tx_id, next);
            wal.close().await.unwrap();
            filesystem.close().await.unwrap();
        }
        eprintln!("[DEBUG] one durable WAL abort survived independent filesystem retirement and reopen, preserving the next transaction id: {name}");
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
            let storage = crate::db_fault_testing::FaultStorage::new(inner.clone()).await;
            let baseline = segment_bytes(&storage, &document, 0).await;
            if tail {
                let writer = storage.acquire_writer(&document).await.unwrap();
                storage.append(&writer, 0, pages(b"uncommitted-tail")).await.unwrap();
                writer.release().await.unwrap();
            }
            let append_boundary = storage.append_calls().await + 1;
            let sync_boundary = storage.sync_calls().await + if tail && !fail_tail_sync { 2 } else { 1 };
            let mut script = crate::db_fault_testing::FaultScript::default();
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
            eprintln!("[DEBUG] WAL abort fault retired owners and reopened idempotently: {}, tail={tail}, fail_tail_sync={fail_tail_sync}", case["name"]);
        }
    }
}

struct AbortCancellationStorage<'a> {
    inner: &'a MemoryStorage,
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    cancel_on_truncate: bool,
    synced: std::sync::atomic::AtomicUsize,
}

impl WalStorage for AbortCancellationStorage<'_> {
    async fn acquire_writer(&self, document: &ArtifactId) -> Result<db_storage::WalWriterPermit, DbError> {
        self.inner.acquire_writer(document).await
    }
    async fn create_segment(&self, writer: &db_storage::WalWriterPermit, index: u64) -> Result<(), DbError> {
        self.inner.create_segment(writer, index).await
    }
    async fn append(&self, writer: &db_storage::WalWriterPermit, index: u64, bytes: db_storage::DbIoPages) -> Result<u64, DbError> {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.inner.append(writer, index, bytes).await
    }
    async fn sync(&self, writer: &db_storage::WalWriterPermit, index: u64, class: DurabilityClass) -> Result<(), DbError> {
        let result = self.inner.sync(writer, index, class).await;
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) && class == DurabilityClass::Fsync && result.is_ok() {
            self.synced.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        }
        result
    }
    async fn seal(&self, writer: &db_storage::WalWriterPermit, index: u64) -> Result<(), DbError> {
        self.inner.seal(writer, index).await
    }
    async fn read(&self, document: &ArtifactId, index: u64, range: pack::ByteRange) -> Result<db_storage::DbIoPages, DbError> {
        self.inner.read(document, index, range).await
    }
    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        self.inner.segment_len(document, index).await
    }
    async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<db_storage::WalSegmentState, DbError> {
        self.inner.segment_state(document, index).await
    }
    async fn list_segments(&self, document: &ArtifactId) -> Result<db_storage::DbIoU64List, DbError> {
        self.inner.list_segments(document).await
    }
    async fn truncate_tail(&self, writer: &db_storage::WalWriterPermit, index: u64, len: u64) -> Result<(), DbError> {
        if self.cancel_on_truncate {
            self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        }
        self.inner.truncate_tail(writer, index, len).await
    }
    async fn delete_segment(&self, writer: &db_storage::WalWriterPermit, index: u64) -> Result<(), DbError> {
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
        eprintln!("[DEBUG] WAL abort recovery rejected pre-boundary cancellation without writes and completed admitted Fsync, cancel_on_truncate={cancel_on_truncate}");
    }
}

#[semio_framework_async_macros::async_test]
async fn wal_recovery_abort_capacity_exact_and_plus_one_preserves_source() {
    let suffix = wal_frame_bytes(8).unwrap() + protocol::format::COMMIT_FRAME_LEN;
    for extra in [0, 1] {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = ArtifactId::from("abort-capacity");
        let writer_permit = storage.acquire_writer(&document).await.unwrap();
        let mut writer = SegmentWriter::begin(&storage, &writer_permit, document.clone(), 0, None, 0).await.unwrap();
        writer.append_record(&WalRecord::TxBegin { tx_id: 7 }, 0).await.unwrap();
        let end = db_storage::DB_IO_MAX_READ_BYTES - suffix + extra;
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
            assert_eq!(storage.segment_len(&document, 0).await.unwrap(), db_storage::DB_IO_MAX_READ_BYTES);
            assert_no_committed_transaction(&storage, &document).await;
            let tip = wal.active.tip_chain_hash().await.unwrap();
            let receipt = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"successor").await), DurabilityClass::Fsync, 2).await;
            assert_eq!((receipt.segment_index, receipt.tx_id), (1, 8));
            assert_eq!(storage.segment_state(&document, 0).await.unwrap(), db_storage::WalSegmentState::Sealed);
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
            assert_eq!(storage.segment_state(&document, 0).await.unwrap(), db_storage::WalSegmentState::Active);
        }
        eprintln!("[DEBUG] WAL abort recovery exact retained capacity plus {extra} had the expected byte-preserving disposition");
    }
}

fn committed_fixture_kind(kind: u8) -> &'static str {
    match kind {
        WAL_COMMAND => "command",
        WAL_FRONTIER => "frontier",
        WAL_SNAPSHOT_PUB => "snapshot",
        WAL_PAYLOAD => "cas",
        _ => panic!("unexpected fixture body kind"),
    }
}

#[semio_framework_async_macros::async_test]
async fn wal_transaction_gate_matches_neutral_committed_spans() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let document = ArtifactId::from("committed-fixture");
        let storage = committed_fixture_storage(row, &document).await;
        let mut gate = WalTransactionGate::new();
        let mut transactions = Vec::new();
        let mut recovery_abort = None;
        let mut failure = None;
        for (index, segment) in row["segments"].as_array().unwrap().iter().enumerate() {
            let bytes = segment_bytes(&storage, &document, index as u64).await;
            let mut source = pages(&bytes);
            let mut offset = protocol::format::HEADER_SIZE;
            let result: Result<(), DbError> = async {
                let prior = if index == 0 { WalPriorChainTip::Genesis } else { WalPriorChainTip::RetainedBoundary };
                let mut chain = WalSegmentChain::new(&source, index as u64, prior)?;
                while !chain.step(&source, bytes.len(), &document, &mut control())? {}
                loop {
                    let frame = match wal_next_verified_page_frame(&source, &mut offset, bytes.len(), &mut control())? {
                        WalVerifiedFrameStep::Frame(frame) => frame,
                        WalVerifiedFrameStep::PhysicalCommit => continue,
                        WalVerifiedFrameStep::Done => break,
                    };
                    if gate.push(&source, frame)? {
                        let kinds: Vec<_> = gate.frames[..gate.frames_len as usize].iter().map(|frame| committed_fixture_kind(frame.unwrap().kind)).collect();
                        transactions.push(serde_json::json!({ "id": gate.ready.unwrap().to_string(), "kinds": kinds }));
                        gate.release()?;
                    }
                    offset = frame.frame_end;
                }
                recovery_abort = gate.finish_segment(segment["state"] == "active" && index + 1 == row["segments"].as_array().unwrap().len())?;
                if index + 1 != row["segments"].as_array().unwrap().len() {
                    gate.advance_segment()?;
                }
                Ok(())
            }
            .await;
            while source.close_step().unwrap().is_some() {}
            if let Err(error) = result {
                failure = Some(match error {
                    DbError::LimitExceeded("wal transaction records") => "capacity",
                    DbError::LimitExceeded("wal transaction sequence") => "sequence",
                    DbError::Corrupt(_) => "corrupt",
                    other => panic!("unexpected fixture error: {other:?}"),
                });
                break;
            }
        }
        let actual = if let Some(error) = failure {
            serde_json::json!({ "accepted": false, "transactions": [], "nextTxId": null, "recoverAbort": null, "error": error })
        } else {
            serde_json::json!({ "accepted": true, "transactions": transactions, "nextTxId": gate.next_tx_id.to_string(), "recoverAbort": recovery_abort.map(|id| id.to_string()), "error": null })
        };
        assert_eq!(actual, row["expected"], "{}", row["name"]);
        if failure.is_none() && recovery_abort.is_none() {
            let mut cursor = replay_committed_document(&storage, &document, control()).await.unwrap();
            let mut replayed = Vec::new();
            loop {
                match cursor.next_transaction_step().await.unwrap() {
                    WalCommittedStep::Transaction(mut transaction) => {
                        let id = transaction.transaction_id().to_string();
                        let mut kinds = Vec::new();
                        loop {
                            match transaction.next_record_step().unwrap() {
                                WalCommittedRecordStep::Record(record) => {
                                    kinds.push(committed_fixture_kind(record.retained_shape().0));
                                }
                                WalCommittedRecordStep::Yield => continue,
                                WalCommittedRecordStep::Done => break,
                            }
                            while transaction.close_record_step().unwrap() {}
                        }
                        transaction.finish().unwrap();
                        replayed.push(serde_json::json!({ "id": id, "kinds": kinds }));
                    }
                    WalCommittedStep::Yield => {}
                    WalCommittedStep::Done => break,
                }
            }
            while cursor.close_owner_step().unwrap() {}
            assert!(cursor.terminal_is_empty());
            assert_eq!(serde_json::json!(replayed), row["expected"]["transactions"], "{}", row["name"]);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn wal_immutable_source_fragmentation_matches_neutral_transactions() {
    struct Fragments<'a> {
        bytes: &'a [u8],
        chunk: usize,
    }
    impl WalImmutableByteSource for Fragments<'_> {
        fn byte_len(&self) -> usize {
            self.bytes.len()
        }
        fn fragment_at(&self, offset: usize, limit: usize) -> Result<&[u8], DbError> {
            if offset >= limit || limit > self.bytes.len() {
                return Err(DbError::Corrupt("fixture immutable range".to_string()));
            }
            Ok(&self.bytes[offset..limit.min((offset / self.chunk + 1) * self.chunk)])
        }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap().iter().filter(|row| row["expected"]["accepted"] == true && row["expected"]["recoverAbort"].is_null()) {
        let document = ArtifactId::from("committed-fragmented-source");
        let storage = committed_fixture_storage(row, &document).await;
        for chunk in [1, 7, 4_093, 4_096, 0] {
            let mut gate = WalTransactionGate::new();
            let mut previous = WalPriorChainTip::Genesis;
            let mut output = Vec::new();
            for index in 0..row["segments"].as_array().unwrap().len() {
                let bytes = segment_bytes(&storage, &document, index as u64).await;
                let fragments = Fragments { bytes: &bytes, chunk: chunk.max(1) };
                let mut ranged = if chunk == 0 {
                    let mut padded = vec![0xad; 4_093];
                    padded.extend_from_slice(&bytes);
                    padded.extend_from_slice(&[0xda; 19]);
                    let mut writer = db_storage::DbIoPageWriter::try_reserve(padded.len().div_ceil(db_storage::DB_IO_PAGE_BYTES)).unwrap();
                    for fragment in padded.chunks(db_storage::DB_IO_PAGE_BYTES) {
                        assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len());
                    }
                    let pages = writer.finish().unwrap();
                    let pages = pages.try_range(4_093).unwrap_or_else(|_| panic!("fixture range must retain its exact WAL"));
                    Some(pages.try_prefix(bytes.len()).unwrap_or_else(|_| panic!("fixture prefix must exclude trailing padding")))
                } else {
                    None
                };
                let source: &dyn WalImmutableByteSource = match ranged.as_ref() {
                    Some(pages) => pages,
                    None => &fragments,
                };
                assert_eq!(source.byte_len(), bytes.len());
                assert!(source.fragment_at(bytes.len(), bytes.len()).is_err());
                assert!(source.fragment_at(0, bytes.len() + 1).is_err());
                let owner = WalAuthenticatedSource::new(source, gate, index as u64, previous);
                let mut owner = match owner.finish() {
                    Err(owner) => owner,
                    Ok(_) => panic!("unverified source must retain its ownership"),
                };
                let mut opportunity = control();
                assert!(matches!(owner.next_step(&mut opportunity), Err(DbError::Corrupt(_))));
                let mut turns = 0;
                loop {
                    turns += 1;
                    assert!(turns <= bytes.len() + 2);
                    opportunity.replenish(std::time::Instant::now() + std::time::Duration::from_secs(30), 1).unwrap();
                    if owner.verify_step(&document, &mut opportunity).unwrap() {
                        break;
                    }
                }
                loop {
                    opportunity.replenish(std::time::Instant::now() + std::time::Duration::from_secs(30), 1).unwrap();
                    match owner.next_step(&mut opportunity).unwrap() {
                        WalAuthenticatedStep::Committed => {
                            let mut kinds = Vec::new();
                            for body in 0..64 {
                                let Some(frame) = owner.committed_frame(body) else {
                                    break;
                                };
                                kinds.push(committed_fixture_kind(frame.kind));
                            }
                            output.push(serde_json::json!({ "id": owner.gate.ready.unwrap().to_string(), "kinds": kinds }));
                            assert!(matches!(owner.next_step(&mut opportunity), Err(DbError::Corrupt(_))));
                            owner.finish_transaction().unwrap();
                        }
                        WalAuthenticatedStep::Yield => {}
                        WalAuthenticatedStep::Done => break,
                    }
                }
                let (_, next_gate, tip) = owner.finish().unwrap_or_else(|_| panic!("drained authenticated source must transfer ownership"));
                gate = next_gate;
                previous = WalPriorChainTip::Verified(tip);
                if let Some(pages) = ranged.as_mut() {
                    while !pages.terminal_is_empty() {
                        pages.close_step().unwrap();
                    }
                    assert!(pages.terminal_is_empty());
                }
            }
            assert_eq!(serde_json::json!(output), row["expected"]["transactions"], "{}: chunk={chunk}", row["name"]);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn wal_committed_cursor_single_fuel_and_expired_turns_match_neutral_transactions() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap().iter().filter(|row| row["expected"]["accepted"] == true && row["expected"]["recoverAbort"].is_null()) {
        let document = ArtifactId::from("committed-one-fuel");
        let storage = committed_fixture_storage(row, &document).await;
        let mut cursor = replay_committed_document(&storage, &document, control()).await.unwrap();
        let mut output = Vec::new();
        let mut turns = 0;
        loop {
            turns += 1;
            assert!(turns < 4_096, "single-fuel replay stopped progressing: {}", row["name"]);
            cursor.replenish(std::time::Instant::now(), 1).unwrap();
            assert!(matches!(cursor.next_transaction_step().await, Err(DbError::Unavailable(message)) if message == "wal cursor deadline reached"));
            cursor.replenish(std::time::Instant::now() + std::time::Duration::from_secs(30), 1).unwrap();
            match cursor.next_transaction_step().await.unwrap() {
                WalCommittedStep::Transaction(mut transaction) => {
                    let id = transaction.transaction_id().to_string();
                    let mut kinds = Vec::new();
                    loop {
                        transaction.replenish(std::time::Instant::now() + std::time::Duration::from_secs(30), 1).unwrap();
                        match transaction.next_record_step().unwrap() {
                            WalCommittedRecordStep::Record(record) => kinds.push(committed_fixture_kind(record.retained_shape().0)),
                            WalCommittedRecordStep::Yield => continue,
                            WalCommittedRecordStep::Done => break,
                        }
                        while transaction.close_record_step().unwrap() {}
                    }
                    transaction.finish().unwrap();
                    output.push(serde_json::json!({ "id": id, "kinds": kinds }));
                }
                WalCommittedStep::Yield => {}
                WalCommittedStep::Done => break,
            }
        }
        while cursor.close_owner_step().unwrap() {}
        assert!(cursor.terminal_is_empty());
        assert_eq!(serde_json::json!(output), row["expected"]["transactions"], "{}", row["name"]);
    }
}

#[semio_framework_async_macros::async_test]
async fn wal_retained_varints_match_neutral_exact_u64_and_atomic_interruption() {
    let _pool = crate::db_storage::db_io_test_pool();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📖️retained-decoder/🔣️.json")).unwrap();
    for row in fixture["varints"].as_array().unwrap() {
        let hex = row["hex"].as_str().unwrap();
        let input: Vec<_> = (0..hex.len()).step_by(2).map(|offset| u8::from_str_radix(&hex[offset..offset + 2], 16).unwrap()).collect();
        let mut bytes = WalBytes { pages: pages(&input) };
        let expected = row["value"].as_str().map(|value| value.parse::<u64>().unwrap());
        let mut reader = WalPageReader::new(&bytes.pages, 0, input.len()).unwrap();
        let parsed = reader.varint();
        assert_eq!(parsed.as_ref().ok().copied(), expected, "{}", row["name"]);
        assert_eq!(reader.position, row["consumed"].as_u64().unwrap_or(0) as usize, "{}", row["name"]);
        let mut cursor = bytes.cursor();
        let parsed = cursor.varint(&mut control());
        assert_eq!(parsed.as_ref().ok().copied(), expected, "{}", row["name"]);
        assert_eq!(cursor.offset, row["consumed"].as_u64().unwrap_or(0) as usize, "{}", row["name"]);
        if row["consumed"].as_u64().is_some_and(|count| count > 1) {
            let mut cursor = bytes.cursor();
            let mut opportunity = control();
            opportunity.replenish(std::time::Instant::now() + std::time::Duration::from_secs(30), 1).unwrap();
            assert!(matches!(cursor.varint(&mut opportunity), Err(DbError::LimitExceeded("wal cursor fuel"))));
            assert_eq!(cursor.offset, 0);
            opportunity.replenish(std::time::Instant::now() + std::time::Duration::from_secs(30), 16).unwrap();
            assert_eq!(cursor.varint(&mut opportunity).unwrap(), expected.unwrap());
        }
        while bytes.close_step().unwrap().is_some() {}
    }
}

#[semio_framework_async_macros::async_test]
async fn wal_retained_decoder_fuel_resumes_exact_fragmented_bytes() {
    let _pool = crate::db_storage::db_io_test_pool();
    let expected: Vec<_> = (0..db_storage::DB_IO_PAGE_BYTES * 3 + 17).map(|index| (index % 251) as u8).collect();
    let mut source = pages(&expected);
    let frame = WalRecordFrame { kind: WAL_COMMAND, payload_start: 0, payload_end: expected.len(), frame_end: expected.len() };
    let mut decoder = WalRetainedRecordDecoder::new(frame);
    let mut opportunity = control();
    let mut attempts = 0;
    let mut record = loop {
        opportunity.replenish(std::time::Instant::now() + std::time::Duration::from_secs(30), 1).unwrap();
        match decoder.step(&source, &mut opportunity).unwrap() {
            Some(record) => break record,
            None => assert!(matches!(decoder.step(&source, &mut opportunity), Err(DbError::LimitExceeded("wal cursor fuel")))),
        }
        attempts += 1;
        assert!(attempts < 1_024);
    };
    assert!(attempts > 3);
    let WalRecord::Command(bytes) = &record else { panic!("expected command") };
    assert_eq!(read_retained(bytes).await, expected);
    while record.close_step().unwrap() {}
    while decoder.close_owner_step().unwrap() {}
    assert!(decoder.terminal_is_empty());
    while source.close_step().unwrap().is_some() {}
}

#[semio_framework_async_macros::async_test]
async fn wal_retained_decoder_cancel_close_preserves_source_and_returns_owner() {
    let _pool = crate::db_storage::db_io_test_pool();
    let expected = vec![0x57; db_storage::DB_IO_PAGE_BYTES + 7];
    let mut source = pages(&expected);
    for cut in 1..=4 {
        let mut decoder = WalRetainedRecordDecoder::new(WalRecordFrame { kind: WAL_COMMAND, payload_start: 0, payload_end: expected.len(), frame_end: expected.len() });
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let mut opportunity = WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 128).unwrap();
        for _ in 0..cut {
            assert!(decoder.step(&source, &mut opportunity).unwrap().is_none());
        }
        cancelled.store(true, std::sync::atomic::Ordering::Release);
        assert!(matches!(decoder.step(&source, &mut opportunity), Err(DbError::Unavailable(message)) if message == "wal cursor cancelled"));
        while decoder.close_owner_step().unwrap() {
            assert!(cancelled.load(std::sync::atomic::Ordering::Acquire));
        }
        assert!(decoder.terminal_is_empty());
        assert_eq!(source.len(), expected.len());
    }
    while source.close_step().unwrap().is_some() {}
}

#[semio_framework_async_macros::async_test]
async fn wal_committed_cursor_cancel_resume_keeps_transaction_position() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == "two-commands-only-after-logical-commit").unwrap();
    let document = ArtifactId::from("committed-cancel");
    let storage = committed_fixture_storage(row, &document).await;
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let opportunity = WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
    let mut cursor = replay_committed_document(&storage, &document, opportunity).await.unwrap();
    loop {
        match cursor.next_transaction_step().await.unwrap() {
            WalCommittedStep::Transaction(mut transaction) => {
                assert_eq!(transaction.record_count(), 2);
                assert!(matches!(transaction.next_record_step().unwrap(), WalCommittedRecordStep::Yield));
                let offset = transaction.cursor.raw.offset;
                cancelled.store(true, std::sync::atomic::Ordering::Release);
                assert!(matches!(transaction.next_record_step(), Err(DbError::Unavailable(message)) if message == "wal cursor cancelled"));
                assert_eq!(transaction.cursor.raw.offset, offset);
                assert_eq!(transaction.cursor.record_index, 0);
                cancelled.store(false, std::sync::atomic::Ordering::Release);
                let mut output = Vec::new();
                loop {
                    match transaction.next_record_step().unwrap() {
                        WalCommittedRecordStep::Record(WalRecord::Command(bytes)) => output.push(read_retained(bytes).await),
                        WalCommittedRecordStep::Record(_) => panic!("expected command"),
                        WalCommittedRecordStep::Yield => continue,
                        WalCommittedRecordStep::Done => break,
                    }
                    while transaction.close_record_step().unwrap() {}
                }
                assert_eq!(output, vec![vec![2], vec![3]]);
                transaction.finish().unwrap();
                break;
            }
            WalCommittedStep::Yield => {}
            WalCommittedStep::Done => panic!("committed transaction was skipped"),
        }
    }
    while cursor.close_owner_step().unwrap() {}
    assert!(cursor.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn wal_committed_cursor_unfinished_borrow_poison_and_cancelled_close() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == "two-commands-only-after-logical-commit").unwrap();
    let document = ArtifactId::from("committed-drop");
    let storage = committed_fixture_storage(row, &document).await;
    for decoded in [false, true] {
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let opportunity = WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
        let mut cursor = replay_committed_document(&storage, &document, opportunity).await.unwrap();
        loop {
            match cursor.next_transaction_step().await.unwrap() {
                WalCommittedStep::Transaction(mut transaction) => {
                    assert!(matches!(transaction.next_record_step().unwrap(), WalCommittedRecordStep::Yield));
                    if decoded {
                        loop {
                            match transaction.next_record_step().unwrap() {
                                WalCommittedRecordStep::Record(WalRecord::Command(_)) => break,
                                WalCommittedRecordStep::Yield => {}
                                _ => panic!("unfinished-record fixture lost its first command"),
                            }
                        }
                    }
                    drop(transaction);
                    break;
                }
                WalCommittedStep::Yield => {}
                WalCommittedStep::Done => panic!("committed transaction was skipped"),
            }
        }
        assert!(matches!(cursor.next_transaction_step().await, Err(DbError::Corrupt(_))));
        cancelled.store(true, std::sync::atomic::Ordering::Release);
        while cursor.close_owner_step().unwrap() {
            assert!(cancelled.load(std::sync::atomic::Ordering::Acquire));
        }
        assert!(cursor.terminal_is_empty());
    }
}

async fn capacity_submission(storage: &impl WalStorage, wal: &mut ArtifactWal, length: usize, durability: DurabilityClass) -> Result<WalAppendReceipt, DbError> {
    let mut batch = WalRecordBatch::new();
    assert!(batch.push(WalRecord::Command(retained(&vec![b'a'; length]).await)).is_ok());
    let result = wal.submit(storage, &batch, durability, 0).await;
    while batch.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    result
}

async fn capacity_backend(storage: &impl WalStorage, fixture: &serde_json::Value, case: &serde_json::Value) {
    let document = doc("d").await;
    let policy = GroupCommitPolicy { max_delay_ms: u64::MAX, max_bytes: u64::MAX, max_records: u32::MAX };
    let mut wal = ArtifactWal::create(storage, document.clone(), policy, 0).await.unwrap();
    let genesis = segment_bytes(storage, &document, 0).await;
    let rejected = capacity_submission(storage, &mut wal, fixture["oversizedPayloadBytes"].as_u64().unwrap() as usize, DurabilityClass::Fsync).await;
    assert!(matches!(rejected, Err(DbError::LimitExceeded(_))));
    assert_eq!(wal.next_tx_id, 1);
    assert_eq!(wal.active.pending_records, 0);
    assert_eq!(segment_bytes(storage, &document, 0).await, genesis);
    let durability = if case["durability"] == "fsync" { DurabilityClass::Fsync } else { DurabilityClass::Memory };
    for (ordinal, expected) in case["segments"].as_array().unwrap().iter().enumerate() {
        let receipt = capacity_submission(storage, &mut wal, fixture["payloadBytes"].as_u64().unwrap() as usize, durability).await.unwrap();
        assert_eq!(receipt.segment_index, expected.as_u64().unwrap());
        assert_eq!(receipt.tx_id, ordinal as u64 + 1);
    }
    wal.force_flush(storage).await.unwrap();
    let before = segment_bytes(storage, &document, 0).await;
    for (index, length) in case["lengths"].as_array().unwrap().iter().enumerate() {
        assert_eq!(storage.segment_len(&document, index as u64).await.unwrap(), length.as_u64().unwrap());
    }
    while wal.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    let (mut reopened, _) = ArtifactWal::open(storage, document.clone(), policy, 1).await.unwrap();
    assert_eq!(reopened.next_tx_id, 4);
    assert_eq!(segment_bytes(storage, &document, 0).await, before);
    assert_eq!(replay_summaries(storage, &document).await.iter().filter(|record| matches!(record, ReplaySummary::Command(_))).count(), 3);
    while reopened.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
}

#[semio_framework_async_macros::async_test]
async fn wal_capacity_preflight_matches_neutral_memory_and_filesystem_boundaries() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📏️capacity/🔣️.json")).unwrap();
    for (ordinal, case) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        capacity_backend(&storage, &fixture, case).await;
        #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
        {
            let base = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
            let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
            let root = base.join(format!("wal-capacity-{}-{nonce}-{ordinal}", std::process::id()));
            let filesystem = db_storage::FsStorage::open(crate::db_storage::db_io_test_pool(), &root).await.unwrap();
            capacity_backend(&filesystem, &fixture, case).await;
            filesystem.close().await.unwrap();
        }
    }
    let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("d").await;
    let mut wal = ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).await.unwrap();
    capacity_submission(&storage, &mut wal, fixture["exactPayloadBytes"].as_u64().unwrap() as usize, DurabilityClass::Fsync).await.unwrap();
    assert_eq!(storage.segment_len(&document, 0).await.unwrap(), fixture["maxSegmentBytes"].as_u64().unwrap());
    while wal.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    let (mut wal, _) = ArtifactWal::open(&storage, document, GroupCommitPolicy::default(), 0).await.unwrap();
    while wal.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    println!("[DEBUG] WAL capacity: Memory and filesystem Fsync/grouped submissions rotate before overflow, reject one-over without effects and reopen the exact maximum");
}

async fn recovery_seed(storage: &MemoryStorage, document: &ArtifactId) -> ArtifactWal {
    let mut wal = ArtifactWal::create(storage, document.clone(), GroupCommitPolicy::default(), 0).await.unwrap();
    for (index, command) in recovery_fixture()["commands"].as_array().unwrap().iter().enumerate() {
        submit_one(storage, &mut wal, WalRecord::Command(retained(command.as_str().unwrap().as_bytes()).await), DurabilityClass::Fsync, index as u64 + 1).await;
    }
    wal
}

#[semio_framework_async_macros::async_test]
async fn wal_recovery_preserves_neutral_committed_prefixes() {
    let fixture = recovery_fixture();
    let input: Vec<u8> = (0..49_152).map(|index| ((index * 17 + 3) % 251) as u8).collect();
    let mut source = SharedBuf::try_new().unwrap();
    pack::PackSink::write_all(&mut source, &input).await.unwrap();
    for row in fixture["fragmentCopies"].as_array().unwrap() {
        let offset = row["offset"].as_u64().unwrap() as usize;
        let length = row["length"].as_u64().unwrap() as usize;
        let mut copied = source.copy_range(offset, length).await.unwrap();
        assert_eq!(wal_crc_range(&copied, 0, length, &mut control()).unwrap(), row["crc32c"].as_u64().unwrap() as u32);
        let prefix = copy_verified_prefix(&copied, length as u64, &mut control()).await.unwrap();
        let mut actual = vec![0; length];
        prefix.read_exact(0, &mut actual).await.unwrap();
        assert_eq!(actual, input[offset..offset + length]);
        while lock(&prefix.0).close_step().unwrap().is_some() {
            semio_framework_async::yield_once().await;
        }
        while copied.close_step().unwrap().is_some() {
            semio_framework_async::yield_once().await;
        }
    }
    while lock(&source.0).close_step().unwrap().is_some() {
        semio_framework_async::yield_once().await;
    }
    let document = doc(fixture["document"].as_str().unwrap()).await;
    let seed = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
    let mut seeded = recovery_seed(&seed, &document).await;
    seeded.close().await.unwrap();
    let full = segment_bytes(&seed, &document, 0).await;
    assert_eq!(full.len() as u64, fixture["commitEnds"][2].as_u64().unwrap());
    for row in fixture["cuts"].as_array().unwrap() {
        let cut = row["cut"].as_u64().unwrap() as usize;
        let trusted = row["trustedEnd"].as_u64().unwrap() as usize;
        let recovered = row["recoveredEnd"].as_u64().unwrap() as usize;
        let next_tx = row["nextTxId"].as_u64().unwrap();
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let writer = storage.acquire_writer(&document).await.unwrap();
        storage.create_segment(&writer, 0).await.unwrap();
        if cut != 0 {
            storage.append(&writer, 0, pages(&full[..cut])).await.unwrap();
        }
        writer.release().await.unwrap();
        let (mut wal, report) = ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 9).await.unwrap_or_else(|error| panic!("cut {cut}: {error:?}"));
        assert_eq!(report.torn_tail_bytes, (cut - trusted) as u64, "cut {cut}");
        assert_eq!(report.segments_seen, 1);
        assert_eq!(wal.next_tx_id, next_tx, "cut {cut}");
        assert_eq!(segment_bytes(&storage, &document, 0).await, full[..recovered], "cut {cut}: exact retained bytes and original commit boundaries");
        assert!(!wal.force_flush(&storage).await.unwrap());
        wal.close().await.unwrap();
        let (mut wal, clean) = ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 10).await.unwrap();
        assert_eq!(clean.torn_tail_bytes, 0);
        assert_eq!(segment_bytes(&storage, &document, 0).await, full[..recovered]);
        let receipt = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"c").await), DurabilityClass::Fsync, 11).await;
        assert_eq!(receipt.tx_id, next_tx);
        let appended = segment_bytes(&storage, &document, 0).await;
        assert_eq!(&appended[..recovered], &full[..recovered]);
        assert!(replay_summaries(&storage, &document).await.contains(&ReplaySummary::Commit(next_tx, 1)));
        wal.close().await.unwrap();
    }
    println!("[DEBUG] WAL recovery: 4 independent CRC page-alignment copies and 18 neutral cuts preserve exact bytes, reopen idempotence, sequence and subsequent transaction ids");
}

#[semio_framework_async_macros::async_test]
async fn wal_recovery_matches_neutral_lifecycle_without_prefix_replacement() {
    let fixture = recovery_fixture();
    let document = doc(fixture["document"].as_str().unwrap()).await;
    for row in fixture["lifecycle"].as_array().unwrap() {
        let name = row["name"].as_str().unwrap();
        let accepted = row["accepted"].as_bool().unwrap();
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        if name == "exhausted-segment" {
            let writer = storage.acquire_writer(&document).await.unwrap();
            let mut segment = SegmentWriter::begin(&storage, &writer, document.clone(), u64::MAX, Some([7; 32]), 0).await.unwrap();
            while segment.close_step().unwrap() {}
            writer.release().await.unwrap();
        } else if name != "missing" {
            let mut wal = recovery_seed(&storage, &document).await;
            if matches!(name, "successor-empty" | "successor-partial" | "successor-header" | "compacted-clean" | "compacted-empty" | "compacted-header" | "earlier-active" | "wrong-chain") {
                wal.rotate(&storage, 3).await.unwrap();
            }
            if name == "compacted-clean" {
                submit_one(&storage, &mut wal, WalRecord::Command(retained(b"c").await), DurabilityClass::Fsync, 4).await;
            }
            if name == "exhausted-tx" {
                wal.active.append_record(&WalRecord::TxBegin { tx_id: u64::MAX }, 4).await.unwrap();
                wal.active.append_record(&WalRecord::TxCommit { tx_id: u64::MAX, record_count: 0 }, 4).await.unwrap();
                wal.force_flush(&storage).await.unwrap();
            }
            let tip = wal.active.tip_chain_hash().await.unwrap();
            wal.close().await.unwrap();
            let writer = storage.acquire_writer(&document).await.unwrap();
            match name {
                "highest-sealed" => storage.seal(&writer, 0).await.unwrap(),
                "successor-empty" | "compacted-empty" => storage.truncate_tail(&writer, 1, 0).await.unwrap(),
                "successor-partial" => storage.truncate_tail(&writer, 1, 15).await.unwrap(),
                "successor-header" | "compacted-header" => storage.truncate_tail(&writer, 1, 32).await.unwrap(),
                "sealed-torn" | "sealed-empty" => {
                    storage.truncate_tail(&writer, 0, if name == "sealed-empty" { 0 } else { 386 }).await.unwrap();
                    storage.seal(&writer, 0).await.unwrap();
                }
                "earlier-active" | "corrupt-crc" | "partial-header-mismatch" => {
                    let mut bytes = segment_bytes(&storage, &document, 0).await;
                    if name == "corrupt-crc" {
                        bytes[100] ^= 1;
                    }
                    if name == "partial-header-mismatch" {
                        bytes.truncate(15);
                        bytes[0] ^= 1;
                    }
                    storage.delete_segment(&writer, 0).await.unwrap();
                    storage.create_segment(&writer, 0).await.unwrap();
                    storage.append(&writer, 0, pages(&bytes)).await.unwrap();
                }
                "wrong-document" => {
                    let other = doc("e").await;
                    let other_writer = storage.acquire_writer(&other).await.unwrap();
                    let mut segment = SegmentWriter::begin(&storage, &other_writer, other.clone(), 0, None, 0).await.unwrap();
                    while segment.close_step().unwrap() {}
                    other_writer.release().await.unwrap();
                    let bytes = segment_bytes(&storage, &other, 0).await;
                    storage.delete_segment(&writer, 0).await.unwrap();
                    storage.create_segment(&writer, 0).await.unwrap();
                    storage.append(&writer, 0, pages(&bytes)).await.unwrap();
                }
                "wrong-chain" => {
                    storage.delete_segment(&writer, 1).await.unwrap();
                    let mut segment = SegmentWriter::begin(&storage, &writer, document.clone(), 1, Some([7; 32]), 0).await.unwrap();
                    while segment.close_step().unwrap() {}
                }
                "index-gap" => {
                    storage.seal(&writer, 0).await.unwrap();
                    let mut segment = SegmentWriter::begin(&storage, &writer, document.clone(), 2, Some(tip), 0).await.unwrap();
                    while segment.close_step().unwrap() {}
                }
                _ => {}
            }
            if name.starts_with("compacted-") {
                storage.delete_segment(&writer, 0).await.unwrap();
            }
            writer.release().await.unwrap();
        }
        let mut indices = storage.list_segments(&document).await.unwrap();
        let mut before = Vec::new();
        for &index in indices.as_slice() {
            before.push((index, segment_bytes(&storage, &document, index).await, storage.segment_state(&document, index).await.unwrap()));
        }
        while indices.close_step() {}
        let result = ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 5).await;
        assert_eq!(result.is_ok(), accepted, "{name}: {:?}", result.as_ref().err());
        for (index, bytes, state) in &before {
            if !accepted || *state == db_storage::WalSegmentState::Sealed || name == "compacted-clean" {
                assert_eq!(segment_bytes(&storage, &document, *index).await, *bytes, "{name}: retained prefix replaced");
                assert_eq!(storage.segment_state(&document, *index).await.unwrap(), *state, "{name}: lifecycle changed");
            }
        }
        match result {
            Ok((mut wal, _)) => {
                let expected_tx = if name == "missing" {
                    1
                } else if name == "compacted-clean" {
                    4
                } else {
                    3
                };
                assert_eq!(wal.next_tx_id, expected_tx, "{name}");
                let receipt = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"z").await), DurabilityClass::Fsync, 6).await;
                assert_eq!(receipt.tx_id, expected_tx);
                assert!(replay_summaries(&storage, &document).await.contains(&ReplaySummary::Commit(expected_tx, 1)), "{name}");
                wal.close().await.unwrap();
            }
            Err(rejected) => {
                let _ = rejected_open_error(rejected).await;
            }
        }
    }
    println!("[DEBUG] WAL recovery: 18 neutral lifecycle rows cover rotation gaps, compaction boundaries, invalid partial headers, sealed damage, identity/chain forgery and exhausted ids without replacement");
}

//#region 🔖️RecordKinds
#[semio_framework_async_macros::async_test]
async fn record_kinds_fill_the_extension_range_uniquely() {
    let kinds = [WAL_SEGMENT_HEADER, WAL_TX_BEGIN, WAL_TX_COMMIT, WAL_TX_ABORT, WAL_COMMAND, WAL_PAYLOAD, WAL_DIFF, WAL_INVERSE, WAL_EVENT, WAL_OUTBOX, WAL_FRONTIER, WAL_VCS_REF, WAL_SNAPSHOT_PUB, WAL_INDEX_CKPT, WAL_LEASE, WAL_MIGRATION];
    assert_eq!(kinds.len(), 16);
    let mut sorted = kinds.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), kinds.len(), "wal record kinds must be pairwise distinct");
    assert_eq!(*sorted.first().unwrap(), 0x40);
    assert_eq!(*sorted.last().unwrap(), 0x4F);
    for kind in kinds {
        assert!(is_wal_record_kind(kind));
    }
    // 🧪️ `0x0C` (protocol::wire::REC_COMMIT) hard-coded rather than depending on protocol_core
    // directly just for this one assertion — this crate's extension range never overlaps it.
    assert!(!is_wal_record_kind(0x0C));
}
//#endregion 🔖️RecordKinds

//#region 🔖️Records
#[semio_framework_async_macros::async_test]
async fn wal_record_round_trips_every_kind_through_encode_decode() {
    let document = doc("doc-1").await;
    let mut samples = WalRecordBatch::new();
    for sample in [
        WalRecord::SegmentHeader { document: document.clone(), segment_index: 0, prev_chain_hash: None },
        WalRecord::SegmentHeader { document: document.clone(), segment_index: 1, prev_chain_hash: Some([3u8; 32]) },
        WalRecord::TxBegin { tx_id: 42 },
        WalRecord::TxCommit { tx_id: 42, record_count: 5 },
        WalRecord::TxAbort { tx_id: 7 },
        WalRecord::Command(retained(b"envelope-bytes").await),
        WalRecord::Payload(WalPayloadRef::Inline(retained(b"small-payload").await)),
        WalRecord::Payload(WalPayloadRef::CasRef(pack::ContentHash([5u8; 32]))),
        WalRecord::Diff(retained(b"diff-bytes").await),
        WalRecord::Inverse(retained(b"inverse-bytes").await),
        WalRecord::Event(retained(b"event-bytes").await),
        WalRecord::Outbox(retained(b"outbox-bytes").await),
        WalRecord::Frontier(sample_frontier(&document).await),
        WalRecord::VcsRef(db_storage::DbIoText::try_from_str("ck-abc123").unwrap()),
        WalRecord::SnapshotPub { generation: 4, frontier: sample_frontier(&document).await },
        WalRecord::IndexCkpt { run_ids: run_ids(&[1, 2, 3, 100]) },
        WalRecord::Lease { resource: db_storage::DbIoText::try_from_str("shard-0").unwrap(), holder: db_storage::DbIoText::try_from_str("node-a").unwrap(), fence: 9, expires_at_ms: 12345 },
        WalRecord::Migration(retained(b"migration-bytes").await),
    ] {
        assert!(samples.push(sample).is_ok());
    }
    for sample in samples.iter() {
        let (kind, critical, payload) = sample.encode().await;
        assert!(critical, "every wal record is critical by design");
        let mut decoded = decode(kind, &payload).await.unwrap();
        let (_, _, round_trip) = decoded.encode().await;
        assert_eq!(round_trip, payload);
        while decoded.close_step().unwrap() {}
    }
    while samples.close_step().unwrap() {}
}

#[semio_framework_async_macros::async_test]
async fn decode_rejects_unknown_kind_and_malformed_payload() {
    assert!(matches!(decode(0x7E, b"").await, Err(DbError::Corrupt(_))));
    assert!(decode(WAL_TX_BEGIN, b"").await.is_err());
}
//#endregion 🔖️Records

//#region 🔖️PayloadTransform
#[semio_framework_async_macros::async_test]
async fn identity_payload_transform_round_trips_without_changing_bytes() {
    let transform = IdentityPayloadTransform;
    let plaintext = b"hello wal";
    let mut control = control();
    let encrypted = transform.encrypt(retained(plaintext).await, &mut control).await.unwrap();
    assert_eq!(read_retained(&encrypted).await, plaintext);
    let mut decrypted = transform.decrypt(encrypted, &mut control).await.unwrap();
    assert_eq!(read_retained(&decrypted).await, plaintext);
    while decrypted.close_step().unwrap().is_some() {}
}

/// @emoji 🔐️ A reversing "cipher" — enough to prove a caller can thread a non-identity
/// `PayloadTransform` through `WalPayloadRef::Inline` end-to-end via this crate's own
/// encode/decode, without `db_wal` itself needing to know encryption happened.
struct ReversingTransform;
impl PayloadTransform for ReversingTransform {
    async fn encrypt(&self, mut plaintext: WalBytes, control: &mut WalCursorControl) -> Result<WalBytes, DbError> {
        let mut prepared = plaintext.prepare_platform().await?;
        let reversed: Vec<u8> = prepared.as_slice().iter().rev().copied().collect();
        while prepared.close_step()? {}
        while plaintext.close_step()?.is_some() {}
        match WalBytes::try_admit(reversed, MAX_FIELD_BYTES, control).await {
            Ok(bytes) => Ok(bytes),
            Err(mut rejected) => {
                while rejected.close_step()? {
                    control.grant()?;
                }
                Err(rejected.into_error())
            }
        }
    }
    async fn decrypt(&self, ciphertext: WalBytes, control: &mut WalCursorControl) -> Result<WalBytes, DbError> {
        self.encrypt(ciphertext, control).await
    }
}

#[semio_framework_async_macros::async_test]
async fn non_identity_payload_transform_round_trips_through_an_inline_wal_payload_record() {
    let transform = ReversingTransform;
    let plaintext = b"round trip me through the wal".to_vec();
    let mut control = control();
    let ciphertext = transform.encrypt(retained(&plaintext).await, &mut control).await.unwrap();
    assert_ne!(read_retained(&ciphertext).await, plaintext, "the transform must actually have changed the bytes");

    let record = WalRecord::Payload(WalPayloadRef::Inline(ciphertext));
    let (kind, _critical, payload) = record.encode().await;
    let decoded = decode(kind, &payload).await.unwrap();
    let WalRecord::Payload(WalPayloadRef::Inline(stored_ciphertext)) = decoded else {
        panic!("expected an inline payload record");
    };

    let mut recovered = transform.decrypt(stored_ciphertext, &mut control).await.unwrap();
    assert_eq!(read_retained(&recovered).await, plaintext);
    while recovered.close_step().unwrap().is_some() {}
}
//#endregion 🔖️PayloadTransform

//#region 🔖️GroupCommit
#[semio_framework_async_macros::async_test]
async fn group_commit_policy_is_due_on_any_threshold() {
    let policy = GroupCommitPolicy { max_delay_ms: 100, max_bytes: 1000, max_records: 10 };
    assert!(!policy.is_due(0, 0, None, 0));
    assert!(policy.is_due(1000, 0, None, 0));
    assert!(policy.is_due(0, 10, None, 0));
    assert!(policy.is_due(0, 0, Some(0), 100));
    assert!(!policy.is_due(0, 0, Some(50), 100));
}
//#endregion 🔖️GroupCommit

//#region 🔖️Segment + ArtifactWal
#[semio_framework_async_macros::async_test]
async fn single_segment_write_commit_flush_recovers_cleanly() {
    let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();

    let receipt = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"cmd-1").await), DurabilityClass::Fsync, 1).await;
    assert!(receipt.committed, "Fsync durability must force an immediate commit");
    assert_eq!(receipt.tx_id, 1);

    let bytes = segment_bytes(&storage, &document, 0).await;
    let report = protocol::format::recover(&bytes, &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).await.unwrap();
    assert_eq!(report.bytes_recovered, bytes.len() as u64);
    assert_eq!(report.torn_tail_bytes, 0);

    assert_eq!(replay_summaries(&storage, &document).await, vec![ReplaySummary::Segment(0, None), ReplaySummary::Begin(1), ReplaySummary::Command(b"cmd-1".to_vec()), ReplaySummary::Commit(1, 1)]);
    wal.close().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn group_commit_batches_until_policy_threshold_then_commits() {
    let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let policy = GroupCommitPolicy { max_delay_ms: 1_000_000, max_bytes: u64::MAX, max_records: 5 };
    let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document.clone(), policy, 0)).unwrap();

    // Each submit writes 3 records (begin/command/commit); Memory durability never forces a
    // commit, so nothing should be flushed to storage until pending_records >= 5.
    let receipt_1 = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"a").await), DurabilityClass::Memory, 10).await;
    assert!(!receipt_1.committed);
    assert_eq!(db_actor::block_on(storage.segment_len(&document, 0)).unwrap(), wal.active.flushed_len, "nothing new should have flushed yet");

    // Second submit pushes pending_records to 6 (>= max_records 5), which must commit.
    let receipt_2 = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"b").await), DurabilityClass::Memory, 11).await;
    assert!(receipt_2.committed);
    assert_eq!(wal.active.pending_records, 0);
    assert!(db_actor::block_on(storage.segment_len(&document, 0)).unwrap() > 32, "flush must have appended past the bare header");
    wal.close().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn fsync_durability_forces_immediate_commit_regardless_of_policy() {
    let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let policy = GroupCommitPolicy { max_delay_ms: u64::MAX, max_bytes: u64::MAX, max_records: u32::MAX };
    let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document, policy, 0)).unwrap();
    let receipt = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"a").await), DurabilityClass::Fsync, 0).await;
    assert!(receipt.committed);
    wal.close().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn torn_tail_is_recovered_by_truncating_only_the_uncommitted_suffix() {
    let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    {
        let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
        submit_one(&storage, &mut wal, WalRecord::Command(retained(b"trusted").await), DurabilityClass::Fsync, 1).await;
        wal.close().await.unwrap();
    }

    // Simulate a crash mid-append: bytes physically present past the last trusted commit,
    // written directly to storage (bypassing SprWriter, exactly like a torn OS-level write).
    let committed = segment_bytes(&storage, &document, 0).await;
    let writer = storage.acquire_writer(&document).await.unwrap();
    storage.append(&writer, 0, pages(b"\x0Fgarbage")).await.unwrap();
    writer.release().await.unwrap();

    let (mut wal, report) = db_actor::block_on(ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 100)).unwrap();
    assert!(report.torn_tail_bytes > 0);
    assert_eq!(report.segments_seen, 1);
    wal.close().await.unwrap();

    assert_eq!(replay_summaries(&storage, &document).await, vec![ReplaySummary::Segment(0, None), ReplaySummary::Begin(1), ReplaySummary::Command(b"trusted".to_vec()), ReplaySummary::Commit(1, 1)]);

    let bytes = segment_bytes(&storage, &document, 0).await;
    let post_recovery = protocol::format::recover(&bytes, &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).await.unwrap();
    assert_eq!(post_recovery.bytes_recovered, bytes.len() as u64, "the truncated segment must itself be torn-tail-free");
    assert_eq!(bytes, committed, "all original commits are byte-preserved");
}

#[semio_framework_async_macros::async_test]
async fn recovery_resumes_next_tx_id_and_accepts_further_submits() {
    let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    {
        let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
        submit_one(&storage, &mut wal, WalRecord::Command(retained(b"one").await), DurabilityClass::Fsync, 1).await;
        submit_one(&storage, &mut wal, WalRecord::Command(retained(b"two").await), DurabilityClass::Fsync, 2).await;
        wal.close().await.unwrap();
    }

    let (mut wal, _report) = db_actor::block_on(ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 3)).unwrap();
    let receipt = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"three").await), DurabilityClass::Fsync, 4).await;
    assert_eq!(receipt.tx_id, 3, "recovery must resume tx ids strictly past whatever was already durable");

    let commands: Vec<_> = replay_summaries(&storage, &document)
        .await
        .into_iter()
        .filter_map(|record| match record {
            ReplaySummary::Command(bytes) => Some(bytes),
            _ => None,
        })
        .collect();
    assert_eq!(commands, vec![b"one".to_vec(), b"two".to_vec(), b"three".to_vec()]);
    wal.close().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn multi_segment_rotation_chains_prev_hash_and_replay_spans_segments() {
    let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
    wal.max_segment_bytes = 200; // force rotation quickly for this test

    for i in 0..20u32 {
        submit_one(&storage, &mut wal, WalRecord::Command(retained(format!("cmd-{i}").as_bytes()).await), DurabilityClass::Fsync, u64::from(i)).await;
    }

    let segments = db_actor::block_on(storage.list_segments(&document)).unwrap();
    assert!(segments.len() >= 2, "the byte threshold must have forced at least one rotation");

    // Cross-check segment 1's WAL_SEGMENT_HEADER.prev_chain_hash against segment 0's
    // independently-recomputed tip chain_hash.
    let seg0_bytes = segment_bytes(&storage, &document, 0).await;
    let seg0_report = protocol::format::recover(&seg0_bytes, &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).await.unwrap();
    let commit_frame_end = (seg0_report.last_commit_offset + protocol::format::COMMIT_FRAME_LEN) as usize;
    let mut cursor = protocol::FrameCursor::new(&seg0_bytes[seg0_report.last_commit_offset as usize..commit_frame_end], 0).await;
    let commit_frame = cursor.next_frame().await.unwrap().unwrap();
    let expected_chain_hash = protocol::format::parse_commit_payload(commit_frame.payload().await).unwrap().chain_hash;

    let full_replay = replay_summaries(&storage, &document).await;
    assert!(full_replay.contains(&ReplaySummary::Segment(0, None)));
    assert!(full_replay.contains(&ReplaySummary::Segment(1, Some(expected_chain_hash))));
    let commands_in_order: Vec<String> = full_replay
        .into_iter()
        .filter_map(|record| match record {
            ReplaySummary::Command(bytes) => Some(String::from_utf8(bytes).unwrap()),
            _ => None,
        })
        .collect();
    let expected: Vec<String> = (0..20u32).map(|i| format!("cmd-{i}")).collect();
    assert_eq!(commands_in_order, expected, "replay must span every segment in rotation order");
    wal.close().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn recovery_rejects_a_torn_non_active_sealed_segment() {
    let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
    wal.max_segment_bytes = 1; // rotate on the very next submit
    submit_one(&storage, &mut wal, WalRecord::Command(retained(b"forces-rotation").await), DurabilityClass::Fsync, 0).await;
    assert!(db_actor::block_on(storage.list_segments(&document)).unwrap().len() >= 2);
    wal.close().await.unwrap();

    // Corrupt the now-sealed segment 0 by truncating a byte off its tail directly in storage
    // — WalStorage::truncate_tail refuses a sealed segment, so simulate on-disk bit rot
    // instead via delete+recreate+append of a shortened copy.
    let seg0_bytes = segment_bytes(&storage, &document, 0).await;
    let writer = storage.acquire_writer(&document).await.unwrap();
    storage.delete_segment(&writer, 0).await.unwrap();
    storage.create_segment(&writer, 0).await.unwrap();
    storage.append(&writer, 0, pages(&seg0_bytes[..seg0_bytes.len() - 1])).await.unwrap();
    storage.seal(&writer, 0).await.unwrap();
    writer.release().await.unwrap();

    let result = db_actor::block_on(ArtifactWal::open(&storage, document, GroupCommitPolicy::default(), 100));
    let rejected = match result {
        Err(rejected) => matches!(rejected_open_error(rejected).await, DbError::Corrupt(_)),
        Ok((mut wal, _)) => {
            wal.close().await.unwrap();
            false
        }
    };
    assert!(rejected, "a torn sealed (non-active) segment must be a hard recovery error");
}

#[semio_framework_async_macros::async_test]
async fn empty_document_open_creates_a_fresh_wal() {
    let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let (mut wal, report) = db_actor::block_on(ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
    assert_eq!(report, WalRecoveryReport::default());
    assert_eq!(wal.active_segment_index().await, 0);
    assert_eq!(db_actor::block_on(storage.list_segments(&document)).unwrap(), vec![0]);
    wal.close().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn artifact_wal_open_rejection_retains_exact_writer_for_close_or_same_owner_retry() {
    let inner = std::sync::Arc::new(db_storage::DbBackend::Memory(MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
    let storage = crate::db_fault_testing::FaultStorage::new(inner).await;
    let document = ArtifactId::from("wal-open-retained-owner");
    storage.set_script(crate::db_fault_testing::FaultScript { fail_nth_write: Some(1), ..crate::db_fault_testing::FaultScript::default() }).await;
    let rejected = match ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).await {
        Err(rejected) => rejected,
        Ok(mut wal) => {
            wal.close().await.unwrap();
            panic!("faulted WAL create was admitted");
        }
    };
    assert!(matches!(rejected.error(), DbError::Io(_)));
    assert!(rejected.has_release_owner());
    assert!(matches!(storage.acquire_writer(&document).await, Err(DbError::Conflict(_))));
    assert!(matches!(rejected_open_error(rejected).await, DbError::Io(_)));

    let writer = storage.acquire_writer(&document).await.unwrap();
    storage.set_script(crate::db_fault_testing::FaultScript { fail_nth_write: Some(storage.append_calls().await + 1), ..crate::db_fault_testing::FaultScript::default() }).await;
    let mut open_control = control();
    let rejected = match ArtifactWal::open_acquired(&storage, writer, GroupCommitPolicy::default(), 1, &mut open_control).await {
        Err(rejected) => rejected,
        Ok((mut wal, _)) => {
            wal.close().await.unwrap();
            panic!("faulted acquired WAL open was admitted");
        }
    };
    assert!(matches!(rejected.error(), DbError::Io(_)));
    assert!(matches!(storage.acquire_writer(&document).await, Err(DbError::Conflict(_))));
    storage.set_script(crate::db_fault_testing::FaultScript::default()).await;
    let mut retry_control = control();
    let (mut wal, report) = rejected.retry_open(&storage, GroupCommitPolicy::default(), 2, &mut retry_control).await.unwrap();
    assert_eq!(report.segments_seen, 1);
    wal.close().await.unwrap();
    storage.acquire_writer(&document).await.unwrap().release().await.unwrap();
    eprintln!("[DEBUG] WAL open rejection retained exact permit for same-owner retry and exact release for terminal close");
}
//#endregion 🔖️Segment + ArtifactWal
