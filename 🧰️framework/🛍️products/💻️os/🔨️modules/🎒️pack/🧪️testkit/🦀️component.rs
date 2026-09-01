//! 📦️ `pack_testkit` — shared test infrastructure for the whole `pack_*` crate family: a
//! deterministic seeded `RecordValueGen` that fabricates `crate::os_dsl::schema::RecordValue`s from any
//! `RecordSpec`, the cross-crate round-trip/determinism/preservation LAWS every encoder/decoder
//! pair must satisfy, a panic-safe truncation/bit-flip corruption harness, and a golden-hash
//! helper for committing expected byte-content as a text constant.
//!
//! See the `## pack_testkit` section of the wave-0 contract at
//! `.🦑️repo/🎫️tickets/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/contract.md` for the binding
//! signatures this crate implements against. Deliberately depends on `dsl_schema`/`dsl_core`
//! directly (not just `pack`) rather than the `arbitrary`/`quickcheck` crates — the generator is
//! a small hand-rolled splitmix64 PRNG, and float generation round-trips through `dsl_core`'s own
//! canonical text form so generated values stay representable by `vcs`/`dsl_derive`'s future
//! DSL-bidirectional tests without this crate needing to depend on either of them.

//#region 🔖️Corrupt
/// 🧪️ Container-level corruption sweeps live with the container itself; re-exported so the
/// historical `os_pack::testkit::fuzz_*` paths keep resolving.
pub use pack::testkit::*;
//#endregion 🔖️Corrupt

use crate::os_dsl::schema::{DslValue, ExprValue, FieldValue, RecordSpec, RecordValue, Shape, WireEdgeLabel, WireNode, WireValue};
use std::collections::HashMap;

