//! binary rep for stdio.gltf 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::snapshot::{GltfAccessorType, GltfComponentType};
use crate::schema::snapshot::{
    GltfAccessor, GltfAlphaMode, GltfAnimation, GltfAnimationChannel, GltfAnimationChannelTarget, GltfAnimationPath, GltfAnimationSampler, GltfAsset, GltfBuffer, GltfBufferView, GltfCamera, GltfCameraProjection, GltfImage, GltfInterpolation,
    GltfJson, GltfMaterial, GltfMesh, GltfMorphTarget, GltfNode, GltfNormalTextureInfo, GltfOcclusionTextureInfo, GltfOrthographic, GltfPbrMetallicRoughness, GltfPerspective, GltfPrimitive, GltfSampler, GltfScene, GltfSkin, GltfSnapshot,
    GltfSourceForm, GltfSparseAccessor, GltfSparseIndices, GltfSparseValues, GltfTexture, GltfTextureInfo,
};
#[cfg(test)]
use crate::schema::snapshot::GltfDocument;
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use protocol::MutationDiff;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_blob(r: &mut dsl::ByteReader<'_>) -> Result<Vec<u8>, dsl::PackRefusal> {
    let len = r.read_varint_u64()? as usize;
    Ok(r.read_bytes(len)?.to_vec())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_str(r: &mut dsl::ByteReader<'_>) -> Result<String, dsl::PackRefusal> {
    let bytes = read_bin_blob(r)?;
    String::from_utf8(bytes).map_err(|e| dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "gltf binary utf8 string", offset: 0, detail: e.to_string() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_option<T>(r: &mut dsl::ByteReader<'_>, read_value: impl FnOnce(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackRefusal>) -> Result<Option<T>, dsl::PackRefusal> {
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
pub(crate) fn write_bin_tri<T>(w: &mut dsl::ByteWriter, v: &Option<Option<T>>, write_value: impl FnOnce(&mut dsl::ByteWriter, &T)) {
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
pub(crate) fn read_bin_tri<T>(r: &mut dsl::ByteReader<'_>, read_value: impl FnOnce(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackRefusal>) -> Result<Option<Option<T>>, dsl::PackRefusal> {
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
pub(crate) fn write_bin_string_vec(w: &mut dsl::ByteWriter, v: &[String]) {
    write_bin_vec(w, v, |w, s: &String| write_bin_str(w, s));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_string_vec(r: &mut dsl::ByteReader<'_>) -> Result<Vec<String>, dsl::PackRefusal> {
    read_bin_vec(r, read_bin_str)
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
pub(crate) fn gltf_bin_err(e: &dsl::PackRefusal) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "gltf binary", offset: 0, detail: e.to_string() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_json(r: &mut dsl::ByteReader<'_>) -> Result<GltfJson, dsl::PackRefusal> {
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
pub(crate) fn write_bin_source_form(w: &mut dsl::ByteWriter, f: GltfSourceForm) {
    w.write_u8(match f {
        GltfSourceForm::Json => 0,
        GltfSourceForm::Glb => 1,
    });
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_source_form(r: &mut dsl::ByteReader<'_>) -> Result<GltfSourceForm, dsl::PackRefusal> {
    Ok(match r.read_u8()? {
        0 => GltfSourceForm::Json,
        1 => GltfSourceForm::Glb,
        other => return Err(dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "gltf source_form", offset: 0, detail: format!("unknown tag {other}") }),
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_asset_diff(w: &mut dsl::ByteWriter, d: &GltfAssetDiff) {
    write_bin_option(w, &d.version, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.generator, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.copyright, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.min_version, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_asset_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfAssetDiff, dsl::PackRefusal> {
    Ok(GltfAssetDiff {
        version: read_bin_option(r, read_bin_str)?,
        generator: read_bin_tri(r, read_bin_str)?,
        copyright: read_bin_tri(r, read_bin_str)?,
        min_version: read_bin_tri(r, read_bin_str)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_scene(w: &mut dsl::ByteWriter, sc: &GltfScene) {
    write_bin_usize_vec(w, &sc.nodes);
    write_bin_option(w, &sc.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &sc.extensions);
    write_bin_json_opt(w, &sc.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_scene(r: &mut dsl::ByteReader<'_>) -> Result<GltfScene, dsl::PackRefusal> {
    Ok(GltfScene { nodes: read_bin_usize_vec(r)?, name: read_bin_option(r, read_bin_str)?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_scene_diff(w: &mut dsl::ByteWriter, d: &GltfSceneDiff) {
    write_bin_option(w, &d.nodes, |w, v| write_bin_usize_vec(w, v));
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_scene_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfSceneDiff, dsl::PackRefusal> {
    Ok(GltfSceneDiff { nodes: read_bin_option(r, read_bin_usize_vec)?, name: read_bin_tri(r, read_bin_str)?, extensions: read_bin_tri(r, read_bin_json)?, extras: read_bin_tri(r, read_bin_json)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_node(w: &mut dsl::ByteWriter, n: &GltfNode) {
    write_bin_usize_vec(w, &n.children);
    write_bin_option(w, &n.mesh, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &n.camera, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &n.skin, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &n.matrix, write_bin_f64_array::<16>);
    write_bin_option(w, &n.translation, write_bin_f64_array::<3>);
    write_bin_option(w, &n.rotation, write_bin_f64_array::<4>);
    write_bin_option(w, &n.scale, write_bin_f64_array::<3>);
    write_bin_f64_vec(w, &n.weights);
    write_bin_option(w, &n.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &n.extensions);
    write_bin_json_opt(w, &n.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_node(r: &mut dsl::ByteReader<'_>) -> Result<GltfNode, dsl::PackRefusal> {
    Ok(GltfNode {
        children: read_bin_usize_vec(r)?,
        mesh: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        camera: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        skin: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        matrix: read_bin_option(r, read_bin_f64_array::<16>)?,
        translation: read_bin_option(r, read_bin_f64_array::<3>)?,
        rotation: read_bin_option(r, read_bin_f64_array::<4>)?,
        scale: read_bin_option(r, read_bin_f64_array::<3>)?,
        weights: read_bin_f64_vec(r)?,
        name: read_bin_option(r, read_bin_str)?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_node_diff(w: &mut dsl::ByteWriter, d: &GltfNodeDiff) {
    write_bin_option(w, &d.children, |w, v| write_bin_usize_vec(w, v));
    write_bin_tri(w, &d.mesh, |w, v| w.write_varint_u64(*v as u64));
    write_bin_tri(w, &d.camera, |w, v| w.write_varint_u64(*v as u64));
    write_bin_tri(w, &d.skin, |w, v| w.write_varint_u64(*v as u64));
    write_bin_tri(w, &d.matrix, write_bin_f64_array::<16>);
    write_bin_tri(w, &d.translation, write_bin_f64_array::<3>);
    write_bin_tri(w, &d.rotation, write_bin_f64_array::<4>);
    write_bin_tri(w, &d.scale, write_bin_f64_array::<3>);
    write_bin_option(w, &d.weights, |w, v| write_bin_f64_vec(w, v));
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_node_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfNodeDiff, dsl::PackRefusal> {
    Ok(GltfNodeDiff {
        children: read_bin_option(r, read_bin_usize_vec)?,
        mesh: read_bin_tri(r, |r| Ok(r.read_varint_u64()? as usize))?,
        camera: read_bin_tri(r, |r| Ok(r.read_varint_u64()? as usize))?,
        skin: read_bin_tri(r, |r| Ok(r.read_varint_u64()? as usize))?,
        matrix: read_bin_tri(r, read_bin_f64_array::<16>)?,
        translation: read_bin_tri(r, read_bin_f64_array::<3>)?,
        rotation: read_bin_tri(r, read_bin_f64_array::<4>)?,
        scale: read_bin_tri(r, read_bin_f64_array::<3>)?,
        weights: read_bin_option(r, read_bin_f64_vec)?,
        name: read_bin_tri(r, read_bin_str)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_primitive(w: &mut dsl::ByteWriter, p: &GltfPrimitive) {
    write_bin_attr_pairs(w, &p.attributes);
    write_bin_option(w, &p.indices, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &p.material, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &p.mode, |w, v| w.write_varint_u64(*v));
    write_bin_vec(w, &p.targets, |w, target| write_bin_attr_pairs(w, &target.0));
    write_bin_json_opt(w, &p.extensions);
    write_bin_json_opt(w, &p.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_primitive(r: &mut dsl::ByteReader<'_>) -> Result<GltfPrimitive, dsl::PackRefusal> {
    Ok(GltfPrimitive {
        attributes: read_bin_attr_pairs(r)?,
        indices: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        material: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        mode: read_bin_option(r, |r| r.read_varint_u64())?,
        targets: read_bin_vec(r, |r| read_bin_attr_pairs(r).map(GltfMorphTarget))?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_primitive_vec(w: &mut dsl::ByteWriter, v: &[GltfPrimitive]) {
    write_bin_vec(w, v, write_bin_primitive);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_primitive_vec(r: &mut dsl::ByteReader<'_>) -> Result<Vec<GltfPrimitive>, dsl::PackRefusal> {
    read_bin_vec(r, read_bin_primitive)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_mesh(w: &mut dsl::ByteWriter, m: &GltfMesh) {
    write_bin_primitive_vec(w, &m.primitives);
    write_bin_f64_vec(w, &m.weights);
    write_bin_option(w, &m.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &m.extensions);
    write_bin_json_opt(w, &m.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_mesh(r: &mut dsl::ByteReader<'_>) -> Result<GltfMesh, dsl::PackRefusal> {
    Ok(GltfMesh { primitives: read_bin_primitive_vec(r)?, weights: read_bin_f64_vec(r)?, name: read_bin_option(r, read_bin_str)?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_mesh_diff(w: &mut dsl::ByteWriter, d: &GltfMeshDiff) {
    write_bin_option(w, &d.primitives, |w, v| write_bin_primitive_vec(w, v));
    write_bin_option(w, &d.weights, |w, v| write_bin_f64_vec(w, v));
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_mesh_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfMeshDiff, dsl::PackRefusal> {
    Ok(GltfMeshDiff {
        primitives: read_bin_option(r, read_bin_primitive_vec)?,
        weights: read_bin_option(r, read_bin_f64_vec)?,
        name: read_bin_tri(r, read_bin_str)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_sparse_indices(w: &mut dsl::ByteWriter, v: &GltfSparseIndices) {
    w.write_varint_u64(v.buffer_view as u64);
    w.write_varint_u64(v.byte_offset as u64);
    write_bin_component_type(w, v.component_type);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_sparse_indices(r: &mut dsl::ByteReader<'_>) -> Result<GltfSparseIndices, dsl::PackRefusal> {
    Ok(GltfSparseIndices { buffer_view: r.read_varint_u64()? as usize, byte_offset: r.read_varint_u64()? as usize, component_type: read_bin_component_type(r)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_sparse_values(w: &mut dsl::ByteWriter, v: &GltfSparseValues) {
    w.write_varint_u64(v.buffer_view as u64);
    w.write_varint_u64(v.byte_offset as u64);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_sparse_values(r: &mut dsl::ByteReader<'_>) -> Result<GltfSparseValues, dsl::PackRefusal> {
    Ok(GltfSparseValues { buffer_view: r.read_varint_u64()? as usize, byte_offset: r.read_varint_u64()? as usize })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_sparse_accessor(w: &mut dsl::ByteWriter, v: &GltfSparseAccessor) {
    w.write_varint_u64(v.count as u64);
    write_bin_sparse_indices(w, &v.indices);
    write_bin_sparse_values(w, &v.values);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_sparse_accessor(r: &mut dsl::ByteReader<'_>) -> Result<GltfSparseAccessor, dsl::PackRefusal> {
    Ok(GltfSparseAccessor { count: r.read_varint_u64()? as usize, indices: read_bin_sparse_indices(r)?, values: read_bin_sparse_values(r)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_accessor(w: &mut dsl::ByteWriter, a: &GltfAccessor) {
    write_bin_option(w, &a.buffer_view, |w, v| w.write_varint_u64(*v as u64));
    w.write_varint_u64(a.byte_offset as u64);
    write_bin_component_type(w, a.component_type);
    w.write_u8(if a.normalized { 1 } else { 0 });
    w.write_varint_u64(a.count as u64);
    write_bin_accessor_type(w, a.kind);
    write_bin_option(w, &a.max, |w, v| write_bin_f64_vec(w, v));
    write_bin_option(w, &a.min, |w, v| write_bin_f64_vec(w, v));
    write_bin_option(w, &a.sparse, write_bin_sparse_accessor);
    write_bin_option(w, &a.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &a.extensions);
    write_bin_json_opt(w, &a.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_accessor(r: &mut dsl::ByteReader<'_>) -> Result<GltfAccessor, dsl::PackRefusal> {
    Ok(GltfAccessor {
        buffer_view: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        byte_offset: r.read_varint_u64()? as usize,
        component_type: read_bin_component_type(r)?,
        normalized: r.read_u8()? != 0,
        count: r.read_varint_u64()? as usize,
        kind: read_bin_accessor_type(r)?,
        max: read_bin_option(r, read_bin_f64_vec)?,
        min: read_bin_option(r, read_bin_f64_vec)?,
        sparse: read_bin_option(r, read_bin_sparse_accessor)?,
        name: read_bin_option(r, read_bin_str)?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_accessor_diff(w: &mut dsl::ByteWriter, d: &GltfAccessorDiff) {
    write_bin_tri(w, &d.buffer_view, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &d.byte_offset, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &d.component_type, |w, v| write_bin_component_type(w, *v));
    write_bin_option(w, &d.normalized, |w, v| w.write_u8(if *v { 1 } else { 0 }));
    write_bin_option(w, &d.count, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &d.kind, |w, v| write_bin_accessor_type(w, *v));
    write_bin_tri(w, &d.max, |w, v| write_bin_f64_vec(w, v));
    write_bin_tri(w, &d.min, |w, v| write_bin_f64_vec(w, v));
    write_bin_tri(w, &d.sparse, write_bin_sparse_accessor);
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_accessor_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfAccessorDiff, dsl::PackRefusal> {
    Ok(GltfAccessorDiff {
        buffer_view: read_bin_tri(r, |r| Ok(r.read_varint_u64()? as usize))?,
        byte_offset: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        component_type: read_bin_option(r, read_bin_component_type)?,
        normalized: read_bin_option(r, |r| Ok(r.read_u8()? != 0))?,
        count: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        kind: read_bin_option(r, read_bin_accessor_type)?,
        max: read_bin_tri(r, read_bin_f64_vec)?,
        min: read_bin_tri(r, read_bin_f64_vec)?,
        sparse: read_bin_tri(r, read_bin_sparse_accessor)?,
        name: read_bin_tri(r, read_bin_str)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_texture_info(r: &mut dsl::ByteReader<'_>) -> Result<GltfTextureInfo, dsl::PackRefusal> {
    Ok(GltfTextureInfo { index: r.read_varint_u64()? as usize, tex_coord: r.read_varint_u64()?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_normal_texture_info(r: &mut dsl::ByteReader<'_>) -> Result<GltfNormalTextureInfo, dsl::PackRefusal> {
    Ok(GltfNormalTextureInfo { index: r.read_varint_u64()? as usize, tex_coord: r.read_varint_u64()?, scale: r.read_f64_le()?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_occlusion_texture_info(r: &mut dsl::ByteReader<'_>) -> Result<GltfOcclusionTextureInfo, dsl::PackRefusal> {
    Ok(GltfOcclusionTextureInfo { index: r.read_varint_u64()? as usize, tex_coord: r.read_varint_u64()?, strength: r.read_f64_le()?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_pbr(w: &mut dsl::ByteWriter, v: &GltfPbrMetallicRoughness) {
    write_bin_f64_array::<4>(w, &v.base_color_factor);
    write_bin_option(w, &v.base_color_texture, write_bin_texture_info);
    w.write_f64_le(v.metallic_factor);
    w.write_f64_le(v.roughness_factor);
    write_bin_option(w, &v.metallic_roughness_texture, write_bin_texture_info);
    write_bin_json_opt(w, &v.extensions);
    write_bin_json_opt(w, &v.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_pbr(r: &mut dsl::ByteReader<'_>) -> Result<GltfPbrMetallicRoughness, dsl::PackRefusal> {
    Ok(GltfPbrMetallicRoughness {
        base_color_factor: read_bin_f64_array::<4>(r)?,
        base_color_texture: read_bin_option(r, read_bin_texture_info)?,
        metallic_factor: r.read_f64_le()?,
        roughness_factor: r.read_f64_le()?,
        metallic_roughness_texture: read_bin_option(r, read_bin_texture_info)?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_material(w: &mut dsl::ByteWriter, m: &GltfMaterial) {
    write_bin_option(w, &m.name, |w, v| write_bin_str(w, v));
    write_bin_option(w, &m.pbr_metallic_roughness, write_bin_pbr);
    write_bin_option(w, &m.normal_texture, write_bin_normal_texture_info);
    write_bin_option(w, &m.occlusion_texture, write_bin_occlusion_texture_info);
    write_bin_option(w, &m.emissive_texture, write_bin_texture_info);
    write_bin_f64_array::<3>(w, &m.emissive_factor);
    write_bin_alpha_mode(w, m.alpha_mode);
    w.write_f64_le(m.alpha_cutoff);
    w.write_u8(if m.double_sided { 1 } else { 0 });
    write_bin_json_opt(w, &m.extensions);
    write_bin_json_opt(w, &m.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_material(r: &mut dsl::ByteReader<'_>) -> Result<GltfMaterial, dsl::PackRefusal> {
    Ok(GltfMaterial {
        name: read_bin_option(r, read_bin_str)?,
        pbr_metallic_roughness: read_bin_option(r, read_bin_pbr)?,
        normal_texture: read_bin_option(r, read_bin_normal_texture_info)?,
        occlusion_texture: read_bin_option(r, read_bin_occlusion_texture_info)?,
        emissive_texture: read_bin_option(r, read_bin_texture_info)?,
        emissive_factor: read_bin_f64_array::<3>(r)?,
        alpha_mode: read_bin_alpha_mode(r)?,
        alpha_cutoff: r.read_f64_le()?,
        double_sided: r.read_u8()? != 0,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_material_diff(w: &mut dsl::ByteWriter, d: &GltfMaterialDiff) {
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.pbr_metallic_roughness, write_bin_pbr);
    write_bin_tri(w, &d.normal_texture, write_bin_normal_texture_info);
    write_bin_tri(w, &d.occlusion_texture, write_bin_occlusion_texture_info);
    write_bin_tri(w, &d.emissive_texture, write_bin_texture_info);
    write_bin_option(w, &d.emissive_factor, write_bin_f64_array::<3>);
    write_bin_option(w, &d.alpha_mode, |w, v| write_bin_alpha_mode(w, *v));
    write_bin_option(w, &d.alpha_cutoff, |w, v| w.write_f64_le(*v));
    write_bin_option(w, &d.double_sided, |w, v| w.write_u8(if *v { 1 } else { 0 }));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_material_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfMaterialDiff, dsl::PackRefusal> {
    Ok(GltfMaterialDiff {
        name: read_bin_tri(r, read_bin_str)?,
        pbr_metallic_roughness: read_bin_tri(r, read_bin_pbr)?,
        normal_texture: read_bin_tri(r, read_bin_normal_texture_info)?,
        occlusion_texture: read_bin_tri(r, read_bin_occlusion_texture_info)?,
        emissive_texture: read_bin_tri(r, read_bin_texture_info)?,
        emissive_factor: read_bin_option(r, read_bin_f64_array::<3>)?,
        alpha_mode: read_bin_option(r, read_bin_alpha_mode)?,
        alpha_cutoff: read_bin_option(r, |r| r.read_f64_le())?,
        double_sided: read_bin_option(r, |r| Ok(r.read_u8()? != 0))?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_buffer(w: &mut dsl::ByteWriter, b: &GltfBuffer) {
    w.write_varint_u64(b.byte_length as u64);
    write_bin_option(w, &b.uri, |w, v| write_bin_str(w, v));
    write_bin_option(w, &b.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &b.extensions);
    write_bin_json_opt(w, &b.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_buffer(r: &mut dsl::ByteReader<'_>) -> Result<GltfBuffer, dsl::PackRefusal> {
    Ok(GltfBuffer { byte_length: r.read_varint_u64()? as usize, uri: read_bin_option(r, read_bin_str)?, name: read_bin_option(r, read_bin_str)?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_buffer_diff(w: &mut dsl::ByteWriter, d: &GltfBufferDiff) {
    write_bin_option(w, &d.byte_length, |w, v| w.write_varint_u64(*v as u64));
    write_bin_tri(w, &d.uri, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_buffer_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfBufferDiff, dsl::PackRefusal> {
    Ok(GltfBufferDiff {
        byte_length: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        uri: read_bin_tri(r, read_bin_str)?,
        name: read_bin_tri(r, read_bin_str)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_buffer_view(w: &mut dsl::ByteWriter, v: &GltfBufferView) {
    w.write_varint_u64(v.buffer as u64);
    w.write_varint_u64(v.byte_offset as u64);
    w.write_varint_u64(v.byte_length as u64);
    write_bin_option(w, &v.byte_stride, |w, x| w.write_varint_u64(*x as u64));
    write_bin_option(w, &v.target, |w, x| w.write_varint_u64(*x));
    write_bin_option(w, &v.name, |w, x| write_bin_str(w, x));
    write_bin_json_opt(w, &v.extensions);
    write_bin_json_opt(w, &v.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_buffer_view(r: &mut dsl::ByteReader<'_>) -> Result<GltfBufferView, dsl::PackRefusal> {
    Ok(GltfBufferView {
        buffer: r.read_varint_u64()? as usize,
        byte_offset: r.read_varint_u64()? as usize,
        byte_length: r.read_varint_u64()? as usize,
        byte_stride: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        target: read_bin_option(r, |r| r.read_varint_u64())?,
        name: read_bin_option(r, read_bin_str)?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_texture(r: &mut dsl::ByteReader<'_>) -> Result<GltfTexture, dsl::PackRefusal> {
    Ok(GltfTexture {
        sampler: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        source: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        name: read_bin_option(r, read_bin_str)?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_image(w: &mut dsl::ByteWriter, i: &GltfImage) {
    write_bin_option(w, &i.uri, |w, v| write_bin_str(w, v));
    write_bin_option(w, &i.mime_type, |w, v| write_bin_str(w, v));
    write_bin_option(w, &i.buffer_view, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &i.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &i.extensions);
    write_bin_json_opt(w, &i.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_image(r: &mut dsl::ByteReader<'_>) -> Result<GltfImage, dsl::PackRefusal> {
    Ok(GltfImage {
        uri: read_bin_option(r, read_bin_str)?,
        mime_type: read_bin_option(r, read_bin_str)?,
        buffer_view: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        name: read_bin_option(r, read_bin_str)?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_sampler(w: &mut dsl::ByteWriter, s: &GltfSampler) {
    write_bin_option(w, &s.mag_filter, |w, v| w.write_varint_u64(*v));
    write_bin_option(w, &s.min_filter, |w, v| w.write_varint_u64(*v));
    w.write_varint_u64(s.wrap_s);
    w.write_varint_u64(s.wrap_t);
    write_bin_option(w, &s.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &s.extensions);
    write_bin_json_opt(w, &s.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_sampler(r: &mut dsl::ByteReader<'_>) -> Result<GltfSampler, dsl::PackRefusal> {
    Ok(GltfSampler {
        mag_filter: read_bin_option(r, |r| r.read_varint_u64())?,
        min_filter: read_bin_option(r, |r| r.read_varint_u64())?,
        wrap_s: r.read_varint_u64()?,
        wrap_t: r.read_varint_u64()?,
        name: read_bin_option(r, read_bin_str)?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_skin(w: &mut dsl::ByteWriter, v: &GltfSkin) {
    write_bin_option(w, &v.inverse_bind_matrices, |w, x| w.write_varint_u64(*x as u64));
    write_bin_option(w, &v.skeleton, |w, x| w.write_varint_u64(*x as u64));
    write_bin_usize_vec(w, &v.joints);
    write_bin_option(w, &v.name, |w, x| write_bin_str(w, x));
    write_bin_json_opt(w, &v.extensions);
    write_bin_json_opt(w, &v.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_skin(r: &mut dsl::ByteReader<'_>) -> Result<GltfSkin, dsl::PackRefusal> {
    Ok(GltfSkin {
        inverse_bind_matrices: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        skeleton: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        joints: read_bin_usize_vec(r)?,
        name: read_bin_option(r, read_bin_str)?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_animation_channel_target(w: &mut dsl::ByteWriter, t: &GltfAnimationChannelTarget) {
    write_bin_option(w, &t.node, |w, v| w.write_varint_u64(*v as u64));
    write_bin_animation_path(w, t.path);
    write_bin_json_opt(w, &t.extensions);
    write_bin_json_opt(w, &t.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_animation_channel_target(r: &mut dsl::ByteReader<'_>) -> Result<GltfAnimationChannelTarget, dsl::PackRefusal> {
    Ok(GltfAnimationChannelTarget { node: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?, path: read_bin_animation_path(r)?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_animation_channel(w: &mut dsl::ByteWriter, c: &GltfAnimationChannel) {
    w.write_varint_u64(c.sampler as u64);
    write_bin_animation_channel_target(w, &c.target);
    write_bin_json_opt(w, &c.extensions);
    write_bin_json_opt(w, &c.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_animation_channel(r: &mut dsl::ByteReader<'_>) -> Result<GltfAnimationChannel, dsl::PackRefusal> {
    Ok(GltfAnimationChannel { sampler: r.read_varint_u64()? as usize, target: read_bin_animation_channel_target(r)?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_animation_sampler(w: &mut dsl::ByteWriter, s: &GltfAnimationSampler) {
    w.write_varint_u64(s.input as u64);
    write_bin_interpolation(w, s.interpolation);
    w.write_varint_u64(s.output as u64);
    write_bin_json_opt(w, &s.extensions);
    write_bin_json_opt(w, &s.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_animation_sampler(r: &mut dsl::ByteReader<'_>) -> Result<GltfAnimationSampler, dsl::PackRefusal> {
    Ok(GltfAnimationSampler { input: r.read_varint_u64()? as usize, interpolation: read_bin_interpolation(r)?, output: r.read_varint_u64()? as usize, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_animation(w: &mut dsl::ByteWriter, a: &GltfAnimation) {
    write_bin_vec(w, &a.channels, write_bin_animation_channel);
    write_bin_vec(w, &a.samplers, write_bin_animation_sampler);
    write_bin_option(w, &a.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &a.extensions);
    write_bin_json_opt(w, &a.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_animation(r: &mut dsl::ByteReader<'_>) -> Result<GltfAnimation, dsl::PackRefusal> {
    Ok(GltfAnimation { channels: read_bin_vec(r, read_bin_animation_channel)?, samplers: read_bin_vec(r, read_bin_animation_sampler)?, name: read_bin_option(r, read_bin_str)?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_perspective(w: &mut dsl::ByteWriter, p: &GltfPerspective) {
    write_bin_option(w, &p.aspect_ratio, |w, v| w.write_f64_le(*v));
    w.write_f64_le(p.yfov);
    write_bin_option(w, &p.zfar, |w, v| w.write_f64_le(*v));
    w.write_f64_le(p.znear);
    write_bin_json_opt(w, &p.extensions);
    write_bin_json_opt(w, &p.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_perspective(r: &mut dsl::ByteReader<'_>) -> Result<GltfPerspective, dsl::PackRefusal> {
    Ok(GltfPerspective { aspect_ratio: read_bin_option(r, |r| r.read_f64_le())?, yfov: r.read_f64_le()?, zfar: read_bin_option(r, |r| r.read_f64_le())?, znear: r.read_f64_le()?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_orthographic(w: &mut dsl::ByteWriter, o: &GltfOrthographic) {
    w.write_f64_le(o.xmag);
    w.write_f64_le(o.ymag);
    w.write_f64_le(o.zfar);
    w.write_f64_le(o.znear);
    write_bin_json_opt(w, &o.extensions);
    write_bin_json_opt(w, &o.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_orthographic(r: &mut dsl::ByteReader<'_>) -> Result<GltfOrthographic, dsl::PackRefusal> {
    Ok(GltfOrthographic { xmag: r.read_f64_le()?, ymag: r.read_f64_le()?, zfar: r.read_f64_le()?, znear: r.read_f64_le()?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}

/// 🔀️ `GltfCameraProjection` real data-carrying enum -- tag `u8` (0=Perspective, 1=Orthographic).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_camera_projection(w: &mut dsl::ByteWriter, p: &GltfCameraProjection) {
    match p {
        GltfCameraProjection::Perspective(v) => {
            w.write_u8(0);
            write_bin_perspective(w, v);
        }
        GltfCameraProjection::Orthographic(v) => {
            w.write_u8(1);
            write_bin_orthographic(w, v);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_camera_projection(r: &mut dsl::ByteReader<'_>) -> Result<GltfCameraProjection, dsl::PackRefusal> {
    match r.read_u8()? {
        0 => Ok(GltfCameraProjection::Perspective(read_bin_perspective(r)?)),
        1 => Ok(GltfCameraProjection::Orthographic(read_bin_orthographic(r)?)),
        other => Err(dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "gltf camera_projection", offset: 0, detail: format!("unknown tag {other}") }),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_camera(w: &mut dsl::ByteWriter, c: &GltfCamera) {
    write_bin_camera_projection(w, &c.projection);
    write_bin_option(w, &c.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &c.extensions);
    write_bin_json_opt(w, &c.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_camera(r: &mut dsl::ByteReader<'_>) -> Result<GltfCamera, dsl::PackRefusal> {
    Ok(GltfCamera { projection: read_bin_camera_projection(r)?, name: read_bin_option(r, read_bin_str)?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}

/// 🧮️ Generic index-keyed collection triple real binary codec, shared by every one of the 14
/// top-level arrays -- mirrors `enc_collection`/`dec_collection`'s TEXT shape exactly, real varint
/// counts + real per-item recursive encoding (never text-as-bytes).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_collection<T, D>(w: &mut dsl::ByteWriter, c: &GltfCollectionDiff<T, D>, write_item: impl Fn(&mut dsl::ByteWriter, &T), write_diff: impl Fn(&mut dsl::ByteWriter, &D)) {
    write_bin_vec(w, &c.removed, |w, v: &usize| w.write_varint_u64(*v as u64));
    write_bin_vec(w, &c.modified, |w, m: &GltfModified<D>| {
        w.write_varint_u64(m.index as u64);
        write_diff(w, &m.diff);
    });
    write_bin_vec(w, &c.added, |w, a: &GltfAdded<T>| {
        w.write_varint_u64(a.index as u64);
        write_item(w, &a.item);
    });
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_collection<T, D>(
    r: &mut dsl::ByteReader<'_>,
    read_item: impl Fn(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackRefusal>,
    read_diff: impl Fn(&mut dsl::ByteReader<'_>) -> Result<D, dsl::PackRefusal>,
) -> Result<GltfCollectionDiff<T, D>, dsl::PackRefusal> {
    let removed = read_bin_usize_vec(r)?;
    let modified = read_bin_vec(r, |r| {
        let index = r.read_varint_u64()? as usize;
        let diff = read_diff(r)?;
        Ok(GltfModified { index, diff })
    })?;
    let added = read_bin_vec(r, |r| {
        let index = r.read_varint_u64()? as usize;
        let item = read_item(r)?;
        Ok(GltfAdded { index, item })
    })?;
    Ok(GltfCollectionDiff { removed, modified, added })
}

/// 🧵 A single opaque length-prefixed blob wrapping one collection's real binary encoding --
/// matches `../💾️binary/📡️.protocol.semio`'s `Array(u8, Field(<name>_len))` fields (the
/// blob's OWN internal removed/modified/added shape isn't further protocol-walkable,
/// `protocol-prim-ref-recursion`/`protocol-array-of-records`, same documented limitation as every
/// other stdio pilot's own nested-payload field).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_collection_blob<T, D>(c: &GltfCollectionDiff<T, D>, write_item: impl Fn(&mut dsl::ByteWriter, &T), write_diff: impl Fn(&mut dsl::ByteWriter, &D)) -> Vec<u8> {
    let mut inner = dsl::ByteWriter::new();
    write_bin_collection(&mut inner, c, write_item, write_diff);
    inner.into_bytes()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_collection_blob<T, D>(
    bytes: &[u8],
    read_item: impl Fn(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackRefusal>,
    read_diff: impl Fn(&mut dsl::ByteReader<'_>) -> Result<D, dsl::PackRefusal>,
) -> Result<GltfCollectionDiff<T, D>, dsl::PackRefusal> {
    let mut inner = dsl::ByteReader::new(bytes);
    read_bin_collection(&mut inner, read_item, read_diff)
}

/// 🧪️ P2-FG3: real binary value codecs for `GltfDiff`/`GltfMutation` — mirrors the text codecs
/// above field-for-field, using `dsl::ByteWriter`/`dsl::ByteReader` (the same real LEB128-varint/
/// length-prefixed framework primitives png's/gif89a's own upgraded binary frames use,
/// `🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs`'s `RealBinaryPrimitives`/
/// `RealBinaryDiffFrame` regions — `dsl`/`store`/`protocol` all alias the same kernel crate root,
/// reachable with no `use` needed beyond the absolute path). `pub(crate)` so `🧬️mutations/
/// 🦀️.rs`'s hand-rolled `OpBinary` can reuse every one of these the same way it already
/// reuses this module's TEXT `enc_*`/`dec_*` primitives.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_blob(w: &mut dsl::ByteWriter, bytes: &[u8]) {
    w.write_varint_u64(bytes.len() as u64);
    w.write_bytes(bytes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_str(w: &mut dsl::ByteWriter, s: &str) {
    write_bin_blob(w, s.as_bytes());
}

/// 🧩 2-way presence flag (`0`=None, `1`=Some) — shared by every plain `Option<T>` field.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_option<T>(w: &mut dsl::ByteWriter, v: &Option<T>, write_value: impl FnOnce(&mut dsl::ByteWriter, &T)) {
    match v {
        None => w.write_u8(0),
        Some(val) => {
            w.write_u8(1);
            write_value(w, val);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_vec<T>(w: &mut dsl::ByteWriter, items: &[T], write_item: impl Fn(&mut dsl::ByteWriter, &T)) {
    w.write_varint_u64(items.len() as u64);
    for item in items {
        write_item(w, item);
    }
}

/// 🌳 `GltfJson` -- genuinely recursive real binary: tag `u8` (0=Null,1=Bool,2=Number,3=String,
/// 4=Array,5=Object) then the payload, matching `enc_json`/`dec_json`'s own tag scheme.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_json(w: &mut dsl::ByteWriter, v: &GltfJson) {
    match v {
        GltfJson::Null => w.write_u8(0),
        GltfJson::Bool(b) => {
            w.write_u8(1);
            w.write_u8(if *b { 1 } else { 0 });
        }
        GltfJson::Number(n) => {
            w.write_u8(2);
            w.write_f64_le(*n);
        }
        GltfJson::String(s) => {
            w.write_u8(3);
            write_bin_str(w, s);
        }
        GltfJson::Array(items) => {
            w.write_u8(4);
            write_bin_vec(w, items, write_bin_json);
        }
        GltfJson::Object(members) => {
            w.write_u8(5);
            write_bin_vec(w, members, |w, (k, v): &(String, GltfJson)| {
                write_bin_str(w, k);
                write_bin_json(w, v);
            });
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_json_opt(w: &mut dsl::ByteWriter, v: &Option<GltfJson>) {
    write_bin_option(w, v, write_bin_json);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_texture_info(w: &mut dsl::ByteWriter, v: &GltfTextureInfo) {
    w.write_varint_u64(v.index as u64);
    w.write_varint_u64(v.tex_coord);
    write_bin_json_opt(w, &v.extensions);
    write_bin_json_opt(w, &v.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_normal_texture_info(w: &mut dsl::ByteWriter, v: &GltfNormalTextureInfo) {
    w.write_varint_u64(v.index as u64);
    w.write_varint_u64(v.tex_coord);
    w.write_f64_le(v.scale);
    write_bin_json_opt(w, &v.extensions);
    write_bin_json_opt(w, &v.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_occlusion_texture_info(w: &mut dsl::ByteWriter, v: &GltfOcclusionTextureInfo) {
    w.write_varint_u64(v.index as u64);
    w.write_varint_u64(v.tex_coord);
    w.write_f64_le(v.strength);
    write_bin_json_opt(w, &v.extensions);
    write_bin_json_opt(w, &v.extras);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_texture(w: &mut dsl::ByteWriter, t: &GltfTexture) {
    write_bin_option(w, &t.sampler, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &t.source, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &t.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &t.extensions);
    write_bin_json_opt(w, &t.extras);
}

impl protocol::DiffBinary for GltfDiff {
/// ⚡️ P2-FG3: real binary diff-frame — upgraded from the F6-era `print_diff().into_bytes()`
/// text-as-binary shortcut (100% of stdio's `DiffCodec` impls were still on that shortcut per
/// the P2-W0 census; the FG1 wave's own closer report flagged leaving this un-upgraded as a
/// real defect to not repeat, and FG2's gif89a upgrade is this file's literal template).
/// Matches `../💾️binary/📡️.protocol.semio`'s real flag-per-field layout exactly,
/// field for field, in `GltfDiff`'s own struct declaration order (2-way flag for plain
/// `Option<T>` fields, 3-way flag for the 3 tri-state fields `scene`/`extensions`/`extras`).
/// Every one of the 14 collection fields is one length-prefixed blob wrapping its own real
/// binary `removed`/`modified`/`added` encoding (`write_bin_collection_blob`) — the blob's
/// OWN internal shape isn't further protocol-walkable (`Prim::Ref` recursion gap), but this
/// Rust side IS genuinely, fully structured real binary throughout, never text-as-bytes.
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut w = dsl::ByteWriter::new();
    // `asset`/`extensions_used`/`extensions_required`/`extensions`/`extras` are each wrapped
    // in a length-prefixed blob (matching `../💾️binary/📡️.protocol.semio`'s
    // `Array(u8, Field(<name>_len))` shape exactly) — NOT bare-inline like `scene`/
    // `source_form`'s fixed-width payloads — because they are NOT the last field in the frame
    // and their own internal shape has no fixed width `walk_protocol` could otherwise skip
    // past without knowing its byte length up front.
    write_bin_option(&mut w, &self.asset, |w, v| {
        write_bin_blob(w, &{
            let mut inner = dsl::ByteWriter::new();
            write_bin_asset_diff(&mut inner, v);
            inner.into_bytes()
        });
    });
    write_bin_tri(&mut w, &self.scene, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(&mut w, &self.scenes, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_scene, write_bin_scene_diff)));
    write_bin_option(&mut w, &self.nodes, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_node, write_bin_node_diff)));
    write_bin_option(&mut w, &self.meshes, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_mesh, write_bin_mesh_diff)));
    write_bin_option(&mut w, &self.accessors, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_accessor, write_bin_accessor_diff)));
    write_bin_option(&mut w, &self.buffer_views, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_buffer_view, write_bin_buffer_view)));
    write_bin_option(&mut w, &self.buffers, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_buffer, write_bin_buffer_diff)));
    write_bin_option(&mut w, &self.buffer_bytes, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, |w, b: &Vec<u8>| write_bin_blob(w, b), |w, b: &Vec<u8>| write_bin_blob(w, b))));
    write_bin_option(&mut w, &self.materials, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_material, write_bin_material_diff)));
    write_bin_option(&mut w, &self.textures, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_texture, write_bin_texture)));
    write_bin_option(&mut w, &self.images, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_image, write_bin_image)));
    write_bin_option(&mut w, &self.samplers, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_sampler, write_bin_sampler)));
    write_bin_option(&mut w, &self.skins, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_skin, write_bin_skin)));
    write_bin_option(&mut w, &self.animations, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_animation, write_bin_animation)));
    write_bin_option(&mut w, &self.cameras, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_camera, write_bin_camera)));
    write_bin_option(&mut w, &self.extensions_used, |w, v| {
        write_bin_blob(w, &{
            let mut inner = dsl::ByteWriter::new();
            write_bin_string_vec(&mut inner, v);
            inner.into_bytes()
        });
    });
    write_bin_option(&mut w, &self.extensions_required, |w, v| {
        write_bin_blob(w, &{
            let mut inner = dsl::ByteWriter::new();
            write_bin_string_vec(&mut inner, v);
            inner.into_bytes()
        });
    });
    write_bin_tri(&mut w, &self.extensions, |w, v| {
        write_bin_blob(w, &{
            let mut inner = dsl::ByteWriter::new();
            write_bin_json(&mut inner, v);
            inner.into_bytes()
        });
    });
    write_bin_tri(&mut w, &self.extras, |w, v| {
        write_bin_blob(w, &{
            let mut inner = dsl::ByteWriter::new();
            write_bin_json(&mut inner, v);
            inner.into_bytes()
        });
    });
    write_bin_option(&mut w, &self.source_form, |w, v| write_bin_source_form(w, *v));
    Ok(w.into_bytes())
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut r = dsl::ByteReader::new(bytes);
    let asset = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        let mut inner = dsl::ByteReader::new(&b);
        read_bin_asset_diff(&mut inner)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let scene = read_bin_tri(&mut r, |r| Ok(r.read_varint_u64()? as usize)).map_err(|error| gltf_bin_err(&error))?;
    let scenes = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_scene, read_bin_scene_diff)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let nodes = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_node, read_bin_node_diff)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let meshes = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_mesh, read_bin_mesh_diff)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let accessors = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_accessor, read_bin_accessor_diff)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let buffer_views = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_buffer_view, read_bin_buffer_view)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let buffers = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_buffer, read_bin_buffer_diff)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let buffer_bytes = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_blob, read_bin_blob)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let materials = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_material, read_bin_material_diff)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let textures = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_texture, read_bin_texture)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let images = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_image, read_bin_image)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let samplers = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_sampler, read_bin_sampler)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let skins = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_skin, read_bin_skin)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let animations = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_animation, read_bin_animation)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let cameras = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_camera, read_bin_camera)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let extensions_used = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        let mut inner = dsl::ByteReader::new(&b);
        read_bin_string_vec(&mut inner)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let extensions_required = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        let mut inner = dsl::ByteReader::new(&b);
        read_bin_string_vec(&mut inner)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let extensions = read_bin_tri(&mut r, |r| {
        let b = read_bin_blob(r)?;
        let mut inner = dsl::ByteReader::new(&b);
        read_bin_json(&mut inner)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let extras = read_bin_tri(&mut r, |r| {
        let b = read_bin_blob(r)?;
        let mut inner = dsl::ByteReader::new(&b);
        read_bin_json(&mut inner)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let source_form = read_bin_option(&mut r, read_bin_source_form).map_err(|error| gltf_bin_err(&error))?;
    Ok(GltfDiff { asset, scene, scenes, nodes, meshes, accessors, buffer_views, buffers, buffer_bytes, materials, textures, images, samplers, skins, animations, cameras, extensions_used, extensions_required, extensions, extras, source_form })
}
}
}
pub use diff_codec::*;
