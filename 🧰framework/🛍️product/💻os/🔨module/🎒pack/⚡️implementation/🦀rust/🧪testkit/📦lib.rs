//! 📦 `pack_testkit` — shared test infrastructure for the whole `pack_*` crate family: a
//! deterministic seeded `RecordValueGen` that fabricates `dsl_schema::RecordValue`s from any
//! `RecordSpec`, the cross-crate round-trip/determinism/preservation LAWS every encoder/decoder
//! pair must satisfy, a panic-safe truncation/bit-flip corruption harness, and a golden-hash
//! helper for committing expected byte-content as a text constant.
//!
//! See the `## pack_testkit` section of the wave-0 contract at
//! `.🦑repo/🎫tickets/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/contract.md` for the binding
//! signatures this crate implements against. Deliberately depends on `dsl_schema`/`dsl_core`
//! directly (not just `pack`) rather than the `arbitrary`/`quickcheck` crates — the generator is
//! a small hand-rolled splitmix64 PRNG, and float generation round-trips through `dsl_core`'s own
//! canonical text form so generated values stay representable by `vcs`/`dsl_derive`'s future
//! DSL-bidirectional tests without this crate needing to depend on either of them.

use dsl_schema::{DslValue, FieldValue, RecordSpec, RecordValue, Shape, WireNode, WireValue};
use std::collections::HashMap;

//#region 🔖Arbitrary
/// @emoji 🎲 Deterministic seeded generator: splitmix64 state, advanced on every draw. NOT the
/// `arbitrary`/`quickcheck` crates — the same seed always produces the same sequence of
/// `RecordValue`s, which is what makes a failing corruption/round-trip test reproducible from a
/// single logged `u64`.
pub struct RecordValueGen {
    state: u64,
}

impl RecordValueGen {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// @emoji 🌱 Fabricates one `RecordValue` matching `spec`, recursing into nested
    /// `Record`/`Block`/`Statements`/`Table`/`Map`/`Value` shapes up to `max_depth` — beyond that,
    /// [`Self::shallow_value`] takes over so genuinely self-referential specs (a recursive
    /// `Statements` table whose own variant list names itself) terminate instead of looping
    /// forever.
    pub fn generate(&mut self, spec: &RecordSpec, max_depth: u16) -> RecordValue {
        self.generate_record(spec, 0, max_depth)
    }

    //#region 🔖Prng
    /// @emoji 🌀 splitmix64 — see <https://prng.di.unimi.it/splitmix64.c>. Small, dependency-free,
    /// good enough statistical spread for test-data generation (not cryptography).
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn next_bool(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }

    /// @emoji 🎯 Uniform-ish `[0, bound)`; `0` for `bound == 0` (modulo bias is irrelevant for
    /// test-data spread).
    fn next_range(&mut self, bound: u64) -> u64 {
        if bound == 0 { 0 } else { self.next_u64() % bound }
    }

    fn next_int(&mut self) -> i64 {
        self.next_range(2001) as i64 - 1000
    }

    fn next_uint(&mut self) -> u64 {
        self.next_range(1_000_000)
    }

    /// @emoji 🔢 Deliberately never NaN/Infinity — `FieldValue`'s derived `PartialEq` uses `==`,
    /// under which `NaN != NaN` always, so a generated NaN would make
    /// `assert_encode_decode_identity` fail even on a perfectly correct codec. Round-trips through
    /// `dsl_core::format_f64`/`parse_f64` so every generated float is also exactly
    /// DSL-representable — load-bearing for any future `assert_dsl_pack_bidirectional` caller that
    /// seeds its sample from this generator.
    fn next_f64(&mut self) -> f64 {
        let magnitude = self.next_range(1_000_000) as f64 / 100.0;
        let sign = if self.next_bool() { -1.0 } else { 1.0 };
        let raw = sign * magnitude;
        dsl_core::parse_f64(&dsl_core::format_f64(raw)).unwrap_or(raw)
    }

    const ALPHABET: &'static [u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_ ";

    fn next_string(&mut self, max_len: usize) -> String {
        let len = self.next_range(max_len as u64 + 1) as usize;
        (0..len)
            .map(|_| {
                let idx = self.next_range(Self::ALPHABET.len() as u64) as usize;
                Self::ALPHABET[idx] as char
            })
            .collect()
    }

    fn next_bytes(&mut self, max_len: usize) -> Vec<u8> {
        let len = self.next_range(max_len as u64 + 1) as usize;
        (0..len).map(|_| (self.next_u64() & 0xFF) as u8).collect()
    }
    //#endregion 🔖Prng

