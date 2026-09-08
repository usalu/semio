
use super::*;

//#region 🔖️Header
#[semio_framework_async_macros::async_test]
async fn header_round_trips_via_begin_and_validate() {
    let options = WriteOptions { required_flags: crate::REQUIRED_HASH_CHAIN, optional_flags: crate::OPTIONAL_CANONICAL };
    let writer = SprWriter::begin(Vec::new(), &options).await.unwrap();
    let bytes = writer.into_sink().await;
    assert_eq!(bytes.len(), HEADER_SIZE);
    assert_eq!(&bytes[0..8], &MAGIC);
    assert!(validate_header(&bytes).await.is_ok());
}

#[semio_framework_async_macros::async_test]
async fn header_rejects_bad_magic() {
    let mut bytes = build_header_bytes(0, 0).await.to_vec();
    bytes[0] = 0x00;
    assert!(matches!(validate_header(&bytes).await, Err(ProtocolError::Pack(PackError::BadMagic))));
}

#[semio_framework_async_macros::async_test]
async fn header_rejects_unknown_required_flags() {
    let bytes = build_header_bytes(1 << 5, 0).await.to_vec();
    let err = validate_header(&bytes).await.unwrap_err();
    assert!(matches!(err, ProtocolError::Pack(PackError::UnknownRequiredFlags(bits)) if bits == 1 << 5));
}

#[semio_framework_async_macros::async_test]
async fn header_rejects_corrupted_crc() {
    let mut bytes = build_header_bytes(0, 0).await.to_vec();
    bytes[15] ^= 0xFF;
    assert!(matches!(validate_header(&bytes).await, Err(ProtocolError::Pack(PackError::ChecksumMismatch { .. }))));
}

#[semio_framework_async_macros::async_test]
async fn read_header_exposes_decoded_fields_to_downstream_crates() {
    let bytes = build_header_bytes(crate::REQUIRED_HASH_CHAIN, crate::OPTIONAL_CANONICAL).await.to_vec();
    let header = read_header(&bytes).await.unwrap();
    assert_eq!(header, Header { version_major: FORMAT_VERSION_MAJOR, version_minor: FORMAT_VERSION_MINOR, required_flags: crate::REQUIRED_HASH_CHAIN, optional_flags: crate::OPTIONAL_CANONICAL });
}

#[semio_framework_async_macros::async_test]
async fn read_header_propagates_validation_failures() {
    let mut bytes = build_header_bytes(0, 0).await.to_vec();
    bytes[0] = 0x00;
    assert!(matches!(read_header(&bytes).await, Err(ProtocolError::Pack(PackError::BadMagic))));
}
//#endregion 🔖️Header

//#region 🔖️Frame
async fn build_small_file() -> Vec<u8> {
    let options = WriteOptions::default();
    let mut writer = SprWriter::begin(Vec::new(), &options).await.unwrap();
    writer.write_record(crate::REC_DOC, true, b"doc-payload", crate::codec::ids::CodecId(0)).await.unwrap();
    writer.write_record(crate::REC_EDIT, true, b"edit-payload", crate::codec::ids::CodecId(0)).await.unwrap();
    writer.commit().await.unwrap();
    writer.into_sink().await
}

#[semio_framework_async_macros::async_test]
async fn frame_round_trips_kind_flags_and_payload() {
    let bytes = build_small_file().await;
    let mut cursor = FrameCursor::new(&bytes, HEADER_SIZE as u64).await;
    let doc = cursor.next_frame().await.unwrap().unwrap();
    assert_eq!(doc.kind, crate::REC_DOC);
    assert_eq!(doc.payload().await, b"doc-payload");
    assert_eq!(doc.flags & FRAME_FLAG_CRITICAL, FRAME_FLAG_CRITICAL);
    assert_eq!(doc.offset, HEADER_SIZE as u64);

    let edit = cursor.next_frame().await.unwrap().unwrap();
    assert_eq!(edit.kind, crate::REC_EDIT);
    assert_eq!(edit.payload().await, b"edit-payload");

    let commit = cursor.next_frame().await.unwrap().unwrap();
    assert_eq!(commit.kind, crate::REC_COMMIT);
    assert_eq!(commit.stored.len(), 64);

    assert!(cursor.next_frame().await.unwrap().is_none());
}

