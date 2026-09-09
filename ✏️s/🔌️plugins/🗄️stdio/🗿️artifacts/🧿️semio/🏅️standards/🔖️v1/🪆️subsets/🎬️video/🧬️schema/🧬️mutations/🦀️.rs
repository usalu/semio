//! 🧬️ SemioVideoMutation — video mutation dispatch. Every variant's `diff()` is handcrafted
//! (never apply-and-capture) and every variant's `inverse()` is handcrafted, index-aware.
//!
//! `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires every variant to wrap exactly one
//! leaf payload, and its sentinel verb `no` is not in `APPROVED_VERBS` — see
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs`. Every variant is now a
//! tuple variant wrapping its own mutation leaf (`./*/🦀️.rs`), and this file's `agg_diff`/
//! `agg_inverse` carry the handcrafted semantics every leaf's `MutationKind` impl delegates back to.

use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use crate::standards::v1::subsets::video::schema::diff::{
    dec_bool, dec_kind, dec_list, dec_rational, dec_sample, dec_str, dec_stream, diff_insert_sample, diff_insert_stream, diff_remove_sample, diff_remove_stream, diff_set_sample_data, diff_set_sample_flags, diff_set_snapshot, diff_set_stream_meta,
    enc_bool, enc_kind, enc_list, enc_rational, enc_sample, enc_str, enc_stream, hex_decode, hex_encode, parse_usize, SemioVideoDiff,
};
use crate::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoSnapshot, SemioVideoStream, SemioVideoStreamKind};
use protocol::OpBinary;
use protocol::{Mutation, OpText};

//#region 🔖️Mutations
#[path = "➕️insert-sample/🦀️.rs"]
pub mod insert_sample;
#[path = "🎥insert-stream/🦀️.rs"]
pub mod insert_stream;
#[path = "🚮remove-sample/🦀️.rs"]
pub mod remove_sample;
#[path = "🗑️remove-stream/🦀️.rs"]
pub mod remove_stream;
#[path = "📀set-sample-data/🦀️.rs"]
pub mod set_sample_data;
#[path = "🚩set-sample-flags/🦀️.rs"]
pub mod set_sample_flags;
/// 📐️ Typed content mutation for `stdio.semio.video`. Beyond the baseline `SetSnapshot`, this
/// addresses `streams` by index and, within a stream, `samples` by index — the same index-only
/// addressing scheme the diff grammar uses (neither collection carries a spec-mandated key). No
/// `#[derive(dsl::DslOps)]` attempted (this ticket's own instruction: hand-roll all op codecs) —
/// `SetSnapshot{snapshot: SemioVideoSnapshot}` alone would hit the same
/// `Vec<SemioVideoStream>`-of-`Vec<SemioVideoSample>` nesting the diff side's own doc comment
/// documents as blocking a derive attempt; `OpText`/`OpBinary` are hand-rolled below instead.
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "📋set-stream-meta/🦀️.rs"]
pub mod set_stream_meta;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SemioVideoSnapshot, diff = SemioVideoDiff, schema = "SemioVideoMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioVideoMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    /// ➕️ Inserts `stream` at `index` (FINAL state).
    InsertStream(insert_stream::InsertStream),
    /// ➖️ Removes the stream at `index` (BASE-state index).
    RemoveStream(remove_stream::RemoveStream),
    /// ✍️ Sets the container-level metadata (kind/codec/dimensions/rate) of the stream at `index`.
    SetStreamMeta(set_stream_meta::SetStreamMeta),
    /// ➕️ Inserts `sample` at `index` within the stream at `stream_index` (FINAL state).
    InsertSample(insert_sample::InsertSample),
    /// ➖️ Removes the sample at `index` (BASE-state index) within `stream_index`.
    RemoveSample(remove_sample::RemoveSample),
    /// ✍️ Replaces one sample's opaque payload.
    SetSampleData(set_sample_data::SetSampleData),
    /// 🏳️ Sets one sample's `pts`/`key` flags.
    SetSampleFlags(set_sample_flags::SetSampleFlags),
}

/// 🏷️ The declared kebab-case mutation vocabulary of `s.stdio.semio.video`, in enum declaration
/// order — what the `🎥️mutate-semio-video` case's completeness gate counts against and what
/// `../../🔣️oracle.json`'s catalog repeats. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this declaration honest.
pub const KINDS: &[&str] = &["set-snapshot", "insert-stream", "remove-stream", "set-stream-meta", "insert-sample", "remove-sample", "set-sample-data", "set-sample-flags"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` -- the diff is the single semantics source, never a separate imperative
/// apply path (apply-and-capture is banned).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_semio_video_mutation(snapshot: &mut SemioVideoSnapshot, mutation: &SemioVideoMutation) -> protocol::MutationOutcome<SemioVideoDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Free-function face of [`SemioVideoMutation`]'s own `protocol::Mutation::inverse`. `Mutation` is
/// declared by the os-kernel, which is an INTERNAL dependency of this plugin (aliased `protocol` in
/// `🦀️.rs`) and is therefore not nameable by a consumer that links only this crate — a
/// generated test host being the concrete case. Paired with [`apply_semio_video_mutation`] it makes the
/// undo law reachable without importing a trait the caller cannot name.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_semio_video_mutation(mutation: &SemioVideoMutation, base: &SemioVideoSnapshot) -> Vec<SemioVideoMutation> {
    <SemioVideoMutation as Mutation<SemioVideoSnapshot>>::inverse(mutation, base)
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn stream_at(base: &SemioVideoSnapshot, index: usize) -> Option<&SemioVideoStream> {
    base.streams.get(index)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sample_at(base: &SemioVideoSnapshot, stream_index: usize, index: usize) -> Option<&SemioVideoSample> {
    base.streams.get(stream_index)?.samples.get(index)
}
//#endregion 🔖️Helpers

//#region 🔖️MutationTrait
/// ↩️ An index that no longer exists in `base` has nothing to restore, so those arms return the
/// empty inverse rather than a sentinel no-op mutation — the convention this migration adopted once
/// `NoMutation` stopped being an available payload.
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &SemioVideoMutation, base: &SemioVideoSnapshot) -> protocol::MutationOutcome<SemioVideoDiff> {
    protocol::MutationOutcome::new(match this {
        SemioVideoMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        SemioVideoMutation::InsertStream(insert_stream::InsertStream { index, stream }) => diff_insert_stream(*index, stream.clone()),
        SemioVideoMutation::RemoveStream(remove_stream::RemoveStream { index }) => diff_remove_stream(*index),
        SemioVideoMutation::SetStreamMeta(set_stream_meta::SetStreamMeta { index, kind, codec, width, height, rate }) => match stream_at(base, *index) {
            Some(old) => diff_set_stream_meta(old, *index, *kind, codec, *width, *height, *rate),
            None => SemioVideoDiff::default(),
        },
        SemioVideoMutation::InsertSample(insert_sample::InsertSample { stream_index, index, sample }) => diff_insert_sample(*stream_index, *index, sample.clone()),
        SemioVideoMutation::RemoveSample(remove_sample::RemoveSample { stream_index, index }) => diff_remove_sample(*stream_index, *index),
        SemioVideoMutation::SetSampleData(set_sample_data::SetSampleData { stream_index, index, data }) => match sample_at(base, *stream_index, *index) {
            Some(old) => diff_set_sample_data(old, *stream_index, *index, data.clone()),
            None => SemioVideoDiff::default(),
        },
        SemioVideoMutation::SetSampleFlags(set_sample_flags::SetSampleFlags { stream_index, index, pts, key }) => match sample_at(base, *stream_index, *index) {
            Some(old) => diff_set_sample_flags(old, *stream_index, *index, *pts, *key),
            None => SemioVideoDiff::default(),
        },
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &SemioVideoMutation, base: &SemioVideoSnapshot) -> Vec<SemioVideoMutation> {
    vec![match this {
        SemioVideoMutation::SetSnapshot(_) => SemioVideoMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        SemioVideoMutation::InsertStream(insert_stream::InsertStream { index, .. }) => SemioVideoMutation::RemoveStream(remove_stream::RemoveStream { index: *index }),
        SemioVideoMutation::RemoveStream(remove_stream::RemoveStream { index }) => match stream_at(base, *index) {
            Some(stream) => SemioVideoMutation::InsertStream(insert_stream::InsertStream { index: *index, stream: stream.clone() }),
            None => return Vec::new(),
        },
        SemioVideoMutation::SetStreamMeta(set_stream_meta::SetStreamMeta { index, .. }) => match stream_at(base, *index) {
            Some(stream) => SemioVideoMutation::SetStreamMeta(set_stream_meta::SetStreamMeta { index: *index, kind: stream.kind, codec: stream.codec.clone(), width: stream.width, height: stream.height, rate: stream.rate }),
            None => return Vec::new(),
        },
        SemioVideoMutation::InsertSample(insert_sample::InsertSample { stream_index, index, .. }) => SemioVideoMutation::RemoveSample(remove_sample::RemoveSample { stream_index: *stream_index, index: *index }),
        SemioVideoMutation::RemoveSample(remove_sample::RemoveSample { stream_index, index }) => match sample_at(base, *stream_index, *index) {
            Some(sample) => SemioVideoMutation::InsertSample(insert_sample::InsertSample { stream_index: *stream_index, index: *index, sample: sample.clone() }),
            None => return Vec::new(),
        },
        SemioVideoMutation::SetSampleData(set_sample_data::SetSampleData { stream_index, index, .. }) => match sample_at(base, *stream_index, *index) {
            Some(sample) => SemioVideoMutation::SetSampleData(set_sample_data::SetSampleData { stream_index: *stream_index, index: *index, data: sample.data.clone() }),
            None => return Vec::new(),
        },
        SemioVideoMutation::SetSampleFlags(set_sample_flags::SetSampleFlags { stream_index, index, .. }) => match sample_at(base, *stream_index, *index) {
            Some(sample) => SemioVideoMutation::SetSampleFlags(set_sample_flags::SetSampleFlags { stream_index: *stream_index, index: *index, pts: sample.pts, key: sample.key }),
            None => return Vec::new(),
        },
    }]
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🎙️ Hand-rolled `OpText`/`OpBinary` for `SemioVideoMutation` — reuses the diff module's
/// `pub(crate)` grammar primitives (`hex_encode`/`enc_stream`/`enc_sample`/`split_top_level`/...)
/// rather than duplicating them a second time in this file. Grammar: `keyword arg=value ...`
/// (space-separated), same shape docx's own hand-rolled op codec uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_semio_video_snapshot(s: &SemioVideoSnapshot) -> String {
    format!("[{},{}]", enc_str(&s.schema), enc_list(&s.streams, enc_stream))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_semio_video_snapshot(s: &str) -> Result<SemioVideoSnapshot, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [schema, streams] = parts.as_slice() else { return Err(format!("snapshot: expected 2 fields, got {}", parts.len())) };
    Ok(SemioVideoSnapshot { schema: dec_str(schema)?, streams: dec_list(streams, dec_stream)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_semio_video_mutation(m: &SemioVideoMutation) -> String {
    match m {
        SemioVideoMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_semio_video_snapshot(snapshot)),
        SemioVideoMutation::InsertStream(insert_stream::InsertStream { index, stream }) => format!("insert-stream index={} stream={}", index, enc_stream(stream)),
        SemioVideoMutation::RemoveStream(remove_stream::RemoveStream { index }) => format!("remove-stream index={index}"),
        SemioVideoMutation::SetStreamMeta(set_stream_meta::SetStreamMeta { index, kind, codec, width, height, rate }) => {
            format!("set-stream-meta index={} kind={} codec={} width={} height={} rate={}", index, enc_kind(kind), enc_str(codec), width, height, enc_rational(rate))
        }
        SemioVideoMutation::InsertSample(insert_sample::InsertSample { stream_index, index, sample }) => format!("insert-sample stream-index={} index={} sample={}", stream_index, index, enc_sample(sample)),
        SemioVideoMutation::RemoveSample(remove_sample::RemoveSample { stream_index, index }) => format!("remove-sample stream-index={stream_index} index={index}"),
        SemioVideoMutation::SetSampleData(set_sample_data::SetSampleData { stream_index, index, data }) => format!("set-sample-data stream-index={} index={} data={}", stream_index, index, hex_encode(data)),
        SemioVideoMutation::SetSampleFlags(set_sample_flags::SetSampleFlags { stream_index, index, pts, key }) => format!("set-sample-flags stream-index={} index={} pts={} key={}", stream_index, index, pts, enc_bool(key)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_semio_video_mutation(line: &str) -> Result<SemioVideoMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> =
        rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("semio video mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("semio video mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { parse_usize(arg(k)?) };
    match keyword {
        "set-snapshot" => Ok(SemioVideoMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_semio_video_snapshot(arg("snapshot")?)? })),
        "insert-stream" => Ok(SemioVideoMutation::InsertStream(insert_stream::InsertStream { index: usize_arg("index")?, stream: dec_stream(arg("stream")?)? })),
        "remove-stream" => Ok(SemioVideoMutation::RemoveStream(remove_stream::RemoveStream { index: usize_arg("index")? })),
        "set-stream-meta" => Ok(SemioVideoMutation::SetStreamMeta(set_stream_meta::SetStreamMeta {
            index: usize_arg("index")?,
            kind: dec_kind(arg("kind")?)?,
            codec: dec_str(arg("codec")?)?,
            width: arg("width")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
            height: arg("height")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
            rate: dec_rational(arg("rate")?)?,
        })),
        "insert-sample" => Ok(SemioVideoMutation::InsertSample(insert_sample::InsertSample { stream_index: usize_arg("stream-index")?, index: usize_arg("index")?, sample: dec_sample(arg("sample")?)? })),
        "remove-sample" => Ok(SemioVideoMutation::RemoveSample(remove_sample::RemoveSample { stream_index: usize_arg("stream-index")?, index: usize_arg("index")? })),
        "set-sample-data" => Ok(SemioVideoMutation::SetSampleData(set_sample_data::SetSampleData { stream_index: usize_arg("stream-index")?, index: usize_arg("index")?, data: hex_decode(arg("data")?)? })),
        "set-sample-flags" => Ok(SemioVideoMutation::SetSampleFlags(set_sample_flags::SetSampleFlags {
            stream_index: usize_arg("stream-index")?,
            index: usize_arg("index")?,
            pts: arg("pts")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
            key: dec_bool(arg("key")?)?,
        })),
        other => Err(format!("semio video mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SemioVideoMutation {
    fn print_op(&self) -> String {
        print_semio_video_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_semio_video_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// 🏷️ Ordinal table, same declaration order as `SemioVideoMutation`'s own enum variants and
/// `parse_semio_video_mutation`'s keyword match — the real binary `tag` field's source of truth.
const OP_KEYWORDS: [&str; 8] = ["set-snapshot", "insert-stream", "remove-stream", "set-stream-meta", "insert-sample", "remove-sample", "set-sample-data", "set-sample-flags"];
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn variant_ordinal(m: &SemioVideoMutation) -> u8 {
    match m {
        SemioVideoMutation::SetSnapshot(_) => 0,
        SemioVideoMutation::InsertStream(_) => 1,
        SemioVideoMutation::RemoveStream(_) => 2,
        SemioVideoMutation::SetStreamMeta(_) => 3,
        SemioVideoMutation::InsertSample(_) => 4,
        SemioVideoMutation::RemoveSample(_) => 5,
        SemioVideoMutation::SetSampleData(_) => 6,
        SemioVideoMutation::SetSampleFlags(_) => 7,
    }
}
/// ✂️ Just the `key=value ...` argument tail of `print_semio_video_mutation` — the binary frame's
/// `tag` byte already carries the keyword, so the text keyword itself is redundant in the binary
/// payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_semio_video_mutation_args(m: &SemioVideoMutation) -> String {
    match print_semio_video_mutation(m).split_once(' ') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

/// ⚡️ Real binary op frame, replacing the old `print_op().into_bytes()` text-as-binary shortcut
/// (same treatment flow's/mesh's own upgraded mutations facets use). `format u8`
/// (`OP_BINARY_FORMAT` convention) + `tag u8` (the variant ordinal, see [`OP_KEYWORDS`]) are two
/// REAL fixed fields; the variant's own `key=value ...` argument payload follows as one opaque
/// trailing `bytes` chain — reusing the already-real, already-tested `print_semio_video_mutation`/
/// `parse_semio_video_mutation` text codec rather than re-deriving a second independent encoding.
impl OpBinary for SemioVideoMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, variant_ordinal(self)];
        out.extend_from_slice(print_semio_video_mutation_args(self).as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "op header", offset: 0, detail: "truncated (need format+tag)".to_string() });
        }
        if bytes[0] != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {}", bytes[0]) });
        }
        let tag = bytes[1];
        let keyword = OP_KEYWORDS.get(tag as usize).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} out of range for {} declared variants", OP_KEYWORDS.len()) })?;
        let args = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 2, detail: e.to_string() })?;
        let line = if args.is_empty() { keyword.to_string() } else { format!("{keyword} {args}") };
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region 🔖️Demo
/// 🌱 Representative `SemioVideoMutation` cases (one per variant, `pub(crate)` module-scope) for
/// the conformance-law tests — delegates to the existing test module's own `sample_mutations()`
/// (byte-identical) rather than keep an independent copy, same dedupe flow's/mesh's own waves
/// perform.
#[cfg(all(test, feature = "conversion-video"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioVideoMutation> {
    tests::sample_mutations()
}
//#endregion 🔖️Demo

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📸️set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📸️set-snapshot/🧪️tests/⏱️retimes-the-track-and-promotes-a-sample-to-a-keyframe/🦀️.rs"]
mod set_snapshot_retimes_the_track_and_promotes_a_sample_to_a_keyframe;
//#endregion 🧪️FixtureCases
