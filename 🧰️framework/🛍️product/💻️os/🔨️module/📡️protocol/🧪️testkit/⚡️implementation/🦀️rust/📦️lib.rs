//! 🎞️ Protocol testkit: seeded generators for `protocol::HistoryLog`/causal-DAG fixtures, the
//! cross-crate law assertions the `protocol_*` family's round-trip/determinism/tamper-detection
//! invariants boil down to, a re-export of `pack_testkit`'s panic-safe corruption fuzzer, and the
//! golden-hash helper. Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` (`## protocol_testkit` +
//! `## Amendment`'s testing notes).
//!
//! This crate deliberately depends on the `protocol` facade rather than the individual
//! `protocol_*` sub-crates (matching every downstream consumer's own dependency rule) — with two
//! narrow exceptions (`protocol_core`, `protocol_format`) documented in `Cargo.toml`, needed only
//! because the facade re-exports frame-level *types* but not the raw-record-stream free functions
//! this crate's frame-level laws walk directly.

//#region 🔖️Gen
// Inline splitmix64 PRNG (NOT arbitrary/quickcheck/proptest — repo precedent, `pack_testkit::
// RecordValueGen`'s convention). Every generated string is built from an ASCII word alphabet plus,
// in `adversarial` mode, a small curated pool of multi-byte unicode characters spliced onto word
// boundaries — never at the very start/end and never containing `'\n'` or `'#'`, so a generated
// value is always safe to embed as an opaque single-line `.ops` op line (see `next_text`'s doc)
// while still exercising `dsl_schema`'s quoted-text escaping for every other field.

/// 🎲️ Deterministic seeded splitmix64 state, advanced on every draw — see
/// <https://prng.di.unimi.it/splitmix64.c>. Reused by every generator in this crate.
struct SplitMix64(u64);

impl SplitMix64 {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn next_bool(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }

    fn next_range(&mut self, bound: u64) -> u64 {
        if bound == 0 { 0 } else { self.next_u64() % bound }
    }
}

const WORD_ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";
const ADVERSARIAL_UNICODE: &[char] = &['é', 'ø', 'ñ', 'ß', '文', '档', '漢', '中', '🎉️', '🚀️'];

fn next_word(rng: &mut SplitMix64, max_len: usize) -> String {
    let len = 1 + rng.next_range(max_len as u64) as usize;
    (0..len).map(|_| WORD_ALPHABET[rng.next_range(WORD_ALPHABET.len() as u64) as usize] as char).collect()
}

/// ✏️ Free-form text safe as either a quoted `dsl_schema` field OR a raw opaque `.ops` op line: at
/// least one word, no leading/trailing whitespace, no `'\n'`, never trims to something starting
/// with `'#'` (which `protocol_history::parse_ops_text` would otherwise swallow as a comment).
/// `adversarial` widens the word count (exercising long descriptions) and splices a unicode
/// character onto the first word (never at a boundary, so trimming never changes the value).
fn next_text(rng: &mut SplitMix64, adversarial: bool) -> String {
    let word_count = if adversarial && rng.next_bool() { 20 + rng.next_range(60) as usize } else { 1 + rng.next_range(5) as usize };
    let mut words: Vec<String> = (0..word_count).map(|_| next_word(rng, 8)).collect();
    if adversarial && rng.next_bool() {
        if let Some(first) = words.first_mut() {
            first.push(ADVERSARIAL_UNICODE[rng.next_range(ADVERSARIAL_UNICODE.len() as u64) as usize]);
        }
    }
    words.join(" ")
}

/// 🪪️ A guaranteed-unique identifier (`"{prefix}-{index}"`, optionally adversarial-unicode-suffixed)
/// — uniqueness matters here since generated ids double as dictionary/edit-ordinal references.
fn next_ident(rng: &mut SplitMix64, prefix: &str, index: usize, adversarial: bool) -> String {
    let mut id = format!("{prefix}-{index}");
    if adversarial && rng.next_bool() {
        id.push(ADVERSARIAL_UNICODE[rng.next_range(ADVERSARIAL_UNICODE.len() as u64) as usize]);
    }
    id
}

/// ⏱️ Either a canonical `YYYY-MM-DDTHH:MM:SS[.fff]Z` string (exercising `protocol_core::scalar`'s
/// compact tag-1/2 timestamp encoding) or, in adversarial mode, a deliberately non-canonical raw
/// string (a non-UTC offset, free text, or a Z-suffixed string with an out-of-grammar shape) that
/// forces the tag-0 raw-text fallback — see `protocol_core::scalar`'s module note: correctness
/// never depends on which tag gets chosen, only on the round-trip equality check, so both branches
/// are safe regardless of exact calendar validity.
fn next_timestamp(rng: &mut SplitMix64, adversarial: bool) -> String {
    if adversarial && rng.next_bool() {
        match rng.next_range(3) {
            0 => "not-a-timestamp".to_string(),
            1 => format!("2024-01-15T10:30:00+0{}:00", 1 + rng.next_range(8)),
            _ => format!("2024-13-{:02}T99:99:99Z", 1 + rng.next_range(28)),
        }
    } else {
        let year = 2020 + rng.next_range(10);
        let month = 1 + rng.next_range(12);
        let day = 1 + rng.next_range(28);
        let hour = rng.next_range(24);
        let minute = rng.next_range(60);
        let second = rng.next_range(60);
        let ms = rng.next_range(1000);
        if ms == 0 {
            format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
        } else {
            format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{ms:03}Z")
        }
    }
}

/// 🎞️ Shape knobs for `HistoryLogGen::generate`. `adversarial: true` widens every generated field
/// (unicode ids, long descriptions, non-canonical timestamps forcing the raw-text fallback) and
/// makes a zero-op edit likely — the contract's "tiny/typical/adversarial" profile density is a
/// caller-side convention (pick small/medium/large `edit_count`/`max_ops_per_edit` plus this flag),
/// not three hardcoded presets.
#[derive(Clone, Debug)]
pub struct GenProfile {
    pub edit_count: usize,
    pub max_ops_per_edit: usize,
    pub checkpoint_every: usize,
    pub adversarial: bool,
}

/// 🎞️ Deterministic seeded `protocol::HistoryLog` fabricator.
pub struct HistoryLogGen {
    state: u64,
}