#[semio_framework_async_macros::async_test]
async fn frame_len_matches_actual_bytes_consumed() {
    let bytes = build_small_file().await;
    let mut cursor = FrameCursor::new(&bytes, HEADER_SIZE as u64).await;
    let mut pos = HEADER_SIZE as u64;
    while let Some(frame) = cursor.next_frame().await.unwrap() {
        assert_eq!(frame.offset, pos);
        pos += frame.frame_len().await;
    }
    assert_eq!(pos, bytes.len() as u64);
}

#[semio_framework_async_macros::async_test]
async fn reverse_cursor_yields_frames_in_reverse_order() {
    let bytes = build_small_file().await;
    let mut forward = FrameCursor::new(&bytes, HEADER_SIZE as u64).await;
    let mut forward_kinds = Vec::new();
    while let Some(frame) = forward.next_frame().await.unwrap() {
        forward_kinds.push(frame.kind);
    }

    // ReverseFrameCursor has no header-boundary awareness (see its doc comment) — a whole-file
    // walk slices the header off first so `prev_frame` naturally bottoms out at pos == 0.
    let mut reverse = ReverseFrameCursor::at_end(&bytes[HEADER_SIZE..]).await;
    let mut reverse_kinds = Vec::new();
    while let Some(frame) = reverse.prev_frame().await.unwrap() {
        reverse_kinds.push(frame.kind);
    }
    reverse_kinds.reverse();
    assert_eq!(forward_kinds, reverse_kinds);
}

#[semio_framework_async_macros::async_test]
async fn reverse_cursor_at_start_of_stream_returns_none() {
    let mut cursor = ReverseFrameCursor::at_end(&[]).await;
    assert!(cursor.prev_frame().await.unwrap().is_none());
}

#[semio_framework_async_macros::async_test]
async fn reverse_cursor_stops_exactly_at_record_stream_start() {
    let bytes = build_small_file().await;
    let mut cursor = ReverseFrameCursor::at_end(&bytes[HEADER_SIZE..]).await;
    let mut count = 0;
    while cursor.prev_frame().await.unwrap().is_some() {
        count += 1;
    }
    assert_eq!(count, 3); // 2 records + 1 commit
    assert!(cursor.prev_frame().await.unwrap().is_none(), "must keep returning None, not error, once exhausted");
}

#[semio_framework_async_macros::async_test]
async fn frame_detects_crc_corruption() {
    let mut bytes = build_small_file().await;
    let corrupt_at = HEADER_SIZE + 4;
    bytes[corrupt_at] ^= 0xFF;
    let mut cursor = FrameCursor::new(&bytes, HEADER_SIZE as u64).await;
    let err = cursor.next_frame().await.unwrap_err();
    assert!(matches!(err, ProtocolError::Pack(PackError::ChecksumMismatch { .. })));
}

#[semio_framework_async_macros::async_test]
async fn frame_detects_back_len_tamper_via_reverse_cursor() {
    let mut bytes = build_small_file().await;
    let last = bytes.len();
    bytes[last - 1] ^= 0xFF;
    let mut cursor = ReverseFrameCursor::at_end(&bytes).await;
    let err = cursor.prev_frame().await.unwrap_err();
    assert!(matches!(err, ProtocolError::Pack(PackError::ChecksumMismatch { .. }) | ProtocolError::FrameFraming(_)));
}

