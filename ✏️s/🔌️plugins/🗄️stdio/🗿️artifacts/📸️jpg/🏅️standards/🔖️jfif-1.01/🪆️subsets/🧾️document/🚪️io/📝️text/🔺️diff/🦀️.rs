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
use crate::schema::snapshot::{JfifDensityUnits, JfifThumbnail, JpgFrameComponent, JpgFrameHeader, JpgHuffmanClass, JpgHuffmanTable, JpgQuantTable, JpgSegment};
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
pub(crate) fn enc_huffman_class(c: &JpgHuffmanClass) -> String {
    c.to_u8().to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_huffman_class(s: &str) -> Result<JpgHuffmanClass, String> {
    JpgHuffmanClass::from_u8(parse_u8(s)?)
}

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
pub(crate) fn enc_frame_component(c: &JpgFrameComponent) -> String {
    format!("[{},{},{},{}]", c.id, c.h_sampling, c.v_sampling, c.quant_table_id)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_frame_component(s: &str) -> Result<JpgFrameComponent, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, h, v, q] = parts.as_slice() else { return Err(format!("frame component: expected 4 fields, got {}", parts.len())) };
    Ok(JpgFrameComponent { id: parse_u8(id)?, h_sampling: parse_u8(h)?, v_sampling: parse_u8(v)?, quant_table_id: parse_u8(q)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_frame_header(f: &JpgFrameHeader) -> String {
    let comps = f.components.iter().map(enc_frame_component).collect::<Vec<_>>().join(",");
    format!("[{},{},{},[{}]]", f.precision, f.width, f.height, comps)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_frame_header(s: &str) -> Result<JpgFrameHeader, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [precision, width, height, components] = parts.as_slice() else { return Err(format!("frame header: expected 4 fields, got {}", parts.len())) };
    let components = split_top_level(strip_brackets(components)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_frame_component).collect::<Result<Vec<_>, String>>()?;
    Ok(JpgFrameHeader { precision: parse_u8(precision)?, width: parse_u16(width)?, height: parse_u16(height)?, components })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_quant_table(t: &JpgQuantTable) -> String {
    format!("[{},{},{}]", t.id, t.precision, enc_quant_values(&t.values))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_quant_table(s: &str) -> Result<JpgQuantTable, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, precision, values] = parts.as_slice() else { return Err(format!("quant table: expected 3 fields, got {}", parts.len())) };
    Ok(JpgQuantTable { id: parse_u8(id)?, precision: parse_u8(precision)?, values: dec_quant_values(values)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_huffman_table(t: &JpgHuffmanTable) -> String {
    format!("[{},{},{},{}]", t.id, enc_huffman_class(&t.class), enc_bits16(&t.bits), hex_encode(&t.values))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_huffman_table(s: &str) -> Result<JpgHuffmanTable, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, class, bits, values] = parts.as_slice() else { return Err(format!("huffman table: expected 4 fields, got {}", parts.len())) };
    Ok(JpgHuffmanTable { id: parse_u8(id)?, class: dec_huffman_class(class)?, bits: dec_bits16(bits)?, values: hex_decode(values)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_huffman_key(k: &JpgHuffmanTableKey) -> String {
    format!("[{},{}]", enc_huffman_class(&k.class), k.id)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_huffman_key(s: &str) -> Result<JpgHuffmanTableKey, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [class, id] = parts.as_slice() else { return Err(format!("huffman key: expected 2 fields, got {}", parts.len())) };
    Ok(JpgHuffmanTableKey { class: dec_huffman_class(class)?, id: parse_u8(id)? })
}

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
pub(crate) fn enc_component_diff(d: &JpgComponentDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.h_sampling, |v| v.to_string()), encode_option(&d.v_sampling, |v| v.to_string()), encode_option(&d.quant_table_id, |v| v.to_string()),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_component_diff(s: &str) -> Result<JpgComponentDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [h, v, q] = parts.as_slice() else { return Err(format!("component diff: expected 3 fields, got {}", parts.len())) };
    Ok(JpgComponentDiff { h_sampling: decode_option(h, parse_u8)?, v_sampling: decode_option(v, parse_u8)?, quant_table_id: decode_option(q, parse_u8)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_components_diff(d: &JpgComponentsDiff) -> String {
    let removed = d.removed.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.id, enc_component_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_frame_component(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_components_diff(body: &str) -> Result<JpgComponentsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("components diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_u8).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (id, rest) = entry.split_once(':').ok_or_else(|| format!("component modified: bad entry {entry:?}"))?;
            Ok(JpgComponentModified { id: parse_u8(id)?, diff: dec_component_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (index, rest) = entry.split_once(':').ok_or_else(|| format!("component added: bad entry {entry:?}"))?;
            Ok(JpgComponentAdded { index: parse_usize(index)?, item: dec_frame_component(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(JpgComponentsDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_quant_table_diff(d: &JpgQuantTableDiff) -> String {
    format!("[{},{}]", encode_option(&d.precision, |v| v.to_string()), encode_option(&d.values, enc_quant_values))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_quant_table_diff(s: &str) -> Result<JpgQuantTableDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [precision, values] = parts.as_slice() else { return Err(format!("quant table diff: expected 2 fields, got {}", parts.len())) };
    Ok(JpgQuantTableDiff { precision: decode_option(precision, parse_u8)?, values: decode_option(values, dec_quant_values)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_quant_tables_diff(d: &JpgQuantTablesDiff) -> String {
    let removed = d.removed.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.id, enc_quant_table_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_quant_table(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_quant_tables_diff(body: &str) -> Result<JpgQuantTablesDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("quant tables diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_u8).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (id, rest) = entry.split_once(':').ok_or_else(|| format!("quant table modified: bad entry {entry:?}"))?;
            Ok(JpgQuantTableModified { id: parse_u8(id)?, diff: dec_quant_table_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (index, rest) = entry.split_once(':').ok_or_else(|| format!("quant table added: bad entry {entry:?}"))?;
            Ok(JpgQuantTableAdded { index: parse_usize(index)?, item: dec_quant_table(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(JpgQuantTablesDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_huffman_table_diff(d: &JpgHuffmanTableDiff) -> String {
    format!("[{},{}]", encode_option(&d.bits, enc_bits16), encode_option(&d.values, |v| hex_encode(v)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_huffman_table_diff(s: &str) -> Result<JpgHuffmanTableDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [bits, values] = parts.as_slice() else { return Err(format!("huffman table diff: expected 2 fields, got {}", parts.len())) };
    Ok(JpgHuffmanTableDiff { bits: decode_option(bits, dec_bits16)?, values: decode_option(values, hex_decode)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_huffman_tables_diff(d: &JpgHuffmanTablesDiff) -> String {
    let removed = d.removed.iter().map(enc_huffman_key).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", enc_huffman_key(&m.key), enc_huffman_table_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_huffman_table(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_huffman_tables_diff(body: &str) -> Result<JpgHuffmanTablesDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("huffman tables diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_huffman_key).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (key, rest) = entry.split_once(':').ok_or_else(|| format!("huffman table modified: bad entry {entry:?}"))?;
            Ok(JpgHuffmanTableModified { key: dec_huffman_key(key)?, diff: dec_huffman_table_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (index, rest) = entry.split_once(':').ok_or_else(|| format!("huffman table added: bad entry {entry:?}"))?;
            Ok(JpgHuffmanTableAdded { index: parse_usize(index)?, item: dec_huffman_table(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(JpgHuffmanTablesDiff { removed, modified, added })
}

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
pub(crate) fn enc_frame_change(fc: &JpgFrameChange) -> String {
    match fc {
        JpgFrameChange::Modify(fd) => format!("M[{}]", enc_frame_fields_diff(fd)),
        JpgFrameChange::Replace { frame } => format!("R[{}]", encode_option(frame, enc_frame_header)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_frame_change(s: &str) -> Result<JpgFrameChange, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "M" => Ok(JpgFrameChange::Modify(dec_frame_fields_diff(inner)?)),
        "R" => Ok(JpgFrameChange::Replace { frame: decode_option(inner, dec_frame_header)? }),
        other => Err(format!("frame change: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_frame_fields_diff(fd: &JpgFrameFieldsDiff) -> String {
    format!("[{},{},{},{}]", encode_option(&fd.precision, |v| v.to_string()), encode_option(&fd.width, |v| v.to_string()), encode_option(&fd.height, |v| v.to_string()), encode_option(&fd.components, enc_components_diff),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_frame_fields_diff(s: &str) -> Result<JpgFrameFieldsDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [precision, width, height, components] = parts.as_slice() else { return Err(format!("frame fields diff: expected 4 fields, got {}", parts.len())) };
    Ok(JpgFrameFieldsDiff { precision: decode_option(precision, parse_u8)?, width: decode_option(width, parse_u16)?, height: decode_option(height, parse_u16)?, components: decode_option(components, dec_components_diff)? })
}

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
    if let Some(v) = &d.re_encode_quality {
        tokens.push(format!("re-encode-quality={}", encode_option(v, |q| q.to_string())));
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
    if let Some(v) = &d.frame {
        tokens.push(format!("frame={}", enc_frame_change(v)));
    }
    if let Some(v) = d.sof_marker {
        tokens.push(format!("sof-marker={v}"));
    }
    if let Some(v) = d.arithmetic {
        tokens.push(format!("arithmetic={}", if v { 1 } else { 0 }));
    }
    if let Some(v) = &d.quant_tables {
        tokens.push(format!("quant-tables={}", enc_quant_tables_diff(v)));
    }
    if let Some(v) = &d.huffman_tables {
        tokens.push(format!("huffman-tables={}", enc_huffman_tables_diff(v)));
    }
    if let Some(v) = &d.restart_interval {
        tokens.push(format!("restart-interval={}", encode_option(v, |ri| ri.to_string())));
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
        } else if let Some(rest) = token.strip_prefix("re-encode-quality=") {
            d.re_encode_quality = Some(decode_option(rest, parse_u8)?);
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
        } else if let Some(rest) = token.strip_prefix("frame=") {
            d.frame = Some(dec_frame_change(rest)?);
        } else if let Some(rest) = token.strip_prefix("sof-marker=") {
            d.sof_marker = Some(parse_u8(rest)?);
        } else if let Some(rest) = token.strip_prefix("arithmetic=") {
            d.arithmetic = Some(parse_bool(rest)?);
        } else if let Some(rest) = token.strip_prefix("quant-tables=") {
            d.quant_tables = Some(dec_quant_tables_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("huffman-tables=") {
            d.huffman_tables = Some(dec_huffman_tables_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("restart-interval=") {
            d.restart_interval = Some(decode_option(rest, parse_u16)?);
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
