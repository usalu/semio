//! 🎞️ Protocol native `.spr` file lifecycle: create/open/resume-append/read-only-open over a real
//! file (wrapping `crate::os_pack::io::FilePackSource`/`FilePackSink`), `.sprc` sidecar checkpoint bodies,
//! forward-scan recovery, poll-based live tailing, and physical compaction. Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` (`## protocol_io`).
//!
//! Whole crate is native-only (`std::fs`) and gated behind `#[cfg(not(target_arch = "wasm32"))]`
//! so it still compiles — as an effectively-empty crate — for a `wasm32-unknown-unknown` target
//! check, mirroring `pack_io`'s pattern exactly.

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};

    use crate::os_pack::{CodecId, PackSource};
    use crate::os_spr::wire::{DictBuilder, ProtocolError, ProtocolLimits, RecordHasher};
    use crate::os_spr::format::{parse_commit_payload, read_header, recover as recover_records, Blake3Hasher, FrameCursor, RecoveryMode, SprWriter, VerificationLevel, WriteOptions, COMMIT_FRAME_LEN, HEADER_SIZE};
    use crate::os_spr::history::{decode_history, encode_active, encode_alternative, encode_change, encode_checkpoint, encode_doc, encode_edit, DecodeOptions, HistoryAppender, HistoryEdit, HistoryReader};

    /// @emoji 🚨️ Wraps a `std::io::Error` into the crate-wide `ProtocolError::Io` variant — the
    /// only place `std::io::Error` is allowed to appear, per the family's no-`std::io::Error`-in-
    /// public-signatures rule.
    #[allow(clippy::needless_pass_by_value)] // used as a `map_err` callback, which passes the error by value
    fn io_err(err: std::io::Error) -> ProtocolError {
        ProtocolError::Io(err.to_string())
    }

    //#region 🔖️File
    /// @emoji 📍️ Where a `.spr` file's trusted content currently ends, and the commit-chain state
    /// at that point — everything a caller needs to keep appending or to report file health.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ResumeState {
        pub end_offset: u64,
        pub last_commit_seq: u64,
        pub chain_hash: [u8; 32],
    }

    /// @emoji 📖️ Re-derives a `ResumeState` from whatever `path` currently contains: runs
    /// `crate::os_spr::format::recover` (`RecoveryMode::LastCommit`), then reads back either the
    /// header-only `chain_0` (no commit yet) or the trusted tail's `REC_COMMIT` payload.
    fn resume_state_for(path: &Path, limits: &ProtocolLimits) -> Result<ResumeState, ProtocolError> {
        let source = crate::os_pack::io::FilePackSource::open(path)?;
        let recovery = recover_records(&source, limits, RecoveryMode::LastCommit)?;
        if recovery.last_commit_seq == 0 {
            let mut header_bytes = [0u8; HEADER_SIZE];
            source.read_exact_at(0, &mut header_bytes)?;
            return Ok(ResumeState { end_offset: recovery.bytes_recovered, last_commit_seq: 0, chain_hash: Blake3Hasher.hash(&header_bytes) });
        }
        let mut commit_bytes = vec![0u8; COMMIT_FRAME_LEN as usize];
        source.read_exact_at(recovery.last_commit_offset, &mut commit_bytes)?;
        // `FrameCursor::new` indexes directly into the slice it's given, so the cursor's own
        // `start_offset` must be 0 here (the local offset within `commit_bytes`), not the file's
        // absolute `last_commit_offset` — see crate::os_spr::format::FrameCursor::new's doc.
        let mut cursor = FrameCursor::new(&commit_bytes, 0);
        let frame = cursor.next_frame()?.ok_or_else(|| ProtocolError::Malformed { what: "resume commit frame", offset: recovery.last_commit_offset, detail: "expected a REC_COMMIT frame at the recovered commit offset".to_string() })?;
        let commit = parse_commit_payload(frame.payload())?;
        Ok(ResumeState { end_offset: recovery.bytes_recovered, last_commit_seq: commit.commit_seq, chain_hash: commit.chain_hash })
    }

    /// @emoji 📼️ A live `.spr` file handle: either a fresh/resumed write path (`appender` set) or a
    /// pure inspection handle (`open_read_only`, `appender` unset). One struct for both so
    /// `resume_state()` reports uniformly regardless of how the handle was opened.
    pub struct HistoryFile {
        appender: Option<HistoryAppender<crate::os_pack::io::FilePackSink>>,
        resume: ResumeState,
    }

    impl HistoryFile {
        /// @emoji 🆕️ Creates `path` fresh (truncating any existing file) and writes the header plus
        /// the `REC_DOC` record. No commit has happened yet — `resume_state().last_commit_seq == 0`
        /// until the caller's first `appender().commit()`.
        pub fn create(path: &Path, doc_id: &str, schema: &str, options: &WriteOptions) -> Result<Self, ProtocolError> {
            let sink = crate::os_pack::io::FilePackSink::create(path)?;
            let appender = HistoryAppender::begin(sink, doc_id, schema, options)?;
            let source = crate::os_pack::io::FilePackSource::open(path)?;
            let mut header_bytes = [0u8; HEADER_SIZE];
            source.read_exact_at(0, &mut header_bytes)?;
            let resume = ResumeState { end_offset: source.len(), last_commit_seq: 0, chain_hash: Blake3Hasher.hash(&header_bytes) };
            Ok(Self { appender: Some(appender), resume })
        }

        /// @emoji ▶️ Opens an existing `.spr` file to keep appending to it.
        ///
        /// 🎯️ Design choice (forced by the crate boundary): `crate::os_spr::format::SprWriter::begin` and
        /// `crate::os_spr::history::HistoryAppender::begin` are the ONLY public constructors for those
        /// types, and both unconditionally write a fresh header (`begin`) / fresh header+`REC_DOC`
        /// (`HistoryAppender::begin`) at the sink's current position — there is no "resume a writer
        /// mid-stream, preserving its running chain hash / dictionary / commit sequence" entry point
        /// anywhere in `protocol_format`/`protocol_history`'s frozen public API, and this crate may
        /// not add one (out of scope: another crate's file). The only correctness-preserving way to
        /// produce a live `HistoryAppender` that continues coherently from existing content is
        /// therefore: recover the trusted prefix, fully decode it to a `HistoryLog`, discard the
        /// physical file (`crate::os_pack::io::FilePackSink::create` truncates), and replay every edit/change/
        /// checkpoint/alternative/active back through a freshly-begun appender's own public methods
        /// (which is what correctly rebuilds its internal dictionary/edit-ordinal bookkeeping) before
        /// handing it back for further live appends. This is O(file size) on every resume rather than
        /// O(torn tail) — a real cost worth revisiting if/when `protocol_format`/`protocol_history`
        /// grow a genuine resume constructor — but it is the only available option that never
        /// corrupts the file. Caveat: `HistoryLog` (protocol_history's model) has no slot for
        /// `REC_PROJECTION`/`REC_INDEX`/`REC_SEALED`/`REC_EPHEMERAL` records, so a resume drops any
        /// of those (this crate deliberately has no `protocol_materialize` dependency to decode
        /// projection bodies with). The op log itself — every edit/change/checkpoint/alternative — is
        /// fully preserved; only acceleration/snapshot data is lost, which the wider system already
        /// tolerates gracefully (`crate::os_spr::materialize::resolve_plan` falls back to full replay from
        /// genesis when a projection is missing/corrupt).
        pub fn open_append(path: &Path, limits: &ProtocolLimits) -> Result<Self, ProtocolError> {
            let source = crate::os_pack::io::FilePackSource::open(path)?;
            let recovery = recover_records(&source, limits, RecoveryMode::LastCommit)?;
            let header = read_header(&source)?;
            let mut trusted = vec![0u8; recovery.bytes_recovered as usize];
            source.read_exact_at(0, &mut trusted)?;
            drop(source);

            let decode_options = DecodeOptions { verification: VerificationLevel::Standard, limits: limits.clone() };
            let log = decode_history(&trusted, &decode_options)?;

            let write_options = WriteOptions { required_flags: header.required_flags, optional_flags: header.optional_flags };
            let sink = crate::os_pack::io::FilePackSink::create(path)?;
            let mut appender = HistoryAppender::begin(sink, &log.doc_id, &log.schema, &write_options)?;
            for edit in &log.edits {
                appender.append_edit(edit)?;
            }
            for change in &log.changes {
                appender.append_change(change)?;
            }
            for checkpoint in &log.checkpoints {
                appender.append_checkpoint(checkpoint)?;
            }
            for alternative in &log.alternatives {
                appender.append_alternative(alternative)?;
            }
            appender.set_active(log.active_alternative_id.as_deref())?;
            appender.commit()?;

            let resume = resume_state_for(path, limits)?;
            Ok(Self { appender: Some(appender), resume })
        }

        /// @emoji 👓️ Opens an existing `.spr` file purely for inspection: computes `resume_state()`
        /// via `crate::os_spr::format::recover` without writing a single byte to `path`. `appender()` must
        /// never be called on a handle opened this way (see its doc).
        pub fn open_read_only(path: &Path, limits: &ProtocolLimits) -> Result<Self, ProtocolError> {
            let resume = resume_state_for(path, limits)?;
            Ok(Self { appender: None, resume })
        }

        pub fn resume_state(&self) -> &ResumeState {
            &self.resume
        }

        /// @emoji ✍️ The live append handle. Panics if called on a handle from `open_read_only` —
        /// that constructor never builds a write path (see its doc); this mirrors the frozen
        /// contract's non-`Option` return type while keeping "read only" an honest guarantee (never
        /// touching the file) rather than a polite suggestion.
        pub fn appender(&mut self) -> &mut HistoryAppender<crate::os_pack::io::FilePackSink> {
            self.appender.as_mut().expect("HistoryFile::appender: this handle was opened via open_read_only and never built a write path")
        }
    }
    //#endregion 🔖️File

    //#region 🔖️Sidecar
    /// @emoji 🧾️ `.sprc` sidecar checkpoint bodies live beside the `.spr` file, named
    /// `<stem>.<hex8-of-body-hash>.sprc`.
    pub fn sidecar_path(protocol_path: &Path, body_hash: &[u8; 32]) -> PathBuf {
        let stem = protocol_path.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
        let mut hex8 = String::with_capacity(8);
        for byte in &body_hash[..4] {
            hex8.push_str(&format!("{byte:02x}"));
        }
        protocol_path.with_file_name(format!("{stem}.{hex8}.sprc"))
    }

    /// @emoji 💾️ Writes a complete `.spk` pack file as a sidecar, atomically (`crate::os_pack::io::write_atomic`
    /// — temp file + fsync + rename, so a reader never observes a torn sidecar).
    pub fn write_sidecar(protocol_path: &Path, body_hash: &[u8; 32], pack_bytes: &[u8]) -> Result<(), ProtocolError> {
        crate::os_pack::io::write_atomic(&sidecar_path(protocol_path, body_hash), pack_bytes)?;
        Ok(())
    }

    pub fn read_sidecar(protocol_path: &Path, body_hash: &[u8; 32]) -> Result<Vec<u8>, ProtocolError> {
        std::fs::read(sidecar_path(protocol_path, body_hash)).map_err(io_err)
    }
    //#endregion 🔖️Sidecar

    //#region 🔖️Recover
    pub fn recover_file(path: &Path, limits: &ProtocolLimits, mode: RecoveryMode) -> Result<crate::os_spr::format::RecoveryReport, ProtocolError> {
        let source = crate::os_pack::io::FilePackSource::open(path)?;
        recover_records(&source, limits, mode)
    }
    //#endregion 🔖️Recover

    //#region 🔖️Sync
    /// @emoji 🐌️ Poll-based live tailing, runtime-neutral (no tokio dependency in the type itself —
    /// the caller drives `poll()` from whatever scheduler it likes).
    pub struct TailFollower {
        path: PathBuf,
        /// @emoji 🔖️ The ordinal boundary this follower has consumed through: `poll()` returns
        /// edits starting at this ordinal and advances it past everything it returns. Equals the
        /// `from_edit_ordinal` given to `open` until the first `poll()` that returns at least one
        /// edit — a deliberate, documented reading of `last_edit_ordinal` given the contract leaves
        /// its exact off-by-one semantics unspecified.
        next_edit_ordinal: u64,
    }

    impl TailFollower {
        /// @emoji 📖️ Validates `path` looks like a real `.spr` file (header check) up front, so a
        /// bad path fails fast at `open` rather than on the first `poll`.
        pub fn open(path: &Path, from_edit_ordinal: u64) -> Result<Self, ProtocolError> {
            let source = crate::os_pack::io::FilePackSource::open(path)?;
            read_header(&source)?;
            Ok(Self { path: path.to_path_buf(), next_edit_ordinal: from_edit_ordinal })
        }

        /// @emoji 🔁️ Re-reads the whole file and re-decodes from the start every call.
        ///
        /// 🎯️ Design choice: `crate::os_spr::history::HistoryReader::edits()` always begins its own fresh
        /// `DictReader`/edit-id table at the start of the trusted record stream (`REC_*_DICT` deltas
        /// are interleaved incrementally through the file, so any single edit's dictrefs can only be
        /// resolved by walking from the top) — there is no public way to seed an `EditIter` with a
        /// prior poll's dictionary state. So a correct incremental "just the new tail" decode isn't
        /// reachable through this crate family's public API; re-scanning from the start on every
        /// poll is the only option, not merely the simplest one.
        pub fn poll(&mut self) -> Result<Vec<HistoryEdit>, ProtocolError> {
            let bytes = std::fs::read(&self.path).map_err(io_err)?;
            let reader = HistoryReader::open(&bytes, &DecodeOptions::default())?;
            let mut out = Vec::new();
            for edit in reader.edits().skip(self.next_edit_ordinal as usize) {
                out.push(edit?);
            }
            self.next_edit_ordinal += out.len() as u64;
            Ok(out)
        }

        pub fn last_edit_ordinal(&self) -> u64 {
            self.next_edit_ordinal
        }
    }
    //#endregion 🔖️Sync

    //#region 🔖️Compact
    pub struct CompactOptions {
        pub drop_ephemeral: bool,
        pub keep_snapshots: KeepSnapshots,
    }

    pub enum KeepSnapshots {
        All,
        LatestPerAlternative,
        LatestN(u32),
    }

    /// @emoji 🗂️ `REC_COMPACTION` payload layout — this crate's own choice (the contract fixes the
    /// `REC_COMPACTION` kind byte in `protocol_core` but defines no payload codec for it anywhere in
    /// the family): `format: u8 (=1), drop_ephemeral: u8 (0/1), keep_snapshots_tag: u8
    /// (0=All, 1=LatestPerAlternative, 2=LatestN), [latest_n: varint u64 iff tag==2]`.
    fn encode_compaction_payload(options: &CompactOptions) -> Vec<u8> {
        let mut out = crate::os_pack::ByteWriter::new();
        out.write_u8(1);
        out.write_u8(options.drop_ephemeral as u8);
        match options.keep_snapshots {
            KeepSnapshots::All => out.write_u8(0),
            KeepSnapshots::LatestPerAlternative => out.write_u8(1),
            KeepSnapshots::LatestN(n) => {
                out.write_u8(2);
                out.write_varint_u64(n as u64);
            }
        }
        out.into_bytes()
    }

    /// @emoji ✂️ Flushes a `REC_STR_DICT` delta record if `dict` grew since `*base` — byte-for-byte
    /// the same wire format `protocol_history`'s own (private) writer uses, reimplemented here since
    /// `compact` cannot drive `crate::os_spr::history::encode_history`/`HistoryAppender` as a black box (it
    /// needs to interleave one extra `REC_COMPACTION` record into the same commit generation, and
    /// neither type exposes a raw `write_record` passthrough for that).
    fn flush_dict(writer: &mut SprWriter<Vec<u8>>, dict: &DictBuilder, base: &mut u32) -> Result<(), ProtocolError> {
        let len = dict.len();
        if len > *base {
            let entries = dict.entries_since(*base);
            let mut payload = crate::os_pack::ByteWriter::new();
            payload.write_u8(1);
            payload.write_varint_u64(*base as u64);
            payload.write_varint_u64(entries.len() as u64);
            for entry in entries {
                payload.write_varint_u64(entry.len() as u64);
                payload.write_bytes(entry.as_bytes());
            }
            writer.write_record(crate::os_spr::REC_STR_DICT, true, &payload.into_bytes(), CodecId(0))?;
            *base = len;
        }
        Ok(())
    }

    /// @emoji 🧹️ Atomic rewrite via `crate::os_pack::io::write_atomic` (temp file + fsync + rename — a reader
    /// never observes a partially-compacted file): decodes the trusted prefix to a `HistoryLog`,
    /// re-encodes it into a brand-new single-generation `.spr` byte stream carrying one
    /// `REC_COMPACTION` provenance record, and swaps it in.
    ///
    /// 🎯️ `keep_snapshots`/`drop_ephemeral` are accepted and recorded verbatim into the
    /// `REC_COMPACTION` payload (so a reader can see what policy produced this generation), but have
    /// no additional filtering effect at this layer today: `HistoryLog` has no `REC_PROJECTION`/
    /// `REC_INDEX`/`REC_SEALED`/`REC_EPHEMERAL` slot to filter in the first place (see
    /// `HistoryFile::open_append`'s doc for the same crate-boundary caveat — this crate has no
    /// `protocol_materialize` dependency to interpret projection bodies with).
    pub fn compact(path: &Path, options: &CompactOptions, limits: &ProtocolLimits) -> Result<(), ProtocolError> {
        let source = crate::os_pack::io::FilePackSource::open(path)?;
        let recovery = recover_records(&source, limits, RecoveryMode::LastCommit)?;
        let header = read_header(&source)?;
        let mut trusted = vec![0u8; recovery.bytes_recovered as usize];
        source.read_exact_at(0, &mut trusted)?;
        drop(source);

        let decode_options = DecodeOptions { verification: VerificationLevel::Standard, limits: limits.clone() };
        let log = decode_history(&trusted, &decode_options)?;

        let write_options = WriteOptions { required_flags: header.required_flags, optional_flags: header.optional_flags };
        let mut writer = SprWriter::begin(Vec::<u8>::new(), &write_options)?;
        let mut dict = DictBuilder::new();
        let mut dict_base = 0u32;

        let doc_payload = encode_doc(&log.doc_id, &log.schema, &mut dict);
        flush_dict(&mut writer, &dict, &mut dict_base)?;
        writer.write_record(crate::os_spr::REC_DOC, true, &doc_payload, CodecId(0))?;

        let compaction_payload = encode_compaction_payload(options);
        writer.write_record(crate::os_spr::REC_COMPACTION, true, &compaction_payload, CodecId(0))?;

        let ordinals: HashMap<&str, u64> = log.edits.iter().enumerate().map(|(i, e)| (e.id.as_str(), i as u64)).collect();
        for edit in &log.edits {
            let payload = encode_edit(edit, &mut dict, |id| ordinals.get(id).copied())?;
            flush_dict(&mut writer, &dict, &mut dict_base)?;
            writer.write_record(crate::os_spr::REC_EDIT, true, &payload, CodecId(0))?;
        }
        for change in &log.changes {
            let payload = encode_change(change, &mut dict, |id| ordinals.get(id).copied())?;
            flush_dict(&mut writer, &dict, &mut dict_base)?;
            writer.write_record(crate::os_spr::REC_CHANGE, true, &payload, CodecId(0))?;
        }
        for checkpoint in &log.checkpoints {
            let payload = encode_checkpoint(checkpoint, &mut dict)?;
            flush_dict(&mut writer, &dict, &mut dict_base)?;
            writer.write_record(crate::os_spr::REC_CHECKPOINT, true, &payload, CodecId(0))?;
        }
        for alternative in &log.alternatives {
            let payload = encode_alternative(alternative, &mut dict)?;
            flush_dict(&mut writer, &dict, &mut dict_base)?;
            writer.write_record(crate::os_spr::REC_ALTERNATIVE, true, &payload, CodecId(0))?;
        }
        let active_payload = encode_active(log.active_alternative_id.as_deref(), &mut dict);
        flush_dict(&mut writer, &dict, &mut dict_base)?;
        writer.write_record(crate::os_spr::REC_ACTIVE, true, &active_payload, CodecId(0))?;

        writer.commit()?;
        crate::os_pack::io::write_atomic(path, &writer.into_sink())?;
        Ok(())
    }
    //#endregion 🔖️Compact

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::os_spr::history::{HistoryAlternative, HistoryChange, HistoryCheckpoint};
        use std::sync::atomic::{AtomicU64, Ordering};

        /// @emoji 🎲️ Per-test unique scratch directory under `std::env::temp_dir()` — no external
        /// `tempfile` crate dependency, matching `pack_io`'s own test convention.
        static DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

        fn scratch_dir(name: &str) -> PathBuf {
            let pid = std::process::id();
            let counter = DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
            let dir = std::env::temp_dir().join(format!("protocol_io_test_{name}_{pid}_{counter}"));
            std::fs::create_dir_all(&dir).expect("create scratch dir");
            dir
        }

        fn sample_edit(id: &str) -> HistoryEdit {
            HistoryEdit {
                id: id.to_string(),
                actor: Some("actor-1".to_string()),
                started_at: "2026-07-27T00:00:00Z".to_string(),
                finished_at: Some("2026-07-27T00:00:01Z".to_string()),
                coalesce_key: None,
                description: Some("a sample edit".to_string()),
                ops: vec![crate::os_spr::history::OpPayload { text: Some("set x 1".to_string()), binary: None }],
                inverse: Vec::new(),
                meta: None,
            }
        }

        //#region 🔖️File
        #[test]
        fn create_then_append_then_commit_round_trips_through_open_read_only() {
            let dir = scratch_dir("create_append");
            let path = dir.join("doc.spr");

            let mut file = HistoryFile::create(&path, "doc-1", "schema-1", &WriteOptions::default()).unwrap();
            assert_eq!(file.resume_state().last_commit_seq, 0);
            file.appender().append_edit(&sample_edit("edit-1")).unwrap();
            file.appender().append_edit(&sample_edit("edit-2")).unwrap();
            file.appender().commit().unwrap();

            let read_only = HistoryFile::open_read_only(&path, &ProtocolLimits::default()).unwrap();
            assert_eq!(read_only.resume_state().last_commit_seq, 1);
            assert!(read_only.resume_state().end_offset > HEADER_SIZE as u64);

            let bytes = std::fs::read(&path).unwrap();
            let log = decode_history(&bytes, &DecodeOptions::default()).unwrap();
            assert_eq!(log.doc_id, "doc-1");
            assert_eq!(log.edits.len(), 2);
            assert_eq!(log.edits[0].id, "edit-1");
        }

        #[test]
        #[should_panic(expected = "open_read_only")]
        fn appender_panics_on_a_read_only_handle() {
            let dir = scratch_dir("read_only_panic");
            let path = dir.join("doc.spr");
            HistoryFile::create(&path, "doc-1", "schema-1", &WriteOptions::default()).unwrap();
            let mut read_only = HistoryFile::open_read_only(&path, &ProtocolLimits::default()).unwrap();
            let _ = read_only.appender();
        }

        #[test]
        fn open_read_only_never_writes_to_the_file() {
            let dir = scratch_dir("read_only_no_write");
            let path = dir.join("doc.spr");
            let mut file = HistoryFile::create(&path, "doc-1", "schema-1", &WriteOptions::default()).unwrap();
            file.appender().append_edit(&sample_edit("edit-1")).unwrap();
            file.appender().commit().unwrap();
            let before = std::fs::read(&path).unwrap();

            let _read_only = HistoryFile::open_read_only(&path, &ProtocolLimits::default()).unwrap();
            let after = std::fs::read(&path).unwrap();
            assert_eq!(before, after, "open_read_only must never mutate the file on disk");
        }

        #[test]
        fn open_append_resumes_and_preserves_prior_edits_across_a_process_restart() {
            let dir = scratch_dir("open_append");
            let path = dir.join("doc.spr");

            {
                let mut file = HistoryFile::create(&path, "doc-1", "schema-1", &WriteOptions::default()).unwrap();
                file.appender().append_edit(&sample_edit("edit-1")).unwrap();
                file.appender().commit().unwrap();
            }
            {
                let mut file = HistoryFile::open_append(&path, &ProtocolLimits::default()).unwrap();
                assert_eq!(file.resume_state().last_commit_seq, 1, "the replay-commit during resume is itself commit #1 of a fresh generation");
                file.appender().append_edit(&sample_edit("edit-2")).unwrap();
                file.appender().commit().unwrap();
            }

            let bytes = std::fs::read(&path).unwrap();
            let log = decode_history(&bytes, &DecodeOptions::default()).unwrap();
            assert_eq!(log.doc_id, "doc-1");
            assert_eq!(log.edits.iter().map(|e| e.id.as_str()).collect::<Vec<_>>(), vec!["edit-1", "edit-2"]);
        }

        #[test]
        fn open_append_truncates_a_torn_tail_before_resuming() {
            let dir = scratch_dir("open_append_torn");
            let path = dir.join("doc.spr");
            {
                let mut file = HistoryFile::create(&path, "doc-1", "schema-1", &WriteOptions::default()).unwrap();
                file.appender().append_edit(&sample_edit("edit-1")).unwrap();
                file.appender().commit().unwrap();
            }
            // Simulate a crash mid-write: append garbage bytes past the last valid commit.
            {
                use std::io::Write;
                let mut f = std::fs::OpenOptions::new().append(true).open(&path).unwrap();
                f.write_all(&[0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x01, 0x02]).unwrap();
            }

            let mut file = HistoryFile::open_append(&path, &ProtocolLimits::default()).unwrap();
            file.appender().append_edit(&sample_edit("edit-2")).unwrap();
            file.appender().commit().unwrap();

            let bytes = std::fs::read(&path).unwrap();
            let log = decode_history(&bytes, &DecodeOptions::default()).unwrap();
            assert_eq!(log.edits.iter().map(|e| e.id.as_str()).collect::<Vec<_>>(), vec!["edit-1", "edit-2"]);
        }
        //#endregion 🔖️File

        //#region 🔖️Sidecar
        #[test]
        fn sidecar_write_read_round_trips_and_names_by_hex8_of_hash() {
            let dir = scratch_dir("sidecar");
            let protocol_path = dir.join("doc.spr");
            let body_hash = [0xABu8; 32];
            let pack_bytes = b"a complete .spk pack file, opaque to this crate";

            write_sidecar(&protocol_path, &body_hash, pack_bytes).unwrap();
            let expected_path = dir.join("doc.abababab.sprc");
            assert!(expected_path.exists());

            let read_back = read_sidecar(&protocol_path, &body_hash).unwrap();
            assert_eq!(read_back, pack_bytes);
        }

        #[test]
        fn read_sidecar_missing_file_is_an_io_error() {
            let dir = scratch_dir("sidecar_missing");
            let protocol_path = dir.join("doc.spr");
            let result = read_sidecar(&protocol_path, &[0u8; 32]);
            assert!(matches!(result, Err(ProtocolError::Io(_))));
        }
        //#endregion 🔖️Sidecar

        //#region 🔖️Recover
        #[test]
        fn recover_file_reports_the_committed_record_count() {
            let dir = scratch_dir("recover");
            let path = dir.join("doc.spr");
            let mut file = HistoryFile::create(&path, "doc-1", "schema-1", &WriteOptions::default()).unwrap();
            file.appender().append_edit(&sample_edit("edit-1")).unwrap();
            file.appender().append_edit(&sample_edit("edit-2")).unwrap();
            file.appender().commit().unwrap();

            let report = recover_file(&path, &ProtocolLimits::default(), RecoveryMode::LastCommit).unwrap();
            assert_eq!(report.last_commit_seq, 1);
            assert_eq!(report.torn_tail_bytes, 0);
            // At least REC_DOC + 2x REC_EDIT + REC_COMMIT; may include extra REC_STR_DICT delta
            // frames for interned actor/doc-id/schema strings, so this is a lower bound, not exact.
            assert!(report.records_recovered >= 4);
            assert!(report.bytes_recovered > HEADER_SIZE as u64);
        }
        //#endregion 🔖️Recover

        //#region 🔖️Sync
        #[test]
        fn tail_follower_polls_new_edits_across_multiple_commits_and_advances_its_ordinal() {
            let dir = scratch_dir("tail");
            let path = dir.join("doc.spr");
            let mut file = HistoryFile::create(&path, "doc-1", "schema-1", &WriteOptions::default()).unwrap();
            file.appender().append_edit(&sample_edit("edit-1")).unwrap();
            file.appender().commit().unwrap();

            let mut follower = TailFollower::open(&path, 0).unwrap();
            let first = follower.poll().unwrap();
            assert_eq!(first.iter().map(|e| e.id.as_str()).collect::<Vec<_>>(), vec!["edit-1"]);
            assert_eq!(follower.last_edit_ordinal(), 1);

            let empty = follower.poll().unwrap();
            assert!(empty.is_empty(), "no new edits since the last poll");

            file.appender().append_edit(&sample_edit("edit-2")).unwrap();
            file.appender().append_edit(&sample_edit("edit-3")).unwrap();
            file.appender().commit().unwrap();

            let second = follower.poll().unwrap();
            assert_eq!(second.iter().map(|e| e.id.as_str()).collect::<Vec<_>>(), vec!["edit-2", "edit-3"]);
            assert_eq!(follower.last_edit_ordinal(), 3);
        }

        #[test]
        fn tail_follower_open_from_a_nonzero_ordinal_skips_already_known_edits() {
            let dir = scratch_dir("tail_from_ordinal");
            let path = dir.join("doc.spr");
            let mut file = HistoryFile::create(&path, "doc-1", "schema-1", &WriteOptions::default()).unwrap();
            file.appender().append_edit(&sample_edit("edit-1")).unwrap();
            file.appender().append_edit(&sample_edit("edit-2")).unwrap();
            file.appender().commit().unwrap();

            let mut follower = TailFollower::open(&path, 1).unwrap();
            let polled = follower.poll().unwrap();
            assert_eq!(polled.iter().map(|e| e.id.as_str()).collect::<Vec<_>>(), vec!["edit-2"]);
        }
        //#endregion 🔖️Sync

        //#region 🔖️Compact
        #[test]
        fn compact_preserves_every_edit_and_records_a_compaction_provenance_marker() {
            let dir = scratch_dir("compact");
            let path = dir.join("doc.spr");
            let mut file = HistoryFile::create(&path, "doc-1", "schema-1", &WriteOptions::default()).unwrap();
            file.appender().append_edit(&sample_edit("edit-1")).unwrap();
            file.appender().append_edit(&sample_edit("edit-2")).unwrap();
            file.appender().append_change(&HistoryChange { id: "change-1".to_string(), saved_at: "2026-07-27T00:00:02Z".to_string(), edit_ids: vec!["edit-1".to_string(), "edit-2".to_string()], description: None }).unwrap();
            file.appender().append_checkpoint(&HistoryCheckpoint { id: "ck-1".to_string(), timestamp: "2026-07-27T00:00:03Z".to_string(), change_ids: vec!["change-1".to_string()], parent_id: None, authors: vec![], message: None }).unwrap();
            file.appender().append_alternative(&HistoryAlternative { id: "alt-1".to_string(), name: "main".to_string(), checkpoint_ids: vec!["ck-1".to_string()] }).unwrap();
            file.appender().set_active(Some("alt-1")).unwrap();
            file.appender().commit().unwrap();
            drop(file);

            let before = decode_history(&std::fs::read(&path).unwrap(), &DecodeOptions::default()).unwrap();

            compact(&path, &CompactOptions { drop_ephemeral: true, keep_snapshots: KeepSnapshots::LatestN(3) }, &ProtocolLimits::default()).unwrap();

            let after_bytes = std::fs::read(&path).unwrap();
            let after = decode_history(&after_bytes, &DecodeOptions::default()).unwrap();
            assert_eq!(before, after, "compact must be identity-preserving over everything HistoryLog models");

            // The REC_COMPACTION provenance marker is present immediately after REC_DOC (there may
            // be a REC_STR_DICT delta frame before REC_DOC, for the interned doc-id/schema/actor
            // strings, but nothing is ever written between REC_DOC and REC_COMPACTION).
            let mut cursor = FrameCursor::new(&after_bytes, HEADER_SIZE as u64);
            let mut kinds = Vec::new();
            while let Some(frame) = cursor.next_frame().unwrap() {
                kinds.push((frame.kind, frame.payload().to_vec()));
            }
            let doc_index = kinds.iter().position(|(kind, _)| *kind == crate::os_spr::REC_DOC).expect("REC_DOC present");
            let (compaction_kind, compaction_payload) = &kinds[doc_index + 1];
            assert_eq!(*compaction_kind, crate::os_spr::REC_COMPACTION);
            assert_eq!(compaction_payload[1], 1, "drop_ephemeral encoded as 1");
            assert_eq!(compaction_payload[2], 2, "keep_snapshots tag 2 = LatestN");

            // The commit chain genuinely restarted: exactly one commit, seq 1.
            let report = recover_file(&path, &ProtocolLimits::default(), RecoveryMode::LastCommit).unwrap();
            assert_eq!(report.last_commit_seq, 1);
        }

        #[test]
        fn compact_on_an_already_minimal_file_is_a_harmless_no_op_content_wise() {
            let dir = scratch_dir("compact_minimal");
            let path = dir.join("doc.spr");
            let mut file = HistoryFile::create(&path, "doc-1", "schema-1", &WriteOptions::default()).unwrap();
            file.appender().commit().unwrap();
            drop(file);

            compact(&path, &CompactOptions { drop_ephemeral: false, keep_snapshots: KeepSnapshots::All }, &ProtocolLimits::default()).unwrap();

            let log = decode_history(&std::fs::read(&path).unwrap(), &DecodeOptions::default()).unwrap();
            assert_eq!(log.doc_id, "doc-1");
            assert!(log.edits.is_empty());
        }
        //#endregion 🔖️Compact
    }
    //#endregion 🧪️Tests
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;