#[semio_framework_async_macros::async_test]
async fn cursor_surfaces_unrecognized_extension_kind_transparently() {
    // 🎯️ "Skip-unknown" for this crate means: an unrecognized kind (extension range, critical
    // bit clear) is structurally indistinguishable from a known one — the cursor advances past
    // it and returns it like any other frame; interpreting/skipping it is a caller decision
    // (see the module doc). This proves the cursor never special-cases `kind`.
    let options = WriteOptions::default();
    let mut writer = SprWriter::begin(Vec::new(), &options).await.unwrap();
    writer.write_record(0x50, false, b"extension-payload", crate::codec::ids::CodecId(0)).await.unwrap();
    writer.write_record(crate::REC_DOC, true, b"doc-payload", crate::codec::ids::CodecId(0)).await.unwrap();
    let bytes = writer.into_sink().await;

    let mut cursor = FrameCursor::new(&bytes, HEADER_SIZE as u64).await;
    let ext = cursor.next_frame().await.unwrap().unwrap();
    assert_eq!(ext.kind, 0x50);
    assert_eq!(ext.flags & FRAME_FLAG_CRITICAL, 0);
    assert_eq!(ext.payload().await, b"extension-payload");

    let doc = cursor.next_frame().await.unwrap().unwrap();
    assert_eq!(doc.kind, crate::REC_DOC);
    assert!(cursor.next_frame().await.unwrap().is_none());
}

#[cfg(feature = "deflate")]
#[semio_framework_async_macros::async_test]
async fn compressed_frame_round_trips_stored_and_raw_len() {
    let options = WriteOptions::default();
    let mut writer = SprWriter::begin(Vec::new(), &options).await.unwrap();
    let payload = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    writer.write_record(crate::REC_DOC, true, payload, crate::codec::ids::CodecId(1)).await.unwrap();
    let bytes = writer.into_sink().await;

    let mut cursor = FrameCursor::new(&bytes, HEADER_SIZE as u64).await;
    let frame = cursor.next_frame().await.unwrap().unwrap();
    assert_eq!(frame.flags & FRAME_FLAG_COMPRESSED, FRAME_FLAG_COMPRESSED);
    assert_eq!(frame.raw_len, Some(payload.len() as u64));
    assert!(frame.stored.len() < payload.len());

    let decompressed = crate::codec::DeflateCodec.decompress(frame.stored, frame.raw_len.unwrap(), 1024).unwrap();
    assert_eq!(decompressed, payload);
}
//#endregion 🔖️Frame

//#region 🔖️Commit
#[semio_framework_async_macros::async_test]
async fn commit_chain_verifies_by_recomputation_on_a_hand_built_file() {
    let bytes = build_small_file().await;

    // chain_0 = blake3(header)
    let chain_0 = *semio_framework_hash::hash(&bytes[..HEADER_SIZE]).as_bytes();

    // Recompute digest_i for every non-commit frame since the header, and the resulting chain_1.
    let mut cursor = FrameCursor::new(&bytes, HEADER_SIZE as u64).await;
    let mut concat = chain_0.to_vec();
    let mut commit_frame: Option<RecordFrame<'_>> = None;
    while let Some(frame) = cursor.next_frame().await.unwrap() {
        if frame.kind == crate::REC_COMMIT {
            commit_frame = Some(frame);
            break;
        }
        let full_frame_bytes = &bytes[frame.offset as usize..(frame.offset + frame.frame_len().await) as usize];
        concat.extend_from_slice(semio_framework_hash::hash(full_frame_bytes).as_bytes());
    }
    let expected_chain_1 = *semio_framework_hash::hash(&concat).as_bytes();

    let commit = parse_commit_payload(commit_frame.unwrap().stored).unwrap();
    assert_eq!(commit.commit_seq, 1);
    assert_eq!(commit.prev_commit_offset, 0);
    assert_eq!(commit.record_count, 2);
    assert_eq!(commit.chain_hash, expected_chain_1);
}

#[semio_framework_async_macros::async_test]
async fn commit_chain_links_prev_commit_offset_across_multiple_commits() {
    let options = WriteOptions::default();
    let mut writer = SprWriter::begin(Vec::new(), &options).await.unwrap();
    writer.write_record(crate::REC_DOC, true, b"a", crate::codec::ids::CodecId(0)).await.unwrap();
    let commit_1_offset = writer.commit().await.unwrap();
    writer.write_record(crate::REC_EDIT, true, b"b", crate::codec::ids::CodecId(0)).await.unwrap();
    let commit_2_offset = writer.commit().await.unwrap();
    let bytes = writer.into_sink().await;

    let mut reverse = ReverseFrameCursor::at_end(&bytes).await;
    let last = reverse.prev_frame().await.unwrap().unwrap();
    assert_eq!(last.offset, commit_2_offset);
    let commit_2 = parse_commit_payload(last.stored).unwrap();
    assert_eq!(commit_2.commit_seq, 2);
    assert_eq!(commit_2.prev_commit_offset, commit_1_offset);
}

