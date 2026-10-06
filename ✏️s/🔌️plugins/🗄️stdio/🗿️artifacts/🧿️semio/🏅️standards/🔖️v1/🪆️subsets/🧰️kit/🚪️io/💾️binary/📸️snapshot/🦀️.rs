//! 💾️ Binary representation codec surface for `s.stdio.semio.kit` (snapshot) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::kit::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;
#[cfg(test)]
use serde::{Deserialize, Serialize};
use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};

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
pub(crate) fn write_child_list<S>(out: &mut Vec<u8>, list: &[store::ArtifactChild<S>]) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for c in list {
        write_child(out, c);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_child_list<S>(reader: &mut store::ByteReader<'_>) -> Result<Vec<store::ArtifactChild<S>>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| read_child(reader)).collect()
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
pub(crate) fn write_pin(out: &mut Vec<u8>, p: &store::LinkPin) {
    match p {
        store::LinkPin::Head => out.push(0),
        store::LinkPin::Checkpoint { id } => {
            out.push(1);
            write_str_lp(out, id);
        }
        store::LinkPin::Snapshot { blob } => {
            out.push(2);
            write_str_lp(out, &blob.hash);
            store::pack_rt::write_varint_u64(out, blob.size);
            write_str_lp(out, &blob.media_type);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_pin(reader: &mut store::ByteReader<'_>) -> Result<store::LinkPin, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(store::LinkPin::Head),
        1 => Ok(store::LinkPin::Checkpoint { id: read_str_lp(reader)? }),
        2 => {
            let hash = read_str_lp(reader)?;
            let size = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let media_type = read_str_lp(reader)?;
            Ok(store::LinkPin::Snapshot { blob: store::BlobRef { hash, size, media_type } })
        }
        other => Err(format!("unsupported link pin tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_link(out: &mut Vec<u8>, l: &store::ArtifactLink) {
    write_ref(out, &l.target);
    write_pin(out, &l.pin);
    write_str_lp(out, &l.role);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_link(reader: &mut store::ByteReader<'_>) -> Result<store::ArtifactLink, String> {
    let target = read_ref(reader)?;
    let pin = read_pin(reader)?;
    let role = read_str_lp(reader)?;
    Ok(store::ArtifactLink { target, pin, role })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_link_list(out: &mut Vec<u8>, list: &[store::ArtifactLink]) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for l in list {
        write_link(out, l);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_link_list(reader: &mut store::ByteReader<'_>) -> Result<Vec<store::ArtifactLink>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| read_link(reader)).collect()
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
pub(crate) fn write_type(out: &mut Vec<u8>, t: &SemioKitType) {
    write_str_lp(out, &t.id);
    write_str_lp(out, &t.name);
    write_str_lp(out, &t.category);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_type(reader: &mut store::ByteReader<'_>) -> Result<SemioKitType, String> {
    Ok(SemioKitType { id: read_str_lp(reader)?, name: read_str_lp(reader)?, category: read_str_lp(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_type_list(out: &mut Vec<u8>, list: &[SemioKitType]) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for t in list {
        write_type(out, t);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_type_list(reader: &mut store::ByteReader<'_>) -> Result<Vec<SemioKitType>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| read_type(reader)).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_piece(out: &mut Vec<u8>, p: &SemioKitPiece) {
    write_str_lp(out, &p.id);
    write_str_lp(out, &p.type_id);
    write_transform(out, &p.transform);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_piece(reader: &mut store::ByteReader<'_>) -> Result<SemioKitPiece, String> {
    Ok(SemioKitPiece { id: read_str_lp(reader)?, type_id: read_str_lp(reader)?, transform: read_transform(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_connection(out: &mut Vec<u8>, c: &SemioKitConnection) {
    write_str_lp(out, &c.id);
    write_str_lp(out, &c.connecting_piece_id);
    write_str_lp(out, &c.connecting_port);
    write_str_lp(out, &c.connected_piece_id);
    write_str_lp(out, &c.connected_port);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_connection(reader: &mut store::ByteReader<'_>) -> Result<SemioKitConnection, String> {
    Ok(SemioKitConnection { id: read_str_lp(reader)?, connecting_piece_id: read_str_lp(reader)?, connecting_port: read_str_lp(reader)?, connected_piece_id: read_str_lp(reader)?, connected_port: read_str_lp(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_design(out: &mut Vec<u8>, d: &SemioKitDesign) {
    write_str_lp(out, &d.id);
    write_str_lp(out, &d.name);
    store::pack_rt::write_varint_u64(out, d.pieces.len() as u64);
    for p in &d.pieces {
        write_piece(out, p);
    }
    store::pack_rt::write_varint_u64(out, d.connections.len() as u64);
    for c in &d.connections {
        write_connection(out, c);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_design(reader: &mut store::ByteReader<'_>) -> Result<SemioKitDesign, String> {
    let id = read_str_lp(reader)?;
    let name = read_str_lp(reader)?;
    let piece_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let pieces = (0..piece_count).map(|_| read_piece(reader)).collect::<Result<Vec<_>, String>>()?;
    let connection_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let connections = (0..connection_count).map(|_| read_connection(reader)).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioKitDesign { id, name, pieces, connections })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_design_list(out: &mut Vec<u8>, list: &[SemioKitDesign]) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for d in list {
        write_design(out, d);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_design_list(reader: &mut store::ByteReader<'_>) -> Result<Vec<SemioKitDesign>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| read_design(reader)).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_kit_snapshot_binary(s: &SemioKitSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_str_lp(&mut out, &s.schema);
    write_type_list(&mut out, &s.types);
    write_design_list(&mut out, &s.designs);
    write_child_list(&mut out, &s.objects);
    write_child_list(&mut out, &s.models);
    write_child_opt(&mut out, &s.properties);
    write_link_list(&mut out, &s.representations);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_kit_snapshot_binary(bytes: &[u8]) -> Result<SemioKitSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let types = read_type_list(&mut reader)?;
    let designs = read_design_list(&mut reader)?;
    let objects = read_child_list(&mut reader)?;
    let models = read_child_list(&mut reader)?;
    let properties = read_child_opt(&mut reader)?;
    let representations = read_link_list(&mut reader)?;
    Ok(SemioKitSnapshot { schema, types, designs, objects, models, properties, representations })
}

impl store::ArtifactPack for SemioKitSnapshot {
    /// 🪶️ Publishes the owned typed relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_kit_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        decode_kit_snapshot_binary(&inner).map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error)))
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::kit::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;
#[cfg(test)]
use serde::{Deserialize, Serialize};
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::kit::io::text::snapshot::*;
/// 📦️ Encodes a [`SemioKitSnapshot`] as a semio pack envelope — the binary twin of the DSL text, produced by a
/// SEPARATE codec, which is what makes the two committed encodings of one document able to
/// contradict each other.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_kit_pack(snapshot: &SemioKitSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
/// 📦️ Decodes a semio pack envelope into a [`SemioKitSnapshot`] — the inverse of
/// [`encode_semio_kit_pack`], reading `../../🖼️assets/🪑️furniture/🎒️.pack.semio`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_kit_pack(bytes: &[u8]) -> Result<SemioKitSnapshot, String> {
    <SemioKitSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
}
pub use native_snapshot_codec::*;
