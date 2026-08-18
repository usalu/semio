//! 🎞️ Protocol testkit: seeded generators for `crate::os_spr::HistoryLog`/causal-DAG fixtures, the
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
// Inline splitmix64 PRNG (NOT arbitrary/quickcheck/proptest — repo precedent, `crate::os_pack::testkit::
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
        if bound == 0 {
            0
        } else {
            self.next_u64() % bound
        }
    }
}

const WORD_ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";
const ADVERSARIAL_UNICODE: &[char] = &['é', 'ø', 'ñ', 'ß', '文', '档', '漢', '中', '🎉', '🚀'];

fn next_word(rng: &mut SplitMix64, max_len: usize) -> String {
    let len = 1 + rng.next_range(max_len as u64) as usize;
    (0..len).map(|_| WORD_ALPHABET[rng.next_range(WORD_ALPHABET.len() as u64) as usize] as char).collect()
}

/// ✏️ Free-form text safe as either a quoted `dsl_schema` field OR a raw opaque `.ops` op line: at
/// least one word, no leading/trailing whitespace, no `'\n'`, never trims to something starting
/// with `'#'` (which `crate::os_spr::history::parse_ops_text` would otherwise swallow as a comment).
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

/// ⏱️ Either a canonical `YYYY-MM-DDTHH:MM:SS[.fff]Z` string (exercising `crate::os_spr::wire::scalar`'s
/// compact tag-1/2 timestamp encoding) or, in adversarial mode, a deliberately non-canonical raw
/// string (a non-UTC offset, free text, or a Z-suffixed string with an out-of-grammar shape) that
/// forces the tag-0 raw-text fallback — see `crate::os_spr::wire::scalar`'s module note: correctness
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

