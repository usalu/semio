//! tiff rep for stdio.tiff 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v6_0::subsets::document::schema::diff::*;
use crate::schema::snapshot::{TiffByteOrder, TiffFieldType, TiffIfd, TiffStorage, TiffStorageKind, TiffTag, TiffValues};
use crate::TiffSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// 🧪️ P2-FG2: real LEB128-varint-framed binary primitives (length-prefixed bytes/utf8) backing
/// the upgraded `DiffCodec`/`OpBinary` frames below (and, via re-export, `../🧬️mutations/
/// 🦀️.rs`'s own upgraded `OpBinary`) — reuses `store::pack_rt::write_varint_u64`/
/// `store::ByteReader` rather than reinventing varint encode/decode, same shape `xml`'s own
/// `write_str_lp`/`read_str_lp` uses.
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

/// 🧪️ P2-FG2: real recursive binary twins of [`enc_tag`]/[`dec_tag`]/[`enc_ifd`]/[`dec_ifd`]/
/// [`enc_values`]/[`dec_values`] above — a 1-byte kind tag (`0`=Byte/`1`=Ascii/`2`=Short/`3`=Long/
/// `4`=Rational/`5`=SByte/`6`=Undefined/`7`=SShort/`8`=SLong/`9`=SRational/`10`=Float/`11`=Double,
/// distinct numbering from the text codec's letter tags) followed by the real typed payload
/// (varint-length-prefixed bytes for `Byte`/`Ascii`/`Undefined`, a varint COUNT then that many
/// fixed-width LE elements for every numeric list, `Rational`/`SRational` pairs as two consecutive
/// fixed-width elements) — genuinely typed binary, NOT text-as-bytes. Backs the upgraded
/// `DiffCodec`/`OpBinary` frames below (`../🧬️mutations/🦀️.rs` reuses these via its own
/// `pub(crate)` re-export, same intra-artifact reuse convention `xml`'s `enc_xml_node_bin` uses).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_values_bin(v: &TiffValues, out: &mut Vec<u8>) {
    match v {
        TiffValues::Byte(b) => {
            out.push(0);
            write_bytes_lp(out, b);
        }
        TiffValues::Ascii(s) => {
            out.push(1);
            write_bytes_lp(out, s);
        }
        TiffValues::Short(v) => {
            out.push(2);
            store::pack_rt::write_varint_u64(out, v.len() as u64);
            v.iter().for_each(|x| out.extend_from_slice(&x.to_le_bytes()));
        }
        TiffValues::Long(v) => {
            out.push(3);
            store::pack_rt::write_varint_u64(out, v.len() as u64);
            v.iter().for_each(|x| out.extend_from_slice(&x.to_le_bytes()));
        }
        TiffValues::Rational(v) => {
            out.push(4);
            store::pack_rt::write_varint_u64(out, v.len() as u64);
            v.iter().for_each(|&(n, d)| {
                out.extend_from_slice(&n.to_le_bytes());
                out.extend_from_slice(&d.to_le_bytes());
            });
        }
        TiffValues::SByte(v) => {
            out.push(5);
            store::pack_rt::write_varint_u64(out, v.len() as u64);
            v.iter().for_each(|&x| out.push(x as u8));
        }
        TiffValues::Undefined(b) => {
            out.push(6);
            write_bytes_lp(out, b);
        }
        TiffValues::SShort(v) => {
            out.push(7);
            store::pack_rt::write_varint_u64(out, v.len() as u64);
            v.iter().for_each(|x| out.extend_from_slice(&x.to_le_bytes()));
        }
        TiffValues::SLong(v) => {
            out.push(8);
            store::pack_rt::write_varint_u64(out, v.len() as u64);
            v.iter().for_each(|x| out.extend_from_slice(&x.to_le_bytes()));
        }
        TiffValues::SRational(v) => {
            out.push(9);
            store::pack_rt::write_varint_u64(out, v.len() as u64);
            v.iter().for_each(|&(n, d)| {
                out.extend_from_slice(&n.to_le_bytes());
                out.extend_from_slice(&d.to_le_bytes());
            });
        }
        TiffValues::Float(v) => {
            out.push(10);
            store::pack_rt::write_varint_u64(out, v.len() as u64);
            v.iter().for_each(|x| out.extend_from_slice(&x.bits.to_le_bytes()));
        }
        TiffValues::Double(v) => {
            out.push(11);
            store::pack_rt::write_varint_u64(out, v.len() as u64);
            v.iter().for_each(|x| out.extend_from_slice(&x.bits.to_le_bytes()));
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_values_bin(reader: &mut store::ByteReader<'_>) -> Result<TiffValues, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    let count = |reader: &mut store::ByteReader<'_>| -> Result<u64, String> { reader.read_varint_u64().map_err(|e| e.to_string()) };
    match tag {
        0 => Ok(TiffValues::Byte(read_bytes_lp(reader)?)),
        1 => Ok(TiffValues::Ascii(read_bytes_lp(reader)?)),
        2 => {
            let n = count(reader)?;
            (0..n).map(|_| reader.read_u16_le().map_err(|e| e.to_string())).collect::<Result<Vec<_>, _>>().map(TiffValues::Short)
        }
        3 => {
            let n = count(reader)?;
            (0..n).map(|_| reader.read_u32_le().map_err(|e| e.to_string())).collect::<Result<Vec<_>, _>>().map(TiffValues::Long)
        }
        4 => {
            let n = count(reader)?;
            (0..n)
                .map(|_| {
                    let a = reader.read_u32_le().map_err(|e| e.to_string())?;
                    let b = reader.read_u32_le().map_err(|e| e.to_string())?;
                    Ok((a, b))
                })
                .collect::<Result<Vec<_>, String>>()
                .map(TiffValues::Rational)
        }
        5 => {
            let n = count(reader)?;
            (0..n).map(|_| reader.read_u8().map_err(|e| e.to_string()).map(|b| b as i8)).collect::<Result<Vec<_>, _>>().map(TiffValues::SByte)
        }
        6 => Ok(TiffValues::Undefined(read_bytes_lp(reader)?)),
        7 => {
            let n = count(reader)?;
            (0..n).map(|_| reader.read_u16_le().map_err(|e| e.to_string()).map(|x| x as i16)).collect::<Result<Vec<_>, _>>().map(TiffValues::SShort)
        }
        8 => {
            let n = count(reader)?;
            (0..n).map(|_| reader.read_u32_le().map_err(|e| e.to_string()).map(|x| x as i32)).collect::<Result<Vec<_>, _>>().map(TiffValues::SLong)
        }
        9 => {
            let n = count(reader)?;
            (0..n)
                .map(|_| {
                    let a = reader.read_u32_le().map_err(|e| e.to_string())? as i32;
                    let b = reader.read_u32_le().map_err(|e| e.to_string())? as i32;
                    Ok((a, b))
                })
                .collect::<Result<Vec<_>, String>>()
                .map(TiffValues::SRational)
        }
        10 => {
            let n = count(reader)?;
            (0..n).map(|_| reader.read_u32_le().map_err(|e| e.to_string()).map(|bits| crate::schema::snapshot::TiffBinary32 { bits })).collect::<Result<Vec<_>, _>>().map(TiffValues::Float)
        }
        11 => {
            let n = count(reader)?;
            (0..n).map(|_| reader.read_bytes(8).map_err(|e| e.to_string()).map(|bytes| crate::schema::snapshot::TiffBinary64 { bits: u64::from_le_bytes(bytes.try_into().expect("8 bytes")) })).collect::<Result<Vec<_>, _>>().map(TiffValues::Double)
        }
        other => Err(format!("tiff values binary: unknown tag {other}")),
    }
}

/// 🏷️ Binary twin of [`enc_tag`]/[`dec_tag`] — `tag:u16le, kind:u8 (TIFF field-type code 1-12,
/// always fits one byte), values:enc_values_bin`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_tag_bin(t: &TiffTag, out: &mut Vec<u8>) {
    out.extend_from_slice(&t.tag.to_le_bytes());
    enc_values_bin(&t.values, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_tag_bin(reader: &mut store::ByteReader<'_>) -> Result<TiffTag, String> {
    let tag = reader.read_u16_le().map_err(|e| e.to_string())?;
    let values = dec_values_bin(reader)?;
    Ok(TiffTag { tag, values })
}

/// 🗂️ Binary twin of [`enc_ifd`]/[`dec_ifd`] — varint entry count, then that many [`enc_tag_bin`]
/// entries.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_ifd_bin(ifd: &TiffIfd, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, ifd.entries.len() as u64);
    ifd.entries.iter().for_each(|t| enc_tag_bin(t, out));
    enc_storage_bin(&ifd.storage,out);
}

pub(crate) fn enc_storage_bin(storage:&TiffStorage,out:&mut Vec<u8>){out.push(match storage.kind{TiffStorageKind::None=>0,TiffStorageKind::Strips=>1,TiffStorageKind::Tiles=>2});out.push(storage.offsets_kind.to_u16() as u8);out.push(storage.byte_counts_kind.to_u16() as u8);store::pack_rt::write_varint_u64(out,storage.chunks.len() as u64);for chunk in &storage.chunks{write_bytes_lp(out,chunk);}}

pub(crate) fn dec_storage_bin(reader:&mut store::ByteReader<'_>)->Result<TiffStorage,String>{let kind=match reader.read_u8().map_err(|e|e.to_string())?{0=>TiffStorageKind::None,1=>TiffStorageKind::Strips,2=>TiffStorageKind::Tiles,other=>return Err(format!("storage binary: unknown kind {other}"))};let offsets_kind=TiffFieldType::from_u16(reader.read_u8().map_err(|e|e.to_string())? as u16)?;let byte_counts_kind=TiffFieldType::from_u16(reader.read_u8().map_err(|e|e.to_string())? as u16)?;let count=reader.read_varint_u64().map_err(|e|e.to_string())?;let mut chunks=Vec::with_capacity(count as usize);for _ in 0..count{chunks.push(read_bytes_lp(reader)?);}Ok(TiffStorage{kind,offsets_kind,byte_counts_kind,chunks})}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_ifd_bin(reader: &mut store::ByteReader<'_>) -> Result<TiffIfd, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut entries = Vec::with_capacity(n as usize);
    for _ in 0..n {
        entries.push(dec_tag_bin(reader)?);
    }
    Ok(TiffIfd { entries, storage:dec_storage_bin(reader)? })
}

/// 🧪️ P2-FG2: real recursive binary twins of [`enc_tags_diff`]/[`dec_tags_diff`]/
/// [`enc_ifds_diff`]/[`dec_ifds_diff`] — every `removed`/`modified`/`added` triple becomes a
/// varint-counted, recursively-encoded list (same shape XML's `enc_children_diff_bin`/
/// `enc_attrs_diff_bin` use), backing the upgraded `DiffBinary::encode_diff`/`decode_diff` below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_tags_diff_bin(d: &TiffTagsDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    d.removed.iter().for_each(|&t| out.extend_from_slice(&t.to_le_bytes()));
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        out.extend_from_slice(&m.tag.to_le_bytes());
        enc_values_bin(&m.values, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        out.extend_from_slice(&a.tag.to_le_bytes());
        enc_values_bin(&a.values, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_tags_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<TiffTagsDiff, String> {
    let rn = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(rn as usize);
    for _ in 0..rn {
        removed.push(reader.read_u16_le().map_err(|e| e.to_string())?);
    }
    let mn = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mn as usize);
    for _ in 0..mn {
        let tag = reader.read_u16_le().map_err(|e| e.to_string())?;
        let values = dec_values_bin(reader)?;
        modified.push(TiffTagModified { tag, values });
    }
    let an = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(an as usize);
    for _ in 0..an {
        let tag = reader.read_u16_le().map_err(|e| e.to_string())?;
        let values = dec_values_bin(reader)?;
        added.push(TiffTagAdded { tag, values });
    }
    Ok(TiffTagsDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_ifds_diff_bin(d: &TiffIfdsDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    d.removed.iter().for_each(|&i| store::pack_rt::write_varint_u64(out, i as u64));
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        store::pack_rt::write_varint_u64(out, m.index as u64);
        enc_tags_diff_bin(&m.diff.entries, out);
        match &m.diff.storage {
            Some(storage) => {
                out.push(1);
                enc_storage_bin(storage,out);
            }
            None => out.push(0),
        }
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_ifd_bin(&a.ifd, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_ifds_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<TiffIfdsDiff, String> {
    let rn = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(rn as usize);
    for _ in 0..rn {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let mn = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mn as usize);
    for _ in 0..mn {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let entries = dec_tags_diff_bin(reader)?;
        let storage = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_storage_bin(reader)?) } else { None };
        modified.push(TiffIfdModified { index, diff: TiffIfdDiff { entries, storage } });
    }
    let an = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(an as usize);
    for _ in 0..an {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let ifd = dec_ifd_bin(reader)?;
        added.push(TiffIfdAdded { index, ifd });
    }
    Ok(TiffIfdsDiff { removed, modified, added })
}

impl protocol::DiffBinary for TiffDiff {
/// 🧪️ P2-FG2: REAL binary frame (`format u8 | flags u8 | [byte_order][ifds]`),
/// matching `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload
/// bytes` shape — upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (100%
/// of stdio's `DiffCodec` impls were still on that shortcut per the P2-W0 census). `flags` bits
/// 0/1 mark `byte_order`/`ifds` presence; each present field's own real typed
/// payload follows in that fixed order (`ifds` recurses through [`enc_ifds_diff_bin`] into the
/// tag-id-keyed triples and the 12-variant `TiffValues` union, genuinely structured all the
/// way down, never text-as-bytes).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut flags: u8 = 0;
    if self.byte_order.is_some() {
        flags |= 0b001;
    }
    if self.ifds.is_some() {
        flags |= 0b010;
    }
    let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
    if let Some(v) = self.byte_order {
        out.push(match v {
            TiffByteOrder::LittleEndian => 0,
            TiffByteOrder::BigEndian => 1,
        });
    }
    if let Some(d) = &self.ifds {
        enc_ifds_diff_bin(d, &mut out);
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
    let flags = reader.read_u8().map_err(|e| malformed("diff flags", 1, e.to_string()))?;
    let byte_order = if flags & 0b001 != 0 {
        let v = reader.read_u8().map_err(|e| malformed("diff byte_order", reader.position(), e.to_string()))?;
        Some(if v == 0 { TiffByteOrder::LittleEndian } else { TiffByteOrder::BigEndian })
    } else {
        None
    };
    let ifds = if flags & 0b010 != 0 { Some(dec_ifds_diff_bin(&mut reader).map_err(|e| malformed("diff ifds", reader.position(), e))?) } else { None };
    if flags & 0b100 != 0 { return Err(malformed("diff flags",1,"reserved flag is set".into())); }
    Ok(TiffDiff { byte_order, ifds })
}
}

}
pub use diff_codec::*;

#[allow(unused_imports)]
mod diff_wire_codec {
use super::*;
use crate::standards::v6_0::subsets::document::schema::diff::*;
use crate::schema::snapshot::{TiffByteOrder, TiffFieldType, TiffIfd, TiffStorage, TiffStorageKind, TiffTag, TiffValues};
use crate::TiffSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, BTreeSet, HashMap};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
}
pub use diff_wire_codec::*;
