
use super::*;

//#region 🔖️Header
#[semio_framework_async_macros::async_test]
async fn header_hand_built_bytes_parse_round_trip() {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&MAGIC);
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&0u16.to_le_bytes());
    bytes.extend_from_slice(&REQUIRED_COMPRESSED.to_le_bytes());
    bytes.extend_from_slice(&OPTIONAL_CANONICAL.to_le_bytes());
    let crc = crc32c(&bytes[0..20]);
    bytes.extend_from_slice(&crc.to_le_bytes());
    bytes.extend_from_slice(&[0u8; 8]);
    assert_eq!(bytes.len(), HEADER_SIZE);

    let header = Header::parse(&bytes).await.unwrap();
    assert_eq!(header.version_major, 1);
    assert_eq!(header.version_minor, 0);
    assert_eq!(header.required_flags, REQUIRED_COMPRESSED);
    assert_eq!(header.optional_flags, OPTIONAL_CANONICAL);
    assert_eq!(header.write_bytes().await[0..24], bytes[0..24]);
}

#[semio_framework_async_macros::async_test]
async fn header_parse_rejects_bad_magic() {
    let bytes = [0u8; HEADER_SIZE];
    assert_eq!(Header::parse(&bytes).await, Err(PackError::BadMagic));
}

#[semio_framework_async_macros::async_test]
async fn header_parse_rejects_bad_crc() {
    let header = Header { version_major: 1, version_minor: 0, required_flags: 0, optional_flags: 0 };
    let mut bytes = header.write_bytes().await;
    bytes[20] ^= 0xFF;
    assert!(matches!(Header::parse(&bytes).await, Err(PackError::ChecksumMismatch { segment: "header", .. })));
}

#[semio_framework_async_macros::async_test]
async fn header_parse_rejects_unknown_required_flags() {
    let header = Header { version_major: 1, version_minor: 0, required_flags: 1 << 4, optional_flags: 0 };
    let bytes = header.write_bytes().await;
    assert_eq!(Header::parse(&bytes).await, Err(PackError::UnknownRequiredFlags(1 << 4)));
}

#[semio_framework_async_macros::async_test]
async fn header_parse_rejects_unsupported_version() {
    let header = Header { version_major: 2, version_minor: 0, required_flags: 0, optional_flags: 0 };
    let bytes = header.write_bytes().await;
    assert_eq!(Header::parse(&bytes).await, Err(PackError::UnsupportedVersion { major: 2, minor: 0 }));
}

#[semio_framework_async_macros::async_test]
async fn header_truncated_at_every_byte_boundary_errors_never_panics() {
    let header = Header { version_major: 1, version_minor: 0, required_flags: REQUIRED_COMPRESSED, optional_flags: 0 };
    let full = header.write_bytes().await;
    for len in 0..HEADER_SIZE {
        let slice = &full[..len];
        assert!(Header::parse(slice).await.is_err(), "expected error at header truncation length {len}");
        let limits = PackLimits::default();
        assert!(PackFile::open_superblock(slice, &limits).await.is_err(), "expected error opening superblock at length {len}");
    }
}
//#endregion 🔖️Header

//#region 🔖️Footer
#[semio_framework_async_macros::async_test]
async fn footer_hand_built_bytes_parse_round_trip() {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&FOOTER_MAGIC);
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&0u16.to_le_bytes());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&32u64.to_le_bytes());
    bytes.extend_from_slice(&100u64.to_le_bytes());
    bytes.extend_from_slice(&500u64.to_le_bytes());
    let hash = [7u8; 32];
    bytes.extend_from_slice(&hash);
    bytes.extend_from_slice(&0u64.to_le_bytes());
    assert_eq!(bytes.len(), FOOTER_SIZE - 4);
    let crc = crc32c(&bytes);
    bytes.extend_from_slice(&crc.to_le_bytes());
    assert_eq!(bytes.len(), FOOTER_SIZE);

    let footer = Footer::parse(&bytes).await.unwrap();
    assert_eq!(footer.version_major, 1);
    assert_eq!(footer.manifest_offset, 32);
    assert_eq!(footer.manifest_len, 100);
    assert_eq!(footer.file_len, 500);
    assert_eq!(footer.content_hash.0, hash);
    assert_eq!(footer.prev_footer_offset, 0);
    assert_eq!(footer.write_bytes().await, bytes);
}