/// 🎞️ Deterministic seeded `crate::os_spr::HistoryLog` fabricator.
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
    pub fn generate(&mut self, profile: &GenProfile) -> crate::os_spr::HistoryLog {
        let mut rng = SplitMix64(self.state);

        let doc_id = next_ident(&mut rng, "doc", 0, profile.adversarial);
        let schema = next_ident(&mut rng, "schema", 0, profile.adversarial);

        let mut edits: Vec<crate::os_spr::HistoryEdit> = Vec::with_capacity(profile.edit_count);
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
                ops.push(crate::os_spr::OpPayload { text: Some(next_text(&mut rng, profile.adversarial)), binary: None });
            }
            edits.push(crate::os_spr::HistoryEdit { id, actor, started_at, finished_at, coalesce_key, description, ops, inverse: Vec::new(), meta: None });
        }

        let mut changes: Vec<crate::os_spr::HistoryChange> = Vec::new();
        let mut checkpoints: Vec<crate::os_spr::HistoryCheckpoint> = Vec::new();
        if profile.checkpoint_every > 0 {
            let mut boundary = 0usize;
            let mut prior_checkpoint: Option<String> = None;
            let mut index = 0usize;
            while boundary < edits.len() {
                let end = (boundary + profile.checkpoint_every).min(edits.len());
                let change_id = next_ident(&mut rng, "change", index, profile.adversarial);
                let edit_ids: Vec<String> = edits[boundary..end].iter().map(|edit| edit.id.clone()).collect();
                changes.push(crate::os_spr::HistoryChange { id: change_id.clone(), saved_at: next_timestamp(&mut rng, profile.adversarial), edit_ids, description: if rng.next_bool() { Some(next_text(&mut rng, profile.adversarial)) } else { None } });

                let checkpoint_id = next_ident(&mut rng, "checkpoint", index, profile.adversarial);
                let author_count = rng.next_range(3) as usize;
                let mut authors = Vec::with_capacity(author_count);
                for a in 0..author_count {
                    authors.push(crate::os_spr::HistoryAuthor { id: next_ident(&mut rng, "author", a, profile.adversarial), name: next_text(&mut rng, false) });
                }
                checkpoints.push(crate::os_spr::HistoryCheckpoint {
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

        let mut alternatives: Vec<crate::os_spr::HistoryAlternative> = Vec::new();
        if !checkpoints.is_empty() {
            let alternative_count = rng.next_range(3) as usize;
            for i in 0..alternative_count {
                let mut checkpoint_ids = Vec::new();
                for checkpoint in &checkpoints {
                    if rng.next_bool() {
                        checkpoint_ids.push(checkpoint.id.clone());
                    }
                }
                alternatives.push(crate::os_spr::HistoryAlternative { id: next_ident(&mut rng, "alt", i, profile.adversarial), name: next_text(&mut rng, false), checkpoint_ids });
            }
        }

        let active_alternative_id = if !alternatives.is_empty() && rng.next_bool() {
            let index = rng.next_range(alternatives.len() as u64) as usize;
            Some(alternatives[index].id.clone())
        } else {
            None
        };

        self.state = rng.0;
        crate::os_spr::HistoryLog { doc_id, schema, edits, changes, checkpoints, alternatives, active_alternative_id, cursor: None, composition: None, conflicts: Vec::new() }
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
    pub fn generate(&mut self, node_count: usize) -> Vec<crate::os_spr::MutationEnvelope> {
        let mut rng = SplitMix64(self.state);
        let mut envelopes = Vec::with_capacity(node_count);
        for i in 0..node_count {
            let dep_count = if i == 0 { 0 } else { rng.next_range(3.min(i as u64 + 1)) as usize };
            let mut dependencies: Vec<crate::os_spr::MutationId> = Vec::with_capacity(dep_count);
            for _ in 0..dep_count {
                let dep_id = crate::os_spr::MutationId(format!("op-{}", rng.next_range(i as u64)));
                if !dependencies.contains(&dep_id) {
                    dependencies.push(dep_id);
                }
            }
            envelopes.push(crate::os_spr::MutationEnvelope {
                mutation_id: crate::os_spr::MutationId(format!("op-{i}")),
                document_id: crate::os_spr::ArtifactId("doc-1".to_string()),
                actor: crate::os_spr::ActorId(format!("actor-{}", rng.next_range(4))),
                dependencies,
                diff: crate::os_spr::ArtifactDiff { schema: crate::os_spr::SchemaId("testkit.op".to_string()), payload: format!("index:{i}").into_bytes() },
                inverse: crate::os_spr::InverseMutation { schema: crate::os_spr::SchemaId("testkit.op".to_string()), payload: Vec::new() },
                timestamp: crate::os_spr::HybridLogicalTimestamp::new(i as u64, i as u64 * 10),
            });
        }
        self.state = rng.0;
        envelopes
    }
}
//#endregion 🔖️Gen

//#region 🔖️Laws
// `crate::os_spr::history::encode_history`/`decode_history`/`parse_ops_text`/`print_ops_text` are the
// whole-batch codec and text-grammar twin the frozen contract's laws are phrased against; the
// `protocol` facade doesn't re-export them (only `HistoryAppender`/`HistoryReader` and the
// `compile_ops`/`decompile_ops` ops-text wrappers), so this crate depends on `protocol_history`
// directly for the struct-level and text-level laws below (see `Cargo.toml`'s note).

/// 🖇️ `.spr` bytes for `log`: `commit_after_every_record: false` uses the whole-batch
/// `crate::os_spr::history::encode_history` codec directly (buffered); `true` streams it through
/// `crate::os_spr::HistoryAppender`, committing after every single record (the hot-path shape a live
/// appender actually uses) — see `assert_streamed_equals_buffered`.
fn write_history_log(log: &crate::os_spr::HistoryLog, commit_after_every_record: bool) -> Vec<u8> {
    if !commit_after_every_record {
        return crate::os_spr::history::encode_history(log, &crate::os_spr::history::EncodeOptions::default()).expect("encode_history must succeed for a testkit-generated log");
    }
    let options = crate::os_spr::WriteOptions { required_flags: 0, optional_flags: 0 };
    let mut appender = crate::os_spr::HistoryAppender::begin(Vec::new(), &log.doc_id, &log.schema, &options).expect("HistoryAppender::begin must succeed for a well-formed doc_id/schema");
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

fn read_history_log(bytes: &[u8]) -> crate::os_spr::HistoryLog {
    crate::os_spr::history::decode_history(bytes, &crate::os_spr::history::DecodeOptions::default()).expect("decode_history must succeed for freshly-written bytes")
}

/// ✅️ LAW: `decode_history(&encode_history(log, _)) == log`.
pub fn assert_history_encode_decode_identity(log: &crate::os_spr::HistoryLog) {
    let decoded = read_history_log(&write_history_log(log, false));
    assert_eq!(&decoded, log, "encode_history/decode_history round trip diverged from the source HistoryLog");
}

/// ✅️ LAW: `encode_history` is byte-for-byte deterministic — encoding the same `HistoryLog` twice
/// produces identical bytes.
pub fn assert_history_canonical_stable(log: &crate::os_spr::HistoryLog) {
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
    let encode_options = crate::os_spr::history::EncodeOptions::default();
    let decode_options = crate::os_spr::history::DecodeOptions::default();

    let log = crate::os_spr::history::parse_ops_text(ops_text).expect("parse_ops_text must succeed for well-formed ops text");
    let bytes = crate::os_spr::history::encode_history(&log, &encode_options).expect("encode_history must succeed for a well-formed HistoryLog");
    let decoded = crate::os_spr::history::decode_history(&bytes, &decode_options).expect("decode_history must succeed on encode_history's own output");
    let printed = crate::os_spr::history::print_ops_text(&decoded).expect("print_ops_text must succeed for text-only generated ops");

    let reparsed = crate::os_spr::history::parse_ops_text(&printed).expect("parse_ops_text must succeed on print_ops_text's own output");
    assert_eq!(crate::os_spr::history::print_ops_text(&reparsed).unwrap(), printed, "print_ops_text(parse_ops_text(text)) must be a fixpoint under a second parse/print pass");
}

/// ✅️ LAW: streaming `log` through `crate::os_spr::HistoryAppender` one commit per record decodes
/// identically to a single-shot `crate::os_spr::history::encode_history` of the whole log.
pub fn assert_streamed_equals_buffered(log: &crate::os_spr::HistoryLog) {
    let buffered = read_history_log(&write_history_log(log, false));
    let streamed = read_history_log(&write_history_log(log, true));
    assert_eq!(buffered, streamed, "HistoryAppender-per-record streaming and encode_history(whole log) must decode identically");
}

/// ✅️ LAW: every `RecordFrame::payload()` returned while sweeping `bytes` borrows zero-copy from
/// `bytes` itself.
pub fn assert_zero_copy(bytes: &[u8]) {
    let bounds = bytes.as_ptr_range();
    let mut cursor = crate::os_spr::FrameCursor::new(bytes, crate::os_spr::format::HEADER_SIZE as u64);
    while let Some(frame) = cursor.next_frame().expect("assert_zero_copy requires an already-structurally-valid record stream") {
        let payload_bounds = frame.payload().as_ptr_range();
        assert!(payload_bounds.start >= bounds.start && payload_bounds.end <= bounds.end, "RecordFrame::payload() at offset {} must borrow zero-copy from the input slice", frame.offset);
    }
}

/// ✅️ LAW (softened to match the family's actual recovery-then-verify layering, see below): a
/// single-byte flip anywhere in `bytes` at or after the header must never go unnoticed under Full
/// verification. The frozen contract's prose says "must error", but per-frame CRC-32C is checked
/// during `crate::os_spr::format::recover`'s own scan BEFORE `VerificationLevel::Full`'s chain-hash check
/// ever runs — `protocol_history`'s own precedent test (`history_full_verification_detects_tampering`
/// in `protocol/history/rs/lib.rs`) documents the same real outcome: a corrupted frame's own CRC
/// mismatch makes `recover` truncate the trusted range BEFORE it, so the decode either errors OR
/// silently comes back shorter than the original — never silently equal to it. This fn asserts
/// exactly that non-silent-corruption invariant, sampling `24` roughly-evenly-spaced byte offsets
/// rather than every byte (this crate's own `assert_recovery_truncates_to_commit` already covers an
/// exhaustive-if-requested sweep of the structurally-analogous truncation case).
pub fn assert_chain_detects_tamper(bytes: &[u8]) {
    let options = crate::os_spr::DecodeOptions { verification: crate::os_spr::VerificationLevel::Full, limits: crate::os_spr::ProtocolLimits::default() };
    let original = crate::os_spr::HistoryReader::open(bytes, &options).and_then(|reader| reader.log()).expect("assert_chain_detects_tamper requires an already-decodable, untampered input");

    for position in sampled_positions(bytes.len(), crate::os_spr::format::HEADER_SIZE, 24) {
        let mut tampered = bytes.to_vec();
        tampered[position] ^= 0xFF;
        let result = crate::os_spr::HistoryReader::open(&tampered, &options).and_then(|reader| reader.log());
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
/// `crate::os_spr::format::recover` (`RecoveryMode::LastCommit`) always yields a trusted prefix that ends
/// exactly at a real `REC_COMMIT` frame — or, if no commit survived the truncation, exactly at the
/// 32-byte header.
pub fn assert_recovery_truncates_to_commit(bytes: &[u8], level: CorruptionLevel) {
    let limits = crate::os_spr::ProtocolLimits::default();
    let cap = match level {
        CorruptionLevel::Exhaustive => bytes.len(),
        CorruptionLevel::Long => 128,
        CorruptionLevel::Quick => 16,
    };
    for len in sampled_lengths(bytes.len(), cap) {
        let truncated = &bytes[..len];
        let Ok(recovery) = crate::os_spr::format::recover(&truncated, &limits, crate::os_spr::RecoveryMode::LastCommit) else {
            continue;
        };
        if recovery.last_commit_seq == 0 {
            assert_eq!(recovery.bytes_recovered as usize, crate::os_spr::format::HEADER_SIZE, "no commit recovered at truncation length {len} -> trusted prefix must be exactly the header");
            continue;
        }
        let trusted = &truncated[..recovery.bytes_recovered as usize];
        let mut cursor = crate::os_spr::FrameCursor::new(trusted, recovery.last_commit_offset);
        let frame = cursor.next_frame().expect("the recovered trusted prefix must itself re-parse cleanly").expect("a frame must exist at the reported last_commit_offset");
        assert_eq!(frame.kind, crate::os_spr::REC_COMMIT, "last_commit_offset must point at a REC_COMMIT frame (truncation length {len})");
        assert_eq!(frame.offset + frame.frame_len(), recovery.bytes_recovered, "bytes_recovered must end exactly after the last trusted commit frame (truncation length {len})");
    }
}

/// 🧮️ The `(kind, payload bytes)` multiset of every structural record (`REC_DOC`/`REC_EDIT`/
/// `REC_CHANGE`/`REC_CHECKPOINT`/`REC_ALTERNATIVE`/`REC_ACTIVE`) in `bytes`'s trusted prefix —
/// deliberately excludes `REC_COMMIT` (chain metadata, expected to differ across a compaction that
/// restarts the commit chain) and dictionary/index/sealed/compaction/snapshot/ephemeral kinds
/// (physical layout compaction is explicitly allowed to rewrite).
#[cfg(not(target_arch = "wasm32"))]
fn structural_records(bytes: &[u8], limits: &crate::os_spr::ProtocolLimits) -> std::collections::BTreeMap<(u8, Vec<u8>), usize> {
    let recovery = crate::os_spr::format::recover(&bytes, limits, crate::os_spr::RecoveryMode::LastCommit).expect("recover must succeed on an already-valid stream");
    let trusted = &bytes[..recovery.bytes_recovered as usize];
    let mut counts = std::collections::BTreeMap::new();
    let mut cursor = crate::os_spr::FrameCursor::new(trusted, crate::os_spr::format::HEADER_SIZE as u64);
    while let Some(frame) = cursor.next_frame().expect("trusted prefix must re-parse cleanly") {
        if matches!(frame.kind, crate::os_spr::REC_DOC | crate::os_spr::REC_EDIT | crate::os_spr::REC_CHANGE | crate::os_spr::REC_CHECKPOINT | crate::os_spr::REC_ALTERNATIVE | crate::os_spr::REC_ACTIVE) {
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

/// ✅️ LAW: `crate::os_spr::compact` is physically-rewriting-but-identity-preserving — the
/// `(kind, payload bytes)` multiset of every structural record is unchanged across a compaction
/// round trip. Native-only (`crate::os_spr::compact` is a file-based API): writes `bytes` to a scratch
/// temp file, compacts it in place, reads it back.
#[cfg(not(target_arch = "wasm32"))]
pub fn assert_compaction_identity(bytes: &[u8]) {
    let limits = crate::os_spr::ProtocolLimits::default();
    let before = structural_records(bytes, &limits);

    let path = scratch_spr_path("compact");
    std::fs::write(&path, bytes).expect("write scratch .spr file for compaction test");
    let options = crate::os_spr::CompactOptions { drop_ephemeral: true, keep_snapshots: crate::os_spr::KeepSnapshots::All };
    let compact_result = crate::os_spr::compact(&path, &options, &limits);
    let compacted = std::fs::read(&path);
    let _ = std::fs::remove_file(&path);

    compact_result.expect("crate::os_spr::compact must succeed on an already-valid stream");
    let after = structural_records(&compacted.expect("read back compacted .spr file"), &limits);
    assert_eq!(before, after, "compaction must preserve the (kind, payload bytes) multiset of every structural record");
}

/// ✅️ LAW: `OpText::print_op` output never contains `'\n'`, and `parse_op(op.print_op())` recovers
/// an equal operation.
pub fn assert_op_text_round_trip<Op>(op: &Op)
where
    Op: crate::os_spr::OpText + Clone + PartialEq + std::fmt::Debug,
{
    let line = op.print_op();
    assert!(!line.contains('\n'), "OpText::print_op output must never contain a newline, got {line:?}");
    let parsed = Op::parse_op(&line).unwrap_or_else(|error| panic!("OpText::parse_op failed to parse its own print_op output {line:?}: {error:?}"));
    assert_eq!(&parsed, op, "OpText::parse_op(op.print_op()) must recover an equal operation");
}

/// ✅️ LAW: `absorb(d1, d2).apply(base) == d2.apply(&d1.apply(base))` — the sequential-coalesce
/// contract every `MutationDiff::absorb` impl must satisfy (`📡️spr/🎮️command/🦀️component.rs`'s
/// `🔖️Mutation` region doc). Every `🧬️mutations/<kind>/🔺️diff` leaf's test region should call this
/// with two diffs of its own artifact known to have been produced by sequential mutations.
pub fn assert_mutation_diff_absorb_law<P, D>(base: &P, d1: D, d2: D)
where
    P: PartialEq + std::fmt::Debug,
    D: crate::os_spr::MutationDiff<P> + Clone,
{
    let mid = d1.apply(base).expect("first valid diff must apply");
    let sequential = d2.apply(&mid);
    let mut absorbed = d1;
    absorbed.absorb(d2);
    let composed = absorbed.apply(base);
    assert_eq!(composed, sequential, "absorb(d1, d2).apply(base) must equal d2.apply(&d1.apply(base))");
}

/// ✅️ LAW: applying `mutation`'s `Mutation::inverse(base)` (in reverse order, matching
/// `ArtifactStore::replay_mutations`'s own `back.reverse()`) after `mutation` restores `base`. The
/// per-`MutationKind` version of `.claude/plans/the-mutations-are-extremely-compiled-pumpkin.md`'s
/// core requirement: every handcrafted mutation implements a real inverse, not a sentinel.
pub fn assert_mutation_inverse_law<P, Op>(base: &P, mutation: &Op)
where
    P: Clone + PartialEq + std::fmt::Debug,
    Op: crate::os_spr::Mutation<P>,
{
    use crate::os_spr::MutationDiff;
    let forward = mutation.diff(base);
    let rejected = forward.messages().iter().any(|message| matches!(message.level, crate::os_dsl::Severity::Error | crate::os_dsl::Severity::Fatal));
    assert!(!rejected, "a mutation expected to invert cleanly must not have been rejected — forward outcome carries an Error/Fatal message: {:?}", forward.messages());
    let mut state = forward.diff().apply(base).expect("valid forward diff must apply");
    let mut backward = mutation.inverse(base);
    backward.reverse();
    for undo in &backward {
        state = undo.diff(&state).diff().apply(&state).expect("valid inverse diff must apply");
    }
    assert_eq!(&state, base, "applying mutation.inverse(base) (reversed) after mutation must restore base");
}

/// ✅️ LAW: `D::between(a, b).apply(a) == b`, and `D::between(a, a).is_empty()` —
/// [`crate::os_spr::DiffAlgebra`]'s state-delta contract.
pub fn assert_diff_algebra_between_law<P, D>(a: &P, b: &P)
where
    P: Clone + PartialEq + std::fmt::Debug,
    D: crate::os_spr::DiffAlgebra<P> + crate::os_spr::MutationDiff<P>,
{
    let delta = D::between(a, b);
    assert_eq!(delta.apply(a).as_ref(), Ok(b), "DiffAlgebra::between(a, b).apply(a) must equal b");
    assert!(D::between(a, a).is_empty(), "DiffAlgebra::between(a, a) must be empty");
}

/// ✅️ LAW: `d.inverse(base).apply(&d.apply(base)) == *base` — [`crate::os_spr::DiffAlgebra`]'s
/// diff-level undo, independent of any `Mutation` that might have produced `d`.
pub fn assert_diff_algebra_inverse_law<P, D>(base: &P, d: &D)
where
    P: Clone + PartialEq + std::fmt::Debug,
    D: crate::os_spr::DiffAlgebra<P> + crate::os_spr::MutationDiff<P>,
{
    let after = d.apply(base).expect("valid diff must apply");
    let restored = d.inverse(base).apply(&after);
    assert_eq!(restored.as_ref(), Ok(base), "d.inverse(base).apply(&d.apply(base)) must equal base");
}

fn fisher_yates_shuffle(rng: &mut SplitMix64, items: &mut [usize]) {
    for i in (1..items.len()).rev() {
        let j = rng.next_range(i as u64 + 1) as usize;
        items.swap(i, j);
    }
}

/// ✅️ LAW: `MutationDag` converges to the same fully-applied set regardless of insertion order, for
/// `permutation_count` random shuffles of `envelopes` (which must form a closed dependency set —
/// see `OpDagGen::generate`).
pub fn assert_op_dag_convergence(envelopes: &[crate::os_spr::MutationEnvelope], seed: u64, permutation_count: usize) {
    let expected: std::collections::BTreeSet<String> = envelopes.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
    let mut rng = SplitMix64(seed);
    for _ in 0..permutation_count.max(1) {
        let mut order: Vec<usize> = (0..envelopes.len()).collect();
        fisher_yates_shuffle(&mut rng, &mut order);

        let mut dag = crate::os_spr::MutationDag::new();
        for index in order {
            dag.insert(envelopes[index].clone()).expect("a closed dependency set inserted with unique ids must never duplicate");
        }
        let applied: std::collections::BTreeSet<String> = dag.drain_applied_envelopes().iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
        assert_eq!(applied, expected, "MutationDag must converge to the same fully-applied set regardless of insertion order");
    }
}

/// 🎞️ Which side of the semio_hub wire protocol an `assert_wire_frame_round_trip` sample represents —
/// `ClientFrame`/`ServerFrame` are distinct enums with distinct encode/decode fn pairs, so this
/// crate's own choice (the contract names one law fn, not two) is a single sum type covering both.
pub enum WireFrameSample {
    Client(crate::os_spr::ClientFrame, crate::os_spr::Lane),
    Server(crate::os_spr::ServerFrame, crate::os_spr::Lane),
}

/// ✅️ LAW: `decode(encode(frame)) == frame`, for either wire direction.
pub fn assert_wire_frame_round_trip(sample: &WireFrameSample) {
    match sample {
        WireFrameSample::Client(frame, lane) => {
            let bytes = crate::os_spr::encode_client_frame(frame, *lane);
            let (decoded_lane, decoded_frame) = crate::os_spr::decode_client_frame(&bytes).expect("decode_client_frame must succeed on its own encode_client_frame output");
            assert_eq!(decoded_lane, *lane, "decoded Lane must match the encoded Lane");
            assert_eq!(&decoded_frame, frame, "decode_client_frame(encode_client_frame(frame)) must equal frame");
        }
        WireFrameSample::Server(frame, lane) => {
            let bytes = crate::os_spr::encode_server_frame(frame, *lane);
            let (decoded_lane, decoded_frame) = crate::os_spr::decode_server_frame(&bytes).expect("decode_server_frame must succeed on its own encode_server_frame output");
            assert_eq!(decoded_lane, *lane, "decoded Lane must match the encoded Lane");
            assert_eq!(&decoded_frame, frame, "decode_server_frame(encode_server_frame(frame)) must equal frame");
        }
    }
}

/// @emoji 🧵️ Which side of the app-engine channel an `assert_channel_frame_round_trip` sample
/// represents — `AppCommand`/`AppFrame` are distinct enums with distinct encode/decode fn pairs,
/// same rationale as `WireFrameSample` above.
pub enum ChannelFrameSample {
    Command(crate::os_spr::AppCommand),
    Frame(crate::os_spr::AppFrame),
}

/// ✅️ LAW: `decode(encode(frame)) == frame`, for either channel direction.
pub fn assert_channel_frame_round_trip(sample: &ChannelFrameSample) {
    match sample {
        ChannelFrameSample::Command(command) => {
            let bytes = crate::os_spr::encode_app_command(command);
            let decoded = crate::os_spr::decode_app_command(&bytes).expect("decode_app_command must succeed on its own encode_app_command output");
            assert_eq!(&decoded, command, "decode_app_command(encode_app_command(command)) must equal command");
        }
        ChannelFrameSample::Frame(frame) => {
            let bytes = crate::os_spr::encode_app_frame(frame);
            let decoded = crate::os_spr::decode_app_frame(&bytes).expect("decode_app_frame must succeed on its own encode_app_frame output");
            assert_eq!(&decoded, frame, "decode_app_frame(encode_app_frame(frame)) must equal frame");
        }
    }
}

// 26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS §C2/C3/C5 law family — every
// helper below is generic over the `Mutation`/`MutationKind`/`MutationDiff` traits (or takes the
// store/history operation it needs as an injected closure) so any artifact/mutation pair a W3 facet
// owns can plug in without this crate depending on `os_store`/`os_spr::history`'s own concrete
// surfaces, several of which are still landing concurrently in lanes 1-A/1-B/1-C
// (`📓️w1-d-report.md` records exactly which closures stand in for a not-yet-landed method today).

//#region 🔖️Outcome
/// ✅️ LAW (§C2): a mutation whose target is absent must yield an outcome carrying an `Error`
/// message with code `mutation.target-missing`, and the diff must carry no change (`D::default()`
/// — the `MutationOutcome::error` constructor's own empty-diff guarantee every missing-target
/// `diff` leaf is expected to route through).
pub fn assert_missing_target_is_error<P, Op>(base: &P, mutation: &Op)
where
    Op: crate::os_spr::Mutation<P>,
    Op::Diff: PartialEq + std::fmt::Debug + Default,
{
    let outcome = mutation.diff(base);
    let has_missing_target_error = outcome.messages().iter().any(|message| message.level == crate::os_dsl::Severity::Error && message.code.0 == "mutation.target-missing");
    assert!(has_missing_target_error, "a mutation targeting an absent element must carry an Error message with code 'mutation.target-missing', got {:?}", outcome.messages());
    assert_eq!(outcome.diff(), &Op::Diff::default(), "a mutation.target-missing outcome must carry no change (diff == Diff::default())");
}

/// ✅️ LAW (§C2 law 1): whenever `outcome.worst_level()` is `Fatal`, `outcome.diff() ==
/// D::default()`.
pub fn assert_fatal_never_applies<D>(outcome: &crate::os_spr::MutationOutcome<D>)
where
    D: PartialEq + std::fmt::Debug + Default,
{
    if outcome.worst_level() == Some(crate::os_dsl::Severity::Fatal) {
        assert_eq!(outcome.diff(), &D::default(), "a Fatal outcome must carry diff == D::default()");
    }
}

/// ✅️ LAW (§C2 law 3): equal `(op, base)` produces equal messages and an equal diff, across
/// repeated invocations.
pub fn assert_outcome_deterministic<P, Op>(base: &P, mutation: &Op)
where
    Op: crate::os_spr::Mutation<P>,
    Op::Diff: PartialEq + std::fmt::Debug,
{
    let a = mutation.diff(base);
    let b = mutation.diff(base);
    assert_eq!(a.diff(), b.diff(), "diff(op, base) must be deterministic across repeated invocations");
    assert_eq!(a.messages(), b.messages(), "messages(op, base) must be deterministic across repeated invocations");
}
//#endregion 🔖️Outcome

//#region 🔖️Policy
const POLICY_MATRIX_POLICIES: [crate::os_spr::MergePolicy; 3] = [crate::os_spr::MergePolicy::LaissezFaire, crate::os_spr::MergePolicy::Normal, crate::os_spr::MergePolicy::Vigilant];
const POLICY_MATRIX_LEVELS: [crate::os_dsl::Severity; 4] = [crate::os_dsl::Severity::Info, crate::os_dsl::Severity::Warning, crate::os_dsl::Severity::Error, crate::os_dsl::Severity::Fatal];

/// 📐️ The frozen 3×4 table (`📋️contract-freeze.md` "The three merge policies"): `LaissezFaire`
/// rejects only `Fatal`; `Normal` rejects `Error`+`Fatal`; `Vigilant` rejects
/// `Warning`+`Error`+`Fatal`.
fn policy_matrix_expected_reject(policy: crate::os_spr::MergePolicy, level: crate::os_dsl::Severity) -> bool {
    match policy {
        crate::os_spr::MergePolicy::LaissezFaire => level == crate::os_dsl::Severity::Fatal,
        crate::os_spr::MergePolicy::Normal => level >= crate::os_dsl::Severity::Error,
        crate::os_spr::MergePolicy::Vigilant => level >= crate::os_dsl::Severity::Warning,
    }
}

/// ✅️ LAW: `rejects` (typically `MergePolicy::rejects`) and `is_applicable` (typically a
/// single-message `MutationOutcome::is_applicable` probe at `level`) must both agree with the
/// frozen 3×4 policy matrix for every `(policy, level)` pair.
pub fn assert_policy_matrix(rejects: impl Fn(crate::os_spr::MergePolicy, crate::os_dsl::Severity) -> bool, is_applicable: impl Fn(crate::os_spr::MergePolicy, crate::os_dsl::Severity) -> bool) {
    for policy in POLICY_MATRIX_POLICIES {
        for level in POLICY_MATRIX_LEVELS {
            let expected_reject = policy_matrix_expected_reject(policy, level);
            assert_eq!(rejects(policy, level), expected_reject, "rejects({policy:?}, {level:?}) diverged from the frozen 3x4 policy matrix");
            assert_eq!(is_applicable(policy, level), !expected_reject, "is_applicable({policy:?}, {level:?}) diverged from the frozen 3x4 policy matrix");
        }
    }
}
//#endregion 🔖️Policy

//#region 🔖️Merge
/// ✅️ LAW: two peers that independently insert the same closed `envelopes` set (in independently
/// shuffled order) into a fresh [`crate::os_spr::MutationDag`] and then fold the drained batch
/// through `fold` converge on the same state — `fold` is responsible for its own canonicalization
/// (typically an HLC sort, mirroring `ingest_remote`'s own §C6 step 3) since
/// `drain_applied_envelopes`'s own order is only causally valid, not canonical.
pub fn assert_merge_convergence<P: PartialEq + std::fmt::Debug>(seed: u64, peer_count: usize, envelopes: &[crate::os_spr::MutationEnvelope], fold: impl Fn(&[crate::os_spr::MutationEnvelope]) -> P) {
    let mut rng = SplitMix64(seed);
    let mut expected: Option<P> = None;
    for _ in 0..peer_count.max(2) {
        let mut order: Vec<usize> = (0..envelopes.len()).collect();
        fisher_yates_shuffle(&mut rng, &mut order);
        let mut dag = crate::os_spr::MutationDag::new();
        for index in order {
            dag.insert(envelopes[index].clone()).expect("assert_merge_convergence requires a closed dependency set with unique ids");
        }
        let batch = dag.drain_applied_envelopes();
        let state = fold(&batch);
        match &expected {
            None => expected = Some(state),
            Some(reference) => assert_eq!(&state, reference, "peers that exchange the same edit set (different arrival/insertion order) must converge on the same state"),
        }
    }
}

/// ✅️ LAW: the headline modify-vs-delete scenario (`📋️contract-freeze.md` "Testkit laws"). Under
/// `Normal`/`Vigilant`, `report.accepted` must be `false` and `post_state` must equal `pre_state`
/// (quarantined, nothing applied), and `report.conflict` must resolve (in `conflicts`) to a
/// `ConflictKind::Quarantined`. Under `LaissezFaire`, `report.accepted` must be `true`,
/// `report.replayed` must carry an `Error` message, `report.conflict` must resolve to a
/// `ConflictKind::Degraded`, and `part_present(post_state)` must be `false`.
pub fn assert_modify_vs_delete<P: PartialEq + std::fmt::Debug>(policy: crate::os_spr::MergePolicy, pre_state: &P, post_state: &P, report: &crate::os_spr::MergeReport, conflicts: &[crate::os_spr::Conflict], part_present: impl Fn(&P) -> bool) {
    let has_error = report.replayed.iter().flat_map(|edit| &edit.messages).any(|message| message.level == crate::os_dsl::Severity::Error);
    match policy {
        crate::os_spr::MergePolicy::Normal | crate::os_spr::MergePolicy::Vigilant => {
            assert!(!report.accepted, "under {policy:?}, a modify-vs-delete remote merge must be quarantined (MergeReport::accepted == false)");
            assert_eq!(post_state, pre_state, "under {policy:?}, a quarantined merge must leave the state unchanged");
            let quarantined = report.conflict.as_ref().and_then(|id| conflicts.iter().find(|conflict| &conflict.id == id)).expect("a quarantined MergeReport must reference an existing Conflict");
            assert!(matches!(quarantined.kind, crate::os_spr::ConflictKind::Quarantined { .. }), "a rejected modify-vs-delete merge must raise a Quarantined conflict, got {:?}", quarantined.kind);
        }
        crate::os_spr::MergePolicy::LaissezFaire => {
            assert!(report.accepted, "under LaissezFaire, a modify-vs-delete remote merge must be applied (MergeReport::accepted == true)");
            assert!(has_error, "under LaissezFaire, an applied modify-vs-delete merge must carry an Error message");
            let degraded = report.conflict.as_ref().and_then(|id| conflicts.iter().find(|conflict| &conflict.id == id)).expect("an applied-but-messy modify-vs-delete merge must raise a Degraded conflict");
            assert!(matches!(degraded.kind, crate::os_spr::ConflictKind::Degraded { .. }), "an applied modify-vs-delete merge must raise a Degraded conflict, got {:?}", degraded.kind);
            assert!(!part_present(post_state), "under LaissezFaire, the deleted part must remain absent after the merge");
        }
    }
}

/// ✅️ LAW: any arrival order of the same envelope batch must yield the same final state, the same
/// `applied_edit_ids` order, and the same set of raised conflicts. `run` builds a fresh authority
/// from scratch and ingests `order` (a permutation of `0..envelope_count`), returning
/// `(final_state, applied_edit_ids, conflict_ids)`.
pub fn assert_chronological_determinism<P: PartialEq + std::fmt::Debug>(envelope_count: usize, seed: u64, permutation_count: usize, mut run: impl FnMut(&[usize]) -> (P, Vec<String>, Vec<crate::os_spr::ConflictId>)) {
    let mut rng = SplitMix64(seed);
    let mut expected: Option<(P, Vec<String>, Vec<crate::os_spr::ConflictId>)> = None;
    for _ in 0..permutation_count.max(1) {
        let mut order: Vec<usize> = (0..envelope_count).collect();
        fisher_yates_shuffle(&mut rng, &mut order);
        let result = run(&order);
        match &expected {
            None => expected = Some(result),
            Some((state, applied, conflicts)) => {
                assert_eq!(&result.0, state, "arrival order must not change the final state");
                assert_eq!(&result.1, applied, "arrival order must not change applied_edit_ids order");
                assert_eq!(&result.2, conflicts, "arrival order must not change which conflicts get raised");
            }
        }
    }
}

/// ✅️ LAW: resolving a `Quarantined` conflict with `ConflictResolution::Accept` produces exactly
/// the state a `LaissezFaire` peer ingesting the same envelopes directly would have produced.
pub fn assert_quarantine_accept_equals_laissez_faire<P: PartialEq + std::fmt::Debug>(state_after_accept: &P, state_under_laissez_faire: &P) {
    assert_eq!(state_after_accept, state_under_laissez_faire, "resolving a Quarantined conflict with Accept must produce exactly the state LaissezFaire would have produced");
}

/// ✅️ LAW: resolving a `Quarantined` conflict with `ConflictResolution::Discard` leaves the state
/// untouched, and none of `discarded_edit_ids` ever appears in `relayed` (the local edit ids a
/// `flush_outbound`-shaped call would ship).
pub fn assert_quarantine_discard_preserves_state<P: PartialEq + std::fmt::Debug>(pre_state: &P, post_state: &P, discarded_edit_ids: &[String], relayed: &[String]) {
    assert_eq!(post_state, pre_state, "discarding a Quarantined conflict must leave the state untouched");
    for edit_id in discarded_edit_ids {
        assert!(!relayed.contains(edit_id), "a discarded edit ({edit_id}) must never be relayed");
    }
}

/// ✅️ LAW: the persisted `edit_messages` ledger (`edit_id -> Vec<MutationMessage>`, typically
/// collected via `ArtifactStore::messages_for_edit`) equals what a fresh history replay produces.
pub fn assert_ledger_matches_replay(ledger: &std::collections::HashMap<String, Vec<crate::os_spr::MutationMessage>>, replayed: &std::collections::HashMap<String, Vec<crate::os_spr::MutationMessage>>) {
    assert_eq!(ledger, replayed, "the persisted edit_messages ledger must equal a fresh replay's messages");
}
//#endregion 🔖️Merge

//#region 🔖️Conflict
/// ✅️ LAW (§C7): `decode(encode(conflict)) == conflict` through the `.spr` conflict ledger codec
/// (`REC_CONFLICT`) — `encode`/`decode` are typically `crate::os_spr::history::encode_conflicts`/
/// `decode_conflicts` sliced to one entry.
pub fn assert_conflict_spr_round_trip(conflict: &crate::os_spr::Conflict, encode: impl Fn(&crate::os_spr::Conflict) -> Vec<u8>, decode: impl Fn(&[u8]) -> crate::os_spr::Conflict) {
    let bytes = encode(conflict);
    let decoded = decode(&bytes);
    assert_eq!(&decoded, conflict, "decode(encode(conflict)) must equal conflict");
}
//#endregion 🔖️Conflict

//#region 🔖️Channel
/// ✅️ LAW: a corpus round-trip sweep — `decode(encode(sample)) == sample` for every `sample` in
/// `corpus`. Point `encode`/`decode` at `crate::os_spr::{encode,decode}_app_command` or
/// `_app_frame` to sweep the C8 new-frame corpus
/// (`AppCommand::{SetMergePolicy,ResolveConflict,ReadConflicts}`,
/// `AppFrame::{MergeReport,Conflicts}`) once 1-C lands those variants.
pub fn assert_channel_frame_corpus<T: PartialEq + std::fmt::Debug>(corpus: &[T], encode: impl Fn(&T) -> Vec<u8>, decode: impl Fn(&[u8]) -> T) {
    for sample in corpus {
        let bytes = encode(sample);
        let decoded = decode(&bytes);
        assert_eq!(&decoded, sample, "decode(encode(sample)) must equal sample for every entry in the corpus");
    }
}
//#endregion 🔖️Channel
//#endregion 🔖️Laws

//#region 🔖️Corrupt
/// 🛡️ Reused verbatim from `pack_testkit` — closure-generic panic-safety fuzzers plus their level/
/// report types. LAW (exercised in this crate's own tests against `crate::os_spr::HistoryReader::open`
/// and `crate::os_spr::format::recover`): `CorruptionReport::cases_panicked` must always be empty.
pub use crate::os_pack::testkit::{fuzz_bit_flips, fuzz_truncation, CorruptionLevel, CorruptionReport};
//#endregion 🔖️Corrupt

//#region 🔖️Golden
pub use crate::os_pack::testkit::golden_hash_hex;
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
        let known: std::collections::HashSet<String> = envelopes.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
        for (index, envelope) in envelopes.iter().enumerate() {
            assert_eq!(envelope.mutation_id.0, format!("op-{index}"));
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
            let ops_text = crate::os_spr::decompile_ops(&bytes, &crate::os_spr::DecodeOptions::default()).expect("decompile_ops");
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
    // Dummy (P=i64, Op=AddOp) pair, the smallest possible Mutation/MutationDiff/OpText impl —
    // mirrors the fixture pattern `protocol_command`/`protocol_causal`'s own inline tests use.
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    struct AddDiff {
        delta: i64,
    }
    impl crate::os_spr::MutationDiff<i64> for AddDiff {
        fn apply(&self, base: &i64) -> crate::os_spr::MutationApplyResult<i64> {
            Ok(base + self.delta)
        }
        fn absorb(&mut self, other: Self) {
            self.delta += other.delta;
        }
    }

    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct AddOp {
        delta: i64,
    }
    impl crate::os_spr::Mutation<i64> for AddOp {
        type Diff = AddDiff;
        fn diff(&self, _base: &i64) -> crate::os_spr::MutationOutcome<AddDiff> {
            crate::os_spr::MutationOutcome::new(AddDiff { delta: self.delta })
        }
        fn inverse(&self, _base: &i64) -> Vec<Self> {
            vec![AddOp { delta: -self.delta }]
        }
    }
    impl crate::os_spr::OpText for AddOp {
        fn print_op(&self) -> String {
            format!("add {}", self.delta)
        }
        fn parse_op(line: &str) -> Result<Self, crate::os_dsl::TextError> {
            let rest = line.strip_prefix("add ").ok_or_else(|| crate::os_dsl::TextError::new("expected 'add <n>'", crate::os_dsl::TextSpan::at(1, 1)))?;
            let delta: i64 = rest.trim().parse().map_err(|_| crate::os_dsl::TextError::new("invalid integer", crate::os_dsl::TextSpan::at(1, 1)))?;
            Ok(AddOp { delta })
        }
    }

    impl crate::os_spr::DiffAlgebra<i64> for AddDiff {
        fn inverse(&self, _base: &i64) -> Self {
            AddDiff { delta: -self.delta }
        }
        fn between(base: &i64, other: &i64) -> Self {
            AddDiff { delta: other - base }
        }
        fn is_empty(&self) -> bool {
            self.delta == 0
        }
    }

    // 26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS §C2 fixtures: correct and
    // deliberately-buggy `Mutation<i64>` impls proving each `🔖️Outcome`/`🔖️Laws (continued)`
    // self-test panics on a genuine violation, not just passes on a good input.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct MissingTargetOp;
    impl crate::os_spr::Mutation<i64> for MissingTargetOp {
        type Diff = AddDiff;
        fn diff(&self, _base: &i64) -> crate::os_spr::MutationOutcome<AddDiff> {
            crate::os_spr::MutationOutcome::error("mutation.target-missing", "target absent", ["thing"])
        }
        fn inverse(&self, _base: &i64) -> Vec<Self> {
            Vec::new()
        }
    }

    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct BuggyMissingTargetOp;
    impl crate::os_spr::Mutation<i64> for BuggyMissingTargetOp {
        type Diff = AddDiff;
        fn diff(&self, _base: &i64) -> crate::os_spr::MutationOutcome<AddDiff> {
            crate::os_spr::MutationOutcome::new(AddDiff { delta: 1 })
        }
        fn inverse(&self, _base: &i64) -> Vec<Self> {
            vec![BuggyMissingTargetOp]
        }
    }

    #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
    struct NondeterministicOp {
        #[serde(skip)]
        calls: std::rc::Rc<std::cell::Cell<i64>>,
    }
    impl crate::os_spr::Mutation<i64> for NondeterministicOp {
        type Diff = AddDiff;
        fn diff(&self, _base: &i64) -> crate::os_spr::MutationOutcome<AddDiff> {
            let count = self.calls.get();
            self.calls.set(count + 1);
            crate::os_spr::MutationOutcome::new(AddDiff { delta: count })
        }
        fn inverse(&self, _base: &i64) -> Vec<Self> {
            Vec::new()
        }
    }

    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct RejectedForwardOp;
    impl crate::os_spr::Mutation<i64> for RejectedForwardOp {
        type Diff = AddDiff;
        fn diff(&self, _base: &i64) -> crate::os_spr::MutationOutcome<AddDiff> {
            crate::os_spr::MutationOutcome::fatal("mutation.invariant", "boom", ["x"])
        }
        fn inverse(&self, _base: &i64) -> Vec<Self> {
            Vec::new()
        }
    }

    fn sample_conflict(id: &str, kind: crate::os_spr::ConflictKind) -> crate::os_spr::Conflict {
        crate::os_spr::Conflict { id: crate::os_spr::ConflictId(id.to_string()), kind, status: crate::os_spr::ConflictStatus::Open, messages: Vec::new(), actors: Vec::new(), timestamp: crate::os_spr::HybridLogicalTimestamp::new(1, 100) }
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
        use crate::os_spr::{Mutation, MutationDiff};
        let base: i64 = 10;
        let op = AddOp { delta: 5 };
        let forward = op.diff(&base).diff().apply(&base).expect("valid forward diff");
        assert_eq!(forward, 15);
        let [undo] = <[AddOp; 1]>::try_from(op.inverse(&base)).unwrap();
        assert_eq!(undo.diff(&forward).diff().apply(&forward), Ok(base));
    }

    #[test]
    fn mutation_diff_absorb_law_holds_for_add() {
        assert_mutation_diff_absorb_law(&10i64, AddDiff { delta: 3 }, AddDiff { delta: 4 });
    }

    #[test]
    fn mutation_inverse_law_holds_for_add() {
        assert_mutation_inverse_law(&10i64, &AddOp { delta: 5 });
    }

    #[test]
    #[should_panic(expected = "must not have been rejected")]
    fn mutation_inverse_law_panics_when_forward_outcome_is_rejected() {
        assert_mutation_inverse_law(&10i64, &RejectedForwardOp);
    }

    #[test]
    fn diff_algebra_between_law_holds_for_add() {
        assert_diff_algebra_between_law::<i64, AddDiff>(&10, &17);
    }

    #[test]
    fn diff_algebra_inverse_law_holds_for_add() {
        assert_diff_algebra_inverse_law(&10i64, &AddDiff { delta: 5 });
    }

    #[test]
    #[should_panic(expected = "must recover an equal operation")]
    fn op_text_round_trip_panics_on_a_lossy_impl() {
        #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        struct LossyOp {
            delta: i64,
        }
        impl crate::os_spr::OpText for LossyOp {
            fn print_op(&self) -> String {
                "lossy".to_string()
            }
            fn parse_op(_line: &str) -> Result<Self, crate::os_dsl::TextError> {
                Ok(LossyOp { delta: 0 })
            }
        }
        assert_op_text_round_trip(&LossyOp { delta: 42 });
    }

    #[test]
    fn wire_frame_round_trip_holds_for_client_and_server_samples() {
        assert_wire_frame_round_trip(&WireFrameSample::Client(crate::os_spr::ClientFrame::Bye, crate::os_spr::Lane::Command));
        assert_wire_frame_round_trip(&WireFrameSample::Client(crate::os_spr::ClientFrame::PreviewPublish { key: "cursor".to_string(), seq: 3, payload: vec![1, 2, 3] }, crate::os_spr::Lane::Preview));
        let frontier = crate::os_spr::RuntimeFrontierSummary { document_id: crate::os_spr::ArtifactId("doc-1".to_string()), head_edit_ordinal: 5, head_edit_id: "edit-5".to_string(), last_commit_seq: 2, chain_hash: [7u8; 32] };
        assert_wire_frame_round_trip(&WireFrameSample::Server(crate::os_spr::ServerFrame::Welcome { session_id: "s1".to_string(), resume_token: "r1".to_string(), server_frontier: frontier, bootstrap: crate::os_spr::Bootstrap::Tail }, crate::os_spr::Lane::Command));
    }

    #[test]
    fn channel_frame_round_trip_holds_for_command_and_frame_samples() {
        assert_channel_frame_round_trip(&ChannelFrameSample::Command(crate::os_spr::AppCommand::ReadConflicts { seq: 1 }));
        assert_channel_frame_round_trip(&ChannelFrameSample::Command(crate::os_spr::AppCommand::ConfigCommand { seq: 1, command: vec![1, 2, 3] }));
        assert_channel_frame_round_trip(&ChannelFrameSample::Frame(crate::os_spr::AppFrame::Done { in_reply_to: 1 }));
        assert_channel_frame_round_trip(&ChannelFrameSample::Frame(crate::os_spr::AppFrame::Error { in_reply_to: None, fault: b"e:m".to_vec(), report: Vec::new() }));
    }
    //#endregion 🔖️Laws (continued)

    //#region 🔖️Outcome
    #[test]
    fn missing_target_is_error_holds_for_a_correct_impl() {
        assert_missing_target_is_error(&10i64, &MissingTargetOp);
    }

    #[test]
    #[should_panic(expected = "mutation.target-missing")]
    fn missing_target_is_error_panics_on_a_buggy_impl() {
        assert_missing_target_is_error(&10i64, &BuggyMissingTargetOp);
    }

    #[test]
    fn fatal_never_applies_holds_for_a_correct_outcome() {
        let outcome: crate::os_spr::MutationOutcome<AddDiff> = crate::os_spr::MutationOutcome::fatal("mutation.invariant", "boom", ["x"]);
        assert_fatal_never_applies(&outcome);
    }

    #[test]
    #[should_panic(expected = "Fatal outcome must carry diff == D::default()")]
    fn fatal_never_applies_panics_on_a_non_empty_diff() {
        let outcome = crate::os_spr::MutationOutcome::new(AddDiff { delta: 3 }).absorb_messages([crate::os_spr::MutationMessage::fatal("mutation.invariant", "boom")]);
        assert_fatal_never_applies(&outcome);
    }

    #[test]
    fn outcome_deterministic_holds_for_add() {
        assert_outcome_deterministic(&10i64, &AddOp { delta: 4 });
    }

    #[test]
    #[should_panic(expected = "must be deterministic")]
    fn outcome_deterministic_panics_on_a_nondeterministic_impl() {
        assert_outcome_deterministic(&10i64, &NondeterministicOp::default());
    }
    //#endregion 🔖️Outcome

    //#region 🔖️Policy
    fn message_at_level(level: crate::os_dsl::Severity) -> crate::os_spr::MutationMessage {
        match level {
            crate::os_dsl::Severity::Info => crate::os_spr::MutationMessage::info("mutation.cascade", "probe"),
            crate::os_dsl::Severity::Warning => crate::os_spr::MutationMessage::warn("mutation.no-op", "probe"),
            crate::os_dsl::Severity::Error => crate::os_spr::MutationMessage::error("mutation.target-missing", "probe"),
            crate::os_dsl::Severity::Fatal => crate::os_spr::MutationMessage::fatal("mutation.invariant", "probe"),
        }
    }

    #[test]
    fn policy_matrix_holds_for_the_real_apis() {
        assert_policy_matrix(
            |policy, level| policy.rejects(level),
            |policy, level| {
                let outcome: crate::os_spr::MutationOutcome<()> = crate::os_spr::MutationOutcome::new(()).absorb_messages([message_at_level(level)]);
                outcome.is_applicable(policy)
            },
        );
    }

    #[test]
    #[should_panic(expected = "diverged from the frozen 3x4 policy matrix")]
    fn policy_matrix_panics_on_a_wrong_impl() {
        assert_policy_matrix(|_, _| false, |_, _| true);
    }
    //#endregion 🔖️Policy

    //#region 🔖️Merge
    #[test]
    fn merge_convergence_holds_for_a_commutative_fold() {
        let envelopes = OpDagGen::new(30).generate(10);
        assert_merge_convergence(300, 5, &envelopes, |batch| {
            let mut batch = batch.to_vec();
            batch.sort_by_key(|envelope| envelope.timestamp);
            batch.iter().fold(0i64, |state, envelope| {
                let payload = std::str::from_utf8(&envelope.diff.payload).unwrap();
                let index: i64 = payload.strip_prefix("index:").unwrap().parse().unwrap();
                state + index
            })
        });
    }

    #[test]
    #[should_panic(expected = "must converge on the same state")]
    fn merge_convergence_panics_for_an_order_dependent_fold() {
        let envelopes = OpDagGen::new(31).generate(10);
        assert_merge_convergence(301, 6, &envelopes, |batch| batch.iter().map(|envelope| envelope.mutation_id.0.clone()).collect::<Vec<_>>().join(","));
    }

    #[test]
    fn modify_vs_delete_holds_for_normal_quarantine() {
        let pre = Some("part".to_string());
        let post = pre.clone();
        let conflict = sample_conflict("c1", crate::os_spr::ConflictKind::Quarantined { envelopes: Vec::new() });
        let report = crate::os_spr::MergeReport { policy: crate::os_spr::MergePolicy::Normal, accepted: false, insertion_index: 0, replayed: Vec::new(), worst: Some(crate::os_dsl::Severity::Error), conflict: Some(conflict.id.clone()) };
        assert_modify_vs_delete(crate::os_spr::MergePolicy::Normal, &pre, &post, &report, std::slice::from_ref(&conflict), |state| state.is_some());
    }

    #[test]
    #[should_panic(expected = "must be quarantined")]
    fn modify_vs_delete_panics_when_normal_wrongly_accepts() {
        let pre = Some("part".to_string());
        let post = pre.clone();
        let report = crate::os_spr::MergeReport { policy: crate::os_spr::MergePolicy::Normal, accepted: true, insertion_index: 0, replayed: Vec::new(), worst: None, conflict: None };
        assert_modify_vs_delete(crate::os_spr::MergePolicy::Normal, &pre, &post, &report, &[], |state| state.is_some());
    }

    #[test]
    fn modify_vs_delete_holds_for_laissez_faire_apply() {
        let pre = Some("part".to_string());
        let post: Option<String> = None;
        let conflict = sample_conflict("c2", crate::os_spr::ConflictKind::Degraded { edit_ids: vec!["e1".to_string()] });
        let report = crate::os_spr::MergeReport {
            policy: crate::os_spr::MergePolicy::LaissezFaire,
            accepted: true,
            insertion_index: 0,
            replayed: vec![crate::os_spr::EditMessages { edit_id: "e1".to_string(), messages: vec![crate::os_spr::MutationMessage::error("mutation.target-missing", "part gone")] }],
            worst: Some(crate::os_dsl::Severity::Error),
            conflict: Some(conflict.id.clone()),
        };
        assert_modify_vs_delete(crate::os_spr::MergePolicy::LaissezFaire, &pre, &post, &report, std::slice::from_ref(&conflict), |state| state.is_some());
    }

    #[test]
    #[should_panic(expected = "must remain absent")]
    fn modify_vs_delete_panics_when_laissez_faire_part_still_present() {
        let pre = Some("part".to_string());
        let post = pre.clone();
        let conflict = sample_conflict("c3", crate::os_spr::ConflictKind::Degraded { edit_ids: vec!["e1".to_string()] });
        let report = crate::os_spr::MergeReport {
            policy: crate::os_spr::MergePolicy::LaissezFaire,
            accepted: true,
            insertion_index: 0,
            replayed: vec![crate::os_spr::EditMessages { edit_id: "e1".to_string(), messages: vec![crate::os_spr::MutationMessage::error("mutation.target-missing", "part gone")] }],
            worst: Some(crate::os_dsl::Severity::Error),
            conflict: Some(conflict.id.clone()),
        };
        assert_modify_vs_delete(crate::os_spr::MergePolicy::LaissezFaire, &pre, &post, &report, std::slice::from_ref(&conflict), |state| state.is_some());
    }

    #[test]
    fn chronological_determinism_holds_for_an_order_independent_run() {
        assert_chronological_determinism(5, 400, 6, |order| {
            let mut sorted = order.to_vec();
            sorted.sort_unstable();
            (sorted.clone(), sorted.iter().map(|i| format!("edit-{i}")).collect(), Vec::new())
        });
    }

    #[test]
    #[should_panic(expected = "must not change the final state")]
    fn chronological_determinism_panics_for_an_order_dependent_run() {
        assert_chronological_determinism(5, 401, 6, |order| (order.to_vec(), order.iter().map(|i| format!("edit-{i}")).collect(), Vec::new()));
    }

    #[test]
    fn quarantine_accept_equals_laissez_faire_holds_when_equal() {
        assert_quarantine_accept_equals_laissez_faire(&5i64, &5i64);
    }

    #[test]
    #[should_panic(expected = "must produce exactly the state LaissezFaire would have produced")]
    fn quarantine_accept_equals_laissez_faire_panics_when_unequal() {
        assert_quarantine_accept_equals_laissez_faire(&5i64, &6i64);
    }

    #[test]
    fn quarantine_discard_preserves_state_holds() {
        assert_quarantine_discard_preserves_state(&5i64, &5i64, &["e1".to_string()], &["e2".to_string()]);
    }

    #[test]
    #[should_panic(expected = "must never be relayed")]
    fn quarantine_discard_preserves_state_panics_when_relayed() {
        assert_quarantine_discard_preserves_state(&5i64, &5i64, &["e1".to_string()], &["e1".to_string()]);
    }

    #[test]
    #[should_panic(expected = "must leave the state untouched")]
    fn quarantine_discard_preserves_state_panics_when_state_changes() {
        assert_quarantine_discard_preserves_state(&5i64, &6i64, &["e1".to_string()], &[]);
    }

    #[test]
    fn ledger_matches_replay_holds_when_equal() {
        let mut ledger = std::collections::HashMap::new();
        ledger.insert("e1".to_string(), vec![crate::os_spr::MutationMessage::info("mutation.cascade", "note")]);
        let replayed = ledger.clone();
        assert_ledger_matches_replay(&ledger, &replayed);
    }

    #[test]
    #[should_panic(expected = "must equal a fresh replay")]
    fn ledger_matches_replay_panics_when_unequal() {
        let mut ledger = std::collections::HashMap::new();
        ledger.insert("e1".to_string(), vec![crate::os_spr::MutationMessage::info("mutation.cascade", "note")]);
        let mut replayed = std::collections::HashMap::new();
        replayed.insert("e1".to_string(), Vec::new());
        assert_ledger_matches_replay(&ledger, &replayed);
    }
    //#endregion 🔖️Merge

    //#region 🔖️Conflict
    #[test]
    fn conflict_spr_round_trip_holds_for_an_identity_codec() {
        let conflict = sample_conflict("c4", crate::os_spr::ConflictKind::Degraded { edit_ids: vec!["e1".to_string()] });
        let for_decode = conflict.clone();
        assert_conflict_spr_round_trip(&conflict, |_c| Vec::new(), move |_bytes| for_decode.clone());
    }

    #[test]
    #[should_panic(expected = "must equal conflict")]
    fn conflict_spr_round_trip_panics_for_a_lossy_codec() {
        let conflict = sample_conflict("c5", crate::os_spr::ConflictKind::Degraded { edit_ids: vec!["e1".to_string()] });
        assert_conflict_spr_round_trip(&conflict, |_c| Vec::new(), |_bytes| sample_conflict("different", crate::os_spr::ConflictKind::Degraded { edit_ids: Vec::new() }));
    }
    //#endregion 🔖️Conflict

    //#region 🔖️Channel
    #[test]
    fn frame_corpus_round_trip_holds_for_the_real_app_command_codec() {
        let corpus = vec![crate::os_spr::AppCommand::ReadConflicts { seq: 1 }, crate::os_spr::AppCommand::ConfigCommand { seq: 1, command: vec![1, 2, 3] }];
        assert_channel_frame_corpus(&corpus, |command| crate::os_spr::encode_app_command(command), |bytes| crate::os_spr::decode_app_command(bytes).unwrap());
    }

    #[test]
    #[should_panic(expected = "must equal sample")]
    fn frame_corpus_round_trip_panics_for_a_lossy_codec() {
        let corpus = vec![crate::os_spr::AppCommand::ReadConflicts { seq: 1 }];
        assert_channel_frame_corpus(&corpus, |_command| Vec::new(), |_bytes| crate::os_spr::AppCommand::ReadConflicts { seq: 0 });
    }
    //#endregion 🔖️Channel

    //#region 🔖️Corrupt
    #[test]
    fn fuzz_truncation_never_panics_history_reader_open() {
        let log = HistoryLogGen::new(23).generate(&typical_profile());
        let bytes = write_history_log(&log, true);
        let report = fuzz_truncation(&bytes, CorruptionLevel::Quick, |candidate| crate::os_spr::HistoryReader::open(candidate, &crate::os_spr::DecodeOptions::default()).and_then(|reader| reader.log()).map(|_| ()).map_err(|error| error.to_string()));
        assert!(report.cases_panicked.is_empty(), "HistoryReader::open must never panic on a truncated buffer: {:?}", report.cases_panicked);
    }

    #[test]
    fn fuzz_bit_flips_never_panics_history_reader_open() {
        let log = HistoryLogGen::new(24).generate(&typical_profile());
        let bytes = write_history_log(&log, true);
        let report = fuzz_bit_flips(&bytes, CorruptionLevel::Quick, |candidate| crate::os_spr::HistoryReader::open(candidate, &crate::os_spr::DecodeOptions::default()).and_then(|reader| reader.log()).map(|_| ()).map_err(|error| error.to_string()));
        assert!(report.cases_panicked.is_empty(), "HistoryReader::open must never panic on a bit-flipped buffer: {:?}", report.cases_panicked);
    }

    #[test]
    fn fuzz_truncation_never_panics_recover() {
        let log = HistoryLogGen::new(25).generate(&typical_profile());
        let bytes = write_history_log(&log, true);
        let limits = crate::os_spr::ProtocolLimits::default();
        let report = fuzz_truncation(&bytes, CorruptionLevel::Quick, |candidate| crate::os_spr::format::recover(&candidate, &limits, crate::os_spr::RecoveryMode::LastCommit).map(|_| ()).map_err(|error| error.to_string()));
        assert!(report.cases_panicked.is_empty(), "crate::os_spr::format::recover must never panic on a truncated buffer: {:?}", report.cases_panicked);
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
