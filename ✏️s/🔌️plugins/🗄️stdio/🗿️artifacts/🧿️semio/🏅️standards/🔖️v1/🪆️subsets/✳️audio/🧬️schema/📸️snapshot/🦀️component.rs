//! 🔊️ SemioAudioSnapshot — complete per the master plan's `audio` row: `sample_rate` +
//! `format` (typed sample-format enum, describing the ORIGINAL encoding the samples were decoded
//! from) + `channels` (ordered, index-keyed — index 0 = left/mono, 1 = right, … matching wav's
//! interleaved-channel-order convention) + `tags` (ordered key/value metadata pairs, ID3/RIFF
//! `LIST INFO`-shaped — duplicate keys are legal on disk, hence a `Vec`, never a `BTreeMap`).
//! Per the ticket's honest-boundary note: audio is schema-complete for ITS OWN shape and stores
//! REAL decoded `f32` samples (unlike `video`, which is deliberately payload-opaque) — decoding a
//! compressed container's samples into this shape is a W3/W4 codec concern, not this subset's.
//! Owned types (see `w1b-type-ownership.md`): `SemioAudioSnapshot`, `SemioAudioChannel`. New this
//! wave: `SemioAudioFormat`, `SemioAudioTag` (the `tags` field was W1b-reserved, not yet defined).
//!
//! ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION's audio wave replaces the old
//! hex-of-`serde_json` envelope passthrough with real hand-rolled text/binary codecs (this is a
//! NEUTRAL semio type, not itself an on-disk file format — real per-format bytes for wav/mp3 are
//! produced by the semio↔format `🚪️io` leaves).

use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA: &str = "stdio.semio.audio";
//#endregion 🔖️Ids

//#region 🔖️Format
/// 🎚️ The sample format the audio was originally encoded in — metadata describing provenance,
/// independent of this snapshot's own always-`f32` sample storage (see module doc comment).
/// `wav`-shaped: mirrors PCM8/16/24/32 + IEEE float, the `fmt ` chunk's `wBitsPerSample`/
/// `wFormatTag` space, without depending on wav's own (future, W3) types — own type, per the
/// repo-wide "own types, not merged into a sibling format" convention (tsv-vs-csv, docx-vs-xlsx).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SemioAudioFormat {
    Pcm8,
    #[default]
    Pcm16,
    Pcm24,
    Pcm32,
    Float32,
    Float64,
}
//#endregion 🔖️Format

//#region 🔖️Channel
/// 🔊️ Owned by the `audio` subset (per `w1b-type-ownership.md`). One channel's full, decoded
/// sample sequence — a strong, per-field-diffable entity (today one field, `samples`, but kept as
/// its own struct + collection triple rather than `Vec<Vec<f32>>` so a future field, e.g. a
/// per-channel gain/pan, slots in without reshaping the collection).
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioAudioChannel {
    #[serde(default)]
    pub samples: Vec<f32>,
}
//#endregion 🔖️Channel

//#region 🔖️Tag
/// 🏷️ One metadata key/value pair (ID3/RIFF `LIST INFO`-shaped: `title`, `artist`, `comment`, …).
/// A weak/value entity per the recipe (its "diff" is the whole new pair, never sub-diffed).
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioAudioTag {
    pub key: String,
    pub value: String,
}
//#endregion 🔖️Tag

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.audio")]
pub struct SemioAudioSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub sample_rate: u32,
    #[state(persistent)]
    #[serde(default)]
    pub format: SemioAudioFormat,
    #[state(persistent)]
    #[serde(default)]
    pub channels: Vec<SemioAudioChannel>,
    #[state(persistent)]
    #[serde(default)]
    pub tags: Vec<SemioAudioTag>,
}

impl Default for SemioAudioSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA.into(),
            sample_rate: 0,
            format: SemioAudioFormat::default(),
            channels: Vec::new(),
            tags: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextPrimitives
