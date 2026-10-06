//! binary rep for stdio.obj 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v3_0::subsets::any::schema::diff::*;
use crate::schema::snapshot::{ObjFace, ObjGroup, ObjNormal, ObjObject, ObjTexCoord, ObjVertex};
use crate::ObjSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeSet, HashMap, HashSet};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::snapshot::ObjFaceVertex;
use crate::schema::snapshot::ObjSmoothingRange;
use crate::schema::snapshot::ObjUnknownStatement;
use crate::schema::snapshot::ObjUsemtlRange;

/// 🧪️ P2-FG1: real LEB128-varint-framed binary primitives (length-prefixed strings, fixed-8-byte
/// `f64` little-endian, varint-encoded `u32`/`usize`, tag-byte `Option<T>`/tri-state
/// `Option<Option<T>>` wrappers) backing the upgraded `DiffBinary::encode_diff`/`decode_diff` frame
/// below — mirrors dxf/md's own `#region 🔖️BinaryPrimitives`, reusing
/// `store::pack_rt::write_varint_u64`/`store::ByteReader` rather than reinventing varint encode/
/// decode. `obj`'s whole diff tree is flat structs/`Vec`/`Option<T>` (module doc comment, §3b) — no
/// `Prim::Ref`-recursion gap applies to any SINGLE value here, only to the collection-triple SHAPE
/// itself (see the `.protocol.semio` sibling's comment), so every value below gets a full,
/// genuinely field-by-field binary frame, never an opaque byte-chain.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_f64_bin(out: &mut Vec<u8>, v: f64) {
    out.extend_from_slice(&v.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_f64_bin(reader: &mut store::ByteReader<'_>) -> Result<f64, String> {
    reader.read_f64_le().map_err(|e| e.to_string())
}

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
pub(crate) fn write_u32_bin(out: &mut Vec<u8>, v: u32) {
    store::pack_rt::write_varint_u64(out, v as u64);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_u32_bin(reader: &mut store::ByteReader<'_>) -> Result<u32, String> {
    Ok(reader.read_varint_u64().map_err(|e| e.to_string())? as u32)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_usize_bin(out: &mut Vec<u8>, v: usize) {
    store::pack_rt::write_varint_u64(out, v as u64);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_usize_bin(reader: &mut store::ByteReader<'_>) -> Result<usize, String> {
    Ok(reader.read_varint_u64().map_err(|e| e.to_string())? as usize)
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
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(dec(reader)?)),
        other => Err(format!("option binary: unknown tag {other}")),
    }
}

/// 🏳️ Tri-state `Option<Option<T>>` binary wrapper (`ObjVertexDiff::w`/`ObjTexCoordDiff::w`'s own
/// diff field, NOT `ObjDiff::mtllib` — that top-level field's OUTER `Option` layer is already
/// carried by `encode_diff`'s presence flags, so its payload only needs [`write_option_bin`] over
/// the remaining `Option<String>`) — `0`=unchanged (`None`), `1`=cleared (`Some(None)`),
/// `2`=set (`Some(Some(v))`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_tristate_bin<T>(out: &mut Vec<u8>, opt: &Option<Option<T>>, enc: impl Fn(&T, &mut Vec<u8>)) {
    match opt {
        None => out.push(0),
        Some(None) => out.push(1),
        Some(Some(v)) => {
            out.push(2);
            enc(v, out);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_tristate_bin<T>(reader: &mut store::ByteReader<'_>, dec: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Option<Option<T>>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(None)),
        2 => Ok(Some(Some(dec(reader)?))),
        other => Err(format!("tristate binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_vec_bin<T>(out: &mut Vec<u8>, items: &[T], enc: impl Fn(&T, &mut Vec<u8>)) {
    store::pack_rt::write_varint_u64(out, items.len() as u64);
    for item in items {
        enc(item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_vec_bin<T>(reader: &mut store::ByteReader<'_>, dec: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Vec<T>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(dec(reader)?);
    }
    Ok(out)
}

/// 🧪️ P2-FG1: real field-by-field binary twins of `#region 🔖️ValueCodecs` above.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vertex_bin(v: &ObjVertex, out: &mut Vec<u8>) {
    write_f64_bin(out, v.x);
    write_f64_bin(out, v.y);
    write_f64_bin(out, v.z);
    write_option_bin(out, &v.w, |w, o| write_f64_bin(o, *w));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vertex_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjVertex, String> {
    let x = read_f64_bin(reader)?;
    let y = read_f64_bin(reader)?;
    let z = read_f64_bin(reader)?;
    let w = read_option_bin(reader, read_f64_bin)?;
    Ok(ObjVertex { x, y, z, w })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_texcoord_bin(t: &ObjTexCoord, out: &mut Vec<u8>) {
    write_f64_bin(out, t.u);
    write_f64_bin(out, t.v);
    write_option_bin(out, &t.w, |w, o| write_f64_bin(o, *w));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_texcoord_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjTexCoord, String> {
    let u = read_f64_bin(reader)?;
    let v = read_f64_bin(reader)?;
    let w = read_option_bin(reader, read_f64_bin)?;
    Ok(ObjTexCoord { u, v, w })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_normal_bin(n: &ObjNormal, out: &mut Vec<u8>) {
    write_f64_bin(out, n.x);
    write_f64_bin(out, n.y);
    write_f64_bin(out, n.z);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_normal_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjNormal, String> {
    let x = read_f64_bin(reader)?;
    let y = read_f64_bin(reader)?;
    let z = read_f64_bin(reader)?;
    Ok(ObjNormal { x, y, z })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_face_vertex_bin(fv: &ObjFaceVertex, out: &mut Vec<u8>) {
    write_u32_bin(out, fv.vertex);
    write_option_bin(out, &fv.texcoord, |v, o| write_u32_bin(o, *v));
    write_option_bin(out, &fv.normal, |v, o| write_u32_bin(o, *v));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_face_vertex_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjFaceVertex, String> {
    let vertex = read_u32_bin(reader)?;
    let texcoord = read_option_bin(reader, read_u32_bin)?;
    let normal = read_option_bin(reader, read_u32_bin)?;
    Ok(ObjFaceVertex { vertex, texcoord, normal })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_face_bin(f: &ObjFace, out: &mut Vec<u8>) {
    write_vec_bin(out, &f.vertices, enc_face_vertex_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_face_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjFace, String> {
    Ok(ObjFace { vertices: read_vec_bin(reader, dec_face_vertex_bin)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_group_bin(g: &ObjGroup, out: &mut Vec<u8>) {
    write_str_bin(out, &g.name);
    write_vec_bin(out, &g.faces, |f, o| write_u64_bin(o, *f));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_group_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjGroup, String> {
    let name = read_str_bin(reader)?;
    let faces = read_vec_bin(reader, read_u64_bin)?;
    Ok(ObjGroup { name, faces })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_object_bin(o: &ObjObject, out: &mut Vec<u8>) {
    write_str_bin(out, &o.name);
    write_vec_bin(out, &o.faces, |f, out| write_u64_bin(out, *f));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_object_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjObject, String> {
    let name = read_str_bin(reader)?;
    let faces = read_vec_bin(reader, read_u64_bin)?;
    Ok(ObjObject { name, faces })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_usemtl_bin(u: &ObjUsemtlRange, out: &mut Vec<u8>) {
    write_u64_bin(out, u.face_index_from);
    write_str_bin(out, &u.material);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_usemtl_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjUsemtlRange, String> {
    let face_index_from = read_u64_bin(reader)?;
    let material = read_str_bin(reader)?;
    Ok(ObjUsemtlRange { face_index_from, material })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_smoothing_bin(sg: &ObjSmoothingRange, out: &mut Vec<u8>) {
    write_u64_bin(out, sg.face_index_from);
    write_option_bin(out, &sg.group, |g, o| write_u32_bin(o, *g));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_smoothing_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjSmoothingRange, String> {
    let face_index_from = read_u64_bin(reader)?;
    let group = read_option_bin(reader, read_u32_bin)?;
    Ok(ObjSmoothingRange { face_index_from, group })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_unknown_bin(u: &ObjUnknownStatement, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, u.line_index);
    write_str_bin(out, &u.raw);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_unknown_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjUnknownStatement, String> {
    let line_index = reader.read_varint_u64().map_err(|error| error.to_string())?;
    let raw = read_str_bin(reader)?;
    Ok(ObjUnknownStatement { line_index, raw })
}

/// 🧪️ P2-FG1: real field-by-field binary twins of `#region 🔖️DiffValueCodecs` above — each
/// per-item sparse patch encodes its fields in fixed declaration order via
/// [`write_option_bin`]/[`write_tristate_bin`] (no tag byte needed per field, unlike an enum
/// variant: the field ORDER itself is the schema, exactly [`enc_vertex_diff_bin`]'s shape
/// mirroring md's own `MdBlockDiff::Heading` arm).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vertex_diff_bin(d: &ObjVertexDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.x, |v, o| write_f64_bin(o, *v));
    write_option_bin(out, &d.y, |v, o| write_f64_bin(o, *v));
    write_option_bin(out, &d.z, |v, o| write_f64_bin(o, *v));
    write_tristate_bin(out, &d.w, |v, o| write_f64_bin(o, *v));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vertex_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjVertexDiff, String> {
    let x = read_option_bin(reader, read_f64_bin)?;
    let y = read_option_bin(reader, read_f64_bin)?;
    let z = read_option_bin(reader, read_f64_bin)?;
    let w = read_tristate_bin(reader, read_f64_bin)?;
    Ok(ObjVertexDiff { x, y, z, w })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_texcoord_diff_bin(d: &ObjTexCoordDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.u, |v, o| write_f64_bin(o, *v));
    write_option_bin(out, &d.v, |v, o| write_f64_bin(o, *v));
    write_tristate_bin(out, &d.w, |v, o| write_f64_bin(o, *v));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_texcoord_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjTexCoordDiff, String> {
    let u = read_option_bin(reader, read_f64_bin)?;
    let v = read_option_bin(reader, read_f64_bin)?;
    let w = read_tristate_bin(reader, read_f64_bin)?;
    Ok(ObjTexCoordDiff { u, v, w })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_normal_diff_bin(d: &ObjNormalDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.x, |v, o| write_f64_bin(o, *v));
    write_option_bin(out, &d.y, |v, o| write_f64_bin(o, *v));
    write_option_bin(out, &d.z, |v, o| write_f64_bin(o, *v));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_normal_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjNormalDiff, String> {
    let x = read_option_bin(reader, read_f64_bin)?;
    let y = read_option_bin(reader, read_f64_bin)?;
    let z = read_option_bin(reader, read_f64_bin)?;
    Ok(ObjNormalDiff { x, y, z })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_face_diff_bin(d: &ObjFaceDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.vertices, |v, o| write_vec_bin(o, v, enc_face_vertex_bin));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_face_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjFaceDiff, String> {
    let vertices = read_option_bin(reader, |r| read_vec_bin(r, dec_face_vertex_bin))?;
    Ok(ObjFaceDiff { vertices })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_group_diff_bin(d: &ObjGroupDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.faces, |v, o| write_vec_bin(o, v, |f, oo| write_u64_bin(oo, *f)));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_group_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjGroupDiff, String> {
    let faces = read_option_bin(reader, |r| read_vec_bin(r, read_u64_bin))?;
    Ok(ObjGroupDiff { faces })
}

/// 🧭️ Generic-shaped 3-section index-keyed/name-keyed collection-triple binary
/// encoder/decoder (mirrors dxf's own `enc_index_triple_bin`/`enc_name_triple_bin`), hand-
/// instantiated per collection below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_index_triple_bin<T, D>(removed: &[usize], modified: &[(usize, D)], added: &[(usize, T)], out: &mut Vec<u8>, enc_diff: impl Fn(&D, &mut Vec<u8>), enc_item: impl Fn(&T, &mut Vec<u8>)) {
    write_vec_bin(out, removed, |idx, o| write_usize_bin(o, *idx));
    store::pack_rt::write_varint_u64(out, modified.len() as u64);
    for (idx, d) in modified {
        write_usize_bin(out, *idx);
        enc_diff(d, out);
    }
    store::pack_rt::write_varint_u64(out, added.len() as u64);
    for (idx, item) in added {
        write_usize_bin(out, *idx);
        enc_item(item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_index_triple_bin<T, D>(reader: &mut store::ByteReader<'_>, dec_diff: impl Fn(&mut store::ByteReader<'_>) -> Result<D, String>, dec_item: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<IndexedDiffParts<D, T>, String> {
    let removed = read_vec_bin(reader, read_usize_bin)?;
    let mc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mc as usize);
    for _ in 0..mc {
        let idx = read_usize_bin(reader)?;
        let d = dec_diff(reader)?;
        modified.push((idx, d));
    }
    let ac = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(ac as usize);
    for _ in 0..ac {
        let idx = read_usize_bin(reader)?;
        let item = dec_item(reader)?;
        added.push((idx, item));
    }
    Ok((removed, modified, added))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_named_triple_bin<T, D>(removed: &[String], modified: &[(String, D)], added: &[(usize, T)], out: &mut Vec<u8>, enc_diff: impl Fn(&D, &mut Vec<u8>), enc_item: impl Fn(&T, &mut Vec<u8>)) {
    write_vec_bin(out, removed, |name, o| write_str_bin(o, name));
    store::pack_rt::write_varint_u64(out, modified.len() as u64);
    for (name, d) in modified {
        write_str_bin(out, name);
        enc_diff(d, out);
    }
    store::pack_rt::write_varint_u64(out, added.len() as u64);
    for (idx, item) in added {
        write_usize_bin(out, *idx);
        enc_item(item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_named_triple_bin<T, D>(reader: &mut store::ByteReader<'_>, dec_diff: impl Fn(&mut store::ByteReader<'_>) -> Result<D, String>, dec_item: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<NamedDiffParts<D, T>, String> {
    let removed = read_vec_bin(reader, read_str_bin)?;
    let mc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mc as usize);
    for _ in 0..mc {
        let name = read_str_bin(reader)?;
        let d = dec_diff(reader)?;
        modified.push((name, d));
    }
    let ac = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(ac as usize);
    for _ in 0..ac {
        let idx = read_usize_bin(reader)?;
        let item = dec_item(reader)?;
        added.push((idx, item));
    }
    Ok((removed, modified, added))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vertices_diff_bin(d: &ObjVerticesDiff, out: &mut Vec<u8>) {
    let modified: Vec<(usize, ObjVertexDiff)> = d.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let added: Vec<(usize, ObjVertex)> = d.added.iter().map(|a| (a.index, a.vertex.clone())).collect();
    enc_index_triple_bin(&d.removed, &modified, &added, out, enc_vertex_diff_bin, enc_vertex_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vertices_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjVerticesDiff, String> {
    let (removed, modified, added) = dec_index_triple_bin(reader, dec_vertex_diff_bin, dec_vertex_bin)?;
    Ok(ObjVerticesDiff { removed, modified: modified.into_iter().map(|(index, diff)| ObjVertexModified { index, diff }).collect(), added: added.into_iter().map(|(index, vertex)| ObjVertexAdded { index, vertex }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_texcoords_diff_bin(d: &ObjTexCoordsDiff, out: &mut Vec<u8>) {
    let modified: Vec<(usize, ObjTexCoordDiff)> = d.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let added: Vec<(usize, ObjTexCoord)> = d.added.iter().map(|a| (a.index, a.texcoord.clone())).collect();
    enc_index_triple_bin(&d.removed, &modified, &added, out, enc_texcoord_diff_bin, enc_texcoord_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_texcoords_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjTexCoordsDiff, String> {
    let (removed, modified, added) = dec_index_triple_bin(reader, dec_texcoord_diff_bin, dec_texcoord_bin)?;
    Ok(ObjTexCoordsDiff { removed, modified: modified.into_iter().map(|(index, diff)| ObjTexCoordModified { index, diff }).collect(), added: added.into_iter().map(|(index, texcoord)| ObjTexCoordAdded { index, texcoord }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_normals_diff_bin(d: &ObjNormalsDiff, out: &mut Vec<u8>) {
    let modified: Vec<(usize, ObjNormalDiff)> = d.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let added: Vec<(usize, ObjNormal)> = d.added.iter().map(|a| (a.index, a.normal.clone())).collect();
    enc_index_triple_bin(&d.removed, &modified, &added, out, enc_normal_diff_bin, enc_normal_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_normals_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjNormalsDiff, String> {
    let (removed, modified, added) = dec_index_triple_bin(reader, dec_normal_diff_bin, dec_normal_bin)?;
    Ok(ObjNormalsDiff { removed, modified: modified.into_iter().map(|(index, diff)| ObjNormalModified { index, diff }).collect(), added: added.into_iter().map(|(index, normal)| ObjNormalAdded { index, normal }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_faces_diff_bin(d: &ObjFacesDiff, out: &mut Vec<u8>) {
    let modified: Vec<(usize, ObjFaceDiff)> = d.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let added: Vec<(usize, ObjFace)> = d.added.iter().map(|a| (a.index, a.face.clone())).collect();
    enc_index_triple_bin(&d.removed, &modified, &added, out, enc_face_diff_bin, enc_face_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_faces_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjFacesDiff, String> {
    let (removed, modified, added) = dec_index_triple_bin(reader, dec_face_diff_bin, dec_face_bin)?;
    Ok(ObjFacesDiff { removed, modified: modified.into_iter().map(|(index, diff)| ObjFaceModified { index, diff }).collect(), added: added.into_iter().map(|(index, face)| ObjFaceAdded { index, face }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_groups_diff_bin(d: &ObjGroupsDiff, out: &mut Vec<u8>) {
    let modified: Vec<(String, ObjGroupDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, ObjGroup)> = d.added.iter().map(|a| (a.index, a.group.clone())).collect();
    enc_named_triple_bin(&d.removed, &modified, &added, out, enc_group_diff_bin, enc_group_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_groups_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjGroupsDiff, String> {
    let (removed, modified, added) = dec_named_triple_bin(reader, dec_group_diff_bin, dec_group_bin)?;
    Ok(ObjGroupsDiff { removed, modified: modified.into_iter().map(|(name, diff)| ObjGroupModified { name, diff }).collect(), added: added.into_iter().map(|(index, group)| ObjGroupAdded { index, group }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_objects_diff_bin(d: &ObjObjectsDiff, out: &mut Vec<u8>) {
    let modified: Vec<(String, ObjGroupDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, ObjObject)> = d.added.iter().map(|a| (a.index, a.object.clone())).collect();
    enc_named_triple_bin(&d.removed, &modified, &added, out, enc_group_diff_bin, enc_object_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_objects_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjObjectsDiff, String> {
    let (removed, modified, added) = dec_named_triple_bin(reader, dec_group_diff_bin, dec_object_bin)?;
    Ok(ObjObjectsDiff { removed, modified: modified.into_iter().map(|(name, diff)| ObjGroupModified { name, diff }).collect(), added: added.into_iter().map(|(index, object)| ObjObjectAdded { index, object }).collect() })
}

pub(crate) fn write_u64_bin(output:&mut Vec<u8>,value:u64){store::pack_rt::write_varint_u64(output,value)}

pub(crate) fn read_u64_bin(reader:&mut store::ByteReader<'_>)->Result<u64,String>{reader.read_varint_u64().map_err(|error|error.to_string())}

impl protocol::DiffBinary for ObjDiff {
/// 🧪️ P2-FG1: REAL binary frame (`format u8 | flags_lo u8 | flags_hi u8 | per-present-field
/// payload`), matching `../💾️binary/📡️.protocol.semio`'s `header fixed 3` + `chain
/// payload bytes` shape — upgraded from F6's `print_diff().into_bytes()` text-as-binary
/// shortcut (the FG1 fixup wave's own finding: `obj` was one of 4 stdio standards left on that
/// shortcut this same wave, despite md/xml/dxf's equally-recursive/flat types already proving
/// the real upgrade achievable). `ObjDiff` has TEN independently optional top-level fields
/// (`vertices`/`texcoords`/`normals`/`faces`/`groups`/`objects`/`mtllib`/`usemtl`/
/// `smoothing_groups`/`unknown_statements`) — one more bit than a single `u8` flags byte can
/// hold, so two flags bytes carry the presence mask (`flags_lo` bits 0-7 = vertices..usemtl,
/// `flags_hi` bit 0 = smoothing_groups, bit 1 = unknown_statements) — same bitmask-over-`u8`
/// device dxf's own `DxfDiff` (4 fields, one `flags` byte) upgrade introduced, just needing a
/// second byte here. Every PRESENT field's own real, field-by-field binary payload follows
/// (`#region 🔖️CollectionBinaryCodecs`/`#region 🔖️BinaryPrimitives` above) — `obj`'s whole diff
/// tree is flat structs/`Vec`/`Option<T>` (module doc comment), so unlike md/dxf's recursive
/// node types, NOTHING here falls back to an opaque byte-chain at the value layer; the ONLY
/// thing still described as an opaque `chain` in the sibling `.protocol.semio` file is the
/// collection-triple SHAPE itself (`Prim::Ref` cannot express a `Vec<Modified{index,diff}>`
/// record-array in the protocol grammar — the same wall every collection-triple diff in this
/// wave hit, documented in that file), never any individual scalar/struct value.
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut flags_lo: u8 = 0;
    let mut flags_hi: u8 = 0;
    if self.vertices.is_some() {
        flags_lo |= 0b0000_0001;
    }
    if self.texcoords.is_some() {
        flags_lo |= 0b0000_0010;
    }
    if self.normals.is_some() {
        flags_lo |= 0b0000_0100;
    }
    if self.faces.is_some() {
        flags_lo |= 0b0000_1000;
    }
    if self.groups.is_some() {
        flags_lo |= 0b0001_0000;
    }
    if self.objects.is_some() {
        flags_lo |= 0b0010_0000;
    }
    if self.mtllib.is_some() {
        flags_lo |= 0b0100_0000;
    }
    if self.usemtl.is_some() {
        flags_lo |= 0b1000_0000;
    }
    if self.smoothing_groups.is_some() {
        flags_hi |= 0b0000_0001;
    }
    if self.unknown_statements.is_some() {
        flags_hi |= 0b0000_0010;
    }
    let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags_lo, flags_hi];
    if let Some(v) = &self.vertices {
        enc_vertices_diff_bin(v, &mut out);
    }
    if let Some(v) = &self.texcoords {
        enc_texcoords_diff_bin(v, &mut out);
    }
    if let Some(v) = &self.normals {
        enc_normals_diff_bin(v, &mut out);
    }
    if let Some(v) = &self.faces {
        enc_faces_diff_bin(v, &mut out);
    }
    if let Some(v) = &self.groups {
        enc_groups_diff_bin(v, &mut out);
    }
    if let Some(v) = &self.objects {
        enc_objects_diff_bin(v, &mut out);
    }
    if let Some(v) = &self.mtllib {
        write_option_bin(&mut out, v, |s, o| write_str_bin(o, s));
    }
    if let Some(v) = &self.usemtl {
        write_vec_bin(&mut out, v, enc_usemtl_bin);
    }
    if let Some(v) = &self.smoothing_groups {
        write_vec_bin(&mut out, v, enc_smoothing_bin);
    }
    if let Some(v) = &self.unknown_statements {
        write_vec_bin(&mut out, v, enc_unknown_bin);
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
    let flags_lo = reader.read_u8().map_err(|e| malformed("diff flags_lo", 1, e.to_string()))?;
    let flags_hi = reader.read_u8().map_err(|e| malformed("diff flags_hi", 2, e.to_string()))?;
    let vertices = if flags_lo & 0b0000_0001 != 0 { Some(dec_vertices_diff_bin(&mut reader).map_err(|e| malformed("diff vertices", reader.position(), e))?) } else { None };
    let texcoords = if flags_lo & 0b0000_0010 != 0 { Some(dec_texcoords_diff_bin(&mut reader).map_err(|e| malformed("diff texcoords", reader.position(), e))?) } else { None };
    let normals = if flags_lo & 0b0000_0100 != 0 { Some(dec_normals_diff_bin(&mut reader).map_err(|e| malformed("diff normals", reader.position(), e))?) } else { None };
    let faces = if flags_lo & 0b0000_1000 != 0 { Some(dec_faces_diff_bin(&mut reader).map_err(|e| malformed("diff faces", reader.position(), e))?) } else { None };
    let groups = if flags_lo & 0b0001_0000 != 0 { Some(dec_groups_diff_bin(&mut reader).map_err(|e| malformed("diff groups", reader.position(), e))?) } else { None };
    let objects = if flags_lo & 0b0010_0000 != 0 { Some(dec_objects_diff_bin(&mut reader).map_err(|e| malformed("diff objects", reader.position(), e))?) } else { None };
    let mtllib = if flags_lo & 0b0100_0000 != 0 { Some(read_option_bin(&mut reader, read_str_bin).map_err(|e| malformed("diff mtllib", reader.position(), e))?) } else { None };
    let usemtl = if flags_lo & 0b1000_0000 != 0 { Some(read_vec_bin(&mut reader, dec_usemtl_bin).map_err(|e| malformed("diff usemtl", reader.position(), e))?) } else { None };
    let smoothing_groups = if flags_hi & 0b0000_0001 != 0 { Some(read_vec_bin(&mut reader, dec_smoothing_bin).map_err(|e| malformed("diff smoothing_groups", reader.position(), e))?) } else { None };
    let unknown_statements = if flags_hi & 0b0000_0010 != 0 { Some(read_vec_bin(&mut reader, dec_unknown_bin).map_err(|e| malformed("diff unknown_statements", reader.position(), e))?) } else { None };
    Ok(ObjDiff { vertices, texcoords, normals, faces, groups, objects, mtllib, usemtl, smoothing_groups, unknown_statements })
}
}
}
pub use diff_codec::*;
