//! 💾️ Binary codec primitives of the semio family: varint length-prefixed strings and bytes, and the geometry records
//! as little-endian floats.

use crate::schema::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform};

/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers `stdio.semio.flow`'s/`stdio.semio.brep`'s upgraded
/// `ArtifactPack` reuse) backing the real `ArtifactPack` below — replaces the old
/// `serde_json::to_vec`-in-envelope shortcut.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_point2(out: &mut Vec<u8>, p: &SemioPoint2) {
    out.extend_from_slice(&p.x.to_le_bytes());
    out.extend_from_slice(&p.y.to_le_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_point2(reader: &mut store::ByteReader<'_>) -> Result<SemioPoint2, String> {
    Ok(SemioPoint2 { x: reader.read_f64_le().map_err(|e| e.to_string())?, y: reader.read_f64_le().map_err(|e| e.to_string())? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_point3(out: &mut Vec<u8>, p: &SemioPoint3) {
    out.extend_from_slice(&p.x.to_le_bytes());
    out.extend_from_slice(&p.y.to_le_bytes());
    out.extend_from_slice(&p.z.to_le_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_point3(reader: &mut store::ByteReader<'_>) -> Result<SemioPoint3, String> {
    Ok(SemioPoint3 { x: reader.read_f64_le().map_err(|e| e.to_string())?, y: reader.read_f64_le().map_err(|e| e.to_string())?, z: reader.read_f64_le().map_err(|e| e.to_string())? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_quaternion(out: &mut Vec<u8>, q: &SemioQuaternion) {
    out.extend_from_slice(&q.x.to_le_bytes());
    out.extend_from_slice(&q.y.to_le_bytes());
    out.extend_from_slice(&q.z.to_le_bytes());
    out.extend_from_slice(&q.w.to_le_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_quaternion(reader: &mut store::ByteReader<'_>) -> Result<SemioQuaternion, String> {
    Ok(SemioQuaternion { x: reader.read_f64_le().map_err(|e| e.to_string())?, y: reader.read_f64_le().map_err(|e| e.to_string())?, z: reader.read_f64_le().map_err(|e| e.to_string())?, w: reader.read_f64_le().map_err(|e| e.to_string())? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_transform(out: &mut Vec<u8>, t: &SemioTransform) {
    write_point3(out, &t.translation);
    write_quaternion(out, &t.rotation);
    write_point3(out, &t.scale);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_transform(reader: &mut store::ByteReader<'_>) -> Result<SemioTransform, String> {
    Ok(SemioTransform { translation: read_point3(reader)?, rotation: read_quaternion(reader)?, scale: read_point3(reader)? })
}
/// 🩹️ `store::ByteReader` has no native `f32` reader (only `f64_le`) — read 4 raw bytes instead.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_f32_le(reader: &mut store::ByteReader<'_>) -> Result<f32, String> {
    let bytes = reader.read_bytes(4).map_err(|e| e.to_string())?;
    let arr: [u8; 4] = bytes.try_into().map_err(|_| "f32 read: truncated".to_string())?;
    Ok(f32::from_le_bytes(arr))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_rgba(out: &mut Vec<u8>, c: &SemioRgba) {
    out.extend_from_slice(&c.r.to_le_bytes());
    out.extend_from_slice(&c.g.to_le_bytes());
    out.extend_from_slice(&c.b.to_le_bytes());
    out.extend_from_slice(&c.a.to_le_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_rgba(reader: &mut store::ByteReader<'_>) -> Result<SemioRgba, String> {
    Ok(SemioRgba { r: read_f32_le(reader)?, g: read_f32_le(reader)?, b: read_f32_le(reader)?, a: read_f32_le(reader)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bool(out: &mut Vec<u8>, b: bool) {
    out.push(if b { 1 } else { 0 });
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bool(reader: &mut store::ByteReader<'_>) -> Result<bool, String> {
    Ok(reader.read_u8().map_err(|e| e.to_string())? != 0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_ref(out: &mut Vec<u8>, r: &semio_framework_artifact_reference::ArtifactRef) {
    for field in [&r.artifact_id,&r.dialect.artifact_kind,&r.dialect.standard,&r.dialect.subset]{write_str_lp(out,field);}
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<semio_framework_artifact_reference::ArtifactRef, String> {
    Ok(semio_framework_artifact_reference::ArtifactRef{artifact_id:read_str_lp(reader)?,dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:read_str_lp(reader)?,standard:read_str_lp(reader)?,subset:read_str_lp(reader)?}})
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_child<S>(out: &mut Vec<u8>, c: &store::os_store::artifact_child::ArtifactChild<S>) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_child<S>(reader: &mut store::ByteReader<'_>) -> Result<store::os_store::artifact_child::ArtifactChild<S>, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::os_store::artifact_child::ArtifactChild::new(child_id, target))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_child_opt<S>(out: &mut Vec<u8>, c: &Option<store::os_store::artifact_child::ArtifactChild<S>>) {
    match c {
        Some(c) => {
            out.push(1);
            write_child(out, c);
        }
        None => out.push(0),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_child_opt<S>(reader: &mut store::ByteReader<'_>) -> Result<Option<store::os_store::artifact_child::ArtifactChild<S>>, String> {
    let presence = reader.read_u8().map_err(|e| e.to_string())?;
    if presence == 0 {
        Ok(None)
    } else {
        Ok(Some(read_child(reader)?))
    }
}