//#region 🔖️Arbitrary
/// @emoji 🎲️ Deterministic seeded generator: splitmix64 state, advanced on every draw. NOT the
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

    /// @emoji 🌱️ Fabricates one `RecordValue` matching `spec`, recursing into nested
    /// `Record`/`Block`/`Statements`/`Table`/`Map`/`Value` shapes up to `max_depth` — beyond that,
    /// [`Self::shallow_value`] takes over so genuinely self-referential specs (a recursive
    /// `Statements` table whose own variant list names itself) terminate instead of looping
    /// forever.
    pub fn generate(&mut self, spec: &RecordSpec, max_depth: u16) -> RecordValue {
        self.generate_record(spec, 0, max_depth)
    }

    //#region 🔖️Prng
    /// @emoji 🌀️ splitmix64 — see <https://prng.di.unimi.it/splitmix64.c>. Small, dependency-free,
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

    /// @emoji 🎯️ Uniform-ish `[0, bound)`; `0` for `bound == 0` (modulo bias is irrelevant for
    /// test-data spread).
    fn next_range(&mut self, bound: u64) -> u64 {
        if bound == 0 {
            0
        } else {
            self.next_u64() % bound
        }
    }

    fn next_int(&mut self) -> i64 {
        self.next_range(2001) as i64 - 1000
    }

    fn next_uint(&mut self) -> u64 {
        self.next_range(1_000_000)
    }

    /// @emoji 🔢️ Deliberately never NaN/Infinity — `FieldValue`'s derived `PartialEq` uses `==`,
    /// under which `NaN != NaN` always, so a generated NaN would make
    /// `assert_encode_decode_identity` fail even on a perfectly correct codec. Round-trips through
    /// `crate::os_dsl::format_f64`/`parse_f64` so every generated float is also exactly
    /// DSL-representable — load-bearing for any future `assert_dsl_pack_bidirectional` caller that
    /// seeds its sample from this generator.
    fn next_f64(&mut self) -> f64 {
        let magnitude = self.next_range(1_000_000) as f64 / 100.0;
        let sign = if self.next_bool() { -1.0 } else { 1.0 };
        let raw = sign * magnitude;
        crate::os_dsl::parse_f64(&crate::os_dsl::format_f64(raw)).unwrap_or(raw)
    }

    const ALPHABET: &'static [u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_ ";

    // 🔁️ `.map(...).collect()` here would need an `async` closure over a `&mut self` PRNG draw —
    // sync closures can't `` (R10 residue shape 1) — so the draws are sequenced by hand.
    fn next_string(&mut self, max_len: usize) -> String {
        let len = self.next_range(max_len as u64 + 1) as usize;
        let mut out = String::with_capacity(len);
        for _ in 0..len {
            let idx = self.next_range(Self::ALPHABET.len() as u64) as usize;
            out.push(Self::ALPHABET[idx] as char);
        }
        out
    }

    fn next_bytes(&mut self, max_len: usize) -> Vec<u8> {
        let len = self.next_range(max_len as u64 + 1) as usize;
        let mut out = Vec::with_capacity(len);
        for _ in 0..len {
            out.push((self.next_u64() & 0xFF) as u8);
        }
        out
    }
    //#endregion 🔖️Prng

    //#region 🔖️Shapes
    // 🔁️ Mutually recursive with `generate_value` (which also recurses into itself directly for
    // `Tuple`/`List`/`Map`/`Block`) — every edge in that cycle is `...` because an
    // `fn`'s own opaque `Future` type cannot embed itself or a cycle-partner's opaque type at
    // an unboxed, unbounded size (R10 residue shape 3).
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
            // 🔁️ Every arm below that used to be `.map(|_| self.generate_value(...)).collect()`
            // is rewritten as an explicit loop: the closure would need to `` a `&mut self`
            // draw, and `Iterator::map`'s closure is sync (R10 residue shape 1).
            Shape::Tuple(elem, len) => {
                let n = match len {
                    Some(n) => *n,
                    None => 1 + self.next_range(3) as usize,
                };
                let mut items = Vec::with_capacity(n);
                for _ in 0..n {
                    items.push(self.generate_value(elem, depth + 1, max_depth));
                }
                FieldValue::Tuple(items)
            }
            Shape::List(elem) => {
                let n = self.next_range(4) as usize;
                let mut items = Vec::with_capacity(n);
                for _ in 0..n {
                    items.push(self.generate_value(elem, depth + 1, max_depth));
                }
                FieldValue::List(items)
            }
            Shape::Record(spec_fn) => FieldValue::Record(self.generate_record(&spec_fn(), depth + 1, max_depth)),
            Shape::Block(inner) => FieldValue::Block(Box::new(self.generate_value(inner, depth + 1, max_depth))),
            Shape::Statements(variants) => {
                if variants.is_empty() {
                    FieldValue::Statements(Vec::new())
                } else {
                    let n = self.next_range(3) as usize;
                    let mut items = Vec::with_capacity(n);
                    for _ in 0..n {
                        let idx = self.next_range(variants.len() as u64) as usize;
                        let (keyword, spec_fn) = &variants[idx];
                        let keyword = keyword.clone();
                        let record = self.generate_record(&spec_fn(), depth + 1, max_depth);
                        items.push((keyword, record));
                    }
                    FieldValue::Statements(items)
                }
            }
            Shape::Map(inner) => {
                let n = self.next_range(3) as usize;
                let mut entries = Vec::with_capacity(n);
                for _ in 0..n {
                    let key = self.next_string(6);
                    let value = self.generate_value(inner, depth + 1, max_depth);
                    entries.push((key, value));
                }
                FieldValue::Map(entries)
            }
            Shape::Value => FieldValue::Value(self.generate_dsl_value(depth + 1, max_depth)),
            Shape::Table(spec_fn) => {
                let row_spec = spec_fn();
                let n = self.next_range(3) as usize;
                let mut rows = Vec::with_capacity(n);
                for _ in 0..n {
                    rows.push(FieldValue::Record(self.generate_record(&row_spec, depth + 1, max_depth)));
                }
                FieldValue::List(rows)
            }
            Shape::Wire => FieldValue::Wire(self.generate_wire(depth + 1, max_depth)),
            Shape::Quantity(_) | Shape::Angle(_) => FieldValue::Float(self.next_f64()),
            Shape::Ref(_) => FieldValue::Text(self.next_string(6)),
            Shape::Coord(dims) => {
                let mut items = Vec::new();
                for _ in 0..*dims {
                    items.push(FieldValue::Float(self.next_f64()));
                }
                FieldValue::Tuple(items)
            }
            Shape::Dir => {
                let mut items = Vec::new();
                for _ in 0..3 {
                    items.push(FieldValue::Float(self.next_f64()));
                }
                FieldValue::Tuple(items)
            }
            Shape::Dim(dims) => {
                let mut items = Vec::new();
                for _ in 0..*dims {
                    items.push(FieldValue::Float(self.next_f64()));
                }
                FieldValue::Tuple(items)
            }
            Shape::Range => FieldValue::Tuple(vec![FieldValue::Float(self.next_f64()), FieldValue::Float(self.next_f64())]),
            Shape::Count => FieldValue::UInt(self.next_uint()),
            Shape::Expr => FieldValue::Expr(ExprValue::Num(self.next_f64())),
            Shape::Embed(_) => FieldValue::Text(self.next_string(8)),
            Shape::EmbedFrom(_) => FieldValue::Text(self.next_string(8)),
        }
    }

    /// @emoji 🛑️ Non-recursing terminal value for `shape`, used once `max_depth` is exhausted.
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
            Shape::Enum(variants) => FieldValue::Enum(variants.first().map_or(0, |(_, ordinal)| *ordinal)),
            Shape::Tuple(_, _) => FieldValue::Tuple(Vec::new()),
            Shape::List(_) => FieldValue::List(Vec::new()),
            Shape::Record(_) => FieldValue::Record(RecordValue::default()),
            Shape::Block(inner) => FieldValue::Block(Box::new(self.shallow_value(inner))),
            Shape::Statements(_) => FieldValue::Statements(Vec::new()),
            Shape::Map(_) => FieldValue::Map(Vec::new()),
            Shape::Value => FieldValue::Value(DslValue::Null),
            Shape::Table(_) => FieldValue::List(Vec::new()),
            Shape::Wire => FieldValue::Wire(WireValue { from: WireNode { id: "n".to_string(), kind: None, port: None }, edge: None, edge_label: WireEdgeLabel::default(), properties: DslValue::Null }),
            Shape::Quantity(_) | Shape::Angle(_) => FieldValue::Float(self.next_f64()),
            Shape::Ref(_) => FieldValue::Text(self.next_string(6)),
            Shape::Coord(dims) => {
                let mut items = Vec::new();
                for _ in 0..*dims {
                    items.push(FieldValue::Float(self.next_f64()));
                }
                FieldValue::Tuple(items)
            }
            Shape::Dir => {
                let mut items = Vec::new();
                for _ in 0..3 {
                    items.push(FieldValue::Float(self.next_f64()));
                }
                FieldValue::Tuple(items)
            }
            Shape::Dim(dims) => {
                let mut items = Vec::new();
                for _ in 0..*dims {
                    items.push(FieldValue::Float(self.next_f64()));
                }
                FieldValue::Tuple(items)
            }
            Shape::Range => FieldValue::Tuple(vec![FieldValue::Float(self.next_f64()), FieldValue::Float(self.next_f64())]),
            Shape::Count => FieldValue::UInt(self.next_uint()),
            Shape::Expr => FieldValue::Expr(ExprValue::Num(self.next_f64())),
            Shape::Embed(_) => FieldValue::Text(self.next_string(8)),
            Shape::EmbedFrom(_) => FieldValue::Text(self.next_string(8)),
        }
    }

    // 🔁️ Self-recursive (`Array`/`Object` arms) — boxed for the same reason as `generate_value`.
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
                let mut items = Vec::with_capacity(n);
                for _ in 0..n {
                    items.push(self.generate_dsl_value(depth + 1, max_depth));
                }
                DslValue::Array(items)
            }
            _ => {
                let n = self.next_range(3) as usize;
                let mut entries = Vec::with_capacity(n);
                for _ in 0..n {
                    let key = self.next_string(6);
                    let value = self.generate_dsl_value(depth + 1, max_depth);
                    entries.push((key, value));
                }
                DslValue::Object(entries)
            }
        }
    }

    fn generate_wire_node(&mut self) -> WireNode {
        let id = self.next_string(6);
        let id = if id.trim().is_empty() { "n".to_string() } else { id };
        WireNode { id, kind: if self.next_bool() { Some(self.next_string(4)) } else { None }, port: if self.next_bool() { Some(self.next_string(4)) } else { None } }
    }

    fn generate_wire(&mut self, depth: u16, max_depth: u16) -> WireValue {
        let from = self.generate_wire_node();
        let edge = if self.next_bool() {
            let directed = self.next_bool();
            let node = self.generate_wire_node();
            Some((directed, node))
        } else {
            None
        };
        let properties = self.generate_dsl_value(depth, max_depth);
        WireValue { from, edge, edge_label: WireEdgeLabel::default(), properties }
    }
    //#endregion 🔖️Shapes
}
//#endregion 🔖️Arbitrary

