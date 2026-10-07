//! 💾️ Binary representation codec surface for `stdio.semio.model` (snapshot).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::model::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion, SemioTransform};
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;

/// 🧪️ P2 pilot (model): real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::
/// write_varint_u64` / `store::ByteReader`, same helpers `stdio.semio.flow`'s upgraded
/// `ArtifactPack` reuses) backing the real `ArtifactPack` below — replaces the old
/// `serde_json::to_vec`-in-envelope shortcut entirely.
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
pub(crate) fn write_option_str(out: &mut Vec<u8>, opt: &Option<String>) {
    match opt {
        None => out.push(0),
        Some(v) => {
            out.push(1);
            write_str_lp(out, v);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_option_str(reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(read_str_lp(reader)?)),
        other => Err(format!("option<str>: unknown presence tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_transform(out: &mut Vec<u8>, t: &SemioTransform) {
    out.extend_from_slice(&t.translation.x.to_le_bytes());
    out.extend_from_slice(&t.translation.y.to_le_bytes());
    out.extend_from_slice(&t.translation.z.to_le_bytes());
    out.extend_from_slice(&t.rotation.x.to_le_bytes());
    out.extend_from_slice(&t.rotation.y.to_le_bytes());
    out.extend_from_slice(&t.rotation.z.to_le_bytes());
    out.extend_from_slice(&t.rotation.w.to_le_bytes());
    out.extend_from_slice(&t.scale.x.to_le_bytes());
    out.extend_from_slice(&t.scale.y.to_le_bytes());
    out.extend_from_slice(&t.scale.z.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_transform(reader: &mut store::ByteReader<'_>) -> Result<SemioTransform, String> {
    let mut next = || reader.read_f64_le().map_err(|e| e.to_string());
    let translation = SemioPoint3 { x: next()?, y: next()?, z: next()? };
    let rotation = SemioQuaternion { x: next()?, y: next()?, z: next()?, w: next()? };
    let scale = SemioPoint3 { x: next()?, y: next()?, z: next()? };
    Ok(SemioTransform { translation, rotation, scale })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_spatial_kind(out: &mut Vec<u8>, k: &SpatialKind) {
    out.push(match k {
        SpatialKind::Site => 0,
        SpatialKind::Building => 1,
        SpatialKind::Storey => 2,
        SpatialKind::Space => 3,
    });
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_spatial_kind(reader: &mut store::ByteReader<'_>) -> Result<SpatialKind, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(SpatialKind::Site),
        1 => Ok(SpatialKind::Building),
        2 => Ok(SpatialKind::Storey),
        3 => Ok(SpatialKind::Space),
        other => Err(format!("spatial kind: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_element_class(out: &mut Vec<u8>, c: &ElementClass) {
    match c {
        ElementClass::Wall => out.push(0),
        ElementClass::Slab => out.push(1),
        ElementClass::Column => out.push(2),
        ElementClass::Beam => out.push(3),
        ElementClass::Door => out.push(4),
        ElementClass::Window => out.push(5),
        ElementClass::Roof => out.push(6),
        ElementClass::Stair => out.push(7),
        ElementClass::Furniture => out.push(8),
        ElementClass::Other { name } => {
            out.push(9);
            write_str_lp(out, name);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_element_class(reader: &mut store::ByteReader<'_>) -> Result<ElementClass, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(ElementClass::Wall),
        1 => Ok(ElementClass::Slab),
        2 => Ok(ElementClass::Column),
        3 => Ok(ElementClass::Beam),
        4 => Ok(ElementClass::Door),
        5 => Ok(ElementClass::Window),
        6 => Ok(ElementClass::Roof),
        7 => Ok(ElementClass::Stair),
        8 => Ok(ElementClass::Furniture),
        9 => Ok(ElementClass::Other { name: read_str_lp(reader)? }),
        other => Err(format!("element class: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_geometry_ref(out: &mut Vec<u8>, g: &GeometryRef) {
    match g {
        GeometryRef::None => out.push(0),
        GeometryRef::Brep { brep_id } => {
            out.push(1);
            write_str_lp(out, brep_id);
        }
        GeometryRef::Mesh { mesh_id } => {
            out.push(2);
            write_str_lp(out, mesh_id);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_geometry_ref(reader: &mut store::ByteReader<'_>) -> Result<GeometryRef, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(GeometryRef::None),
        1 => Ok(GeometryRef::Brep { brep_id: read_str_lp(reader)? }),
        2 => Ok(GeometryRef::Mesh { mesh_id: read_str_lp(reader)? }),
        other => Err(format!("geometry ref: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_pset_value(out: &mut Vec<u8>, v: &PsetValue) {
    match v {
        PsetValue::Text { value } => {
            out.push(0);
            write_str_lp(out, value);
        }
        PsetValue::Number { value } => {
            out.push(1);
            out.extend_from_slice(&value.to_le_bytes());
        }
        PsetValue::Boolean { value } => {
            out.push(2);
            out.push(if *value { 1 } else { 0 });
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_pset_value(reader: &mut store::ByteReader<'_>) -> Result<PsetValue, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(PsetValue::Text { value: read_str_lp(reader)? }),
        1 => Ok(PsetValue::Number { value: reader.read_f64_le().map_err(|e| e.to_string())? }),
        2 => Ok(PsetValue::Boolean { value: reader.read_u8().map_err(|e| e.to_string())? != 0 }),
        other => Err(format!("pset value: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_property(out: &mut Vec<u8>, p: &Property) {
    write_str_lp(out, &p.key);
    write_pset_value(out, &p.value);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_property(reader: &mut store::ByteReader<'_>) -> Result<Property, String> {
    let key = read_str_lp(reader)?;
    let value = read_pset_value(reader)?;
    Ok(Property { key, value })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_property_set(out: &mut Vec<u8>, ps: &PropertySet) {
    write_str_lp(out, &ps.name);
    store::pack_rt::write_varint_u64(out, ps.properties.len() as u64);
    for p in &ps.properties {
        write_property(out, p);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_property_set(reader: &mut store::ByteReader<'_>) -> Result<PropertySet, String> {
    let name = read_str_lp(reader)?;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut properties = Vec::with_capacity(count as usize);
    for _ in 0..count {
        properties.push(read_property(reader)?);
    }
    Ok(PropertySet { name, properties })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_spatial_node(out: &mut Vec<u8>, n: &SpatialNode) {
    write_str_lp(out, &n.id);
    write_spatial_kind(out, &n.kind);
    write_str_lp(out, &n.name);
    write_option_str(out, &n.parent_id);
    write_transform(out, &n.placement);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_spatial_node(reader: &mut store::ByteReader<'_>) -> Result<SpatialNode, String> {
    let id = read_str_lp(reader)?;
    let kind = read_spatial_kind(reader)?;
    let name = read_str_lp(reader)?;
    let parent_id = read_option_str(reader)?;
    let placement = read_transform(reader)?;
    Ok(SpatialNode { id, kind, name, parent_id, placement })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_element(out: &mut Vec<u8>, e: &SemioModelElement) {
    write_str_lp(out, &e.id);
    write_element_class(out, &e.class);
    write_transform(out, &e.placement);
    write_geometry_ref(out, &e.geometry);
    write_option_str(out, &e.spatial_id);
    store::pack_rt::write_varint_u64(out, e.psets.len() as u64);
    for ps in &e.psets {
        write_property_set(out, ps);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_element(reader: &mut store::ByteReader<'_>) -> Result<SemioModelElement, String> {
    let id = read_str_lp(reader)?;
    let class = read_element_class(reader)?;
    let placement = read_transform(reader)?;
    let geometry = read_geometry_ref(reader)?;
    let spatial_id = read_option_str(reader)?;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut psets = Vec::with_capacity(count as usize);
    for _ in 0..count {
        psets.push(read_property_set(reader)?);
    }
    Ok(SemioModelElement { id, class, placement, geometry, spatial_id, psets })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_relation_kind(out: &mut Vec<u8>, k: &RelationKind) {
    match k {
        RelationKind::Aggregates => out.push(0),
        RelationKind::ContainedIn => out.push(1),
        RelationKind::ConnectsTo => out.push(2),
        RelationKind::FillsVoid => out.push(3),
        RelationKind::VoidsElement => out.push(4),
        RelationKind::Other { label } => {
            out.push(5);
            write_str_lp(out, label);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_relation_kind(reader: &mut store::ByteReader<'_>) -> Result<RelationKind, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(RelationKind::Aggregates),
        1 => Ok(RelationKind::ContainedIn),
        2 => Ok(RelationKind::ConnectsTo),
        3 => Ok(RelationKind::FillsVoid),
        4 => Ok(RelationKind::VoidsElement),
        5 => Ok(RelationKind::Other { label: read_str_lp(reader)? }),
        other => Err(format!("relation kind: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_relation(out: &mut Vec<u8>, r: &ModelRelation) {
    write_str_lp(out, &r.id);
    write_relation_kind(out, &r.kind);
    write_str_lp(out, &r.from);
    write_str_lp(out, &r.to);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_relation(reader: &mut store::ByteReader<'_>) -> Result<ModelRelation, String> {
    let id = read_str_lp(reader)?;
    let kind = read_relation_kind(reader)?;
    let from = read_str_lp(reader)?;
    let to = read_str_lp(reader)?;
    Ok(ModelRelation { id, kind, from, to })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_model_snapshot_binary(s: &SemioModelSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.spatial.len() as u64);
    for n in &s.spatial {
        write_spatial_node(&mut out, n);
    }
    store::pack_rt::write_varint_u64(&mut out, s.elements.len() as u64);
    for e in &s.elements {
        write_element(&mut out, e);
    }
    store::pack_rt::write_varint_u64(&mut out, s.relations.len() as u64);
    for r in &s.relations {
        write_relation(&mut out, r);
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_model_snapshot_binary(bytes: &[u8]) -> Result<SemioModelSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let spatial_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut spatial = Vec::with_capacity(spatial_count as usize);
    for _ in 0..spatial_count {
        spatial.push(read_spatial_node(&mut reader)?);
    }
    let element_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut elements = Vec::with_capacity(element_count as usize);
    for _ in 0..element_count {
        elements.push(read_element(&mut reader)?);
    }
    let relation_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut relations = Vec::with_capacity(relation_count as usize);
    for _ in 0..relation_count {
        relations.push(read_relation(&mut reader)?);
    }
    Ok(SemioModelSnapshot { schema, spatial, elements, relations })
}

impl store::ArtifactPack for SemioModelSnapshot {
    /// 🪶️ Publishes the owned typed relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_model_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        decode_model_snapshot_binary(&inner).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::model::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion, SemioTransform};
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::model::io::text::snapshot::*;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_model_pack(snapshot: &SemioModelSnapshot) -> Vec<u8> {
    <SemioModelSnapshot as store::ArtifactPack>::encode_pack(snapshot)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_model_pack(bytes: &[u8]) -> Result<SemioModelSnapshot, String> {
    <SemioModelSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| e.to_string())
}
}
pub use native_snapshot_codec::*;
