//! binary rep for stdio.dxf 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
#[path = "🧬️diff-codec/🦀️.rs"]
mod diff_codec;
pub use diff_codec::*;

#[allow(unused_imports)]
mod diff_wire_codec {
use super::*;
use crate::standards::v_r12::subsets::any::schema::diff::*;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use crate::schema::snapshot::{DxfBlock, DxfEntity, DxfArc, DxfCircle, DxfInsert, DxfLine, DxfOther, DxfPolyline, DxfSolid, DxfText, DxfHeaderVar, DxfLayer, DxfLinetype, DxfOtherTable, DxfStyle, DxfTables, DxfTag, DxfValue, DxfVertex};
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
pub fn enc_other_table_bin(t: &DxfOtherTable, out: &mut Vec<u8>) {
    write_str_lp(out, &t.name);
    store::pack_rt::write_varint_u64(out, t.tags.len() as u64);
    for tag in &t.tags {
        enc_dxf_tag_bin(tag, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_other_table_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfOtherTable, String> {
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
