//! 🧬️ SemioVideoSnapshot — streams{kind, codec, width, height, rate:Rational, samples{pts, key,
//! opaque data}} — container-typed, payload-opaque (honest boundary per the master plan: real,
//! complete metadata for this subset's own shape; the compressed sample bytes themselves are
//! never decoded here — that is W3/W4's container-format job, mp4/avi).
//!
//! 🧩️ `#[derive(dsl::DslArtifact)]` was tried first per this ticket's brief. Blocked the same way
//! image's own bare-`Option<Vec<u8>>`-on-the-snapshot gap generalizes: `SemioVideoStream.samples:
//! Vec<SemioVideoSample>` nests a `Vec<u8>` buffer field (`data`) inside a `Vec<T>`-of-struct field
//! (`streams`) — the derive's `#[dsl(table)]`/`Vec<Record>` support (confirmed by reading the
//! framework's `SceneDocument`/`TableDocument` worked examples) covers one level of id-keyed
//! `Vec<Record>`; it has no tested path for a nested buffer-bearing leaf record two collections
//! deep, the SAME `derive-nested-multi-buffer-record` wall mesh's own report first named. Hand-rolled
//! instead — see this wave's report `mechanism_gaps`.

use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️VideoModel
/// 🎞️ Owned by the `video` subset (per `w1b-type-ownership.md`): `SemioVideoStream`,
/// `SemioVideoSample`, plus this subset's own `SemioVideoStreamKind`/`SemioRational` (not shared
/// engine types — `Rational` is video-specific, unlike `SemioPoint3`/`SemioTransform` etc).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SemioVideoStreamKind {
    #[default]
    Video,
    Audio,
    Subtitle,
}

/// 🎚️ A frame/sample rate as an exact fraction — named struct, never a bare tuple (f6-final-summary.md
/// §4.3: `dsl` has no blanket `DslField` impl for tuples of any arity).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioRational {
    pub num: i64,
    pub den: i64,
}

impl Default for SemioRational {
    /// 🎯️ `1/1`, not `0/0` — a rational's denominator must never default to zero.
    fn default() -> Self {
        Self { num: 1, den: 1 }
    }
}

/// 🎯️ One decoded/encoded unit within a stream. `data` is the format's opaque compressed payload
/// (honest boundary — never decoded by this subset).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioVideoSample {
    pub pts: u64,
    #[serde(default)]
    pub key: bool,
    #[serde(default)]
    pub data: Vec<u8>,
}

/// 🎞️ One elementary stream (video/audio/subtitle track) inside the container.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioVideoStream {
    #[serde(default)]
    pub kind: SemioVideoStreamKind,
    #[serde(default)]
    pub codec: String,
    #[serde(default)]
    pub width: u32,
    #[serde(default)]
    pub height: u32,
    #[serde(default)]
    pub rate: SemioRational,
    #[serde(default)]
    pub samples: Vec<SemioVideoSample>,
}
//#endregion 🔖️VideoModel

//#region 🔖️Ids
pub const STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA: &str = "stdio.semio.video";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.video")]
pub struct SemioVideoSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub streams: Vec<SemioVideoStream>,
}

