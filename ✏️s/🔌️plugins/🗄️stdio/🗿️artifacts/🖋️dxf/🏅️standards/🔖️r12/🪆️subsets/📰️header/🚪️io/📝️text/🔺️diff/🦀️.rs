//! 📝️ Text representation codec surface for `stdio.dxf` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type DxfDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_r12::subsets::any::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use crate::schema::snapshot::{DxfBlock, DxfEntity, DxfHeaderVar, DxfLayer, DxfLinetype, DxfOtherTable, DxfStyle, DxfTables, DxfTag, DxfValue, DxfVertex};
use crate::DxfSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
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
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_f64(v: f64) -> String {
    format!("{v}")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
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

/// 🧺 Self-bracketing plain list — the shared core every `Vec<T>` field's grammar uses (group
/// codes, vertices, entities, header vars, …).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point3(p: &[f64; 3]) -> String {
    format!("[{},{},{}]", enc_f64(p[0]), enc_f64(p[1]), enc_f64(p[2]))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point3(s: &str) -> Result<[f64; 3], String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok([dec_f64(x)?, dec_f64(y)?, dec_f64(z)?])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_points4(p: &[[f64; 3]; 4]) -> String {
    enc_list(p.as_slice(), enc_point3)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_points4(s: &str) -> Result<[[f64; 3]; 4], String> {
    let v = dec_list(s, dec_point3)?;
    let arr: [[f64; 3]; 4] = v.clone().try_into().map_err(|_| format!("points4: expected 4 points, got {}", v.len()))?;
    Ok(arr)
}

/// 🧮 `DxfValue` (data-carrying enum — this file's own root cause for the `DslDiff`/`DslOps`
/// rejection): `S[hex]`/`I[digits]`/`D[float]`/`P[x,y,z]`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_value(v: &DxfValue) -> String {
    match v {
        DxfValue::Str { value } => format!("S[{}]", enc_str(value)),
        DxfValue::Int { value } => format!("I[{value}]"),
        DxfValue::Double { value } => format!("D[{}]", enc_f64(*value)),
        DxfValue::Point { value } => format!("P[{},{},{}]", enc_f64(value[0]), enc_f64(value[1]), enc_f64(value[2])),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_value(s: &str) -> Result<DxfValue, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "S" => Ok(DxfValue::Str { value: dec_str(inner)? }),
        "I" => Ok(DxfValue::Int { value: inner.parse().map_err(|e: std::num::ParseIntError| e.to_string())? }),
        "D" => Ok(DxfValue::Double { value: dec_f64(inner)? }),
        "P" => {
            let parts = split_top_level(inner, ',');
            let [x, y, z] = parts.as_slice() else { return Err(format!("dxf value point: expected 3 fields, got {}", parts.len())) };
            Ok(DxfValue::Point { value: [dec_f64(x)?, dec_f64(y)?, dec_f64(z)?] })
        }
        other => Err(format!("dxf value: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_group_code(pair: &(i32, DxfValue)) -> String {
    format!("[{},{}]", pair.0, enc_dxf_value(&pair.1))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_group_code(s: &str) -> Result<(i32, DxfValue), String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [code, value] = parts.as_slice() else { return Err(format!("group code: expected 2 fields, got {}", parts.len())) };
    Ok((code.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, dec_dxf_value(value)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_group_codes(v: &[(i32, DxfValue)]) -> String {
    enc_list(v, enc_group_code)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_group_codes(s: &str) -> Result<Vec<(i32, DxfValue)>, String> {
    dec_list(s, dec_group_code)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vertex(v: &DxfVertex) -> String {
    format!("[{},{},{},{},{}]", enc_f64(v.x), enc_f64(v.y), enc_f64(v.z), enc_f64(v.bulge), enc_group_codes(&v.unknown_group_codes))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vertex(s: &str) -> Result<DxfVertex, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z, bulge, unknown] = parts.as_slice() else { return Err(format!("vertex: expected 5 fields, got {}", parts.len())) };
    Ok(DxfVertex { x: dec_f64(x)?, y: dec_f64(y)?, z: dec_f64(z)?, bulge: dec_f64(bulge)?, unknown_group_codes: dec_group_codes(unknown)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vertices(vs: &[DxfVertex]) -> String {
    enc_list(vs, enc_vertex)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vertices(s: &str) -> Result<Vec<DxfVertex>, String> {
    dec_list(s, dec_vertex)
}

/// 📐️ `DxfEntity` (data-carrying enum, the whole entity — not its diff): tag prefix per kind,
/// `L`=Line, `C`=Circle, `A`=Arc, `W`=Polyline, `T`=Text, `S`=Solid, `I`=Insert, `O`=Other.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_entity(e: &DxfEntity) -> String {
    match e {
        DxfEntity::Line { start, end, layer, unknown_group_codes } => {
            format!("L[{},{},{},{}]", enc_point3(start), enc_point3(end), enc_str(layer), enc_group_codes(unknown_group_codes))
        }
        DxfEntity::Circle { center, radius, layer, unknown_group_codes } => {
            format!("C[{},{},{},{}]", enc_point3(center), enc_f64(*radius), enc_str(layer), enc_group_codes(unknown_group_codes))
        }
        DxfEntity::Arc { center, radius, start_angle, end_angle, layer, unknown_group_codes } => {
            format!("A[{},{},{},{},{},{}]", enc_point3(center), enc_f64(*radius), enc_f64(*start_angle), enc_f64(*end_angle), enc_str(layer), enc_group_codes(unknown_group_codes))
        }
        DxfEntity::Polyline { vertices, closed, layer, unknown_group_codes } => format!("W[{},{},{},{}]", enc_vertices(vertices), if *closed { "1" } else { "0" }, enc_str(layer), enc_group_codes(unknown_group_codes)),
        DxfEntity::Text { position, height, value, layer, unknown_group_codes } => format!("T[{},{},{},{},{}]", enc_point3(position), enc_f64(*height), enc_str(value), enc_str(layer), enc_group_codes(unknown_group_codes)),
        DxfEntity::Solid { points, layer, unknown_group_codes } => {
            format!("S[{},{},{}]", enc_points4(points), enc_str(layer), enc_group_codes(unknown_group_codes))
        }
        DxfEntity::Insert { block_name, position, scale, rotation, layer, unknown_group_codes } => {
            format!("I[{},{},{},{},{},{}]", enc_str(block_name), enc_point3(position), enc_point3(scale), enc_f64(*rotation), enc_str(layer), enc_group_codes(unknown_group_codes))
        }
        DxfEntity::Other { kind, group_codes } => format!("O[{},{}]", enc_str(kind), enc_group_codes(group_codes)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_entity(s: &str) -> Result<DxfEntity, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "L" => {
            let parts = split_top_level(inner, ',');
            let [start, end, layer, unknown] = parts.as_slice() else { return Err(format!("entity line: expected 4 fields, got {}", parts.len())) };
            Ok(DxfEntity::Line { start: dec_point3(start)?, end: dec_point3(end)?, layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? })
        }
        "C" => {
            let parts = split_top_level(inner, ',');
            let [center, radius, layer, unknown] = parts.as_slice() else { return Err(format!("entity circle: expected 4 fields, got {}", parts.len())) };
            Ok(DxfEntity::Circle { center: dec_point3(center)?, radius: dec_f64(radius)?, layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? })
        }
        "A" => {
            let parts = split_top_level(inner, ',');
            let [center, radius, start_angle, end_angle, layer, unknown] = parts.as_slice() else { return Err(format!("entity arc: expected 6 fields, got {}", parts.len())) };
            Ok(DxfEntity::Arc { center: dec_point3(center)?, radius: dec_f64(radius)?, start_angle: dec_f64(start_angle)?, end_angle: dec_f64(end_angle)?, layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? })
        }
        "W" => {
            let parts = split_top_level(inner, ',');
            let [vertices, closed, layer, unknown] = parts.as_slice() else { return Err(format!("entity polyline: expected 4 fields, got {}", parts.len())) };
            Ok(DxfEntity::Polyline { vertices: dec_vertices(vertices)?, closed: *closed == "1", layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? })
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [position, height, value, layer, unknown] = parts.as_slice() else { return Err(format!("entity text: expected 5 fields, got {}", parts.len())) };
            Ok(DxfEntity::Text { position: dec_point3(position)?, height: dec_f64(height)?, value: dec_str(value)?, layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? })
        }
        "S" => {
            let parts = split_top_level(inner, ',');
            let [points, layer, unknown] = parts.as_slice() else { return Err(format!("entity solid: expected 3 fields, got {}", parts.len())) };
            Ok(DxfEntity::Solid { points: dec_points4(points)?, layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? })
        }
        "I" => {
            let parts = split_top_level(inner, ',');
            let [block_name, position, scale, rotation, layer, unknown] = parts.as_slice() else { return Err(format!("entity insert: expected 6 fields, got {}", parts.len())) };
            Ok(DxfEntity::Insert { block_name: dec_str(block_name)?, position: dec_point3(position)?, scale: dec_point3(scale)?, rotation: dec_f64(rotation)?, layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? })
        }
        "O" => {
            let parts = split_top_level(inner, ',');
            let [kind, group_codes] = parts.as_slice() else { return Err(format!("entity other: expected 2 fields, got {}", parts.len())) };
            Ok(DxfEntity::Other { kind: dec_str(kind)?, group_codes: dec_group_codes(group_codes)? })
        }
        other => Err(format!("dxf entity: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_entities(es: &[DxfEntity]) -> String {
    enc_list(es, enc_dxf_entity)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_entities(s: &str) -> Result<Vec<DxfEntity>, String> {
    dec_list(s, dec_dxf_entity)
}

/// 🏷️ Full (non-diff) item encoders — self-bracketing positional tuples, used by `added` entries
/// in every collection triple AND by `🧬️mutations`'s `Insert*`/`Set*` argument
/// payloads (hence `pub(crate)`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_header_var(hv: &DxfHeaderVar) -> String {
    format!("[{},{},{},{}]", enc_str(&hv.name), hv.group_code, enc_dxf_value(&hv.value), enc_group_codes(&hv.extra_group_codes))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_header_var(s: &str) -> Result<DxfHeaderVar, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, group_code, value, extra] = parts.as_slice() else { return Err(format!("header var: expected 4 fields, got {}", parts.len())) };
    Ok(DxfHeaderVar { name: dec_str(name)?, group_code: group_code.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, value: dec_dxf_value(value)?, extra_group_codes: dec_group_codes(extra)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_layer(l: &DxfLayer) -> String {
    format!("[{},{},{},{},{}]", enc_str(&l.name), l.color, enc_str(&l.linetype), l.flags, enc_group_codes(&l.unknown_group_codes))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_layer(s: &str) -> Result<DxfLayer, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, color, linetype, flags, unknown] = parts.as_slice() else { return Err(format!("layer: expected 5 fields, got {}", parts.len())) };
    Ok(DxfLayer {
        name: dec_str(name)?,
        color: color.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        linetype: dec_str(linetype)?,
        flags: flags.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        unknown_group_codes: dec_group_codes(unknown)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_style(s: &DxfStyle) -> String {
    format!("[{},{},{},{}]", enc_str(&s.name), s.flags, enc_str(&s.font_name), enc_group_codes(&s.unknown_group_codes))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_style(s: &str) -> Result<DxfStyle, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, flags, font_name, unknown] = parts.as_slice() else { return Err(format!("style: expected 4 fields, got {}", parts.len())) };
    Ok(DxfStyle { name: dec_str(name)?, flags: flags.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, font_name: dec_str(font_name)?, unknown_group_codes: dec_group_codes(unknown)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_linetype(l: &DxfLinetype) -> String {
    format!("[{},{},{},{}]", enc_str(&l.name), l.flags, enc_str(&l.description), enc_group_codes(&l.unknown_group_codes))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_linetype(s: &str) -> Result<DxfLinetype, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, flags, description, unknown] = parts.as_slice() else { return Err(format!("linetype: expected 4 fields, got {}", parts.len())) };
    Ok(DxfLinetype { name: dec_str(name)?, flags: flags.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, description: dec_str(description)?, unknown_group_codes: dec_group_codes(unknown)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block(b: &DxfBlock) -> String {
    format!("[{},{},{},{}]", enc_str(&b.name), enc_point3(&b.base_point), enc_dxf_entities(&b.entities), enc_group_codes(&b.unknown_group_codes))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block(s: &str) -> Result<DxfBlock, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, base_point, entities, unknown] = parts.as_slice() else { return Err(format!("block: expected 4 fields, got {}", parts.len())) };
    Ok(DxfBlock { name: dec_str(name)?, base_point: dec_point3(base_point)?, entities: dec_dxf_entities(entities)?, unknown_group_codes: dec_group_codes(unknown)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_header_var_diff(d: &DxfHeaderVarDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.group_code, |v| v.to_string()), encode_option(&d.value, enc_dxf_value), encode_option(&d.extra_group_codes, |v| enc_group_codes(v)),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_header_var_diff(s: &str) -> Result<DxfHeaderVarDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [group_code, value, extra] = parts.as_slice() else { return Err(format!("header var diff: expected 3 fields, got {}", parts.len())) };
    Ok(DxfHeaderVarDiff { group_code: decode_option(group_code, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?, value: decode_option(value, dec_dxf_value)?, extra_group_codes: decode_option(extra, dec_group_codes)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_layer_diff(d: &DxfLayerDiff) -> String {
    format!("[{},{},{},{}]", encode_option(&d.color, |v| v.to_string()), encode_option(&d.linetype, |v| enc_str(v)), encode_option(&d.flags, |v| v.to_string()), encode_option(&d.unknown_group_codes, |v| enc_group_codes(v)),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_layer_diff(s: &str) -> Result<DxfLayerDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [color, linetype, flags, unknown] = parts.as_slice() else { return Err(format!("layer diff: expected 4 fields, got {}", parts.len())) };
    Ok(DxfLayerDiff {
        color: decode_option(color, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?,
        linetype: decode_option(linetype, dec_str)?,
        flags: decode_option(flags, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?,
        unknown_group_codes: decode_option(unknown, dec_group_codes)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_style_diff(d: &DxfStyleDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.flags, |v| v.to_string()), encode_option(&d.font_name, |v| enc_str(v)), encode_option(&d.unknown_group_codes, |v| enc_group_codes(v)),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_style_diff(s: &str) -> Result<DxfStyleDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [flags, font_name, unknown] = parts.as_slice() else { return Err(format!("style diff: expected 3 fields, got {}", parts.len())) };
    Ok(DxfStyleDiff { flags: decode_option(flags, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?, font_name: decode_option(font_name, dec_str)?, unknown_group_codes: decode_option(unknown, dec_group_codes)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_linetype_diff(d: &DxfLinetypeDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.flags, |v| v.to_string()), encode_option(&d.description, |v| enc_str(v)), encode_option(&d.unknown_group_codes, |v| enc_group_codes(v)),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_linetype_diff(s: &str) -> Result<DxfLinetypeDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [flags, description, unknown] = parts.as_slice() else { return Err(format!("linetype diff: expected 3 fields, got {}", parts.len())) };
    Ok(DxfLinetypeDiff { flags: decode_option(flags, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?, description: decode_option(description, dec_str)?, unknown_group_codes: decode_option(unknown, dec_group_codes)? })
}

/// 🌳 `DxfEntityDiff` itself needs a tag (like `SvgNodeDiff`/`XmlNode`) since it appears standalone
/// at the `entities=`/`blocks=`'s nested `modified` entry position, not always inside an
/// already-disambiguating container. `R`=Replace (carries a WHOLE `DxfEntity`, itself tag-prefixed
/// — a tag-inside-a-tag, unambiguous since the outer bracket always closes the inner one first),
/// else one letter per typed entity kind (same letters `enc_dxf_entity` uses).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entity_diff(d: &DxfEntityDiff) -> String {
    match d {
        DxfEntityDiff::Replace { entity } => format!("R[{}]", enc_dxf_entity(entity)),
        DxfEntityDiff::Line(x) => format!("L[{},{},{},{}]", encode_option(&x.start, enc_point3), encode_option(&x.end, enc_point3), encode_option(&x.layer, |v| enc_str(v)), encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))),
        DxfEntityDiff::Circle(x) => format!("C[{},{},{},{}]", encode_option(&x.center, enc_point3), encode_option(&x.radius, |v| enc_f64(*v)), encode_option(&x.layer, |v| enc_str(v)), encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))),
        DxfEntityDiff::Arc(x) => format!(
            "A[{},{},{},{},{},{}]",
            encode_option(&x.center, enc_point3),
            encode_option(&x.radius, |v| enc_f64(*v)),
            encode_option(&x.start_angle, |v| enc_f64(*v)),
            encode_option(&x.end_angle, |v| enc_f64(*v)),
            encode_option(&x.layer, |v| enc_str(v)),
            encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))
        ),
        DxfEntityDiff::Polyline(x) => format!(
            "W[{},{},{},{}]",
            encode_option(&x.vertices, |v| enc_vertices(v)),
            encode_option(&x.closed, |v| if *v { "1".to_string() } else { "0".to_string() }),
            encode_option(&x.layer, |v| enc_str(v)),
            encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))
        ),
        DxfEntityDiff::Text(x) => format!(
            "T[{},{},{},{},{}]",
            encode_option(&x.position, enc_point3),
            encode_option(&x.height, |v| enc_f64(*v)),
            encode_option(&x.value, |v| enc_str(v)),
            encode_option(&x.layer, |v| enc_str(v)),
            encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))
        ),
        DxfEntityDiff::Solid(x) => format!("S[{},{},{}]", encode_option(&x.points, enc_points4), encode_option(&x.layer, |v| enc_str(v)), encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))),
        DxfEntityDiff::Insert(x) => format!(
            "I[{},{},{},{},{},{}]",
            encode_option(&x.block_name, |v| enc_str(v)),
            encode_option(&x.position, enc_point3),
            encode_option(&x.scale, enc_point3),
            encode_option(&x.rotation, |v| enc_f64(*v)),
            encode_option(&x.layer, |v| enc_str(v)),
            encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))
        ),
        DxfEntityDiff::Other(x) => format!("O[{}]", encode_option(&x.group_codes, |v| enc_group_codes(v))),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entity_diff(s: &str) -> Result<DxfEntityDiff, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "R" => Ok(DxfEntityDiff::Replace { entity: dec_dxf_entity(inner)? }),
        "L" => {
            let parts = split_top_level(inner, ',');
            let [start, end, layer, unknown] = parts.as_slice() else { return Err(format!("line diff: expected 4 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Line(DxfLineDiff { start: decode_option(start, dec_point3)?, end: decode_option(end, dec_point3)?, layer: decode_option(layer, dec_str)?, unknown_group_codes: decode_option(unknown, dec_group_codes)? }))
        }
        "C" => {
            let parts = split_top_level(inner, ',');
            let [center, radius, layer, unknown] = parts.as_slice() else { return Err(format!("circle diff: expected 4 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Circle(DxfCircleDiff { center: decode_option(center, dec_point3)?, radius: decode_option(radius, dec_f64)?, layer: decode_option(layer, dec_str)?, unknown_group_codes: decode_option(unknown, dec_group_codes)? }))
        }
        "A" => {
            let parts = split_top_level(inner, ',');
            let [center, radius, start_angle, end_angle, layer, unknown] = parts.as_slice() else { return Err(format!("arc diff: expected 6 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Arc(DxfArcDiff {
                center: decode_option(center, dec_point3)?,
                radius: decode_option(radius, dec_f64)?,
                start_angle: decode_option(start_angle, dec_f64)?,
                end_angle: decode_option(end_angle, dec_f64)?,
                layer: decode_option(layer, dec_str)?,
                unknown_group_codes: decode_option(unknown, dec_group_codes)?,
            }))
        }
        "W" => {
            let parts = split_top_level(inner, ',');
            let [vertices, closed, layer, unknown] = parts.as_slice() else { return Err(format!("polyline diff: expected 4 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Polyline(DxfPolylineDiff {
                vertices: decode_option(vertices, dec_vertices)?,
                closed: decode_option(closed, |v| Ok(v == "1"))?,
                layer: decode_option(layer, dec_str)?,
                unknown_group_codes: decode_option(unknown, dec_group_codes)?,
            }))
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [position, height, value, layer, unknown] = parts.as_slice() else { return Err(format!("text diff: expected 5 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Text(DxfTextDiff {
                position: decode_option(position, dec_point3)?,
                height: decode_option(height, dec_f64)?,
                value: decode_option(value, dec_str)?,
                layer: decode_option(layer, dec_str)?,
                unknown_group_codes: decode_option(unknown, dec_group_codes)?,
            }))
        }
        "S" => {
            let parts = split_top_level(inner, ',');
            let [points, layer, unknown] = parts.as_slice() else { return Err(format!("solid diff: expected 3 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Solid(DxfSolidDiff { points: decode_option(points, dec_points4)?, layer: decode_option(layer, dec_str)?, unknown_group_codes: decode_option(unknown, dec_group_codes)? }))
        }
        "I" => {
            let parts = split_top_level(inner, ',');
            let [block_name, position, scale, rotation, layer, unknown] = parts.as_slice() else { return Err(format!("insert diff: expected 6 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Insert(DxfInsertDiff {
                block_name: decode_option(block_name, dec_str)?,
                position: decode_option(position, dec_point3)?,
                scale: decode_option(scale, dec_point3)?,
                rotation: decode_option(rotation, dec_f64)?,
                layer: decode_option(layer, dec_str)?,
                unknown_group_codes: decode_option(unknown, dec_group_codes)?,
            }))
        }
        "O" => Ok(DxfEntityDiff::Other(DxfOtherDiff { group_codes: decode_option(inner, dec_group_codes)? })),
        other => Err(format!("entity diff: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_diff(d: &DxfBlockDiff) -> String {
    format!("[{},{},{},{}]", encode_option(&d.name, |v| enc_str(v)), encode_option(&d.base_point, enc_point3), encode_option(&d.entities, enc_entities_diff), encode_option(&d.unknown_group_codes, |v| enc_group_codes(v)),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_diff(s: &str) -> Result<DxfBlockDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, base_point, entities, unknown] = parts.as_slice() else { return Err(format!("block diff: expected 4 fields, got {}", parts.len())) };
    Ok(DxfBlockDiff { name: decode_option(name, dec_str)?, base_point: decode_option(base_point, dec_point3)?, entities: decode_option(entities, dec_entities_diff)?, unknown_group_codes: decode_option(unknown, dec_group_codes)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_tables_diff(t: &DxfTablesDiff) -> String {
    format!("[{},{},{}]", encode_option(&t.layers, enc_layers_diff), encode_option(&t.styles, enc_styles_diff), encode_option(&t.linetypes, enc_linetypes_diff),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_tables_diff(s: &str) -> Result<DxfTablesDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [layers, styles, linetypes] = parts.as_slice() else { return Err(format!("tables diff: expected 3 fields, got {}", parts.len())) };
    Ok(DxfTablesDiff { layers: decode_option(layers, dec_layers_diff)?, styles: decode_option(styles, dec_styles_diff)?, linetypes: decode_option(linetypes, dec_linetypes_diff)? })
}

/// 🧮 Generic name-keyed `[removed];[modified];[added]` core — mirrors this file's own
/// `DxfNamedElem`/`named_between` generic core above, one level up (string grammar instead of
/// structural diff algebra). `modified` entries are `hexname:diffpayload`, `added` entries are
/// `index:fullitempayload` (recipe's own convention — the index is always a bare decimal
/// preceding the first colon, unambiguous since no payload anywhere in this grammar emits a
/// literal `:`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_name_triple<T, D>(removed: &[String], modified: &[(String, D)], added: &[(usize, T)], enc_diff: impl Fn(&D) -> String, enc_item: impl Fn(&T) -> String) -> String {
    let removed_s = removed.iter().map(|n| enc_str(n)).collect::<Vec<_>>().join(",");
    let modified_s = modified.iter().map(|(n, d)| format!("{}:{}", enc_str(n), enc_diff(d))).collect::<Vec<_>>().join(",");
    let added_s = added.iter().map(|(i, t)| format!("{i}:{}", enc_item(t))).collect::<Vec<_>>().join(",");
    format!("[{removed_s}];[{modified_s}];[{added_s}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_name_triple<T, D>(body: &str, dec_diff: impl Fn(&str) -> Result<D, String>, dec_item: impl Fn(&str) -> Result<T, String>) -> Result<NamedDiffParts<D, T>, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("name triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (name, rest) = entry.split_once(':').ok_or_else(|| format!("name triple modified: bad entry {entry:?}"))?;
            Ok((dec_str(name)?, dec_diff(rest)?))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("name triple added: bad entry {entry:?}"))?;
            Ok((parse_usize(idx)?, dec_item(rest)?))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok((removed, modified, added))
}

/// 🧮 Generic index-keyed twin of [`enc_name_triple`]/[`dec_name_triple`] — used by
/// `entities`/`blocks` (both top-level and, for `entities`, nested inside a block diff).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_index_triple<T, D>(removed: &[usize], modified: &[(usize, D)], added: &[(usize, T)], enc_diff: impl Fn(&D) -> String, enc_item: impl Fn(&T) -> String) -> String {
    let removed_s = removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified_s = modified.iter().map(|(i, d)| format!("{i}:{}", enc_diff(d))).collect::<Vec<_>>().join(",");
    let added_s = added.iter().map(|(i, t)| format!("{i}:{}", enc_item(t))).collect::<Vec<_>>().join(",");
    format!("[{removed_s}];[{modified_s}];[{added_s}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_index_triple<T, D>(body: &str, dec_diff: impl Fn(&str) -> Result<D, String>, dec_item: impl Fn(&str) -> Result<T, String>) -> Result<IndexedDiffParts<D, T>, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("index triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("index triple modified: bad entry {entry:?}"))?;
            Ok((parse_usize(idx)?, dec_diff(rest)?))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("index triple added: bad entry {entry:?}"))?;
            Ok((parse_usize(idx)?, dec_item(rest)?))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok((removed, modified, added))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_header_vars_diff(d: &DxfHeaderVarsDiff) -> String {
    let modified: Vec<(String, DxfHeaderVarDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfHeaderVar)> = d.added.iter().map(|a| (a.index, a.header_var.clone())).collect();
    enc_name_triple(&d.removed, &modified, &added, enc_header_var_diff, enc_header_var)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_header_vars_diff(s: &str) -> Result<DxfHeaderVarsDiff, String> {
    let (removed, modified, added) = dec_name_triple(s, dec_header_var_diff, dec_header_var)?;
    Ok(DxfHeaderVarsDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfHeaderVarModified { name, diff }).collect(), added: added.into_iter().map(|(index, header_var)| DxfHeaderVarAdded { index, header_var }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_layers_diff(d: &DxfLayersDiff) -> String {
    let modified: Vec<(String, DxfLayerDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfLayer)> = d.added.iter().map(|a| (a.index, a.layer.clone())).collect();
    enc_name_triple(&d.removed, &modified, &added, enc_layer_diff, enc_layer)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_layers_diff(s: &str) -> Result<DxfLayersDiff, String> {
    let (removed, modified, added) = dec_name_triple(s, dec_layer_diff, dec_layer)?;
    Ok(DxfLayersDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfLayerModified { name, diff }).collect(), added: added.into_iter().map(|(index, layer)| DxfLayerAdded { index, layer }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_styles_diff(d: &DxfStylesDiff) -> String {
    let modified: Vec<(String, DxfStyleDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfStyle)> = d.added.iter().map(|a| (a.index, a.style.clone())).collect();
    enc_name_triple(&d.removed, &modified, &added, enc_style_diff, enc_style)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_styles_diff(s: &str) -> Result<DxfStylesDiff, String> {
    let (removed, modified, added) = dec_name_triple(s, dec_style_diff, dec_style)?;
    Ok(DxfStylesDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfStyleModified { name, diff }).collect(), added: added.into_iter().map(|(index, style)| DxfStyleAdded { index, style }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_linetypes_diff(d: &DxfLinetypesDiff) -> String {
    let modified: Vec<(String, DxfLinetypeDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfLinetype)> = d.added.iter().map(|a| (a.index, a.linetype.clone())).collect();
    enc_name_triple(&d.removed, &modified, &added, enc_linetype_diff, enc_linetype)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_linetypes_diff(s: &str) -> Result<DxfLinetypesDiff, String> {
    let (removed, modified, added) = dec_name_triple(s, dec_linetype_diff, dec_linetype)?;
    Ok(DxfLinetypesDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfLinetypeModified { name, diff }).collect(), added: added.into_iter().map(|(index, linetype)| DxfLinetypeAdded { index, linetype }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entities_diff(d: &DxfEntitiesDiff) -> String {
    let modified: Vec<(usize, DxfEntityDiff)> = d.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let added: Vec<(usize, DxfEntity)> = d.added.iter().map(|a| (a.index, a.entity.clone())).collect();
    enc_index_triple(&d.removed, &modified, &added, enc_entity_diff, enc_dxf_entity)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entities_diff(s: &str) -> Result<DxfEntitiesDiff, String> {
    let (removed, modified, added) = dec_index_triple(s, dec_entity_diff, dec_dxf_entity)?;
    Ok(DxfEntitiesDiff { removed, modified: modified.into_iter().map(|(index, diff)| DxfEntityModified { index, diff }).collect(), added: added.into_iter().map(|(index, entity)| DxfEntityAdded { index, entity }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_blocks_diff(d: &DxfBlocksDiff) -> String {
    let modified: Vec<(usize, DxfBlockDiff)> = d.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let added: Vec<(usize, DxfBlock)> = d.added.iter().map(|a| (a.index, a.block.clone())).collect();
    enc_index_triple(&d.removed, &modified, &added, enc_block_diff, enc_block)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_blocks_diff(s: &str) -> Result<DxfBlocksDiff, String> {
    let (removed, modified, added) = dec_index_triple(s, dec_block_diff, dec_block)?;
    Ok(DxfBlocksDiff { removed, modified: modified.into_iter().map(|(index, diff)| DxfBlockModified { index, diff }).collect(), added: added.into_iter().map(|(index, block)| DxfBlockAdded { index, block }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_dxf_diff(d: &DxfDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.header_vars {
        tokens.push(format!("header-vars={}", enc_header_vars_diff(v)));
    }
    if let Some(v) = &d.tables {
        tokens.push(format!("tables={}", enc_tables_diff(v)));
    }
    if let Some(v) = &d.blocks {
        tokens.push(format!("blocks={}", enc_blocks_diff(v)));
    }
    if let Some(v) = &d.entities {
        tokens.push(format!("entities={}", enc_entities_diff(v)));
    }
    if let Some(v) = &d.other_tables {
        tokens.push(format!("other-tables={}", enc_list(v, enc_other_table)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_dxf_diff(line: &str) -> Result<DxfDiff, String> {
    let mut d = DxfDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("header-vars=") {
            d.header_vars = Some(dec_header_vars_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("tables=") {
            d.tables = Some(dec_tables_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("blocks=") {
            d.blocks = Some(dec_blocks_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("entities=") {
            d.entities = Some(dec_entities_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("other-tables=") {
            d.other_tables = Some(dec_list(rest, dec_other_table)?);
        } else {
            return Err(format!("dxf diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for DxfDiff {
fn print_diff(&self) -> String {
    print_dxf_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_dxf_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;

#[allow(unused_imports)]
mod diff_wire_codec {
use super::*;
use crate::standards::v_r12::subsets::any::schema::diff::*;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use crate::schema::snapshot::{DxfBlock, DxfEntity, DxfHeaderVar, DxfLayer, DxfLinetype, DxfOtherTable, DxfStyle, DxfTables, DxfTag, DxfValue, DxfVertex};
use crate::DxfSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_tag(t: &DxfTag) -> String {
    format!("[{},{}]", t.code, enc_str(&t.value))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_tag(s: &str) -> Result<DxfTag, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [code, value] = parts.as_slice() else { return Err(format!("tag: expected 2 fields, got {}", parts.len())) };
    Ok(DxfTag { code: code.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, value: dec_str(value)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_other_table(t: &DxfOtherTable) -> String {
    format!("[{},{}]", enc_str(&t.name), enc_list(&t.tags, enc_dxf_tag))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_other_table(s: &str) -> Result<DxfOtherTable, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, tags] = parts.as_slice() else { return Err(format!("other table: expected 2 fields, got {}", parts.len())) };
    Ok(DxfOtherTable { name: dec_str(name)?, tags: dec_list(tags, dec_dxf_tag)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_tables(t: &DxfTables) -> String {
    format!("[{},{},{}]", enc_list(&t.layers, enc_layer), enc_list(&t.styles, enc_style), enc_list(&t.linetypes, enc_linetype))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_tables(s: &str) -> Result<DxfTables, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [layers, styles, linetypes] = parts.as_slice() else { return Err(format!("tables: expected 3 fields, got {}", parts.len())) };
    Ok(DxfTables { layers: dec_list(layers, dec_layer)?, styles: dec_list(styles, dec_style)?, linetypes: dec_list(linetypes, dec_linetype)? })
}
}
pub use diff_wire_codec::*;
