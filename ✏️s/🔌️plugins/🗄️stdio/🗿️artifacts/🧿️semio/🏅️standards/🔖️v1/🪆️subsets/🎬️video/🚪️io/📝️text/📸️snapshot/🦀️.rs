//! 📝️ Text representation grammar surface for `stdio.semio.video` (snapshot): real structured DSL
//! body — `schema=<hex>` then `streams=[<stream>,...]`, every leaf its own hex/bracket-encoded
//! token (video wave, replacing the old envelope-header-plus-hex(JSON) scaffold) — actual
//! parse/print lives on `SemioVideoSnapshot`'s `store::ArtifactDsl` impl in the facet root
//! `🦀️.rs`; this leaf carries the normative grammar description.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::video::schema::snapshot::*;
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;

/// 🧪️ Real hex/bracket-encoded value primitives backing the hand-rolled `ArtifactDsl` below — same
/// style as this subset's own `🔺️diff`/`🧬️mutations` facets (`GifDiff`/`SvgDiff`/`DocxDiff`'s
/// established hand-rolled convention). Duplicated here (not imported from `schema::diff`) to keep
/// `snapshot` — the base type `diff`/`mutations` both depend ON — free of a reverse dependency on
/// either sibling facet (same convention `flow`'s own pilot established).
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
pub(crate) fn enc_bool(b: &bool) -> String {
    if *b {
        "1".to_string()
    } else {
        "0".to_string()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bool: bad value {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_kind(k: &SemioVideoStreamKind) -> String {
    match k {
        SemioVideoStreamKind::Video => "V",
        SemioVideoStreamKind::Audio => "A",
        SemioVideoStreamKind::Subtitle => "S",
    }
    .to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_kind(s: &str) -> Result<SemioVideoStreamKind, String> {
    match s {
        "V" => Ok(SemioVideoStreamKind::Video),
        "A" => Ok(SemioVideoStreamKind::Audio),
        "S" => Ok(SemioVideoStreamKind::Subtitle),
        other => Err(format!("stream kind: bad value {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_rational(r: &SemioRational) -> String {
    format!("[{},{}]", r.num, r.den)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_rational(s: &str) -> Result<SemioRational, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [num, den] = parts.as_slice() else { return Err(format!("rational: expected 2 fields, got {}", parts.len())) };
    Ok(SemioRational { num: num.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, den: den.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_sample(s: &SemioVideoSample) -> String {
    format!("[{},{},{}]", s.pts, enc_bool(&s.key), hex_encode(&s.data))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_sample(s: &str) -> Result<SemioVideoSample, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [pts, key, data] = parts.as_slice() else { return Err(format!("sample: expected 3 fields, got {}", parts.len())) };
    Ok(SemioVideoSample { pts: pts.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, key: dec_bool(key)?, data: hex_decode(data)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_stream(s: &SemioVideoStream) -> String {
    format!("[{},{},{},{},{},[{}]]", enc_kind(&s.kind), enc_str(&s.codec), s.width, s.height, enc_rational(&s.rate), s.samples.iter().map(enc_sample).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_stream(s: &str) -> Result<SemioVideoStream, String> {
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
pub(crate) fn print_video_snapshot_body(s: &SemioVideoSnapshot) -> String {
    format!("schema={}\nstreams=[{}]", enc_str(&s.schema), s.streams.iter().map(enc_stream).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_video_snapshot_body(body: &str) -> Result<SemioVideoSnapshot, String> {
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

/// 🎁 Real structured text/binary codecs (video wave — off the old hex-dump-of-`serde_json`
/// shortcut, following flow's/mesh's/image's proven pattern). Wrapped in the repo-wide
/// `store::semio_format` envelope, unchanged.
impl store::ArtifactDsl for SemioVideoSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_video_snapshot_body(body).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_video_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📝️ Free-function face of [`SemioVideoSnapshot`]'s own `store::ArtifactDsl` text codec. `ArtifactDsl` is
/// declared by the os-kernel, which is an INTERNAL dependency of this plugin (aliased `store` in
/// `🦀️.rs`) and is therefore not nameable by a consumer that links only this crate — a
/// generated test host being the concrete case. The codec itself is the subset's, so its entry
/// point belongs here rather than behind a trait the caller cannot import.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_video_dsl(text: &str) -> Result<SemioVideoSnapshot, String> {
    <SemioVideoSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|e| e.to_string())
}

/// 📝️ Free-function face of [`SemioVideoSnapshot`]'s own `store::ArtifactDsl` printer — see
/// [`parse_semio_video_dsl`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_video_dsl(snapshot: &SemioVideoSnapshot) -> String {
    <SemioVideoSnapshot as store::ArtifactDsl>::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;