impl Default for SemioVideoSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(), streams: Default::default() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextPrimitives
/// 🧪️ Real hex/bracket-encoded value primitives backing the hand-rolled `ArtifactDsl` below — same
/// style as this subset's own `🔺️diff`/`🧬️mutations` facets (`GifDiff`/`SvgDiff`/`DocxDiff`'s
/// established hand-rolled convention). Duplicated here (not imported from `schema::diff`) to keep
/// `snapshot` — the base type `diff`/`mutations` both depend ON — free of a reverse dependency on
/// either sibling facet (same convention `flow`'s own pilot established).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_bool(b: &bool) -> String {
    if *b {
        "1".to_string()
    } else {
        "0".to_string()
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bool: bad value {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_kind(k: &SemioVideoStreamKind) -> String {
    match k {
        SemioVideoStreamKind::Video => "V",
        SemioVideoStreamKind::Audio => "A",
        SemioVideoStreamKind::Subtitle => "S",
    }
    .to_string()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_kind(s: &str) -> Result<SemioVideoStreamKind, String> {
    match s {
        "V" => Ok(SemioVideoStreamKind::Video),
        "A" => Ok(SemioVideoStreamKind::Audio),
        "S" => Ok(SemioVideoStreamKind::Subtitle),
        other => Err(format!("stream kind: bad value {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_rational(r: &SemioRational) -> String {
    format!("[{},{}]", r.num, r.den)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_rational(s: &str) -> Result<SemioRational, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [num, den] = parts.as_slice() else { return Err(format!("rational: expected 2 fields, got {}", parts.len())) };
    Ok(SemioRational { num: num.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, den: den.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_sample(s: &SemioVideoSample) -> String {
    format!("[{},{},{}]", s.pts, enc_bool(&s.key), hex_encode(&s.data))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_sample(s: &str) -> Result<SemioVideoSample, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [pts, key, data] = parts.as_slice() else { return Err(format!("sample: expected 3 fields, got {}", parts.len())) };
    Ok(SemioVideoSample { pts: pts.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, key: dec_bool(key)?, data: hex_decode(data)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_stream(s: &SemioVideoStream) -> String {
    format!("[{},{},{},{},{},[{}]]", enc_kind(&s.kind), enc_str(&s.codec), s.width, s.height, enc_rational(&s.rate), s.samples.iter().map(enc_sample).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_stream(s: &str) -> Result<SemioVideoStream, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [kind, codec, width, height, rate, samples] = parts.as_slice() else { return Err(format!("stream: expected 6 fields, got {}", parts.len())) };
    let samples = split_top_level(strip_brackets(samples)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_sample).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioVideoStream {
        kind: dec_kind(kind)?,
        codec: dec_str(codec)?,
        width: width.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        height: height.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        rate: dec_rational(rate)?,
        samples,
    })
}

/// 📄️ The real structured text body: two lines — `schema=<hex>`, `streams=[<stream>,...]` —
/// matching the grammar's `document = artifact-mark schema-line streams-line`. Newlines are pure
/// lexer trivia in the shared dialect, so this is genuinely recognizable by `dsl::Recognizer`, not
/// merely readable.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_video_snapshot_body(s: &SemioVideoSnapshot) -> String {
    format!("schema={}\nstreams=[{}]", enc_str(&s.schema), s.streams.iter().map(enc_stream).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_video_snapshot_body(body: &str) -> Result<SemioVideoSnapshot, String> {
    let mut schema = None;
    let mut streams = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("streams=") {
            let inner = strip_brackets(rest)?;
            streams = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_stream).collect::<Result<Vec<_>, String>>()?;
        } else {
            return Err(format!("video snapshot: unknown line {line:?}"));
        }
    }
    let schema = schema.ok_or_else(|| "video snapshot: missing schema line".to_string())?;
    Ok(SemioVideoSnapshot { schema, streams })
}
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers flow's/mesh's upgraded facets reuse) backing the real
/// `ArtifactPack` below — replaces the old `serde_json::to_vec`-in-envelope shortcut.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).await.map_err(|e| e.to_string())?.to_vec())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn kind_tag(k: SemioVideoStreamKind) -> u8 {
    match k {
        SemioVideoStreamKind::Video => 0,
        SemioVideoStreamKind::Audio => 1,
        SemioVideoStreamKind::Subtitle => 2,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn kind_from_tag(t: u8) -> Result<SemioVideoStreamKind, String> {
    match t {
        0 => Ok(SemioVideoStreamKind::Video),
        1 => Ok(SemioVideoStreamKind::Audio),
        2 => Ok(SemioVideoStreamKind::Subtitle),
        other => Err(format!("stream kind: bad tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_video_snapshot_binary(s: &SemioVideoSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.streams.len() as u64);
    for stream in &s.streams {
        out.push(kind_tag(stream.kind));
        write_str_lp(&mut out, &stream.codec);
        out.extend_from_slice(&stream.width.to_le_bytes());
        out.extend_from_slice(&stream.height.to_le_bytes());
        out.extend_from_slice(&stream.rate.num.to_le_bytes());
        out.extend_from_slice(&stream.rate.den.to_le_bytes());
        store::pack_rt::write_varint_u64(&mut out, stream.samples.len() as u64);
        for sample in &stream.samples {
            out.extend_from_slice(&sample.pts.to_le_bytes());
            out.push(if sample.key { 1 } else { 0 });
            write_bytes_lp(&mut out, &sample.data);
        }
    }
    out
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_video_snapshot_binary(bytes: &[u8]) -> Result<SemioVideoSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = semio_framework_plugin::resolve_ready(store::ByteReader::new(bytes));
    let format = reader.read_u8().await.map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let stream_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut streams = Vec::with_capacity(stream_count as usize);
    for _ in 0..stream_count {
        let kind = kind_from_tag(reader.read_u8().await.map_err(|e| e.to_string())?)?;
        let codec = read_str_lp(&mut reader)?;
        let width = reader.read_u32_le().await.map_err(|e| e.to_string())?;
        let height = reader.read_u32_le().await.map_err(|e| e.to_string())?;
        let num = i64::from_le_bytes(reader.read_bytes(8).await.map_err(|e| e.to_string())?.try_into().map_err(|_| "rate.num: truncated".to_string())?);
        let den = i64::from_le_bytes(reader.read_bytes(8).await.map_err(|e| e.to_string())?.try_into().map_err(|_| "rate.den: truncated".to_string())?);
        let sample_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
        let mut samples = Vec::with_capacity(sample_count as usize);
        for _ in 0..sample_count {
            let pts = reader.read_u64_le().await.map_err(|e| e.to_string())?;
            let key = match reader.read_u8().await.map_err(|e| e.to_string())? {
                0 => false,
                1 => true,
                other => return Err(format!("sample key: bad tag {other}")),
            };
            let data = read_bytes_lp(&mut reader)?;
            samples.push(SemioVideoSample { pts, key, data });
        }
        streams.push(SemioVideoStream { kind, codec, width, height, rate: SemioRational { num, den }, samples });
    }
    Ok(SemioVideoSnapshot { schema, streams })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁 Real structured text/binary codecs (video wave — off the old hex-dump-of-`serde_json`
/// shortcut, following flow's/mesh's/image's proven pattern). Wrapped in the repo-wide
/// `store::semio_format` envelope, unchanged.
impl store::ArtifactDsl for SemioVideoSnapshot {
    const EXTENSION: &'static str = "semio";
    async fn envelope_id() -> &'static str {
        STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_video_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    async fn print_dsl(&self) -> String {
        let body = print_video_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioVideoSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_video_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_video_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.video` document — 2 streams (one video w/ 2 samples incl. a key
/// frame, one audio w/ no samples), exercising every leaf shape at least once. Single source of
/// truth for `📚️examples/…/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` and for the
/// conformance-law tests in `🎹️composer/🦀️component.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_video_snapshot() -> SemioVideoSnapshot {
    SemioVideoSnapshot {
        schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
        streams: vec![
            SemioVideoStream {
                kind: SemioVideoStreamKind::Video,
                codec: "h264".into(),
                width: 1920,
                height: 1080,
                rate: SemioRational { num: 30, den: 1 },
                samples: vec![SemioVideoSample { pts: 0, key: true, data: vec![0x00, 0x01, 0x02, 0x03] }, SemioVideoSample { pts: 33, key: false, data: vec![0x04, 0x05] }],
            },
            SemioVideoStream { kind: SemioVideoStreamKind::Audio, codec: "aac".into(), width: 0, height: 0, rate: SemioRational { num: 48_000, den: 1_000 }, samples: Vec::new() },
        ],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_snapshot() -> SemioVideoSnapshot {
        SemioVideoSnapshot {
            schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
            streams: vec![
                SemioVideoStream { kind: SemioVideoStreamKind::Video, codec: "h264".into(), width: 1920, height: 1080, rate: SemioRational { num: 30, den: 1 }, samples: vec![SemioVideoSample { pts: 0, key: true, data: vec![1, 2, 3] }] },
                SemioVideoStream { kind: SemioVideoStreamKind::Audio, codec: "aac".into(), width: 0, height: 0, rate: SemioRational { num: 48_000, den: 1_000 }, samples: Vec::new() },
            ],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn json_pack_round_trips() {
        let snap = sample_snapshot();
        let bytes = <SemioVideoSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioVideoSnapshot as store::ArtifactPack>::decode_pack(&bytes).await.expect("decode");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_text_round_trips() {
        let snap = sample_snapshot();
        let text = <SemioVideoSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioVideoSnapshot as store::ArtifactDsl>::parse_dsl(&text).await.expect("parse");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn stream_kind_defaults_to_video_and_rational_defaults_to_one_over_one() {
        assert_eq!(SemioVideoStreamKind::default(), SemioVideoStreamKind::Video);
        assert_eq!(SemioRational::default(), SemioRational { num: 1, den: 1 });
    }
}
//#endregion 🔖️Tests
