//! 📝️ Text representation codec surface for `s.stdio.semio.audio` (snapshot).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type SemioAudioSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::audio::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;

/// 🧪️ Real hex/bracket-encoded value primitives backing the hand-rolled `ArtifactDsl` below — same
/// style as this subset's own `🔺️diff`/`🧬️mutations` facets (`GifDiff`/`SvgDiff`/`DocxDiff`'s
/// established hand-rolled convention), duplicated here (not imported from `schema::diff`) to keep
/// `snapshot` — the base type `diff`/`mutations` both depend ON — free of a reverse dependency on
/// either sibling facet (same rationale `🌊️flow`'s/`🖼️image`'s own pilots document).
///
/// 🧩️ The `#[derive(dsl::DslArtifact)]` path was tried first per this ticket's brief. It is
/// blocked here for the SAME reason `🖼️image`'s own pilot documents: even though NO field here is a
/// bare `Option<T>`, `SemioAudioChannel.samples: Vec<f32>` and `SemioAudioTag` are both plain
/// `Vec<Record>` collections nested one level under the snapshot's own `Vec<SemioAudioChannel>`/
/// `Vec<SemioAudioTag>` fields — fine for the derive's tested `#[dsl(table)]` shape in isolation,
/// but this subset's own `🔺️diff`/`🧬️mutations` facets ALREADY hand-roll their codecs (pre-wave),
/// and per the ticket's blanket instruction ("hand-roll all diff/op codecs — do not fight the
/// derive"), keeping the snapshot on the SAME hand-rolled hex/bracket convention as its sibling
/// facets (rather than a derive-based codec that would print/parse a structurally different wire
/// shape) is the honest, single-source-of-truth choice — same boundary `🌊️flow`'s/`🔺️mesh`'s/
/// `🖼️image`'s own pilots each independently reached for their own shape.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_format(f: SemioAudioFormat) -> &'static str {
    match f {
        SemioAudioFormat::Pcm8 => "pcm8",
        SemioAudioFormat::Pcm16 => "pcm16",
        SemioAudioFormat::Pcm24 => "pcm24",
        SemioAudioFormat::Pcm32 => "pcm32",
        SemioAudioFormat::Float32 => "f32",
        SemioAudioFormat::Float64 => "f64",
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_format(s: &str) -> Result<SemioAudioFormat, String> {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_f32_list(v: &[f32]) -> String {
    format!("[{}]", v.iter().map(|f| format!("{:08x}", f.to_bits())).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_f32_list(s: &str) -> Result<Vec<f32>, String> {
    let inner = strip_brackets(s)?;
    if inner.is_empty() {
        return Ok(Vec::new());
    }
    split_top_level(inner, ',').into_iter().map(|tok| u32::from_str_radix(tok, 16).map(f32::from_bits).map_err(|e| e.to_string())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_channel(c: &SemioAudioChannel) -> String {
    enc_f32_list(&c.samples)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_channel(s: &str) -> Result<SemioAudioChannel, String> {
    Ok(SemioAudioChannel { samples: dec_f32_list(s)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_tag(t: &SemioAudioTag) -> String {
    format!("[{},{}]", enc_str(&t.key), enc_str(&t.value))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_tag(s: &str) -> Result<SemioAudioTag, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [key, value] = parts.as_slice() else { return Err(format!("tag: expected 2 fields, got {}", parts.len())) };
    Ok(SemioAudioTag { key: dec_str(key)?, value: dec_str(value)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}

/// 📄️ The real structured text body: five lines — `schema=<hex>`, `sampleRate=<N>`,
/// `format=<f>`, `channels=[<channel>,...]`, `tags=[<tag>,...]` — matching the grammar's
/// `document = artifact-mark schema-line sample-rate-line format-line channels-line tags-line`.
/// Newlines are pure lexer trivia in the shared dialect, so this is genuinely recognizable by
/// `dsl::Recognizer`, not merely readable.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_audio_snapshot_body(s: &SemioAudioSnapshot) -> String {
    format!("schema={}\nsampleRate={}\nformat={}\nchannels={}\ntags={}", enc_str(&s.schema), s.sample_rate, enc_format(s.format), enc_list(&s.channels, enc_channel), enc_list(&s.tags, enc_tag),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_audio_snapshot_body(body: &str) -> Result<SemioAudioSnapshot, String> {
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

/// 🎁 Real structured text/binary codecs — replaces the old hex-dump-of-`serde_json` shortcut.
/// Wrapped in the repo-wide `store::semio_format` envelope, unchanged.
impl store::ArtifactDsl for SemioAudioSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_audio_snapshot_body(body).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_audio_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📝️ Free-function face of [`SemioAudioSnapshot`]'s own `store::ArtifactDsl` text codec. `ArtifactDsl` is
/// declared by the os-kernel, which is an INTERNAL dependency of this plugin (aliased `store` in
/// `🦀️.rs`) and is therefore not nameable by a consumer that links only this crate — a
/// generated test host being the concrete case. The codec itself is the subset's, so its entry
/// point belongs here rather than behind a trait the caller cannot import.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_audio_dsl(text: &str) -> Result<SemioAudioSnapshot, String> {
    <SemioAudioSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|e| e.to_string())
}

/// 📝️ Free-function face of [`SemioAudioSnapshot`]'s own `store::ArtifactDsl` printer — see
/// [`parse_semio_audio_dsl`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_audio_dsl(snapshot: &SemioAudioSnapshot) -> String {
    <SemioAudioSnapshot as store::ArtifactDsl>::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::audio::schema::diff::*;
use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};
use crate::standards::v1::subsets::base::schema::triples::{self, IndexAdded, IndexModified, IndexedTripleDiff};
use protocol::command::DiffAlgebra;
/// 🔧️ Unconditional — `impl protocol::DiffCodec for SemioAudioDiff` below's `encode_diff`/
/// `decode_diff` are now real production code (binary upgrade, this wave), not test-only.
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::MutationDiff;

/// 🧩️ Full bracket encoding of a snapshot — used both by [`protocol::DiffCodec`]'s `SetSnapshot`
/// payload (via the mutations module) and directly nowhere else; kept here alongside its sibling
/// value codecs.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_snapshot(s: &SemioAudioSnapshot) -> String {
    format!("[{},{},{},[{}],[{}]]", hex_encode(s.schema.as_bytes()), s.sample_rate, enc_format(s.format), s.channels.iter().map(enc_channel).collect::<Vec<_>>().join(","), s.tags.iter().map(enc_tag).collect::<Vec<_>>().join(","),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_snapshot(s: &str) -> Result<SemioAudioSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema_hex, sample_rate, format, channels_s, tags_s] = parts.as_slice() else {
        return Err(format!("snapshot: expected 5 fields, got {}", parts.len()));
    };
    let channels = split_top_level(strip_brackets(channels_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_channel).collect::<Result<Vec<_>, String>>()?;
    let tags = split_top_level(strip_brackets(tags_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_tag).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioAudioSnapshot { schema: hex_decode_string(schema_hex)?, sample_rate: parse_u32(sample_rate)?, format: dec_format(format)?, channels, tags })
}
}
pub use diff_codec::*;