/// 🧪️ Real hex/bracket-encoded value primitives backing the hand-rolled `ArtifactDsl` below — same
/// style as this subset's own `🔺️diff`/`🧬️mutations` facets (`GifDiff`/`SvgDiff`/`DocxDiff`'s
/// established hand-rolled convention), duplicated here (not imported from `schema::diff`) to keep
/// `snapshot` — the base type `diff`/`mutations` both depend ON — free of a reverse dependency on
/// either sibling facet (same rationale `✳️flow`'s/`✳️image`'s own pilots document).
///
/// 🧩️ The `#[derive(dsl::DslArtifact)]` path was tried first per this ticket's brief. It is
/// blocked here for the SAME reason `✳️image`'s own pilot documents: even though NO field here is a
/// bare `Option<T>`, `SemioAudioChannel.samples: Vec<f32>` and `SemioAudioTag` are both plain
/// `Vec<Record>` collections nested one level under the snapshot's own `Vec<SemioAudioChannel>`/
/// `Vec<SemioAudioTag>` fields — fine for the derive's tested `#[dsl(table)]` shape in isolation,
/// but this subset's own `🔺️diff`/`🧬️mutations` facets ALREADY hand-roll their codecs (pre-wave),
/// and per the ticket's blanket instruction ("hand-roll all diff/op codecs — do not fight the
/// derive"), keeping the snapshot on the SAME hand-rolled hex/bracket convention as its sibling
/// facets (rather than a derive-based codec that would print/parse a structurally different wire
/// shape) is the honest, single-source-of-truth choice — same boundary `✳️flow`'s/`✳️mesh`'s/
/// `✳️image`'s own pilots each independently reached for their own shape.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
fn parse_u32(s: &str) -> Result<u32, String> { s.parse().map_err(|e: std::num::ParseIntError| e.to_string()) }

fn enc_format(f: SemioAudioFormat) -> &'static str {
    match f {
        SemioAudioFormat::Pcm8 => "pcm8",
        SemioAudioFormat::Pcm16 => "pcm16",
        SemioAudioFormat::Pcm24 => "pcm24",
        SemioAudioFormat::Pcm32 => "pcm32",
        SemioAudioFormat::Float32 => "f32",
        SemioAudioFormat::Float64 => "f64",
    }
}
fn dec_format(s: &str) -> Result<SemioAudioFormat, String> {
    match s {
        "pcm8" => Ok(SemioAudioFormat::Pcm8),
        "pcm16" => Ok(SemioAudioFormat::Pcm16),
        "pcm24" => Ok(SemioAudioFormat::Pcm24),
        "pcm32" => Ok(SemioAudioFormat::Pcm32),
        "f32" => Ok(SemioAudioFormat::Float32),
        "f64" => Ok(SemioAudioFormat::Float64),
        other => Err(format!("bad audio format {other:?}")),
    }
}

/// 🔢️ Exact-round-trip `f32` list — `to_bits()` hex tokens inside a bracket, never decimal
/// text (sidesteps float-formatting precision loss and NaN/-0.0 print-ambiguity entirely). Same
/// convention this subset's own `🔺️diff` facet's `enc_f32_list` uses (duplicated, not imported —
/// see this region's own doc comment for why).
fn enc_f32_list(v: &[f32]) -> String {
    format!("[{}]", v.iter().map(|f| format!("{:08x}", f.to_bits())).collect::<Vec<_>>().join(","))
}
fn dec_f32_list(s: &str) -> Result<Vec<f32>, String> {
    let inner = strip_brackets(s)?;
    if inner.is_empty() { return Ok(Vec::new()); }
    split_top_level(inner, ',').into_iter().map(|tok| u32::from_str_radix(tok, 16).map(f32::from_bits).map_err(|e| e.to_string())).collect()
}
fn enc_channel(c: &SemioAudioChannel) -> String { enc_f32_list(&c.samples) }
fn dec_channel(s: &str) -> Result<SemioAudioChannel, String> { Ok(SemioAudioChannel { samples: dec_f32_list(s)? }) }
fn enc_tag(t: &SemioAudioTag) -> String { format!("[{},{}]", enc_str(&t.key), enc_str(&t.value)) }
fn dec_tag(s: &str) -> Result<SemioAudioTag, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [key, value] = parts.as_slice() else { return Err(format!("tag: expected 2 fields, got {}", parts.len())) };
    Ok(SemioAudioTag { key: dec_str(key)?, value: dec_str(value)? })
}
fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|it| enc(it)).collect::<Vec<_>>().join(","))
}
fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| dec(entry)).collect()
}