impl HistoryLogGen {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// 🌱️ Fabricates a `HistoryLog` matching `profile`: a doc identity, `edit_count` edits (each
    /// with 0..=`max_ops_per_edit` opaque op lines), a `HistoryChange` + `HistoryCheckpoint` every
    /// `checkpoint_every` edits (chained via `parent_id`), 0..=2 alternatives referencing a random
    /// subset of checkpoints, and an optional active alternative — every field this crate's own
    /// `assert_history_*` laws round-trip.
    pub fn generate(&mut self, profile: &GenProfile) -> protocol::HistoryLog {
        let mut rng = SplitMix64(self.state);

        let doc_id = next_ident(&mut rng, "doc", 0, profile.adversarial);
        let schema = next_ident(&mut rng, "schema", 0, profile.adversarial);

        let mut edits: Vec<protocol::HistoryEdit> = Vec::with_capacity(profile.edit_count);
        for i in 0..profile.edit_count {
            let id = next_ident(&mut rng, "edit", i, profile.adversarial);
            let started_at = next_timestamp(&mut rng, profile.adversarial);
            let finished_at = if rng.next_bool() { Some(next_timestamp(&mut rng, profile.adversarial)) } else { None };
            let actor = if rng.next_bool() { Some(next_ident(&mut rng, "actor", i, profile.adversarial)) } else { None };
            let coalesce_key = if rng.next_bool() { Some(next_ident(&mut rng, "key", i, profile.adversarial)) } else { None };
            let description = if rng.next_bool() { Some(next_text(&mut rng, profile.adversarial)) } else { None };
            let op_count = if profile.adversarial && rng.next_bool() { 0 } else { rng.next_range(profile.max_ops_per_edit as u64 + 1) as usize };
            let mut ops = Vec::with_capacity(op_count);
            for _ in 0..op_count {
                ops.push(protocol::OpPayload { text: Some(next_text(&mut rng, profile.adversarial)), binary: None });
            }
            edits.push(protocol::HistoryEdit { id, actor, started_at, finished_at, coalesce_key, description, ops, backwards: Vec::new(), meta: None });
        }

        let mut changes: Vec<protocol::HistoryChange> = Vec::new();
        let mut checkpoints: Vec<protocol::HistoryCheckpoint> = Vec::new();
        if profile.checkpoint_every > 0 {
            let mut boundary = 0usize;
            let mut prior_checkpoint: Option<String> = None;
            let mut index = 0usize;
            while boundary < edits.len() {
                let end = (boundary + profile.checkpoint_every).min(edits.len());
                let change_id = next_ident(&mut rng, "change", index, profile.adversarial);
                let edit_ids: Vec<String> = edits[boundary..end].iter().map(|edit| edit.id.clone()).collect();
                changes.push(protocol::HistoryChange {
                    id: change_id.clone(),
                    saved_at: next_timestamp(&mut rng, profile.adversarial),
                    edit_ids,
                    description: if rng.next_bool() { Some(next_text(&mut rng, profile.adversarial)) } else { None },
                });

                let checkpoint_id = next_ident(&mut rng, "checkpoint", index, profile.adversarial);
                let author_count = rng.next_range(3) as usize;
                let mut authors = Vec::with_capacity(author_count);
                for a in 0..author_count {
                    authors.push(protocol::HistoryAuthor { id: next_ident(&mut rng, "author", a, profile.adversarial), name: next_text(&mut rng, false) });
                }
                checkpoints.push(protocol::HistoryCheckpoint {
                    id: checkpoint_id.clone(),
                    timestamp: next_timestamp(&mut rng, profile.adversarial),
                    change_ids: vec![change_id],
                    parent_id: prior_checkpoint.clone(),
                    authors,
                    message: if rng.next_bool() { Some(next_text(&mut rng, profile.adversarial)) } else { None },
                });

                prior_checkpoint = Some(checkpoint_id);
                index += 1;
                boundary = end;
            }
        }

        let mut alternatives: Vec<protocol::HistoryAlternative> = Vec::new();
        if !checkpoints.is_empty() {
            let alternative_count = rng.next_range(3) as usize;
            for i in 0..alternative_count {
                let mut checkpoint_ids = Vec::new();
                for checkpoint in &checkpoints {
                    if rng.next_bool() {
                        checkpoint_ids.push(checkpoint.id.clone());
                    }
                }
                alternatives.push(protocol::HistoryAlternative { id: next_ident(&mut rng, "alt", i, profile.adversarial), name: next_text(&mut rng, false), checkpoint_ids });
            }
        }

        let active_alternative_id = if !alternatives.is_empty() && rng.next_bool() {
            let index = rng.next_range(alternatives.len() as u64) as usize;
            Some(alternatives[index].id.clone())
        } else {
            None
        };

        self.state = rng.0;
        protocol::HistoryLog { doc_id, schema, edits, changes, checkpoints, alternatives, active_alternative_id, cursor: None }
    }
}

/// 🎞️ Deterministic seeded random causal-DAG fabricator for `assert_op_dag_convergence`'s
/// exhaustive tier — `protocol_causal`'s own inline tests already cover a handful of hand-built
/// 3-4-node diamonds in every topological order at the `quick` tier (per the amendment's testing
/// note); this generates larger closed DAGs for a random-permutation sweep.
pub struct OpDagGen {
    state: u64,
}

impl OpDagGen {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// 🌱️ Generates `node_count` envelopes `"op-0".."op-{node_count-1}"`, each depending on 0..=2
    /// strictly earlier-indexed operations — dependencies always reference a smaller index, so the
    /// result is by construction a closed, acyclic dependency graph over exactly this returned set
    /// (every id a dependency names is itself present in the returned `Vec`).
    pub fn generate(&mut self, node_count: usize) -> Vec<protocol::OperationEnvelope> {
        let mut rng = SplitMix64(self.state);
        let mut envelopes = Vec::with_capacity(node_count);
        for i in 0..node_count {
            let dep_count = if i == 0 { 0 } else { rng.next_range(3.min(i as u64 + 1)) as usize };
            let mut dependencies: Vec<protocol::OperationId> = Vec::with_capacity(dep_count);
            for _ in 0..dep_count {
                let dep_id = protocol::OperationId(format!("op-{}", rng.next_range(i as u64)));
                if !dependencies.contains(&dep_id) {
                    dependencies.push(dep_id);
                }
            }
            envelopes.push(protocol::OperationEnvelope {
                operation_id: protocol::OperationId(format!("op-{i}")),
                document_id: protocol::DocumentId("doc-1".to_string()),
                actor: protocol::ActorId(format!("actor-{}", rng.next_range(4))),
                dependencies,
                diff: protocol::DocumentDiff { schema: protocol::SchemaId("testkit.op".to_string()), payload: format!("index:{i}").into_bytes() },
                inverse: protocol::InverseOperation { schema: protocol::SchemaId("testkit.op".to_string()), payload: Vec::new() },
                timestamp: protocol::HybridLogicalTimestamp::new(i as u64, i as u64 * 10),
            });
        }
        self.state = rng.0;
        envelopes
    }
}
//#endregion 🔖️Gen

