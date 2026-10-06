//! 💾️ Binary representation codec surface for `s.stdio.semio.object` (snapshot) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::object::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;

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
pub(crate) fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) {
    for field in [&r.artifact_id,&r.dialect.artifact_kind,&r.dialect.standard,&r.dialect.subset]{write_str_lp(out,field);}
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<store::os_io::ArtifactRef, String> {
    Ok(store::os_io::ArtifactRef{artifact_id:read_str_lp(reader)?,dialect:store::os_io::ArtifactDialect{artifact_kind:read_str_lp(reader)?,standard:read_str_lp(reader)?,subset:read_str_lp(reader)?}})
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_child<S>(out: &mut Vec<u8>, c: &store::ArtifactChild<S>) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_child<S>(reader: &mut store::ByteReader<'_>) -> Result<store::ArtifactChild<S>, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_child_opt<S>(out: &mut Vec<u8>, c: &Option<store::ArtifactChild<S>>) {
    match c {
        Some(c) => {
            out.push(1);
            write_child(out, c);
        }
        None => out.push(0),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_child_opt<S>(reader: &mut store::ByteReader<'_>) -> Result<Option<store::ArtifactChild<S>>, String> {
    let presence = reader.read_u8().map_err(|e| e.to_string())?;
    if presence == 0 {
        Ok(None)
    } else {
        Ok(Some(read_child(reader)?))
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_transform(out: &mut Vec<u8>, t: &SemioTransform) {
    for v in [t.translation.x, t.translation.y, t.translation.z, t.rotation.x, t.rotation.y, t.rotation.z, t.rotation.w, t.scale.x, t.scale.y, t.scale.z] {
        out.extend_from_slice(&v.to_le_bytes());
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_transform(reader: &mut store::ByteReader<'_>) -> Result<SemioTransform, String> {
    use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion};
    let mut next = || -> Result<f64, String> { Ok(f64::from_le_bytes(reader.read_bytes(8).map_err(|e| e.to_string())?.try_into().map_err(|_| "transform: short read".to_string())?)) };
    Ok(SemioTransform { translation: SemioPoint3 { x: next()?, y: next()?, z: next()? }, rotation: SemioQuaternion { x: next()?, y: next()?, z: next()?, w: next()? }, scale: SemioPoint3 { x: next()?, y: next()?, z: next()? } })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_object_snapshot_binary(s: &SemioObjectSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_str_lp(&mut out, &s.schema);
    write_transform(&mut out, &s.transform);
    write_child_opt(&mut out, &s.brep);
    write_child_opt(&mut out, &s.mesh);
    write_child_opt(&mut out, &s.properties);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_object_snapshot_binary(bytes: &[u8]) -> Result<SemioObjectSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let transform = read_transform(&mut reader)?;
    let brep = read_child_opt(&mut reader)?;
    let mesh = read_child_opt(&mut reader)?;
    let properties = read_child_opt(&mut reader)?;
    Ok(SemioObjectSnapshot { schema, transform, brep, mesh, properties })
}

impl store::ArtifactPack for SemioObjectSnapshot {
    /// 🪶️ Publishes the owned typed relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_object_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        decode_object_snapshot_binary(&inner).and_then(|snapshot| { snapshot.validate()?; Ok(snapshot) }).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::object::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::object::io::text::snapshot::*;
/// 📦️ Encodes a [`SemioObjectSnapshot`] as a semio pack envelope — the binary twin of the DSL text, produced by a
/// SEPARATE codec, which is what makes the two committed encodings of one document able to
/// contradict each other.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_object_pack(snapshot: &SemioObjectSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
/// 📦️ Decodes a semio pack envelope into a [`SemioObjectSnapshot`] — the inverse of
/// [`encode_semio_object_pack`], reading `../../🖼️assets/📦️crate/🎒️.pack.semio`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_object_pack(bytes: &[u8]) -> Result<SemioObjectSnapshot, String> {
    <SemioObjectSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
}
pub use native_snapshot_codec::*;
