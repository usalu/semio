//! 💾️ Binary representation codec surface for `stdio.semio.animation.snapshot`.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::animation::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion};
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;

/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, the same helpers every semio wave's upgraded `ArtifactPack` reuses)
/// backing the real `ArtifactPack` below — replaces the old `serde_json::to_vec`-in-envelope
/// shortcut.
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
pub(crate) fn write_point3(out: &mut Vec<u8>, p: &SemioPoint3) {
    out.extend_from_slice(&p.x.to_le_bytes());
    out.extend_from_slice(&p.y.to_le_bytes());
    out.extend_from_slice(&p.z.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_point3(reader: &mut store::ByteReader<'_>) -> Result<SemioPoint3, String> {
    Ok(SemioPoint3 { x: reader.read_f64_le().map_err(|e| e.to_string())?, y: reader.read_f64_le().map_err(|e| e.to_string())?, z: reader.read_f64_le().map_err(|e| e.to_string())? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_quat(out: &mut Vec<u8>, q: &SemioQuaternion) {
    out.extend_from_slice(&q.x.to_le_bytes());
    out.extend_from_slice(&q.y.to_le_bytes());
    out.extend_from_slice(&q.z.to_le_bytes());
    out.extend_from_slice(&q.w.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_quat(reader: &mut store::ByteReader<'_>) -> Result<SemioQuaternion, String> {
    Ok(SemioQuaternion { x: reader.read_f64_le().map_err(|e| e.to_string())?, y: reader.read_f64_le().map_err(|e| e.to_string())?, z: reader.read_f64_le().map_err(|e| e.to_string())?, w: reader.read_f64_le().map_err(|e| e.to_string())? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_f64_vec(out: &mut Vec<u8>, v: &[f64]) {
    store::pack_rt::write_varint_u64(out, v.len() as u64);
    for x in v {
        out.extend_from_slice(&x.to_le_bytes());
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_f64_vec(reader: &mut store::ByteReader<'_>) -> Result<Vec<f64>, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut v = Vec::with_capacity(n as usize);
    for _ in 0..n {
        v.push(reader.read_f64_le().map_err(|e| e.to_string())?);
    }
    Ok(v)
}

/// 🏷️ `AnimTargetProperty` variant tags — 0=Translation, 1=Rotation, 2=Scale, 3=Weights, 4=Custom.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_property(out: &mut Vec<u8>, p: &AnimTargetProperty) {
    match p {
        AnimTargetProperty::Translation => out.push(0),
        AnimTargetProperty::Rotation => out.push(1),
        AnimTargetProperty::Scale => out.push(2),
        AnimTargetProperty::Weights => out.push(3),
        AnimTargetProperty::Custom { name } => {
            out.push(4);
            write_str_lp(out, name);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_property(reader: &mut store::ByteReader<'_>) -> Result<AnimTargetProperty, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(AnimTargetProperty::Translation),
        1 => Ok(AnimTargetProperty::Rotation),
        2 => Ok(AnimTargetProperty::Scale),
        3 => Ok(AnimTargetProperty::Weights),
        4 => Ok(AnimTargetProperty::Custom { name: read_str_lp(reader)? }),
        other => Err(format!("property: unknown binary tag {other}")),
    }
}

/// 🏷️ `AnimInterpolation` variant tags — 0=Linear, 1=Step, 2=CubicSpline.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_interpolation(out: &mut Vec<u8>, i: AnimInterpolation) {
    out.push(match i {
        AnimInterpolation::Linear => 0,
        AnimInterpolation::Step => 1,
        AnimInterpolation::CubicSpline => 2,
    });
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_interpolation(reader: &mut store::ByteReader<'_>) -> Result<AnimInterpolation, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(AnimInterpolation::Linear),
        1 => Ok(AnimInterpolation::Step),
        2 => Ok(AnimInterpolation::CubicSpline),
        other => Err(format!("interpolation: unknown binary tag {other}")),
    }
}

/// 🏷️ `AnimValue` variant tags — 0=Scalar, 1=Vec3, 2=Quat, 3=Weights.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_value(out: &mut Vec<u8>, v: &AnimValue) {
    match v {
        AnimValue::Scalar { value } => {
            out.push(0);
            out.extend_from_slice(&value.to_le_bytes());
        }
        AnimValue::Vec3 { value } => {
            out.push(1);
            write_point3(out, value);
        }
        AnimValue::Quat { value } => {
            out.push(2);
            write_quat(out, value);
        }
        AnimValue::Weights { values } => {
            out.push(3);
            write_f64_vec(out, values);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_value(reader: &mut store::ByteReader<'_>) -> Result<AnimValue, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(AnimValue::Scalar { value: reader.read_f64_le().map_err(|e| e.to_string())? }),
        1 => Ok(AnimValue::Vec3 { value: read_point3(reader)? }),
        2 => Ok(AnimValue::Quat { value: read_quat(reader)? }),
        3 => Ok(AnimValue::Weights { values: read_f64_vec(reader)? }),
        other => Err(format!("value: unknown binary tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_target(out: &mut Vec<u8>, t: &AnimTarget) {
    write_str_lp(out, &t.node);
    write_property(out, &t.property);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_target(reader: &mut store::ByteReader<'_>) -> Result<AnimTarget, String> {
    Ok(AnimTarget { node: read_str_lp(reader)?, property: read_property(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_keyframe(out: &mut Vec<u8>, k: &AnimKeyframe) {
    out.extend_from_slice(&k.t.to_le_bytes());
    write_value(out, &k.value);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_keyframe(reader: &mut store::ByteReader<'_>) -> Result<AnimKeyframe, String> {
    Ok(AnimKeyframe { t: reader.read_f64_le().map_err(|e| e.to_string())?, value: read_value(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_channel(out: &mut Vec<u8>, c: &AnimChannel) {
    write_target(out, &c.target);
    write_interpolation(out, c.interpolation);
    store::pack_rt::write_varint_u64(out, c.keyframes.len() as u64);
    for k in &c.keyframes {
        write_keyframe(out, k);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_channel(reader: &mut store::ByteReader<'_>) -> Result<AnimChannel, String> {
    let target = read_target(reader)?;
    let interpolation = read_interpolation(reader)?;
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut keyframes = Vec::with_capacity(n as usize);
    for _ in 0..n {
        keyframes.push(read_keyframe(reader)?);
    }
    Ok(AnimChannel { target, interpolation, keyframes })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_timeline(out: &mut Vec<u8>, t: &AnimTimeline) {
    match &t.name {
        Some(n) => {
            out.push(1);
            write_str_lp(out, n);
        }
        None => out.push(0),
    }
    store::pack_rt::write_varint_u64(out, t.channels.len() as u64);
    for c in &t.channels {
        write_channel(out, c);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_timeline(reader: &mut store::ByteReader<'_>) -> Result<AnimTimeline, String> {
    let has_name = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let name = if has_name { Some(read_str_lp(reader)?) } else { None };
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut channels = Vec::with_capacity(n as usize);
    for _ in 0..n {
        channels.push(read_channel(reader)?);
    }
    Ok(AnimTimeline { name, channels })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_animation_snapshot_binary(s: &SemioAnimationSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.timelines.len() as u64);
    for t in &s.timelines {
        write_timeline(&mut out, t);
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_animation_snapshot_binary(bytes: &[u8]) -> Result<SemioAnimationSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut timelines = Vec::with_capacity(n as usize);
    for _ in 0..n {
        timelines.push(read_timeline(&mut reader)?);
    }
    Ok(SemioAnimationSnapshot { schema, timelines })
}

impl store::ArtifactPack for SemioAnimationSnapshot {
    /// 🪶️ Publishes the owned typed relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_animation_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        decode_animation_snapshot_binary(&inner).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
}
pub use snapshot_codec::*;
