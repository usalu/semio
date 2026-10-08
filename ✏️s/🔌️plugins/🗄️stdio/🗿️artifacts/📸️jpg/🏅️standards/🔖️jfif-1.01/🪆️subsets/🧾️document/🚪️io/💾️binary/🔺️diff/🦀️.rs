//! jpg rep for stdio.jpg 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_jfif_1_01::subsets::document::schema::diff::*;
use crate::schema::snapshot::{JfifDensityUnits, JfifThumbnail, JpgSegment};
use crate::JpgSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, HashMap};

/// 🧪️ P2-FG2: real LEB128-varint-framed binary primitives backing the upgraded `DiffCodec`
/// (below) and, via `pub(crate)`, `../🧬️mutations/🦀️.rs`'s upgraded `OpBinary` — mirrors
/// `📰️xml`'s own `write_bytes_lp`/`read_bytes_lp` shape (`📖️grammar-recipe.md` §2.5), reusing
/// `store::pack_rt::write_varint_u64`/`store::ByteReader` rather than reinventing varint codecs.
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}

pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = usize::try_from(reader.read_varint_u64().map_err(|e| e.to_string())?).map_err(|_|"JPEG byte length exceeds platform".to_string())?;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}

/// 🏳️ Generic `Option<T>` presence-byte codec (`0`/`1` + payload) — the binary twin of the text
/// side's `encode_option`/`decode_option`, used for every tri-state/plain-optional field below.
pub(crate) fn write_opt<T>(out: &mut Vec<u8>, opt: &Option<T>, enc: impl FnOnce(&T, &mut Vec<u8>)) {
    out.push(if opt.is_some() { 1 } else { 0 });
    if let Some(v) = opt {
        enc(v, out);
    }
}

pub(crate) fn read_opt<T>(reader: &mut store::ByteReader<'_>, dec: impl FnOnce(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Option<T>, String> {
    let has = reader.read_u8().map_err(|e| e.to_string())?;
    if has > 1{return Err("JPEG optional presence must be zero or one".into());}
    if has == 1 {
        Ok(Some(dec(reader)?))
    } else {
        Ok(None)
    }
}

/// 🧪️ P2-FG2: real recursive-free binary twins of `§ValueCodecs` above — every type here is a
/// bounded, non-recursive record/collection (unlike xml's self-recursive `XmlNode`), so every
/// field is genuinely, individually written/read; no opaque payload anywhere in this region.
pub(crate) fn enc_version_bin(v: &(u8, u8), out: &mut Vec<u8>) {
    out.push(v.0);
    out.push(v.1);
}

pub(crate) fn dec_version_bin(reader: &mut store::ByteReader<'_>) -> Result<(u8, u8), String> {
    Ok((reader.read_u8().map_err(|e| e.to_string())?, reader.read_u8().map_err(|e| e.to_string())?))
}

pub(crate) fn enc_density_units_bin(u: &JfifDensityUnits, out: &mut Vec<u8>) {
    out.push(u.to_u8());
}

pub(crate) fn dec_density_units_bin(reader: &mut store::ByteReader<'_>) -> Result<JfifDensityUnits, String> {
    JfifDensityUnits::from_u8(reader.read_u8().map_err(|e| e.to_string())?)
}

pub(crate) fn enc_thumbnail_bin(t: &JfifThumbnail, out: &mut Vec<u8>) {
    out.push(t.width);
    out.push(t.height);
    write_bytes_lp(out, &t.rgb_data);
}

pub(crate) fn dec_thumbnail_bin(reader: &mut store::ByteReader<'_>) -> Result<JfifThumbnail, String> {
    let width = reader.read_u8().map_err(|e| e.to_string())?;
    let height = reader.read_u8().map_err(|e| e.to_string())?;
    let rgb_data = read_bytes_lp(reader)?;
    Ok(JfifThumbnail { width, height, rgb_data })
}

pub(crate) fn enc_segment_bin(s: &JpgSegment, out: &mut Vec<u8>) {
    out.push(s.marker);
    write_bytes_lp(out, &s.data);
}

pub(crate) fn dec_segment_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgSegment, String> {
    let marker = reader.read_u8().map_err(|e| e.to_string())?;
    let data = read_bytes_lp(reader)?;
    Ok(JpgSegment { marker, data })
}

/// 🧪️ P2-FG2: real binary twins of `§DiffValueCodecs` above — every collection triple below is
/// `varint-count + real-item` (removed/modified/added, matching the recipe's §1.4 shape but
/// binary-framed rather than bracket-text-framed); no opaque payload anywhere in this region since
/// none of jpg's diff types are self-recursive (unlike xml's `XmlNodeDiff`).


pub(crate) fn enc_segment_diff_bin(d: &JpgSegmentDiff, out: &mut Vec<u8>) {
    write_opt(out, &d.marker, |v, out| out.push(*v));
    write_opt(out, &d.data, |v, out| write_bytes_lp(out, v));
}

pub(crate) fn dec_segment_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgSegmentDiff, String> {
    Ok(JpgSegmentDiff { marker: read_opt(reader, |r| r.read_u8().map_err(|e| e.to_string()))?, data: read_opt(reader, read_bytes_lp)? })
}

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

