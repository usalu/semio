//! 📝️ Text representation codec surface for `stdio.las` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type LasDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1_0::subsets::any::schema::diff::*;
use std::collections::{BTreeSet, HashMap, HashSet};
use crate::schema::snapshot::{LasHeader, LasPoint, LasVlr};
use crate::LasSnapshot;
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use framework_schema::ArtifactSchema;

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
pub(crate) fn parse_i8(s: &str) -> Result<i8, String> {
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
pub(crate) fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}

/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only): a top-level `sep` inside nested brackets is
/// never mistaken for a field separator — the whole hand-rolled grammar's parsing primitive
/// (identical to gif 89a's copy — own copy per artifact, no cross-artifact type sharing).
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
pub(crate) fn enc_rgb(t: &(u16, u16, u16)) -> String {
    format!("[{},{},{}]", t.0, t.1, t.2)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_rgb(s: &str) -> Result<(u16, u16, u16), String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [r, g, b] = parts.as_slice() else { return Err(format!("rgb: expected 3 fields, got {}", parts.len())) };
    Ok((parse_u16(r)?, parse_u16(g)?, parse_u16(b)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_u32x5(a: &[u32; 5]) -> String {
    format!("[{},{},{},{},{}]", a[0], a[1], a[2], a[3], a[4])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_u32x5(s: &str) -> Result<[u32; 5], String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let vals: Vec<u32> = parts.iter().map(|p| parse_u32(p)).collect::<Result<_, String>>()?;
    vals.try_into().map_err(|v: Vec<u32>| format!("points-by-return: expected 5 values, got {}", v.len()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vlr(v: &LasVlr) -> String {
    format!("[{},{},{},{}]", hex_encode(v.user_id.as_bytes()), v.record_id, hex_encode(v.description.as_bytes()), hex_encode(&v.data))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vlr(s: &str) -> Result<LasVlr, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [user_id, record_id, description, data] = parts.as_slice() else {
        return Err(format!("vlr: expected 4 fields, got {}", parts.len()));
    };
    Ok(LasVlr { user_id: String::from_utf8(hex_decode(user_id)?).map_err(|e| e.to_string())?, record_id: parse_u16(record_id)?, description: String::from_utf8(hex_decode(description)?).map_err(|e| e.to_string())?, data: hex_decode(data)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point(p: &LasPoint) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{},{},{}]",
        p.x,
        p.y,
        p.z,
        p.intensity,
        p.return_number,
        p.number_of_returns,
        if p.scan_direction_flag { 1 } else { 0 },
        if p.edge_of_flight_line { 1 } else { 0 },
        p.classification,
        p.scan_angle_rank,
        p.user_data,
        p.point_source_id,
        encode_option(&p.gps_time, |v| v.to_string()),
        encode_option(&p.rgb, enc_rgb),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point(s: &str) -> Result<LasPoint, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z, intensity, return_number, number_of_returns, scan_direction_flag, edge_of_flight_line, classification, scan_angle_rank, user_data, point_source_id, gps_time, rgb] = parts.as_slice() else {
        return Err(format!("point: expected 14 fields, got {}", parts.len()));
    };
    Ok(LasPoint {
        x: parse_f64(x)?,
        y: parse_f64(y)?,
        z: parse_f64(z)?,
        intensity: parse_u16(intensity)?,
        return_number: parse_u8(return_number)?,
        number_of_returns: parse_u8(number_of_returns)?,
        scan_direction_flag: *scan_direction_flag == "1",
        edge_of_flight_line: *edge_of_flight_line == "1",
        classification: parse_u8(classification)?,
        scan_angle_rank: parse_i8(scan_angle_rank)?,
        user_data: parse_u8(user_data)?,
        point_source_id: parse_u16(point_source_id)?,
        gps_time: decode_option(gps_time, parse_f64)?,
        rgb: decode_option(rgb, dec_rgb)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vlr_diff(d: &LasVlrDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = &d.user_id {
        parts.push(format!("U:{}", hex_encode(v.as_bytes())));
    }
    if let Some(v) = d.record_id {
        parts.push(format!("R:{v}"));
    }
    if let Some(v) = &d.description {
        parts.push(format!("N:{}", hex_encode(v.as_bytes())));
    }
    if let Some(v) = &d.data {
        parts.push(format!("X:{}", hex_encode(v)));
    }
    format!("[{}]", parts.join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vlr_diff(s: &str) -> Result<LasVlrDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = LasVlrDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("vlr diff: bad entry {entry:?}"))?;
        match tag {
            "U" => d.user_id = Some(String::from_utf8(hex_decode(val)?).map_err(|e| e.to_string())?),
            "R" => d.record_id = Some(parse_u16(val)?),
            "N" => d.description = Some(String::from_utf8(hex_decode(val)?).map_err(|e| e.to_string())?),
            "X" => d.data = Some(hex_decode(val)?),
            other => return Err(format!("vlr diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point_diff(d: &LasPointDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = d.x {
        parts.push(format!("X:{v}"));
    }
    if let Some(v) = d.y {
        parts.push(format!("Y:{v}"));
    }
    if let Some(v) = d.z {
        parts.push(format!("Z:{v}"));
    }
    if let Some(v) = d.intensity {
        parts.push(format!("I:{v}"));
    }
    if let Some(v) = d.return_number {
        parts.push(format!("R:{v}"));
    }
    if let Some(v) = d.number_of_returns {
        parts.push(format!("N:{v}"));
    }
    if let Some(v) = d.scan_direction_flag {
        parts.push(format!("D:{}", if v { 1 } else { 0 }));
    }
    if let Some(v) = d.edge_of_flight_line {
        parts.push(format!("E:{}", if v { 1 } else { 0 }));
    }
    if let Some(v) = d.classification {
        parts.push(format!("C:{v}"));
    }
    if let Some(v) = d.scan_angle_rank {
        parts.push(format!("A:{v}"));
    }
    if let Some(v) = d.user_data {
        parts.push(format!("U:{v}"));
    }
    if let Some(v) = d.point_source_id {
        parts.push(format!("P:{v}"));
    }
    if let Some(v) = d.gps_time {
        parts.push(format!("G:{}", encode_option(&v, |x| x.to_string())));
    }
    if let Some(v) = d.rgb {
        parts.push(format!("B:{}", encode_option(&v, enc_rgb)));
    }
    format!("[{}]", parts.join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point_diff(s: &str) -> Result<LasPointDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = LasPointDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("point diff: bad entry {entry:?}"))?;
        match tag {
            "X" => d.x = Some(parse_f64(val)?),
            "Y" => d.y = Some(parse_f64(val)?),
            "Z" => d.z = Some(parse_f64(val)?),
            "I" => d.intensity = Some(parse_u16(val)?),
            "R" => d.return_number = Some(parse_u8(val)?),
            "N" => d.number_of_returns = Some(parse_u8(val)?),
            "D" => d.scan_direction_flag = Some(val == "1"),
            "E" => d.edge_of_flight_line = Some(val == "1"),
            "C" => d.classification = Some(parse_u8(val)?),
            "A" => d.scan_angle_rank = Some(parse_i8(val)?),
            "U" => d.user_data = Some(parse_u8(val)?),
            "P" => d.point_source_id = Some(parse_u16(val)?),
            "G" => d.gps_time = Some(decode_option(val, parse_f64)?),
            "B" => d.rgb = Some(decode_option(val, dec_rgb)?),
            other => return Err(format!("point diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

/// 🧭️ Generic-shaped 3-section `[removed];[modified];[added]` collection-triple printer/parser
/// (identical shape to gif 89a's copy — own copy per artifact, no cross-artifact type sharing).
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
pub(crate) fn enc_vlrs_diff(d: &LasVlrsDiff) -> String {
    enc_collection_triple("vlrs", &d.removed, &d.modified.iter().map(|m| (m.index, enc_vlr_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_vlr(&a.vlr))).collect::<Vec<_>>())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vlrs_diff(body: &str) -> Result<LasVlrsDiff, String> {
    let (removed, modified, added) = dec_collection_triple(body)?;
    Ok(LasVlrsDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(LasVlrModified { index, diff: dec_vlr_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(LasVlrAdded { index, vlr: dec_vlr(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_points_diff(d: &LasPointsDiff) -> String {
    enc_collection_triple("points", &d.removed, &d.modified.iter().map(|m| (m.index, enc_point_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_point(&a.point))).collect::<Vec<_>>())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_points_diff(body: &str) -> Result<LasPointsDiff, String> {
    let (removed, modified, added) = dec_collection_triple(body)?;
    Ok(LasPointsDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(LasPointModified { index, diff: dec_point_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(LasPointAdded { index, point: dec_point(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_las_diff(d: &LasDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.version_major {
        tokens.push(format!("version-major={v}"));
    }
    if let Some(v) = d.version_minor {
        tokens.push(format!("version-minor={v}"));
    }
    if let Some(v) = &d.system_identifier {
        tokens.push(format!("system-identifier={}", hex_encode(v.as_bytes())));
    }
    if let Some(v) = &d.generating_software {
        tokens.push(format!("generating-software={}", hex_encode(v.as_bytes())));
    }
    if let Some(v) = d.creation_day_of_year {
        tokens.push(format!("creation-day-of-year={v}"));
    }
    if let Some(v) = d.creation_year {
        tokens.push(format!("creation-year={v}"));
    }
    if let Some(v) = d.header_size {
        tokens.push(format!("header-size={v}"));
    }
    if let Some(v) = d.offset_to_point_data {
        tokens.push(format!("offset-to-point-data={v}"));
    }
    if let Some(v) = d.number_of_vlrs {
        tokens.push(format!("number-of-vlrs={v}"));
    }
    if let Some(v) = d.point_data_format_id {
        tokens.push(format!("point-data-format-id={v}"));
    }
    if let Some(v) = d.point_data_record_length {
        tokens.push(format!("point-data-record-length={v}"));
    }
    if let Some(v) = d.number_of_point_records {
        tokens.push(format!("number-of-point-records={v}"));
    }
    if let Some(v) = d.points_by_return {
        tokens.push(format!("points-by-return={}", enc_u32x5(&v)));
    }
    if let Some(v) = d.x_scale {
        tokens.push(format!("x-scale={v}"));
    }
    if let Some(v) = d.y_scale {
        tokens.push(format!("y-scale={v}"));
    }
    if let Some(v) = d.z_scale {
        tokens.push(format!("z-scale={v}"));
    }
    if let Some(v) = d.x_offset {
        tokens.push(format!("x-offset={v}"));
    }
    if let Some(v) = d.y_offset {
        tokens.push(format!("y-offset={v}"));
    }
    if let Some(v) = d.z_offset {
        tokens.push(format!("z-offset={v}"));
    }
    if let Some(v) = d.max_x {
        tokens.push(format!("max-x={v}"));
    }
    if let Some(v) = d.min_x {
        tokens.push(format!("min-x={v}"));
    }
    if let Some(v) = d.max_y {
        tokens.push(format!("max-y={v}"));
    }
    if let Some(v) = d.min_y {
        tokens.push(format!("min-y={v}"));
    }
    if let Some(v) = d.max_z {
        tokens.push(format!("max-z={v}"));
    }
    if let Some(v) = d.min_z {
        tokens.push(format!("min-z={v}"));
    }
    if let Some(v) = &d.vlrs {
        tokens.push(enc_vlrs_diff(v));
    }
    if let Some(v) = &d.points {
        tokens.push(enc_points_diff(v));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_las_diff(line: &str) -> Result<LasDiff, String> {
    let mut d = LasDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("version-major=") {
            d.version_major = Some(parse_u8(rest)?);
        } else if let Some(rest) = token.strip_prefix("version-minor=") {
            d.version_minor = Some(parse_u8(rest)?);
        } else if let Some(rest) = token.strip_prefix("system-identifier=") {
            d.system_identifier = Some(String::from_utf8(hex_decode(rest)?).map_err(|e| e.to_string())?);
        } else if let Some(rest) = token.strip_prefix("generating-software=") {
            d.generating_software = Some(String::from_utf8(hex_decode(rest)?).map_err(|e| e.to_string())?);
        } else if let Some(rest) = token.strip_prefix("creation-day-of-year=") {
            d.creation_day_of_year = Some(parse_u16(rest)?);
        } else if let Some(rest) = token.strip_prefix("creation-year=") {
            d.creation_year = Some(parse_u16(rest)?);
        } else if let Some(rest) = token.strip_prefix("header-size=") {
            d.header_size = Some(parse_u16(rest)?);
        } else if let Some(rest) = token.strip_prefix("offset-to-point-data=") {
            d.offset_to_point_data = Some(parse_u32(rest)?);
        } else if let Some(rest) = token.strip_prefix("number-of-vlrs=") {
            d.number_of_vlrs = Some(parse_u32(rest)?);
        } else if let Some(rest) = token.strip_prefix("point-data-format-id=") {
            d.point_data_format_id = Some(parse_u8(rest)?);
        } else if let Some(rest) = token.strip_prefix("point-data-record-length=") {
            d.point_data_record_length = Some(parse_u16(rest)?);
        } else if let Some(rest) = token.strip_prefix("number-of-point-records=") {
            d.number_of_point_records = Some(parse_u32(rest)?);
        } else if let Some(rest) = token.strip_prefix("points-by-return=") {
            d.points_by_return = Some(dec_u32x5(rest)?);
        } else if let Some(rest) = token.strip_prefix("x-scale=") {
            d.x_scale = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("y-scale=") {
            d.y_scale = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("z-scale=") {
            d.z_scale = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("x-offset=") {
            d.x_offset = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("y-offset=") {
            d.y_offset = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("z-offset=") {
            d.z_offset = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("max-x=") {
            d.max_x = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("min-x=") {
            d.min_x = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("max-y=") {
            d.max_y = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("min-y=") {
            d.min_y = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("max-z=") {
            d.max_z = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("min-z=") {
            d.min_z = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("vlrs{") {
            d.vlrs = Some(dec_vlrs_diff(rest.strip_suffix('}').ok_or_else(|| "vlrs: missing closing brace".to_string())?)?);
        } else if let Some(rest) = token.strip_prefix("points{") {
            d.points = Some(dec_points_diff(rest.strip_suffix('}').ok_or_else(|| "points: missing closing brace".to_string())?)?);
        } else {
            return Err(format!("las diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for LasDiff {
fn print_diff(&self) -> String {
    print_las_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_las_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
