//! 📝️ Text representation codec surface for `stdio.jpg` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type JpgDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_jfif_1_01::subsets::document::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::schema::snapshot::{JfifDensityUnits, JfifThumbnail, JpgSegment};
use crate::JpgSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, HashMap};

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
pub(crate) fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
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
pub(crate) fn parse_u8(s: &str) -> Result<u8, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_u16(s: &str) -> Result<u16, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_bool(s: &str) -> Result<bool, String> {
    match s {
        "0" => Ok(false),
        "1" => Ok(true),
        other => Err(format!("bad bool {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_density_units(u: &JfifDensityUnits) -> String {
    u.to_u8().to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_density_units(s: &str) -> Result<JfifDensityUnits, String> {
    JfifDensityUnits::from_u8(parse_u8(s)?)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_version(v: &(u8, u8)) -> String {
    format!("[{},{}]", v.0, v.1)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_version(s: &str) -> Result<(u8, u8), String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [a, b] = parts.as_slice() else { return Err(format!("jfif version: expected 2 fields, got {}", parts.len())) };
    Ok((parse_u8(a)?, parse_u8(b)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_quant_values(v: &[u16; 64]) -> String {
    format!("[{}]", v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_quant_values(s: &str) -> Result<[u16; 64], String> {
    let values: Vec<u16> = split_top_level(strip_brackets(s)?, ',').into_iter().map(parse_u16).collect::<Result<_, _>>()?;
    <[u16; 64]>::try_from(values).map_err(|v: Vec<u16>| format!("quant values: expected 64, got {}", v.len()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_bits16(b: &[u8; 16]) -> String {
    hex_encode(b)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_bits16(s: &str) -> Result<[u8; 16], String> {
    <[u8; 16]>::try_from(hex_decode(s)?).map_err(|v: Vec<u8>| format!("huffman bits: expected 16, got {}", v.len()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_thumbnail(t: &JfifThumbnail) -> String {
    format!("[{},{},{}]", t.width, t.height, hex_encode(&t.rgb_data))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_thumbnail(s: &str) -> Result<JfifThumbnail, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [width, height, rgb_data] = parts.as_slice() else { return Err(format!("thumbnail: expected 3 fields, got {}", parts.len())) };
    Ok(JfifThumbnail { width: parse_u8(width)?, height: parse_u8(height)?, rgb_data: hex_decode(rgb_data)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_segment(s: &JpgSegment) -> String {
    format!("[{},{}]", s.marker, hex_encode(&s.data))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_segment(s: &str) -> Result<JpgSegment, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [marker, data] = parts.as_slice() else { return Err(format!("segment: expected 2 fields, got {}", parts.len())) };
    Ok(JpgSegment { marker: parse_u8(marker)?, data: hex_decode(data)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_segment_diff(d: &JpgSegmentDiff) -> String {
    format!("[{},{}]", encode_option(&d.marker, |v| v.to_string()), encode_option(&d.data, |v| hex_encode(v)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_segment_diff(s: &str) -> Result<JpgSegmentDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [marker, data] = parts.as_slice() else { return Err(format!("segment diff: expected 2 fields, got {}", parts.len())) };
    Ok(JpgSegmentDiff { marker: decode_option(marker, parse_u8)?, data: decode_option(data, hex_decode)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_other_segments_diff(d: &JpgOtherSegmentsDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_segment_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_segment(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_other_segments_diff(body: &str) -> Result<JpgOtherSegmentsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("other segments diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (index, rest) = entry.split_once(':').ok_or_else(|| format!("segment modified: bad entry {entry:?}"))?;
            Ok(JpgSegmentModified { index: parse_usize(index)?, diff: dec_segment_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (index, rest) = entry.split_once(':').ok_or_else(|| format!("segment added: bad entry {entry:?}"))?;
            Ok(JpgSegmentAdded { index: parse_usize(index)?, item: dec_segment(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(JpgOtherSegmentsDiff { removed, modified, added })
}

/// 🌲 `JpgFrameChange`'s tag prefix: `M[fields-diff]` (Modify) / `R[frame-opt]` (Replace) — mirrors
/// `enc_xml_node`/`enc_node_diff`'s single-letter-tag convention (svg/gif precedent).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


/// 🧾 Top-level line: space-separated `name=value` tokens, one per changed field, absent token =
/// unchanged (recipe convention). Tri-state fields (`re-encode-quality`/`jfif-thumbnail`/
/// `restart-interval`) additionally wrap their value in `[0]`/`[1,x]` since the token's presence
/// alone only means "the tri-state slot changed", not which of {cleared, set} it changed to.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_jpg_diff(d: &JpgDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.width {
        tokens.push(format!("width={v}"));
    }
    if let Some(v) = d.height {
        tokens.push(format!("height={v}"));
    }
    if let Some(v) = &d.pixels {
        tokens.push(format!("pixels={}", hex_encode(v)));
    }
    if let Some(v) = d.jfif_version {
        tokens.push(format!("jfif-version={}", enc_version(&v)));
    }
    if let Some(v) = d.jfif_density_units {
        tokens.push(format!("jfif-density-units={}", enc_density_units(&v)));
    }
    if let Some(v) = d.jfif_x_density {
        tokens.push(format!("jfif-x-density={v}"));
    }
    if let Some(v) = d.jfif_y_density {
        tokens.push(format!("jfif-y-density={v}"));
    }
    if let Some(v) = &d.jfif_thumbnail {
        tokens.push(format!("jfif-thumbnail={}", encode_option(v, enc_thumbnail)));
    }
    if let Some(v) = &d.other_segments {
        tokens.push(format!("other-segments={}", enc_other_segments_diff(v)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_jpg_diff(line: &str) -> Result<JpgDiff, String> {
    let mut d = JpgDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("width=") {
            d.width = Some(parse_u32(rest)?);
        } else if let Some(rest) = token.strip_prefix("height=") {
            d.height = Some(parse_u32(rest)?);
        } else if let Some(rest) = token.strip_prefix("pixels=") {
            d.pixels = Some(hex_decode(rest)?);
        } else if let Some(rest) = token.strip_prefix("jfif-version=") {
            d.jfif_version = Some(dec_version(rest)?);
        } else if let Some(rest) = token.strip_prefix("jfif-density-units=") {
            d.jfif_density_units = Some(dec_density_units(rest)?);
        } else if let Some(rest) = token.strip_prefix("jfif-x-density=") {
            d.jfif_x_density = Some(parse_u16(rest)?);
        } else if let Some(rest) = token.strip_prefix("jfif-y-density=") {
            d.jfif_y_density = Some(parse_u16(rest)?);
        } else if let Some(rest) = token.strip_prefix("jfif-thumbnail=") {
            d.jfif_thumbnail = Some(decode_option(rest, dec_thumbnail)?);
        } else if let Some(rest) = token.strip_prefix("other-segments=") {
            d.other_segments = Some(dec_other_segments_diff(rest)?);
        } else {
            return Err(format!("jpg diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for JpgDiff {
fn print_diff(&self) -> String {
    print_jpg_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_jpg_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}

}
pub use diff_codec::*;