#[semio_framework_async_macros::async_test]
async fn parse_commit_payload_is_public_for_downstream_frontier_summaries() {
    // Mirrors how `os_spr::history::FrontierSummary`/`protocol_materialize` are expected to
    // pull `last_commit_seq`/`chain_hash` out of a `REC_COMMIT` frame found via `FrameCursor`.
    let bytes = build_small_file().await;
    let mut cursor = FrameCursor::new(&bytes, HEADER_SIZE as u64).await;
    let mut commit_frame = None;
    loop {
        match cursor.next_frame().await.unwrap() {
            Some(frame) if frame.kind == crate::REC_COMMIT => {
                commit_frame = Some(frame);
                break;
            }
            Some(_) => continue,
            None => break,
        }
    }
    let commit_frame = commit_frame.unwrap();
    let payload: CommitPayload = parse_commit_payload(commit_frame.payload().await).unwrap();
    assert_eq!(payload.commit_seq, 1);
    assert_eq!(payload.record_count, 2);
}
//#endregion 🔖️Commit

//#region 🔖️Recovery
#[semio_framework_async_macros::async_test]
async fn recover_fast_path_trusts_a_cleanly_committed_file() {
    let bytes = build_small_file().await;
    let report = recover(&bytes, &ProtocolLimits::default(), RecoveryMode::LastCommit).await.unwrap();
    assert_eq!(report.bytes_recovered, bytes.len() as u64);
    assert_eq!(report.torn_tail_bytes, 0);
    assert_eq!(report.last_commit_seq, 1);
    assert_eq!(report.records_recovered, 3); // 2 records + 1 commit frame
}

#[semio_framework_async_macros::async_test]
async fn recover_fast_path_walks_multi_commit_chain_to_genesis() {
    let options = WriteOptions::default();
    let mut writer = SprWriter::begin(Vec::new(), &options).await.unwrap();
    for i in 0..5u8 {
        writer.write_record(crate::REC_EDIT, true, &[i], crate::codec::ids::CodecId(0)).await.unwrap();
        writer.commit().await.unwrap();
    }
    let bytes = writer.into_sink().await;
    let report = recover(&bytes, &ProtocolLimits::default(), RecoveryMode::LastCommit).await.unwrap();
    assert_eq!(report.last_commit_seq, 5);
    assert_eq!(report.bytes_recovered, bytes.len() as u64);
    assert_eq!(report.torn_tail_bytes, 0);
    assert_eq!(report.records_recovered, 10); // 5 records + 5 commits
}

#[semio_framework_async_macros::async_test]
async fn recover_forward_scan_truncates_to_last_commit_on_torn_tail() {
    let options = WriteOptions::default();
    let mut writer = SprWriter::begin(Vec::new(), &options).await.unwrap();
    writer.write_record(crate::REC_DOC, true, b"a", crate::codec::ids::CodecId(0)).await.unwrap();
    let commit_end = writer.commit().await.unwrap() + COMMIT_FRAME_LEN;
    writer.write_record(crate::REC_EDIT, true, b"uncommitted", crate::codec::ids::CodecId(0)).await.unwrap();
    let mut bytes = writer.into_sink().await;
    bytes.truncate(bytes.len() - 3); // tear the tail mid-record, after the commit

    let report = recover(&bytes, &ProtocolLimits::default(), RecoveryMode::LastCommit).await.unwrap();
    assert_eq!(report.bytes_recovered, commit_end);
    assert_eq!(report.torn_tail_bytes, bytes.len() as u64 - commit_end);
    assert_eq!(report.last_commit_seq, 1);
}

