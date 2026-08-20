//! 🧬️ SemioVideoMutation — video mutation dispatch. Every variant's `diff()` is handcrafted
//! (never apply-and-capture) and every variant's `inverse()` is handcrafted, index-aware.

use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};
use crate::artifacts::semio::standards::v1::subsets::video::schema::diff::{
    dec_bool, dec_kind, dec_list, dec_rational, dec_sample, dec_str, dec_stream, diff_insert_sample, diff_insert_stream, diff_remove_sample, diff_remove_stream, diff_set_sample_data, diff_set_sample_flags, diff_set_snapshot, diff_set_stream_meta,
    enc_bool, enc_kind, enc_list, enc_rational, enc_sample, enc_str, enc_stream, hex_decode, hex_encode, parse_usize, SemioVideoDiff,
};
use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoSnapshot, SemioVideoStream, SemioVideoStreamKind};
use protocol::OpBinary;
use protocol::{Mutation, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.semio.video`. Beyond the baseline `{NoMutation,
/// SetSnapshot}`, this addresses `streams` by index and, within a stream, `samples` by index —
/// the same index-only addressing scheme the diff grammar uses (neither collection carries a
/// spec-mandated key). No `#[derive(dsl::DslOps)]` attempted (this ticket's own instruction:
/// hand-roll all op codecs) — `SetSnapshot{snapshot: SemioVideoSnapshot}` alone would hit the same
/// `Vec<SemioVideoStream>`-of-`Vec<SemioVideoSample>` nesting the diff side's own doc comment
/// documents as blocking a derive attempt; `OpText`/`OpBinary` are hand-rolled below instead.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SemioVideoMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: SemioVideoSnapshot,
    },
    /// ➕️ Inserts `stream` at `index` (FINAL state).
    InsertStream {
        index: usize,
        stream: SemioVideoStream,
    },
    /// ➖️ Removes the stream at `index` (BASE-state index).
    RemoveStream {
        index: usize,
    },
    /// ✍️ Sets the container-level metadata (kind/codec/dimensions/rate) of the stream at `index`.
    SetStreamMeta {
        index: usize,
        kind: SemioVideoStreamKind,
        codec: String,
        width: u32,
        height: u32,
        rate: SemioRational,
    },
    /// ➕️ Inserts `sample` at `index` within the stream at `stream_index` (FINAL state).
    InsertSample {
        stream_index: usize,
        index: usize,
        sample: SemioVideoSample,
    },
    /// ➖️ Removes the sample at `index` (BASE-state index) within `stream_index`.
    RemoveSample {
        stream_index: usize,
        index: usize,
    },
    /// ✍️ Replaces one sample's opaque payload.
    SetSampleData {
        stream_index: usize,
        index: usize,
        data: Vec<u8>,
    },
    /// 🏳️ Sets one sample's `pts`/`key` flags.
    SetSampleFlags {
        stream_index: usize,
        index: usize,
        pts: u64,
        key: bool,
    },
}
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
//#endregion 🔖️Apply

//#region 🔖️Helpers
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn stream_at<'a>(base: &'a SemioVideoSnapshot, index: usize) -> Option<&'a SemioVideoStream> {
    base.streams.get(index)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sample_at<'a>(base: &'a SemioVideoSnapshot, stream_index: usize, index: usize) -> Option<&'a SemioVideoSample> {
    base.streams.get(stream_index)?.samples.get(index)
}
//#endregion 🔖️Helpers

//#region 🔖️MutationTrait
impl Mutation<SemioVideoSnapshot> for SemioVideoMutation {
    type Diff = SemioVideoDiff;

