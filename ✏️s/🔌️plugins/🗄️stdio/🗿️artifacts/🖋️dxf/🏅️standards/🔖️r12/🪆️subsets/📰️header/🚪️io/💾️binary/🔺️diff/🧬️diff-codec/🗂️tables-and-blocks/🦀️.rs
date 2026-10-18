//! 🗂️ Table items and blocks of the DXF diff codec: the header variable, layer, style, linetype and block pair codecs. See [`super`].

use super::*;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_header_var_bin(hv: &DxfHeaderVar, out: &mut Vec<u8>) {
    write_str_lp(out, &hv.name);
    store::write_varint_i64(out, hv.group_code as i64);
    enc_dxf_value_bin(&hv.value, out);
    enc_group_codes_bin(&hv.extra_group_codes, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_header_var_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfHeaderVar, String> {
    let name = read_str_lp(reader)?;
    let group_code = reader.read_varint_i64().map_err(|e| e.to_string())? as i32;
    let value = dec_dxf_value_bin(reader)?;
    let extra_group_codes = dec_group_codes_bin(reader)?;
    Ok(DxfHeaderVar { name, group_code, value, extra_group_codes })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_layer_bin(l: &DxfLayer, out: &mut Vec<u8>) {
    write_str_lp(out, &l.name);
    store::write_varint_i64(out, l.color as i64);
    write_str_lp(out, &l.linetype);
    store::write_varint_i64(out, l.flags as i64);
    enc_group_codes_bin(&l.unknown_group_codes, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_layer_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfLayer, String> {
    let name = read_str_lp(reader)?;
    let color = reader.read_varint_i64().map_err(|e| e.to_string())? as i32;
    let linetype = read_str_lp(reader)?;
    let flags = reader.read_varint_i64().map_err(|e| e.to_string())? as i32;
    let unknown_group_codes = dec_group_codes_bin(reader)?;
    Ok(DxfLayer { name, color, linetype, flags, unknown_group_codes })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_style_bin(s: &DxfStyle, out: &mut Vec<u8>) {
    write_str_lp(out, &s.name);
    store::write_varint_i64(out, s.flags as i64);
    write_str_lp(out, &s.font_name);
    enc_group_codes_bin(&s.unknown_group_codes, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_style_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfStyle, String> {
    let name = read_str_lp(reader)?;
    let flags = reader.read_varint_i64().map_err(|e| e.to_string())? as i32;
    let font_name = read_str_lp(reader)?;
    let unknown_group_codes = dec_group_codes_bin(reader)?;
    Ok(DxfStyle { name, flags, font_name, unknown_group_codes })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_linetype_bin(l: &DxfLinetype, out: &mut Vec<u8>) {
    write_str_lp(out, &l.name);
    store::write_varint_i64(out, l.flags as i64);
    write_str_lp(out, &l.description);
    enc_group_codes_bin(&l.unknown_group_codes, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_linetype_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfLinetype, String> {
    let name = read_str_lp(reader)?;
    let flags = reader.read_varint_i64().map_err(|e| e.to_string())? as i32;
    let description = read_str_lp(reader)?;
    let unknown_group_codes = dec_group_codes_bin(reader)?;
    Ok(DxfLinetype { name, flags, description, unknown_group_codes })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_block_bin(b: &DxfBlock, out: &mut Vec<u8>) {
    write_str_lp(out, &b.name);
    write_point3_bin(out, &b.base_point);
    enc_dxf_entities_bin(&b.entities, out);
    enc_group_codes_bin(&b.unknown_group_codes, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_block_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfBlock, String> {
    let name = read_str_lp(reader)?;
    let base_point = read_point3_bin(reader)?;
    let entities = dec_dxf_entities_bin(reader)?;
    let unknown_group_codes = dec_group_codes_bin(reader)?;
    Ok(DxfBlock { name, base_point, entities, unknown_group_codes })
}
