//! jpg rep for stdio.jpg 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_jfif_1_01::subsets::document::schema::diff::*;
use crate::schema::snapshot::{JfifDensityUnits, JfifThumbnail, JpgFrameComponent, JpgFrameHeader, JpgHuffmanClass, JpgHuffmanTable, JpgQuantTable, JpgSegment};
use crate::JpgSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, HashMap};

/// 🧪️ P2-FG2: real LEB128-varint-framed binary primitives backing the upgraded `DiffCodec`
/// (below) and, via `pub(crate)`, `../🧬️mutations/🦀️.rs`'s upgraded `OpBinary` — mirrors
/// `📰️xml`'s own `write_bytes_lp`/`read_bytes_lp` shape (`📖️grammar-recipe.md` §2.5), reusing
/// `store::pack_rt::write_varint_u64`/`store::ByteReader` rather than reinventing varint codecs.
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

/// 🏳️ Generic `Option<T>` presence-byte codec (`0`/`1` + payload) — the binary twin of the text
/// side's `encode_option`/`decode_option`, used for every tri-state/plain-optional field below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_opt<T>(out: &mut Vec<u8>, opt: &Option<T>, enc: impl FnOnce(&T, &mut Vec<u8>)) {
    out.push(if opt.is_some() { 1 } else { 0 });
    if let Some(v) = opt {
        enc(v, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_opt<T>(reader: &mut store::ByteReader<'_>, dec: impl FnOnce(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Option<T>, String> {
    let has = reader.read_u8().map_err(|e| e.to_string())?;
    if has != 0 {
        Ok(Some(dec(reader)?))
    } else {
        Ok(None)
    }
}

/// 🧪️ P2-FG2: real recursive-free binary twins of `§ValueCodecs` above — every type here is a
/// bounded, non-recursive record/collection (unlike xml's self-recursive `XmlNode`), so every
/// field is genuinely, individually written/read; no opaque payload anywhere in this region.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_version_bin(v: &(u8, u8), out: &mut Vec<u8>) {
    out.push(v.0);
    out.push(v.1);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_version_bin(reader: &mut store::ByteReader<'_>) -> Result<(u8, u8), String> {
    Ok((reader.read_u8().map_err(|e| e.to_string())?, reader.read_u8().map_err(|e| e.to_string())?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_density_units_bin(u: &JfifDensityUnits, out: &mut Vec<u8>) {
    out.push(u.to_u8());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_density_units_bin(reader: &mut store::ByteReader<'_>) -> Result<JfifDensityUnits, String> {
    JfifDensityUnits::from_u8(reader.read_u8().map_err(|e| e.to_string())?)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_huffman_class_bin(c: &JpgHuffmanClass, out: &mut Vec<u8>) {
    out.push(c.to_u8());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_huffman_class_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgHuffmanClass, String> {
    JpgHuffmanClass::from_u8(reader.read_u8().map_err(|e| e.to_string())?)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_thumbnail_bin(t: &JfifThumbnail, out: &mut Vec<u8>) {
    out.push(t.width);
    out.push(t.height);
    write_bytes_lp(out, &t.rgb_data);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_thumbnail_bin(reader: &mut store::ByteReader<'_>) -> Result<JfifThumbnail, String> {
    let width = reader.read_u8().map_err(|e| e.to_string())?;
    let height = reader.read_u8().map_err(|e| e.to_string())?;
    let rgb_data = read_bytes_lp(reader)?;
    Ok(JfifThumbnail { width, height, rgb_data })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_frame_component_bin(c: &JpgFrameComponent, out: &mut Vec<u8>) {
    out.push(c.id);
    out.push(c.h_sampling);
    out.push(c.v_sampling);
    out.push(c.quant_table_id);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_frame_component_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgFrameComponent, String> {
    Ok(JpgFrameComponent {
        id: reader.read_u8().map_err(|e| e.to_string())?,
        h_sampling: reader.read_u8().map_err(|e| e.to_string())?,
        v_sampling: reader.read_u8().map_err(|e| e.to_string())?,
        quant_table_id: reader.read_u8().map_err(|e| e.to_string())?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_frame_header_bin(f: &JpgFrameHeader, out: &mut Vec<u8>) {
    out.push(f.precision);
    store::pack_rt::write_varint_u64(out, f.width as u64);
    store::pack_rt::write_varint_u64(out, f.height as u64);
    store::pack_rt::write_varint_u64(out, f.components.len() as u64);
    for c in &f.components {
        enc_frame_component_bin(c, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_frame_header_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgFrameHeader, String> {
    let precision = reader.read_u8().map_err(|e| e.to_string())?;
    let width = reader.read_varint_u64().map_err(|e| e.to_string())? as u16;
    let height = reader.read_varint_u64().map_err(|e| e.to_string())? as u16;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut components = Vec::with_capacity(count as usize);
    for _ in 0..count {
        components.push(dec_frame_component_bin(reader)?);
    }
    Ok(JpgFrameHeader { precision, width, height, components })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_quant_table_bin(t: &JpgQuantTable, out: &mut Vec<u8>) {
    out.push(t.id);
    out.push(t.precision);
    for v in t.values.iter() {
        out.extend_from_slice(&v.to_le_bytes());
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_quant_table_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgQuantTable, String> {
    let id = reader.read_u8().map_err(|e| e.to_string())?;
    let precision = reader.read_u8().map_err(|e| e.to_string())?;
    let mut values = [0u16; 64];
    for v in values.iter_mut() {
        *v = reader.read_u16_le().map_err(|e| e.to_string())?;
    }
    Ok(JpgQuantTable { id, precision, values })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_huffman_table_bin(t: &JpgHuffmanTable, out: &mut Vec<u8>) {
    out.push(t.id);
    enc_huffman_class_bin(&t.class, out);
    out.extend_from_slice(&t.bits);
    write_bytes_lp(out, &t.values);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_huffman_table_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgHuffmanTable, String> {
    let id = reader.read_u8().map_err(|e| e.to_string())?;
    let class = dec_huffman_class_bin(reader)?;
    let bits_vec = reader.read_bytes(16).map_err(|e| e.to_string())?.to_vec();
    let bits: [u8; 16] = bits_vec.try_into().map_err(|_| "huffman bits: expected 16 bytes".to_string())?;
    let values = read_bytes_lp(reader)?;
    Ok(JpgHuffmanTable { id, class, bits, values })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_huffman_key_bin(k: &JpgHuffmanTableKey, out: &mut Vec<u8>) {
    enc_huffman_class_bin(&k.class, out);
    out.push(k.id);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_huffman_key_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgHuffmanTableKey, String> {
    let class = dec_huffman_class_bin(reader)?;
    let id = reader.read_u8().map_err(|e| e.to_string())?;
    Ok(JpgHuffmanTableKey { class, id })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_segment_bin(s: &JpgSegment, out: &mut Vec<u8>) {
    out.push(s.marker);
    write_bytes_lp(out, &s.data);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_segment_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgSegment, String> {
    let marker = reader.read_u8().map_err(|e| e.to_string())?;
    let data = read_bytes_lp(reader)?;
    Ok(JpgSegment { marker, data })
}

/// 🧪️ P2-FG2: real binary twins of `§DiffValueCodecs` above — every collection triple below is
/// `varint-count + real-item` (removed/modified/added, matching the recipe's §1.4 shape but
/// binary-framed rather than bracket-text-framed); no opaque payload anywhere in this region since
/// none of jpg's diff types are self-recursive (unlike xml's `XmlNodeDiff`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_component_diff_bin(d: &JpgComponentDiff, out: &mut Vec<u8>) {
    write_opt(out, &d.h_sampling, |v, out| out.push(*v));
    write_opt(out, &d.v_sampling, |v, out| out.push(*v));
    write_opt(out, &d.quant_table_id, |v, out| out.push(*v));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_component_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgComponentDiff, String> {
    Ok(JpgComponentDiff {
        h_sampling: read_opt(reader, |r| r.read_u8().map_err(|e| e.to_string()))?,
        v_sampling: read_opt(reader, |r| r.read_u8().map_err(|e| e.to_string()))?,
        quant_table_id: read_opt(reader, |r| r.read_u8().map_err(|e| e.to_string()))?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_components_diff_bin(d: &JpgComponentsDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for id in &d.removed {
        out.push(*id);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        out.push(m.id);
        enc_component_diff_bin(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_frame_component_bin(&a.item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_components_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgComponentsDiff, String> {
    let rc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(rc as usize);
    for _ in 0..rc {
        removed.push(reader.read_u8().map_err(|e| e.to_string())?);
    }
    let mc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mc as usize);
    for _ in 0..mc {
        let id = reader.read_u8().map_err(|e| e.to_string())?;
        modified.push(JpgComponentModified { id, diff: dec_component_diff_bin(reader)? });
    }
    let ac = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(ac as usize);
    for _ in 0..ac {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        added.push(JpgComponentAdded { index, item: dec_frame_component_bin(reader)? });
    }
    Ok(JpgComponentsDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_quant_table_diff_bin(d: &JpgQuantTableDiff, out: &mut Vec<u8>) {
    write_opt(out, &d.precision, |v, out| out.push(*v));
    write_opt(out, &d.values, |v, out| {
        for x in v.iter() {
            out.extend_from_slice(&x.to_le_bytes());
        }
    });
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_quant_table_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgQuantTableDiff, String> {
    Ok(JpgQuantTableDiff {
        precision: read_opt(reader, |r| r.read_u8().map_err(|e| e.to_string()))?,
        values: read_opt(reader, |r| {
            let mut values = [0u16; 64];
            for v in values.iter_mut() {
                *v = r.read_u16_le().map_err(|e| e.to_string())?;
            }
            Ok(values)
        })?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_quant_tables_diff_bin(d: &JpgQuantTablesDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for id in &d.removed {
        out.push(*id);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        out.push(m.id);
        enc_quant_table_diff_bin(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_quant_table_bin(&a.item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_quant_tables_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgQuantTablesDiff, String> {
    let rc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(rc as usize);
    for _ in 0..rc {
        removed.push(reader.read_u8().map_err(|e| e.to_string())?);
    }
    let mc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mc as usize);
    for _ in 0..mc {
        let id = reader.read_u8().map_err(|e| e.to_string())?;
        modified.push(JpgQuantTableModified { id, diff: dec_quant_table_diff_bin(reader)? });
    }
    let ac = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(ac as usize);
    for _ in 0..ac {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        added.push(JpgQuantTableAdded { index, item: dec_quant_table_bin(reader)? });
    }
    Ok(JpgQuantTablesDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_huffman_table_diff_bin(d: &JpgHuffmanTableDiff, out: &mut Vec<u8>) {
    write_opt(out, &d.bits, |v, out| out.extend_from_slice(v));
    write_opt(out, &d.values, |v, out| write_bytes_lp(out, v));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_huffman_table_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgHuffmanTableDiff, String> {
    Ok(JpgHuffmanTableDiff {
        bits: read_opt(reader, |r| {
            let v = r.read_bytes(16).map_err(|e| e.to_string())?.to_vec();
            v.try_into().map_err(|_| "huffman table diff bits: expected 16 bytes".to_string())
        })?,
        values: read_opt(reader, read_bytes_lp)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_huffman_tables_diff_bin(d: &JpgHuffmanTablesDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for k in &d.removed {
        enc_huffman_key_bin(k, out);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        enc_huffman_key_bin(&m.key, out);
        enc_huffman_table_diff_bin(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_huffman_table_bin(&a.item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_huffman_tables_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgHuffmanTablesDiff, String> {
    let rc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(rc as usize);
    for _ in 0..rc {
        removed.push(dec_huffman_key_bin(reader)?);
    }
    let mc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mc as usize);
    for _ in 0..mc {
        let key = dec_huffman_key_bin(reader)?;
        modified.push(JpgHuffmanTableModified { key, diff: dec_huffman_table_diff_bin(reader)? });
    }
    let ac = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(ac as usize);
    for _ in 0..ac {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        added.push(JpgHuffmanTableAdded { index, item: dec_huffman_table_bin(reader)? });
    }
    Ok(JpgHuffmanTablesDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_segment_diff_bin(d: &JpgSegmentDiff, out: &mut Vec<u8>) {
    write_opt(out, &d.marker, |v, out| out.push(*v));
    write_opt(out, &d.data, |v, out| write_bytes_lp(out, v));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_segment_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgSegmentDiff, String> {
    Ok(JpgSegmentDiff { marker: read_opt(reader, |r| r.read_u8().map_err(|e| e.to_string()))?, data: read_opt(reader, read_bytes_lp)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_other_segments_diff_bin(d: &JpgOtherSegmentsDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for i in &d.removed {
        store::pack_rt::write_varint_u64(out, *i as u64);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        store::pack_rt::write_varint_u64(out, m.index as u64);
        enc_segment_diff_bin(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_segment_bin(&a.item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_other_segments_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgOtherSegmentsDiff, String> {
    let rc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(rc as usize);
    for _ in 0..rc {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let mc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mc as usize);
    for _ in 0..mc {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        modified.push(JpgSegmentModified { index, diff: dec_segment_diff_bin(reader)? });
    }
    let ac = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(ac as usize);
    for _ in 0..ac {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        added.push(JpgSegmentAdded { index, item: dec_segment_bin(reader)? });
    }
    Ok(JpgOtherSegmentsDiff { removed, modified, added })
}

/// 🌲 `JpgFrameChange`'s binary tag: `0`=Modify(frame-fields-diff) / `1`=Replace(opt-frame-header)
/// — same tag numbering convention as `xml`'s `enc_node_diff_bin` (0/1/2 by declaration order).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_frame_change_bin(fc: &JpgFrameChange, out: &mut Vec<u8>) {
    match fc {
        JpgFrameChange::Modify(fd) => {
            out.push(0);
            enc_frame_fields_diff_bin(fd, out);
        }
        JpgFrameChange::Replace { frame } => {
            out.push(1);
            write_opt(out, frame, enc_frame_header_bin);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_frame_change_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgFrameChange, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(JpgFrameChange::Modify(dec_frame_fields_diff_bin(reader)?)),
        1 => Ok(JpgFrameChange::Replace { frame: read_opt(reader, dec_frame_header_bin)? }),
        other => Err(format!("frame change binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_frame_fields_diff_bin(fd: &JpgFrameFieldsDiff, out: &mut Vec<u8>) {
    write_opt(out, &fd.precision, |v, out| out.push(*v));
    write_opt(out, &fd.width, |v, out| store::pack_rt::write_varint_u64(out, *v as u64));
    write_opt(out, &fd.height, |v, out| store::pack_rt::write_varint_u64(out, *v as u64));
    write_opt(out, &fd.components, enc_components_diff_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_frame_fields_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgFrameFieldsDiff, String> {
    Ok(JpgFrameFieldsDiff {
        precision: read_opt(reader, |r| r.read_u8().map_err(|e| e.to_string()))?,
        width: read_opt(reader, |r| Ok(r.read_varint_u64().map_err(|e| e.to_string())? as u16))?,
        height: read_opt(reader, |r| Ok(r.read_varint_u64().map_err(|e| e.to_string())? as u16))?,
        components: read_opt(reader, dec_components_diff_bin)?,
    })
}

impl protocol::DiffBinary for JpgDiff {
/// 🧪️ P2-FG2: REAL binary frame (`format u8 | flags u16le | <present fields, in declaration
/// order>`), matching `../💾️binary/📡️.protocol.semio`'s `header fixed 3` + `chain
/// payload bytes` shape — upgraded from F6's `print_diff().into_bytes()` text-as-binary
/// shortcut (100% of stdio's `DiffCodec` impls were still on that shortcut per the P2-W0
/// census). `flags` bit `i` (LSB first) marks the field at declaration position `i` present
/// (`width`=0 .. `other_segments`=15 — 16 fields, hence `u16` rather than xml's `u8`); each
/// present field's own (possibly tri-state) binary payload follows in that fixed order, using
/// the real, non-recursive binary codecs in `§ValueBinaryCodecs`/`§DiffValueBinaryCodecs`
/// above — no opaque tail anywhere in THIS frame (unlike xml's `XmlNodeDiff`, none of jpg's
/// diff payloads are self-recursive).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut flags: u16 = 0;
    if self.width.is_some() {
        flags |= 1 << 0;
    }
    if self.height.is_some() {
        flags |= 1 << 1;
    }
    if self.pixels.is_some() {
        flags |= 1 << 2;
    }
    if self.jfif_version.is_some() {
        flags |= 1 << 3;
    }
    if self.jfif_density_units.is_some() {
        flags |= 1 << 4;
    }
    if self.jfif_x_density.is_some() {
        flags |= 1 << 5;
    }
    if self.jfif_y_density.is_some() {
        flags |= 1 << 6;
    }
    if self.jfif_thumbnail.is_some() {
        flags |= 1 << 7;
    }
    if self.frame.is_some() {
        flags |= 1 << 8;
    }
    if self.sof_marker.is_some() {
        flags |= 1 << 9;
    }
    if self.arithmetic.is_some() {
        flags |= 1 << 10;
    }
    if self.quant_tables.is_some() {
        flags |= 1 << 11;
    }
    if self.huffman_tables.is_some() {
        flags |= 1 << 12;
    }
    if self.restart_interval.is_some() {
        flags |= 1 << 13;
    }
    if self.other_segments.is_some() {
        flags |= 1 << 14;
    }

    let mut out = vec![store::pack_rt::OP_BINARY_FORMAT];
    out.extend_from_slice(&flags.to_le_bytes());
    if let Some(v) = self.width {
        store::pack_rt::write_varint_u64(&mut out, v as u64);
    }
    if let Some(v) = self.height {
        store::pack_rt::write_varint_u64(&mut out, v as u64);
    }
    if let Some(v) = &self.pixels {
        write_bytes_lp(&mut out, v);
    }
    if let Some(v) = self.jfif_version {
        enc_version_bin(&v, &mut out);
    }
    if let Some(v) = self.jfif_density_units {
        enc_density_units_bin(&v, &mut out);
    }
    if let Some(v) = self.jfif_x_density {
        store::pack_rt::write_varint_u64(&mut out, v as u64);
    }
    if let Some(v) = self.jfif_y_density {
        store::pack_rt::write_varint_u64(&mut out, v as u64);
    }
    if let Some(v) = &self.jfif_thumbnail {
        write_opt(&mut out, v, enc_thumbnail_bin);
    }
    if let Some(v) = &self.frame {
        enc_frame_change_bin(v, &mut out);
    }
    if let Some(v) = self.sof_marker {
        out.push(v);
    }
    if let Some(v) = self.arithmetic {
        out.push(if v { 1 } else { 0 });
    }
    if let Some(v) = &self.quant_tables {
        enc_quant_tables_diff_bin(v, &mut out);
    }
    if let Some(v) = &self.huffman_tables {
        enc_huffman_tables_diff_bin(v, &mut out);
    }
    if let Some(v) = &self.restart_interval {
        write_opt(&mut out, v, |ri, out| store::pack_rt::write_varint_u64(out, *ri as u64));
    }
    if let Some(v) = &self.other_segments {
        enc_other_segments_diff_bin(v, &mut out);
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
    let flags = reader.read_u16_le().map_err(|e| malformed("diff flags", 1, e.to_string()))?;

    let width = if flags & (1 << 0) != 0 { Some(reader.read_varint_u64().map_err(|e| malformed("diff width", reader.position(), e.to_string()))? as u32) } else { None };
    let height = if flags & (1 << 1) != 0 { Some(reader.read_varint_u64().map_err(|e| malformed("diff height", reader.position(), e.to_string()))? as u32) } else { None };
    let pixels = if flags & (1 << 2) != 0 { Some(read_bytes_lp(&mut reader).map_err(|e| malformed("diff pixels", reader.position(), e))?) } else { None };
    let jfif_version = if flags & (1 << 3) != 0 { Some(dec_version_bin(&mut reader).map_err(|e| malformed("diff jfif-version", reader.position(), e))?) } else { None };
    let jfif_density_units = if flags & (1 << 4) != 0 { Some(dec_density_units_bin(&mut reader).map_err(|e| malformed("diff jfif-density-units", reader.position(), e))?) } else { None };
    let jfif_x_density = if flags & (1 << 5) != 0 { Some(reader.read_varint_u64().map_err(|e| malformed("diff jfif-x-density", reader.position(), e.to_string()))? as u16) } else { None };
    let jfif_y_density = if flags & (1 << 6) != 0 { Some(reader.read_varint_u64().map_err(|e| malformed("diff jfif-y-density", reader.position(), e.to_string()))? as u16) } else { None };
    let jfif_thumbnail = if flags & (1 << 7) != 0 { Some(read_opt(&mut reader, dec_thumbnail_bin).map_err(|e| malformed("diff jfif-thumbnail", reader.position(), e))?) } else { None };
    let frame = if flags & (1 << 8) != 0 { Some(dec_frame_change_bin(&mut reader).map_err(|e| malformed("diff frame", reader.position(), e))?) } else { None };
    let sof_marker = if flags & (1 << 9) != 0 { Some(reader.read_u8().map_err(|e| malformed("diff sof-marker", reader.position(), e.to_string()))?) } else { None };
    let arithmetic = if flags & (1 << 10) != 0 { Some(reader.read_u8().map_err(|e| malformed("diff arithmetic", reader.position(), e.to_string()))? != 0) } else { None };
    let quant_tables = if flags & (1 << 11) != 0 { Some(dec_quant_tables_diff_bin(&mut reader).map_err(|e| malformed("diff quant-tables", reader.position(), e))?) } else { None };
    let huffman_tables = if flags & (1 << 12) != 0 { Some(dec_huffman_tables_diff_bin(&mut reader).map_err(|e| malformed("diff huffman-tables", reader.position(), e))?) } else { None };
    let restart_interval = if flags & (1 << 13) != 0 { Some(read_opt(&mut reader, |r| Ok(r.read_varint_u64().map_err(|e| e.to_string())? as u16)).map_err(|e| malformed("diff restart-interval", reader.position(), e))?) } else { None };
    let other_segments = if flags & (1 << 14) != 0 { Some(dec_other_segments_diff_bin(&mut reader).map_err(|e| malformed("diff other-segments", reader.position(), e))?) } else { None };

    Ok(JpgDiff { width, height, pixels, jfif_version, jfif_density_units, jfif_x_density, jfif_y_density, jfif_thumbnail, frame, sof_marker, arithmetic, quant_tables, huffman_tables, restart_interval, other_segments })
}
}

}
pub use diff_codec::*;