//#region 🔖️Laws
// `protocol_history::encode_history`/`decode_history`/`parse_ops_text`/`print_ops_text` are the
// whole-batch codec and text-grammar twin the frozen contract's laws are phrased against; the
// `protocol` facade doesn't re-export them (only `HistoryAppender`/`HistoryReader` and the
// `compile_ops`/`decompile_ops` ops-text wrappers), so this crate depends on `protocol_history`
// directly for the struct-level and text-level laws below (see `Cargo.toml`'s note).

/// 🖇️ `.spr` bytes for `log`: `commit_after_every_record: false` uses the whole-batch
/// `protocol_history::encode_history` codec directly (buffered); `true` streams it through
/// `protocol::HistoryAppender`, committing after every single record (the hot-path shape a live
/// appender actually uses) — see `assert_streamed_equals_buffered`.
fn write_history_log(log: &protocol::HistoryLog, commit_after_every_record: bool) -> Vec<u8> {
    if !commit_after_every_record {
        return protocol_history::encode_history(log, &protocol_history::EncodeOptions::default()).expect("encode_history must succeed for a testkit-generated log");
    }
    let options = protocol::WriteOptions { required_flags: 0, optional_flags: 0 };
    let mut appender =
        protocol::HistoryAppender::begin(Vec::new(), &log.doc_id, &log.schema, &options).expect("HistoryAppender::begin must succeed for a well-formed doc_id/schema");
    appender.commit().expect("commit after REC_DOC");
    for edit in &log.edits {
        appender.append_edit(edit).expect("append_edit must succeed for a testkit-generated edit");
        appender.commit().expect("commit after edit");
    }
    for change in &log.changes {
        appender.append_change(change).expect("append_change must succeed for a testkit-generated change");
        appender.commit().expect("commit after change");
    }
    for checkpoint in &log.checkpoints {
        appender.append_checkpoint(checkpoint).expect("append_checkpoint must succeed for a testkit-generated checkpoint");
        appender.commit().expect("commit after checkpoint");
    }
    for alternative in &log.alternatives {
        appender.append_alternative(alternative).expect("append_alternative must succeed for a testkit-generated alternative");
        appender.commit().expect("commit after alternative");
    }
    appender.set_active(log.active_alternative_id.as_deref()).expect("set_active must always succeed");
    appender.commit().expect("final commit");
    appender.into_sink()
}

fn read_history_log(bytes: &[u8]) -> protocol::HistoryLog {
    protocol_history::decode_history(bytes, &protocol_history::DecodeOptions::default()).expect("decode_history must succeed for freshly-written bytes")
}

/// ✅️ LAW: `decode_history(&encode_history(log, _)) == log`.
pub fn assert_history_encode_decode_identity(log: &protocol::HistoryLog) {
    let decoded = read_history_log(&write_history_log(log, false));
    assert_eq!(&decoded, log, "encode_history/decode_history round trip diverged from the source HistoryLog");
}

/// ✅️ LAW: `encode_history` is byte-for-byte deterministic — encoding the same `HistoryLog` twice
/// produces identical bytes.
pub fn assert_history_canonical_stable(log: &protocol::HistoryLog) {
    let a = write_history_log(log, false);
    let b = write_history_log(log, false);
    assert_eq!(a, b, "encode_history must be byte-identical across repeated calls (canonical determinism law)");
}

/// ✅️ LAW: `parse_ops_text -> encode -> decode -> print_ops_text` is a fixpoint — reprinting the
/// result of one full round trip through `parse_ops_text`/`encode_history`/`decode_history`/
/// `print_ops_text` must reproduce the same text on a second parse/print pass (comments/blank
/// lines normalize away on the FIRST parse, matching `protocol_history`'s own
/// `ops_text_is_a_fixpoint_under_reprint` precedent test).
pub fn assert_ops_protocol_bidirectional(ops_text: &str) {
    let encode_options = protocol_history::EncodeOptions::default();
    let decode_options = protocol_history::DecodeOptions::default();

    let log = protocol_history::parse_ops_text(ops_text).expect("parse_ops_text must succeed for well-formed ops text");
    let bytes = protocol_history::encode_history(&log, &encode_options).expect("encode_history must succeed for a well-formed HistoryLog");
    let decoded = protocol_history::decode_history(&bytes, &decode_options).expect("decode_history must succeed on encode_history's own output");
    let printed = protocol_history::print_ops_text(&decoded).expect("print_ops_text must succeed for text-only generated ops");

    let reparsed = protocol_history::parse_ops_text(&printed).expect("parse_ops_text must succeed on print_ops_text's own output");
    assert_eq!(protocol_history::print_ops_text(&reparsed).unwrap(), printed, "print_ops_text(parse_ops_text(text)) must be a fixpoint under a second parse/print pass");
}

/// ✅️ LAW: streaming `log` through `protocol::HistoryAppender` one commit per record decodes
/// identically to a single-shot `protocol_history::encode_history` of the whole log.
pub fn assert_streamed_equals_buffered(log: &protocol::HistoryLog) {
    let buffered = read_history_log(&write_history_log(log, false));
    let streamed = read_history_log(&write_history_log(log, true));
    assert_eq!(buffered, streamed, "HistoryAppender-per-record streaming and encode_history(whole log) must decode identically");
}

/// ✅️ LAW: every `RecordFrame::payload()` returned while sweeping `bytes` borrows zero-copy from
/// `bytes` itself.
pub fn assert_zero_copy(bytes: &[u8]) {
    let bounds = bytes.as_ptr_range();
    let mut cursor = protocol::FrameCursor::new(bytes, protocol_format::HEADER_SIZE as u64);
    while let Some(frame) = cursor.next_frame().expect("assert_zero_copy requires an already-structurally-valid record stream") {
        let payload_bounds = frame.payload().as_ptr_range();
        assert!(
            payload_bounds.start >= bounds.start && payload_bounds.end <= bounds.end,
            "RecordFrame::payload() at offset {} must borrow zero-copy from the input slice",
            frame.offset
        );
    }
}

