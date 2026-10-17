//! 📐️ Group codes and entities of the DXF diff codec: the group-code and vertex pair codecs and the entity and entity collection pair codecs. See [`super`].

use super::*;

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
        DxfEntity::Line(DxfLine { start, end, layer, unknown_group_codes }) => {
            out.push(0);
            write_point3_bin(out, start);
            write_point3_bin(out, end);
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Circle(DxfCircle { center, radius, layer, unknown_group_codes }) => {
            out.push(1);
            write_point3_bin(out, center);
            write_f64_bin(out, *radius);
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Arc(DxfArc { center, radius, start_angle, end_angle, layer, unknown_group_codes }) => {
            out.push(2);
            write_point3_bin(out, center);
            write_f64_bin(out, *radius);
            write_f64_bin(out, *start_angle);
            write_f64_bin(out, *end_angle);
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Polyline(DxfPolyline { vertices, closed, layer, unknown_group_codes }) => {
            out.push(3);
            enc_vertices_bin(vertices, out);
            out.push(if *closed { 1 } else { 0 });
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Text(DxfText { position, height, value, layer, unknown_group_codes }) => {
            out.push(4);
            write_point3_bin(out, position);
            write_f64_bin(out, *height);
            write_str_lp(out, value);
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Solid(DxfSolid { points, layer, unknown_group_codes }) => {
            out.push(5);
            write_points4_bin(out, points);
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Insert(DxfInsert { block_name, position, scale, rotation, layer, unknown_group_codes }) => {
            out.push(6);
            write_str_lp(out, block_name);
            write_point3_bin(out, position);
            write_point3_bin(out, scale);
            write_f64_bin(out, *rotation);
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Other(DxfOther { kind, group_codes }) => {
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
            Ok(DxfEntity::Line(DxfLine { start, end, layer, unknown_group_codes }))
        }
        1 => {
            let center = read_point3_bin(reader)?;
            let radius = read_f64_bin(reader)?;
            let layer = read_str_lp(reader)?;
            let unknown_group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Circle(DxfCircle { center, radius, layer, unknown_group_codes }))
        }
        2 => {
            let center = read_point3_bin(reader)?;
            let radius = read_f64_bin(reader)?;
            let start_angle = read_f64_bin(reader)?;
            let end_angle = read_f64_bin(reader)?;
            let layer = read_str_lp(reader)?;
            let unknown_group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Arc(DxfArc { center, radius, start_angle, end_angle, layer, unknown_group_codes }))
        }
        3 => {
            let vertices = dec_vertices_bin(reader)?;
            let closed = reader.read_u8().map_err(|e| e.to_string())? != 0;
            let layer = read_str_lp(reader)?;
            let unknown_group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Polyline(DxfPolyline { vertices, closed, layer, unknown_group_codes }))
        }
        4 => {
            let position = read_point3_bin(reader)?;
            let height = read_f64_bin(reader)?;
            let value = read_str_lp(reader)?;
            let layer = read_str_lp(reader)?;
            let unknown_group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Text(DxfText { position, height, value, layer, unknown_group_codes }))
        }
        5 => {
            let points = read_points4_bin(reader)?;
            let layer = read_str_lp(reader)?;
            let unknown_group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Solid(DxfSolid { points, layer, unknown_group_codes }))
        }
        6 => {
            let block_name = read_str_lp(reader)?;
            let position = read_point3_bin(reader)?;
            let scale = read_point3_bin(reader)?;
            let rotation = read_f64_bin(reader)?;
            let layer = read_str_lp(reader)?;
            let unknown_group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Insert(DxfInsert { block_name, position, scale, rotation, layer, unknown_group_codes }))
        }
        7 => {
            let kind = read_str_lp(reader)?;
            let group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Other(DxfOther { kind, group_codes }))
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
