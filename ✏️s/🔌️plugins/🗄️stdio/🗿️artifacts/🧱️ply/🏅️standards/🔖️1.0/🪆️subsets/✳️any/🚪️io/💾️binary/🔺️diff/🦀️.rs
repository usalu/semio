//! binary rep for stdio.ply 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1_0::subsets::any::schema::diff::*;
use crate::schema::snapshot::{PlyElement, PlyFormat, PlyProperty, PlyRow, PlyScalarType};
use crate::PlySnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, BTreeSet, HashSet};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::snapshot::PlyValue;

/// 🧪️ P2-FG3: real binary value codecs for `PlyDiff`'s (and `PlyMutation`'s, which reuses these
/// `pub(crate)` fns the same way it already reuses the text-codec primitives above) nested
/// types — mirrors the text codecs field-for-field, using `dsl::ByteWriter`/`dsl::ByteReader`
/// (the same real LEB128-varint/length-prefixed framework primitives gif89a's own upgraded
/// `GifDiff` binary frame uses, `🎞️gif/…/🏅️standards/🔖️89a/…/🔺️diff/🦀️.rs`'s
/// `RealBinaryPrimitives`/`RealBinaryDiffFrame` regions — `dsl`/`store`/`protocol` all alias the
/// same kernel crate root, reachable with no `use` needed beyond the absolute path). `ByteWriter`/
/// `ByteReader` have no i8/i16/f32 methods (only u8/u16/u32/u64/f64 + varint), so the signed/
/// narrow PLY scalar kinds go through raw `to_le_bytes`/`from_le_bytes` via `write_bytes`/
/// `read_bytes`, exactly like `⚙️engine/🦀️.rs`'s own `push_scalar_bin`/`read_scalar_bin`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_blob(w: &mut dsl::ByteWriter, bytes: &[u8]) {
    w.write_varint_u64(bytes.len() as u64);
    w.write_bytes(bytes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_blob(r: &mut dsl::ByteReader<'_>) -> Result<Vec<u8>, dsl::PackRefusal> {
    let len = r.read_varint_u64()? as usize;
    Ok(r.read_bytes(len)?.to_vec())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_str(w: &mut dsl::ByteWriter, s: &str) {
    write_bin_blob(w, s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_str(r: &mut dsl::ByteReader<'_>) -> Result<String, dsl::PackRefusal> {
    let bytes = read_bin_blob(r)?;
    String::from_utf8(bytes).map_err(|e| dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "ply binary utf8 string", offset: 0, detail: e.to_string() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_vec<T>(w: &mut dsl::ByteWriter, items: &[T], write_item: impl Fn(&mut dsl::ByteWriter, &T)) {
    w.write_varint_u64(items.len() as u64);
    for item in items {
        write_item(w, item);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_vec<T>(r: &mut dsl::ByteReader<'_>, mut read_item: impl FnMut(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackRefusal>) -> Result<Vec<T>, dsl::PackRefusal> {
    let n = r.read_varint_u64()? as usize;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        out.push(read_item(r)?);
    }
    Ok(out)
}

/// 🧩 2-way presence flag (`0`=None, `1`=Some) — shared by every plain `Option<T>` field.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_option<T>(w: &mut dsl::ByteWriter, v: &Option<T>, write_value: impl FnOnce(&mut dsl::ByteWriter, &T)) {
    match v {
        None => w.write_u8(0),
        Some(val) => {
            w.write_u8(1);
            write_value(w, val);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_option<T>(r: &mut dsl::ByteReader<'_>, read_value: impl FnOnce(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackRefusal>) -> Result<Option<T>, dsl::PackRefusal> {
    match r.read_u8()? {
        0 => Ok(None),
        1 => Ok(Some(read_value(r)?)),
        other => Err(dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "ply binary option tag", offset: 0, detail: format!("unknown tag {other}") }),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_format(w: &mut dsl::ByteWriter, f: PlyFormat) {
    w.write_u8(match f {
        PlyFormat::Ascii => 0,
        PlyFormat::BinaryLittleEndian => 1,
        PlyFormat::BinaryBigEndian => 2,
    });
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_format(r: &mut dsl::ByteReader<'_>) -> Result<PlyFormat, dsl::PackRefusal> {
    match r.read_u8()? {
        0 => Ok(PlyFormat::Ascii),
        1 => Ok(PlyFormat::BinaryLittleEndian),
        2 => Ok(PlyFormat::BinaryBigEndian),
        other => Err(dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "ply binary format tag", offset: 0, detail: format!("unknown tag {other}") }),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_scalar_type(w: &mut dsl::ByteWriter, k: PlyScalarType) {
    w.write_u8(match k {
        PlyScalarType::Char => 0,
        PlyScalarType::UChar => 1,
        PlyScalarType::Short => 2,
        PlyScalarType::UShort => 3,
        PlyScalarType::Int => 4,
        PlyScalarType::UInt => 5,
        PlyScalarType::Float => 6,
        PlyScalarType::Double => 7,
    });
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_scalar_type(r: &mut dsl::ByteReader<'_>) -> Result<PlyScalarType, dsl::PackRefusal> {
    match r.read_u8()? {
        0 => Ok(PlyScalarType::Char),
        1 => Ok(PlyScalarType::UChar),
        2 => Ok(PlyScalarType::Short),
        3 => Ok(PlyScalarType::UShort),
        4 => Ok(PlyScalarType::Int),
        5 => Ok(PlyScalarType::UInt),
        6 => Ok(PlyScalarType::Float),
        7 => Ok(PlyScalarType::Double),
        other => Err(dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "ply binary scalar type tag", offset: 0, detail: format!("unknown tag {other}") }),
    }
}

/// 🔣️ `PlyValue` real binary — one tag byte (matching `write_bin_scalar_type`'s own 0-7 order for
/// the 8 scalar kinds) then the raw little-endian payload at its declared width, plus `8` for the
/// recursive `List(Vec<PlyValue>)` variant (self-recursion, real — not opaque, `write_bin_vec`
/// calling back into `write_bin_value` for every item).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_value(w: &mut dsl::ByteWriter, v: &PlyValue) {
    match v {
        PlyValue::Char(x) => {
            w.write_u8(0);
            w.write_bytes(&x.to_le_bytes());
        }
        PlyValue::UChar(x) => {
            w.write_u8(1);
            w.write_u8(*x);
        }
        PlyValue::Short(x) => {
            w.write_u8(2);
            w.write_bytes(&x.to_le_bytes());
        }
        PlyValue::UShort(x) => {
            w.write_u8(3);
            w.write_u16_le(*x);
        }
        PlyValue::Int(x) => {
            w.write_u8(4);
            w.write_bytes(&x.to_le_bytes());
        }
        PlyValue::UInt(x) => {
            w.write_u8(5);
            w.write_u32_le(*x);
        }
        PlyValue::Float(x) => {
            w.write_u8(6);
            w.write_bytes(&x.to_le_bytes());
        }
        PlyValue::Double(x) => {
            w.write_u8(7);
            w.write_f64_le(*x);
        }
        PlyValue::List(items) => {
            w.write_u8(8);
            write_bin_vec(w, items, write_bin_value);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_value(r: &mut dsl::ByteReader<'_>) -> Result<PlyValue, dsl::PackRefusal> {
    let malformed = |offset: usize, detail: String| dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "ply binary value", offset: offset as u64, detail };
    match r.read_u8()? {
        0 => Ok(PlyValue::Char(i8::from_le_bytes(r.read_bytes(1)?.try_into().map_err(|_| malformed(r.position(), "expected 1 byte".into()))?))),
        1 => Ok(PlyValue::UChar(r.read_u8()?)),
        2 => Ok(PlyValue::Short(i16::from_le_bytes(r.read_bytes(2)?.try_into().map_err(|_| malformed(r.position(), "expected 2 bytes".into()))?))),
        3 => Ok(PlyValue::UShort(r.read_u16_le()?)),
        4 => Ok(PlyValue::Int(i32::from_le_bytes(r.read_bytes(4)?.try_into().map_err(|_| malformed(r.position(), "expected 4 bytes".into()))?))),
        5 => Ok(PlyValue::UInt(r.read_u32_le()?)),
        6 => Ok(PlyValue::Float(f32::from_le_bytes(r.read_bytes(4)?.try_into().map_err(|_| malformed(r.position(), "expected 4 bytes".into()))?))),
        7 => Ok(PlyValue::Double(r.read_f64_le()?)),
        8 => Ok(PlyValue::List(read_bin_vec(r, read_bin_value)?)),
        other => Err(dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "ply binary value tag", offset: 0, detail: format!("unknown tag {other}") }),
    }
}

/// 🔣️ `PlyProperty` real binary — `0`=Scalar`{name,kind}`, `1`=List`{name,count_kind,value_kind}`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_property(w: &mut dsl::ByteWriter, p: &PlyProperty) {
    match p {
        PlyProperty::Scalar { name, kind } => {
            w.write_u8(0);
            write_bin_str(w, name);
            write_bin_scalar_type(w, *kind);
        }
        PlyProperty::List { name, count_kind, value_kind } => {
            w.write_u8(1);
            write_bin_str(w, name);
            write_bin_scalar_type(w, *count_kind);
            write_bin_scalar_type(w, *value_kind);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_property(r: &mut dsl::ByteReader<'_>) -> Result<PlyProperty, dsl::PackRefusal> {
    match r.read_u8()? {
        0 => Ok(PlyProperty::Scalar { name: read_bin_str(r)?, kind: read_bin_scalar_type(r)? }),
        1 => Ok(PlyProperty::List { name: read_bin_str(r)?, count_kind: read_bin_scalar_type(r)?, value_kind: read_bin_scalar_type(r)? }),
        other => Err(dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "ply binary property tag", offset: 0, detail: format!("unknown tag {other}") }),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_row(w: &mut dsl::ByteWriter, row: &PlyRow) {
    write_bin_vec(w, &row.values, write_bin_value);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_row(r: &mut dsl::ByteReader<'_>) -> Result<PlyRow, dsl::PackRefusal> {
    Ok(PlyRow { values: read_bin_vec(r, read_bin_value)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_element(w: &mut dsl::ByteWriter, e: &PlyElement) {
    write_bin_str(w, &e.name);
    w.write_varint_u64(e.count as u64);
    write_bin_vec(w, &e.properties, write_bin_property);
    write_bin_vec(w, &e.rows, write_bin_row);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_element(r: &mut dsl::ByteReader<'_>) -> Result<PlyElement, dsl::PackRefusal> {
    let name = read_bin_str(r)?;
    let count = r.read_varint_u64()?;
    let properties = read_bin_vec(r, read_bin_property)?;
    let rows = read_bin_vec(r, read_bin_row)?;
    Ok(PlyElement { name, count, properties, rows })
}

/// 🧪️ P2-FG3: real binary encodings for `PlyRowDiff`/`PlyRowsDiff`/`PlyElementDiff`/
/// `PlyElementsDiff` — each produces one opaque `Vec<u8>` blob matching
/// `../💾️binary/📡️.protocol.semio`'s `Array(u8, Field(<name>_len))` fields exactly (the
/// blob's OWN internal removed/modified/added shape isn't further protocol-walkable, see that
/// file's own doc comment); the Rust codec here IS genuinely, fully structured (real varint
/// counts, real per-item recursive encoding, incl. `PlyValue::List`'s own self-recursion), never
/// text-as-bytes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_row_field_change(w: &mut dsl::ByteWriter, c: &PlyRowFieldChange) {
    write_bin_str(w, &c.name);
    write_bin_value(w, &c.value);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_row_field_change(r: &mut dsl::ByteReader<'_>) -> Result<PlyRowFieldChange, dsl::PackRefusal> {
    Ok(PlyRowFieldChange { name: read_bin_str(r)?, value: read_bin_value(r)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_row_diff(w: &mut dsl::ByteWriter, d: &PlyRowDiff) {
    write_bin_vec(w, &d.fields, write_bin_row_field_change);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_row_diff(r: &mut dsl::ByteReader<'_>) -> Result<PlyRowDiff, dsl::PackRefusal> {
    Ok(PlyRowDiff { fields: read_bin_vec(r, read_bin_row_field_change)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_rows_diff(w: &mut dsl::ByteWriter, d: &PlyRowsDiff) {
    write_bin_vec(w, &d.removed, |w, v: &usize| w.write_varint_u64(*v as u64));
    write_bin_vec(w, &d.modified, |w, m: &PlyRowModified| {
        w.write_varint_u64(m.index as u64);
        write_bin_row_diff(w, &m.diff);
    });
    write_bin_vec(w, &d.added, |w, a: &PlyRowAdded| {
        w.write_varint_u64(a.index as u64);
        write_bin_row(w, &a.row);
    });
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_rows_diff(r: &mut dsl::ByteReader<'_>) -> Result<PlyRowsDiff, dsl::PackRefusal> {
    let removed = read_bin_vec(r, |r| Ok(r.read_varint_u64()? as usize))?;
    let modified = read_bin_vec(r, |r| {
        let index = r.read_varint_u64()? as usize;
        let diff = read_bin_row_diff(r)?;
        Ok(PlyRowModified { index, diff })
    })?;
    let added = read_bin_vec(r, |r| {
        let index = r.read_varint_u64()? as usize;
        let row = read_bin_row(r)?;
        Ok(PlyRowAdded { index, row })
    })?;
    Ok(PlyRowsDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_element_diff(w: &mut dsl::ByteWriter, d: &PlyElementDiff) {
    write_bin_option(w,&d.count,|writer,count|writer.write_varint_u64(*count));
    write_bin_option(w, &d.properties, |w, props: &Vec<PlyProperty>| write_bin_vec(w, props, write_bin_property));
    write_bin_option(w, &d.rows, write_bin_rows_diff);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_element_diff(r: &mut dsl::ByteReader<'_>) -> Result<PlyElementDiff, dsl::PackRefusal> {
    let count=read_bin_option(r,|reader|reader.read_varint_u64())?;
    let properties = read_bin_option(r, |r| read_bin_vec(r, read_bin_property))?;
    let rows = read_bin_option(r, read_bin_rows_diff)?;
    Ok(PlyElementDiff { count, properties, rows })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_elements_diff_bin(d: &PlyElementsDiff) -> Vec<u8> {
    let mut w = dsl::ByteWriter::new();
    write_bin_vec(&mut w, &d.removed, |w, n: &String| write_bin_str(w, n));
    write_bin_vec(&mut w, &d.modified, |w, m: &PlyElementModified| {
        write_bin_str(w, &m.name);
        write_bin_element_diff(w, &m.diff);
    });
    write_bin_vec(&mut w, &d.added, |w, a: &PlyElementAdded| {
        w.write_varint_u64(a.index as u64);
        write_bin_element(w, &a.element);
    });
    w.into_bytes()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_elements_diff_bin(bytes: &[u8]) -> Result<PlyElementsDiff, dsl::PackRefusal> {
    let mut r = dsl::ByteReader::new(bytes);
    let removed = read_bin_vec(&mut r, read_bin_str)?;
    let modified = read_bin_vec(&mut r, |r| {
        let name = read_bin_str(r)?;
        let diff = read_bin_element_diff(r)?;
        Ok(PlyElementModified { name, diff })
    })?;
    let added = read_bin_vec(&mut r, |r| {
        let index = r.read_varint_u64()? as usize;
        let element = read_bin_element(r)?;
        Ok(PlyElementAdded { index, element })
    })?;
    Ok(PlyElementsDiff { removed, modified, added })
}

impl protocol::DiffBinary for PlyDiff {
/// ⚡️ P2-FG3: real binary diff-frame — upgraded from the F6-era `print_diff().into_bytes()`
/// text-as-binary shortcut (100% of stdio's `DiffCodec` impls were still on that shortcut per
/// the P2-W0 census). Matches `../💾️binary/📡️.protocol.semio`'s real flag-per-field
/// layout exactly, field for field, in struct order (`format`, `comments`, `elements`).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut w = dsl::ByteWriter::new();
    write_bin_option(&mut w, &self.format, |w, f| write_bin_format(w, *f));
    write_bin_option(&mut w, &self.comments, |w, v: &Vec<String>| {
        let mut inner = dsl::ByteWriter::new();
        write_bin_vec(&mut inner, v, |w, c: &String| write_bin_str(w, c));
        write_bin_blob(w, &inner.into_bytes());
    });
    write_bin_option(&mut w, &self.elements, |w, v| write_bin_blob(w, &enc_elements_diff_bin(v)));
    Ok(w.into_bytes())
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut r = dsl::ByteReader::new(bytes);
    let format = read_bin_option(&mut r, read_bin_format).map_err(|error| diff_pack_err(&error))?;
    let comments = read_bin_option(&mut r, |r| {
        let blob = read_bin_blob(r)?;
        let mut inner = dsl::ByteReader::new(&blob);
        read_bin_vec(&mut inner, read_bin_str)
    })
    .map_err(|error| diff_pack_err(&error))?;
    let elements = read_bin_option(&mut r, |r| dec_elements_diff_bin(&read_bin_blob(r)?)).map_err(|error| diff_pack_err(&error))?;
    Ok(PlyDiff { format, comments, elements })
}
}
}
pub use diff_codec::*;

#[allow(unused_imports)]
mod diff_wire_codec {
use super::*;
use crate::standards::v1_0::subsets::any::schema::diff::*;
use crate::schema::snapshot::{PlyElement, PlyFormat, PlyProperty, PlyRow, PlyScalarType};
use crate::PlySnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, BTreeSet, HashSet};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::snapshot::PlyValue;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn diff_pack_err(e: &dsl::PackRefusal) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "ply diff binary", offset: 0, detail: e.to_string() }
}
}
pub use diff_wire_codec::*;
