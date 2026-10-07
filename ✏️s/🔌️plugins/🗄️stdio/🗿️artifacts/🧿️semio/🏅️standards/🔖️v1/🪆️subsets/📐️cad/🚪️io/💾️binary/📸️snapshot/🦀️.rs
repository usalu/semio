//! 💾️ Binary representation grammar surface for `s.stdio.semio.cad` (snapshot).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::cad::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;

/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers `stdio.semio.flow`/`stdio.semio.brep`'s upgraded
/// `OpBinary`/`DiffCodec` reuse) backing the real `ArtifactPack` below — replaces the old
/// `serde_json::to_vec`-in-envelope shortcut.
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
pub(crate) fn write_point2(out: &mut Vec<u8>, p: &SemioPoint2) {
    out.extend_from_slice(&p.x.to_le_bytes());
    out.extend_from_slice(&p.y.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_point2(reader: &mut store::ByteReader<'_>) -> Result<SemioPoint2, String> {
    let x = reader.read_f64_le().map_err(|e| e.to_string())?;
    let y = reader.read_f64_le().map_err(|e| e.to_string())?;
    Ok(SemioPoint2 { x, y })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_point2_vec(out: &mut Vec<u8>, v: &[SemioPoint2]) {
    store::pack_rt::write_varint_u64(out, v.len() as u64);
    for p in v {
        write_point2(out, p);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_point2_vec(reader: &mut store::ByteReader<'_>) -> Result<Vec<SemioPoint2>, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut v = Vec::with_capacity(n as usize);
    for _ in 0..n {
        v.push(read_point2(reader)?);
    }
    Ok(v)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bool(out: &mut Vec<u8>, b: bool) {
    out.push(if b { 1 } else { 0 });
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bool(reader: &mut store::ByteReader<'_>) -> Result<bool, String> {
    Ok(reader.read_u8().map_err(|e| e.to_string())? != 0)
}

/// 🏷️ `CadEntity` variant tags — 0=Line, 1=Arc, 2=Circle, 3=Ellipse, 4=Polyline, 5=Text, 6=Insert,
/// 7=Solid, 8=Dimension (declaration order).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_entity(out: &mut Vec<u8>, e: &CadEntity) {
    match e {
        CadEntity::Line { a, b } => {
            out.push(0);
            write_point2(out, a);
            write_point2(out, b);
        }
        CadEntity::Arc { center, radius, start_angle, end_angle } => {
            out.push(1);
            write_point2(out, center);
            out.extend_from_slice(&radius.to_le_bytes());
            out.extend_from_slice(&start_angle.to_le_bytes());
            out.extend_from_slice(&end_angle.to_le_bytes());
        }
        CadEntity::Circle { center, radius } => {
            out.push(2);
            write_point2(out, center);
            out.extend_from_slice(&radius.to_le_bytes());
        }
        CadEntity::Ellipse { center, major_axis_end, ratio, start_param, end_param } => {
            out.push(3);
            write_point2(out, center);
            write_point2(out, major_axis_end);
            out.extend_from_slice(&ratio.to_le_bytes());
            out.extend_from_slice(&start_param.to_le_bytes());
            out.extend_from_slice(&end_param.to_le_bytes());
        }
        CadEntity::Polyline { vertices, closed } => {
            out.push(4);
            write_point2_vec(out, vertices);
            write_bool(out, *closed);
        }
        CadEntity::Text { position, height, rotation, content } => {
            out.push(5);
            write_point2(out, position);
            out.extend_from_slice(&height.to_le_bytes());
            out.extend_from_slice(&rotation.to_le_bytes());
            write_str_lp(out, content);
        }
        CadEntity::Insert { block_name, insertion_point, scale, rotation } => {
            out.push(6);
            write_str_lp(out, block_name);
            write_point2(out, insertion_point);
            write_point2(out, scale);
            out.extend_from_slice(&rotation.to_le_bytes());
        }
        CadEntity::Solid { p1, p2, p3, p4 } => {
            out.push(7);
            write_point2(out, p1);
            write_point2(out, p2);
            write_point2(out, p3);
            write_point2(out, p4);
        }
        CadEntity::Dimension { def_point, text_position, measurement, text } => {
            out.push(8);
            write_point2(out, def_point);
            write_point2(out, text_position);
            out.extend_from_slice(&measurement.to_le_bytes());
            write_str_lp(out, text);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_entity(reader: &mut store::ByteReader<'_>) -> Result<CadEntity, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(CadEntity::Line { a: read_point2(reader)?, b: read_point2(reader)? }),
        1 => Ok(CadEntity::Arc { center: read_point2(reader)?, radius: reader.read_f64_le().map_err(|e| e.to_string())?, start_angle: reader.read_f64_le().map_err(|e| e.to_string())?, end_angle: reader.read_f64_le().map_err(|e| e.to_string())? }),
        2 => Ok(CadEntity::Circle { center: read_point2(reader)?, radius: reader.read_f64_le().map_err(|e| e.to_string())? }),
        3 => Ok(CadEntity::Ellipse {
            center: read_point2(reader)?,
            major_axis_end: read_point2(reader)?,
            ratio: reader.read_f64_le().map_err(|e| e.to_string())?,
            start_param: reader.read_f64_le().map_err(|e| e.to_string())?,
            end_param: reader.read_f64_le().map_err(|e| e.to_string())?,
        }),
        4 => Ok(CadEntity::Polyline { vertices: read_point2_vec(reader)?, closed: read_bool(reader)? }),
        5 => Ok(CadEntity::Text { position: read_point2(reader)?, height: reader.read_f64_le().map_err(|e| e.to_string())?, rotation: reader.read_f64_le().map_err(|e| e.to_string())?, content: read_str_lp(reader)? }),
        6 => Ok(CadEntity::Insert { block_name: read_str_lp(reader)?, insertion_point: read_point2(reader)?, scale: read_point2(reader)?, rotation: reader.read_f64_le().map_err(|e| e.to_string())? }),
        7 => Ok(CadEntity::Solid { p1: read_point2(reader)?, p2: read_point2(reader)?, p3: read_point2(reader)?, p4: read_point2(reader)? }),
        8 => Ok(CadEntity::Dimension { def_point: read_point2(reader)?, text_position: read_point2(reader)?, measurement: reader.read_f64_le().map_err(|e| e.to_string())?, text: read_str_lp(reader)? }),
        other => Err(format!("entity: unknown binary tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_layer(out: &mut Vec<u8>, l: &CadLayer) {
    write_str_lp(out, &l.name);
    store::pack_rt::write_varint_u64(out, l.color_index as u64);
    write_str_lp(out, &l.line_type);
    write_bool(out, l.visible);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_layer(reader: &mut store::ByteReader<'_>) -> Result<CadLayer, String> {
    let name = read_str_lp(reader)?;
    let color_index = reader.read_varint_u64().map_err(|e| e.to_string())? as i32;
    let line_type = read_str_lp(reader)?;
    let visible = read_bool(reader)?;
    Ok(CadLayer { name, color_index, line_type, visible })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_entity_record(out: &mut Vec<u8>, r: &CadEntityRecord) {
    write_str_lp(out, &r.handle);
    write_str_lp(out, &r.layer);
    write_entity(out, &r.entity);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_entity_record(reader: &mut store::ByteReader<'_>) -> Result<CadEntityRecord, String> {
    Ok(CadEntityRecord { handle: read_str_lp(reader)?, layer: read_str_lp(reader)?, entity: read_entity(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_block(out: &mut Vec<u8>, b: &CadBlock) {
    write_str_lp(out, &b.name);
    write_point2(out, &b.base_point);
    store::pack_rt::write_varint_u64(out, b.entities.len() as u64);
    for r in &b.entities {
        write_entity_record(out, r);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_block(reader: &mut store::ByteReader<'_>) -> Result<CadBlock, String> {
    let name = read_str_lp(reader)?;
    let base_point = read_point2(reader)?;
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut entities = Vec::with_capacity(n as usize);
    for _ in 0..n {
        entities.push(read_entity_record(reader)?);
    }
    Ok(CadBlock { name, base_point, entities })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_cad_snapshot_binary(s: &SemioCadSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.layers.len() as u64);
    for l in &s.layers {
        write_layer(&mut out, l);
    }
    store::pack_rt::write_varint_u64(&mut out, s.blocks.len() as u64);
    for b in &s.blocks {
        write_block(&mut out, b);
    }
    store::pack_rt::write_varint_u64(&mut out, s.entities.len() as u64);
    for r in &s.entities {
        write_entity_record(&mut out, r);
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_cad_snapshot_binary(bytes: &[u8]) -> Result<SemioCadSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let layer_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut layers = Vec::with_capacity(layer_count as usize);
    for _ in 0..layer_count {
        layers.push(read_layer(&mut reader)?);
    }
    let block_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut blocks = Vec::with_capacity(block_count as usize);
    for _ in 0..block_count {
        blocks.push(read_block(&mut reader)?);
    }
    let entity_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut entities = Vec::with_capacity(entity_count as usize);
    for _ in 0..entity_count {
        entities.push(read_entity_record(&mut reader)?);
    }
    Ok(SemioCadSnapshot { schema, layers, blocks, entities })
}

impl store::ArtifactPack for SemioCadSnapshot {
    /// 🪶️ Publishes the owned typed relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_cad_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        decode_cad_snapshot_binary(&inner).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::cad::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::cad::io::text::snapshot::*;
/// 📥️ Decodes this subset's own committed `.pack.semio` bytes into a real [`SemioCadSnapshot`] — the
/// binary half of the same bridge, so a caller outside this crate can check the two codecs against
/// each other on the two real committed artifacts instead of against itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_cad_pack(bytes: &[u8]) -> Result<SemioCadSnapshot, String> {
    <SemioCadSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
/// 📤️ The `store::ArtifactPack::encode_pack` inverse of [`decode_semio_cad_pack`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_cad_pack(snapshot: &SemioCadSnapshot) -> Vec<u8> {
    <SemioCadSnapshot as store::ArtifactPack>::encode_pack(snapshot)
}
}
pub use native_snapshot_codec::*;
