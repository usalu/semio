//! 💾️ Binary representation codec surface for `s.stdio.semio.mesh` (snapshot) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::mesh::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioRgba, SemioUv};
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;

/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers `🌊️flow`'s own upgraded `ArtifactPack` uses) backing the
/// real `ArtifactPack` below — replaces the old `serde_json::to_vec`-in-envelope shortcut.
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_f32_le(reader: &mut store::ByteReader<'_>) -> Result<f32, String> {
    let bytes = reader.read_bytes(4).map_err(|e| e.to_string())?;
    let arr: [u8; 4] = bytes.try_into().map_err(|_| "f32 read: truncated".to_string())?;
    Ok(f32::from_le_bytes(arr))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_point3_list(out: &mut Vec<u8>, items: &[SemioPoint3]) {
    store::pack_rt::write_varint_u64(out, items.len() as u64);
    for p in items {
        out.extend_from_slice(&p.x.to_le_bytes());
        out.extend_from_slice(&p.y.to_le_bytes());
        out.extend_from_slice(&p.z.to_le_bytes());
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_point3_list(reader: &mut store::ByteReader<'_>) -> Result<Vec<SemioPoint3>, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let x = reader.read_f64_le().map_err(|e| e.to_string())?;
        let y = reader.read_f64_le().map_err(|e| e.to_string())?;
        let z = reader.read_f64_le().map_err(|e| e.to_string())?;
        out.push(SemioPoint3 { x, y, z });
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_uv_list(out: &mut Vec<u8>, items: &[SemioUv]) {
    store::pack_rt::write_varint_u64(out, items.len() as u64);
    for v in items {
        out.extend_from_slice(&v.u.to_le_bytes());
        out.extend_from_slice(&v.v.to_le_bytes());
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_uv_list(reader: &mut store::ByteReader<'_>) -> Result<Vec<SemioUv>, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let u = reader.read_f64_le().map_err(|e| e.to_string())?;
        let v = reader.read_f64_le().map_err(|e| e.to_string())?;
        out.push(SemioUv { u, v });
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_rgba(out: &mut Vec<u8>, c: &SemioRgba) {
    out.extend_from_slice(&c.r.to_le_bytes());
    out.extend_from_slice(&c.g.to_le_bytes());
    out.extend_from_slice(&c.b.to_le_bytes());
    out.extend_from_slice(&c.a.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_rgba(reader: &mut store::ByteReader<'_>) -> Result<SemioRgba, String> {
    Ok(SemioRgba { r: read_f32_le(reader)?, g: read_f32_le(reader)?, b: read_f32_le(reader)?, a: read_f32_le(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_rgba_list(out: &mut Vec<u8>, items: &[SemioRgba]) {
    store::pack_rt::write_varint_u64(out, items.len() as u64);
    for c in items {
        write_rgba(out, c);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_rgba_list(reader: &mut store::ByteReader<'_>) -> Result<Vec<SemioRgba>, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(n as usize);
    for _ in 0..n {
        out.push(read_rgba(reader)?);
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_mesh_snapshot_binary(s: &SemioMeshSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.meshes.len() as u64);
    for m in &s.meshes {
        write_str_lp(&mut out, &m.id);
        store::pack_rt::write_varint_u64(&mut out, m.primitives.len() as u64);
        for p in &m.primitives {
            write_str_lp(&mut out, &p.id);
            out.push(match p.topology {
                SemioTopology::Points => 0u8,
                SemioTopology::Lines => 1,
                SemioTopology::LineStrip => 2,
                SemioTopology::Triangles => 3,
                SemioTopology::TriangleStrip => 4,
                SemioTopology::TriangleFan => 5,
            });
            write_point3_list(&mut out, &p.positions);
            write_point3_list(&mut out, &p.normals);
            write_uv_list(&mut out, &p.uvs);
            write_rgba_list(&mut out, &p.colors);
            store::pack_rt::write_varint_u64(&mut out, p.indices.len() as u64);
            for idx in &p.indices {
                out.extend_from_slice(&idx.to_le_bytes());
            }
            match &p.material_id {
                Some(v) => {
                    out.push(1);
                    write_str_lp(&mut out, v);
                }
                None => out.push(0),
            }
        }
    }
    store::pack_rt::write_varint_u64(&mut out, s.materials.len() as u64);
    for mat in &s.materials {
        write_str_lp(&mut out, &mat.id);
        write_rgba(&mut out, &mat.base_color);
        out.extend_from_slice(&mat.metallic.to_le_bytes());
        out.extend_from_slice(&mat.roughness.to_le_bytes());
        for reference in [&mat.base_color_texture, &mat.metallic_roughness_texture, &mat.normal_texture, &mat.occlusion_texture, &mat.emissive_texture] {
            match reference { None => out.push(0), Some(id) => { out.push(1); write_str_lp(&mut out, id); } }
        }
    }
    store::pack_rt::write_varint_u64(&mut out, s.textures.len() as u64);
    for t in &s.textures {
        write_str_lp(&mut out, &t.id);
        write_str_lp(&mut out, &t.mime);
        write_bytes_lp(&mut out, &t.bytes);
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_mesh_snapshot_binary(bytes: &[u8]) -> Result<SemioMeshSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let mesh_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut meshes = Vec::with_capacity(mesh_count as usize);
    for _ in 0..mesh_count {
        let id = read_str_lp(&mut reader)?;
        let primitive_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
        let mut primitives = Vec::with_capacity(primitive_count as usize);
        for _ in 0..primitive_count {
            let pid = read_str_lp(&mut reader)?;
            let topology_tag = reader.read_u8().map_err(|e| e.to_string())?;
            let topology = match topology_tag {
                0 => SemioTopology::Points,
                1 => SemioTopology::Lines,
                2 => SemioTopology::LineStrip,
                3 => SemioTopology::Triangles,
                4 => SemioTopology::TriangleStrip,
                5 => SemioTopology::TriangleFan,
                other => return Err(format!("unsupported topology tag {other}")),
            };
            let positions = read_point3_list(&mut reader)?;
            let normals = read_point3_list(&mut reader)?;
            let uvs = read_uv_list(&mut reader)?;
            let colors = read_rgba_list(&mut reader)?;
            let index_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut indices = Vec::with_capacity(index_count as usize);
            for _ in 0..index_count {
                indices.push(reader.read_u32_le().map_err(|e| e.to_string())?);
            }
            let material_id = match reader.read_u8().map_err(|e| e.to_string())? {
                0 => None,
                1 => Some(read_str_lp(&mut reader)?),
                other => return Err(format!("unsupported material_id tag {other}")),
            };
            primitives.push(SemioPrimitive { id: pid, topology, positions, normals, uvs, colors, indices, material_id });
        }
        meshes.push(SemioMesh { id, primitives });
    }
    let material_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut materials = Vec::with_capacity(material_count as usize);
    for _ in 0..material_count {
        let id = read_str_lp(&mut reader)?;
        let base_color = read_rgba(&mut reader)?;
        let metallic = read_f32_le(&mut reader)?;
        let roughness = read_f32_le(&mut reader)?;
        let base_color_texture = match reader.read_u8().map_err(|e| e.to_string())? { 0 => None, 1 => Some(read_str_lp(&mut reader)?), _ => return Err("invalid texture reference presence".into()) };
        let metallic_roughness_texture = match reader.read_u8().map_err(|e| e.to_string())? { 0 => None, 1 => Some(read_str_lp(&mut reader)?), _ => return Err("invalid texture reference presence".into()) };
        let normal_texture = match reader.read_u8().map_err(|e| e.to_string())? { 0 => None, 1 => Some(read_str_lp(&mut reader)?), _ => return Err("invalid texture reference presence".into()) };
        let occlusion_texture = match reader.read_u8().map_err(|e| e.to_string())? { 0 => None, 1 => Some(read_str_lp(&mut reader)?), _ => return Err("invalid texture reference presence".into()) };
        let emissive_texture = match reader.read_u8().map_err(|e| e.to_string())? { 0 => None, 1 => Some(read_str_lp(&mut reader)?), _ => return Err("invalid texture reference presence".into()) };
        materials.push(SemioMaterial { id, base_color, metallic, roughness, base_color_texture, metallic_roughness_texture, normal_texture, occlusion_texture, emissive_texture });
    }
    let texture_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut textures = Vec::with_capacity(texture_count as usize);
    for _ in 0..texture_count {
        let id = read_str_lp(&mut reader)?;
        let mime = read_str_lp(&mut reader)?;
        let bytes = read_bytes_lp(&mut reader)?;
        textures.push(SemioTexture { id, mime, bytes });
    }
    Ok(SemioMeshSnapshot { schema, meshes, materials, textures })
}

impl store::ArtifactPack for SemioMeshSnapshot {
    /// 🪶️ Publishes the owned typed relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_mesh_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        decode_mesh_snapshot_binary(&inner).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::mesh::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioRgba, SemioUv};
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::mesh::io::text::snapshot::*;
/// 📦 Encode a `SemioMeshSnapshot` as a semio pack envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_mesh_pack(snapshot: &SemioMeshSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
/// 📦 Decode a semio pack envelope into a `SemioMeshSnapshot`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_mesh_pack(bytes: &[u8]) -> Result<SemioMeshSnapshot, store::PackError> {
    <SemioMeshSnapshot as store::ArtifactPack>::decode_pack(bytes)
}
}
pub use native_snapshot_codec::*;