    async fn diff(&self, base: &SemioVideoSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            SemioVideoMutation::NoMutation => SemioVideoDiff::default(),
            SemioVideoMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            SemioVideoMutation::InsertStream { index, stream } => diff_insert_stream(*index, stream.clone()),
            SemioVideoMutation::RemoveStream { index } => diff_remove_stream(*index),
            SemioVideoMutation::SetStreamMeta { index, kind, codec, width, height, rate } => match stream_at(base, *index) {
                Some(old) => diff_set_stream_meta(old, *index, *kind, codec, *width, *height, *rate),
                None => SemioVideoDiff::default(),
            },
            SemioVideoMutation::InsertSample { stream_index, index, sample } => diff_insert_sample(*stream_index, *index, sample.clone()),
            SemioVideoMutation::RemoveSample { stream_index, index } => diff_remove_sample(*stream_index, *index),
            SemioVideoMutation::SetSampleData { stream_index, index, data } => match sample_at(base, *stream_index, *index) {
                Some(old) => diff_set_sample_data(old, *stream_index, *index, data.clone()),
                None => SemioVideoDiff::default(),
            },
            SemioVideoMutation::SetSampleFlags { stream_index, index, pts, key } => match sample_at(base, *stream_index, *index) {
                Some(old) => diff_set_sample_flags(old, *stream_index, *index, *pts, *key),
                None => SemioVideoDiff::default(),
            },
        }).await
    }

    async fn inverse(&self, base: &SemioVideoSnapshot) -> Vec<Self> {
        match self {
            SemioVideoMutation::NoMutation => vec![SemioVideoMutation::NoMutation],
            SemioVideoMutation::SetSnapshot { .. } => vec![SemioVideoMutation::SetSnapshot { snapshot: base.clone() }],
            SemioVideoMutation::InsertStream { index, .. } => vec![SemioVideoMutation::RemoveStream { index: *index }],
            SemioVideoMutation::RemoveStream { index } => match stream_at(base, *index) {
                Some(stream) => vec![SemioVideoMutation::InsertStream { index: *index, stream: stream.clone() }],
                None => vec![SemioVideoMutation::NoMutation],
            },
            SemioVideoMutation::SetStreamMeta { index, .. } => match stream_at(base, *index) {
                Some(stream) => vec![SemioVideoMutation::SetStreamMeta { index: *index, kind: stream.kind, codec: stream.codec.clone(), width: stream.width, height: stream.height, rate: stream.rate }],
                None => vec![SemioVideoMutation::NoMutation],
            },
            SemioVideoMutation::InsertSample { stream_index, index, .. } => vec![SemioVideoMutation::RemoveSample { stream_index: *stream_index, index: *index }],
            SemioVideoMutation::RemoveSample { stream_index, index } => match sample_at(base, *stream_index, *index) {
                Some(sample) => vec![SemioVideoMutation::InsertSample { stream_index: *stream_index, index: *index, sample: sample.clone() }],
                None => vec![SemioVideoMutation::NoMutation],
            },
            SemioVideoMutation::SetSampleData { stream_index, index, .. } => match sample_at(base, *stream_index, *index) {
                Some(sample) => vec![SemioVideoMutation::SetSampleData { stream_index: *stream_index, index: *index, data: sample.data.clone() }],
                None => vec![SemioVideoMutation::NoMutation],
            },
            SemioVideoMutation::SetSampleFlags { stream_index, index, .. } => match sample_at(base, *stream_index, *index) {
                Some(sample) => vec![SemioVideoMutation::SetSampleFlags { stream_index: *stream_index, index: *index, pts: sample.pts, key: sample.key }],
                None => vec![SemioVideoMutation::NoMutation],
            },
        }
    }
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
        SemioVideoMutation::NoMutation => "no-mutation".to_string(),
        SemioVideoMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_semio_video_snapshot(snapshot)),
        SemioVideoMutation::InsertStream { index, stream } => format!("insert-stream index={} stream={}", index, enc_stream(stream)),
        SemioVideoMutation::RemoveStream { index } => format!("remove-stream index={index}"),
        SemioVideoMutation::SetStreamMeta { index, kind, codec, width, height, rate } => format!("set-stream-meta index={} kind={} codec={} width={} height={} rate={}", index, enc_kind(kind), enc_str(codec), width, height, enc_rational(rate)),
        SemioVideoMutation::InsertSample { stream_index, index, sample } => format!("insert-sample stream-index={} index={} sample={}", stream_index, index, enc_sample(sample)),
        SemioVideoMutation::RemoveSample { stream_index, index } => format!("remove-sample stream-index={stream_index} index={index}"),
        SemioVideoMutation::SetSampleData { stream_index, index, data } => format!("set-sample-data stream-index={} index={} data={}", stream_index, index, hex_encode(data)),
        SemioVideoMutation::SetSampleFlags { stream_index, index, pts, key } => format!("set-sample-flags stream-index={} index={} pts={} key={}", stream_index, index, pts, enc_bool(key)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_semio_video_mutation(line: &str) -> Result<SemioVideoMutation, String> {
    if line == "no-mutation" {
        return Ok(SemioVideoMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> =
        rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("semio video mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("semio video mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { parse_usize(arg(k)?) };
    match keyword {
        "set-snapshot" => Ok(SemioVideoMutation::SetSnapshot { snapshot: dec_semio_video_snapshot(arg("snapshot")?)? }),
        "insert-stream" => Ok(SemioVideoMutation::InsertStream { index: usize_arg("index")?, stream: dec_stream(arg("stream")?)? }),
        "remove-stream" => Ok(SemioVideoMutation::RemoveStream { index: usize_arg("index")? }),
        "set-stream-meta" => Ok(SemioVideoMutation::SetStreamMeta {
            index: usize_arg("index")?,
            kind: dec_kind(arg("kind")?)?,
            codec: dec_str(arg("codec")?)?,
            width: arg("width")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
            height: arg("height")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
            rate: dec_rational(arg("rate")?)?,
        }),
        "insert-sample" => Ok(SemioVideoMutation::InsertSample { stream_index: usize_arg("stream-index")?, index: usize_arg("index")?, sample: dec_sample(arg("sample")?)? }),
        "remove-sample" => Ok(SemioVideoMutation::RemoveSample { stream_index: usize_arg("stream-index")?, index: usize_arg("index")? }),
        "set-sample-data" => Ok(SemioVideoMutation::SetSampleData { stream_index: usize_arg("stream-index")?, index: usize_arg("index")?, data: hex_decode(arg("data")?)? }),
        "set-sample-flags" => Ok(SemioVideoMutation::SetSampleFlags { stream_index: usize_arg("stream-index")?, index: usize_arg("index")?, pts: arg("pts")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, key: dec_bool(arg("key")?)? }),
        other => Err(format!("semio video mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SemioVideoMutation {
    async fn print_op(&self) -> String {
        print_semio_video_mutation(self)
    }
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_semio_video_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// 🏷️ Ordinal table, same declaration order as `SemioVideoMutation`'s own enum variants and
/// `parse_semio_video_mutation`'s keyword match — the real binary `tag` field's source of truth.
const OP_KEYWORDS: [&str; 9] = ["no-mutation", "set-snapshot", "insert-stream", "remove-stream", "set-stream-meta", "insert-sample", "remove-sample", "set-sample-data", "set-sample-flags"];
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn variant_ordinal(m: &SemioVideoMutation) -> u8 {
    match m {
        SemioVideoMutation::NoMutation => 0,
        SemioVideoMutation::SetSnapshot { .. } => 1,
        SemioVideoMutation::InsertStream { .. } => 2,
        SemioVideoMutation::RemoveStream { .. } => 3,
        SemioVideoMutation::SetStreamMeta { .. } => 4,
        SemioVideoMutation::InsertSample { .. } => 5,
        SemioVideoMutation::RemoveSample { .. } => 6,
        SemioVideoMutation::SetSampleData { .. } => 7,
        SemioVideoMutation::SetSampleFlags { .. } => 8,
    }
}
/// ✂️ Just the `key=value ...` argument tail of `print_semio_video_mutation` (empty for
/// `no-mutation`) — the binary frame's `tag` byte already carries the keyword, so the text keyword
/// itself is redundant in the binary payload.
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
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, variant_ordinal(self)];
        out.extend_from_slice(print_semio_video_mutation_args(self).as_bytes());
        Ok(out)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
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
        Self::parse_op(&line).await.map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region 🔖️Demo
/// 🌱 Representative `SemioVideoMutation` cases (one per variant, `pub(crate)` module-scope) for
/// the conformance-law tests — delegates to the existing test module's own `sample_mutations()`
/// (byte-identical) rather than keep an independent copy, same dedupe flow's/mesh's own waves
/// perform.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioVideoMutation> {
    tests::sample_mutations()
}
//#endregion 🔖️Demo

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::command::DiffAlgebra;
    use protocol::MutationDiff;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fixture() -> SemioVideoSnapshot {
        SemioVideoSnapshot {
            schema: crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
            streams: vec![
                SemioVideoStream {
                    kind: SemioVideoStreamKind::Video,
                    codec: "h264".into(),
                    width: 1920,
                    height: 1080,
                    rate: SemioRational { num: 30, den: 1 },
                    samples: vec![SemioVideoSample { pts: 0, key: true, data: vec![1, 2, 3] }, SemioVideoSample { pts: 33, key: false, data: vec![4, 5, 6] }],
                },
                SemioVideoStream { kind: SemioVideoStreamKind::Audio, codec: "aac".into(), width: 0, height: 0, rate: SemioRational { num: 48_000, den: 1_000 }, samples: Vec::new() },
            ],
        }
    }

    //#region 🔖️Fixtures
    /// 🌱 `sweep_a`/`sweep_b` — differ in EVERY mutable field, at BOTH nesting levels. `streams`
    /// uses different-length lists so the recipe's naive positional `between_indexed` shows
    /// removed+modified simultaneously in one direction (a removed tail, a modified-in-every-field
    /// first stream whose OWN nested `samples` shows removed+modified too) and added in the
    /// reverse direction (the dropped stream, plus that same modified stream's nested `samples`
    /// added) — same "known structural trap" technique docx's own sweep fixtures use.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_a() -> SemioVideoSnapshot {
        SemioVideoSnapshot {
            schema: crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
            streams: vec![
                SemioVideoStream {
                    kind: SemioVideoStreamKind::Video,
                    codec: "old-codec".into(),
                    width: 640,
                    height: 480,
                    rate: SemioRational { num: 24, den: 1 },
                    samples: vec![SemioVideoSample { pts: 1, key: true, data: vec![9] }, SemioVideoSample { pts: 2, key: false, data: vec![8] }, SemioVideoSample { pts: 3, key: true, data: vec![7] }],
                },
                SemioVideoStream { kind: SemioVideoStreamKind::Audio, codec: "aac".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() },
                SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "srt".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() },
            ],
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_b() -> SemioVideoSnapshot {
        SemioVideoSnapshot {
            schema: crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
            streams: vec![
                SemioVideoStream {
                    kind: SemioVideoStreamKind::Audio,
                    codec: "new-codec".into(),
                    width: 1280,
                    height: 720,
                    rate: SemioRational { num: 30, den: 1 },
                    samples: vec![SemioVideoSample { pts: 1, key: true, data: vec![9] }, SemioVideoSample { pts: 22, key: true, data: vec![80] }],
                },
                SemioVideoStream { kind: SemioVideoStreamKind::Audio, codec: "aac".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() },
            ],
        }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️MutationDiffLaw
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn sample_mutations() -> Vec<SemioVideoMutation> {
        vec![
            SemioVideoMutation::NoMutation,
            SemioVideoMutation::SetSnapshot { snapshot: sweep_b() },
            SemioVideoMutation::InsertStream { index: 1, stream: SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "srt".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() } },
            SemioVideoMutation::RemoveStream { index: 0 },
            SemioVideoMutation::SetStreamMeta { index: 0, kind: SemioVideoStreamKind::Audio, codec: "vp9".into(), width: 1280, height: 720, rate: SemioRational { num: 60, den: 1 } },
            SemioVideoMutation::InsertSample { stream_index: 0, index: 1, sample: SemioVideoSample { pts: 99, key: true, data: vec![9, 9] } },
            SemioVideoMutation::RemoveSample { stream_index: 0, index: 0 },
            SemioVideoMutation::SetSampleData { stream_index: 0, index: 0, data: vec![42] },
            SemioVideoMutation::SetSampleFlags { stream_index: 0, index: 0, pts: 500, key: true },
        ]
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn apply_valid(diff: &SemioVideoDiff, base: &SemioVideoSnapshot) -> SemioVideoSnapshot {
        MutationDiff::apply(diff, base).expect("valid Semio video diff fixture")
    }

    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        for mutation in sample_mutations() {
            let base = fixture();
            let diff_direct = Mutation::diff(&mutation, &base);
            let applied_via_diff = apply_valid(diff_direct.diff(), &base);

            let mut via_apply = base.clone();
            let diff_from_apply = apply_semio_video_mutation(&mut via_apply, &mutation);

            assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
            assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        for mutation in sample_mutations() {
            let base = fixture();

            let mut round_tripped = base.clone();
            apply_semio_video_mutation(&mut round_tripped, &mutation);
            for inverse_mutation in <SemioVideoMutation as Mutation<SemioVideoSnapshot>>::inverse(&mutation, &base) {
                apply_semio_video_mutation(&mut round_tripped, &inverse_mutation);
            }
            assert_eq!(round_tripped, base, "inverse_law (mutation-level) failed for {mutation:?}");

            let diff = Mutation::diff(&mutation, &base);
            let next = apply_valid(diff.diff(), &base);
            let inverse_diff = DiffAlgebra::inverse(diff.diff(), &base);
            let restored = apply_valid(&inverse_diff, &next);
            assert_eq!(restored, base, "inverse_law (diff-level) failed for {mutation:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️AbsorbLaw
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn assert_absorb_matches_sequential(base: &SemioVideoSnapshot, d1: &SemioVideoDiff, d2: &SemioVideoDiff) -> SemioVideoDiff {
        let sequential = apply_valid(d2, &apply_valid(d1, base));
        let mut absorbed = d1.clone();
        MutationDiff::absorb(&mut absorbed, d2.clone());
        assert_eq!(apply_valid(&absorbed, base), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
        absorbed
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn streams_diff(diff: &SemioVideoDiff) -> &crate::artifacts::semio::standards::v1::subsets::video::schema::diff::SemioVideoStreamsDiff {
        diff.streams.as_ref().expect("streams diff present")
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law() {
        // Canonical: Insert(2)+Remove(0) -> {removed:[0], added:[(1,f)]}.
        {
            let base = fixture();
            let f = SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "f".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() };
            let d1 = Mutation::diff(&SemioVideoMutation::InsertStream { index: 2, stream: f.clone() }, &base);
            let mid = apply_valid(d1.diff(), &base);
            let d2 = Mutation::diff(&SemioVideoMutation::RemoveStream { index: 0 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
            let triple = streams_diff(&absorbed);
            assert_eq!(triple.removed, vec![0]);
            assert_eq!(triple.added.len(), 1);
            assert_eq!(triple.added[0].index, 1);
            assert_eq!(triple.added[0].item, f);
        }

        // Canonical: Insert(2,f)+Insert(2,g) -> both survive.
        {
            let base = fixture();
            let f = SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "f".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() };
            let g = SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "g".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() };
            let d1 = Mutation::diff(&SemioVideoMutation::InsertStream { index: 2, stream: f.clone() }, &base);
            let mid = apply_valid(d1.diff(), &base);
            let d2 = Mutation::diff(&SemioVideoMutation::InsertStream { index: 2, stream: g.clone() }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
            let triple = streams_diff(&absorbed);
            assert_eq!(triple.added.len(), 2, "both inserts must survive absorb, not LWW-clobber");
            assert!(triple.added.iter().any(|a| a.item == f));
            assert!(triple.added.iter().any(|a| a.item == g));
        }

        // Canonical: Insert(1,f)+SetField(1,v) -> patch into the added payload.
        {
            let base = fixture();
            let f = SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "f".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() };
            let d1 = Mutation::diff(&SemioVideoMutation::InsertStream { index: 1, stream: f.clone() }, &base);
            let mid = apply_valid(d1.diff(), &base);
            let d2 = Mutation::diff(&SemioVideoMutation::SetStreamMeta { index: 1, kind: SemioVideoStreamKind::Audio, codec: "patched".into(), width: 1, height: 1, rate: SemioRational { num: 2, den: 1 } }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
            let triple = streams_diff(&absorbed);
            assert!(triple.modified.is_empty(), "patch-into-added must not surface as a separate modified entry");
            assert_eq!(triple.added.len(), 1);
            assert_eq!(triple.added[0].item.codec, "patched");
            assert_eq!(triple.added[0].item.kind, SemioVideoStreamKind::Audio);
        }

        // Canonical: Modify+Remove -> the modify is annihilated by the later remove.
        {
            let base = fixture();
            let d1 = Mutation::diff(&SemioVideoMutation::SetStreamMeta { index: 1, kind: SemioVideoStreamKind::Video, codec: "patched".into(), width: 1, height: 1, rate: SemioRational { num: 2, den: 1 } }, &base);
            let mid = apply_valid(d1.diff(), &base);
            let d2 = Mutation::diff(&SemioVideoMutation::RemoveStream { index: 1 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
            let triple = streams_diff(&absorbed);
            assert!(triple.modified.is_empty(), "modify of a since-removed item must not survive absorb");
            assert_eq!(triple.removed, vec![1]);
        }

        // Associativity over a triple.
        {
            let base = fixture();
            let f = SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "f".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() };
            let g = SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "g".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() };
            let d1 = Mutation::diff(&SemioVideoMutation::InsertStream { index: 2, stream: f }, &base);
            let mid1 = apply_valid(d1.diff(), &base);
            let d2 = Mutation::diff(&SemioVideoMutation::InsertStream { index: 2, stream: g }, &mid1);
            let mid2 = apply_valid(d2.diff(), &mid1);
            let d3 = Mutation::diff(&SemioVideoMutation::RemoveStream { index: 0 }, &mid2);
            let sequential = apply_valid(d3.diff(), &mid2);

            let mut left = d1.diff().clone();
            MutationDiff::absorb(&mut left, d2.diff().clone());
            MutationDiff::absorb(&mut left, d3.diff().clone());

            let mut d2_then_d3 = d2.diff().clone();
            MutationDiff::absorb(&mut d2_then_d3, d3.diff().clone());
            let mut right = d1.diff().clone();
            MutationDiff::absorb(&mut right, d2_then_d3);

            assert_eq!(apply_valid(&left, &base), sequential, "absorb associativity (left) failed");
            assert_eq!(apply_valid(&right, &base), sequential, "absorb associativity (right) failed");
        }
    }
    //#endregion 🔖️AbsorbLaw

    //#region 🔖️BetweenRoundtripLaw
    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        assert_eq!(apply_valid(&<SemioVideoDiff as DiffAlgebra<SemioVideoSnapshot>>::between(&a, &b), &a), b);
        assert_eq!(apply_valid(&<SemioVideoDiff as DiffAlgebra<SemioVideoSnapshot>>::between(&b, &a), &b), a);

        let sample = fixture();
        assert_eq!(apply_valid(&<SemioVideoDiff as DiffAlgebra<SemioVideoSnapshot>>::between(&sample, &sample), &sample), sample);

        // "Real" fixture leg: a realistic 2-stream snapshot diffed against a mutated variant.
        let real = fixture();
        let mut mutated = real.clone();
        apply_semio_video_mutation(&mut mutated, &SemioVideoMutation::SetSampleFlags { stream_index: 0, index: 0, pts: 1_000, key: true });
        assert_ne!(real, mutated);
        assert_eq!(apply_valid(&<SemioVideoDiff as DiffAlgebra<SemioVideoSnapshot>>::between(&real, &mutated), &real), mutated);
        assert_eq!(apply_valid(&<SemioVideoDiff as DiffAlgebra<SemioVideoSnapshot>>::between(&mutated, &real), &mutated), real);
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️CodecRetentionLaw
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let snap = fixture();
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <SemioVideoSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
    //#endregion 🔖️CodecRetentionLaw

    //#region 🔖️FieldSweep
    /// 🎯️ THE acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable field at both
    /// nesting levels (see the fixtures' own doc comment for exactly how removed/modified/added
    /// is exercised at each level, and why the two directions of `between()` are both asserted).
    #[semio_framework_async_macros::async_test]
    async fn field_sweep() {
        let a = sweep_a();
        let b = sweep_b();

        let diff_ab = <SemioVideoDiff as DiffAlgebra<SemioVideoSnapshot>>::between(&a, &b);
        assert_eq!(apply_valid(&diff_ab, &a), b);
        let diff_ba = <SemioVideoDiff as DiffAlgebra<SemioVideoSnapshot>>::between(&b, &a);
        assert_eq!(apply_valid(&diff_ba, &b), a);
        assert!(<SemioVideoDiff as DiffAlgebra<SemioVideoSnapshot>>::between(&a, &a).is_empty());

        // a -> b: streams.removed (dropped subtitle stream) + streams.modified[0] (every scalar
        // field changed, incl. the SemioVideoStreamKind enum) whose OWN nested samples diff shows
        // removed + modified simultaneously.
        let streams_diff_ab = diff_ab.streams.as_ref().expect("streams diff present");
        assert!(!streams_diff_ab.removed.is_empty(), "streams: removed not exercised");
        assert_eq!(streams_diff_ab.modified.len(), 1);
        let stream_mod = &streams_diff_ab.modified[0].diff;
        assert!(stream_mod.kind.is_some(), "modified stream: kind (enum) not exercised");
        assert!(stream_mod.codec.is_some(), "modified stream: codec not exercised");
        assert!(stream_mod.width.is_some(), "modified stream: width not exercised");
        assert!(stream_mod.height.is_some(), "modified stream: height not exercised");
        assert!(stream_mod.rate.is_some(), "modified stream: rate not exercised");
        let samples_diff = stream_mod.samples.as_ref().expect("modified stream: samples diff not exercised");
        assert!(!samples_diff.removed.is_empty(), "samples: removed not exercised");
        assert!(!samples_diff.modified.is_empty(), "samples: modified not exercised");
        let sample_mod = &samples_diff.modified[0].diff;
        assert!(sample_mod.pts.is_some() && sample_mod.key.is_some() && sample_mod.data.is_some(), "modified sample: not every field exercised");

        // b -> a: the OTHER direction's top-level `added` (the very same dropped subtitle stream)
        // plus that same stream's nested `samples.added`.
        let streams_diff_ba = diff_ba.streams.as_ref().expect("streams diff (b->a) present");
        assert!(!streams_diff_ba.added.is_empty(), "streams (b->a): added not exercised");
        let stream_mod_ba = &streams_diff_ba.modified[0].diff;
        let samples_diff_ba = stream_mod_ba.samples.as_ref().expect("samples diff (b->a) present");
        assert!(!samples_diff_ba.added.is_empty(), "samples (b->a): added not exercised");
    }
    //#endregion 🔖️FieldSweep

    //#region 🔖️OpTextBinaryRoundtripLaw
    /// 🧪️ `OpText`/`OpBinary` round-trip laws for the hand-rolled `SemioVideoMutation` grammar —
    /// exercises every variant, incl. `InsertStream`'s bare `SemioVideoStream` payload (with
    /// nested samples), `SetSnapshot`'s whole `SemioVideoSnapshot`, and the `SemioVideoStreamKind`
    /// enum tag.
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        let stream = SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "srt".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: vec![SemioVideoSample { pts: 5, key: true, data: vec![1, 2] }] };
        let mutations = vec![
            SemioVideoMutation::NoMutation,
            SemioVideoMutation::SetSnapshot { snapshot: sweep_b() },
            SemioVideoMutation::InsertStream { index: 1, stream: stream.clone() },
            SemioVideoMutation::RemoveStream { index: 0 },
            SemioVideoMutation::SetStreamMeta { index: 0, kind: SemioVideoStreamKind::Audio, codec: "hello world".into(), width: 7, height: 9, rate: SemioRational { num: 25, den: 1 } },
            SemioVideoMutation::InsertSample { stream_index: 0, index: 0, sample: SemioVideoSample { pts: 1, key: false, data: vec![0, 255] } },
            SemioVideoMutation::RemoveSample { stream_index: 0, index: 0 },
            SemioVideoMutation::SetSampleData { stream_index: 0, index: 0, data: vec![1, 2, 3, 4] },
            SemioVideoMutation::SetSampleFlags { stream_index: 0, index: 0, pts: 12345, key: true },
        ];
        for mutation in mutations {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = SemioVideoMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = SemioVideoMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️OpTextBinaryRoundtripLaw
}
//#endregion 🧪️Tests