/// 📄️ The real structured text body: five lines — `schema=<hex>`, `sampleRate=<N>`,
/// `format=<f>`, `channels=[<channel>,...]`, `tags=[<tag>,...]` — matching the grammar's
/// `document = artifact-mark schema-line sample-rate-line format-line channels-line tags-line`.
/// Newlines are pure lexer trivia in the shared dialect, so this is genuinely recognizable by
/// `dsl::Recognizer`, not merely readable.
fn print_audio_snapshot_body(s: &SemioAudioSnapshot) -> String {
    format!(
        "schema={}\nsampleRate={}\nformat={}\nchannels={}\ntags={}",
        enc_str(&s.schema),
        s.sample_rate,
        enc_format(s.format),
        enc_list(&s.channels, enc_channel),
        enc_list(&s.tags, enc_tag),
    )
}
fn parse_audio_snapshot_body(body: &str) -> Result<SemioAudioSnapshot, String> {
    let mut schema = None;
    let mut sample_rate = None;
    let mut format = None;
    let mut channels = Vec::new();
    let mut tags = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("sampleRate=") {
            sample_rate = Some(parse_u32(rest)?);
        } else if let Some(rest) = line.strip_prefix("format=") {
            format = Some(dec_format(rest)?);
        } else if let Some(rest) = line.strip_prefix("channels=") {
            channels = dec_list(rest, dec_channel)?;
        } else if let Some(rest) = line.strip_prefix("tags=") {
            tags = dec_list(rest, dec_tag)?;
        } else {
            return Err(format!("semio audio snapshot: unknown line {line:?}"));
        }
    }
    Ok(SemioAudioSnapshot {
        schema: schema.ok_or_else(|| "semio audio snapshot: missing schema line".to_string())?,
        sample_rate: sample_rate.ok_or_else(|| "semio audio snapshot: missing sampleRate line".to_string())?,
        format: format.unwrap_or_default(),
        channels,
        tags,
    })
}
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers `✳️flow`'s/`✳️mesh`'s/`✳️image`'s own upgraded
/// `ArtifactPack` uses) backing the real `ArtifactPack` below — replaces the old
/// `serde_json::to_vec`-in-envelope shortcut.
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
fn read_f32_le(reader: &mut store::ByteReader<'_>) -> Result<f32, String> {
    let bytes = reader.read_bytes(4).map_err(|e| e.to_string())?;
    let arr: [u8; 4] = bytes.try_into().map_err(|_| "f32 read: truncated".to_string())?;
    Ok(f32::from_le_bytes(arr))
}

fn format_tag(f: SemioAudioFormat) -> u8 {
    match f {
        SemioAudioFormat::Pcm8 => 0,
        SemioAudioFormat::Pcm16 => 1,
        SemioAudioFormat::Pcm24 => 2,
        SemioAudioFormat::Pcm32 => 3,
        SemioAudioFormat::Float32 => 4,
        SemioAudioFormat::Float64 => 5,
    }
}
fn format_from_tag(tag: u8) -> Result<SemioAudioFormat, String> {
    match tag {
        0 => Ok(SemioAudioFormat::Pcm8),
        1 => Ok(SemioAudioFormat::Pcm16),
        2 => Ok(SemioAudioFormat::Pcm24),
        3 => Ok(SemioAudioFormat::Pcm32),
        4 => Ok(SemioAudioFormat::Float32),
        5 => Ok(SemioAudioFormat::Float64),
        other => Err(format!("unsupported audio format tag {other}")),
    }
}