/// ✅️ LAW (softened to match the family's actual recovery-then-verify layering, see below): a
/// single-byte flip anywhere in `bytes` at or after the header must never go unnoticed under Full
/// verification. The frozen contract's prose says "must error", but per-frame CRC-32C is checked
/// during `protocol_format::recover`'s own scan BEFORE `VerificationLevel::Full`'s chain-hash check
/// ever runs — `protocol_history`'s own precedent test (`history_full_verification_detects_tampering`
/// in `protocol/history/rs/lib.rs`) documents the same real outcome: a corrupted frame's own CRC
/// mismatch makes `recover` truncate the trusted range BEFORE it, so the decode either errors OR
/// silently comes back shorter than the original — never silently equal to it. This fn asserts
/// exactly that non-silent-corruption invariant, sampling `24` roughly-evenly-spaced byte offsets
/// rather than every byte (this crate's own `assert_recovery_truncates_to_commit` already covers an
/// exhaustive-if-requested sweep of the structurally-analogous truncation case).
pub fn assert_chain_detects_tamper(bytes: &[u8]) {
    let options = protocol::DecodeOptions { verification: protocol::VerificationLevel::Full, limits: protocol::ProtocolLimits::default() };
    let original =
        protocol::HistoryReader::open(bytes, &options).and_then(|reader| reader.log()).expect("assert_chain_detects_tamper requires an already-decodable, untampered input");

    for position in sampled_positions(bytes.len(), protocol_format::HEADER_SIZE, 24) {
        let mut tampered = bytes.to_vec();
        tampered[position] ^= 0xFF;
        let result = protocol::HistoryReader::open(&tampered, &options).and_then(|reader| reader.log());
        let went_unnoticed = matches!(&result, Ok(decoded) if decoded == &original);
        assert!(!went_unnoticed, "tampering byte {position} under Full verification must not silently decode to the original untampered log");
    }
}

/// 📐️ Up to `cap` roughly-evenly-spaced indices from `[floor, total)` — shared sampling core for
/// this crate's own byte-position law sweeps (`pack_testkit`'s equivalent is private).
fn sampled_positions(total: usize, floor: usize, cap: usize) -> Vec<usize> {
    if total <= floor {
        return Vec::new();
    }
    let span = total - floor;
    if span <= cap {
        return (floor..total).collect();
    }
    let step = span as f64 / cap as f64;
    let mut positions: Vec<usize> = (0..cap).map(|i| floor + ((i as f64 * step) as usize).min(span - 1)).collect();
    positions.sort_unstable();
    positions.dedup();
    positions
}

/// 📐️ Up to `cap` roughly-evenly-spaced truncation lengths in `[0, total]`, always including `0`
/// and `total`.
fn sampled_lengths(total: usize, cap: usize) -> Vec<usize> {
    if total == 0 {
        return vec![0];
    }
    if total <= cap {
        return (0..=total).collect();
    }
    let step = total as f64 / cap as f64;
    let mut lengths: Vec<usize> = (0..cap).map(|i| ((i as f64 * step) as usize).min(total)).collect();
    lengths.push(total);
    lengths.sort_unstable();
    lengths.dedup();
    lengths
}

/// ✅️ LAW: truncating `bytes` at every sampled length (density per `level`) then running
/// `protocol_format::recover` (`RecoveryMode::LastCommit`) always yields a trusted prefix that ends
/// exactly at a real `REC_COMMIT` frame — or, if no commit survived the truncation, exactly at the
/// 32-byte header.
pub fn assert_recovery_truncates_to_commit(bytes: &[u8], level: CorruptionLevel) {
    let limits = protocol::ProtocolLimits::default();
    let cap = match level {
        CorruptionLevel::Exhaustive => bytes.len(),
        CorruptionLevel::Long => 128,
        CorruptionLevel::Quick => 16,
    };
    for len in sampled_lengths(bytes.len(), cap) {
        let truncated = &bytes[..len];
        let Ok(recovery) = protocol_format::recover(&truncated, &limits, protocol::RecoveryMode::LastCommit) else {
            continue;
        };
        if recovery.last_commit_seq == 0 {
            assert_eq!(recovery.bytes_recovered as usize, protocol_format::HEADER_SIZE, "no commit recovered at truncation length {len} -> trusted prefix must be exactly the header");
            continue;
        }
        let trusted = &truncated[..recovery.bytes_recovered as usize];
        let mut cursor = protocol::FrameCursor::new(trusted, recovery.last_commit_offset);
        let frame = cursor
            .next_frame()
            .expect("the recovered trusted prefix must itself re-parse cleanly")
            .expect("a frame must exist at the reported last_commit_offset");
        assert_eq!(frame.kind, protocol_core::REC_COMMIT, "last_commit_offset must point at a REC_COMMIT frame (truncation length {len})");
        assert_eq!(frame.offset + frame.frame_len(), recovery.bytes_recovered, "bytes_recovered must end exactly after the last trusted commit frame (truncation length {len})");
    }
}

/// 🧮️ The `(kind, payload bytes)` multiset of every structural record (`REC_DOC`/`REC_EDIT`/
/// `REC_CHANGE`/`REC_CHECKPOINT`/`REC_ALTERNATIVE`/`REC_ACTIVE`) in `bytes`'s trusted prefix —
/// deliberately excludes `REC_COMMIT` (chain metadata, expected to differ across a compaction that
/// restarts the commit chain) and dictionary/index/sealed/compaction/projection/ephemeral kinds
/// (physical layout compaction is explicitly allowed to rewrite).
#[cfg(not(target_arch = "wasm32"))]
fn structural_records(bytes: &[u8], limits: &protocol::ProtocolLimits) -> std::collections::BTreeMap<(u8, Vec<u8>), usize> {
    let recovery = protocol_format::recover(&bytes, limits, protocol::RecoveryMode::LastCommit).expect("recover must succeed on an already-valid stream");
    let trusted = &bytes[..recovery.bytes_recovered as usize];
    let mut counts = std::collections::BTreeMap::new();
    let mut cursor = protocol::FrameCursor::new(trusted, protocol_format::HEADER_SIZE as u64);
    while let Some(frame) = cursor.next_frame().expect("trusted prefix must re-parse cleanly") {
        if matches!(
            frame.kind,
            protocol_core::REC_DOC | protocol_core::REC_EDIT | protocol_core::REC_CHANGE | protocol_core::REC_CHECKPOINT | protocol_core::REC_ALTERNATIVE | protocol_core::REC_ACTIVE
        ) {
            *counts.entry((frame.kind, frame.payload().to_vec())).or_insert(0) += 1;
        }
    }
    counts
}

#[cfg(not(target_arch = "wasm32"))]
fn scratch_spr_path(label: &str) -> std::path::PathBuf {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_nanos());
    std::env::temp_dir().join(format!("protocol-testkit-{label}-{}-{nanos}.spr", std::process::id()))
}

