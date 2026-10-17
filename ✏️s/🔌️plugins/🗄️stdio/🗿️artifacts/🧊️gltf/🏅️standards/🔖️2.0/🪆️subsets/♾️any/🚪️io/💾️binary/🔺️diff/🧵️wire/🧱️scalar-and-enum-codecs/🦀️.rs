//! 🧱️ Binary primitives, option, tri-state and vector framing, JSON blobs and the glTF enum codecs.

use crate::{standards::v2_0::subsets::any::schema::snapshot::GltfAccessorType, standards::v2_0::subsets::any::schema::snapshot::GltfComponentType};
use crate::{standards::v2_0::subsets::any::schema::snapshot::GltfAlphaMode, standards::v2_0::subsets::any::schema::snapshot::GltfAnimationPath, standards::v2_0::subsets::any::schema::snapshot::GltfInterpolation, standards::v2_0::subsets::any::schema::snapshot::GltfJson, standards::v2_0::subsets::any::schema::snapshot::GltfSourceForm};
use crate::standards::v2_0::subsets::any::io::binary::diff::wire::*;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_blob(r: &mut dsl::ByteReader<'_>) -> Result<Vec<u8>, dsl::PackRefusal> {
    let len = r.read_varint_u64()? as usize;
    Ok(r.read_bytes(len)?.to_vec())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_str(r: &mut dsl::ByteReader<'_>) -> Result<String, dsl::PackRefusal> {
    let bytes = read_bin_blob(r)?;
    String::from_utf8(bytes).map_err(|e| dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "gltf binary utf8 string", offset: 0, detail: e.to_string() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_option<T>(r: &mut dsl::ByteReader<'_>, read_value: impl FnOnce(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackRefusal>) -> Result<Option<T>, dsl::PackRefusal> {
    match r.read_u8()? {
        0 => Ok(None),
        1 => Ok(Some(read_value(r)?)),
        other => Err(dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "gltf binary option tag", offset: 0, detail: format!("unknown tag {other}") }),
    }
}
/// 🧩 3-way flag (`0`=unchanged/absent, `1`=cleared-to-`None`, `2`=set-to-`Some(value)`) for every
/// TRI-STATE `Option<Option<T>>` field — same shape as png's/gif's own doc comment (avoids
/// chaining two `if`-guarded conditional fields at the PROTOCOL-DESCRIPTION level,
/// `protocol-cond-cannot-chain`; the Rust codec here has no such limitation but keeps the same
/// 3-way-flag SHAPE for parity with `../💾️binary/📡️.protocol.semio`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_tri<T>(w: &mut dsl::ByteWriter, v: &Option<Option<T>>, write_value: impl FnOnce(&mut dsl::ByteWriter, &T)) {
    match v {
        None => w.write_u8(0),
        Some(None) => w.write_u8(1),
        Some(Some(val)) => {
            w.write_u8(2);
            write_value(w, val);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_tri<T>(r: &mut dsl::ByteReader<'_>, read_value: impl FnOnce(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackRefusal>) -> Result<Option<Option<T>>, dsl::PackRefusal> {
    match r.read_u8()? {
        0 => Ok(None),
        1 => Ok(Some(None)),
        2 => Ok(Some(Some(read_value(r)?))),
        other => Err(dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "gltf binary tri-flag", offset: 0, detail: format!("unknown flag {other}") }),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_vec<T>(r: &mut dsl::ByteReader<'_>, mut read_item: impl FnMut(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackRefusal>) -> Result<Vec<T>, dsl::PackRefusal> {
    let n = r.read_varint_u64()? as usize;
    let mut out = Vec::with_capacity(n.min(1 << 20));
    for _ in 0..n {
        out.push(read_item(r)?);
    }
    Ok(out)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_f64_array<const N: usize>(w: &mut dsl::ByteWriter, v: &[f64; N]) {
    for x in v {
        w.write_f64_le(*x);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_f64_array<const N: usize>(r: &mut dsl::ByteReader<'_>) -> Result<[f64; N], dsl::PackRefusal> {
    let mut out = [0.0f64; N];
    for slot in out.iter_mut() {
        *slot = r.read_f64_le()?;
    }
    Ok(out)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_f64_vec(w: &mut dsl::ByteWriter, v: &[f64]) {
    write_bin_vec(w, v, |w, x| w.write_f64_le(*x));
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_f64_vec(r: &mut dsl::ByteReader<'_>) -> Result<Vec<f64>, dsl::PackRefusal> {
    read_bin_vec(r, |r| r.read_f64_le())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_usize_vec(w: &mut dsl::ByteWriter, v: &[usize]) {
    write_bin_vec(w, v, |w, x: &usize| w.write_varint_u64(*x as u64));
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_usize_vec(r: &mut dsl::ByteReader<'_>) -> Result<Vec<usize>, dsl::PackRefusal> {
    read_bin_vec(r, |r| Ok(r.read_varint_u64()? as usize))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_attr_pairs(w: &mut dsl::ByteWriter, v: &[(String, usize)]) {
    write_bin_vec(w, v, |w, (k, idx): &(String, usize)| {
        write_bin_str(w, k);
        w.write_varint_u64(*idx as u64);
    });
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_attr_pairs(r: &mut dsl::ByteReader<'_>) -> Result<Vec<(String, usize)>, dsl::PackRefusal> {
    read_bin_vec(r, |r| Ok((read_bin_str(r)?, r.read_varint_u64()? as usize)))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn gltf_bin_err(e: &dsl::PackRefusal) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "gltf binary", offset: 0, detail: e.to_string() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_json(r: &mut dsl::ByteReader<'_>) -> Result<GltfJson, dsl::PackRefusal> {
    match r.read_u8()? {
        0 => Ok(GltfJson::Null),
        1 => Ok(GltfJson::Bool(r.read_u8()? != 0)),
        2 => Ok(GltfJson::Number(r.read_f64_le()?)),
        3 => Ok(GltfJson::String(read_bin_str(r)?)),
        4 => Ok(GltfJson::Array(read_bin_vec(r, read_bin_json)?)),
        5 => Ok(GltfJson::Object(read_bin_vec(r, |r| Ok((read_bin_str(r)?, read_bin_json(r)?)))?)),
        other => Err(dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "gltf json binary tag", offset: 0, detail: format!("unknown tag {other}") }),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_json_opt(r: &mut dsl::ByteReader<'_>) -> Result<Option<GltfJson>, dsl::PackRefusal> {
    read_bin_option(r, read_bin_json)
}
/// 🔢️ Real spec numeric code (5120..5126), matching `GltfComponentType::code`/`from_code` exactly
/// -- NOT a re-derived discriminant table (the spec code IS this enum's real wire value, same one
/// the artifact's own `serde` impl emits).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_component_type(w: &mut dsl::ByteWriter, t: GltfComponentType) {
    w.write_u32_le(t.code() as u32);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_component_type(r: &mut dsl::ByteReader<'_>) -> Result<GltfComponentType, dsl::PackRefusal> {
    GltfComponentType::from_code(r.read_u32_le()? as u64).map_err(|e| dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "gltf component_type", offset: 0, detail: e })
}
/// 🔢️ Compact `u8` discriminants for the remaining small unit-variant enums (real spec strings
/// only exist on the TEXT side; the binary frame is free to use its own dense encoding since
/// nothing outside this codec pair ever reads these bytes directly).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_accessor_type(w: &mut dsl::ByteWriter, t: GltfAccessorType) {
    w.write_u8(match t {
        GltfAccessorType::Scalar => 0,
        GltfAccessorType::Vec2 => 1,
        GltfAccessorType::Vec3 => 2,
        GltfAccessorType::Vec4 => 3,
        GltfAccessorType::Mat2 => 4,
        GltfAccessorType::Mat3 => 5,
        GltfAccessorType::Mat4 => 6,
    });
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_accessor_type(r: &mut dsl::ByteReader<'_>) -> Result<GltfAccessorType, dsl::PackRefusal> {
    Ok(match r.read_u8()? {
        0 => GltfAccessorType::Scalar,
        1 => GltfAccessorType::Vec2,
        2 => GltfAccessorType::Vec3,
        3 => GltfAccessorType::Vec4,
        4 => GltfAccessorType::Mat2,
        5 => GltfAccessorType::Mat3,
        6 => GltfAccessorType::Mat4,
        other => return Err(dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "gltf accessor_type", offset: 0, detail: format!("unknown tag {other}") }),
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_alpha_mode(w: &mut dsl::ByteWriter, m: GltfAlphaMode) {
    w.write_u8(match m {
        GltfAlphaMode::Opaque => 0,
        GltfAlphaMode::Mask => 1,
        GltfAlphaMode::Blend => 2,
    });
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_alpha_mode(r: &mut dsl::ByteReader<'_>) -> Result<GltfAlphaMode, dsl::PackRefusal> {
    Ok(match r.read_u8()? {
        0 => GltfAlphaMode::Opaque,
        1 => GltfAlphaMode::Mask,
        2 => GltfAlphaMode::Blend,
        other => return Err(dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "gltf alpha_mode", offset: 0, detail: format!("unknown tag {other}") }),
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_interpolation(w: &mut dsl::ByteWriter, i: GltfInterpolation) {
    w.write_u8(match i {
        GltfInterpolation::Linear => 0,
        GltfInterpolation::Step => 1,
        GltfInterpolation::CubicSpline => 2,
    });
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_interpolation(r: &mut dsl::ByteReader<'_>) -> Result<GltfInterpolation, dsl::PackRefusal> {
    Ok(match r.read_u8()? {
        0 => GltfInterpolation::Linear,
        1 => GltfInterpolation::Step,
        2 => GltfInterpolation::CubicSpline,
        other => return Err(dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "gltf interpolation", offset: 0, detail: format!("unknown tag {other}") }),
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_animation_path(w: &mut dsl::ByteWriter, p: GltfAnimationPath) {
    w.write_u8(match p {
        GltfAnimationPath::Translation => 0,
        GltfAnimationPath::Rotation => 1,
        GltfAnimationPath::Scale => 2,
        GltfAnimationPath::Weights => 3,
    });
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_animation_path(r: &mut dsl::ByteReader<'_>) -> Result<GltfAnimationPath, dsl::PackRefusal> {
    Ok(match r.read_u8()? {
        0 => GltfAnimationPath::Translation,
        1 => GltfAnimationPath::Rotation,
        2 => GltfAnimationPath::Scale,
        3 => GltfAnimationPath::Weights,
        other => return Err(dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "gltf animation_path", offset: 0, detail: format!("unknown tag {other}") }),
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_source_form(w: &mut dsl::ByteWriter, f: GltfSourceForm) {
    w.write_u8(match f {
        GltfSourceForm::Json => 0,
        GltfSourceForm::Glb => 1,
    });
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_source_form(r: &mut dsl::ByteReader<'_>) -> Result<GltfSourceForm, dsl::PackRefusal> {
    Ok(match r.read_u8()? {
        0 => GltfSourceForm::Json,
        1 => GltfSourceForm::Glb,
        other => return Err(dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "gltf source_form", offset: 0, detail: format!("unknown tag {other}") }),
    })
}