    //#region 🔖Shapes
    fn generate_record(&mut self, spec: &RecordSpec, depth: u16, max_depth: u16) -> RecordValue {
        let mut fields = HashMap::with_capacity(spec.fields.len());
        for field in &spec.fields {
            let value = if field.optional && self.next_range(4) == 0 { FieldValue::Absent } else { self.generate_value(&field.shape, depth, max_depth) };
            fields.insert(field.id, value);
        }
        RecordValue { fields }
    }

    /// @emoji ✏️ One field value matching `shape`, self-describing enough that the pack codec
    /// always accepts it. `depth > max_depth` defers to [`Self::shallow_value`] — the recursion
    /// backstop for lazy `fn() -> RecordSpec` shapes (`Record`/`Statements`/`Table`) that could
    /// otherwise recurse forever on a self-referential grammar.
    fn generate_value(&mut self, shape: &Shape, depth: u16, max_depth: u16) -> FieldValue {
        if depth > max_depth {
            return self.shallow_value(shape);
        }
        match shape {
            Shape::Bool => FieldValue::Bool(self.next_bool()),
            Shape::Int => FieldValue::Int(self.next_int()),
            Shape::UInt => FieldValue::UInt(self.next_uint()),
            Shape::Float => FieldValue::Float(self.next_f64()),
            Shape::Text => FieldValue::Text(self.next_string(12)),
            Shape::Bytes64 => FieldValue::Bytes64(self.next_bytes(16)),
            Shape::Enum(variants) => {
                if variants.is_empty() {
                    FieldValue::Enum(0)
                } else {
                    let idx = self.next_range(variants.len() as u64) as usize;
                    FieldValue::Enum(variants[idx].1)
                }
            }
            Shape::Tuple(elem, len) => {
                let n = len.unwrap_or_else(|| 1 + self.next_range(3) as usize);
                let items = (0..n).map(|_| self.generate_value(elem, depth + 1, max_depth)).collect();
                FieldValue::Tuple(items)
            }
            Shape::List(elem) => {
                let n = self.next_range(4) as usize;
                let items = (0..n).map(|_| self.generate_value(elem, depth + 1, max_depth)).collect();
                FieldValue::List(items)
            }
            Shape::Record(spec_fn) => FieldValue::Record(self.generate_record(&spec_fn(), depth + 1, max_depth)),
            Shape::Block(inner) => FieldValue::Block(Box::new(self.generate_value(inner, depth + 1, max_depth))),
            Shape::Statements(variants) => {
                if variants.is_empty() {
                    FieldValue::Statements(Vec::new())
                } else {
                    let n = self.next_range(3) as usize;
                    let items = (0..n)
                        .map(|_| {
                            let idx = self.next_range(variants.len() as u64) as usize;
                            let (keyword, spec_fn) = &variants[idx];
                            (keyword.clone(), self.generate_record(&spec_fn(), depth + 1, max_depth))
                        })
                        .collect();
                    FieldValue::Statements(items)
                }
            }
            Shape::Map(inner) => {
                let n = self.next_range(3) as usize;
                let entries = (0..n).map(|_| (self.next_string(6), self.generate_value(inner, depth + 1, max_depth))).collect();
                FieldValue::Map(entries)
            }
            Shape::Value => FieldValue::Value(self.generate_dsl_value(depth + 1, max_depth)),
            Shape::Table(spec_fn) => {
                let row_spec = spec_fn();
                let n = self.next_range(3) as usize;
                let rows = (0..n).map(|_| FieldValue::Record(self.generate_record(&row_spec, depth + 1, max_depth))).collect();
                FieldValue::List(rows)
            }
            Shape::Wire => FieldValue::Wire(self.generate_wire(depth + 1, max_depth)),
        }
    }

    /// @emoji 🛑 Non-recursing terminal value for `shape`, used once `max_depth` is exhausted.
    /// Safe to call unconditionally: `Tuple`/`List`/`Map`/`Block` are structurally finite Rust
    /// values (no lazy indirection), so only `Record`/`Statements`/`Table` — the three genuinely
    /// self-referential shapes — need the empty/default fallback rather than real recursion.
    fn shallow_value(&mut self, shape: &Shape) -> FieldValue {
        match shape {
            Shape::Bool => FieldValue::Bool(self.next_bool()),
            Shape::Int => FieldValue::Int(self.next_int()),
            Shape::UInt => FieldValue::UInt(self.next_uint()),
            Shape::Float => FieldValue::Float(self.next_f64()),
            Shape::Text => FieldValue::Text(self.next_string(6)),
            Shape::Bytes64 => FieldValue::Bytes64(Vec::new()),
            Shape::Enum(variants) => FieldValue::Enum(variants.first().map(|(_, ordinal)| *ordinal).unwrap_or(0)),
            Shape::Tuple(_, _) => FieldValue::Tuple(Vec::new()),
            Shape::List(_) => FieldValue::List(Vec::new()),
            Shape::Record(_) => FieldValue::Record(RecordValue::default()),
            Shape::Block(inner) => FieldValue::Block(Box::new(self.shallow_value(inner))),
            Shape::Statements(_) => FieldValue::Statements(Vec::new()),
            Shape::Map(_) => FieldValue::Map(Vec::new()),
            Shape::Value => FieldValue::Value(DslValue::Null),
            Shape::Table(_) => FieldValue::List(Vec::new()),
            Shape::Wire => FieldValue::Wire(WireValue { from: WireNode { id: "n".to_string(), kind: None, port: None }, edge: None, properties: DslValue::Null }),
        }
    }