//#region 🔖️Laws
/// @emoji 🧹️ Strips `FieldValue::Absent` entries at every nesting level (the "pure-Absent noise"
/// `decode_document` reinserts for every spec field not found on the wire — canonical mode never
/// encodes `Absent`, so a freshly-decoded record always carries one for every spec field the
/// generator happened to skip) and sorts `Map` entries by key bytes (`encode_map` always sorts,
/// unconditionally, per `pack_value`'s purity LAW — a generator that inserted map entries in
/// non-canonical order would otherwise fail this comparison on ordering alone, not content).
/// Shared by every LAW below that compares an original `RecordValue` against a decoded one.
// 🔁️ Mutually recursive (`normalize_record` <-> `normalize_value`, plus `normalize_value`'s own
// self-recursion for `Tuple`/`List`/`Block`) — every edge is `...` (R10 residue
// shape 3), and the `.iter().map(normalize_value).collect()` shapes are rewritten as loops since
// `Iterator::map`'s closure can't `` (R10 residue shape 1).
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
        FieldValue::Tuple(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(normalize_value(item));
            }
            FieldValue::Tuple(out)
        }
        FieldValue::List(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(normalize_value(item));
            }
            FieldValue::List(out)
        }
        FieldValue::Block(inner) => FieldValue::Block(Box::new(normalize_value(inner))),
        FieldValue::Statements(items) => {
            let mut out = Vec::with_capacity(items.len());
            for (k, r) in items {
                out.push((k.clone(), normalize_record(r)));
            }
            FieldValue::Statements(out)
        }
        FieldValue::Map(entries) => {
            let mut sorted: Vec<(String, FieldValue)> = Vec::with_capacity(entries.len());
            for (k, v) in entries {
                sorted.push((k.clone(), normalize_value(v)));
            }
            sorted.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
            FieldValue::Map(sorted)
        }
        other => other.clone(),
    }
}