#[semio_framework_async_macros::async_test]
async fn footer_parse_rejects_bad_magic() {
    let bytes = [0u8; FOOTER_SIZE];
    assert_eq!(Footer::parse(&bytes).await, Err(PackError::BadMagic));
}

#[semio_framework_async_macros::async_test]
async fn footer_parse_rejects_bad_crc() {
    let footer = Footer { version_major: 1, version_minor: 0, required_flags: 0, manifest_offset: 32, manifest_len: 10, file_len: 200, content_hash: ContentHash([1u8; 32]), prev_footer_offset: 0 };
    let mut bytes = footer.write_bytes().await;
    let last = bytes.len() - 1;
    bytes[last] ^= 0xFF;
    assert!(matches!(Footer::parse(&bytes).await, Err(PackError::ChecksumMismatch { segment: "footer", .. })));
}

#[semio_framework_async_macros::async_test]
async fn footer_truncated_at_every_byte_boundary_errors_never_panics() {
    let footer = Footer { version_major: 1, version_minor: 0, required_flags: 0, manifest_offset: 32, manifest_len: 10, file_len: 200, content_hash: ContentHash([2u8; 32]), prev_footer_offset: 0 };
    let full = footer.write_bytes().await;
    for len in 0..FOOTER_SIZE {
        let slice = &full[..len];
        assert!(Footer::parse(slice).await.is_err(), "expected error at footer truncation length {len}");
    }
}

#[semio_framework_async_macros::async_test]
async fn open_superblock_truncated_at_every_footer_byte_boundary_errors_never_panics() {
    let options = WriteOptions { required_flags: 0, optional_flags: 0, codec: CodecId(0) };
    let mut writer = PackWriter::begin(Vec::<u8>::new(), &options).await.unwrap();
    writer.write_segment(crate::KIND_DOCUMENT, b"hello world").await.unwrap();
    let manifest = Manifest {
        schema_name: String::new(),
        schema_hash: [0u8; 32],
        doc_span: ByteRange { offset: HEADER_SIZE as u64, len: 0 },
        doc_frame_count: 0,
        symbols_span: ByteRange { offset: 0, len: 0 },
        chunk_table_span: ByteRange { offset: 0, len: 0 },
        field_index_span: ByteRange { offset: 0, len: 0 },
        uncompressed_body_len: 11,
        field_count: 0,
        chunk_count: 0,
        symbol_count: 0,
    };
    let full = writer.finish(&manifest).await.unwrap();
    let limits = PackLimits::default();
    let start = full.len() - FOOTER_SIZE;
    for len in start..full.len() {
        let slice = &full[..len];
        assert!(PackFile::open_superblock(slice, &limits).await.is_err(), "expected error at file truncation length {len}");
    }
    assert!(PackFile::open_superblock(full.as_slice(), &limits).await.is_ok());
}
//#endregion 🔖️Footer

//#region 🔖️Segment
#[semio_framework_async_macros::async_test]
async fn segment_skip_unknown_kind_decodes_without_error() {
    let unknown_kind = 0x50u8;
    let encoded = encode_segment(unknown_kind, CodecId(0), b"extension payload").await.unwrap();
    let mut file = vec![0u8; HEADER_SIZE];
    file.extend_from_slice(&encoded.bytes);
    let limits = PackLimits::default();
    let decoded = decode_segment_at(&file, HEADER_SIZE as u64, &limits, true).await.unwrap();
    assert_eq!(decoded.kind, unknown_kind);
    assert_eq!(decoded.payload, b"extension payload");
}

#[semio_framework_async_macros::async_test]
async fn recover_accumulates_unknown_kind_segments_without_erroring() {
    let header = Header { version_major: 1, version_minor: 0, required_flags: 0, optional_flags: 0 };
    let mut file = header.write_bytes().await.to_vec();
    file.extend_from_slice(&encode_segment(0x60, CodecId(0), b"future extension").await.unwrap().bytes);
    file.extend_from_slice(&encode_segment(crate::KIND_END, CodecId(0), &[]).await.unwrap().bytes);
    let limits = PackLimits::default();
    let report = recover(&file, &limits).await.unwrap();
    assert_eq!(report.segments_recovered, 2);
    assert!(report.manifest.is_none());
}

