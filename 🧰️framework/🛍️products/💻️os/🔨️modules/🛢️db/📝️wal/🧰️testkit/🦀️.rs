//! 🧰️ Shared WAL test fixtures for the other db crates (behind the `testkit` feature).

use super::*;

use db_io_core::WalStorage;
use db_storage_memory::MemoryStorage;
use db_wal_log::{MAX_FIELD_BYTES, WalBytes, WalCursorControl, WalPayloadRef, WalRecord};

async fn sample_frontier(document: &ArtifactId) -> Frontier {
    Frontier { document: document.clone(), head_seq: 7, commit_seq: 3, chain_hash: [9u8; 32], epoch: 1 }
}

fn control() -> WalCursorControl {
    WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap()
}

async fn retained(source: &[u8]) -> WalBytes {
    let mut control = control();
    WalBytes::try_admit(source.to_vec(), MAX_FIELD_BYTES, &mut control).await.unwrap()
}

pub async fn committed_fixture_storage(row: &serde_json::Value, document: &ArtifactId) -> MemoryStorage {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    write_committed_fixture(&storage, row, document).await;
    storage
}

pub async fn aborted_event_fixture_storage(document: &ArtifactId, event: &[u8]) -> MemoryStorage {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
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
