/// 🧰️ WAL test fixtures: the page, document, frontier and control helpers, the rejected open error, the replay summaries, the segment and committed fixtures and the no-committed-transaction assertion.

fn pages(bytes: &[u8]) -> db_io_core::DbIoPages {
    let mut writer = db_io_core::DbIoPageWriter::try_reserve(bytes.len().div_ceil(db_io_core::DB_IO_PAGE_BYTES)).expect("test WAL writer admitted");
    for fragment in bytes.chunks(db_io_core::DB_IO_PAGE_BYTES) {
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

async fn submit_one(storage: &impl WalStorage, wal: &mut ArtifactWal, record: WalRecord, durability: DurabilityClass, now_ms: u64) -> WalAppendReceipt {
    let mut records = WalRecordBatch::new();
    assert!(records.push(record).is_ok());
    let receipt = wal.submit(storage, &[], &records, durability, now_ms).await.unwrap();
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
    let mut prepared = db_io_registry::db_io_prepare_platform(&pages).unwrap().await.unwrap();
    let output = prepared.as_slice().to_vec();
    while prepared.close_step().unwrap() {}
    while pages.close_step().unwrap().is_some() {}
    output
}

fn recovery_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🚑️recovery/🔣️.json")).unwrap()
}

pub async fn committed_fixture_storage(row: &serde_json::Value, document: &ArtifactId) -> MemoryStorage {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    write_committed_fixture(&storage, row, document).await;
    storage
}

async fn write_committed_fixture(storage: &impl WalStorage, row: &serde_json::Value, document: &ArtifactId) {
    let writer_permit = storage.acquire_writer(document).await.unwrap();
    let mut previous = None;
    for (index, segment) in row["segments"].as_array().unwrap().iter().enumerate() {
        let mut writer = SegmentWriter::begin(storage, &writer_permit, document.clone(), index as u64, previous, 0).await.unwrap();
        let frames = segment["frames"].as_array().unwrap().iter().flat_map(|frame| std::iter::repeat_n(frame, frame["repeat"].as_u64().unwrap_or(1) as usize));
        for (ordinal, frame) in frames.enumerate().skip(1) {
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
        previous = Some(writer.tip_chain_hash().unwrap());
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