/// @emoji 🔁️ LAW: `decode_document(encode_document(spec, record)) == record`, modulo the
/// pure-Absent noise [`normalize_record`] strips.
pub async fn assert_encode_decode_identity(spec: &RecordSpec, record: &RecordValue) {
    let options = crate::os_pack::EncodeOptions::default();
    let bytes = crate::os_pack::encode_document(spec, record, &options).expect("encode_document should succeed for a well-formed record");
    let (decoded, _report) = crate::os_pack::decode_document(&bytes, spec, &crate::os_pack::DecodeOptions::default()).expect("decode_document should succeed for a just-encoded pack file");
    assert_eq!(normalize_record(&decoded), normalize_record(record), "encode/decode round trip diverged (ignoring pure-Absent noise)");
}

/// @emoji 🧊️ LAW: `encode_document` is a pure function of `(spec, record)` — byte-identical
/// output across repeated calls, regardless of `HashMap` iteration order inside `record.fields`.
pub async fn assert_canonical_stable(spec: &RecordSpec, record: &RecordValue) {
    let options = crate::os_pack::EncodeOptions::default();
    let a = crate::os_pack::encode_document(spec, record, &options).expect("first encode_document call");
    let b = crate::os_pack::encode_document(spec, record, &options).expect("second encode_document call");
    assert_eq!(a, b, "encode_document must be byte-identical across repeated calls (canonical determinism law)");
}

/// @emoji 🕳️ LAW: field ids present in `record_with_extra_fields` but absent from `spec` still
/// round-trip through the wire and are reported in `DecodeReport.unknown_field_ids` — the
/// mechanism that lets an older reader tolerate a newer writer's additive schema evolution.
pub async fn assert_unknown_field_preserved(spec: &RecordSpec, record_with_extra_fields: &RecordValue, extra_ids: &[u16]) {
    let options = crate::os_pack::EncodeOptions::default();
    let bytes = crate::os_pack::encode_document(spec, record_with_extra_fields, &options).expect("encode_document with extra fields");
    let (decoded, report) = crate::os_pack::decode_document(&bytes, spec, &crate::os_pack::DecodeOptions::default()).expect("decode_document with extra fields");

    let mut expected_extra: Vec<u16> = extra_ids.to_vec();
    expected_extra.sort_unstable();
    let mut actual_extra: Vec<u16> = report.unknown_field_ids;
    actual_extra.sort_unstable();
    assert_eq!(actual_extra, expected_extra, "DecodeReport.unknown_field_ids must exactly match the caller-declared extra field ids");

    for id in extra_ids {
        let expected_value = record_with_extra_fields.fields.get(id).expect("extra_ids must reference fields present in record_with_extra_fields");
        let actual_value = decoded.fields.get(id).expect("an unknown field must still be preserved in the decoded RecordValue");
        assert_eq!(normalize_value(actual_value), normalize_value(expected_value), "unknown field {id} must round-trip unchanged");
    }
}

