//! 📝️ Text representation codec surface for `stdio.gif` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type GifDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v87a::subsets::any::schema::diff::*;
use crate::standards::v87a::subsets::any::schema::snapshot::{GifColorTable, GifImage, GifRgb, GifSnapshot};
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

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
pub(crate) fn parse_u8(s: &str) -> Result<u8, String> {
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

/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only): a top-level `sep` inside nested brackets is
/// never mistaken for a field separator — the whole hand-rolled grammar's parsing primitive.
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
pub(crate) fn enc_rgb(c: &GifRgb) -> String {
    format!("[{},{},{}]", c.r, c.g, c.b)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_rgb(s: &str) -> Result<GifRgb, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [r, g, b] = parts.as_slice() else { return Err(format!("rgb: expected 3 fields, got {}", parts.len())) };
    Ok(GifRgb { r: parse_u8(r)?, g: parse_u8(g)?, b: parse_u8(b)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_color_table(t: &GifColorTable) -> String {
    let colors = t.colors.iter().map(enc_rgb).collect::<Vec<_>>().join(",");
    format!("[{},[{}]]", if t.sorted { 1 } else { 0 }, colors)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_color_table(s: &str) -> Result<GifColorTable, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [sorted, colors] = parts.as_slice() else { return Err(format!("color table: expected 2 fields, got {}", parts.len())) };
    let colors = split_top_level(strip_brackets(colors)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_rgb).collect::<Result<Vec<_>, String>>()?;
    Ok(GifColorTable { sorted: *sorted == "1", colors })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_image(f: &GifImage) -> String {
    format!("[{},{},{},{},{},{},{}]", f.left, f.top, f.width, f.height, if f.interlace { 1 } else { 0 }, encode_option(&f.lct, enc_color_table), hex_encode(&f.indices),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_image(s: &str) -> Result<GifImage, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [left, top, width, height, interlace, lct, indices] = parts.as_slice() else {
        return Err(format!("image: expected 7 fields, got {}", parts.len()));
    };
    Ok(GifImage { left: parse_u32(left)?, top: parse_u32(top)?, width: parse_u32(width)?, height: parse_u32(height)?, interlace: *interlace == "1", lct: decode_option(lct, dec_color_table)?, indices: hex_decode(indices)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_image_diff(d: &GifImageDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = d.left {
        parts.push(format!("L:{v}"));
    }
    if let Some(v) = d.top {
        parts.push(format!("T:{v}"));
    }
    if let Some(v) = d.width {
        parts.push(format!("W:{v}"));
    }
    if let Some(v) = d.height {
        parts.push(format!("H:{v}"));
    }
    if let Some(v) = d.interlace {
        parts.push(format!("I:{}", if v { 1 } else { 0 }));
    }
    if let Some(v) = &d.lct {
        parts.push(format!("C:{}", encode_option(v, enc_color_table)));
    }
    if let Some(v) = &d.indices {
        parts.push(format!("X:{}", hex_encode(v)));
    }
    format!("[{}]", parts.join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_image_diff(s: &str) -> Result<GifImageDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = GifImageDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("image diff: bad entry {entry:?}"))?;
        match tag {
            "L" => d.left = Some(parse_u32(val)?),
            "T" => d.top = Some(parse_u32(val)?),
            "W" => d.width = Some(parse_u32(val)?),
            "H" => d.height = Some(parse_u32(val)?),
            "I" => d.interlace = Some(val == "1"),
            "C" => d.lct = Some(decode_option(val, dec_color_table)?),
            "X" => d.indices = Some(hex_decode(val)?),
            other => return Err(format!("image diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

/// 🧭️ Generic-shaped 3-section `[removed];[modified];[added]` collection-triple printer/parser —
/// identical shape to gif89a's `enc_collection_triple`/`dec_collection_triple`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_collection_triple(name: &str, removed: &[usize], modified: &[(usize, String)], added: &[(usize, String)]) -> String {
    let removed = removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = modified.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    let added = added.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    format!("{name}{{[{removed}];[{modified}];[{added}]}}")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_collection_triple(body: &str) -> Result<IndexedDiffParts<String, String>, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("collection: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let parse_entries = |s: &str| -> Result<Vec<(usize, String)>, String> {
        split_top_level(strip_brackets(s)?, ',')
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|entry| {
                let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("collection entry: bad entry {entry:?}"))?;
                Ok((parse_usize(idx)?, rest.to_string()))
            })
            .collect()
    };
    Ok((removed, parse_entries(modified_s)?, parse_entries(added_s)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_images_diff(d: &GifImagesDiff) -> String {
    enc_collection_triple("images", &d.removed, &d.modified.iter().map(|m| (m.index, enc_image_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_image(&a.image))).collect::<Vec<_>>())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_images_diff(body: &str) -> Result<GifImagesDiff, String> {
    let (removed, modified, added) = dec_collection_triple(body)?;
    Ok(GifImagesDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(GifImageModified { index, diff: dec_image_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(GifImageAdded { index, image: dec_image(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_gif_diff(d: &GifDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.width {
        tokens.push(format!("width={v}"));
    }
    if let Some(v) = d.height {
        tokens.push(format!("height={v}"));
    }
    if let Some(v) = &d.gct {
        tokens.push(format!("gct={}", encode_option(v, enc_color_table)));
    }
    if let Some(v) = d.background_color_index {
        tokens.push(format!("bg={v}"));
    }
    if let Some(v) = d.pixel_aspect_ratio {
        tokens.push(format!("par={v}"));
    }
    if let Some(v) = &d.images {
        tokens.push(enc_images_diff(v));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_gif_diff(line: &str) -> Result<GifDiff, String> {
    let mut d = GifDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("width=") {
            d.width = Some(parse_u32(rest)?);
        } else if let Some(rest) = token.strip_prefix("height=") {
            d.height = Some(parse_u32(rest)?);
        } else if let Some(rest) = token.strip_prefix("gct=") {
            d.gct = Some(decode_option(rest, dec_color_table)?);
        } else if let Some(rest) = token.strip_prefix("bg=") {
            d.background_color_index = Some(parse_u8(rest)?);
        } else if let Some(rest) = token.strip_prefix("par=") {
            d.pixel_aspect_ratio = Some(parse_u8(rest)?);
        } else if let Some(rest) = token.strip_prefix("images{") {
            d.images = Some(dec_images_diff(rest.strip_suffix('}').ok_or_else(|| "images: missing closing brace".to_string())?)?);
        } else {
            return Err(format!("gif diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for GifDiff {
fn print_diff(&self) -> String {
    print_gif_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_gif_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}

}
pub use diff_codec::*;
