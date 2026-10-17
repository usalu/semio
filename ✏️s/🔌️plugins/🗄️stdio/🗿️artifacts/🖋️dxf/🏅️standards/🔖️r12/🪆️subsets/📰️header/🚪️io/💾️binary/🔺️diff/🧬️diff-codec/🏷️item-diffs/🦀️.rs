//! 🏷️ Item diffs of the DXF diff codec: the header variable, layer, style, linetype, entity, block and tables diff pair codecs. See [`super`].

use super::*;

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