/// ✅️ LAW: `protocol::compact` is physically-rewriting-but-identity-preserving — the
/// `(kind, payload bytes)` multiset of every structural record is unchanged across a compaction
/// round trip. Native-only (`protocol::compact` is a file-based API): writes `bytes` to a scratch
/// temp file, compacts it in place, reads it back.
#[cfg(not(target_arch = "wasm32"))]
pub fn assert_compaction_identity(bytes: &[u8]) {
    let limits = protocol::ProtocolLimits::default();
    let before = structural_records(bytes, &limits);

    let path = scratch_spr_path("compact");
    std::fs::write(&path, bytes).expect("write scratch .spr file for compaction test");
    let options = protocol::CompactOptions { drop_ephemeral: true, keep_snapshots: protocol::KeepSnapshots::All };
    let compact_result = protocol::compact(&path, &options, &limits);
    let compacted = std::fs::read(&path);
    let _ = std::fs::remove_file(&path);

    compact_result.expect("protocol::compact must succeed on an already-valid stream");
    let after = structural_records(&compacted.expect("read back compacted .spr file"), &limits);
    assert_eq!(before, after, "compaction must preserve the (kind, payload bytes) multiset of every structural record");
}

/// ✅️ LAW: `OpText::print_op` output never contains `'\n'`, and `parse_op(op.print_op())` recovers
/// an equal operation.
pub fn assert_op_text_round_trip<Op>(op: &Op)
where
    Op: protocol::OpText + Clone + PartialEq + std::fmt::Debug,
{
    let line = op.print_op();
    assert!(!line.contains('\n'), "OpText::print_op output must never contain a newline, got {line:?}");
    let parsed = Op::parse_op(&line).unwrap_or_else(|error| panic!("OpText::parse_op failed to parse its own print_op output {line:?}: {error:?}"));
    assert_eq!(&parsed, op, "OpText::parse_op(op.print_op()) must recover an equal operation");
}

fn fisher_yates_shuffle(rng: &mut SplitMix64, items: &mut [usize]) {
    for i in (1..items.len()).rev() {
        let j = rng.next_range(i as u64 + 1) as usize;
        items.swap(i, j);
    }
}

/// ✅️ LAW: `OpDag` converges to the same fully-applied set regardless of insertion order, for
/// `permutation_count` random shuffles of `envelopes` (which must form a closed dependency set —
/// see `OpDagGen::generate`).
pub fn assert_op_dag_convergence(envelopes: &[protocol::OperationEnvelope], seed: u64, permutation_count: usize) {
    let expected: std::collections::BTreeSet<String> = envelopes.iter().map(|envelope| envelope.operation_id.0.clone()).collect();
    let mut rng = SplitMix64(seed);
    for _ in 0..permutation_count.max(1) {
        let mut order: Vec<usize> = (0..envelopes.len()).collect();
        fisher_yates_shuffle(&mut rng, &mut order);

        let mut dag = protocol::OpDag::new();
        for index in order {
            dag.insert(envelopes[index].clone()).expect("a closed dependency set inserted with unique ids must never duplicate");
        }
        let applied: std::collections::BTreeSet<String> = dag.drain_applied_envelopes().iter().map(|envelope| envelope.operation_id.0.clone()).collect();
        assert_eq!(applied, expected, "OpDag must converge to the same fully-applied set regardless of insertion order");
    }
}

/// ✅️ LAW: `merge_concurrent_diffs(strategy, a, b, ..) == merge_concurrent_diffs(strategy, b, a, ..)`.
pub fn assert_crdt_commutative<P, D>(strategy: protocol::MergeStrategyKind, a: D, b: D, meta_a: &protocol::OperationMeta, meta_b: &protocol::OperationMeta)
where
    D: protocol::OperationDiff<P> + PartialEq + std::fmt::Debug,
{
    let forward = protocol::merge_concurrent_diffs(strategy, a.clone(), b.clone(), meta_a, meta_b);
    let backward = protocol::merge_concurrent_diffs(strategy, b, a, meta_b, meta_a);
    assert_eq!(forward, backward, "merge_concurrent_diffs must be commutative for {strategy:?}");
}

/// ✅️ LAW: `merge_concurrent_diffs(strategy, a, a, ..) == a`.
pub fn assert_crdt_idempotent<P, D>(strategy: protocol::MergeStrategyKind, a: &D, meta_a: &protocol::OperationMeta)
where
    D: protocol::OperationDiff<P> + PartialEq + std::fmt::Debug,
{
    let merged = protocol::merge_concurrent_diffs(strategy, a.clone(), a.clone(), meta_a, meta_a);
    assert_eq!(&merged, a, "merge_concurrent_diffs must be idempotent for {strategy:?}");
}

/// 🎞️ Which side of the semio_hub wire protocol an `assert_wire_frame_round_trip` sample represents —
/// `ClientFrame`/`ServerFrame` are distinct enums with distinct encode/decode fn pairs, so this
/// crate's own choice (the contract names one law fn, not two) is a single sum type covering both.
pub enum WireFrameSample {
    Client(protocol::ClientFrame, protocol::Lane),
    Server(protocol::ServerFrame, protocol::Lane),
}

/// ✅️ LAW: `decode(encode(frame)) == frame`, for either wire direction.
pub fn assert_wire_frame_round_trip(sample: &WireFrameSample) {
    match sample {
        WireFrameSample::Client(frame, lane) => {
            let bytes = protocol::encode_client_frame(frame, *lane);
            let (decoded_lane, decoded_frame) = protocol::decode_client_frame(&bytes).expect("decode_client_frame must succeed on its own encode_client_frame output");
            assert_eq!(decoded_lane, *lane, "decoded Lane must match the encoded Lane");
            assert_eq!(&decoded_frame, frame, "decode_client_frame(encode_client_frame(frame)) must equal frame");
        }
        WireFrameSample::Server(frame, lane) => {
            let bytes = protocol::encode_server_frame(frame, *lane);
            let (decoded_lane, decoded_frame) = protocol::decode_server_frame(&bytes).expect("decode_server_frame must succeed on its own encode_server_frame output");
            assert_eq!(decoded_lane, *lane, "decoded Lane must match the encoded Lane");
            assert_eq!(&decoded_frame, frame, "decode_server_frame(encode_server_frame(frame)) must equal frame");
        }
    }
}

/// @emoji 🧵️ Which side of the app-engine channel an `assert_channel_frame_round_trip` sample
/// represents — `AppCommand`/`AppFrame` are distinct enums with distinct encode/decode fn pairs,
/// same rationale as `WireFrameSample` above.
pub enum ChannelFrameSample {
    Command(protocol::AppCommand),
    Frame(protocol::AppFrame),
}

