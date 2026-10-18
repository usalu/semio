/// 🧮️ Capacity preflight and neutral recovery laws: the capacity helpers, the capacity preflight boundaries, the recovery seed and the neutral committed prefix and lifecycle recovery.

async fn capacity_submission(storage: &impl WalStorage, wal: &mut ArtifactWal, length: usize, durability: DurabilityClass) -> Result<WalAppendReceipt, DbError> {
    let mut batch = WalRecordBatch::new();
    assert!(batch.push(WalRecord::Command(retained(&vec![b'a'; length]).await)).is_ok());
    let result = wal.submit(storage, &[], &batch, durability, 0).await;
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
    wal.close().await.unwrap();
    let (mut reopened, _) = ArtifactWal::open(storage, document.clone(), policy, 1).await.unwrap();
    assert_eq!(reopened.next_tx_id, 4);
    assert_eq!(segment_bytes(storage, &document, 0).await, before);
    assert_eq!(replay_summaries(storage, &document).await.iter().filter(|record| matches!(record, ReplaySummary::Command(_))).count(), 3);
    reopened.close().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn wal_capacity_preflight_matches_neutral_memory_and_filesystem_boundaries() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📏️capacity/🔣️.json")).unwrap();
    for (ordinal, case) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
        capacity_backend(&storage, &fixture, case).await;
        #[cfg(not(target_arch = "wasm32"))]
        {
            let base = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
            let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
            let root = base.join(format!("wal-capacity-{}-{nonce}-{ordinal}", std::process::id()));
            let filesystem = db_storage_fs::FsStorage::open(db_io_driver::db_io_test_pool(), &root).await.unwrap();
            capacity_backend(&filesystem, &fixture, case).await;
            filesystem.close().await.unwrap();
        }
    }
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document = doc("d").await;
    let mut wal = ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).await.unwrap();
    capacity_submission(&storage, &mut wal, fixture["exactPayloadBytes"].as_u64().unwrap() as usize, DurabilityClass::Fsync).await.unwrap();
    assert_eq!(storage.segment_len(&document, 0).await.unwrap(), fixture["maxSegmentBytes"].as_u64().unwrap());
    wal.close().await.unwrap();
    let (mut wal, _) = ArtifactWal::open(&storage, document, GroupCommitPolicy::default(), 0).await.unwrap();
    wal.close().await.unwrap();
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
    let source = SharedBuf::at(0);
    let mut sink = source.clone();
    let mut cursor = 0;
    for chunk in [1_000usize, 15_384, 32_768] {
        source.admit(chunk as u64).unwrap();
        semio_framework_pack_source::PackSink::write_all(&mut sink, &input[cursor..cursor + chunk]).await.unwrap();
        cursor += chunk;
    }
    assert_eq!(cursor, input.len());
    for row in fixture["fragmentCopies"].as_array().unwrap() {
        let offset = row["offset"].as_u64().unwrap() as usize;
        let length = row["length"].as_u64().unwrap() as usize;
        let mut copied = source.copy_range(offset, length).await.unwrap();
        assert_eq!(wal_crc_range(&copied, 0, length, &mut control()).unwrap(), row["crc32c"].as_u64().unwrap() as u32);
        let mut actual = vec![0; length];
        source.read_exact(offset, &mut actual).await.unwrap();
        assert_eq!(actual, input[offset..offset + length]);
        while copied.close_step().unwrap().is_some() {
            semio_framework_async::yield_once().await;
        }
    }
    drop(sink);
    while source.release_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    assert_eq!(semio_framework_pack_source::PackSink::position(&source).await, input.len() as u64, "a released window keeps its position");
    let mut byte = [0u8; 1];
    assert!(source.read_exact(0, &mut byte).await.is_err(), "flushed bytes leave the window");
    let document = doc(fixture["document"].as_str().unwrap()).await;
    let seed = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let mut seeded = recovery_seed(&seed, &document).await;
    seeded.close().await.unwrap();
    let full = segment_bytes(&seed, &document, 0).await;
    assert_eq!(full.len() as u64, fixture["commitEnds"][2].as_u64().unwrap());
    for row in fixture["cuts"].as_array().unwrap() {
        let cut = row["cut"].as_u64().unwrap() as usize;
        let trusted = row["trustedEnd"].as_u64().unwrap() as usize;
        let recovered = row["recoveredEnd"].as_u64().unwrap() as usize;
        let next_tx = row["nextTxId"].as_u64().unwrap();
        let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
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
}

#[semio_framework_async_macros::async_test]
async fn wal_recovery_matches_neutral_lifecycle_without_prefix_replacement() {
    let fixture = recovery_fixture();
    let document = doc(fixture["document"].as_str().unwrap()).await;
    for row in fixture["lifecycle"].as_array().unwrap() {
        let name = row["name"].as_str().unwrap();
        let accepted = row["accepted"].as_bool().unwrap();
        let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
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
            let tip = wal.active.tip_chain_hash().unwrap();
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
            if !accepted || *state == db_io_core::WalSegmentState::Sealed || name == "compacted-clean" {
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
}
