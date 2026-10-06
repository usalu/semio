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
use crate::standards::v89a::subsets::any::schema::diff::*;
use crate::standards::v89a::subsets::any::schema::snapshot::{GifAppExtension, GifColorTable, GifDisposal, GifFrame, GifPlainText, GifRgb, GifSnapshot};
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};






}
pub use diff_codec::*;

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v89a::subsets::any::schema::diff::*;
use crate::standards::v89a::subsets::any::schema::snapshot::{GifAppExtension, GifColorTable, GifDisposal, GifFrame, GifPlainText, GifRgb, GifSnapshot};
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
pub(crate) fn enc_plain_text(p: &GifPlainText) -> String {
    format!("[{},{},{},{},{},{},{},{},{}]", p.left, p.top, p.width, p.height, p.cell_width, p.cell_height, p.fg_color_index, p.bg_color_index, hex_encode(p.text.as_bytes()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_plain_text(s: &str) -> Result<GifPlainText, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [left, top, width, height, cw, ch, fg, bg, text] = parts.as_slice() else {
        return Err(format!("plain text: expected 9 fields, got {}", parts.len()));
    };
    Ok(GifPlainText {
        left: parse_u32(left)?,
        top: parse_u32(top)?,
        width: parse_u32(width)?,
        height: parse_u32(height)?,
        cell_width: parse_u8(cw)?,
        cell_height: parse_u8(ch)?,
        fg_color_index: parse_u8(fg)?,
        bg_color_index: parse_u8(bg)?,
        text: String::from_utf8(hex_decode(text)?).map_err(|e| e.to_string())?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_disposal(d: GifDisposal) -> char {
    match d {
        GifDisposal::Unspecified => 'u',
        GifDisposal::DoNotDispose => 'd',
        GifDisposal::RestoreToBackground => 'b',
        GifDisposal::RestoreToPrevious => 'p',
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_disposal(s: &str) -> Result<GifDisposal, String> {
    match s {
        "u" => Ok(GifDisposal::Unspecified),
        "d" => Ok(GifDisposal::DoNotDispose),
        "b" => Ok(GifDisposal::RestoreToBackground),
        "p" => Ok(GifDisposal::RestoreToPrevious),
        other => Err(format!("bad disposal {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_frame(f: &GifFrame) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{}]",
        f.left,
        f.top,
        f.width,
        f.height,
        if f.interlace { 1 } else { 0 },
        encode_option(&f.lct, enc_color_table),
        hex_encode(&f.indices),
        f.delay_cs,
        enc_disposal(f.disposal),
        encode_option(&f.transparent_index, |v| v.to_string()),
        if f.user_input { 1 } else { 0 },
        encode_option(&f.plain_text, enc_plain_text),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_frame(s: &str) -> Result<GifFrame, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [left, top, width, height, interlace, lct, indices, delay_cs, disposal, transparent_index, user_input, plain_text] = parts.as_slice() else {
        return Err(format!("frame: expected 12 fields, got {}", parts.len()));
    };
    Ok(GifFrame {
        left: parse_u32(left)?,
        top: parse_u32(top)?,
        width: parse_u32(width)?,
        height: parse_u32(height)?,
        interlace: *interlace == "1",
        lct: decode_option(lct, dec_color_table)?,
        indices: hex_decode(indices)?,
        delay_cs: parse_u16(delay_cs)?,
        disposal: dec_disposal(disposal)?,
        transparent_index: decode_option(transparent_index, parse_u8)?,
        user_input: *user_input == "1",
        plain_text: decode_option(plain_text, dec_plain_text)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_app_extension(e: &GifAppExtension) -> String {
    format!("[{},{},{}]", hex_encode(&e.identifier), hex_encode(&e.auth_code), hex_encode(&e.data))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_app_extension(s: &str) -> Result<GifAppExtension, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id_hex, auth_hex, data_hex] = parts.as_slice() else { return Err(format!("app extension: expected 3 fields, got {}", parts.len())) };
    let identifier: [u8; 8] = hex_decode(id_hex)?.try_into().map_err(|_| "app extension: identifier must be 8 bytes".to_string())?;
    let auth_code: [u8; 3] = hex_decode(auth_hex)?.try_into().map_err(|_| "app extension: auth_code must be 3 bytes".to_string())?;
    Ok(GifAppExtension { identifier, auth_code, data: hex_decode(data_hex)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_frame_diff(d: &GifFrameDiff) -> String {
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
    if let Some(v) = d.delay_cs {
        parts.push(format!("D:{v}"));
    }
    if let Some(v) = d.disposal {
        parts.push(format!("S:{}", enc_disposal(v)));
    }
    if let Some(v) = d.transparent_index {
        parts.push(format!("P:{}", encode_option(&v, |x| x.to_string())));
    }
    if let Some(v) = d.user_input {
        parts.push(format!("U:{}", if v { 1 } else { 0 }));
    }
    if let Some(v) = &d.plain_text {
        parts.push(format!("Q:{}", encode_option(v, enc_plain_text)));
    }
    format!("[{}]", parts.join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_frame_diff(s: &str) -> Result<GifFrameDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = GifFrameDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("frame diff: bad entry {entry:?}"))?;
        match tag {
            "L" => d.left = Some(parse_u32(val)?),
            "T" => d.top = Some(parse_u32(val)?),
            "W" => d.width = Some(parse_u32(val)?),
            "H" => d.height = Some(parse_u32(val)?),
            "I" => d.interlace = Some(val == "1"),
            "C" => d.lct = Some(decode_option(val, dec_color_table)?),
            "X" => d.indices = Some(hex_decode(val)?),
            "D" => d.delay_cs = Some(parse_u16(val)?),
            "S" => d.disposal = Some(dec_disposal(val)?),
            "P" => d.transparent_index = Some(decode_option(val, parse_u8)?),
            "U" => d.user_input = Some(val == "1"),
            "Q" => d.plain_text = Some(decode_option(val, dec_plain_text)?),
            other => return Err(format!("frame diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

/// 🧭️ Generic-shaped 3-section `[removed];[modified];[added]` collection-triple printer/parser,
/// hand-instantiated per weak/strong item type (mirrors `absorb_indexed_collection`'s genericity
/// above, but for text rendering instead of algebra).
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
pub(crate) fn enc_frames_diff(d: &GifFramesDiff) -> String {
    enc_collection_triple("frames", &d.removed, &d.modified.iter().map(|m| (m.index, enc_frame_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_frame(&a.frame))).collect::<Vec<_>>())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_frames_diff(body: &str) -> Result<GifFramesDiff, String> {
    let (removed, modified, added) = dec_collection_triple(body)?;
    Ok(GifFramesDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(GifFrameModified { index, diff: dec_frame_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(GifFrameAdded { index, frame: dec_frame(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_comments_diff(d: &GifCommentsDiff) -> String {
    enc_collection_triple("comments", &d.removed, &d.modified.iter().map(|m| (m.index, hex_encode(m.text.as_bytes()))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, hex_encode(a.text.as_bytes()))).collect::<Vec<_>>())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_comments_diff(body: &str) -> Result<GifCommentsDiff, String> {
    let (removed, modified, added) = dec_collection_triple(body)?;
    let text_of = |hex: &str| -> Result<String, String> { String::from_utf8(hex_decode(hex)?).map_err(|e| e.to_string()) };
    Ok(GifCommentsDiff {
        removed,
        modified: modified.into_iter().map(|(index, hex)| Ok(GifCommentModified { index, text: text_of(&hex)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, hex)| Ok(GifCommentAdded { index, text: text_of(&hex)? })).collect::<Result<Vec<_>, String>>()?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_app_extensions_diff(d: &GifAppExtensionsDiff) -> String {
    enc_collection_triple("appext", &d.removed, &d.modified.iter().map(|m| (m.index, enc_app_extension(&m.extension))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_app_extension(&a.extension))).collect::<Vec<_>>())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_app_extensions_diff(body: &str) -> Result<GifAppExtensionsDiff, String> {
    let (removed, modified, added) = dec_collection_triple(body)?;
    Ok(GifAppExtensionsDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(GifAppExtensionModified { index, extension: dec_app_extension(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(GifAppExtensionAdded { index, extension: dec_app_extension(&enc)? })).collect::<Result<Vec<_>, String>>()?,
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
    if let Some(v) = d.loop_count {
        tokens.push(format!("loop={}", encode_option(&v, |x| x.to_string())));
    }
    if let Some(v) = &d.frames {
        tokens.push(enc_frames_diff(v));
    }
    if let Some(v) = &d.comments {
        tokens.push(enc_comments_diff(v));
    }
    if let Some(v) = &d.app_extensions {
        tokens.push(enc_app_extensions_diff(v));
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
        } else if let Some(rest) = token.strip_prefix("loop=") {
            d.loop_count = Some(decode_option(rest, parse_u16)?);
        } else if let Some(rest) = token.strip_prefix("frames{") {
            d.frames = Some(dec_frames_diff(rest.strip_suffix('}').ok_or_else(|| "frames: missing closing brace".to_string())?)?);
        } else if let Some(rest) = token.strip_prefix("comments{") {
            d.comments = Some(dec_comments_diff(rest.strip_suffix('}').ok_or_else(|| "comments: missing closing brace".to_string())?)?);
        } else if let Some(rest) = token.strip_prefix("appext{") {
            d.app_extensions = Some(dec_app_extensions_diff(rest.strip_suffix('}').ok_or_else(|| "appext: missing closing brace".to_string())?)?);
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

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v89a::subsets::any::schema::snapshot::*;
use framework_schema::ArtifactSchema;

}
pub use diff_codec::*;
