//! 📦️ `pack_io` — native file I/O for the `pack` binary document container family:
//! `FilePackSource`/`FilePackSink` implementing `pack_core`'s `PackSource`/`PackSink` traits over
//! `std::fs::File`, `write_atomic` (temp-file + fsync + rename, no partial file ever visible),
//! `StreamingPackWriter` (an incremental `pack_format::PackWriter<FilePackSink>` that flushes
//! segments to disk as produced rather than buffering the whole file in memory), and
//! `recover_file` (opens a file and forward-scans it via `pack_format::recover`).
//!
//! Everything here is native-only (`std::fs`, `std::sync::Mutex`) and gated behind
//! `#[cfg(not(target_arch = "wasm32"))]` so the crate still compiles — as an effectively-empty
//! crate — for a `wasm32-unknown-unknown` target check. See the `## pack_io` section of the
//! wave-0 contract at `.🦑️repo/🎫️tickets/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/contract.md`.

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use std::io::Write;
    use std::path::Path;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicU64, Ordering};

    use pack_core::{ChunkId, PackError, PackLimits, PackSink, PackSource};
    use pack_format::{Manifest, PackWriter, RecoveryReport, WriteOptions};

    /// @emoji 🚨️ Wraps a `std::io::Error` into the crate-wide `PackError::Io` variant — the only
    /// place `std::io::Error` is allowed to appear, per the contract's no-`std::io::Error`-in-
    /// public-signatures rule.
    #[allow(clippy::needless_pass_by_value)] // used as a `map_err` callback, which passes the error by value
    fn io_err(err: std::io::Error) -> PackError {
        PackError::Io(err.to_string())
    }

    //#region 🔖️File
    /// @emoji 📂️ A read-only, random-access file source. Positional reads go through
    /// `std::os::unix::fs::FileExt::read_at`/`std::os::windows::fs::FileExt::seek_read` (both
    /// take `&self`, no locking needed) on unix/windows, and a `Mutex`-guarded seek+read fallback
    /// on any other native target — kept behind one `Mutex<File>` field for a single code path.
    pub struct FilePackSource {
        file: Mutex<std::fs::File>,
        len: u64,
    }

    impl FilePackSource {
        /// @emoji 📖️ Opens `path` for reading and stat's its length up front.
        pub fn open(path: &Path) -> Result<Self, PackError> {
            let file = std::fs::File::open(path).map_err(io_err)?;
            let len = file.metadata().map_err(io_err)?.len();
            Ok(Self { file: Mutex::new(file), len })
        }
    }

    impl PackSource for FilePackSource {
        fn len(&self) -> u64 {
            self.len
        }

        fn read_at(&self, offset: u64, buf: &mut [u8]) -> Result<usize, PackError> {
            if offset > self.len {
                return Err(PackError::Truncated(offset));
            }
            let guard = self.file.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            #[cfg(unix)]
            {
                use std::os::unix::fs::FileExt;
                guard.read_at(buf, offset).map_err(io_err)
            }
            #[cfg(windows)]
            {
                use std::os::windows::fs::FileExt;
                guard.seek_read(buf, offset).map_err(io_err)
            }
            #[cfg(not(any(unix, windows)))]
            {
                use std::io::{Read, Seek, SeekFrom};
                let mut guard = guard;
                guard.seek(SeekFrom::Start(offset)).map_err(io_err)?;
                guard.read(buf).map_err(io_err)
            }
        }
    }

    /// @emoji 📤️ A write-only file sink opened truncate-on-create; tracks its own write position
    /// since `std::fs::File` exposes none without a `&mut self` seek.
    pub struct FilePackSink {
        file: std::fs::File,
        position: u64,
    }

    impl FilePackSink {
        /// @emoji 🆕️ Creates (truncating any existing file) `path` for writing.
        pub fn create(path: &Path) -> Result<Self, PackError> {
            let file = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(path).map_err(io_err)?;
            Ok(Self { file, position: 0 })
        }
    }

    impl PackSink for FilePackSink {
        fn write_all(&mut self, bytes: &[u8]) -> Result<(), PackError> {
            self.file.write_all(bytes).map_err(io_err)?;
            self.position += bytes.len() as u64;
            Ok(())
        }

        fn position(&self) -> u64 {
            self.position
        }

        fn flush(&mut self) -> Result<(), PackError> {
            self.file.flush().map_err(io_err)?;
            self.file.sync_all().map_err(io_err)
        }
    }
    //#endregion 🔖️File

    //#region 🔖️Atomic
    /// @emoji 🔢️ Per-process monotonic counter mixed into temp-file names so concurrent
    /// `write_atomic` calls (even to the same `path`, even on the same PID) never collide.
    static TMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    /// @emoji 🛟️ Writes `bytes` to `path` atomically: writes to a sibling temp file, `fsync`s it,
    /// then `rename`s it into place. `rename` is atomic on every platform this targets, so a
    /// reader can never observe a partially-written `path` — it sees either the old content or
    /// the fully-written new content, never a torn write.
    pub fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), PackError> {
        let file_name = path.file_name().ok_or_else(|| PackError::Io("write_atomic: path has no file name".to_string()))?;
        let pid = std::process::id();
        let counter = TMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let tmp_name = format!("{}.tmp-{pid}-{counter}", file_name.to_string_lossy());
        let tmp_path = path.with_file_name(tmp_name);
        {
            let mut tmp_file = std::fs::File::create(&tmp_path).map_err(io_err)?;
            tmp_file.write_all(bytes).map_err(io_err)?;
            tmp_file.sync_all().map_err(io_err)?;
        }
        std::fs::rename(&tmp_path, path).map_err(io_err)?;
        Ok(())
    }
    //#endregion 🔖️Atomic

    //#region 🔖️Stream
    /// @emoji 🌊️ An incremental pack file writer: segments/chunks are framed and flushed straight
    /// to disk as they're written rather than buffered in memory for the whole file, wrapping
    /// `pack_format::PackWriter<FilePackSink>`.
    pub struct StreamingPackWriter {
        inner: PackWriter<FilePackSink>,
    }

    impl StreamingPackWriter {
        /// @emoji 🚀️ Creates `path` and writes the 32-byte header.
        pub fn create(path: &Path, options: &WriteOptions) -> Result<Self, PackError> {
            let sink = FilePackSink::create(path)?;
            let inner = PackWriter::begin(sink, options)?;
            Ok(Self { inner })
        }

        /// @emoji 🖇️ Frames, compresses, CRCs, and flushes one segment to disk.
        pub fn write_segment(&mut self, kind: u8, payload: &[u8]) -> Result<(), PackError> {
            self.inner.write_segment(kind, payload)
        }

        /// @emoji 🧱️ Writes and flushes one chunk segment, returning its `ChunkId`.
        pub fn write_chunk(&mut self, payload: &[u8]) -> Result<ChunkId, PackError> {
            self.inner.write_chunk(payload)
        }

        /// @emoji 🏁️ Writes the chunk table, manifest, end marker, and footer, `fsync`s (via
        /// `FilePackSink::flush`, called internally by `PackWriter::finish`), and closes the file.
        pub fn finish(self, manifest: &Manifest) -> Result<(), PackError> {
            self.inner.finish(manifest)?;
            Ok(())
        }
    }
    //#endregion 🔖️Stream

    //#region 🔖️Recover
    /// @emoji 🩺️ Opens `path` and forward-scans it via `pack_format::recover` — for use when a
    /// file's footer fails to parse/validate and the caller wants to salvage whatever valid
    /// segments precede the corruption.
    pub fn recover_file(path: &Path, limits: &PackLimits) -> Result<RecoveryReport, PackError> {
        let source = FilePackSource::open(path)?;
        pack_format::recover(&source, limits)
    }
    //#endregion 🔖️Recover

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use pack_core::{ByteRange, CodecId, KIND_DOCUMENT, KIND_SCHEMA};
        use pack_format::VerificationLevel;

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
        #[test]
        fn file_source_sink_write_then_read_back() {
            let dir = scratch_dir("file_rw");
            let path = dir.join("blob.bin");
            let payload = b"hello pack_io world, this is a test payload";

            let mut sink = FilePackSink::create(&path).unwrap();
            assert_eq!(sink.position(), 0);
            sink.write_all(&payload[..10]).unwrap();
            assert_eq!(sink.position(), 10);
            sink.write_all(&payload[10..]).unwrap();
            assert_eq!(sink.position(), payload.len() as u64);
            sink.flush().unwrap();
            drop(sink);

            let source = FilePackSource::open(&path).unwrap();
            assert_eq!(PackSource::len(&source), payload.len() as u64);
            assert!(!source.is_empty());

            let mut buf = vec![0u8; payload.len()];
            let n = source.read_at(0, &mut buf).unwrap();
            assert_eq!(n, payload.len());
            assert_eq!(&buf, payload);

            let mut mid = [0u8; 5];
            let n = source.read_at(6, &mut mid).unwrap();
            assert_eq!(n, 5);
            assert_eq!(&mid, &payload[6..11]);

            let mut exact = vec![0u8; payload.len()];
            source.read_exact_at(0, &mut exact).unwrap();
            assert_eq!(exact, payload);

            let mut past_end = [0u8; 4];
            let result = source.read_at(1_000_000, &mut past_end);
            assert!(matches!(result, Err(PackError::Truncated(1_000_000))));

            let mut short = [0u8; 100];
            let n = source.read_at((payload.len() - 3) as u64, &mut short).unwrap();
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

            let leftover_tmp = std::fs::read_dir(&dir)
                .unwrap()
                .filter_map(|entry| entry.ok())
                .any(|entry| entry.file_name().to_string_lossy().contains(".tmp-"));
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
        fn uncompressed_segment_wire_len(payload_len: usize) -> u64 {
            let mut len_bytes = Vec::new();
            pack_core::write_varint_u64(&mut len_bytes, payload_len as u64);
            (1 + 1 + len_bytes.len() + payload_len + 4) as u64
        }

        fn no_compression_options() -> WriteOptions {
            WriteOptions { required_flags: 0, optional_flags: pack_format::OPTIONAL_STREAMED, codec: CodecId(0) }
        }

        #[test]
        fn streaming_writer_full_session_multiple_segments_and_a_chunk_round_trips() {
            let dir = scratch_dir("stream_session");
            let path = dir.join("session.spk");
            let doc_payload = b"the streamed document body";
            let chunk_payload = b"a chunk of blob bytes carried alongside the document";
            let schema_payload = b"an extra schema segment written after the chunk";

            let mut writer = StreamingPackWriter::create(&path, &no_compression_options()).unwrap();
            writer.write_segment(KIND_DOCUMENT, doc_payload).unwrap();
            let chunk_id = writer.write_chunk(chunk_payload).unwrap();
            writer.write_segment(KIND_SCHEMA, schema_payload).unwrap();

            let manifest = Manifest {
                schema_name: String::new(),
                schema_hash: [0u8; 32],
                doc_span: ByteRange { offset: pack_format::HEADER_SIZE as u64, len: uncompressed_segment_wire_len(doc_payload.len()) },
                doc_frame_count: 1,
                symbols_span: ByteRange { offset: 0, len: 0 },
                chunk_table_span: ByteRange { offset: 0, len: 0 },
                field_index_span: ByteRange { offset: 0, len: 0 },
                uncompressed_body_len: doc_payload.len() as u64,
                field_count: 0,
                chunk_count: 0,
                symbol_count: 0,
            };
            writer.finish(&manifest).unwrap();

            let source = FilePackSource::open(&path).unwrap();
            let limits = PackLimits::default();
            let pack_file = pack_format::PackFile::open_manifest(source, &limits, VerificationLevel::Standard).unwrap();

            let loaded_manifest = pack_file.manifest().unwrap();
            assert_eq!(loaded_manifest.doc_frame_count, 1);
            assert_eq!(loaded_manifest.uncompressed_body_len, doc_payload.len() as u64);
            assert_eq!(pack_file.chunk_count(), 1);

            let read_chunk = pack_file.read_chunk(chunk_id, VerificationLevel::Full).unwrap();
            assert_eq!(read_chunk, chunk_payload);

            // `Full` verification also cross-checks the concatenated document body against the
            // footer's blake3 content hash, proving StreamingPackWriter's running hash matches.
            let body = pack_file.body_bytes(VerificationLevel::Full).unwrap();
            assert_eq!(body, doc_payload);

            // Forward-scan recovery should independently see every segment this session wrote:
            // document, chunk, schema, chunk table, manifest, end.
            let report = recover_file(&path, &limits).unwrap();
            assert_eq!(report.segments_recovered, 6);
            assert!(report.manifest.is_some());
        }
        //#endregion 🔖️Stream

        //#region 🔖️Recover
        fn build_valid_session_file(dir: &Path, name: &str) -> std::path::PathBuf {
            let path = dir.join(name);
            let doc_payload = b"recoverable document body";
            let chunk_payload = b"recoverable chunk payload";

            let mut writer = StreamingPackWriter::create(&path, &no_compression_options()).unwrap();
            writer.write_segment(KIND_DOCUMENT, doc_payload).unwrap();
            writer.write_chunk(chunk_payload).unwrap();

            let manifest = Manifest {
                schema_name: String::new(),
                schema_hash: [0u8; 32],
                doc_span: ByteRange { offset: pack_format::HEADER_SIZE as u64, len: uncompressed_segment_wire_len(doc_payload.len()) },
                doc_frame_count: 1,
                symbols_span: ByteRange { offset: 0, len: 0 },
                chunk_table_span: ByteRange { offset: 0, len: 0 },
                field_index_span: ByteRange { offset: 0, len: 0 },
                uncompressed_body_len: doc_payload.len() as u64,
                field_count: 0,
                chunk_count: 0,
                symbol_count: 0,
            };
            writer.finish(&manifest).unwrap();
            path
        }

        #[test]
        fn recover_file_with_footer_stripped_still_recovers_every_body_segment() {
            let dir = scratch_dir("recover_no_footer");
            let path = build_valid_session_file(&dir, "truncated_footer.spk");
            let limits = PackLimits::default();

            let full_len = std::fs::metadata(&path).unwrap().len();
            let file = std::fs::OpenOptions::new().write(true).open(&path).unwrap();
            file.set_len(full_len - pack_format::FOOTER_SIZE as u64).unwrap();
            drop(file);

            // Superblock open must fail (no valid footer) — this is the scenario recover_file is for.
            let source = FilePackSource::open(&path).unwrap();
            assert!(pack_format::PackFile::open_superblock(source, &limits).is_err());

            let report = recover_file(&path, &limits).unwrap();
            // document, chunk, chunk_table (since one chunk was written), manifest, end.
            assert_eq!(report.segments_recovered, 5);
            assert!(report.bytes_recovered > 0);
            assert!(report.manifest.is_some());
        }

        #[test]
        fn recover_file_truncated_mid_segment_recovers_a_strict_prefix_without_panicking() {
            let dir = scratch_dir("recover_mid_segment");
            let path = build_valid_session_file(&dir, "truncated_mid.spk");
            let limits = PackLimits::default();

            let full_len = std::fs::metadata(&path).unwrap().len();
            // Cut well past the footer AND into the tail of the body stream, guaranteeing the
            // last segment present (the `End` marker) is itself truncated mid-frame.
            let cut_len = full_len - pack_format::FOOTER_SIZE as u64 - 3;
            let file = std::fs::OpenOptions::new().write(true).open(&path).unwrap();
            file.set_len(cut_len).unwrap();
            drop(file);

            let report = recover_file(&path, &limits).unwrap();
            // Fewer than the full 5 segments (document, chunk, chunk_table, manifest, end) since
            // the trailing bytes of the last segment are gone; recovery must stop cleanly there,
            // never panic, and never report more than what a full session would produce.
            assert!(report.segments_recovered < 5);
            assert!(report.segments_recovered >= 1);
        }

        #[test]
        fn recover_file_on_a_file_too_short_for_a_header_errors_never_panics() {
            let dir = scratch_dir("recover_too_short");
            let path = dir.join("empty.spk");
            std::fs::write(&path, b"short").unwrap();
            let limits = PackLimits::default();
            let result = recover_file(&path, &limits);
            assert!(matches!(result, Err(PackError::Truncated(_))));
        }
        //#endregion 🔖️Recover
    }
    //#endregion 🧪️Tests
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;