pub(crate) fn dec_other_segments_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgOtherSegmentsDiff, String> {
    let rc = usize::try_from(reader.read_varint_u64().map_err(|e| e.to_string())?).map_err(|_|"JPEG collection count exceeds platform".to_string())?;
    if rc>reader.remaining(){return Err("JPEG collection count exceeds payload".into());}
    let mut removed = Vec::with_capacity(rc as usize);
    for _ in 0..rc {
        removed.push(usize::try_from(reader.read_varint_u64().map_err(|e| e.to_string())?).map_err(|_|"JPEG segment index exceeds platform".to_string())?);
    }
    let mc = usize::try_from(reader.read_varint_u64().map_err(|e| e.to_string())?).map_err(|_|"JPEG collection count exceeds platform".to_string())?;
    if mc>reader.remaining(){return Err("JPEG collection count exceeds payload".into());}
    let mut modified = Vec::with_capacity(mc as usize);
    for _ in 0..mc {
        let index = usize::try_from(reader.read_varint_u64().map_err(|e| e.to_string())?).map_err(|_|"JPEG segment index exceeds platform".to_string())?;
        modified.push(JpgSegmentModified { index, diff: dec_segment_diff_bin(reader)? });
    }
    let ac = usize::try_from(reader.read_varint_u64().map_err(|e| e.to_string())?).map_err(|_|"JPEG collection count exceeds platform".to_string())?;
    if ac>reader.remaining(){return Err("JPEG collection count exceeds payload".into());}
    let mut added = Vec::with_capacity(ac as usize);
    for _ in 0..ac {
        let index = usize::try_from(reader.read_varint_u64().map_err(|e| e.to_string())?).map_err(|_|"JPEG segment index exceeds platform".to_string())?;
        added.push(JpgSegmentAdded { index, item: dec_segment_bin(reader)? });
    }
    Ok(JpgOtherSegmentsDiff { removed, modified, added })
}


impl protocol::DiffBinary for JpgDiff {
/// 💾️ Encodes nine semantic content fields in declared flag order.
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
    if self.other_segments.is_some() {
        flags |= 1 << 8;
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
    if let Some(v) = &self.other_segments {
        enc_other_segments_diff_bin(v, &mut out);
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    static SPEC:std::sync::OnceLock<Result<semio_framework_dsl::ProtocolFile,String>>=std::sync::OnceLock::new();
    let spec=SPEC.get_or_init(||semio_framework_dsl::parse_protocol(COMPONENT_PROTOCOL_SEMIO).map_err(|error|error.to_string())).as_ref().map_err(|detail|protocol::ProtocolError::Malformed{what:"JPEG diff schema",offset:0,detail:detail.clone()})?;
    semio_framework_dsl::walk_protocol(spec,bytes).map_err(|error|protocol::ProtocolError::Malformed{what:"JPEG diff framing",offset:error.offset as u64,detail:error.message})?;
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
    if format!=store::pack_rt::OP_BINARY_FORMAT{return Err(malformed("diff format",0,"unknown JPEG diff format".into()));}
    let flags = reader.read_u16_le().map_err(|e| malformed("diff flags", 1, e.to_string()))?;

    if flags&!0x01ff!=0{return Err(malformed("diff flags",1,"unknown JPEG content diff field".into()));}
    let width = if flags & (1 << 0) != 0 { Some(u32::try_from(reader.read_varint_u64().map_err(|e| malformed("diff width", reader.position(), e.to_string()))?).map_err(|e|malformed("diff width",reader.position(),e.to_string()))?) } else { None };
    let height = if flags & (1 << 1) != 0 { Some(u32::try_from(reader.read_varint_u64().map_err(|e| malformed("diff height", reader.position(), e.to_string()))?).map_err(|e|malformed("diff height",reader.position(),e.to_string()))?) } else { None };
    let pixels = if flags & (1 << 2) != 0 { Some(read_bytes_lp(&mut reader).map_err(|e| malformed("diff pixels", reader.position(), e))?) } else { None };
    let jfif_version = if flags & (1 << 3) != 0 { Some(dec_version_bin(&mut reader).map_err(|e| malformed("diff jfif-version", reader.position(), e))?) } else { None };
    let jfif_density_units = if flags & (1 << 4) != 0 { Some(dec_density_units_bin(&mut reader).map_err(|e| malformed("diff jfif-density-units", reader.position(), e))?) } else { None };
    let jfif_x_density = if flags & (1 << 5) != 0 { Some(u16::try_from(reader.read_varint_u64().map_err(|e| malformed("diff jfif-x-density", reader.position(), e.to_string()))?).map_err(|e|malformed("diff jfif-x-density",reader.position(),e.to_string()))?) } else { None };
    let jfif_y_density = if flags & (1 << 6) != 0 { Some(u16::try_from(reader.read_varint_u64().map_err(|e| malformed("diff jfif-y-density", reader.position(), e.to_string()))?).map_err(|e|malformed("diff jfif-y-density",reader.position(),e.to_string()))?) } else { None };
    let jfif_thumbnail = if flags & (1 << 7) != 0 { Some(read_opt(&mut reader, dec_thumbnail_bin).map_err(|e| malformed("diff jfif-thumbnail", reader.position(), e))?) } else { None };
    let other_segments = if flags & (1 << 8) != 0 { Some(dec_other_segments_diff_bin(&mut reader).map_err(|e| malformed("diff other-segments", reader.position(), e))?) } else { None };

    if reader.position()!=bytes.len(){return Err(malformed("diff payload",reader.position(),"trailing JPEG content diff bytes".into()));}
    Ok(JpgDiff { width, height, pixels, jfif_version, jfif_density_units, jfif_x_density, jfif_y_density, jfif_thumbnail, other_segments })
}
}

}
pub use diff_codec::*;