#[semio_framework_async_macros::async_test]
async fn segment_crc_mismatch_is_detected() {
    let encoded = encode_segment(crate::KIND_DOCUMENT, CodecId(0), b"payload").await.unwrap();
    let mut file = vec![0u8; HEADER_SIZE];
    file.extend_from_slice(&encoded.bytes);
    let last = file.len() - 1;
    file[last] ^= 0xFF;
    let limits = PackLimits::default();
    let result = decode_segment_at(&file, HEADER_SIZE as u64, &limits, true).await;
    assert!(matches!(result, Err(PackError::ChecksumMismatch { segment: "segment", .. })));
}

#[semio_framework_async_macros::async_test]
async fn segment_crc_mismatch_ignored_at_trusted_level() {
    let encoded = encode_segment(crate::KIND_DOCUMENT, CodecId(0), b"payload").await.unwrap();
    let mut file = vec![0u8; HEADER_SIZE];
    file.extend_from_slice(&encoded.bytes);
    let last = file.len() - 1;
    file[last] ^= 0xFF;
    let limits = PackLimits::default();
    let result = decode_segment_at(&file, HEADER_SIZE as u64, &limits, false).await;
    assert!(result.is_ok());
}
//#endregion 🔖️Segment

//#region 🔖️Writer
async fn build_sample_pack(codec: CodecId) -> (Vec<u8>, ChunkId, Vec<u8>) {
    let options = WriteOptions { required_flags: 0, optional_flags: OPTIONAL_CANONICAL, codec };
    let mut writer = PackWriter::begin(Vec::<u8>::new(), &options).await.unwrap();

    writer.write_segment(crate::KIND_SYMBOLS, &encode_symbols(&["widget.v1".to_string(), "name".to_string()]).await).await.unwrap();

    let doc_offset = writer.position().await;
    let document_payload = b"the quick brown fox jumps over the lazy dog, repeatedly, for compressibility".to_vec();
    writer.write_segment(crate::KIND_DOCUMENT, &document_payload).await.unwrap();
    let doc_len = writer.position().await - doc_offset;

    let chunk_payload = vec![42u8; 4096];
    let chunk_id = writer.write_chunk(&chunk_payload).await.unwrap();

    let manifest = Manifest {
        schema_name: "widget.v1".to_string(),
        schema_hash: [9u8; 32],
        doc_span: ByteRange { offset: doc_offset, len: doc_len },
        doc_frame_count: 1,
        symbols_span: ByteRange { offset: 0, len: 0 },
        chunk_table_span: ByteRange { offset: 0, len: 0 },
        field_index_span: ByteRange { offset: 0, len: 0 },
        uncompressed_body_len: document_payload.len() as u64,
        field_count: 1,
        chunk_count: 0,
        symbol_count: 0,
    };
    let bytes = writer.finish(&manifest).await.unwrap();
    (bytes, chunk_id, chunk_payload)
}

