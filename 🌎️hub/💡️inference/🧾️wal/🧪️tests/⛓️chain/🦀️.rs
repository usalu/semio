//! ⛓️ Real replay and independent third-party BLAKE3 over hostile CRC-valid committed frames.

use super::*;

#[derive(Clone, Copy)]
struct Frame {
    start: usize,
    body: usize,
    end: usize,
    next: usize,
    kind: u8,
}

fn varint(bytes: &[u8], offset: &mut usize) -> usize {
    let mut value = 0;
    for shift in (0..70).step_by(7) {
        let byte = bytes[*offset];
        *offset += 1;
        value |= usize::from(byte & 127) << shift;
        if byte & 128 == 0 {
            return value;
        }
    }
    panic!("fixture varint exceeds u64");
}

fn frames(bytes: &[u8]) -> Vec<Frame> {
    let mut offset = 32;
    let mut output = Vec::new();
    while offset < bytes.len() {
        let start = offset;
        let length = varint(bytes, &mut offset);
        let end = offset + length;
        output.push(Frame { start, body: offset, end, next: end + 8, kind: bytes[offset] });
        offset = end + 8;
    }
    assert_eq!(offset, bytes.len());
    output
}

fn u64_at(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap())
}

fn independent_chain(segments: &[Vec<u8>], document: &str) -> bool {
    let mut previous = None;
    for (index, bytes) in segments.iter().enumerate() {
        if bytes[12..16] != 1u32.to_le_bytes() {
            return false;
        }
        let mut chain = *blake3::hash(&bytes[..32]).as_bytes();
        let mut pending = blake3::Hasher::new();
        pending.update(&chain);
        let mut count = 0u32;
        let mut length = 0u64;
        let mut sequence = 1;
        let mut offset = 0;
        let all = frames(bytes);
        for (position, frame) in all.iter().enumerate() {
            let payload = &bytes[frame.body + 2..frame.end];
            if bytes[frame.body + 1] != protocol::wire::FRAME_FLAG_CRITICAL {
                return false;
            }
            if frame.kind == protocol::wire::REC_COMMIT {
                if payload.len() != 64
                    || u64_at(payload, 0) != sequence
                    || u64_at(payload, 8) != offset
                    || u64_at(payload, 16) != length
                    || u32::from_le_bytes(payload[24..28].try_into().unwrap()) != count
                    || payload[28..32] != [0; 4]
                    || payload[32..] != *pending.finalize().as_bytes()
                {
                    return false;
                }
                chain.copy_from_slice(&payload[32..]);
                pending = blake3::Hasher::new();
                pending.update(&chain);
                count = 0;
                length = 0;
                sequence += 1;
                offset = frame.start as u64;
            } else {
                if position == 0 {
                    if frame.kind != db::wal::WAL_SEGMENT_HEADER {
                        return false;
                    }
                    let mut cursor = 0;
                    let len = varint(payload, &mut cursor);
                    if payload[cursor..cursor + len] != *document.as_bytes() {
                        return false;
                    }
                    cursor += len;
                    if u64_at(payload, cursor) != index as u64 {
                        return false;
                    }
                    cursor += 8;
                    if index == 0 {
                        if payload[cursor..] != [0] {
                            return false;
                        }
                    } else if payload[cursor] != 1 || payload[cursor + 1..] != previous.unwrap() {
                        return false;
                    }
                } else if frame.kind == db::wal::WAL_SEGMENT_HEADER {
                    return false;
                }
                pending.update(blake3::hash(&bytes[frame.start..frame.next]).as_bytes());
                count += 1;
                length += (frame.next - frame.start) as u64;
            }
        }
        if count != 0 || sequence == 1 || all.last().unwrap().kind != protocol::wire::REC_COMMIT {
            return false;
        }
        previous = Some(chain);
    }
    true
}