    fn generate_dsl_value(&mut self, depth: u16, max_depth: u16) -> DslValue {
        if depth > max_depth {
            return DslValue::Null;
        }
        match self.next_range(6) {
            0 => DslValue::Null,
            1 => DslValue::Bool(self.next_bool()),
            2 => DslValue::Number(self.next_f64()),
            3 => DslValue::String(self.next_string(8)),
            4 => {
                let n = self.next_range(3) as usize;
                DslValue::Array((0..n).map(|_| self.generate_dsl_value(depth + 1, max_depth)).collect())
            }
            _ => {
                let n = self.next_range(3) as usize;
                DslValue::Object((0..n).map(|_| (self.next_string(6), self.generate_dsl_value(depth + 1, max_depth))).collect())
            }
        }
    }

    fn generate_wire_node(&mut self) -> WireNode {
        let id = self.next_string(6);
        WireNode {
            id: if id.trim().is_empty() { "n".to_string() } else { id },
            kind: if self.next_bool() { Some(self.next_string(4)) } else { None },
            port: if self.next_bool() { Some(self.next_string(4)) } else { None },
        }
    }

    fn generate_wire(&mut self, depth: u16, max_depth: u16) -> WireValue {
        let from = self.generate_wire_node();
        let edge = if self.next_bool() { Some((self.next_bool(), self.generate_wire_node())) } else { None };
        let properties = self.generate_dsl_value(depth, max_depth);
        WireValue { from, edge, properties }
    }
    //#endregion 🔖Shapes
}
//#endregion 🔖Arbitrary

//#region 🔖Laws
/// @emoji 🧹 Strips `FieldValue::Absent` entries at every nesting level (the "pure-Absent noise"
/// `decode_document` reinserts for every spec field not found on the wire — canonical mode never
/// encodes `Absent`, so a freshly-decoded record always carries one for every spec field the
/// generator happened to skip) and sorts `Map` entries by key bytes (`encode_map` always sorts,
/// unconditionally, per `pack_value`'s purity LAW — a generator that inserted map entries in
/// non-canonical order would otherwise fail this comparison on ordering alone, not content).
/// Shared by every LAW below that compares an original `RecordValue` against a decoded one.
fn normalize_record(record: &RecordValue) -> RecordValue {
    let mut fields = HashMap::with_capacity(record.fields.len());
    for (id, value) in &record.fields {
        if matches!(value, FieldValue::Absent) {
            continue;
        }
        fields.insert(*id, normalize_value(value));
    }
    RecordValue { fields }
}

fn normalize_value(value: &FieldValue) -> FieldValue {
    match value {
        FieldValue::Record(r) => FieldValue::Record(normalize_record(r)),
        FieldValue::Tuple(items) => FieldValue::Tuple(items.iter().map(normalize_value).collect()),
        FieldValue::List(items) => FieldValue::List(items.iter().map(normalize_value).collect()),
        FieldValue::Block(inner) => FieldValue::Block(Box::new(normalize_value(inner))),
        FieldValue::Statements(items) => FieldValue::Statements(items.iter().map(|(k, r)| (k.clone(), normalize_record(r))).collect()),
        FieldValue::Map(entries) => {
            let mut sorted: Vec<(String, FieldValue)> = entries.iter().map(|(k, v)| (k.clone(), normalize_value(v))).collect();
            sorted.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
            FieldValue::Map(sorted)
        }
        other => other.clone(),
    }
}

/// @emoji 🔁 LAW: `decode_document(encode_document(spec, record)) == record`, modulo the
/// pure-Absent noise [`normalize_record`] strips.
pub fn assert_encode_decode_identity(spec: &RecordSpec, record: &RecordValue) {
    let options = pack::EncodeOptions::default();
    let bytes = pack::encode_document(spec, record, &options).expect("encode_document should succeed for a well-formed record");
    let (decoded, _report) =
        pack::decode_document(&bytes, spec, &pack::DecodeOptions::default()).expect("decode_document should succeed for a just-encoded pack file");
    assert_eq!(normalize_record(&decoded), normalize_record(record), "encode/decode round trip diverged (ignoring pure-Absent noise)");
}