/// 🎁 `format u8` + varint-length-prefixed `schema` UTF-8 + real fixed `sample_rate` (`u32` LE) +
/// `audio_format` (`u8` tag) — all genuinely, individually protocol-walkable, matching the real
/// `📡️component.protocol.semio` header/segment fields exactly — then `channels` (varint count +
/// per-channel varint sample count + real 4-byte LE `f32` samples, no hex/text detour) and `tags`
/// (varint count + per-entry varint-length-prefixed `key`/`value` UTF-8) as the honest opaque
/// `payload` tail (`protocol-array-of-records` gap — `channels`/`tags` are homogeneous
/// variable-length repeated records).
fn encode_audio_snapshot_binary(s: &SemioAudioSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    out.extend_from_slice(&s.sample_rate.to_le_bytes());
    out.push(format_tag(s.format));
    store::pack_rt::write_varint_u64(&mut out, s.channels.len() as u64);
    for c in &s.channels {
        store::pack_rt::write_varint_u64(&mut out, c.samples.len() as u64);
        for sample in &c.samples {
            out.extend_from_slice(&sample.to_le_bytes());
        }
    }
    store::pack_rt::write_varint_u64(&mut out, s.tags.len() as u64);
    for tag in &s.tags {
        write_str_lp(&mut out, &tag.key);
        write_str_lp(&mut out, &tag.value);
    }
    out
}
fn decode_audio_snapshot_binary(bytes: &[u8]) -> Result<SemioAudioSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let sample_rate = reader.read_u32_le().map_err(|e| e.to_string())?;
    let audio_format = format_from_tag(reader.read_u8().map_err(|e| e.to_string())?)?;
    let channel_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut channels = Vec::with_capacity(channel_count as usize);
    for _ in 0..channel_count {
        let sample_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
        let mut samples = Vec::with_capacity(sample_count as usize);
        for _ in 0..sample_count {
            samples.push(read_f32_le(&mut reader)?);
        }
        channels.push(SemioAudioChannel { samples });
    }
    let tag_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut tags = Vec::with_capacity(tag_count as usize);
    for _ in 0..tag_count {
        let key = read_str_lp(&mut reader)?;
        let value = read_str_lp(&mut reader)?;
        tags.push(SemioAudioTag { key, value });
    }
    Ok(SemioAudioSnapshot { schema, sample_rate, format: audio_format, channels, tags })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁 Real structured text/binary codecs — replaces the old hex-dump-of-`serde_json` shortcut.
/// Wrapped in the repo-wide `store::semio_format` envelope, unchanged.
impl store::ArtifactDsl for SemioAudioSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str { STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_audio_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_audio_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioAudioSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_audio_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        decode_audio_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Demo
/// 🌱 The demo `stdio.semio.audio` document — two channels (a short sweep each), a non-default
/// sample format, and one metadata tag — exercising every leaf/collection shape at least once.
/// Single source of truth for `📚️examples/…/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio`
/// and for the conformance-law tests in `🎹️composer/🦀️component.rs`.
#[cfg(test)]
pub(crate) fn demo_audio_snapshot() -> SemioAudioSnapshot {
    SemioAudioSnapshot {
        schema: STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA.into(),
        sample_rate: 44_100,
        format: SemioAudioFormat::Float32,
        channels: vec![
            SemioAudioChannel { samples: vec![0.0, 0.5, -0.5, 1.0] },
            SemioAudioChannel { samples: vec![0.0, -0.5, 0.5, -1.0] },
        ],
        tags: vec![SemioAudioTag { key: "title".into(), value: "test tone".into() }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🌱 Reuses `demo_audio_snapshot()` (single source of truth, also feeds the shipped fixtures
    /// and `🎹️composer/🦀️component.rs`'s conformance-law tests) rather than an independent copy.
    fn sample_snapshot() -> SemioAudioSnapshot {
        demo_audio_snapshot()
    }

    #[test]
    fn json_pack_round_trips() {
        let snap = sample_snapshot();
        let bytes = <SemioAudioSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioAudioSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[test]
    fn dsl_text_round_trips() {
        let snap = sample_snapshot();
        let text = <SemioAudioSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioAudioSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    #[test]
    fn default_snapshot_has_no_channels_or_tags() {
        let snap = SemioAudioSnapshot::default();
        assert!(snap.channels.is_empty());
        assert!(snap.tags.is_empty());
        assert_eq!(snap.format, SemioAudioFormat::Pcm16);
    }

    /// 🧪️ codec_retention_law: decode(encode(snapshot)) is byte-for-byte structurally identical
    /// on a fully-populated snapshot (channels/tags non-empty), not just the default.
    #[test]
    fn codec_retention_law() {
        let snap = sample_snapshot();
        let bytes = <SemioAudioSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioAudioSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
        let text = <SemioAudioSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back_text = <SemioAudioSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back_text);
    }
}
//#endregion 🔖️Tests
