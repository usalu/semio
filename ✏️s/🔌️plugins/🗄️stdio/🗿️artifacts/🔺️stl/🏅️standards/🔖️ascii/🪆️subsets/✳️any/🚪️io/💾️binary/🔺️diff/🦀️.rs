//! binary rep for stdio.stl 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_ascii::subsets::any::schema::diff::*;
use crate::schema::snapshot::StlTriangle;
use crate::StlSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeSet, HashMap, HashSet};

/// 🧪️ P2-FG1-FIX: real LEB128-varint-framed binary primitives backing the upgraded `OpBinary`
/// (`../🧬️mutations/🦀️.rs`) and `DiffCodec` (below) frames — reuses
/// `store::pack_rt::write_varint_u64`/`store::ByteReader` rather than reinventing varint encode/
/// decode (same shape `dxf`'s own `BinaryPrimitives`/`ItemBinaryCodecs` regions use).
/// `pub(crate)` so the mutations sibling reuses these rather than duplicating them a second time.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_bin(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_bin(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    String::from_utf8(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec()).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_f64_bin(out: &mut Vec<u8>, v: f64) {
    out.extend_from_slice(&v.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_f64_bin(reader: &mut store::ByteReader<'_>) -> Result<f64, String> {
    reader.read_f64_le().map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_option_bin<T>(out: &mut Vec<u8>, opt: &Option<T>, enc: impl FnOnce(&T, &mut Vec<u8>)) {
    match opt {
        None => out.push(0),
        Some(v) => {
            out.push(1);
            enc(v, out);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_option_bin<T>(reader: &mut store::ByteReader<'_>, dec: impl FnOnce(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Option<T>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(dec(reader)?)),
        other => Err(format!("option binary: unknown tag {other}")),
    }
}

/// 🧪️ P2-FG1-FIX: real recursive binary twins of [`enc_vec3`]/[`enc_vertices`]/[`enc_triangle`]
/// above — genuinely flat (no self-recursion, `StlTriangle` never references itself), so every
/// level is real fixed/varint-framed binary, never an opaque byte-chain.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vec3_bin(v: &[f64; 3], out: &mut Vec<u8>) {
    write_f64_bin(out, v[0]);
    write_f64_bin(out, v[1]);
    write_f64_bin(out, v[2]);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vec3_bin(reader: &mut store::ByteReader<'_>) -> Result<[f64; 3], String> {
    Ok([read_f64_bin(reader)?, read_f64_bin(reader)?, read_f64_bin(reader)?])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vertices_bin(vs: &[[f64; 3]; 3], out: &mut Vec<u8>) {
    for v in vs {
        enc_vec3_bin(v, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vertices_bin(reader: &mut store::ByteReader<'_>) -> Result<[[f64; 3]; 3], String> {
    Ok([dec_vec3_bin(reader)?, dec_vec3_bin(reader)?, dec_vec3_bin(reader)?])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_triangle_bin(t: &StlTriangle, out: &mut Vec<u8>) {
    enc_vec3_bin(&t.normal, out);
    enc_vertices_bin(&t.vertices, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_triangle_bin(reader: &mut store::ByteReader<'_>) -> Result<StlTriangle, String> {
    Ok(StlTriangle { normal: dec_vec3_bin(reader)?, vertices: dec_vertices_bin(reader)? })
}

/// 🧪️ P2-FG1-FIX: real recursive binary twin of [`enc_triangle_diff`]/[`dec_triangle_diff`] and
/// [`enc_triangles_diff`]/[`dec_triangles_diff`] above — `StlTriangleDiff` has no enum/tri-state
/// field (both `normal`/`vertices` are plain `Option<T>`), so [`write_option_bin`]/
/// [`read_option_bin`] cover both. `StlTrianglesDiff` is the one genuinely variable-length,
/// collection-of-records part of this frame (`removed: Vec<usize>`, `modified: Vec<{index,
/// diff}>`, `added: Vec<{index, triangle}>`) — real varint-counted, recursively-encoded lists,
/// same shape `md`'s own `enc_blocks_diff_bin`/`dec_blocks_diff_bin` uses for its collection
/// triple.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_triangle_diff_bin(d: &StlTriangleDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.normal, enc_vec3_bin);
    write_option_bin(out, &d.vertices, enc_vertices_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_triangle_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<StlTriangleDiff, String> {
    let normal = read_option_bin(reader, dec_vec3_bin)?;
    let vertices = read_option_bin(reader, dec_vertices_bin)?;
    Ok(StlTriangleDiff { normal, vertices })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_triangles_diff_bin(d: &StlTrianglesDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for idx in &d.removed {
        store::pack_rt::write_varint_u64(out, *idx as u64);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for entry in &d.modified {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_triangle_diff_bin(&entry.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for entry in &d.added {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_triangle_bin(&entry.triangle, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_triangles_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<StlTrianglesDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let diff = dec_triangle_diff_bin(reader)?;
        modified.push(StlTriangleModified { index, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let triangle = dec_triangle_bin(reader)?;
        added.push(StlTriangleAdded { index, triangle });
    }
    Ok(StlTrianglesDiff { removed, modified, added })
}

impl protocol::DiffBinary for StlDiff {
/// 🧪️ P2-FG1-FIX: REAL binary frame (`format u8 | flags u8 | [solid_name] | [triangles]`),
/// matching `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload
/// bytes` shape — upgraded from the prior `print_diff().into_bytes()` text-as-binary
/// shortcut. `flags` is a 2-bit presence mask (bit0=`solid_name`, bit1=`triangles`) since
/// `StlDiff` has TWO independently optional top-level fields (unlike `MdDiff`'s single
/// `blocks`, which only needed one `has_value` byte). `StlDiff`'s own field tree has ZERO
/// self-recursion (`StlTriangleDiff`/`StlTrianglesDiff` never reference `StlDiff` or
/// themselves) — every present field is real field-by-field binary all the way down
/// (`enc_triangles_diff_bin` → `enc_triangle_diff_bin`/`enc_triangle_bin` →
/// `enc_vertices_bin`/`enc_vec3_bin`/`write_f64_bin`), never an opaque byte-chain at the Rust
/// layer. Only the protocol-DIALECT file (not the Rust code) still frames `triangles`'
/// payload as one opaque trailing `chain payload bytes`: `removed`/`modified`/`added` are
/// variable-length VECTORS OF RECORDS, which hits the same `protocol-array-of-records`
/// `walk_protocol` gap this wave's `dxf`/`md` upgrades independently document (the dialect's
/// `array-prim`/`record`-block constructs are unexercised anywhere in this codebase and
/// `Prim::Ref`-adjacent array-of-records framing is the documented, non-blocking
/// `mechanism_gaps` entry every collection-triple diff hits this wave).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut flags = 0u8;
    if self.solid_name.is_some() {
        flags |= 0b01;
    }
    if self.triangles.is_some() {
        flags |= 0b10;
    }
    let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
    if let Some(name) = &self.solid_name {
        write_str_bin(&mut out, name);
    }
    if let Some(triangles) = &self.triangles {
        enc_triangles_diff_bin(triangles, &mut out);
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let _format = reader.read_u8().map_err(|e| protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: e.to_string() })?;
    let flags = reader.read_u8().map_err(|e| protocol::ProtocolError::Malformed { what: "diff flags", offset: 1, detail: e.to_string() })?;
    let solid_name = if flags & 0b01 != 0 { Some(read_str_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff solid_name", offset: reader.position() as u64, detail: e })?) } else { None };
    let triangles = if flags & 0b10 != 0 { Some(dec_triangles_diff_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff triangles", offset: reader.position() as u64, detail: e })?) } else { None };
    Ok(StlDiff { solid_name, triangles })
}
}
}
pub use diff_codec::*;