/// @emoji 🧊 LAW: `encode_document` is a pure function of `(spec, record)` — byte-identical
/// output across repeated calls, regardless of `HashMap` iteration order inside `record.fields`.
pub fn assert_canonical_stable(spec: &RecordSpec, record: &RecordValue) {
    let options = pack::EncodeOptions::default();
    let a = pack::encode_document(spec, record, &options).expect("first encode_document call");
    let b = pack::encode_document(spec, record, &options).expect("second encode_document call");
    assert_eq!(a, b, "encode_document must be byte-identical across repeated calls (canonical determinism law)");
}

/// @emoji 🕳️ LAW: field ids present in `record_with_extra_fields` but absent from `spec` still
/// round-trip through the wire and are reported in `DecodeReport.unknown_field_ids` — the
/// mechanism that lets an older reader tolerate a newer writer's additive schema evolution.
pub fn assert_unknown_field_preserved(spec: &RecordSpec, record_with_extra_fields: &RecordValue, extra_ids: &[u16]) {
    let options = pack::EncodeOptions::default();
    let bytes = pack::encode_document(spec, record_with_extra_fields, &options).expect("encode_document with extra fields");
    let (decoded, report) = pack::decode_document(&bytes, spec, &pack::DecodeOptions::default()).expect("decode_document with extra fields");

    let mut expected_extra: Vec<u16> = extra_ids.to_vec();
    expected_extra.sort_unstable();
    let mut actual_extra: Vec<u16> = report.unknown_field_ids.clone();
    actual_extra.sort_unstable();
    assert_eq!(actual_extra, expected_extra, "DecodeReport.unknown_field_ids must exactly match the caller-declared extra field ids");

    for id in extra_ids {
        let expected_value = record_with_extra_fields.fields.get(id).expect("extra_ids must reference fields present in record_with_extra_fields");
        let actual_value = decoded.fields.get(id).expect("an unknown field must still be preserved in the decoded RecordValue");
        assert_eq!(normalize_value(actual_value), normalize_value(expected_value), "unknown field {id} must round-trip unchanged");
    }
}

/// @emoji 🌊 LAW: splitting a document's body across many small `Document` frames (a "streamed"
/// encode, `frame_size = 1`) decodes to the exact same `RecordValue` as encoding it as one large
/// frame (a "buffered" encode) — `decode_document` must reassemble frames transparently
/// regardless of how many the encoder chose to emit.
pub fn assert_streamed_equals_buffered(spec: &RecordSpec, record: &RecordValue) {
    let mut buffered_options = pack::EncodeOptions::default();
    buffered_options.frame_size = 8 * 1024 * 1024;
    let mut streamed_options = pack::EncodeOptions::default();
    streamed_options.frame_size = 1;

    let buffered_bytes = pack::encode_document(spec, record, &buffered_options).expect("buffered (single-frame) encode_document");
    let streamed_bytes = pack::encode_document(spec, record, &streamed_options).expect("streamed (many-frame) encode_document");

    let (buffered_decoded, _) = pack::decode_document(&buffered_bytes, spec, &pack::DecodeOptions::default()).expect("decode buffered encoding");
    let (streamed_decoded, _) = pack::decode_document(&streamed_bytes, spec, &pack::DecodeOptions::default()).expect("decode streamed encoding");

    assert_eq!(
        normalize_record(&buffered_decoded),
        normalize_record(&streamed_decoded),
        "single-frame and many-small-frame encodings of the same document must decode identically"
    );
}

/// @emoji 🔀 LAW: `decode_pack(encode_pack(sample)) == parse_dsl(print_dsl(sample)) == sample` —
/// the DSL text and pack binary encodings of the same value must agree with each other and with
/// the original. Kept generic over closures so this crate needs no dependency on `vcs`/
/// `dsl_derive`; their own `test_support` wraps this with concrete `P: DocumentDsl + DocumentPack`
/// bounds in wave 1.
pub fn assert_dsl_pack_bidirectional<P>(parse_dsl: impl Fn(&str) -> P, print_dsl: impl Fn(&P) -> String, encode_pack: impl Fn(&P) -> Vec<u8>, decode_pack: impl Fn(&[u8]) -> P, sample: &P)
where
    P: PartialEq + std::fmt::Debug,
{
    let printed = print_dsl(sample);
    let reparsed = parse_dsl(&printed);
    let encoded = encode_pack(sample);
    let decoded = decode_pack(&encoded);
    assert_eq!(&decoded, sample, "decode_pack(encode_pack(sample)) must equal sample");
    assert_eq!(&reparsed, sample, "parse_dsl(print_dsl(sample)) must equal sample");
    assert_eq!(decoded, reparsed, "decode_pack(encode_pack(sample)) must equal parse_dsl(print_dsl(sample))");
}
//#endregion 🔖Laws