#[semio_framework_async_macros::async_test]
async fn retained_identity_chunk_fragment_parity_exact_boundary_and_interrupted_finish() {
    let options = WriteOptions { required_flags: REQUIRED_CHUNKED, optional_flags: 0, codec: CodecId(0) };
    let payload = vec![0xA5; 16_385];
    let manifest = Manifest {
        schema_name: String::new(),
        schema_hash: [0; 32],
        doc_span: ByteRange { offset: 0, len: 0 },
        doc_frame_count: 0,
        symbols_span: ByteRange { offset: 0, len: 0 },
        chunk_table_span: ByteRange { offset: 0, len: 0 },
        field_index_span: ByteRange { offset: 0, len: 0 },
        uncompressed_body_len: 0,
        field_count: 0,
        chunk_count: 0,
        symbol_count: 0,
    };
    let mut oracle = PackWriter::begin(Vec::new(), &options).await.unwrap();
    oracle.write_chunk(&payload).await.unwrap();
    let oracle = oracle.finish(&manifest).await.unwrap();

    let mut retained = PackWriter::begin(Vec::new(), &options).await.unwrap();
    let mut chunk = retained.begin_identity_chunk(payload.len()).await.unwrap();
    chunk.write_fragment(&payload[..1]).await.unwrap();
    chunk.write_fragment(&payload[1..16_384]).await.unwrap();
    chunk.write_fragment(&payload[16_384..]).await.unwrap();
    chunk.finish().await.unwrap();
    let retained = retained.finish(&manifest).await.unwrap();
    assert_eq!(retained, oracle);

    let mut interrupted = PackWriter::begin(Vec::new(), &options).await.unwrap();
    let mut chunk = interrupted.begin_identity_chunk(2).await.unwrap();
    chunk.write_fragment(&payload[..1]).await.unwrap();
    assert!(matches!(chunk.finish().await, Err(PackError::LimitExceeded(_))));

    let mut maximum = PackWriter::begin(Vec::new(), &options).await.unwrap();
    let mut chunk = maximum.begin_identity_chunk(1).await.unwrap();
    assert!(matches!(chunk.write_fragment(&payload[..2]).await, Err(PackError::LimitExceeded(_))));
    chunk.close();
    assert_eq!(payload[0], 0xA5);

    let mut closed = PackWriter::begin(Vec::new(), &options).await.unwrap();
    let chunk = closed.begin_identity_chunk(1).await.unwrap();
    chunk.close();
    assert!(closed.chunks.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn write_then_read_round_trip_uncompressed() {
    let (bytes, chunk_id, chunk_payload) = build_sample_pack(CodecId(0)).await;
    let limits = PackLimits::default();
    let file = PackFile::open_manifest(bytes.as_slice(), &limits, VerificationLevel::Full).await.unwrap();

    let manifest = file.manifest().unwrap();
    assert_eq!(manifest.schema_name, "widget.v1");
    assert_eq!(manifest.chunk_count, 1);
    assert_eq!(manifest.symbol_count, 2);

    assert_eq!(file.chunk_count(), 1);
    let read_back = file.read_chunk(chunk_id, VerificationLevel::Full).await.unwrap();
    assert_eq!(read_back, chunk_payload);

    let body = file.body_bytes(VerificationLevel::Full).await.unwrap();
    assert_eq!(body, b"the quick brown fox jumps over the lazy dog, repeatedly, for compressibility");

    assert_eq!(file.content_hash(), file.superblock().footer.content_hash);
}

#[semio_framework_async_macros::async_test]
async fn identity_chunk_cursor_retains_fragment_progress_and_terminal_verification() {
    let (bytes, chunk_id, expected) = build_sample_pack(CodecId(0)).await;
    let file = PackFile::open_manifest(bytes.as_slice(), &PackLimits::default(), VerificationLevel::Full).await.unwrap();
    let mut cursor = file.identity_chunk_cursor(chunk_id, VerificationLevel::Full).unwrap();
    let mut fragment = [0u8; 3];
    let mut actual = Vec::new();
    loop {
        let read = cursor.read_fragment(&mut fragment).await.unwrap();
        if read == 0 {
            break;
        }
        actual.extend_from_slice(&fragment[..read]);
    }
    assert_eq!(actual, expected);
    assert!(cursor.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn write_then_read_round_trip_with_compressed_segment_and_chunk() {
    let (bytes, chunk_id, chunk_payload) = build_sample_pack(CodecId(1)).await;
    let limits = PackLimits::default();
    assert_eq!(bytes[0..8], MAGIC);

    let superblock_only = PackFile::open_superblock(bytes.as_slice(), &limits).await.unwrap();
    assert_eq!(superblock_only.superblock().header.required_flags & REQUIRED_COMPRESSED, REQUIRED_COMPRESSED);

    let file = PackFile::open_manifest(bytes.as_slice(), &limits, VerificationLevel::Full).await.unwrap();
    let manifest = file.manifest().unwrap();
    assert_eq!(manifest.schema_name, "widget.v1");

    let read_back = file.read_chunk(chunk_id, VerificationLevel::Full).await.unwrap();
    assert_eq!(read_back, chunk_payload);

    let body = file.body_bytes(VerificationLevel::Full).await.unwrap();
    assert_eq!(body, b"the quick brown fox jumps over the lazy dog, repeatedly, for compressibility");
}

#[semio_framework_async_macros::async_test]
async fn read_footer_only_matches_full_open() {
    let (bytes, _chunk_id, _chunk_payload) = build_sample_pack(CodecId(0)).await;
    let footer = read_footer_only(&bytes).await.unwrap();
    let limits = PackLimits::default();
    let file = PackFile::open_superblock(bytes.as_slice(), &limits).await.unwrap();
    assert_eq!(footer, file.superblock().footer);
}

#[semio_framework_async_macros::async_test]
async fn empty_document_and_no_chunks_round_trips() {
    let options = WriteOptions { required_flags: 0, optional_flags: 0, codec: CodecId(0) };
    let writer = PackWriter::begin(Vec::<u8>::new(), &options).await.unwrap();
    let manifest = Manifest {
        schema_name: String::new(),
        schema_hash: [0u8; 32],
        doc_span: ByteRange { offset: 0, len: 0 },
        doc_frame_count: 0,
        symbols_span: ByteRange { offset: 0, len: 0 },
        chunk_table_span: ByteRange { offset: 0, len: 0 },
        field_index_span: ByteRange { offset: 0, len: 0 },
        uncompressed_body_len: 0,
        field_count: 0,
        chunk_count: 0,
        symbol_count: 0,
    };
    let bytes = writer.finish(&manifest).await.unwrap();
    let limits = PackLimits::default();
    let file = PackFile::open_manifest(bytes.as_slice(), &limits, VerificationLevel::Full).await.unwrap();
    assert_eq!(file.chunk_count(), 0);
    assert_eq!(file.body_bytes(VerificationLevel::Full).await.unwrap(), Vec::<u8>::new());
    let expected_empty_hash = ContentHash(*semio_framework_hash::hash(b"").as_bytes());
    assert_eq!(file.content_hash(), expected_empty_hash);
}

#[semio_framework_async_macros::async_test]
async fn finish_errors_when_schema_name_not_in_symbols() {
    let options = WriteOptions { required_flags: 0, optional_flags: 0, codec: CodecId(0) };
    let writer = PackWriter::begin(Vec::<u8>::new(), &options).await.unwrap();
    let manifest = Manifest {
        schema_name: "missing".to_string(),
        schema_hash: [0u8; 32],
        doc_span: ByteRange { offset: 0, len: 0 },
        doc_frame_count: 0,
        symbols_span: ByteRange { offset: 0, len: 0 },
        chunk_table_span: ByteRange { offset: 0, len: 0 },
        field_index_span: ByteRange { offset: 0, len: 0 },
        uncompressed_body_len: 0,
        field_count: 0,
        chunk_count: 0,
        symbol_count: 0,
    };
    let result = writer.finish(&manifest).await;
    assert!(matches!(result, Err(PackError::Schema(_))));
}

#[semio_framework_async_macros::async_test]
async fn begin_rejects_unknown_required_flags() {
    let options = WriteOptions { required_flags: 1 << 5, optional_flags: 0, codec: CodecId(0) };
    let result = PackWriter::begin(Vec::<u8>::new(), &options).await;
    assert_eq!(result.err(), Some(PackError::UnknownRequiredFlags(1 << 5)));
}
//#endregion 🔖️Writer

//#region 🔖️Corruption
#[semio_framework_async_macros::async_test]
async fn open_manifest_rejects_flipped_chunk_payload_crc_at_standard_level() {
    let (bytes, chunk_id, _chunk_payload) = build_sample_pack(CodecId(0)).await;
    let limits = PackLimits::default();
    let mut corrupted = bytes.clone();
    let file = PackFile::open_manifest(bytes.as_slice(), &limits, VerificationLevel::Standard).await.unwrap();
    let range = file.chunk_range(chunk_id).unwrap();
    corrupted[range.offset as usize] ^= 0xFF;
    let corrupted_file = PackFile::open_manifest(corrupted.as_slice(), &limits, VerificationLevel::Standard).await.unwrap();
    let result = corrupted_file.read_chunk(chunk_id, VerificationLevel::Standard).await;
    assert!(matches!(result, Err(PackError::ChecksumMismatch { segment: "chunk", .. })));
}

#[semio_framework_async_macros::async_test]
async fn open_manifest_rejects_wrong_kind_at_manifest_offset() {
    let (bytes, _chunk_id, _chunk_payload) = build_sample_pack(CodecId(0)).await;
    let limits = PackLimits::default();
    let mut footer_bytes = bytes[bytes.len() - FOOTER_SIZE..].to_vec();
    let mut footer = Footer::parse(&footer_bytes).await.unwrap();
    footer.manifest_offset = HEADER_SIZE as u64; // points at the symbols segment instead
    footer_bytes = footer.write_bytes().await;
    let mut corrupted = bytes[..bytes.len() - FOOTER_SIZE].to_vec();
    corrupted.extend_from_slice(&footer_bytes);
    let result = PackFile::open_manifest(corrupted.as_slice(), &limits, VerificationLevel::Standard).await;
    assert!(matches!(result, Err(PackError::Malformed { what: "manifest", .. })));
}
//#endregion 🔖️Corruption