#[semio_framework_async_macros::async_test]
async fn recover_last_valid_record_mode_trusts_past_the_last_commit() {
    let options = WriteOptions::default();
    let mut writer = SprWriter::begin(Vec::new(), &options).await.unwrap();
    writer.write_record(crate::REC_DOC, true, b"a", crate::codec::ids::CodecId(0)).await.unwrap();
    writer.commit().await.unwrap();
    writer.write_record(crate::REC_EDIT, true, b"uncommitted-but-well-formed", crate::codec::ids::CodecId(0)).await.unwrap();
    let bytes = writer.into_sink().await;

    let last_commit_report = recover(&bytes, &ProtocolLimits::default(), RecoveryMode::LastCommit).await.unwrap();
    let last_valid_report = recover(&bytes, &ProtocolLimits::default(), RecoveryMode::LastValidRecord).await.unwrap();
    assert!(last_valid_report.bytes_recovered > last_commit_report.bytes_recovered);
    assert_eq!(last_valid_report.bytes_recovered, bytes.len() as u64);
}

#[semio_framework_async_macros::async_test]
async fn recover_header_only_file_reports_zero_records() {
    let writer = SprWriter::begin(Vec::new(), &WriteOptions::default()).await.unwrap();
    let bytes = writer.into_sink().await;
    let report = recover(&bytes, &ProtocolLimits::default(), RecoveryMode::LastCommit).await.unwrap();
    assert_eq!(report.records_recovered, 0);
    assert_eq!(report.bytes_recovered, HEADER_SIZE as u64);
    assert_eq!(report.last_commit_seq, 0);
}

#[semio_framework_async_macros::async_test]
async fn recover_rejects_files_shorter_than_the_header() {
    let bytes = vec![0u8; 10];
    assert!(recover(&bytes, &ProtocolLimits::default(), RecoveryMode::LastCommit).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn recover_never_panics_on_truncation_at_every_byte() {
    let options = WriteOptions::default();
    let mut writer = SprWriter::begin(Vec::new(), &options).await.unwrap();
    for i in 0..4u8 {
        writer.write_record(crate::REC_EDIT, true, &[i; 5], crate::codec::ids::CodecId(0)).await.unwrap();
    }
    writer.commit().await.unwrap();
    writer.write_record(crate::REC_EDIT, true, b"tail-record-after-commit", crate::codec::ids::CodecId(0)).await.unwrap();
    let bytes = writer.into_sink().await;

    for len in 0..=bytes.len() {
        let truncated: Vec<u8> = bytes[..len].to_vec();
        let result = recover(&truncated, &ProtocolLimits::default(), RecoveryMode::LastCommit).await;
        if let Ok(report) = result {
            assert!(report.bytes_recovered <= len as u64);
            assert!(report.torn_tail_bytes <= len as u64);
        }
    }
}
//#endregion 🔖️Recovery

//#region 🔖️Crypto
#[semio_framework_async_macros::async_test]
async fn blake3_hasher_matches_direct_blake3_call() {
    let hasher = Blake3Hasher;
    assert_eq!(hasher.hash(b"hello"), *semio_framework_hash::hash(b"hello").as_bytes());
}
//#endregion 🔖️Crypto

//#region 🔖️Writer
#[semio_framework_async_macros::async_test]
async fn begin_rejects_unknown_required_flags() {
    let options = WriteOptions { required_flags: 1 << 10, optional_flags: 0 };
    let result = SprWriter::begin(Vec::new(), &options).await;
    match result {
        Ok(_) => panic!("expected UnknownRequiredFlags error, got Ok"),
        Err(err) => assert!(matches!(err, ProtocolError::Pack(PackError::UnknownRequiredFlags(bits)) if bits == 1 << 10)),
    }
}

#[semio_framework_async_macros::async_test]
async fn position_tracks_sink_length_across_writes() {
    let mut writer = SprWriter::begin(Vec::new(), &WriteOptions::default()).await.unwrap();
    assert_eq!(writer.position().await, HEADER_SIZE as u64);
    writer.write_record(crate::REC_DOC, true, b"x", crate::codec::ids::CodecId(0)).await.unwrap();
    assert_eq!(writer.position().await, writer.into_sink().await.len() as u64);
}
//#endregion 🔖️Writer
