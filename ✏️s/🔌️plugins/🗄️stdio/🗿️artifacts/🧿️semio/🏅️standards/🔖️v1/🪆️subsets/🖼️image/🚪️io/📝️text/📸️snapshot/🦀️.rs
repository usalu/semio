//! 📝️ Text representation codec surface for `stdio.semio.image` (snapshot) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::image::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;

/// 🧪️ Real hex/bracket-encoded value primitives backing the hand-rolled `ArtifactDsl` below — same
/// style as this subset's own `🔺️diff`/`🧬️mutations` facets (`GifDiff`/`SvgDiff`/`DocxDiff`'s
/// established hand-rolled convention), duplicated here (not imported from `schema::diff`) to keep
/// `snapshot` — the base type `diff`/`mutations` both depend ON — free of a reverse dependency on
/// either sibling facet (same rationale `🌊️flow`'s/`🔺️mesh`'s own pilots document).
///
/// 🧩️ The `#[derive(dsl::DslArtifact)]` path was tried first per this ticket's brief. It is
/// blocked here: `SemioImageSnapshot.icc: Option<Vec<u8>>` is a BARE `Option<T>` field directly on
/// the snapshot struct — `dsl` has no blanket `Option<T>: DslField` impl (the exact same shape
/// this subset's own `🔺️diff`/`🧬️mutations` facets already document as blocking their derive path,
/// matching gif's/docx's established precedent — `f6-final-summary.md` §4.3/§4.4). Hand-rolled
/// instead, same boundary this ticket's other semio pilots hit for their own bare-`Option`/nested-
/// buffer collection shapes.
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
pub(crate) fn enc_bytes(b: &[u8]) -> String {
    hex_encode(b)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_bytes(s: &str) -> Result<Vec<u8>, String> {
    hex_decode(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_u8(s: &str) -> Result<u8, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_colorspace(c: SemioColorspace) -> char {
    match c {
        SemioColorspace::Rgb => 'r',
        SemioColorspace::Rgba => 'a',
        SemioColorspace::Grayscale => 'g',
        SemioColorspace::GrayscaleAlpha => 'y',
        SemioColorspace::Indexed => 'i',
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_colorspace(s: &str) -> Result<SemioColorspace, String> {
    match s {
        "r" => Ok(SemioColorspace::Rgb),
        "a" => Ok(SemioColorspace::Rgba),
        "g" => Ok(SemioColorspace::Grayscale),
        "y" => Ok(SemioColorspace::GrayscaleAlpha),
        "i" => Ok(SemioColorspace::Indexed),
        other => Err(format!("bad colorspace {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_frame(f: &SemioImageFrame) -> String {
    format!("[{},{}]", f.delay_ms, hex_encode(&f.rgba8))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_frame(s: &str) -> Result<SemioImageFrame, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [delay, rgba] = parts.as_slice() else { return Err(format!("frame: expected 2 fields, got {}", parts.len())) };
    Ok(SemioImageFrame { delay_ms: parse_u32(delay)?, rgba8: hex_decode(rgba)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_metadata_entry(e: &SemioImageMetadataEntry) -> String {
    format!("[{},{}]", enc_str(&e.key), enc_str(&e.value))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_metadata_entry(s: &str) -> Result<SemioImageMetadataEntry, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [key, value] = parts.as_slice() else { return Err(format!("metadata entry: expected 2 fields, got {}", parts.len())) };
    Ok(SemioImageMetadataEntry { key: dec_str(key)?, value: dec_str(value)? })
}

/// 📄️ The real structured text body: eight lines — `schema=<hex>`, `width=<N>`, `height=<N>`,
/// `colorspace=<c>`, `bitDepth=<N>`, `icc=<option-hex>`, `frames=[<frame>,...]`,
/// `metadata=[<entry>,...]` — matching the grammar's `document = artifact-mark schema-line
/// width-line height-line colorspace-line bit-depth-line icc-line frames-line metadata-line`.
/// Newlines are pure lexer trivia in the shared dialect, so this is genuinely recognizable by
/// `dsl::Recognizer`, not merely readable.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_image_snapshot_body(s: &SemioImageSnapshot) -> String {
    format!(
        "schema={}\nwidth={}\nheight={}\ncolorspace={}\nbitDepth={}\nicc={}\nframes={}\nmetadata={}",
        enc_str(&s.schema),
        s.width,
        s.height,
        enc_colorspace(s.colorspace),
        s.bit_depth,
        encode_option(&s.icc, |b| enc_bytes(b)),
        enc_list(&s.frames, enc_frame),
        enc_list(&s.metadata, enc_metadata_entry),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_image_snapshot_body(body: &str) -> Result<SemioImageSnapshot, String> {
    let mut schema = None;
    let mut width = None;
    let mut height = None;
    let mut colorspace = None;
    let mut bit_depth = None;
    let mut icc = None;
    let mut frames = Vec::new();
    let mut metadata = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("width=") {
            width = Some(parse_u32(rest)?);
        } else if let Some(rest) = line.strip_prefix("height=") {
            height = Some(parse_u32(rest)?);
        } else if let Some(rest) = line.strip_prefix("colorspace=") {
            colorspace = Some(dec_colorspace(rest)?);
        } else if let Some(rest) = line.strip_prefix("bitDepth=") {
            bit_depth = Some(parse_u8(rest)?);
        } else if let Some(rest) = line.strip_prefix("icc=") {
            icc = Some(decode_option(rest, dec_bytes)?);
        } else if let Some(rest) = line.strip_prefix("frames=") {
            frames = dec_list(rest, dec_frame)?;
        } else if let Some(rest) = line.strip_prefix("metadata=") {
            metadata = dec_list(rest, dec_metadata_entry)?;
        } else {
            return Err(format!("semio image snapshot: unknown line {line:?}"));
        }
    }
    Ok(SemioImageSnapshot {
        schema: schema.ok_or_else(|| "semio image snapshot: missing schema line".to_string())?,
        width: width.ok_or_else(|| "semio image snapshot: missing width line".to_string())?,
        height: height.ok_or_else(|| "semio image snapshot: missing height line".to_string())?,
        colorspace: colorspace.unwrap_or_default(),
        bit_depth: bit_depth.unwrap_or(0),
        icc: icc.unwrap_or(None),
        frames,
        metadata,
    })
}

/// 🎁 Real structured text/binary codecs — replaces the old hex-dump-of-`serde_json` shortcut.
/// Wrapped in the repo-wide `store::semio_format` envelope, unchanged.
impl store::ArtifactDsl for SemioImageSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_image_snapshot_body(body).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_image_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📥️ Parses this subset's own committed `.dsl.semio` text into a real [`SemioImageSnapshot`] — a
/// thin wrapper over `store::ArtifactDsl::parse_dsl` so external Rust callers that cannot name this
/// crate's private `store` extern-crate item (the `🖼️mutate-semio-image` test adapter, whose
/// `identity-round-trip` scenario reads the REAL committed `📚️examples/🖼️swatch` artifact rather than
/// a JSON transcription of it) can still drive the same codec production does. Same shape and same
/// rationale as `🌊️flow`'s own bridge.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_image_dsl(text: &str) -> Result<SemioImageSnapshot, String> {
    <SemioImageSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 📤️ The `store::ArtifactDsl::print_dsl` inverse of [`parse_semio_image_dsl`] — same rationale.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_image_dsl(snapshot: &SemioImageSnapshot) -> String {
    <SemioImageSnapshot as store::ArtifactDsl>::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::image::schema::snapshot::*;
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use framework_schema::ArtifactSchema;




}
pub use snapshot_wire_codec::*;
