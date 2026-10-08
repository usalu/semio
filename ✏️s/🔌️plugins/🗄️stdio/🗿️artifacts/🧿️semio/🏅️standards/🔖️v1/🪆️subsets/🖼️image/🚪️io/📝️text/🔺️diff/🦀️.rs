//! 📝️ Text representation codec surface for `stdio.semio.image` (diff) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::image::schema::diff::*;
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff, NamedModified, NamedTripleDiff};
use crate::standards::v1::subsets::base::io::text::snapshot::{dec_indexed_triple, dec_named_added, dec_named_triple, enc_indexed_triple, enc_named_added, enc_named_triple};
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
/// 🔧️ Unconditional — the `#[cfg(test)] mod tests` block below calls `print_diff`/`parse_diff`/
/// `encode_diff`/`decode_diff` via method syntax on `SemioImageDiff`, which needs `DiffCodec` in
/// scope (the `impl protocol::DiffCodec for SemioImageDiff` block itself compiles fine unqualified,
/// but callers using method syntax do not get the trait for free) (W2b closer fix).
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::MutationDiff;
use crate::standards::v1::subsets::image::io::text::snapshot::encode_option;
use crate::standards::v1::subsets::image::io::text::snapshot::decode_option;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_image_diff(d: &SemioImageDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.width {
        tokens.push(format!("width={v}"));
    }
    if let Some(v) = d.height {
        tokens.push(format!("height={v}"));
    }
    if let Some(v) = d.colorspace {
        tokens.push(format!("colorspace={}", enc_colorspace(v)));
    }
    if let Some(v) = d.bit_depth {
        tokens.push(format!("bitDepth={v}"));
    }
    if let Some(v) = &d.icc {
        tokens.push(format!("icc={}", encode_option(v, |b| hex_encode(b))));
    }
    if let Some(v) = &d.frames {
        tokens.push(format!("frames{{{}}}", enc_frames_diff(v)));
    }
    if let Some(v) = &d.metadata {
        tokens.push(format!("metadata{{{}}}", enc_metadata_diff(v)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_image_diff(line: &str) -> Result<SemioImageDiff, String> {
    let mut d = SemioImageDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("width=") {
            d.width = Some(parse_u32(rest)?);
        } else if let Some(rest) = token.strip_prefix("height=") {
            d.height = Some(parse_u32(rest)?);
        } else if let Some(rest) = token.strip_prefix("colorspace=") {
            d.colorspace = Some(dec_colorspace(rest)?);
        } else if let Some(rest) = token.strip_prefix("bitDepth=") {
            d.bit_depth = Some(parse_u8(rest)?);
        } else if let Some(rest) = token.strip_prefix("icc=") {
            d.icc = Some(decode_option(rest, hex_decode)?);
        } else if let Some(rest) = token.strip_prefix("frames{") {
            d.frames = Some(dec_frames_diff(rest.strip_suffix('}').ok_or_else(|| "frames: missing closing brace".to_string())?)?);
        } else if let Some(rest) = token.strip_prefix("metadata{") {
            d.metadata = Some(dec_metadata_diff(rest.strip_suffix('}').ok_or_else(|| "metadata: missing closing brace".to_string())?)?);
        } else {
            return Err(format!("image diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for SemioImageDiff {
fn print_diff(&self) -> String {
    print_image_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_image_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
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
pub(crate) fn hex_encode_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
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
pub(crate) fn enc_frame_diff(d: &SemioImageFrameDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = d.delay_ms {
        parts.push(format!("D:{v}"));
    }
    if let Some(v) = &d.rgba8 {
        parts.push(format!("X:{}", hex_encode(v)));
    }
    format!("[{}]", parts.join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_frame_diff(s: &str) -> Result<SemioImageFrameDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = SemioImageFrameDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("frame diff: bad entry {entry:?}"))?;
        match tag {
            "D" => d.delay_ms = Some(parse_u32(val)?),
            "X" => d.rgba8 = Some(hex_decode(val)?),
            other => return Err(format!("frame diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_metadata_entry(e: &SemioImageMetadataEntry) -> String {
    format!("[{},{}]", hex_encode_str(&e.key), hex_encode_str(&e.value))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_metadata_entry(s: &str) -> Result<SemioImageMetadataEntry, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [key, value] = parts.as_slice() else { return Err(format!("metadata entry: expected 2 fields, got {}", parts.len())) };
    Ok(SemioImageMetadataEntry { key: hex_decode_str(key)?, value: hex_decode_str(value)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_frames_diff(d: &SemioImageFramesDiff) -> String {
    enc_indexed_triple(d, enc_frame_diff, enc_frame)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_frames_diff(s: &str) -> Result<SemioImageFramesDiff, String> {
    dec_indexed_triple(s, dec_frame_diff, dec_frame)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_metadata_diff(d: &SemioImageMetadataDiff) -> String {
    enc_named_triple(d, |k: &String| hex_encode_str(k), |v: &String| hex_encode_str(v), |a| enc_named_added(a, enc_metadata_entry))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_metadata_diff(s: &str) -> Result<SemioImageMetadataDiff, String> {
    dec_named_triple(s, hex_decode_str, hex_decode_str, |t| dec_named_added(t, dec_metadata_entry))
}
}
pub use diff_codec::*;
