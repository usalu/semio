//! binary rep for stdio.dxf 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_r12::subsets::any::schema::diff::*;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use crate::schema::snapshot::{DxfBlock, DxfEntity, DxfHeaderVar, DxfLayer, DxfLinetype, DxfOtherTable, DxfStyle, DxfTables, DxfTag, DxfValue, DxfVertex};
use crate::DxfSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

/// 🧪️ P2-FG1: real LEB128-varint-framed binary primitives (length-prefixed bytes/utf8, tri-state
/// `Option<T>` presence byte) backing the upgraded `OpBinary`/`DiffCodec` frames below — reuses
/// `store::write_varint_u64`/`store::write_varint_i64`/`store::ByteReader` (crate-root re-exports
/// of `os_pack`'s own varint/reader primitives — `store`/`dsl`/`protocol` are all `extern crate
/// self as …` aliases for the SAME kernel crate, see `🦀️.rs`) rather than reinventing varint
/// encode/decode. Same shape `stdio.json`'s own upgrade introduced
/// (`🔣️json/…/🔺️diff/🦀️.rs`'s `#region 🔖️BinaryPrimitives`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_option_bin<T>(out: &mut Vec<u8>, opt: &Option<T>, enc: impl Fn(&T, &mut Vec<u8>)) {
    match opt {
        None => out.push(0),
        Some(v) => {
            out.push(1);
            enc(v, out);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_option_bin<T>(reader: &mut store::ByteReader<'_>, dec: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Option<T>, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(None),
        1 => Ok(Some(dec(reader)?)),
        other => Err(format!("option binary: bad tag {other}")),
    }
}

/// 🧪️ P2-FG1: real recursive binary twins of every `enc_*`/`dec_*` item-literal codec above
/// (`#region 🔖️GeometryCodecs`/`#region 🔖️ValueCodecs`/`#region 🔖️EntityValueCodecs`/
/// `#region 🔖️ItemCodecs`) — backs the upgraded `OpBinary` frame
/// (`../🧬️mutations/🦀️.rs`) and every collection's `added`-entry / `entity-diff`'s
/// `Replace`-arm payload below. Tag numbering mirrors each text codec's own letter-tag order
/// (`0`=first letter, `1`=second, …), independent per type (not reused across unrelated shapes).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_f64_bin(out: &mut Vec<u8>, v: f64) {
    out.extend_from_slice(&v.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_f64_bin(reader: &mut store::ByteReader<'_>) -> Result<f64, String> {
    reader.read_f64_le().map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_point3_bin(out: &mut Vec<u8>, p: &[f64; 3]) {
    write_f64_bin(out, p[0]);
    write_f64_bin(out, p[1]);
    write_f64_bin(out, p[2]);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_point3_bin(reader: &mut store::ByteReader<'_>) -> Result<[f64; 3], String> {
    Ok([read_f64_bin(reader)?, read_f64_bin(reader)?, read_f64_bin(reader)?])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_points4_bin(out: &mut Vec<u8>, p: &[[f64; 3]; 4]) {
    for pt in p {
        write_point3_bin(out, pt);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_points4_bin(reader: &mut store::ByteReader<'_>) -> Result<[[f64; 3]; 4], String> {
    Ok([read_point3_bin(reader)?, read_point3_bin(reader)?, read_point3_bin(reader)?, read_point3_bin(reader)?])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_value_bin(v: &DxfValue, out: &mut Vec<u8>) {
    match v {
        DxfValue::Str { value } => {
            out.push(0);
            write_str_lp(out, value);
        }
        DxfValue::Int { value } => {
            out.push(1);
            store::write_varint_i64(out, *value);
        }
        DxfValue::Double { value } => {
            out.push(2);
            write_f64_bin(out, *value);
        }
        DxfValue::Point { value } => {
            out.push(3);
            write_point3_bin(out, value);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_value_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfValue, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(DxfValue::Str { value: read_str_lp(reader)? }),
        1 => Ok(DxfValue::Int { value: reader.read_varint_i64().map_err(|e| e.to_string())? }),
        2 => Ok(DxfValue::Double { value: read_f64_bin(reader)? }),
        3 => Ok(DxfValue::Point { value: read_point3_bin(reader)? }),
        other => Err(format!("dxf value binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_group_code_bin(pair: &(i32, DxfValue), out: &mut Vec<u8>) {
    store::write_varint_i64(out, pair.0 as i64);
    enc_dxf_value_bin(&pair.1, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_group_code_bin(reader: &mut store::ByteReader<'_>) -> Result<(i32, DxfValue), String> {
    let code = reader.read_varint_i64().map_err(|e| e.to_string())? as i32;
    let value = dec_dxf_value_bin(reader)?;
    Ok((code, value))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_group_codes_bin(v: &[(i32, DxfValue)], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, v.len() as u64);
    for pair in v {
        enc_group_code_bin(pair, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_group_codes_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<(i32, DxfValue)>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(dec_group_code_bin(reader)?);
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vertex_bin(v: &DxfVertex, out: &mut Vec<u8>) {
    write_f64_bin(out, v.x);
    write_f64_bin(out, v.y);
    write_f64_bin(out, v.z);
    write_f64_bin(out, v.bulge);
    enc_group_codes_bin(&v.unknown_group_codes, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vertex_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfVertex, String> {
    let x = read_f64_bin(reader)?;
    let y = read_f64_bin(reader)?;
    let z = read_f64_bin(reader)?;
    let bulge = read_f64_bin(reader)?;
    let unknown_group_codes = dec_group_codes_bin(reader)?;
    Ok(DxfVertex { x, y, z, bulge, unknown_group_codes })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vertices_bin(vs: &[DxfVertex], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, vs.len() as u64);
    for v in vs {
        enc_vertex_bin(v, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vertices_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<DxfVertex>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(dec_vertex_bin(reader)?);
    }
    Ok(out)
}

/// 📐️ `DxfEntity` binary twin of [`enc_dxf_entity`]/[`dec_dxf_entity`] — tag `0`=Line,`1`=Circle,
/// `2`=Arc,`3`=Polyline,`4`=Text,`5`=Solid,`6`=Insert,`7`=Other (same order `enc_dxf_entity`'s own
/// L/C/A/W/T/S/I/O letters appear in).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_entity_bin(e: &DxfEntity, out: &mut Vec<u8>) {
    match e {
        DxfEntity::Line { start, end, layer, unknown_group_codes } => {
            out.push(0);
            write_point3_bin(out, start);
            write_point3_bin(out, end);
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Circle { center, radius, layer, unknown_group_codes } => {
            out.push(1);
            write_point3_bin(out, center);
            write_f64_bin(out, *radius);
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Arc { center, radius, start_angle, end_angle, layer, unknown_group_codes } => {
            out.push(2);
            write_point3_bin(out, center);
            write_f64_bin(out, *radius);
            write_f64_bin(out, *start_angle);
            write_f64_bin(out, *end_angle);
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Polyline { vertices, closed, layer, unknown_group_codes } => {
            out.push(3);
            enc_vertices_bin(vertices, out);
            out.push(if *closed { 1 } else { 0 });
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Text { position, height, value, layer, unknown_group_codes } => {
            out.push(4);
            write_point3_bin(out, position);
            write_f64_bin(out, *height);
            write_str_lp(out, value);
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Solid { points, layer, unknown_group_codes } => {
            out.push(5);
            write_points4_bin(out, points);
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Insert { block_name, position, scale, rotation, layer, unknown_group_codes } => {
            out.push(6);
            write_str_lp(out, block_name);
            write_point3_bin(out, position);
            write_point3_bin(out, scale);
            write_f64_bin(out, *rotation);
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Other { kind, group_codes } => {
            out.push(7);
            write_str_lp(out, kind);
            enc_group_codes_bin(group_codes, out);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_entity_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfEntity, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let start = read_point3_bin(reader)?;
            let end = read_point3_bin(reader)?;
            let layer = read_str_lp(reader)?;
            let unknown_group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Line { start, end, layer, unknown_group_codes })
        }
        1 => {
            let center = read_point3_bin(reader)?;
            let radius = read_f64_bin(reader)?;
            let layer = read_str_lp(reader)?;
            let unknown_group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Circle { center, radius, layer, unknown_group_codes })
        }
        2 => {
            let center = read_point3_bin(reader)?;
            let radius = read_f64_bin(reader)?;
            let start_angle = read_f64_bin(reader)?;
            let end_angle = read_f64_bin(reader)?;
            let layer = read_str_lp(reader)?;
            let unknown_group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Arc { center, radius, start_angle, end_angle, layer, unknown_group_codes })
        }
        3 => {
            let vertices = dec_vertices_bin(reader)?;
            let closed = reader.read_u8().map_err(|e| e.to_string())? != 0;
            let layer = read_str_lp(reader)?;
            let unknown_group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Polyline { vertices, closed, layer, unknown_group_codes })
        }
        4 => {
            let position = read_point3_bin(reader)?;
            let height = read_f64_bin(reader)?;
            let value = read_str_lp(reader)?;
            let layer = read_str_lp(reader)?;
            let unknown_group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Text { position, height, value, layer, unknown_group_codes })
        }
        5 => {
            let points = read_points4_bin(reader)?;
            let layer = read_str_lp(reader)?;
            let unknown_group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Solid { points, layer, unknown_group_codes })
        }
        6 => {
            let block_name = read_str_lp(reader)?;
            let position = read_point3_bin(reader)?;
            let scale = read_point3_bin(reader)?;
            let rotation = read_f64_bin(reader)?;
            let layer = read_str_lp(reader)?;
            let unknown_group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Insert { block_name, position, scale, rotation, layer, unknown_group_codes })
        }
        7 => {
            let kind = read_str_lp(reader)?;
            let group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Other { kind, group_codes })
        }
        other => Err(format!("dxf entity binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_entities_bin(es: &[DxfEntity], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, es.len() as u64);
    for e in es {
        enc_dxf_entity_bin(e, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_entities_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<DxfEntity>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(dec_dxf_entity_bin(reader)?);
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_header_var_bin(hv: &DxfHeaderVar, out: &mut Vec<u8>) {
    write_str_lp(out, &hv.name);
    store::write_varint_i64(out, hv.group_code as i64);
    enc_dxf_value_bin(&hv.value, out);
    enc_group_codes_bin(&hv.extra_group_codes, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_header_var_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfHeaderVar, String> {
    let name = read_str_lp(reader)?;
    let group_code = reader.read_varint_i64().map_err(|e| e.to_string())? as i32;
    let value = dec_dxf_value_bin(reader)?;
    let extra_group_codes = dec_group_codes_bin(reader)?;
    Ok(DxfHeaderVar { name, group_code, value, extra_group_codes })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_layer_bin(l: &DxfLayer, out: &mut Vec<u8>) {
    write_str_lp(out, &l.name);
    store::write_varint_i64(out, l.color as i64);
    write_str_lp(out, &l.linetype);
    store::write_varint_i64(out, l.flags as i64);
    enc_group_codes_bin(&l.unknown_group_codes, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_layer_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfLayer, String> {
    let name = read_str_lp(reader)?;
    let color = reader.read_varint_i64().map_err(|e| e.to_string())? as i32;
    let linetype = read_str_lp(reader)?;
    let flags = reader.read_varint_i64().map_err(|e| e.to_string())? as i32;
    let unknown_group_codes = dec_group_codes_bin(reader)?;
    Ok(DxfLayer { name, color, linetype, flags, unknown_group_codes })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_style_bin(s: &DxfStyle, out: &mut Vec<u8>) {
    write_str_lp(out, &s.name);
    store::write_varint_i64(out, s.flags as i64);
    write_str_lp(out, &s.font_name);
    enc_group_codes_bin(&s.unknown_group_codes, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_style_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfStyle, String> {
    let name = read_str_lp(reader)?;
    let flags = reader.read_varint_i64().map_err(|e| e.to_string())? as i32;
    let font_name = read_str_lp(reader)?;
    let unknown_group_codes = dec_group_codes_bin(reader)?;
    Ok(DxfStyle { name, flags, font_name, unknown_group_codes })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_linetype_bin(l: &DxfLinetype, out: &mut Vec<u8>) {
    write_str_lp(out, &l.name);
    store::write_varint_i64(out, l.flags as i64);
    write_str_lp(out, &l.description);
    enc_group_codes_bin(&l.unknown_group_codes, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_linetype_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfLinetype, String> {
    let name = read_str_lp(reader)?;
    let flags = reader.read_varint_i64().map_err(|e| e.to_string())? as i32;
    let description = read_str_lp(reader)?;
    let unknown_group_codes = dec_group_codes_bin(reader)?;
    Ok(DxfLinetype { name, flags, description, unknown_group_codes })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_bin(b: &DxfBlock, out: &mut Vec<u8>) {
    write_str_lp(out, &b.name);
    write_point3_bin(out, &b.base_point);
    enc_dxf_entities_bin(&b.entities, out);
    enc_group_codes_bin(&b.unknown_group_codes, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfBlock, String> {
    let name = read_str_lp(reader)?;
    let base_point = read_point3_bin(reader)?;
    let entities = dec_dxf_entities_bin(reader)?;
    let unknown_group_codes = dec_group_codes_bin(reader)?;
    Ok(DxfBlock { name, base_point, entities, unknown_group_codes })
}

/// 🧪️ P2-FG1: real recursive binary twins of every `enc_*_diff`/`dec_*_diff` codec above
/// (`#region 🔖️DiffValueCodecs`) — backs the upgraded `DiffCodec` frame (`#region 🔖️TopLevel`
/// below). Every `Option<T>` field uses [`write_option_bin`]/[`read_option_bin`]'s tri-state
/// presence byte, the binary twin of the text codec's `encode_option`/`decode_option` `[0]`/
/// `[1,<value>]` pair.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_header_var_diff_bin(d: &DxfHeaderVarDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.group_code, |v, out| store::write_varint_i64(out, *v as i64));
    write_option_bin(out, &d.value, enc_dxf_value_bin);
    write_option_bin(out, &d.extra_group_codes, |v, out| enc_group_codes_bin(v, out));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_header_var_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfHeaderVarDiff, String> {
    let group_code = read_option_bin(reader, |r| Ok(r.read_varint_i64().map_err(|e| e.to_string())? as i32))?;
    let value = read_option_bin(reader, dec_dxf_value_bin)?;
    let extra_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
    Ok(DxfHeaderVarDiff { group_code, value, extra_group_codes })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_layer_diff_bin(d: &DxfLayerDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.color, |v, out| store::write_varint_i64(out, *v as i64));
    write_option_bin(out, &d.linetype, |v, out| write_str_lp(out, v));
    write_option_bin(out, &d.flags, |v, out| store::write_varint_i64(out, *v as i64));
    write_option_bin(out, &d.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_layer_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfLayerDiff, String> {
    let color = read_option_bin(reader, |r| Ok(r.read_varint_i64().map_err(|e| e.to_string())? as i32))?;
    let linetype = read_option_bin(reader, read_str_lp)?;
    let flags = read_option_bin(reader, |r| Ok(r.read_varint_i64().map_err(|e| e.to_string())? as i32))?;
    let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
    Ok(DxfLayerDiff { color, linetype, flags, unknown_group_codes })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_style_diff_bin(d: &DxfStyleDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.flags, |v, out| store::write_varint_i64(out, *v as i64));
    write_option_bin(out, &d.font_name, |v, out| write_str_lp(out, v));
    write_option_bin(out, &d.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_style_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfStyleDiff, String> {
    let flags = read_option_bin(reader, |r| Ok(r.read_varint_i64().map_err(|e| e.to_string())? as i32))?;
    let font_name = read_option_bin(reader, read_str_lp)?;
    let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
    Ok(DxfStyleDiff { flags, font_name, unknown_group_codes })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_linetype_diff_bin(d: &DxfLinetypeDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.flags, |v, out| store::write_varint_i64(out, *v as i64));
    write_option_bin(out, &d.description, |v, out| write_str_lp(out, v));
    write_option_bin(out, &d.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_linetype_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfLinetypeDiff, String> {
    let flags = read_option_bin(reader, |r| Ok(r.read_varint_i64().map_err(|e| e.to_string())? as i32))?;
    let description = read_option_bin(reader, read_str_lp)?;
    let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
    Ok(DxfLinetypeDiff { flags, description, unknown_group_codes })
}

/// 🌳 `DxfEntityDiff` binary twin of [`enc_entity_diff`]/[`dec_entity_diff`] — tag `0`-`7` matches
/// [`enc_dxf_entity_bin`]'s own per-kind numbering (kind-specific field diff), `8`=`Replace`
/// (carries a whole binary-encoded `DxfEntity`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entity_diff_bin(d: &DxfEntityDiff, out: &mut Vec<u8>) {
    match d {
        DxfEntityDiff::Replace { entity } => {
            out.push(8);
            enc_dxf_entity_bin(entity, out);
        }
        DxfEntityDiff::Line(x) => {
            out.push(0);
            write_option_bin(out, &x.start, |v, out| write_point3_bin(out, v));
            write_option_bin(out, &x.end, |v, out| write_point3_bin(out, v));
            write_option_bin(out, &x.layer, |v, out| write_str_lp(out, v));
            write_option_bin(out, &x.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
        }
        DxfEntityDiff::Circle(x) => {
            out.push(1);
            write_option_bin(out, &x.center, |v, out| write_point3_bin(out, v));
            write_option_bin(out, &x.radius, |v, out| write_f64_bin(out, *v));
            write_option_bin(out, &x.layer, |v, out| write_str_lp(out, v));
            write_option_bin(out, &x.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
        }
        DxfEntityDiff::Arc(x) => {
            out.push(2);
            write_option_bin(out, &x.center, |v, out| write_point3_bin(out, v));
            write_option_bin(out, &x.radius, |v, out| write_f64_bin(out, *v));
            write_option_bin(out, &x.start_angle, |v, out| write_f64_bin(out, *v));
            write_option_bin(out, &x.end_angle, |v, out| write_f64_bin(out, *v));
            write_option_bin(out, &x.layer, |v, out| write_str_lp(out, v));
            write_option_bin(out, &x.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
        }
        DxfEntityDiff::Polyline(x) => {
            out.push(3);
            write_option_bin(out, &x.vertices, |v, out| enc_vertices_bin(v, out));
            write_option_bin(out, &x.closed, |v, out| out.push(if *v { 1 } else { 0 }));
            write_option_bin(out, &x.layer, |v, out| write_str_lp(out, v));
            write_option_bin(out, &x.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
        }
        DxfEntityDiff::Text(x) => {
            out.push(4);
            write_option_bin(out, &x.position, |v, out| write_point3_bin(out, v));
            write_option_bin(out, &x.height, |v, out| write_f64_bin(out, *v));
            write_option_bin(out, &x.value, |v, out| write_str_lp(out, v));
            write_option_bin(out, &x.layer, |v, out| write_str_lp(out, v));
            write_option_bin(out, &x.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
        }
        DxfEntityDiff::Solid(x) => {
            out.push(5);
            write_option_bin(out, &x.points, |v, out| write_points4_bin(out, v));
            write_option_bin(out, &x.layer, |v, out| write_str_lp(out, v));
            write_option_bin(out, &x.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
        }
        DxfEntityDiff::Insert(x) => {
            out.push(6);
            write_option_bin(out, &x.block_name, |v, out| write_str_lp(out, v));
            write_option_bin(out, &x.position, |v, out| write_point3_bin(out, v));
            write_option_bin(out, &x.scale, |v, out| write_point3_bin(out, v));
            write_option_bin(out, &x.rotation, |v, out| write_f64_bin(out, *v));
            write_option_bin(out, &x.layer, |v, out| write_str_lp(out, v));
            write_option_bin(out, &x.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
        }
        DxfEntityDiff::Other(x) => {
            out.push(7);
            write_option_bin(out, &x.group_codes, |v, out| enc_group_codes_bin(v, out));
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entity_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfEntityDiff, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        8 => Ok(DxfEntityDiff::Replace { entity: dec_dxf_entity_bin(reader)? }),
        0 => {
            let start = read_option_bin(reader, read_point3_bin)?;
            let end = read_option_bin(reader, read_point3_bin)?;
            let layer = read_option_bin(reader, read_str_lp)?;
            let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
            Ok(DxfEntityDiff::Line(DxfLineDiff { start, end, layer, unknown_group_codes }))
        }
        1 => {
            let center = read_option_bin(reader, read_point3_bin)?;
            let radius = read_option_bin(reader, read_f64_bin)?;
            let layer = read_option_bin(reader, read_str_lp)?;
            let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
            Ok(DxfEntityDiff::Circle(DxfCircleDiff { center, radius, layer, unknown_group_codes }))
        }
        2 => {
            let center = read_option_bin(reader, read_point3_bin)?;
            let radius = read_option_bin(reader, read_f64_bin)?;
            let start_angle = read_option_bin(reader, read_f64_bin)?;
            let end_angle = read_option_bin(reader, read_f64_bin)?;
            let layer = read_option_bin(reader, read_str_lp)?;
            let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
            Ok(DxfEntityDiff::Arc(DxfArcDiff { center, radius, start_angle, end_angle, layer, unknown_group_codes }))
        }
        3 => {
            let vertices = read_option_bin(reader, dec_vertices_bin)?;
            let closed = read_option_bin(reader, |r| Ok(r.read_u8().map_err(|e| e.to_string())? != 0))?;
            let layer = read_option_bin(reader, read_str_lp)?;
            let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
            Ok(DxfEntityDiff::Polyline(DxfPolylineDiff { vertices, closed, layer, unknown_group_codes }))
        }
        4 => {
            let position = read_option_bin(reader, read_point3_bin)?;
            let height = read_option_bin(reader, read_f64_bin)?;
            let value = read_option_bin(reader, read_str_lp)?;
            let layer = read_option_bin(reader, read_str_lp)?;
            let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
            Ok(DxfEntityDiff::Text(DxfTextDiff { position, height, value, layer, unknown_group_codes }))
        }
        5 => {
            let points = read_option_bin(reader, read_points4_bin)?;
            let layer = read_option_bin(reader, read_str_lp)?;
            let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
            Ok(DxfEntityDiff::Solid(DxfSolidDiff { points, layer, unknown_group_codes }))
        }
        6 => {
            let block_name = read_option_bin(reader, read_str_lp)?;
            let position = read_option_bin(reader, read_point3_bin)?;
            let scale = read_option_bin(reader, read_point3_bin)?;
            let rotation = read_option_bin(reader, read_f64_bin)?;
            let layer = read_option_bin(reader, read_str_lp)?;
            let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
            Ok(DxfEntityDiff::Insert(DxfInsertDiff { block_name, position, scale, rotation, layer, unknown_group_codes }))
        }
        7 => {
            let group_codes = read_option_bin(reader, dec_group_codes_bin)?;
            Ok(DxfEntityDiff::Other(DxfOtherDiff { group_codes }))
        }
        other => Err(format!("entity diff binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_diff_bin(d: &DxfBlockDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.name, |v, out| write_str_lp(out, v));
    write_option_bin(out, &d.base_point, |v, out| write_point3_bin(out, v));
    write_option_bin(out, &d.entities, enc_entities_diff_bin);
    write_option_bin(out, &d.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfBlockDiff, String> {
    let name = read_option_bin(reader, read_str_lp)?;
    let base_point = read_option_bin(reader, read_point3_bin)?;
    let entities = read_option_bin(reader, dec_entities_diff_bin)?;
    let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
    Ok(DxfBlockDiff { name, base_point, entities, unknown_group_codes })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_tables_diff_bin(t: &DxfTablesDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &t.layers, enc_layers_diff_bin);
    write_option_bin(out, &t.styles, enc_styles_diff_bin);
    write_option_bin(out, &t.linetypes, enc_linetypes_diff_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_tables_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfTablesDiff, String> {
    let layers = read_option_bin(reader, dec_layers_diff_bin)?;
    let styles = read_option_bin(reader, dec_styles_diff_bin)?;
    let linetypes = read_option_bin(reader, dec_linetypes_diff_bin)?;
    Ok(DxfTablesDiff { layers, styles, linetypes })
}

/// 🧮 Binary twins of [`enc_name_triple`]/[`dec_name_triple`] and [`enc_index_triple`]/
/// [`dec_index_triple`] above — varint-counted lists instead of `;`-separated bracket sections,
/// same removed/modified/added shape.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_name_triple_bin<T, D>(removed: &[String], modified: &[(String, D)], added: &[(usize, T)], out: &mut Vec<u8>, enc_diff: impl Fn(&D, &mut Vec<u8>), enc_item: impl Fn(&T, &mut Vec<u8>)) {
    store::pack_rt::write_varint_u64(out, removed.len() as u64);
    for name in removed {
        write_str_lp(out, name);
    }
    store::pack_rt::write_varint_u64(out, modified.len() as u64);
    for (name, d) in modified {
        write_str_lp(out, name);
        enc_diff(d, out);
    }
    store::pack_rt::write_varint_u64(out, added.len() as u64);
    for (idx, item) in added {
        store::pack_rt::write_varint_u64(out, *idx as u64);
        enc_item(item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_name_triple_bin<T, D>(reader: &mut store::ByteReader<'_>, dec_diff: impl Fn(&mut store::ByteReader<'_>) -> Result<D, String>, dec_item: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<NamedDiffParts<D, T>, String> {
    let rc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(rc as usize);
    for _ in 0..rc {
        removed.push(read_str_lp(reader)?);
    }
    let mc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mc as usize);
    for _ in 0..mc {
        let name = read_str_lp(reader)?;
        let d = dec_diff(reader)?;
        modified.push((name, d));
    }
    let ac = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(ac as usize);
    for _ in 0..ac {
        let idx = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_item(reader)?;
        added.push((idx, item));
    }
    Ok((removed, modified, added))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_index_triple_bin<T, D>(removed: &[usize], modified: &[(usize, D)], added: &[(usize, T)], out: &mut Vec<u8>, enc_diff: impl Fn(&D, &mut Vec<u8>), enc_item: impl Fn(&T, &mut Vec<u8>)) {
    store::pack_rt::write_varint_u64(out, removed.len() as u64);
    for idx in removed {
        store::pack_rt::write_varint_u64(out, *idx as u64);
    }
    store::pack_rt::write_varint_u64(out, modified.len() as u64);
    for (idx, d) in modified {
        store::pack_rt::write_varint_u64(out, *idx as u64);
        enc_diff(d, out);
    }
    store::pack_rt::write_varint_u64(out, added.len() as u64);
    for (idx, item) in added {
        store::pack_rt::write_varint_u64(out, *idx as u64);
        enc_item(item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_index_triple_bin<T, D>(reader: &mut store::ByteReader<'_>, dec_diff: impl Fn(&mut store::ByteReader<'_>) -> Result<D, String>, dec_item: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<IndexedDiffParts<D, T>, String> {
    let rc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(rc as usize);
    for _ in 0..rc {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let mc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mc as usize);
    for _ in 0..mc {
        let idx = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let d = dec_diff(reader)?;
        modified.push((idx, d));
    }
    let ac = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(ac as usize);
    for _ in 0..ac {
        let idx = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_item(reader)?;
        added.push((idx, item));
    }
    Ok((removed, modified, added))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_header_vars_diff_bin(d: &DxfHeaderVarsDiff, out: &mut Vec<u8>) {
    let modified: Vec<(String, DxfHeaderVarDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfHeaderVar)> = d.added.iter().map(|a| (a.index, a.header_var.clone())).collect();
    enc_name_triple_bin(&d.removed, &modified, &added, out, enc_header_var_diff_bin, enc_header_var_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_header_vars_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfHeaderVarsDiff, String> {
    let (removed, modified, added) = dec_name_triple_bin(reader, dec_header_var_diff_bin, dec_header_var_bin)?;
    Ok(DxfHeaderVarsDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfHeaderVarModified { name, diff }).collect(), added: added.into_iter().map(|(index, header_var)| DxfHeaderVarAdded { index, header_var }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_layers_diff_bin(d: &DxfLayersDiff, out: &mut Vec<u8>) {
    let modified: Vec<(String, DxfLayerDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfLayer)> = d.added.iter().map(|a| (a.index, a.layer.clone())).collect();
    enc_name_triple_bin(&d.removed, &modified, &added, out, enc_layer_diff_bin, enc_layer_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_layers_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfLayersDiff, String> {
    let (removed, modified, added) = dec_name_triple_bin(reader, dec_layer_diff_bin, dec_layer_bin)?;
    Ok(DxfLayersDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfLayerModified { name, diff }).collect(), added: added.into_iter().map(|(index, layer)| DxfLayerAdded { index, layer }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_styles_diff_bin(d: &DxfStylesDiff, out: &mut Vec<u8>) {
    let modified: Vec<(String, DxfStyleDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfStyle)> = d.added.iter().map(|a| (a.index, a.style.clone())).collect();
    enc_name_triple_bin(&d.removed, &modified, &added, out, enc_style_diff_bin, enc_style_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_styles_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfStylesDiff, String> {
    let (removed, modified, added) = dec_name_triple_bin(reader, dec_style_diff_bin, dec_style_bin)?;
    Ok(DxfStylesDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfStyleModified { name, diff }).collect(), added: added.into_iter().map(|(index, style)| DxfStyleAdded { index, style }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_linetypes_diff_bin(d: &DxfLinetypesDiff, out: &mut Vec<u8>) {
    let modified: Vec<(String, DxfLinetypeDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfLinetype)> = d.added.iter().map(|a| (a.index, a.linetype.clone())).collect();
    enc_name_triple_bin(&d.removed, &modified, &added, out, enc_linetype_diff_bin, enc_linetype_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_linetypes_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfLinetypesDiff, String> {
    let (removed, modified, added) = dec_name_triple_bin(reader, dec_linetype_diff_bin, dec_linetype_bin)?;
    Ok(DxfLinetypesDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfLinetypeModified { name, diff }).collect(), added: added.into_iter().map(|(index, linetype)| DxfLinetypeAdded { index, linetype }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entities_diff_bin(d: &DxfEntitiesDiff, out: &mut Vec<u8>) {
    let modified: Vec<(usize, DxfEntityDiff)> = d.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let added: Vec<(usize, DxfEntity)> = d.added.iter().map(|a| (a.index, a.entity.clone())).collect();
    enc_index_triple_bin(&d.removed, &modified, &added, out, enc_entity_diff_bin, enc_dxf_entity_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entities_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfEntitiesDiff, String> {
    let (removed, modified, added) = dec_index_triple_bin(reader, dec_entity_diff_bin, dec_dxf_entity_bin)?;
    Ok(DxfEntitiesDiff { removed, modified: modified.into_iter().map(|(index, diff)| DxfEntityModified { index, diff }).collect(), added: added.into_iter().map(|(index, entity)| DxfEntityAdded { index, entity }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_blocks_diff_bin(d: &DxfBlocksDiff, out: &mut Vec<u8>) {
    let modified: Vec<(usize, DxfBlockDiff)> = d.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let added: Vec<(usize, DxfBlock)> = d.added.iter().map(|a| (a.index, a.block.clone())).collect();
    enc_index_triple_bin(&d.removed, &modified, &added, out, enc_block_diff_bin, enc_block_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_blocks_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfBlocksDiff, String> {
    let (removed, modified, added) = dec_index_triple_bin(reader, dec_block_diff_bin, dec_block_bin)?;
    Ok(DxfBlocksDiff { removed, modified: modified.into_iter().map(|(index, diff)| DxfBlockModified { index, diff }).collect(), added: added.into_iter().map(|(index, block)| DxfBlockAdded { index, block }).collect() })
}

impl protocol::DiffBinary for DxfDiff {
/// 🧪️ P2-FG1: REAL binary frame (`format u8 | flags u8 | per-present-field payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (100% of stdio's
/// `DiffCodec` impls were still on that shortcut per the P2-W0 census). `flags` is a 4-bit
/// presence mask (bit0=`header_vars`,bit1=`tables`,bit2=`blocks`,bit3=`entities`) since
/// `DxfDiff` has FOUR independently optional top-level fields, unlike `stdio.json`'s single
/// `value` (one `has_value` byte there); each PRESENT field's own real recursive
/// collection-triple/tri-state binary payload follows, genuinely structured
/// (`#region 🔖️DiffBinaryCodecs`/`#region 🔖️CollectionTripleBinaryCodecs` above), never
/// text-as-bytes.
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut flags: u8 = 0;
    if self.header_vars.is_some() {
        flags |= 0b0001;
    }
    if self.tables.is_some() {
        flags |= 0b0010;
    }
    if self.blocks.is_some() {
        flags |= 0b0100;
    }
    if self.entities.is_some() {
        flags |= 0b1000;
    }
    let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
    if let Some(v) = &self.header_vars {
        enc_header_vars_diff_bin(v, &mut out);
    }
    if let Some(v) = &self.tables {
        enc_tables_diff_bin(v, &mut out);
    }
    if let Some(v) = &self.blocks {
        enc_blocks_diff_bin(v, &mut out);
    }
    if let Some(v) = &self.entities {
        enc_entities_diff_bin(v, &mut out);
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
    let flags = reader.read_u8().map_err(|e| malformed("diff flags", 1, e.to_string()))?;
    let header_vars = if flags & 0b0001 != 0 { Some(dec_header_vars_diff_bin(&mut reader).map_err(|e| malformed("diff header_vars", reader.position(), e))?) } else { None };
    let tables = if flags & 0b0010 != 0 { Some(dec_tables_diff_bin(&mut reader).map_err(|e| malformed("diff tables", reader.position(), e))?) } else { None };
    let blocks = if flags & 0b0100 != 0 { Some(dec_blocks_diff_bin(&mut reader).map_err(|e| malformed("diff blocks", reader.position(), e))?) } else { None };
    let entities = if flags & 0b1000 != 0 { Some(dec_entities_diff_bin(&mut reader).map_err(|e| malformed("diff entities", reader.position(), e))?) } else { None };
    Ok(DxfDiff { header_vars, tables, blocks, entities })
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
pub(crate) fn enc_dxf_tag_bin(t: &DxfTag, out: &mut Vec<u8>) {
    store::write_varint_i64(out, t.code as i64);
    write_str_lp(out, &t.value);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_tag_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfTag, String> {
    let code = reader.read_varint_i64().map_err(|e| e.to_string())? as i32;
    let value = read_str_lp(reader)?;
    Ok(DxfTag { code, value })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_other_table_bin(t: &DxfOtherTable, out: &mut Vec<u8>) {
    write_str_lp(out, &t.name);
    store::pack_rt::write_varint_u64(out, t.tags.len() as u64);
    for tag in &t.tags {
        enc_dxf_tag_bin(tag, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_other_table_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfOtherTable, String> {
    let name = read_str_lp(reader)?;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut tags = Vec::with_capacity(count as usize);
    for _ in 0..count {
        tags.push(dec_dxf_tag_bin(reader)?);
    }
    Ok(DxfOtherTable { name, tags })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_tables_bin(t: &DxfTables, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, t.layers.len() as u64);
    for l in &t.layers {
        enc_layer_bin(l, out);
    }
    store::pack_rt::write_varint_u64(out, t.styles.len() as u64);
    for s in &t.styles {
        enc_style_bin(s, out);
    }
    store::pack_rt::write_varint_u64(out, t.linetypes.len() as u64);
    for l in &t.linetypes {
        enc_linetype_bin(l, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_tables_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfTables, String> {
    let lc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut layers = Vec::with_capacity(lc as usize);
    for _ in 0..lc {
        layers.push(dec_layer_bin(reader)?);
    }
    let sc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut styles = Vec::with_capacity(sc as usize);
    for _ in 0..sc {
        styles.push(dec_style_bin(reader)?);
    }
    let ltc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut linetypes = Vec::with_capacity(ltc as usize);
    for _ in 0..ltc {
        linetypes.push(dec_linetype_bin(reader)?);
    }
    Ok(DxfTables { layers, styles, linetypes })
}
}
pub use diff_wire_codec::*;