async fn chain_segments(fixture: &serde_json::Value, case: &serde_json::Value) -> Vec<Vec<u8>> {
    let count = case["segments"].as_u64().unwrap();
    let mutation = case["mutation"].as_str().unwrap();
    let mut segments: Vec<Vec<u8>> = Vec::new();
    for index in 0..count {
        let flags = if mutation == "missing-required-chain" { 0 } else { 1 };
        let mut writer = protocol::SprWriter::begin(Vec::new(), &protocol::format::WriteOptions { required_flags: flags, optional_flags: 0 }).await.unwrap();
        let mut header = Vec::new();
        let document = if index == 1 && mutation == "wrong-segment-document" { "other-document" } else { fixture["documentKey"].as_str().unwrap() };
        protocol::write_str(&mut header, document);
        header.extend_from_slice(&(if index == 1 && mutation == "skipped-segment-index" { 2 } else { index }).to_le_bytes());
        header.push(u8::from(index != 0));
        if index != 0 {
            let previous = segments.last().unwrap();
            let mut tip = previous[previous.len() - 40..previous.len() - 8].to_vec();
            if mutation == "wrong-prior-tip" {
                tip[0] ^= 1;
            }
            header.extend_from_slice(&tip);
        }
        writer.write_record(db::wal::WAL_SEGMENT_HEADER, true, &header, protocol::codec::ids::CodecId(0)).await.unwrap();
        writer.commit().await.unwrap();
        let transactions = if count == 1 { 1..=2 } else { index + 1..=index + 1 };
        for tx in transactions {
            writer.write_record(db::wal::WAL_TX_BEGIN, true, &tx.to_le_bytes(), protocol::codec::ids::CodecId(0)).await.unwrap();
            let mut command = Vec::new();
            protocol::encode_envelope(&envelope(fixture), &mut command);
            if tx == 1 {
                let last = command.len() - 1;
                command[last] ^= 1;
            }
            writer.write_record(db::wal::WAL_COMMAND, true, &command, protocol::codec::ids::CodecId(0)).await.unwrap();
            let mut commit = tx.to_le_bytes().to_vec();
            commit.extend_from_slice(&1u32.to_le_bytes());
            writer.write_record(db::wal::WAL_TX_COMMIT, true, &commit, protocol::codec::ids::CodecId(0)).await.unwrap();
            writer.commit().await.unwrap();
        }
        segments.push(writer.into_sink().await);
    }
    let first = &mut segments[0];
    let all = frames(first);
    let selected = if mutation == "record-crc-repaired" {
        all.iter().find(|frame| frame.kind == db::wal::WAL_COMMAND).copied()
    } else if mutation.ends_with("crc-repaired") {
        all.iter().filter(|frame| frame.kind == protocol::wire::REC_COMMIT).nth(1).copied()
    } else {
        None
    };
    if let Some(frame) = selected {
        let payload = frame.body + 2;
        let offset = match mutation {
            "record-crc-repaired" => frame.end - 1,
            "commit-hash-crc-repaired" => payload + 32,
            "commit-count-crc-repaired" => payload + 24,
            "commit-length-crc-repaired" => payload + 16,
            "commit-sequence-crc-repaired" => payload,
            "commit-offset-crc-repaired" => payload + 8,
            "noncritical-commit-crc-repaired" => frame.body + 1,
            "commit-reserved-crc-repaired" => payload + 28,
            _ => panic!("unknown literal tamper"),
        };
        first[offset] ^= match mutation {
            "noncritical-commit-crc-repaired" => protocol::wire::FRAME_FLAG_CRITICAL,
            "record-crc-repaired" => 2,
            _ => 1,
        };
        let crc = protocol::codec::crc32c(&first[frame.body..frame.end]);
        first[frame.end..frame.end + 4].copy_from_slice(&crc.to_le_bytes());
    }
    segments
}