/// @emoji 🌊️ LAW: splitting a document's body across many small `Document` frames (a "streamed"
/// encode, `frame_size = 1`) decodes to the exact same `RecordValue` as encoding it as one large
/// frame (a "buffered" encode) — `decode_document` must reassemble frames transparently
/// regardless of how many the encoder chose to emit.
pub async fn assert_streamed_equals_buffered(spec: &RecordSpec, record: &RecordValue) {
    let mut buffered_options = crate::os_pack::EncodeOptions::default();
    buffered_options.frame_size = 8 * 1024 * 1024;
    let mut streamed_options = crate::os_pack::EncodeOptions::default();
    streamed_options.frame_size = 1;

    let buffered_bytes = crate::os_pack::encode_document(spec, record, &buffered_options).expect("buffered (single-frame) encode_document");
    let streamed_bytes = crate::os_pack::encode_document(spec, record, &streamed_options).expect("streamed (many-frame) encode_document");

    let (buffered_decoded, _) = crate::os_pack::decode_document(&buffered_bytes, spec, &crate::os_pack::DecodeOptions::default()).expect("decode buffered encoding");
    let (streamed_decoded, _) = crate::os_pack::decode_document(&streamed_bytes, spec, &crate::os_pack::DecodeOptions::default()).expect("decode streamed encoding");

    assert_eq!(normalize_record(&buffered_decoded), normalize_record(&streamed_decoded), "single-frame and many-small-frame encodings of the same document must decode identically");
}

/// @emoji 🔀️ LAW: `decode_pack(encode_pack(sample)) == parse_dsl(print_dsl(sample)) == sample` —
/// the DSL text and pack binary encodings of the same value must agree with each other and with
/// the original. Kept generic over closures so this crate needs no dependency on `vcs`/
/// `dsl_derive`; their own `test_support` wraps this with concrete `P: ArtifactDsl + ArtifactPack`
/// bounds in wave 1.
pub async fn assert_dsl_pack_bidirectional<P>(parse_dsl: impl AsyncFn(&str) -> P, print_dsl: impl AsyncFn(&P) -> String, encode_pack: impl AsyncFn(&P) -> Vec<u8>, decode_pack: impl AsyncFn(&[u8]) -> P, sample: &P)
where
    P: PartialEq + std::fmt::Debug,
{
    let printed = print_dsl(sample).await;
    let reparsed = parse_dsl(&printed).await;
    let encoded = encode_pack(sample).await;
    let decoded = decode_pack(&encoded).await;
    assert_eq!(&decoded, sample, "decode_pack(encode_pack(sample)) must equal sample");
    assert_eq!(&reparsed, sample, "parse_dsl(print_dsl(sample)) must equal sample");
    assert_eq!(decoded, reparsed, "decode_pack(encode_pack(sample)) must equal parse_dsl(print_dsl(sample))");
}
//#endregion 🔖️Laws

