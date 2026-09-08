mod tests {
    use super::*;
    use crate::format::VerificationLevel;
    use crate::{ByteRange, CodecId, KIND_DOCUMENT, KIND_SCHEMA};

    /// @emoji 🎲️ Per-test unique scratch directory under `std::env::temp_dir()` — no external
    /// `tempfile` crate dependency, per the contract's std-only preference.
    static DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn scratch_dir(name: &str) -> std::path::PathBuf {
        let pid = std::process::id();
        let counter = DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("pack_io_test_{name}_{pid}_{counter}"));
        std::fs::create_dir_all(&dir).expect("create scratch dir");
        dir
    }

    //#region 🔖️File
    #[semio_framework_async_macros::async_test]
    async fn file_source_sink_write_then_read_back() {
        let dir = scratch_dir("file_rw");
        let path = dir.join("blob.bin");
        let payload = b"hello pack_io world, this is a test payload";

        let mut sink = FilePackSink::create(&path).unwrap();
        assert_eq!(sink.position().await, 0);
        sink.write_all(&payload[..10]).await.unwrap();
        assert_eq!(sink.position().await, 10);
        sink.write_all(&payload[10..]).await.unwrap();
        assert_eq!(sink.position().await, payload.len() as u64);
        sink.flush().await.unwrap();
        drop(sink);

        let source = FilePackSource::open(&path).unwrap();
        assert_eq!(PackSource::len(&source).await, payload.len() as u64);
        assert!(!source.is_empty().await);

        let mut buf = vec![0u8; payload.len()];
        let n = source.read_at(0, &mut buf).await.unwrap();
        assert_eq!(n, payload.len());
        assert_eq!(&buf, payload);

        let mut mid = [0u8; 5];
        let n = source.read_at(6, &mut mid).await.unwrap();
        assert_eq!(n, 5);
        assert_eq!(&mid, &payload[6..11]);

        let mut exact = vec![0u8; payload.len()];
        source.read_exact_at(0, &mut exact).await.unwrap();
        assert_eq!(exact, payload);

        let mut past_end = [0u8; 4];
        let result = source.read_at(1_000_000, &mut past_end).await;
        assert!(matches!(result, Err(PackError::Truncated(1_000_000))));

        let mut short = [0u8; 100];
        let n = source.read_at((payload.len() - 3) as u64, &mut short).await.unwrap();
        assert_eq!(n, 3);
        assert_eq!(&short[..3], &payload[payload.len() - 3..]);
    }
    //#endregion 🔖️File

    //#region 🔖️Atomic
    #[test]
    fn write_atomic_produces_full_content_and_no_stray_tmp_file() {
        let dir = scratch_dir("atomic_ok");
        let path = dir.join("doc.spk");
        let bytes = b"atomic write payload";

        write_atomic(&path, bytes).unwrap();

        let read_back = std::fs::read(&path).unwrap();
        assert_eq!(read_back, bytes);

        let leftover_tmp = std::fs::read_dir(&dir).unwrap().filter_map(|entry| entry.ok()).any(|entry| entry.file_name().to_string_lossy().contains(".tmp-"));
        assert!(!leftover_tmp, "no .tmp- file should remain after a successful write_atomic");
    }

    #[test]
    fn write_atomic_never_exposes_a_partial_target_from_a_simulated_interrupted_write() {
        let dir = scratch_dir("atomic_interrupt");
        let path = dir.join("doc.spk");
        let original = b"original committed content";

        write_atomic(&path, original).unwrap();

        // Simulate a crash between "temp file written" and "rename into place": create a
        // stray tmp file with the naming scheme write_atomic uses, but never rename it.
        let stray_tmp = path.with_file_name("doc.spk.tmp-999999-0");
        std::fs::write(&stray_tmp, b"garbage-from-a-simulated-crashed-write").unwrap();

        // The target must still show only the original, fully-committed content — the stray
        // (simulated-interrupted) tmp file is never visible at `path`.
        let read_back = std::fs::read(&path).unwrap();
        assert_eq!(read_back, original);

        // A second, real write_atomic call still succeeds and atomically replaces the content
        // even with an unrelated stray tmp file sitting in the same directory.
        let updated = b"second committed content, replaces the first";
        write_atomic(&path, updated).unwrap();
        let read_back = std::fs::read(&path).unwrap();
        assert_eq!(read_back, updated);
    }
    //#endregion 🔖️Atomic

    //#region 🔖️Stream
    /// @emoji 📏️ Wire length of a non-compressed segment frame (`kind, flags, seg_len varint,
    /// payload, crc32`) — mirrors `pack_format`'s private `encode_segment` for `CodecId(0)`,
    /// used here only to compute a valid `doc_span.len` for a hand-built `Manifest`.
    async fn uncompressed_segment_wire_len(payload_len: usize) -> u64 {
        let mut len_bytes = Vec::new();
        crate::write_varint_u64(&mut len_bytes, payload_len as u64);
        (1 + 1 + len_bytes.len() + payload_len + 4) as u64
    }

    fn no_compression_options() -> WriteOptions {
        WriteOptions { required_flags: 0, optional_flags: crate::format::OPTIONAL_STREAMED, codec: CodecId(0) }
    }

    #[semio_framework_async_macros::async_test]
    async fn streaming_writer_full_session_multiple_segments_and_a_chunk_round_trips() {
        let dir = scratch_dir("stream_session");
        let path = dir.join("session.spk");
        let doc_payload = b"the streamed document body";
        let chunk_payload = b"a chunk of blob bytes carried alongside the document";
        let schema_payload = b"an extra schema segment written after the chunk";

        let mut writer = StreamingPackWriter::create(&path, &no_compression_options()).await.unwrap();
        writer.write_segment(KIND_DOCUMENT, doc_payload).await.unwrap();
        let mut chunk = writer.begin_identity_chunk(chunk_payload.len()).await.unwrap();
        for fragment in chunk_payload.chunks(7) {
            chunk.write_fragment(fragment).await.unwrap();
        }
        let chunk_id = chunk.finish().await.unwrap();
        writer.write_segment(KIND_SCHEMA, schema_payload).await.unwrap();

        let manifest = Manifest {
            schema_name: String::new(),
            schema_hash: [0u8; 32],
            doc_span: ByteRange { offset: crate::format::HEADER_SIZE as u64, len: uncompressed_segment_wire_len(doc_payload.len()).await },
            doc_frame_count: 1,
            symbols_span: ByteRange { offset: 0, len: 0 },
            chunk_table_span: ByteRange { offset: 0, len: 0 },
            field_index_span: ByteRange { offset: 0, len: 0 },
            uncompressed_body_len: doc_payload.len() as u64,
            field_count: 0,
            chunk_count: 0,
            symbol_count: 0,
        };
        writer.finish(&manifest).await.unwrap();

        let source = FilePackSource::open(&path).unwrap();
        let limits = PackLimits::default();
        let pack_file = crate::format::PackFile::open_manifest(source, &limits, VerificationLevel::Standard).await.unwrap();

        let loaded_manifest = pack_file.manifest().unwrap();
        assert_eq!(loaded_manifest.doc_frame_count, 1);
        assert_eq!(loaded_manifest.uncompressed_body_len, doc_payload.len() as u64);
        assert_eq!(pack_file.chunk_count(), 1);

        let read_chunk = pack_file.read_chunk(chunk_id, VerificationLevel::Full).await.unwrap();
        assert_eq!(read_chunk, chunk_payload);

        // `Full` verification also cross-checks the concatenated document body against the
        // footer's blake3 content hash, proving StreamingPackWriter's running hash matches.
        let body = pack_file.body_bytes(VerificationLevel::Full).await.unwrap();
        assert_eq!(body, doc_payload);

        // Forward-scan recovery should independently see every segment this session wrote:
        // document, chunk, schema, chunk table, manifest, end.
        let report = recover_file(&path, &limits).await.unwrap();
        assert_eq!(report.segments_recovered, 6);
        assert!(report.manifest.is_some());
    }
    //#endregion 🔖️Stream

    //#region 🔖️Recover
    async fn build_valid_session_file(dir: &Path, name: &str) -> std::path::PathBuf {
        let path = dir.join(name);
        let doc_payload = b"recoverable document body";
        let chunk_payload = b"recoverable chunk payload";

        let mut writer = StreamingPackWriter::create(&path, &no_compression_options()).await.unwrap();
        writer.write_segment(KIND_DOCUMENT, doc_payload).await.unwrap();
        let mut chunk = writer.begin_identity_chunk(chunk_payload.len()).await.unwrap();
        for fragment in chunk_payload.chunks(5) {
            chunk.write_fragment(fragment).await.unwrap();
        }
        chunk.finish().await.unwrap();

        let manifest = Manifest {
            schema_name: String::new(),
            schema_hash: [0u8; 32],
            doc_span: ByteRange { offset: crate::format::HEADER_SIZE as u64, len: uncompressed_segment_wire_len(doc_payload.len()).await },
            doc_frame_count: 1,
            symbols_span: ByteRange { offset: 0, len: 0 },
            chunk_table_span: ByteRange { offset: 0, len: 0 },
            field_index_span: ByteRange { offset: 0, len: 0 },
            uncompressed_body_len: doc_payload.len() as u64,
            field_count: 0,
            chunk_count: 0,
            symbol_count: 0,
        };
        writer.finish(&manifest).await.unwrap();
        path
    }

    #[semio_framework_async_macros::async_test]
    async fn recover_file_with_footer_stripped_still_recovers_every_body_segment() {
        let dir = scratch_dir("recover_no_footer");
        let path = build_valid_session_file(&dir, "truncated_footer.spk").await;
        let limits = PackLimits::default();

        let full_len = std::fs::metadata(&path).unwrap().len();
        let file = std::fs::OpenOptions::new().write(true).open(&path).unwrap();
        file.set_len(full_len - crate::format::FOOTER_SIZE as u64).unwrap();
        drop(file);

        // Superblock open must fail (no valid footer) — this is the scenario recover_file is for.
        let source = FilePackSource::open(&path).unwrap();
        assert!(crate::format::PackFile::open_superblock(source, &limits).await.is_err());

        let report = recover_file(&path, &limits).await.unwrap();
        // document, chunk, chunk_table (since one chunk was written), manifest, end.
        assert_eq!(report.segments_recovered, 5);
        assert!(report.bytes_recovered > 0);
        assert!(report.manifest.is_some());
    }

    #[semio_framework_async_macros::async_test]
    async fn recover_file_truncated_mid_segment_recovers_a_strict_prefix_without_panicking() {
        let dir = scratch_dir("recover_mid_segment");
        let path = build_valid_session_file(&dir, "truncated_mid.spk").await;
        let limits = PackLimits::default();

        let full_len = std::fs::metadata(&path).unwrap().len();
        // Cut well past the footer AND into the tail of the body stream, guaranteeing the
        // last segment present (the `End` marker) is itself truncated mid-frame.
        let cut_len = full_len - crate::format::FOOTER_SIZE as u64 - 3;
        let file = std::fs::OpenOptions::new().write(true).open(&path).unwrap();
        file.set_len(cut_len).unwrap();
        drop(file);

        let report = recover_file(&path, &limits).await.unwrap();
        // Fewer than the full 5 segments (document, chunk, chunk_table, manifest, end) since
        // the trailing bytes of the last segment are gone; recovery must stop cleanly there,
        // never panic, and never report more than what a full session would produce.
        assert!(report.segments_recovered < 5);
        assert!(report.segments_recovered >= 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn recover_file_on_a_file_too_short_for_a_header_errors_never_panics() {
        let dir = scratch_dir("recover_too_short");
        let path = dir.join("empty.spk");
        std::fs::write(&path, b"short").unwrap();
        let limits = PackLimits::default();
        let result = recover_file(&path, &limits).await;
        assert!(matches!(result, Err(PackError::Truncated(_))));
    }
    //#endregion 🔖️Recover
}