//#region 🔖Corrupt
/// @emoji ⏱️ How exhaustively [`fuzz_truncation`]/[`fuzz_bit_flips`] sample the corruption space —
/// mirrors the repo-wide `quick`/`long`/`exhaustive` leveled-test convention.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CorruptionLevel {
    Quick,
    Long,
    Exhaustive,
}

/// @emoji 🩹 Outcome of a corruption-fuzz run. `cases_panicked` must be empty for a correct
/// decoder — a corrupted input is allowed to be rejected (`cases_errored`) or, rarely, to still
/// happen to decode (neither counted, since a coincidentally-valid truncation/bit-flip isn't a
/// bug), but it must never panic or abort the process.
#[derive(Clone, Debug, Default)]
pub struct CorruptionReport {
    pub cases_run: u64,
    pub cases_errored: u64,
    pub cases_panicked: Vec<String>,
}

/// @emoji 📐 Picks up to `cap` roughly-evenly-spaced indices from `[0, total)`, always including
/// the very first and last index once `total > cap`. Shared sampling core for both the
/// truncation-length and bit-flip-position candidate lists below.
fn sample_evenly(total: usize, cap: usize) -> Vec<usize> {
    if total == 0 {
        return Vec::new();
    }
    if total <= cap {
        return (0..total).collect();
    }
    let step = total as f64 / cap as f64;
    let mut out: Vec<usize> = (0..cap).map(|i| ((i as f64 * step) as usize).min(total - 1)).collect();
    out.sort_unstable();
    out.dedup();
    out
}

fn truncation_candidates(len: usize, level: CorruptionLevel) -> Vec<usize> {
    if len == 0 {
        return Vec::new();
    }
    match level {
        CorruptionLevel::Exhaustive => (0..len).collect(),
        CorruptionLevel::Long => sample_evenly(len, 128),
        CorruptionLevel::Quick => sample_evenly(len, 16),
    }
}

fn bit_flip_candidates(len: usize, level: CorruptionLevel) -> Vec<(usize, u8)> {
    if len == 0 {
        return Vec::new();
    }
    let total_bits = len * 8;
    let cap = match level {
        CorruptionLevel::Exhaustive => total_bits,
        CorruptionLevel::Long => 128,
        CorruptionLevel::Quick => 16,
    };
    sample_evenly(total_bits, cap).into_iter().map(|bit| (bit / 8, (bit % 8) as u8)).collect()
}

/// @emoji 💬 Best-effort human-readable message from a `catch_unwind` payload.
fn panic_payload_message(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "non-string panic payload".to_string()
    }
}

/// @emoji 🛡️ Runs `decode` over every `(label, bytes)` case inside `catch_unwind`, silencing the
/// default panic hook for the duration (a fuzz run intentionally trips dozens of panics when it
/// finds a bug; letting the default hook print each one to stderr would drown the actual test
/// output). Restores the previous hook before returning, including on an unexpected early return.
fn run_corruption_cases(cases: impl Iterator<Item = (String, Vec<u8>)>, decode: &impl Fn(&[u8]) -> Result<(), String>) -> CorruptionReport {
    let mut report = CorruptionReport::default();
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    for (label, bytes) in cases {
        report.cases_run += 1;
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| decode(&bytes)));
        match outcome {
            Ok(Ok(())) => {}
            Ok(Err(_)) => report.cases_errored += 1,
            Err(payload) => report.cases_panicked.push(format!("{label}: {}", panic_payload_message(payload.as_ref()))),
        }
    }
    std::panic::set_hook(previous_hook);
    report
}

/// @emoji ✂️ Truncates `valid_pack` at a sampled set of lengths (density per `level`) and calls
/// `decode` on each — proves a decoder never panics on a merely-shorter-than-expected input.
pub fn fuzz_truncation(valid_pack: &[u8], level: CorruptionLevel, decode: impl Fn(&[u8]) -> Result<(), String>) -> CorruptionReport {
    let lengths = truncation_candidates(valid_pack.len(), level);
    let cases = lengths.into_iter().map(|len| (format!("truncate_to_len_{len}"), valid_pack[..len].to_vec()));
    run_corruption_cases(cases, &decode)
}

/// @emoji 🔀 Flips one bit of `valid_pack` at a sampled set of byte/bit positions (density per
/// `level`) and calls `decode` on each — proves a decoder never panics on single-bit corruption
/// (the failure mode CRC/blake3 verification exists to catch, not to crash on).
pub fn fuzz_bit_flips(valid_pack: &[u8], level: CorruptionLevel, decode: impl Fn(&[u8]) -> Result<(), String>) -> CorruptionReport {
    let positions = bit_flip_candidates(valid_pack.len(), level);
    let cases = positions.into_iter().map(|(byte_idx, bit_idx)| {
        let mut corrupted = valid_pack.to_vec();
        corrupted[byte_idx] ^= 1 << bit_idx;
        (format!("flip_byte_{byte_idx}_bit_{bit_idx}"), corrupted)
    });
    run_corruption_cases(cases, &decode)
}
//#endregion 🔖Corrupt