//#region 🔖️Golden
/// @emoji 🔑️ `hex(blake3(bytes))` — for committing an expected pack encoding's hash as a text
/// constant in a caller's own test, so a future unintended encoding change is caught by a one-line
/// diff instead of a giant byte-literal.
pub async fn golden_hash_hex(bytes: &[u8]) -> String {
    semio_framework_hash::hash(bytes).to_hex().to_string()
}
//#endregion 🔖️Golden

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::os_dsl::schema::{FieldSpec, JoinMode, ParseOptions, RecordLayout};

    //#region 🔖️Fixtures
    /// @emoji 🧬️ One field of most scalar `Shape` variants plus a nested `Record`, a `List`, a
    /// `Map`, and a `Tuple` — enough shape variety to exercise `RecordValueGen` and the round-trip
    /// laws without duplicating `pack_value`'s own exhaustive per-tag coverage.
    // 🚫️async: E4 fn-pointer slot — stored bare as `fn() -> RecordSpec` via `Shape::Record` below
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

    /// @emoji 📷️ Simple scalar spec, small enough to print/parse deterministically for
    /// `assert_dsl_pack_bidirectional`.
    fn camera_spec() -> RecordSpec {
        RecordSpec::new(Some("camera"), RecordLayout::Inline, vec![FieldSpec::new(0, "x", Shape::Float), FieldSpec::new(1, "y", Shape::Float), FieldSpec::new(2, "zoom", Shape::Float), FieldSpec::new(3, "label", Shape::Text).optional()])
    }

    fn camera_sample() -> RecordValue {
        let mut fields = HashMap::new();
        fields.insert(0, FieldValue::Float(1.0));
        fields.insert(1, FieldValue::Float(2.5));
        fields.insert(2, FieldValue::Float(3.0));
        fields.insert(3, FieldValue::Absent);
        RecordValue { fields }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️Arbitrary
    #[semio_framework_async_macros::async_test]
    async fn record_value_gen_is_deterministic_for_the_same_seed() {
        let spec = mixed_spec();
        let mut a = RecordValueGen::new(42);
        let mut b = RecordValueGen::new(42);
        assert_eq!(normalize_record(&a.generate(&spec, 4)), normalize_record(&b.generate(&spec, 4)), "same seed must produce the same RecordValue");
    }

    #[semio_framework_async_macros::async_test]
    async fn record_value_gen_differs_across_seeds() {
        let spec = mixed_spec();
        let mut a = RecordValueGen::new(1);
        let mut b = RecordValueGen::new(2);
        assert_ne!(normalize_record(&a.generate(&spec, 4)), normalize_record(&b.generate(&spec, 4)), "different seeds should (almost always) diverge");
    }

    #[semio_framework_async_macros::async_test]
    async fn record_value_gen_produces_values_the_pack_codec_accepts() {
        let spec = mixed_spec();
        for seed in 0..20u64 {
            let record = RecordValueGen::new(seed).generate(&spec, 4);
            let bytes = crate::os_pack::encode_document(&spec, &record, &crate::os_pack::EncodeOptions::default()).unwrap_or_else(|e| panic!("seed {seed}: encode_document failed: {e}"));
            crate::os_pack::decode_document(&bytes, &spec, &crate::os_pack::DecodeOptions::default()).unwrap_or_else(|e| panic!("seed {seed}: decode_document failed: {e}"));
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn record_value_gen_bounds_recursion_by_max_depth() {
        // 🌳️ `group_spec`-style self-referential Statements table: its own single variant's spec
        // points right back at itself. A generator that ignored `max_depth` would stack-overflow.
        // 🚫️async: E4 fn-pointer slot — stored bare as `fn() -> RecordSpec` via `Shape::Statements` below
        fn recursive_spec() -> RecordSpec {
            RecordSpec::new(Some("group"), RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Text), FieldSpec::new(1, "children", Shape::Statements(vec![("group".to_string(), recursive_spec)]))])
        }
        let spec = recursive_spec();
        let mut gen = RecordValueGen::new(7);
        let record = gen.generate(&spec, 3);
        // 🌱️ Merely surviving without a stack overflow (plus a successful pack round trip) is the
        // assertion here — the value's shape isn't otherwise constrained.
        assert_encode_decode_identity(&spec, &record).await;
    }
    //#endregion 🔖️Arbitrary

    //#region 🔖️Laws
    #[semio_framework_async_macros::async_test]
    async fn law_encode_decode_identity_holds_for_generated_records() {
        let spec = mixed_spec();
        for seed in 0..12u64 {
            let record = RecordValueGen::new(seed).generate(&spec, 4);
            assert_encode_decode_identity(&spec, &record).await;
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn law_canonical_stable_holds_for_generated_records() {
        let spec = mixed_spec();
        for seed in 0..12u64 {
            let record = RecordValueGen::new(seed).generate(&spec, 4);
            assert_canonical_stable(&spec, &record).await;
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn law_unknown_field_preserved_holds() {
        let spec = mixed_spec();
        let mut record = RecordValueGen::new(3).generate(&spec, 3);
        record.fields.insert(999, FieldValue::Text("from-the-future".to_string()));
        record.fields.insert(1000, FieldValue::UInt(7));
        assert_unknown_field_preserved(&spec, &record, &[999, 1000]).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn law_streamed_equals_buffered_holds() {
        let spec = mixed_spec();
        for seed in 0..6u64 {
            let record = RecordValueGen::new(seed).generate(&spec, 4);
            assert_streamed_equals_buffered(&spec, &record).await;
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn law_dsl_pack_bidirectional_holds_for_a_hand_built_sample() {
        let spec = camera_spec();
        let parse_dsl = async |text: &str| crate::os_dsl::schema::parse(text, &spec, &ParseOptions::default()).unwrap_or_else(|e| panic!("parse failed: {e}"));
        let print_dsl = async |value: &RecordValue| crate::os_dsl::schema::print(value, &spec, JoinMode::Document);
        let encode_pack = async |value: &RecordValue| crate::os_pack::encode_document(&spec, value, &crate::os_pack::EncodeOptions::default()).expect("encode_pack");
        let decode_pack = async |bytes: &[u8]| crate::os_pack::decode_document(bytes, &spec, &crate::os_pack::DecodeOptions::default()).expect("decode_pack").0;
        assert_dsl_pack_bidirectional(parse_dsl, print_dsl, encode_pack, decode_pack, &camera_sample()).await;
    }
    //#endregion 🔖️Laws

    //#region 🔖️Corrupt
    // 🚫️async: E1 pure adapter consumed by `fuzz_truncation`/`fuzz_bit_flips`'s sync `impl Fn`
    // decoder slot — bridges via `os_io::resolve_ready` (the same sanctioned pattern already used
    // by `📡️spr/🧪️testkit`'s `fuzz_truncation_never_panics_history_reader_open`), see R9/E5.
    fn decode_closure(spec: RecordSpec) -> impl Fn(&[u8]) -> Result<(), String> {
        move |bytes: &[u8]| crate::os_pack::decode_document(bytes, &spec, &crate::os_pack::DecodeOptions::default()).map(|_| ()).map_err(|e| e.to_string())
    }

    #[semio_framework_async_macros::async_test]
    async fn fuzz_truncation_never_panics_on_a_real_encoded_document() {
        let spec = mixed_spec();
        let record = RecordValueGen::new(9).generate(&spec, 4);
        let bytes = crate::os_pack::encode_document(&spec, &record, &crate::os_pack::EncodeOptions::default()).expect("encode_document");
        let report = fuzz_truncation(&bytes, CorruptionLevel::Quick, decode_closure(spec));
        assert!(report.cases_panicked.is_empty(), "fuzz_truncation observed panics: {:?}", report.cases_panicked);
        assert!(report.cases_run > 0, "fuzz_truncation must have run at least one case");
    }

    #[semio_framework_async_macros::async_test]
    async fn fuzz_bit_flips_never_panics_on_a_real_encoded_document() {
        let spec = mixed_spec();
        let record = RecordValueGen::new(11).generate(&spec, 4);
        let bytes = crate::os_pack::encode_document(&spec, &record, &crate::os_pack::EncodeOptions::default()).expect("encode_document");
        let report = fuzz_bit_flips(&bytes, CorruptionLevel::Quick, decode_closure(spec));
        assert!(report.cases_panicked.is_empty(), "fuzz_bit_flips observed panics: {:?}", report.cases_panicked);
        assert!(report.cases_run > 0, "fuzz_bit_flips must have run at least one case");
    }

    #[semio_framework_async_macros::async_test]
    async fn fuzz_truncation_and_bit_flips_report_zero_cases_for_empty_input() {
        let decode: fn(&[u8]) -> Result<(), String> = |_| Ok(());
        let truncation_report = fuzz_truncation(&[], CorruptionLevel::Quick, decode);
        let bit_flip_report = fuzz_bit_flips(&[], CorruptionLevel::Quick, decode);
        assert_eq!(truncation_report.cases_run, 0);
        assert_eq!(bit_flip_report.cases_run, 0);
        assert!(truncation_report.cases_panicked.is_empty());
        assert!(bit_flip_report.cases_panicked.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn fuzz_harness_catches_a_panicking_decoder_instead_of_crashing_the_process() {
        let always_panics: fn(&[u8]) -> Result<(), String> = |_| panic!("intentional panic for harness self-test");
        let report = fuzz_truncation(&[1, 2, 3, 4], CorruptionLevel::Quick, always_panics);
        assert_eq!(report.cases_panicked.len() as u64, report.cases_run, "every case should have been caught as a panic");
        assert!(report.cases_panicked.iter().all(|msg| msg.contains("intentional panic")), "panic message should be captured: {:?}", report.cases_panicked);
    }
    //#endregion 🔖️Corrupt

    //#region 🔖️Golden
    #[semio_framework_async_macros::async_test]
    async fn golden_hash_hex_is_deterministic_and_sensitive_to_content() {
        let a = golden_hash_hex(b"hello pack").await;
        let b = golden_hash_hex(b"hello pack").await;
        let c = golden_hash_hex(b"hello pack!").await;
        assert_eq!(a, b, "golden_hash_hex must be deterministic for the same bytes");
        assert_ne!(a, c, "golden_hash_hex must differ for different bytes");
        assert_eq!(a.len(), 64, "blake3 hex digest is 64 hex chars (32 bytes)");
        assert!(a.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()), "golden_hash_hex must be lowercase hex: {a}");
    }

    #[semio_framework_async_macros::async_test]
    async fn golden_hash_hex_matches_a_real_encoded_document_across_two_encodes() {
        let spec = mixed_spec();
        let record = RecordValueGen::new(5).generate(&spec, 4);
        let a = crate::os_pack::encode_document(&spec, &record, &crate::os_pack::EncodeOptions::default()).unwrap();
        let b = crate::os_pack::encode_document(&spec, &record, &crate::os_pack::EncodeOptions::default()).unwrap();
        assert_eq!(golden_hash_hex(&a).await, golden_hash_hex(&b).await, "golden hash of a canonical encoding must be stable across repeated encodes");
    }
    //#endregion 🔖️Golden

    //#region 🔖️Long
    mod long {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn fuzz_truncation_and_bit_flips_never_panic_at_long_density() {
            let spec = mixed_spec();
            let record = RecordValueGen::new(21).generate(&spec, 5);
            let bytes = crate::os_pack::encode_document(&spec, &record, &crate::os_pack::EncodeOptions::default()).expect("encode_document");
            let truncation_report = fuzz_truncation(&bytes, CorruptionLevel::Long, decode_closure(spec.clone()));
            let bit_flip_report = fuzz_bit_flips(&bytes, CorruptionLevel::Long, decode_closure(spec));
            assert!(truncation_report.cases_panicked.is_empty(), "long-level truncation fuzz observed panics: {:?}", truncation_report.cases_panicked);
            assert!(bit_flip_report.cases_panicked.is_empty(), "long-level bit-flip fuzz observed panics: {:?}", bit_flip_report.cases_panicked);
            assert!(truncation_report.cases_run >= bytes.len().min(128) as u64);
            assert!(bit_flip_report.cases_run >= 128);
        }
    }
    //#endregion 🔖️Long

    //#region 🔖️Exhaustive
    mod exhaustive {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn fuzz_truncation_never_panics_at_every_single_byte_offset() {
            let spec = mixed_spec();
            let record = RecordValueGen::new(33).generate(&spec, 5);
            let bytes = crate::os_pack::encode_document(&spec, &record, &crate::os_pack::EncodeOptions::default()).expect("encode_document");
            let report = fuzz_truncation(&bytes, CorruptionLevel::Exhaustive, decode_closure(spec));
            assert!(report.cases_panicked.is_empty(), "exhaustive truncation fuzz observed panics: {:?}", report.cases_panicked);
            assert_eq!(report.cases_run, bytes.len() as u64, "exhaustive truncation must try every offset");
        }

        #[semio_framework_async_macros::async_test]
        async fn fuzz_bit_flips_never_panics_at_every_single_bit() {
            let spec = mixed_spec();
            let record = RecordValueGen::new(34).generate(&spec, 3);
            let bytes = crate::os_pack::encode_document(&spec, &record, &crate::os_pack::EncodeOptions::default()).expect("encode_document");
            let report = fuzz_bit_flips(&bytes, CorruptionLevel::Exhaustive, decode_closure(spec));
            assert!(report.cases_panicked.is_empty(), "exhaustive bit-flip fuzz observed panics: {:?}", report.cases_panicked);
            assert_eq!(report.cases_run, bytes.len() as u64 * 8, "exhaustive bit-flip must try every bit");
        }
    }
    //#endregion 🔖️Exhaustive
}
//#endregion 🧪️Tests