/// ✅️ LAW: `decode(encode(frame)) == frame`, for either channel direction.
pub fn assert_channel_frame_round_trip(sample: &ChannelFrameSample) {
    match sample {
        ChannelFrameSample::Command(command) => {
            let bytes = protocol::encode_app_command(command);
            let decoded = protocol::decode_app_command(&bytes).expect("decode_app_command must succeed on its own encode_app_command output");
            assert_eq!(&decoded, command, "decode_app_command(encode_app_command(command)) must equal command");
        }
        ChannelFrameSample::Frame(frame) => {
            let bytes = protocol::encode_app_frame(frame);
            let decoded = protocol::decode_app_frame(&bytes).expect("decode_app_frame must succeed on its own encode_app_frame output");
            assert_eq!(&decoded, frame, "decode_app_frame(encode_app_frame(frame)) must equal frame");
        }
    }
}
//#endregion 🔖️Laws

//#region 🔖️Corrupt
/// 🛡️ Reused verbatim from `pack_testkit` — closure-generic panic-safety fuzzers plus their level/
/// report types. LAW (exercised in this crate's own tests against `protocol::HistoryReader::open`
/// and `protocol_format::recover`): `CorruptionReport::cases_panicked` must always be empty.
pub use pack_testkit::{CorruptionLevel, CorruptionReport, fuzz_bit_flips, fuzz_truncation};
//#endregion 🔖️Corrupt