//#region 🔖Golden
/// @emoji 🔑 `hex(blake3(bytes))` — for committing an expected pack encoding's hash as a text
/// constant in a caller's own test, so a future unintended encoding change is caught by a one-line
/// diff instead of a giant byte-literal.
pub fn golden_hash_hex(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}
//#endregion 🔖Golden

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use dsl_schema::{FieldSpec, JoinMode, ParseOptions, RecordLayout};

    //#region 🔖Fixtures
    /// @emoji 🧬 One field of most scalar `Shape` variants plus a nested `Record`, a `List`, a
    /// `Map`, and a `Tuple` — enough shape variety to exercise `RecordValueGen` and the round-trip
    /// laws without duplicating `pack_value`'s own exhaustive per-tag coverage.
    fn nested_point_spec() -> RecordSpec {
        RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "x", Shape::Float), FieldSpec::new(1, "y", Shape::Float)])
    }

    fn mixed_spec() -> RecordSpec {
        RecordSpec::new(
            None,
            RecordLayout::Lines,
            vec![
                FieldSpec::new(1, "flag", Shape::Bool),
                FieldSpec::new(2, "count", Shape::UInt),
                FieldSpec::new(3, "delta", Shape::Int),
                FieldSpec::new(4, "ratio", Shape::Float),
                FieldSpec::new(5, "name", Shape::Text),
                FieldSpec::new(6, "nickname", Shape::Text).optional(),
                FieldSpec::new(7, "payload", Shape::Bytes64),
                FieldSpec::new(8, "color", Shape::Enum(vec![("red".to_string(), 0), ("green".to_string(), 1), ("blue".to_string(), 2)])),
                FieldSpec::new(9, "point", Shape::Record(nested_point_spec)),
                FieldSpec::new(10, "tags", Shape::List(Box::new(Shape::Text))),
                FieldSpec::new(11, "coords", Shape::Tuple(Box::new(Shape::Int), Some(3))),
                FieldSpec::new(12, "labels", Shape::Map(Box::new(Shape::Int))),
            ],
        )
    }

    /// @emoji 📷 Simple scalar spec, small enough to print/parse deterministically for
    /// `assert_dsl_pack_bidirectional`.
    fn camera_spec() -> RecordSpec {
        RecordSpec::new(
            Some("camera"),
            RecordLayout::Inline,
            vec![FieldSpec::new(0, "x", Shape::Float), FieldSpec::new(1, "y", Shape::Float), FieldSpec::new(2, "zoom", Shape::Float), FieldSpec::new(3, "label", Shape::Text).optional()],
        )
    }

    fn camera_sample() -> RecordValue {
        let mut fields = HashMap::new();
        fields.insert(0, FieldValue::Float(1.0));
        fields.insert(1, FieldValue::Float(2.5));
        fields.insert(2, FieldValue::Float(3.0));
        fields.insert(3, FieldValue::Absent);
        RecordValue { fields }
    }
    //#endregion 🔖Fixtures

    //#region 🔖Arbitrary
    #[test]
    fn record_value_gen_is_deterministic_for_the_same_seed() {
        let spec = mixed_spec();
        let mut a = RecordValueGen::new(42);
        let mut b = RecordValueGen::new(42);
        assert_eq!(normalize_record(&a.generate(&spec, 4)), normalize_record(&b.generate(&spec, 4)), "same seed must produce the same RecordValue");
    }

    #[test]
    fn record_value_gen_differs_across_seeds() {
        let spec = mixed_spec();
        let mut a = RecordValueGen::new(1);
        let mut b = RecordValueGen::new(2);
        assert_ne!(normalize_record(&a.generate(&spec, 4)), normalize_record(&b.generate(&spec, 4)), "different seeds should (almost always) diverge");
    }

    #[test]
    fn record_value_gen_produces_values_the_pack_codec_accepts() {
        let spec = mixed_spec();
        for seed in 0..20u64 {
            let record = RecordValueGen::new(seed).generate(&spec, 4);
            let bytes = pack::encode_document(&spec, &record, &pack::EncodeOptions::default()).unwrap_or_else(|e| panic!("seed {seed}: encode_document failed: {e}"));
            pack::decode_document(&bytes, &spec, &pack::DecodeOptions::default()).unwrap_or_else(|e| panic!("seed {seed}: decode_document failed: {e}"));
        }
    }

    #[test]
    fn record_value_gen_bounds_recursion_by_max_depth() {
        // 🌳 `group_spec`-style self-referential Statements table: its own single variant's spec
        // points right back at itself. A generator that ignored `max_depth` would stack-overflow.
        fn recursive_spec() -> RecordSpec {
            RecordSpec::new(
                Some("group"),
                RecordLayout::Inline,
                vec![FieldSpec::new(0, "id", Shape::Text), FieldSpec::new(1, "children", Shape::Statements(vec![("group".to_string(), recursive_spec)]))],
            )
        }
        let spec = recursive_spec();
        let mut gen = RecordValueGen::new(7);
        let record = gen.generate(&spec, 3);
        // 🌱 Merely surviving without a stack overflow (plus a successful pack round trip) is the
        // assertion here — the value's shape isn't otherwise constrained.
        assert_encode_decode_identity(&spec, &record);
    }
    //#endregion 🔖Arbitrary

    //#region 🔖Laws
    #[test]
    fn law_encode_decode_identity_holds_for_generated_records() {
        let spec = mixed_spec();
        for seed in 0..12u64 {
            let record = RecordValueGen::new(seed).generate(&spec, 4);
            assert_encode_decode_identity(&spec, &record);
        }
    }

    #[test]
    fn law_canonical_stable_holds_for_generated_records() {
        let spec = mixed_spec();
        for seed in 0..12u64 {
            let record = RecordValueGen::new(seed).generate(&spec, 4);
            assert_canonical_stable(&spec, &record);
        }
    }

    #[test]
    fn law_unknown_field_preserved_holds() {
        let spec = mixed_spec();
        let mut record = RecordValueGen::new(3).generate(&spec, 3);
        record.fields.insert(999, FieldValue::Text("from-the-future".to_string()));
        record.fields.insert(1000, FieldValue::UInt(7));
        assert_unknown_field_preserved(&spec, &record, &[999, 1000]);
    }

    #[test]
    fn law_streamed_equals_buffered_holds() {
        let spec = mixed_spec();
        for seed in 0..6u64 {
            let record = RecordValueGen::new(seed).generate(&spec, 4);
            assert_streamed_equals_buffered(&spec, &record);
        }
    }

    #[test]
    fn law_dsl_pack_bidirectional_holds_for_a_hand_built_sample() {
        let spec = camera_spec();
        let parse_dsl = |text: &str| dsl_schema::parse(text, &spec, &ParseOptions::default()).unwrap_or_else(|e| panic!("parse failed: {e}"));
        let print_dsl = |value: &RecordValue| dsl_schema::print(value, &spec, JoinMode::Document);
        let encode_pack = |value: &RecordValue| pack::encode_document(&spec, value, &pack::EncodeOptions::default()).expect("encode_pack");
        let decode_pack = |bytes: &[u8]| pack::decode_document(bytes, &spec, &pack::DecodeOptions::default()).expect("decode_pack").0;
        assert_dsl_pack_bidirectional(parse_dsl, print_dsl, encode_pack, decode_pack, &camera_sample());
    }
    //#endregion 🔖Laws

    //#region 🔖Corrupt
    fn decode_closure(spec: RecordSpec) -> impl Fn(&[u8]) -> Result<(), String> {
        move |bytes: &[u8]| pack::decode_document(bytes, &spec, &pack::DecodeOptions::default()).map(|_| ()).map_err(|e| e.to_string())
    }

    #[test]
    fn fuzz_truncation_never_panics_on_a_real_encoded_document() {
        let spec = mixed_spec();
        let record = RecordValueGen::new(9).generate(&spec, 4);
        let bytes = pack::encode_document(&spec, &record, &pack::EncodeOptions::default()).expect("encode_document");
        let report = fuzz_truncation(&bytes, CorruptionLevel::Quick, decode_closure(spec));
        assert!(report.cases_panicked.is_empty(), "fuzz_truncation observed panics: {:?}", report.cases_panicked);
        assert!(report.cases_run > 0, "fuzz_truncation must have run at least one case");
    }

    #[test]
    fn fuzz_bit_flips_never_panics_on_a_real_encoded_document() {
        let spec = mixed_spec();
        let record = RecordValueGen::new(11).generate(&spec, 4);
        let bytes = pack::encode_document(&spec, &record, &pack::EncodeOptions::default()).expect("encode_document");
        let report = fuzz_bit_flips(&bytes, CorruptionLevel::Quick, decode_closure(spec));
        assert!(report.cases_panicked.is_empty(), "fuzz_bit_flips observed panics: {:?}", report.cases_panicked);
        assert!(report.cases_run > 0, "fuzz_bit_flips must have run at least one case");
    }

    #[test]
    fn fuzz_truncation_and_bit_flips_report_zero_cases_for_empty_input() {
        let decode: fn(&[u8]) -> Result<(), String> = |_| Ok(());
        let truncation_report = fuzz_truncation(&[], CorruptionLevel::Quick, decode);
        let bit_flip_report = fuzz_bit_flips(&[], CorruptionLevel::Quick, decode);
        assert_eq!(truncation_report.cases_run, 0);
        assert_eq!(bit_flip_report.cases_run, 0);
        assert!(truncation_report.cases_panicked.is_empty());
        assert!(bit_flip_report.cases_panicked.is_empty());
    }

    #[test]
    fn fuzz_harness_catches_a_panicking_decoder_instead_of_crashing_the_process() {
        let always_panics: fn(&[u8]) -> Result<(), String> = |_| panic!("intentional panic for harness self-test");
        let report = fuzz_truncation(&[1, 2, 3, 4], CorruptionLevel::Quick, always_panics);
        assert_eq!(report.cases_panicked.len() as u64, report.cases_run, "every case should have been caught as a panic");
        assert!(report.cases_panicked.iter().all(|msg| msg.contains("intentional panic")), "panic message should be captured: {:?}", report.cases_panicked);
    }
    //#endregion 🔖Corrupt

    //#region 🔖Golden
    #[test]
    fn golden_hash_hex_is_deterministic_and_sensitive_to_content() {
        let a = golden_hash_hex(b"hello pack");
        let b = golden_hash_hex(b"hello pack");
        let c = golden_hash_hex(b"hello pack!");
        assert_eq!(a, b, "golden_hash_hex must be deterministic for the same bytes");
        assert_ne!(a, c, "golden_hash_hex must differ for different bytes");
        assert_eq!(a.len(), 64, "blake3 hex digest is 64 hex chars (32 bytes)");
        assert!(a.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()), "golden_hash_hex must be lowercase hex: {a}");
    }

    #[test]
    fn golden_hash_hex_matches_a_real_encoded_document_across_two_encodes() {
        let spec = mixed_spec();
        let record = RecordValueGen::new(5).generate(&spec, 4);
        let a = pack::encode_document(&spec, &record, &pack::EncodeOptions::default()).unwrap();
        let b = pack::encode_document(&spec, &record, &pack::EncodeOptions::default()).unwrap();
        assert_eq!(golden_hash_hex(&a), golden_hash_hex(&b), "golden hash of a canonical encoding must be stable across repeated encodes");
    }
    //#endregion 🔖Golden

    //#region 🔖Long
    mod long {
        use super::*;

        #[test]
        fn fuzz_truncation_and_bit_flips_never_panic_at_long_density() {
            let spec = mixed_spec();
            let record = RecordValueGen::new(21).generate(&spec, 5);
            let bytes = pack::encode_document(&spec, &record, &pack::EncodeOptions::default()).expect("encode_document");
            let truncation_report = fuzz_truncation(&bytes, CorruptionLevel::Long, decode_closure(spec.clone()));
            let bit_flip_report = fuzz_bit_flips(&bytes, CorruptionLevel::Long, decode_closure(spec));
            assert!(truncation_report.cases_panicked.is_empty(), "long-level truncation fuzz observed panics: {:?}", truncation_report.cases_panicked);
            assert!(bit_flip_report.cases_panicked.is_empty(), "long-level bit-flip fuzz observed panics: {:?}", bit_flip_report.cases_panicked);
            assert!(truncation_report.cases_run >= bytes.len().min(128) as u64);
            assert!(bit_flip_report.cases_run >= 128);
        }
    }
    //#endregion 🔖Long

    //#region 🔖Exhaustive
    mod exhaustive {
        use super::*;

        #[test]
        fn fuzz_truncation_never_panics_at_every_single_byte_offset() {
            let spec = mixed_spec();
            let record = RecordValueGen::new(33).generate(&spec, 5);
            let bytes = pack::encode_document(&spec, &record, &pack::EncodeOptions::default()).expect("encode_document");
            let report = fuzz_truncation(&bytes, CorruptionLevel::Exhaustive, decode_closure(spec));
            assert!(report.cases_panicked.is_empty(), "exhaustive truncation fuzz observed panics: {:?}", report.cases_panicked);
            assert_eq!(report.cases_run, bytes.len() as u64, "exhaustive truncation must try every offset");
        }

        #[test]
        fn fuzz_bit_flips_never_panics_at_every_single_bit() {
            let spec = mixed_spec();
            let record = RecordValueGen::new(34).generate(&spec, 3);
            let bytes = pack::encode_document(&spec, &record, &pack::EncodeOptions::default()).expect("encode_document");
            let report = fuzz_bit_flips(&bytes, CorruptionLevel::Exhaustive, decode_closure(spec));
            assert!(report.cases_panicked.is_empty(), "exhaustive bit-flip fuzz observed panics: {:?}", report.cases_panicked);
            assert_eq!(report.cases_run, bytes.len() as u64 * 8, "exhaustive bit-flip must try every bit");
        }
    }
    //#endregion 🔖Exhaustive
}
//#endregion 🧪Tests
