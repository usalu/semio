use super::*;

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

fn run_ids(values: &[u64]) -> db_io_core::DbIoU64List {
    let mut list = db_io_core::DbIoU64List::new();
    for value in values {
        list.push(*value).unwrap();
    }
    list
}

#[semio_framework_async_macros::async_test]
async fn wal_retained_varints_match_neutral_exact_u64_and_atomic_interruption() {
    let _pool = db_io_driver::db_io_test_pool();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📖️retained-decoder/🔣️.json")).unwrap();
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
    let _pool = db_io_driver::db_io_test_pool();
    let expected: Vec<_> = (0..db_io_core::DB_IO_PAGE_BYTES * 3 + 17).map(|index| (index % 251) as u8).collect();
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
    let _pool = db_io_driver::db_io_test_pool();
    let expected = vec![0x57; db_io_core::DB_IO_PAGE_BYTES + 7];
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
//#region 🔖️RecordKinds
#[semio_framework_async_macros::async_test]
async fn record_kinds_are_distinct_members_of_the_extension_range() {
    let kinds = [WAL_SEGMENT_HEADER, WAL_TX_BEGIN, WAL_TX_COMMIT, WAL_TX_ABORT, WAL_COMMAND, WAL_PAYLOAD, WAL_DIFF, WAL_INVERSE, WAL_EVENT, WAL_OUTBOX, WAL_FRONTIER, WAL_SNAPSHOT_PUB, WAL_INDEX_CKPT, WAL_LEASE, WAL_MIGRATION];
    assert_eq!(kinds.len(), 15);
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
        WalRecord::SnapshotPub { generation: 4, frontier: sample_frontier(&document).await },
        WalRecord::IndexCkpt { run_ids: run_ids(&[1, 2, 3, 100]) },
        WalRecord::Lease { resource: db_io_core::DbIoText::try_from_str("shard-0").unwrap(), holder: db_io_core::DbIoText::try_from_str("node-a").unwrap(), fence: 9, expires_at_ms: 12345 },
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

/// 🔐️ A reversing "cipher" — enough to prove a caller can thread a non-identity
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

//#region 🔖️Source laws
#[test]
fn history_crc_faults_are_authenticated_by_the_wal_chain() {
    assert!(include_str!("../../🦀️.rs").contains("wal chain crc or frame length differs"), "history CRC faults are authenticated by the WAL chain");
}

#[test]
fn wal_crc_cursor_and_frame_tokenizer_advance_one_page_per_grant() {
    let wal = include_str!("../../🦀️.rs");
    assert!(wal.contains("protocol::codec::Crc32cCursor"));
    assert!(wal.contains("frame.crc.update_page(&bytes[crc_start - frame.position..crc_end - frame.position])"));
    assert!(wal.contains("fragment.len().min(db_io_core::DB_IO_PAGE_BYTES)"));
}
//#endregion 🔖️Source laws