//#region 🔖️Golden
pub use pack_testkit::golden_hash_hex;
//#endregion 🔖️Golden

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️Gen
    #[test]
    fn history_log_gen_is_deterministic_for_a_fixed_seed() {
        let profile = GenProfile { edit_count: 5, max_ops_per_edit: 3, checkpoint_every: 2, adversarial: false };
        let a = HistoryLogGen::new(42).generate(&profile);
        let b = HistoryLogGen::new(42).generate(&profile);
        assert_eq!(a, b, "the same seed must generate the same HistoryLog");
    }

    #[test]
    fn history_log_gen_different_seeds_usually_differ() {
        let profile = GenProfile { edit_count: 5, max_ops_per_edit: 3, checkpoint_every: 2, adversarial: false };
        let a = HistoryLogGen::new(1).generate(&profile);
        let b = HistoryLogGen::new(2).generate(&profile);
        assert_ne!(a, b);
    }

    #[test]
    fn history_log_gen_respects_edit_count_and_checkpoint_cadence() {
        let profile = GenProfile { edit_count: 9, max_ops_per_edit: 2, checkpoint_every: 3, adversarial: false };
        let log = HistoryLogGen::new(7).generate(&profile);
        assert_eq!(log.edits.len(), 9);
        assert_eq!(log.changes.len(), 3, "9 edits at checkpoint_every=3 must produce 3 changes");
        assert_eq!(log.checkpoints.len(), 3);
        for edit in &log.edits {
            assert!(edit.ops.len() <= 2);
            assert!(edit.meta.is_none(), "this generator never populates the derived-data meta slot");
        }
    }

    #[test]
    fn history_log_gen_zero_checkpoint_every_produces_no_changes_or_checkpoints() {
        let profile = GenProfile { edit_count: 4, max_ops_per_edit: 2, checkpoint_every: 0, adversarial: false };
        let log = HistoryLogGen::new(3).generate(&profile);
        assert!(log.changes.is_empty());
        assert!(log.checkpoints.is_empty());
        assert!(log.alternatives.is_empty(), "no checkpoints -> no alternatives to reference them");
    }

    #[test]
    fn history_log_gen_adversarial_op_text_never_breaks_the_ops_line_grammar() {
        let profile = GenProfile { edit_count: 40, max_ops_per_edit: 6, checkpoint_every: 0, adversarial: true };
        for seed in 0..20u64 {
            let log = HistoryLogGen::new(seed).generate(&profile);
            for edit in &log.edits {
                for op in &edit.ops {
                    let text = op.text.as_deref().expect("generator always sets text");
                    assert!(!text.is_empty(), "generated op text must never be empty");
                    assert!(!text.contains('\n'), "generated op text must never contain a newline");
                    assert_eq!(text.trim(), text, "generated op text must have no leading/trailing whitespace");
                    assert!(!text.starts_with('#'), "generated op text must never look like a comment line");
                }
            }
        }
    }

    #[test]
    fn op_dag_gen_produces_a_closed_topologically_orderable_set() {
        let envelopes = OpDagGen::new(11).generate(30);
        assert_eq!(envelopes.len(), 30);
        let known: std::collections::HashSet<String> = envelopes.iter().map(|envelope| envelope.operation_id.0.clone()).collect();
        for (index, envelope) in envelopes.iter().enumerate() {
            assert_eq!(envelope.operation_id.0, format!("op-{index}"));
            for dependency in &envelope.dependencies {
                assert!(known.contains(&dependency.0), "every dependency must reference a node within the generated set");
                let dep_index: usize = dependency.0.strip_prefix("op-").unwrap().parse().unwrap();
                assert!(dep_index < index, "every dependency must reference a strictly earlier index");
            }
        }
    }
    //#endregion 🔖️Gen

    //#region 🔖️Laws
    fn tiny_profile() -> GenProfile {
        GenProfile { edit_count: 3, max_ops_per_edit: 3, checkpoint_every: 0, adversarial: false }
    }

    fn typical_profile() -> GenProfile {
        GenProfile { edit_count: 20, max_ops_per_edit: 5, checkpoint_every: 4, adversarial: false }
    }

    fn adversarial_profile() -> GenProfile {
        GenProfile { edit_count: 15, max_ops_per_edit: 8, checkpoint_every: 5, adversarial: true }
    }

    #[test]
    fn history_encode_decode_identity_across_profiles() {
        for (seed, profile) in [(1u64, tiny_profile()), (2, typical_profile()), (3, adversarial_profile())] {
            let log = HistoryLogGen::new(seed).generate(&profile);
            assert_history_encode_decode_identity(&log);
        }
    }

    #[test]
    fn history_canonical_stable_across_profiles() {
        for (seed, profile) in [(4u64, tiny_profile()), (5, typical_profile()), (6, adversarial_profile())] {
            let log = HistoryLogGen::new(seed).generate(&profile);
            assert_history_canonical_stable(&log);
        }
    }

    #[test]
    fn history_streamed_equals_buffered_across_profiles() {
        for (seed, profile) in [(7u64, tiny_profile()), (8, typical_profile()), (9, adversarial_profile())] {
            let log = HistoryLogGen::new(seed).generate(&profile);
            assert_streamed_equals_buffered(&log);
        }
    }

    #[test]
    fn history_encode_decode_identity_handles_empty_edits_and_history() {
        assert_history_encode_decode_identity(&HistoryLogGen::new(10).generate(&GenProfile { edit_count: 0, max_ops_per_edit: 0, checkpoint_every: 0, adversarial: false }));
        let mut zero_op_profile = tiny_profile();
        zero_op_profile.max_ops_per_edit = 0;
        assert_history_encode_decode_identity(&HistoryLogGen::new(11).generate(&zero_op_profile));
    }

    #[test]
    fn ops_protocol_bidirectional_on_a_hand_written_sample() {
        assert_ops_protocol_bidirectional(
            "doc \"doc-1\" schema=\"schema-1\"\n\
             edit \"e0\" started=\"2026-07-27T00:00:00Z\" actor=\"actor-1\" description=\"first edit\"\n\
             \x20\x20set foo = 1\n\
             \x20\x20set bar = 2\n\
             edit \"e1\" started=\"2026-07-27T00:00:01Z\" finished=\"2026-07-27T00:00:05Z\"\n\
             \x20\x20noop\n",
        );
    }

    #[test]
    fn ops_protocol_bidirectional_skips_comments_and_blank_lines() {
        assert_ops_protocol_bidirectional("doc \"doc-2\" schema=\"schema-2\"\n\n# a comment before active\nactive \"alt-1\"\n");
    }

    #[test]
    fn ops_protocol_bidirectional_on_generated_logs() {
        for (seed, profile) in [(12u64, tiny_profile()), (13, typical_profile())] {
            let log = HistoryLogGen::new(seed).generate(&profile);
            let bytes = write_history_log(&log, false);
            let ops_text = protocol::decompile_ops(&bytes, &protocol::DecodeOptions::default()).expect("decompile_ops");
            assert_ops_protocol_bidirectional(&ops_text);
        }
    }

    #[test]
    fn zero_copy_holds_across_a_generated_history() {
        let log = HistoryLogGen::new(14).generate(&typical_profile());
        assert_zero_copy(&write_history_log(&log, false));
    }

    #[test]
    fn chain_detects_tamper_on_a_generated_history() {
        let log = HistoryLogGen::new(15).generate(&typical_profile());
        assert_chain_detects_tamper(&write_history_log(&log, false));
    }

    #[test]
    fn recovery_truncates_to_commit_quick() {
        let log = HistoryLogGen::new(16).generate(&typical_profile());
        assert_recovery_truncates_to_commit(&write_history_log(&log, true), CorruptionLevel::Quick);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn compaction_identity_on_a_generated_history() {
        let log = HistoryLogGen::new(17).generate(&typical_profile());
        assert_compaction_identity(&write_history_log(&log, false));
    }

    //#region 🏃️quick
    mod quick {
        use super::*;

        #[test]
        fn op_dag_convergence_holds_on_a_small_generated_dag() {
            let envelopes = OpDagGen::new(20).generate(6);
            assert_op_dag_convergence(&envelopes, 100, 8);
        }
    }
    //#endregion 🏃️quick

    //#region 🏃️exhaustive
    mod exhaustive {
        use super::*;

        #[test]
        fn op_dag_convergence_holds_on_a_larger_generated_dag() {
            let envelopes = OpDagGen::new(21).generate(60);
            assert_op_dag_convergence(&envelopes, 200, 40);
        }

        #[test]
        fn recovery_truncates_to_commit_exhaustive_on_a_small_fixture() {
            let log = HistoryLogGen::new(22).generate(&tiny_profile());
            assert_recovery_truncates_to_commit(&write_history_log(&log, true), CorruptionLevel::Exhaustive);
        }
    }
    //#endregion 🏃️exhaustive

    //#region 🧸️Fixtures
    // Dummy (P=i64, Op=AddOp) pair, the smallest possible Operation/OperationDiff/OpText impl —
    // mirrors the fixture pattern `protocol_command`/`protocol_causal`'s own inline tests use.
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    struct AddDiff {
        delta: i64,
    }
    impl protocol::OperationDiff<i64> for AddDiff {
        fn apply(&self, base: &i64) -> i64 {
            base + self.delta
        }
        fn absorb(&mut self, other: Self) {
            self.delta += other.delta;
        }
    }

    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct AddOp {
        delta: i64,
    }
    impl protocol::Operation<i64> for AddOp {
        type Diff = AddDiff;
        fn diff(&self, _base: &i64) -> AddDiff {
            AddDiff { delta: self.delta }
        }
        fn backwards(&self, _base: &i64) -> Vec<Self> {
            vec![AddOp { delta: -self.delta }]
        }
    }
    impl protocol::OpText for AddOp {
        fn print_op(&self) -> String {
            format!("add {}", self.delta)
        }
        fn parse_op(line: &str) -> Result<Self, dsl_core::TextError> {
            let rest = line.strip_prefix("add ").ok_or_else(|| dsl_core::TextError::new("expected 'add <n>'", dsl_core::TextSpan::at(1, 1)))?;
            let delta: i64 = rest.trim().parse().map_err(|_| dsl_core::TextError::new("invalid integer", dsl_core::TextSpan::at(1, 1)))?;
            Ok(AddOp { delta })
        }
    }

    // `RegisterDiff`: two independently-overwritable fields, used to demonstrate LwwRegister's
    // "discard the loser whole" behavior versus the semio_compose_rs strategies' "merge per field" behavior.
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    struct RegisterDiff {
        field_a: Option<i64>,
        field_b: Option<i64>,
    }
    impl protocol::OperationDiff<(i64, i64)> for RegisterDiff {
        fn apply(&self, base: &(i64, i64)) -> (i64, i64) {
            (self.field_a.unwrap_or(base.0), self.field_b.unwrap_or(base.1))
        }
        fn absorb(&mut self, other: Self) {
            if other.field_a.is_some() {
                self.field_a = other.field_a;
            }
            if other.field_b.is_some() {
                self.field_b = other.field_b;
            }
        }
    }

    fn meta_at(actor: u64, physical_ms: u64) -> protocol::OperationMeta {
        protocol::OperationMeta {
            operation_id: None,
            dependencies: Vec::new(),
            base_version: 0,
            author_id: None,
            timestamp: protocol::HybridLogicalTimestamp::new(actor, physical_ms),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
        }
    }
    //#endregion 🧸️Fixtures

    //#region 🔖️Laws (continued)
    #[test]
    fn op_text_round_trip_holds_and_rejects_a_broken_impl() {
        assert_op_text_round_trip(&AddOp { delta: -7 });
        assert_op_text_round_trip(&AddOp { delta: 0 });
    }

    #[test]
    fn operation_diff_apply_matches_backwards_inverse() {
        use protocol::{Operation, OperationDiff};
        let base: i64 = 10;
        let op = AddOp { delta: 5 };
        let forward = op.diff(&base).apply(&base);
        assert_eq!(forward, 15);
        let [undo] = <[AddOp; 1]>::try_from(op.backwards(&base)).unwrap();
        assert_eq!(undo.diff(&forward).apply(&forward), base);
    }

    #[test]
    #[should_panic(expected = "must recover an equal operation")]
    fn op_text_round_trip_panics_on_a_lossy_impl() {
        #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        struct LossyOp {
            delta: i64,
        }
        impl protocol::OpText for LossyOp {
            fn print_op(&self) -> String {
                "lossy".to_string()
            }
            fn parse_op(_line: &str) -> Result<Self, dsl_core::TextError> {
                Ok(LossyOp { delta: 0 })
            }
        }
        assert_op_text_round_trip(&LossyOp { delta: 42 });
    }

    #[test]
    fn crdt_commutative_and_idempotent_hold_for_every_strategy() {
        let a = RegisterDiff { field_a: Some(1), field_b: Some(2) };
        let b = RegisterDiff { field_a: Some(3), field_b: None };
        let ma = meta_at(1, 10);
        let mb = meta_at(2, 20);
        for strategy in [
            protocol::MergeStrategyKind::LwwRegister,
            protocol::MergeStrategyKind::OrderedSequence,
            protocol::MergeStrategyKind::TextSequence,
            protocol::MergeStrategyKind::TombstonedGraphSet,
            protocol::MergeStrategyKind::ContentAddressedBlob,
        ] {
            assert_crdt_commutative::<(i64, i64), RegisterDiff>(strategy, a.clone(), b.clone(), &ma, &mb);
            assert_crdt_idempotent::<(i64, i64), RegisterDiff>(strategy, &a, &ma);
        }
    }

    #[test]
    fn wire_frame_round_trip_holds_for_client_and_server_samples() {
        assert_wire_frame_round_trip(&WireFrameSample::Client(protocol::ClientFrame::Bye, protocol::Lane::Command));
        assert_wire_frame_round_trip(&WireFrameSample::Client(
            protocol::ClientFrame::PreviewPublish { key: "cursor".to_string(), seq: 3, payload: vec![1, 2, 3] },
            protocol::Lane::Preview,
        ));
        let frontier = protocol::RuntimeFrontierSummary {
            document_id: protocol::DocumentId("doc-1".to_string()),
            head_edit_ordinal: 5,
            head_edit_id: "edit-5".to_string(),
            last_commit_seq: 2,
            chain_hash: [7u8; 32],
        };
        assert_wire_frame_round_trip(&WireFrameSample::Server(
            protocol::ServerFrame::Welcome { session_id: "s1".to_string(), resume_token: "r1".to_string(), server_frontier: frontier, bootstrap: protocol::Bootstrap::Tail },
            protocol::Lane::Command,
        ));
    }

    #[test]
    fn channel_frame_round_trip_holds_for_command_and_frame_samples() {
        assert_channel_frame_round_trip(&ChannelFrameSample::Command(protocol::AppCommand::Bye));
        assert_channel_frame_round_trip(&ChannelFrameSample::Command(protocol::AppCommand::Hello {
            channel_version: protocol::CHANNEL_VERSION,
            app_id: "app-1".to_string(),
            actor: "actor-1".to_string(),
            config: vec![1, 2, 3],
        }));
        assert_channel_frame_round_trip(&ChannelFrameSample::Frame(protocol::AppFrame::Welcome { channel_version: protocol::CHANNEL_VERSION, instance: 1, manifest: vec![1, 2] }));
        assert_channel_frame_round_trip(&ChannelFrameSample::Frame(protocol::AppFrame::Error { in_reply_to: None, code: "e".to_string(), message: "m".to_string() }));
    }
    //#endregion 🔖️Laws (continued)

    //#region 🔖️Corrupt
    #[test]
    fn fuzz_truncation_never_panics_history_reader_open() {
        let log = HistoryLogGen::new(23).generate(&typical_profile());
        let bytes = write_history_log(&log, true);
        let report = fuzz_truncation(&bytes, CorruptionLevel::Quick, |candidate| {
            protocol::HistoryReader::open(candidate, &protocol::DecodeOptions::default()).and_then(|reader| reader.log()).map(|_| ()).map_err(|error| error.to_string())
        });
        assert!(report.cases_panicked.is_empty(), "HistoryReader::open must never panic on a truncated buffer: {:?}", report.cases_panicked);
    }

    #[test]
    fn fuzz_bit_flips_never_panics_history_reader_open() {
        let log = HistoryLogGen::new(24).generate(&typical_profile());
        let bytes = write_history_log(&log, true);
        let report = fuzz_bit_flips(&bytes, CorruptionLevel::Quick, |candidate| {
            protocol::HistoryReader::open(candidate, &protocol::DecodeOptions::default()).and_then(|reader| reader.log()).map(|_| ()).map_err(|error| error.to_string())
        });
        assert!(report.cases_panicked.is_empty(), "HistoryReader::open must never panic on a bit-flipped buffer: {:?}", report.cases_panicked);
    }

    #[test]
    fn fuzz_truncation_never_panics_recover() {
        let log = HistoryLogGen::new(25).generate(&typical_profile());
        let bytes = write_history_log(&log, true);
        let limits = protocol::ProtocolLimits::default();
        let report = fuzz_truncation(&bytes, CorruptionLevel::Quick, |candidate| {
            protocol_format::recover(&candidate, &limits, protocol::RecoveryMode::LastCommit).map(|_| ()).map_err(|error| error.to_string())
        });
        assert!(report.cases_panicked.is_empty(), "protocol_format::recover must never panic on a truncated buffer: {:?}", report.cases_panicked);
    }
    //#endregion 🔖️Corrupt

    //#region 🔖️Golden
    #[test]
    fn golden_hash_hex_is_deterministic_and_hex_encoded() {
        let a = golden_hash_hex(b"protocol_testkit golden fixture");
        let b = golden_hash_hex(b"protocol_testkit golden fixture");
        assert_eq!(a, b);
        assert_eq!(a.len(), 64);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
    }
    //#endregion 🔖️Golden
}
//#endregion 🧪️Tests