async fn retained_storage(fixture: &serde_json::Value, segments: &[Vec<u8>], first: usize) -> Arc<db::storage::DbBackend> {
    let pool = Arc::new(db::semio_framework_async::process_worker_pool(db::semio_framework_async::WorkerPoolConfig::new(db::semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let backend = db::storage::MemoryStorage::new(pool).await.unwrap();
    let document = db::ArtifactId(fixture["documentKey"].as_str().unwrap().into());
    for (index, bytes) in segments.iter().enumerate() {
        backend.create_segment(&document, index as u64).await.unwrap();
        let mut pages = db::storage::DbIoPageWriter::try_reserve(bytes.len().div_ceil(db::storage::DB_IO_PAGE_BYTES)).unwrap();
        for fragment in bytes.chunks(db::storage::DB_IO_PAGE_BYTES) {
            assert_eq!(pages.write_fragment(fragment).unwrap(), fragment.len());
        }
        backend.append(&document, index as u64, pages.seal_retained().await.unwrap()).await.unwrap();
        backend.sync(&document, index as u64, db::DurabilityClass::Fsync).await.unwrap();
        if index + 1 < segments.len() {
            backend.seal(&document, index as u64).await.unwrap();
        }
    }
    for index in 0..first {
        backend.delete_segment(&document, index as u64).await.unwrap();
    }
    Arc::new(db::storage::DbBackend::Memory(backend))
}

#[tokio::test]
async fn inference_wal_chain_rejects_crc_valid_tampering_and_exact_cross_segment_tip_mismatch() {
    let chain: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/⛓️inference-wal-chain-v1/🔣️.json")).unwrap();
    let fixture = fixture();
    for case in chain["cases"].as_array().unwrap() {
        let segments = chain_segments(&fixture, case).await;
        for bytes in &segments {
            for frame in frames(bytes) {
                assert_eq!(protocol::codec::crc32c(&bytes[frame.body..frame.end]), u32::from_le_bytes(bytes[frame.end..frame.end + 4].try_into().unwrap()));
            }
        }
        let expected = case["accepted"].as_bool().unwrap();
        assert_eq!(independent_chain(&segments, fixture["documentKey"].as_str().unwrap()), expected, "independent blake3 {}", case["name"]);
        let verifier = InferenceWalVerifierV1::new(retained_storage(&fixture, &segments, 0).await);
        let fence = Arc::new(InferenceDocumentFenceV1::new(scope(&fixture), 17).unwrap());
        let result = verifier.verify(target(&fixture, &fixture["traces"][0]), fence, Arc::new(InferenceOperationControlV1::new(2000, 64).unwrap())).await;
        assert_eq!(matches!(result, Ok(Some(_))), expected, "actual retained WAL {}", case["name"]);
        assert_eq!(verifier.active(), 0);
        assert!(verifier.close_steps() > 0);
    }
}

#[tokio::test]
async fn inference_wal_chain_cancellation_retires_hashing_and_compacted_suffix_is_not_a_genesis_proof() {
    let chain: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/⛓️inference-wal-chain-v1/🔣️.json")).unwrap();
    let fixture = fixture();
    let segments = chain_segments(&fixture, &chain["cases"][0]).await;
    for owner in chain["hashingOwnership"].as_array().unwrap() {
        let mut verifier = InferenceWalVerifierV1::new(retained_storage(&fixture, &segments, 0).await);
        let gate = Arc::new(tokio::sync::Semaphore::new(0));
        Arc::get_mut(&mut verifier.state).unwrap().hashing_gate = Some(gate.clone());
        let fence = Arc::new(InferenceDocumentFenceV1::new(scope(&fixture), 17).unwrap());
        let control = Arc::new(InferenceOperationControlV1::new(2000, 64).unwrap());
        let mut future = Box::pin(verifier.verify(target(&fixture, &fixture["traces"][0]), fence, control.clone()));
        assert!(futures::poll!(future.as_mut()).is_pending());
        tokio::time::timeout(Duration::from_secs(2), async {
            while verifier.state.hashing_steps.load(Ordering::Acquire) == 0 {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        let expected = if owner["expected"] == "expired" { InferenceErrorV1::Expired } else { InferenceErrorV1::Cancelled };
        match owner["interrupt"].as_str().unwrap() {
            "drop" => drop(future),
            "cancel" => {
                control.cancel();
                assert!(matches!(tokio::time::timeout(Duration::from_secs(1), future).await.unwrap(), Err(error) if error == expected));
            }
            "deadline" => {
                assert!(matches!(tokio::time::timeout(Duration::from_secs(3), future).await.unwrap(), Err(error) if error == expected));
            }
            _ => panic!("unknown hashing interrupt"),
        }
        assert_eq!(control.checkpoint(0), Err(expected));
        assert_eq!(verifier.active() as u64, owner["heldActive"].as_u64().unwrap());
        assert_eq!(verifier.close_steps(), 0);
        assert_eq!(control.progress().0, owner["stoppedProgress"].as_u64().unwrap());
        assert_eq!(verifier.state.hashing_steps.load(Ordering::Acquire), owner["hashingSteps"].as_u64().unwrap());
        gate.add_permits(1);
        tokio::time::timeout(Duration::from_secs(2), async {
            while verifier.active() != 0 {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        assert_eq!(verifier.active() as u64, owner["releasedActive"].as_u64().unwrap());
        assert!(verifier.close_steps() > 0);
        assert_eq!(control.progress().0, 0);
        assert_eq!(verifier.state.hashing_steps.load(Ordering::Acquire), 1);
    }
    for boundary in chain["retainedBoundaries"].as_array().unwrap() {
        let segments = chain_segments(&fixture, &serde_json::json!({"segments": 2, "mutation": boundary["mutation"]})).await;
        let backend = retained_storage(&fixture, &segments, 1).await;
        let storage = backend.wal().await;
        let document = db::ArtifactId(fixture["documentKey"].as_str().unwrap().into());
        let control = WalCursorControl::new(Arc::new(AtomicBool::new(false)), Instant::now() + Duration::from_secs(2), 65_536).unwrap();
        let mut replay = WalReplayCursor::open(&storage, &document, control).await.unwrap();
        let mut records = 0;
        loop {
            match replay.next_step().await.unwrap() {
                WalReplayStep::Record(mut record) => {
                    records += 1;
                    while record.close_step().unwrap() {}
                }
                WalReplayStep::Yield => tokio::task::yield_now().await,
                WalReplayStep::Done => break,
            }
        }
        while replay.close_owner_step().unwrap() {}
        assert!(replay.terminal_is_empty());
        assert_eq!(records == 4, boundary["replayAccepted"].as_bool().unwrap());
        drop(replay);
        drop(storage);
        let verifier = InferenceWalVerifierV1::new(backend);
        let fence = Arc::new(InferenceDocumentFenceV1::new(scope(&fixture), 17).unwrap());
        let result = verifier.verify(target(&fixture, &fixture["traces"][0]), fence, Arc::new(InferenceOperationControlV1::new(2000, 64).unwrap())).await;
        assert_eq!(matches!(result, Ok(Some(_))), boundary["genesisProofAccepted"].as_bool().unwrap());
        assert!(matches!(result, Err(InferenceErrorV1::Storage)));
        assert_eq!(verifier.active(), 0);
        assert!(verifier.close_steps() > 0);
    }
}
