//! 📝️ Text representation codec surface for `stdio.gltf` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");


#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v2_0::subsets::any::schema::diff::*;
use protocol::{DiffText,DiffBinary};
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
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}

/// 🎲️ `Option<Option<T>>` tri-state -- outer layer is peeled by the CALLER (top-level line
/// tokenizing decides "token present or not" for a change slot); this helper handles the INNER
/// layer when the tri-state value itself is embedded as a single positional field inside a larger
/// bracketed tuple (e.g. one field of `GltfAssetDiff`/`GltfNodeDiff`), where both layers must be
/// explicit since there is no "absent token" to lean on.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_option_option<T>(opt: &Option<Option<T>>, enc: impl Fn(&T) -> String) -> String {
    encode_option(opt, |inner: &Option<T>| encode_option(inner, &enc))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_option_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<Option<T>>, String> {
    decode_option(s, |inner: &str| decode_option(inner, &dec))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_f64(v: f64) -> String {
    v.to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_f64(s: &str) -> Result<f64, String> {
    s.parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_u64(v: u64) -> String {
    v.to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_u64(s: &str) -> Result<u64, String> {
    s.parse::<u64>().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_bool(v: bool) -> String {
    if v {
        "1".to_string()
    } else {
        "0".to_string()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_bool(s: &str) -> Result<bool, String> {
    match s {
        "0" => Ok(false),
        "1" => Ok(true),
        other => Err(format!("bool: expected 0/1, got {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_f64_slice(v: &[f64]) -> String {
    format!("[{}]", v.iter().map(|x| enc_f64(*x)).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_f64_vec(s: &str) -> Result<Vec<f64>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_f64).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_f64_array<const N: usize>(s: &str) -> Result<[f64; N], String> {
    let v = dec_f64_vec(s)?;
    let len = v.len();
    v.try_into().map_err(|_| format!("expected {N} floats, got {len}"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_usize_vec(v: &[usize]) -> String {
    format!("[{}]", v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_usize_vec(s: &str) -> Result<Vec<usize>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect()
}

/// 🏷️ `GltfPrimitive::attributes` -- `Vec<(String, usize)>`, name-keyed and order-preserving.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_attr_pairs(v: &[(String, usize)]) -> String {
    format!("[{}]", v.iter().map(|(k, idx)| format!("{}:{idx}", enc_str(k))).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_attr_pairs(s: &str) -> Result<Vec<(String, usize)>, String> {
    split_top_level(strip_brackets(s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (k, v) = entry.split_once(':').ok_or_else(|| format!("attr pair: bad entry {entry:?}"))?;
            Ok((dec_str(k)?, parse_usize(v)?))
        })
        .collect()
}

/// 🌳 `GltfJson` -- this artifact's own local `extras`/`extensions` value enum (F4). Tag prefix:
/// `Z`=Null (bare, no payload), `B[0|1]`=Bool, `F[<f64>]`=Number, `S[<hex>]`=String,
/// `A[v,v,...]`=Array, `O[k:v,k:v,...]`=Object (member order preserved, matching `GltfJson::
/// Object`'s own `Vec<(String,GltfJson)>` shape rather than a map).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_json(v: &GltfJson) -> String {
    match v {
        GltfJson::Null => "Z".to_string(),
        GltfJson::Bool(b) => format!("B[{}]", enc_bool(*b)),
        GltfJson::Number(n) => format!("F[{}]", enc_f64(*n)),
        GltfJson::String(s) => format!("S[{}]", enc_str(s)),
        GltfJson::Array(items) => format!("A[{}]", items.iter().map(enc_json).collect::<Vec<_>>().join(",")),
        GltfJson::Object(members) => {
            format!("O[{}]", members.iter().map(|(k, v)| format!("{}:{}", enc_str(k), enc_json(v))).collect::<Vec<_>>().join(","))
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_json(s: &str) -> Result<GltfJson, String> {
    if s == "Z" {
        return Ok(GltfJson::Null);
    }
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "B" => Ok(GltfJson::Bool(dec_bool(inner)?)),
        "F" => Ok(GltfJson::Number(dec_f64(inner)?)),
        "S" => Ok(GltfJson::String(dec_str(inner)?)),
        "A" => Ok(GltfJson::Array(split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_json).collect::<Result<Vec<_>, String>>()?)),
        "O" => Ok(GltfJson::Object(
            split_top_level(inner, ',')
                .into_iter()
                .filter(|s| !s.is_empty())
                .map(|entry| {
                    let (k, v) = entry.split_once(':').ok_or_else(|| format!("json object entry: bad {entry:?}"))?;
                    Ok((dec_str(k)?, dec_json(v)?))
                })
                .collect::<Result<Vec<_>, String>>()?,
        )),
        other => Err(format!("json: unknown tag {other:?}")),
    }
}

/// 🔢️ Wire code, not a word tag -- reuses [`GltfComponentType::code`]/`from_code` (the same spec
/// numeric code the JSON serde impl uses, `crate::engine`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_component_type(t: GltfComponentType) -> String {
    t.code().to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_component_type(s: &str) -> Result<GltfComponentType, String> {
    GltfComponentType::from_code(dec_u64(s)?)
}

/// 🔤️ Word tag -- reuses [`GltfAccessorType::as_str`]/`from_str`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_accessor_type(t: GltfAccessorType) -> String {
    t.as_str().to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_accessor_type(s: &str) -> Result<GltfAccessorType, String> {
    s.parse()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_alpha_mode(m: GltfAlphaMode) -> String {
    match m {
        GltfAlphaMode::Opaque => "OPAQUE",
        GltfAlphaMode::Mask => "MASK",
        GltfAlphaMode::Blend => "BLEND",
    }
    .to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_alpha_mode(s: &str) -> Result<GltfAlphaMode, String> {
    match s {
        "OPAQUE" => Ok(GltfAlphaMode::Opaque),
        "MASK" => Ok(GltfAlphaMode::Mask),
        "BLEND" => Ok(GltfAlphaMode::Blend),
        other => Err(format!("alpha mode: unknown {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_interpolation(i: GltfInterpolation) -> String {
    match i {
        GltfInterpolation::Linear => "LINEAR",
        GltfInterpolation::Step => "STEP",
        GltfInterpolation::CubicSpline => "CUBICSPLINE",
    }
    .to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_interpolation(s: &str) -> Result<GltfInterpolation, String> {
    match s {
        "LINEAR" => Ok(GltfInterpolation::Linear),
        "STEP" => Ok(GltfInterpolation::Step),
        "CUBICSPLINE" => Ok(GltfInterpolation::CubicSpline),
        other => Err(format!("interpolation: unknown {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_animation_path(p: GltfAnimationPath) -> String {
    match p {
        GltfAnimationPath::Translation => "translation",
        GltfAnimationPath::Rotation => "rotation",
        GltfAnimationPath::Scale => "scale",
        GltfAnimationPath::Weights => "weights",
    }
    .to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_animation_path(s: &str) -> Result<GltfAnimationPath, String> {
    match s {
        "translation" => Ok(GltfAnimationPath::Translation),
        "rotation" => Ok(GltfAnimationPath::Rotation),
        "scale" => Ok(GltfAnimationPath::Scale),
        "weights" => Ok(GltfAnimationPath::Weights),
        other => Err(format!("animation path: unknown {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_source_form(f: GltfSourceForm) -> String {
    match f {
        GltfSourceForm::Json => "json",
        GltfSourceForm::Glb => "glb",
    }
    .to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_source_form(s: &str) -> Result<GltfSourceForm, String> {
    match s {
        "json" => Ok(GltfSourceForm::Json),
        "glb" => Ok(GltfSourceForm::Glb),
        other => Err(format!("source form: unknown {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_asset_diff(d: &GltfAssetDiff) -> String {
    format!(
        "[{},{},{},{},{},{}]",
        encode_option(&d.version, |v| enc_str(v)),
        encode_option_option(&d.generator, |v| enc_str(v)),
        encode_option_option(&d.copyright, |v| enc_str(v)),
        encode_option_option(&d.min_version, |v| enc_str(v)),
        encode_option_option(&d.extensions, enc_json),
        encode_option_option(&d.extras, enc_json),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_asset_diff(s: &str) -> Result<GltfAssetDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [version, generator, copyright, min_version, extensions, extras] = parts.as_slice() else {
        return Err(format!("asset diff: expected 6 fields, got {}", parts.len()));
    };
    Ok(GltfAssetDiff {
        version: decode_option(version, dec_str)?,
        generator: decode_option_option(generator, dec_str)?,
        copyright: decode_option_option(copyright, dec_str)?,
        min_version: decode_option_option(min_version, dec_str)?,
        extensions: decode_option_option(extensions, dec_json)?,
        extras: decode_option_option(extras, dec_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_scene(sc: &GltfScene) -> String {
    format!("[{},{},{},{}]", enc_usize_vec(&sc.nodes), encode_option(&sc.name, |v| enc_str(v)), encode_option(&sc.extensions, enc_json), encode_option(&sc.extras, enc_json),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_scene(s: &str) -> Result<GltfScene, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [nodes, name, extensions, extras] = parts.as_slice() else { return Err(format!("scene: expected 4 fields, got {}", parts.len())) };
    Ok(GltfScene { nodes: dec_usize_vec(nodes)?, name: decode_option(name, dec_str)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_scene_diff(d: &GltfSceneDiff) -> String {
    format!("[{},{},{},{}]", encode_option(&d.nodes, enc_refs_delta), encode_option_option(&d.name, |v| enc_str(v)), encode_option_option(&d.extensions, enc_json), encode_option_option(&d.extras, enc_json),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_scene_diff(s: &str) -> Result<GltfSceneDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [nodes, name, extensions, extras] = parts.as_slice() else { return Err(format!("scene diff: expected 4 fields, got {}", parts.len())) };
    Ok(GltfSceneDiff { nodes: decode_option(nodes, dec_refs_delta)?, name: decode_option_option(name, dec_str)?, extensions: decode_option_option(extensions, dec_json)?, extras: decode_option_option(extras, dec_json)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_node(n: &GltfNode) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{}]",
        enc_usize_vec(&n.children),
        encode_option(&n.mesh, |v| v.to_string()),
        encode_option(&n.camera, |v| v.to_string()),
        encode_option(&n.skin, |v| v.to_string()),
        encode_option(&n.matrix, |v| enc_f64_slice(v)),
        encode_option(&n.translation, |v| enc_f64_slice(v)),
        encode_option(&n.rotation, |v| enc_f64_slice(v)),
        encode_option(&n.scale, |v| enc_f64_slice(v)),
        enc_f64_slice(&n.weights),
        encode_option(&n.name, |v| enc_str(v)),
        encode_option(&n.extensions, enc_json),
        encode_option(&n.extras, enc_json),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node(s: &str) -> Result<GltfNode, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [children, mesh, camera, skin, matrix, translation, rotation, scale, weights, name, extensions, extras] = parts.as_slice() else {
        return Err(format!("node: expected 12 fields, got {}", parts.len()));
    };
    Ok(GltfNode {
        children: dec_usize_vec(children)?,
        mesh: decode_option(mesh, parse_usize)?,
        camera: decode_option(camera, parse_usize)?,
        skin: decode_option(skin, parse_usize)?,
        matrix: decode_option(matrix, dec_f64_array::<16>)?,
        translation: decode_option(translation, dec_f64_array::<3>)?,
        rotation: decode_option(rotation, dec_f64_array::<4>)?,
        scale: decode_option(scale, dec_f64_array::<3>)?,
        weights: dec_f64_vec(weights)?,
        name: decode_option(name, dec_str)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_node_diff(d: &GltfNodeDiff) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{}]",
        encode_option(&d.children, enc_refs_delta),
        encode_option_option(&d.mesh, |v| v.to_string()),
        encode_option_option(&d.camera, |v| v.to_string()),
        encode_option_option(&d.skin, |v| v.to_string()),
        encode_option_option(&d.matrix, |v| enc_f64_slice(v)),
        encode_option_option(&d.translation, |v| enc_f64_slice(v)),
        encode_option_option(&d.rotation, |v| enc_f64_slice(v)),
        encode_option_option(&d.scale, |v| enc_f64_slice(v)),
        encode_option(&d.weights, |v| enc_f64_slice(v)),
        encode_option_option(&d.name, |v| enc_str(v)),
        encode_option_option(&d.extensions, enc_json),
        encode_option_option(&d.extras, enc_json),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node_diff(s: &str) -> Result<GltfNodeDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [children, mesh, camera, skin, matrix, translation, rotation, scale, weights, name, extensions, extras] = parts.as_slice() else {
        return Err(format!("node diff: expected 12 fields, got {}", parts.len()));
    };
    Ok(GltfNodeDiff {
        children: decode_option(children, dec_refs_delta)?,
        mesh: decode_option_option(mesh, parse_usize)?,
        camera: decode_option_option(camera, parse_usize)?,
        skin: decode_option_option(skin, parse_usize)?,
        matrix: decode_option_option(matrix, dec_f64_array::<16>)?,
        translation: decode_option_option(translation, dec_f64_array::<3>)?,
        rotation: decode_option_option(rotation, dec_f64_array::<4>)?,
        scale: decode_option_option(scale, dec_f64_array::<3>)?,
        weights: decode_option(weights, dec_f64_vec)?,
        name: decode_option_option(name, dec_str)?,
        extensions: decode_option_option(extensions, dec_json)?,
        extras: decode_option_option(extras, dec_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_primitive(p: &GltfPrimitive) -> String {
    format!(
        "[{},{},{},{},{},{},{}]",
        enc_attr_pairs(&p.attributes),
        encode_option(&p.indices, |v| v.to_string()),
        encode_option(&p.material, |v| v.to_string()),
        encode_option(&p.mode, |v| enc_u64(*v)),
        format_args!("[{}]", p.targets.iter().map(|target| enc_attr_pairs(&target.0)).collect::<Vec<_>>().join(",")),
        encode_option(&p.extensions, enc_json),
        encode_option(&p.extras, enc_json),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_primitive(s: &str) -> Result<GltfPrimitive, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [attributes, indices, material, mode, targets, extensions, extras] = parts.as_slice() else {
        return Err(format!("primitive: expected 7 fields, got {}", parts.len()));
    };
    Ok(GltfPrimitive {
        attributes: dec_attr_pairs(attributes)?,
        indices: decode_option(indices, parse_usize)?,
        material: decode_option(material, parse_usize)?,
        mode: decode_option(mode, dec_u64)?,
        targets: split_top_level(strip_brackets(targets)?, ',').into_iter().filter(|value| !value.is_empty()).map(|value| dec_attr_pairs(value).map(GltfMorphTarget)).collect::<Result<Vec<_>, _>>()?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_primitive_vec(v: &[GltfPrimitive]) -> String {
    format!("[{}]", v.iter().map(enc_primitive).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_primitive_vec(s: &str) -> Result<Vec<GltfPrimitive>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_primitive).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_mesh(m: &GltfMesh) -> String {
    format!("[{},{},{},{},{}]", enc_primitive_vec(&m.primitives), enc_f64_slice(&m.weights), encode_option(&m.name, |v| enc_str(v)), encode_option(&m.extensions, enc_json), encode_option(&m.extras, enc_json),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_mesh(s: &str) -> Result<GltfMesh, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [primitives, weights, name, extensions, extras] = parts.as_slice() else { return Err(format!("mesh: expected 5 fields, got {}", parts.len())) };
    Ok(GltfMesh { primitives: dec_primitive_vec(primitives)?, weights: dec_f64_vec(weights)?, name: decode_option(name, dec_str)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_mesh_diff(d: &GltfMeshDiff) -> String {
    format!(
        "[{},{},{},{},{}]",
        encode_option(&d.primitives, |v| enc_collection(v, enc_primitive, enc_primitive_diff)),
        encode_option(&d.weights, |v| enc_f64_slice(v)),
        encode_option_option(&d.name, |v| enc_str(v)),
        encode_option_option(&d.extensions, enc_json),
        encode_option_option(&d.extras, enc_json),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_mesh_diff(s: &str) -> Result<GltfMeshDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [primitives, weights, name, extensions, extras] = parts.as_slice() else { return Err(format!("mesh diff: expected 5 fields, got {}", parts.len())) };
    Ok(GltfMeshDiff {
        primitives: decode_option(primitives, |s| dec_collection(s, dec_primitive, dec_primitive_diff))?,
        weights: decode_option(weights, dec_f64_vec)?,
        name: decode_option_option(name, dec_str)?,
        extensions: decode_option_option(extensions, dec_json)?,
        extras: decode_option_option(extras, dec_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_sparse_indices(v: &GltfSparseIndices) -> String {
    format!("[{},{},{}]", v.buffer_view, v.byte_offset, enc_component_type(v.component_type))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_sparse_indices(s: &str) -> Result<GltfSparseIndices, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [buffer_view, byte_offset, component_type] = parts.as_slice() else { return Err(format!("sparse indices: expected 3 fields, got {}", parts.len())) };
    Ok(GltfSparseIndices { buffer_view: parse_usize(buffer_view)?, byte_offset: parse_usize(byte_offset)?, component_type: dec_component_type(component_type)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_sparse_values(v: &GltfSparseValues) -> String {
    format!("[{},{}]", v.buffer_view, v.byte_offset)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_sparse_values(s: &str) -> Result<GltfSparseValues, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [buffer_view, byte_offset] = parts.as_slice() else { return Err(format!("sparse values: expected 2 fields, got {}", parts.len())) };
    Ok(GltfSparseValues { buffer_view: parse_usize(buffer_view)?, byte_offset: parse_usize(byte_offset)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_sparse_accessor(v: &GltfSparseAccessor) -> String {
    format!("[{},{},{}]", v.count, enc_sparse_indices(&v.indices), enc_sparse_values(&v.values))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_sparse_accessor(s: &str) -> Result<GltfSparseAccessor, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [count, indices, values] = parts.as_slice() else { return Err(format!("sparse accessor: expected 3 fields, got {}", parts.len())) };
    Ok(GltfSparseAccessor { count: parse_usize(count)?, indices: dec_sparse_indices(indices)?, values: dec_sparse_values(values)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_accessor(a: &GltfAccessor) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{}]",
        encode_option(&a.buffer_view, |v| v.to_string()),
        a.byte_offset,
        enc_component_type(a.component_type),
        enc_bool(a.normalized),
        a.count,
        enc_accessor_type(a.kind),
        encode_option(&a.max, |v| enc_f64_slice(v)),
        encode_option(&a.min, |v| enc_f64_slice(v)),
        encode_option(&a.sparse, enc_sparse_accessor),
        encode_option(&a.name, |v| enc_str(v)),
        encode_option(&a.extensions, enc_json),
        encode_option(&a.extras, enc_json),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_accessor(s: &str) -> Result<GltfAccessor, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [buffer_view, byte_offset, component_type, normalized, count, kind, max, min, sparse, name, extensions, extras] = parts.as_slice() else {
        return Err(format!("accessor: expected 12 fields, got {}", parts.len()));
    };
    Ok(GltfAccessor {
        buffer_view: decode_option(buffer_view, parse_usize)?,
        byte_offset: parse_usize(byte_offset)?,
        component_type: dec_component_type(component_type)?,
        normalized: dec_bool(normalized)?,
        count: parse_usize(count)?,
        kind: dec_accessor_type(kind)?,
        max: decode_option(max, dec_f64_vec)?,
        min: decode_option(min, dec_f64_vec)?,
        sparse: decode_option(sparse, dec_sparse_accessor)?,
        name: decode_option(name, dec_str)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_accessor_diff(d: &GltfAccessorDiff) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{}]",
        encode_option_option(&d.buffer_view, |v| v.to_string()),
        encode_option(&d.byte_offset, |v| v.to_string()),
        encode_option(&d.component_type, |v| enc_component_type(*v)),
        encode_option(&d.normalized, |v| enc_bool(*v)),
        encode_option(&d.count, |v| v.to_string()),
        encode_option(&d.kind, |v| enc_accessor_type(*v)),
        encode_option_option(&d.max, |v| enc_f64_slice(v)),
        encode_option_option(&d.min, |v| enc_f64_slice(v)),
        encode_option_option(&d.sparse, enc_sparse_accessor),
        encode_option_option(&d.name, |v| enc_str(v)),
        encode_option_option(&d.extensions, enc_json),
        encode_option_option(&d.extras, enc_json),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_accessor_diff(s: &str) -> Result<GltfAccessorDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [buffer_view, byte_offset, component_type, normalized, count, kind, max, min, sparse, name, extensions, extras] = parts.as_slice() else {
        return Err(format!("accessor diff: expected 12 fields, got {}", parts.len()));
    };
    Ok(GltfAccessorDiff {
        buffer_view: decode_option_option(buffer_view, parse_usize)?,
        byte_offset: decode_option(byte_offset, parse_usize)?,
        component_type: decode_option(component_type, dec_component_type)?,
        normalized: decode_option(normalized, dec_bool)?,
        count: decode_option(count, parse_usize)?,
        kind: decode_option(kind, dec_accessor_type)?,
        max: decode_option_option(max, dec_f64_vec)?,
        min: decode_option_option(min, dec_f64_vec)?,
        sparse: decode_option_option(sparse, dec_sparse_accessor)?,
        name: decode_option_option(name, dec_str)?,
        extensions: decode_option_option(extensions, dec_json)?,
        extras: decode_option_option(extras, dec_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_texture_info(v: &GltfTextureInfo) -> String {
    format!("[{},{},{},{}]", v.index, enc_u64(v.tex_coord), encode_option(&v.extensions, enc_json), encode_option(&v.extras, enc_json))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_texture_info(s: &str) -> Result<GltfTextureInfo, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [index, tex_coord, extensions, extras] = parts.as_slice() else { return Err(format!("texture info: expected 4 fields, got {}", parts.len())) };
    Ok(GltfTextureInfo { index: parse_usize(index)?, tex_coord: dec_u64(tex_coord)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_normal_texture_info(v: &GltfNormalTextureInfo) -> String {
    format!("[{},{},{},{},{}]", v.index, enc_u64(v.tex_coord), enc_f64(v.scale), encode_option(&v.extensions, enc_json), encode_option(&v.extras, enc_json))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_normal_texture_info(s: &str) -> Result<GltfNormalTextureInfo, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [index, tex_coord, scale, extensions, extras] = parts.as_slice() else { return Err(format!("normal texture info: expected 5 fields, got {}", parts.len())) };
    Ok(GltfNormalTextureInfo { index: parse_usize(index)?, tex_coord: dec_u64(tex_coord)?, scale: dec_f64(scale)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_occlusion_texture_info(v: &GltfOcclusionTextureInfo) -> String {
    format!("[{},{},{},{},{}]", v.index, enc_u64(v.tex_coord), enc_f64(v.strength), encode_option(&v.extensions, enc_json), encode_option(&v.extras, enc_json))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_occlusion_texture_info(s: &str) -> Result<GltfOcclusionTextureInfo, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [index, tex_coord, strength, extensions, extras] = parts.as_slice() else { return Err(format!("occlusion texture info: expected 5 fields, got {}", parts.len())) };
    Ok(GltfOcclusionTextureInfo { index: parse_usize(index)?, tex_coord: dec_u64(tex_coord)?, strength: dec_f64(strength)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_pbr(v: &GltfPbrMetallicRoughness) -> String {
    format!(
        "[{},{},{},{},{},{},{}]",
        enc_f64_slice(&v.base_color_factor),
        encode_option(&v.base_color_texture, enc_texture_info),
        enc_f64(v.metallic_factor),
        enc_f64(v.roughness_factor),
        encode_option(&v.metallic_roughness_texture, enc_texture_info),
        encode_option(&v.extensions, enc_json),
        encode_option(&v.extras, enc_json),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_pbr(s: &str) -> Result<GltfPbrMetallicRoughness, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [base_color_factor, base_color_texture, metallic_factor, roughness_factor, metallic_roughness_texture, extensions, extras] = parts.as_slice() else {
        return Err(format!("pbr: expected 7 fields, got {}", parts.len()));
    };
    Ok(GltfPbrMetallicRoughness {
        base_color_factor: dec_f64_array::<4>(base_color_factor)?,
        base_color_texture: decode_option(base_color_texture, dec_texture_info)?,
        metallic_factor: dec_f64(metallic_factor)?,
        roughness_factor: dec_f64(roughness_factor)?,
        metallic_roughness_texture: decode_option(metallic_roughness_texture, dec_texture_info)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_material(m: &GltfMaterial) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{}]",
        encode_option(&m.name, |v| enc_str(v)),
        encode_option(&m.pbr_metallic_roughness, enc_pbr),
        encode_option(&m.normal_texture, enc_normal_texture_info),
        encode_option(&m.occlusion_texture, enc_occlusion_texture_info),
        encode_option(&m.emissive_texture, enc_texture_info),
        enc_f64_slice(&m.emissive_factor),
        enc_alpha_mode(m.alpha_mode),
        enc_f64(m.alpha_cutoff),
        enc_bool(m.double_sided),
        encode_option(&m.extensions, enc_json),
        encode_option(&m.extras, enc_json),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_material(s: &str) -> Result<GltfMaterial, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, pbr, normal_texture, occlusion_texture, emissive_texture, emissive_factor, alpha_mode, alpha_cutoff, double_sided, extensions, extras] = parts.as_slice() else {
        return Err(format!("material: expected 11 fields, got {}", parts.len()));
    };
    Ok(GltfMaterial {
        name: decode_option(name, dec_str)?,
        pbr_metallic_roughness: decode_option(pbr, dec_pbr)?,
        normal_texture: decode_option(normal_texture, dec_normal_texture_info)?,
        occlusion_texture: decode_option(occlusion_texture, dec_occlusion_texture_info)?,
        emissive_texture: decode_option(emissive_texture, dec_texture_info)?,
        emissive_factor: dec_f64_array::<3>(emissive_factor)?,
        alpha_mode: dec_alpha_mode(alpha_mode)?,
        alpha_cutoff: dec_f64(alpha_cutoff)?,
        double_sided: dec_bool(double_sided)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_material_diff(d: &GltfMaterialDiff) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{}]",
        encode_option_option(&d.name, |v| enc_str(v)),
        encode_option_option(&d.pbr_metallic_roughness, enc_pbr),
        encode_option_option(&d.normal_texture, enc_normal_texture_info),
        encode_option_option(&d.occlusion_texture, enc_occlusion_texture_info),
        encode_option_option(&d.emissive_texture, enc_texture_info),
        encode_option(&d.emissive_factor, |v| enc_f64_slice(v)),
        encode_option(&d.alpha_mode, |v| enc_alpha_mode(*v)),
        encode_option(&d.alpha_cutoff, |v| enc_f64(*v)),
        encode_option(&d.double_sided, |v| enc_bool(*v)),
        encode_option_option(&d.extensions, enc_json),
        encode_option_option(&d.extras, enc_json),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_material_diff(s: &str) -> Result<GltfMaterialDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, pbr, normal_texture, occlusion_texture, emissive_texture, emissive_factor, alpha_mode, alpha_cutoff, double_sided, extensions, extras] = parts.as_slice() else {
        return Err(format!("material diff: expected 11 fields, got {}", parts.len()));
    };
    Ok(GltfMaterialDiff {
        name: decode_option_option(name, dec_str)?,
        pbr_metallic_roughness: decode_option_option(pbr, dec_pbr)?,
        normal_texture: decode_option_option(normal_texture, dec_normal_texture_info)?,
        occlusion_texture: decode_option_option(occlusion_texture, dec_occlusion_texture_info)?,
        emissive_texture: decode_option_option(emissive_texture, dec_texture_info)?,
        emissive_factor: decode_option(emissive_factor, dec_f64_array::<3>)?,
        alpha_mode: decode_option(alpha_mode, dec_alpha_mode)?,
        alpha_cutoff: decode_option(alpha_cutoff, dec_f64)?,
        double_sided: decode_option(double_sided, dec_bool)?,
        extensions: decode_option_option(extensions, dec_json)?,
        extras: decode_option_option(extras, dec_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_buffer(b: &GltfBuffer) -> String {
    format!("[{},{},{},{},{}]", b.byte_length, encode_option(&b.uri, |v| enc_str(v)), encode_option(&b.name, |v| enc_str(v)), encode_option(&b.extensions, enc_json), encode_option(&b.extras, enc_json))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_buffer(s: &str) -> Result<GltfBuffer, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [byte_length, uri, name, extensions, extras] = parts.as_slice() else { return Err(format!("buffer: expected 5 fields, got {}", parts.len())) };
    Ok(GltfBuffer { byte_length: parse_usize(byte_length)?, uri: decode_option(uri, dec_str)?, name: decode_option(name, dec_str)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_buffer_diff(d: &GltfBufferDiff) -> String {
    format!(
        "[{},{},{},{},{}]",
        encode_option(&d.byte_length, |v| v.to_string()),
        encode_option_option(&d.uri, |v| enc_str(v)),
        encode_option_option(&d.name, |v| enc_str(v)),
        encode_option_option(&d.extensions, enc_json),
        encode_option_option(&d.extras, enc_json),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_buffer_diff(s: &str) -> Result<GltfBufferDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [byte_length, uri, name, extensions, extras] = parts.as_slice() else { return Err(format!("buffer diff: expected 5 fields, got {}", parts.len())) };
    Ok(GltfBufferDiff {
        byte_length: decode_option(byte_length, parse_usize)?,
        uri: decode_option_option(uri, dec_str)?,
        name: decode_option_option(name, dec_str)?,
        extensions: decode_option_option(extensions, dec_json)?,
        extras: decode_option_option(extras, dec_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_buffer_view(v: &GltfBufferView) -> String {
    format!(
        "[{},{},{},{},{},{},{},{}]",
        v.buffer,
        v.byte_offset,
        v.byte_length,
        encode_option(&v.byte_stride, |x| x.to_string()),
        encode_option(&v.target, |x| enc_u64(*x)),
        encode_option(&v.name, |x| enc_str(x)),
        encode_option(&v.extensions, enc_json),
        encode_option(&v.extras, enc_json),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_buffer_view(s: &str) -> Result<GltfBufferView, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [buffer, byte_offset, byte_length, byte_stride, target, name, extensions, extras] = parts.as_slice() else {
        return Err(format!("buffer view: expected 8 fields, got {}", parts.len()));
    };
    Ok(GltfBufferView {
        buffer: parse_usize(buffer)?,
        byte_offset: parse_usize(byte_offset)?,
        byte_length: parse_usize(byte_length)?,
        byte_stride: decode_option(byte_stride, parse_usize)?,
        target: decode_option(target, dec_u64)?,
        name: decode_option(name, dec_str)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}

/// 🧬️ Raw buffer bytes (`GltfSnapshot::buffers[i]`) -- hex, same as every other byte payload in
/// this grammar (no base64: no external dep, matching the family's own hex idiom, see
/// `f6-recon-report.md` §5).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_bytes(v: &[u8]) -> String {
    hex_encode(v)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_bytes(s: &str) -> Result<Vec<u8>, String> {
    hex_decode(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_texture(t: &GltfTexture) -> String {
    format!("[{},{},{},{},{}]", encode_option(&t.sampler, |v| v.to_string()), encode_option(&t.source, |v| v.to_string()), encode_option(&t.name, |v| enc_str(v)), encode_option(&t.extensions, enc_json), encode_option(&t.extras, enc_json),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_texture(s: &str) -> Result<GltfTexture, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [sampler, source, name, extensions, extras] = parts.as_slice() else { return Err(format!("texture: expected 5 fields, got {}", parts.len())) };
    Ok(GltfTexture { sampler: decode_option(sampler, parse_usize)?, source: decode_option(source, parse_usize)?, name: decode_option(name, dec_str)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_image(i: &GltfImage) -> String {
    format!(
        "[{},{},{},{},{},{}]",
        encode_option(&i.uri, |v| enc_str(v)),
        encode_option(&i.mime_type, |v| enc_str(v)),
        encode_option(&i.buffer_view, |v| v.to_string()),
        encode_option(&i.name, |v| enc_str(v)),
        encode_option(&i.extensions, enc_json),
        encode_option(&i.extras, enc_json),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_image(s: &str) -> Result<GltfImage, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [uri, mime_type, buffer_view, name, extensions, extras] = parts.as_slice() else { return Err(format!("image: expected 6 fields, got {}", parts.len())) };
    Ok(GltfImage {
        uri: decode_option(uri, dec_str)?,
        mime_type: decode_option(mime_type, dec_str)?,
        buffer_view: decode_option(buffer_view, parse_usize)?,
        name: decode_option(name, dec_str)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_sampler(s: &GltfSampler) -> String {
    format!(
        "[{},{},{},{},{},{},{}]",
        encode_option(&s.mag_filter, |v| enc_u64(*v)),
        encode_option(&s.min_filter, |v| enc_u64(*v)),
        enc_u64(s.wrap_s),
        enc_u64(s.wrap_t),
        encode_option(&s.name, |v| enc_str(v)),
        encode_option(&s.extensions, enc_json),
        encode_option(&s.extras, enc_json),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_sampler(s: &str) -> Result<GltfSampler, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [mag_filter, min_filter, wrap_s, wrap_t, name, extensions, extras] = parts.as_slice() else {
        return Err(format!("sampler: expected 7 fields, got {}", parts.len()));
    };
    Ok(GltfSampler {
        mag_filter: decode_option(mag_filter, dec_u64)?,
        min_filter: decode_option(min_filter, dec_u64)?,
        wrap_s: dec_u64(wrap_s)?,
        wrap_t: dec_u64(wrap_t)?,
        name: decode_option(name, dec_str)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_skin(v: &GltfSkin) -> String {
    format!(
        "[{},{},{},{},{},{}]",
        encode_option(&v.inverse_bind_matrices, |x| x.to_string()),
        encode_option(&v.skeleton, |x| x.to_string()),
        enc_usize_vec(&v.joints),
        encode_option(&v.name, |x| enc_str(x)),
        encode_option(&v.extensions, enc_json),
        encode_option(&v.extras, enc_json),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_skin(s: &str) -> Result<GltfSkin, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [inverse_bind_matrices, skeleton, joints, name, extensions, extras] = parts.as_slice() else {
        return Err(format!("skin: expected 6 fields, got {}", parts.len()));
    };
    Ok(GltfSkin {
        inverse_bind_matrices: decode_option(inverse_bind_matrices, parse_usize)?,
        skeleton: decode_option(skeleton, parse_usize)?,
        joints: dec_usize_vec(joints)?,
        name: decode_option(name, dec_str)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_animation_channel_target(t: &GltfAnimationChannelTarget) -> String {
    format!("[{},{},{},{}]", encode_option(&t.node, |v| v.to_string()), enc_animation_path(t.path), encode_option(&t.extensions, enc_json), encode_option(&t.extras, enc_json))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_animation_channel_target(s: &str) -> Result<GltfAnimationChannelTarget, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [node, path, extensions, extras] = parts.as_slice() else { return Err(format!("animation channel target: expected 4 fields, got {}", parts.len())) };
    Ok(GltfAnimationChannelTarget { node: decode_option(node, parse_usize)?, path: dec_animation_path(path)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_animation_channel(c: &GltfAnimationChannel) -> String {
    format!("[{},{},{},{}]", c.sampler, enc_animation_channel_target(&c.target), encode_option(&c.extensions, enc_json), encode_option(&c.extras, enc_json))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_animation_channel(s: &str) -> Result<GltfAnimationChannel, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [sampler, target, extensions, extras] = parts.as_slice() else { return Err(format!("animation channel: expected 4 fields, got {}", parts.len())) };
    Ok(GltfAnimationChannel { sampler: parse_usize(sampler)?, target: dec_animation_channel_target(target)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_animation_sampler(s: &GltfAnimationSampler) -> String {
    format!("[{},{},{},{},{}]", s.input, enc_interpolation(s.interpolation), s.output, encode_option(&s.extensions, enc_json), encode_option(&s.extras, enc_json))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_animation_sampler(s: &str) -> Result<GltfAnimationSampler, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [input, interpolation, output, extensions, extras] = parts.as_slice() else { return Err(format!("animation sampler: expected 5 fields, got {}", parts.len())) };
    Ok(GltfAnimationSampler { input: parse_usize(input)?, interpolation: dec_interpolation(interpolation)?, output: parse_usize(output)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_animation(a: &GltfAnimation) -> String {
    format!(
        "[{},{},{},{},{}]",
        format_args!("[{}]", a.channels.iter().map(enc_animation_channel).collect::<Vec<_>>().join(",")),
        format_args!("[{}]", a.samplers.iter().map(enc_animation_sampler).collect::<Vec<_>>().join(",")),
        encode_option(&a.name, |v| enc_str(v)),
        encode_option(&a.extensions, enc_json),
        encode_option(&a.extras, enc_json),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_animation(s: &str) -> Result<GltfAnimation, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [channels, samplers, name, extensions, extras] = parts.as_slice() else { return Err(format!("animation: expected 5 fields, got {}", parts.len())) };
    Ok(GltfAnimation {
        channels: split_top_level(strip_brackets(channels)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_animation_channel).collect::<Result<Vec<_>, String>>()?,
        samplers: split_top_level(strip_brackets(samplers)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_animation_sampler).collect::<Result<Vec<_>, String>>()?,
        name: decode_option(name, dec_str)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_perspective(p: &GltfPerspective) -> String {
    format!("[{},{},{},{},{},{}]", encode_option(&p.aspect_ratio, |v| enc_f64(*v)), enc_f64(p.yfov), encode_option(&p.zfar, |v| enc_f64(*v)), enc_f64(p.znear), encode_option(&p.extensions, enc_json), encode_option(&p.extras, enc_json),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_perspective(s: &str) -> Result<GltfPerspective, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [aspect_ratio, yfov, zfar, znear, extensions, extras] = parts.as_slice() else { return Err(format!("perspective: expected 6 fields, got {}", parts.len())) };
    Ok(GltfPerspective {
        aspect_ratio: decode_option(aspect_ratio, dec_f64)?,
        yfov: dec_f64(yfov)?,
        zfar: decode_option(zfar, dec_f64)?,
        znear: dec_f64(znear)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_orthographic(o: &GltfOrthographic) -> String {
    format!("[{},{},{},{},{},{}]", enc_f64(o.xmag), enc_f64(o.ymag), enc_f64(o.zfar), enc_f64(o.znear), encode_option(&o.extensions, enc_json), encode_option(&o.extras, enc_json))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_orthographic(s: &str) -> Result<GltfOrthographic, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [xmag, ymag, zfar, znear, extensions, extras] = parts.as_slice() else { return Err(format!("orthographic: expected 6 fields, got {}", parts.len())) };
    Ok(GltfOrthographic { xmag: dec_f64(xmag)?, ymag: dec_f64(ymag)?, zfar: dec_f64(zfar)?, znear: dec_f64(znear)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}

/// 🔀️ `GltfCameraProjection` is a real data-carrying enum (§3a) -- tag prefix `P`=Perspective,
/// `O`=Orthographic.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_camera_projection(p: &GltfCameraProjection) -> String {
    match p {
        GltfCameraProjection::Perspective(v) => format!("P{}", enc_perspective(v)),
        GltfCameraProjection::Orthographic(v) => format!("O{}", enc_orthographic(v)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_camera_projection(s: &str) -> Result<GltfCameraProjection, String> {
    let (tag, rest) = s.split_at(1);
    match tag {
        "P" => Ok(GltfCameraProjection::Perspective(dec_perspective(rest)?)),
        "O" => Ok(GltfCameraProjection::Orthographic(dec_orthographic(rest)?)),
        other => Err(format!("camera projection: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_camera(c: &GltfCamera) -> String {
    format!("[{},{},{},{}]", enc_camera_projection(&c.projection), encode_option(&c.name, |v| enc_str(v)), encode_option(&c.extensions, enc_json), encode_option(&c.extras, enc_json))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_camera(s: &str) -> Result<GltfCamera, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [projection, name, extensions, extras] = parts.as_slice() else { return Err(format!("camera: expected 4 fields, got {}", parts.len())) };
    Ok(GltfCamera { projection: dec_camera_projection(projection)?, name: decode_option(name, dec_str)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}

/// 🧮️ Generic index-keyed collection triple codec, shared by every one of the 14 top-level arrays
/// (STRONG entities pass a real per-item diff encoder; WEAK entities pass the same `enc_item`/
/// `dec_item` for both `enc_item`/`enc_diff` via `GltfWeakCollectionDiff<T> = GltfCollectionDiff<T,
/// T>`) -- one real generic codec, not 14 hand-duplicated ones.
/// 🧷️ Text codec of one positional list delta: `[id@index,…];[index:row,…];[id@from>to,…]` -- removed ids with their base positions,
/// inserted rows with their after positions, moved ids with both.
macro_rules! delta_text_codec {
    ($enc:ident, $dec:ident, $delta:ident, $removal:ident, $insertion:ident, $relocation:ident, $enc_id:expr, $dec_id:expr, $enc_row:expr, $dec_row:expr) => {
        // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
        pub(crate) fn $enc(delta: &$delta) -> String {
            let removed = delta.removed.iter().map(|entry| format!("{}@{}", $enc_id(&entry.id), entry.index)).collect::<Vec<_>>().join(",");
            let inserted = delta.inserted.iter().map(|entry| format!("{}:{}", entry.index, $enc_row(&entry.row))).collect::<Vec<_>>().join(",");
            let moved = delta.moved.iter().map(|entry| format!("{}@{}>{}", $enc_id(&entry.id), entry.from, entry.to)).collect::<Vec<_>>().join(",");
            format!("[{removed}];[{inserted}];[{moved}]")
        }

        // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
        pub(crate) fn $dec(s: &str) -> Result<$delta, String> {
            let three = split_top_level(s, ';');
            let [removed_s, inserted_s, moved_s] = three.as_slice() else { return Err(format!("list delta: expected 3 sections, got {}", three.len())) };
            let removed = split_top_level(strip_brackets(removed_s)?, ',')
                .into_iter()
                .filter(|entry| !entry.is_empty())
                .map(|entry| {
                    let (id, index) = entry.rsplit_once('@').ok_or_else(|| format!("list delta removed: bad entry {entry:?}"))?;
                    Ok($removal { id: $dec_id(id)?, index: parse_usize(index)? })
                })
                .collect::<Result<Vec<_>, String>>()?;
            let inserted = split_top_level(strip_brackets(inserted_s)?, ',')
                .into_iter()
                .filter(|entry| !entry.is_empty())
                .map(|entry| {
                    let (index, row) = entry.split_once(':').ok_or_else(|| format!("list delta inserted: bad entry {entry:?}"))?;
                    Ok($insertion { index: parse_usize(index)?, row: $dec_row(row)? })
                })
                .collect::<Result<Vec<_>, String>>()?;
            let moved = split_top_level(strip_brackets(moved_s)?, ',')
                .into_iter()
                .filter(|entry| !entry.is_empty())
                .map(|entry| {
                    let (id, positions) = entry.rsplit_once('@').ok_or_else(|| format!("list delta moved: bad entry {entry:?}"))?;
                    let (from, to) = positions.split_once('>').ok_or_else(|| format!("list delta moved: bad positions {positions:?}"))?;
                    Ok($relocation { id: $dec_id(id)?, from: parse_usize(from)?, to: parse_usize(to)? })
                })
                .collect::<Result<Vec<_>, String>>()?;
            Ok($delta { removed, inserted, moved })
        }
    };
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_attribute(attribute: &GltfAttribute) -> String {
    format!("[{},{}]", enc_str(&attribute.semantic), attribute.accessor)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_attribute(s: &str) -> Result<GltfAttribute, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [semantic, accessor] = parts.as_slice() else { return Err(format!("attribute: expected 2 fields, got {}", parts.len())) };
    Ok(GltfAttribute { semantic: dec_str(semantic)?, accessor: parse_usize(accessor)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_reference(reference: &GltfRef) -> String {
    reference.0.to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_reference(s: &str) -> Result<GltfRef, String> {
    parse_usize(s).map(GltfRef)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_position(position: &usize) -> String {
    position.to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_key(key: &String) -> String {
    enc_str(key)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_target_row(target: &GltfMorphTarget) -> String {
    enc_attr_pairs(&target.0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_target_row(s: &str) -> Result<GltfMorphTarget, String> {
    dec_attr_pairs(s).map(GltfMorphTarget)
}

delta_text_codec!(enc_refs_delta, dec_refs_delta, GltfRefsDelta, GltfRefRemoval, GltfRefInsertion, GltfRefRelocation, enc_position, parse_usize, enc_reference, dec_reference);
delta_text_codec!(enc_strings_delta, dec_strings_delta, GltfStringsDelta, GltfStringRemoval, GltfStringInsertion, GltfStringRelocation, enc_key, dec_str, enc_key, dec_str);
delta_text_codec!(enc_attributes_delta, dec_attributes_delta, GltfAttributesDelta, GltfAttributeRemoval, GltfAttributeInsertion, GltfAttributeRelocation, enc_key, dec_str, enc_attribute, dec_attribute);
delta_text_codec!(enc_targets_delta, dec_targets_delta, GltfTargetsDelta, GltfTargetRemoval, GltfTargetInsertion, GltfTargetRelocation, enc_key, dec_str, enc_target_row, dec_target_row);
delta_text_codec!(enc_channels_delta, dec_channels_delta, GltfChannelsDelta, GltfChannelRemoval, GltfChannelInsertion, GltfChannelRelocation, enc_key, dec_str, enc_animation_channel, dec_animation_channel);
delta_text_codec!(enc_animation_samplers_delta, dec_animation_samplers_delta, GltfAnimationSamplersDelta, GltfAnimationSamplerRemoval, GltfAnimationSamplerInsertion, GltfAnimationSamplerRelocation, enc_key, dec_str, enc_animation_sampler, dec_animation_sampler);

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_primitive_diff(d: &GltfPrimitiveDiff) -> String {
    format!("[{},{},{},{},{},{},{}]", encode_option(&d.attributes, enc_attributes_delta), encode_option_option(&d.indices, |v| v.to_string()), encode_option_option(&d.material, |v| v.to_string()), encode_option_option(&d.mode, |v| enc_u64(*v)), encode_option(&d.targets, enc_targets_delta), encode_option_option(&d.extensions, enc_json), encode_option_option(&d.extras, enc_json))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_primitive_diff(s: &str) -> Result<GltfPrimitiveDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [attributes, indices, material, mode, targets, extensions, extras] = parts.as_slice() else { return Err(format!("primitive diff: expected 7 fields, got {}", parts.len())) };
    Ok(GltfPrimitiveDiff { attributes: decode_option(attributes, dec_attributes_delta)?, indices: decode_option_option(indices, parse_usize)?, material: decode_option_option(material, parse_usize)?, mode: decode_option_option(mode, dec_u64)?, targets: decode_option(targets, dec_targets_delta)?, extensions: decode_option_option(extensions, dec_json)?, extras: decode_option_option(extras, dec_json)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_texture_diff(d: &GltfTextureDiff) -> String {
    format!("[{},{},{},{},{}]", encode_option_option(&d.sampler, |v| v.to_string()), encode_option_option(&d.source, |v| v.to_string()), encode_option_option(&d.name, |v| enc_str(v)), encode_option_option(&d.extensions, enc_json), encode_option_option(&d.extras, enc_json))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_texture_diff(s: &str) -> Result<GltfTextureDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [sampler, source, name, extensions, extras] = parts.as_slice() else { return Err(format!("texture diff: expected 5 fields, got {}", parts.len())) };
    Ok(GltfTextureDiff { sampler: decode_option_option(sampler, parse_usize)?, source: decode_option_option(source, parse_usize)?, name: decode_option_option(name, dec_str)?, extensions: decode_option_option(extensions, dec_json)?, extras: decode_option_option(extras, dec_json)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_image_diff(d: &GltfImageDiff) -> String {
    format!("[{},{},{},{},{},{}]", encode_option_option(&d.uri, |v| enc_str(v)), encode_option_option(&d.mime_type, |v| enc_str(v)), encode_option_option(&d.buffer_view, |v| v.to_string()), encode_option_option(&d.name, |v| enc_str(v)), encode_option_option(&d.extensions, enc_json), encode_option_option(&d.extras, enc_json))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_image_diff(s: &str) -> Result<GltfImageDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [uri, mime_type, buffer_view, name, extensions, extras] = parts.as_slice() else { return Err(format!("image diff: expected 6 fields, got {}", parts.len())) };
    Ok(GltfImageDiff { uri: decode_option_option(uri, dec_str)?, mime_type: decode_option_option(mime_type, dec_str)?, buffer_view: decode_option_option(buffer_view, parse_usize)?, name: decode_option_option(name, dec_str)?, extensions: decode_option_option(extensions, dec_json)?, extras: decode_option_option(extras, dec_json)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_buffer_view_diff(d: &GltfBufferViewDiff) -> String {
    format!("[{},{},{},{},{},{},{},{}]", encode_option(&d.buffer, |v| v.to_string()), encode_option(&d.byte_offset, |v| v.to_string()), encode_option(&d.byte_length, |v| v.to_string()), encode_option_option(&d.byte_stride, |v| v.to_string()), encode_option_option(&d.target, |v| enc_u64(*v)), encode_option_option(&d.name, |v| enc_str(v)), encode_option_option(&d.extensions, enc_json), encode_option_option(&d.extras, enc_json))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_buffer_view_diff(s: &str) -> Result<GltfBufferViewDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [buffer, byte_offset, byte_length, byte_stride, target, name, extensions, extras] = parts.as_slice() else { return Err(format!("buffer_view diff: expected 8 fields, got {}", parts.len())) };
    Ok(GltfBufferViewDiff { buffer: decode_option(buffer, parse_usize)?, byte_offset: decode_option(byte_offset, parse_usize)?, byte_length: decode_option(byte_length, parse_usize)?, byte_stride: decode_option_option(byte_stride, parse_usize)?, target: decode_option_option(target, dec_u64)?, name: decode_option_option(name, dec_str)?, extensions: decode_option_option(extensions, dec_json)?, extras: decode_option_option(extras, dec_json)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_skin_diff(d: &GltfSkinDiff) -> String {
    format!("[{},{},{},{},{},{}]", encode_option_option(&d.inverse_bind_matrices, |v| v.to_string()), encode_option_option(&d.skeleton, |v| v.to_string()), encode_option(&d.joints, enc_refs_delta), encode_option_option(&d.name, |v| enc_str(v)), encode_option_option(&d.extensions, enc_json), encode_option_option(&d.extras, enc_json))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_skin_diff(s: &str) -> Result<GltfSkinDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [inverse_bind_matrices, skeleton, joints, name, extensions, extras] = parts.as_slice() else { return Err(format!("skin diff: expected 6 fields, got {}", parts.len())) };
    Ok(GltfSkinDiff { inverse_bind_matrices: decode_option_option(inverse_bind_matrices, parse_usize)?, skeleton: decode_option_option(skeleton, parse_usize)?, joints: decode_option(joints, dec_refs_delta)?, name: decode_option_option(name, dec_str)?, extensions: decode_option_option(extensions, dec_json)?, extras: decode_option_option(extras, dec_json)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_animation_diff(d: &GltfAnimationDiff) -> String {
    format!("[{},{},{},{},{}]", encode_option(&d.channels, enc_channels_delta), encode_option(&d.samplers, enc_animation_samplers_delta), encode_option_option(&d.name, |v| enc_str(v)), encode_option_option(&d.extensions, enc_json), encode_option_option(&d.extras, enc_json))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_animation_diff(s: &str) -> Result<GltfAnimationDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [channels, samplers, name, extensions, extras] = parts.as_slice() else { return Err(format!("animation diff: expected 5 fields, got {}", parts.len())) };
    Ok(GltfAnimationDiff { channels: decode_option(channels, dec_channels_delta)?, samplers: decode_option(samplers, dec_animation_samplers_delta)?, name: decode_option_option(name, dec_str)?, extensions: decode_option_option(extensions, dec_json)?, extras: decode_option_option(extras, dec_json)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_collection<T, D>(c: &GltfCollectionDiff<T, D>, enc_item: impl Fn(&T) -> String, enc_diff: impl Fn(&D) -> String) -> String {
    let removed = c.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = c.modified.iter().map(|m| format!("{}:{}", m.index, enc_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = c.added.iter().map(|a| format!("{}:{}", a.index, enc_item(&a.item))).collect::<Vec<_>>().join(",");
    if c.amended.is_empty() {
        return format!("[{removed}];[{modified}];[{added}]");
    }
    let amended = c.amended.iter().map(|m| format!("{}:{}", m.index, enc_diff(&m.diff))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}];[{amended}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_collection<T, D>(s: &str, dec_item: impl Fn(&str) -> Result<T, String>, dec_diff: impl Fn(&str) -> Result<D, String>) -> Result<GltfCollectionDiff<T, D>, String> {
    let sections = split_top_level(s, ';');
    let (removed_s, modified_s, added_s, amended_s) = match sections.as_slice() {
        [removed, modified, added] => (removed, modified, added, &"[]"),
        [removed, modified, added, amended] => (removed, modified, added, amended),
        other => return Err(format!("collection: expected 3 or 4 sections, got {}", other.len())),
    };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("collection modified: bad entry {entry:?}"))?;
            Ok(GltfModified { index: parse_usize(idx)?, diff: dec_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("collection added: bad entry {entry:?}"))?;
            Ok(GltfAdded { index: parse_usize(idx)?, item: dec_item(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let amended = split_top_level(strip_brackets(amended_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("collection amended: bad entry {entry:?}"))?;
            Ok(GltfModified { index: parse_usize(idx)?, diff: dec_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(GltfCollectionDiff { removed, modified, added, amended })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_gltf_diff(d: &GltfDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.asset {
        tokens.push(format!("asset={}", enc_asset_diff(v)));
    }
    if let Some(v) = d.scene {
        tokens.push(format!("scene={}", encode_option(&v, |x| x.to_string())));
    }
    if let Some(v) = &d.scenes {
        tokens.push(format!("scenes={}", enc_collection(v, enc_scene, enc_scene_diff)));
    }
    if let Some(v) = &d.nodes {
        tokens.push(format!("nodes={}", enc_collection(v, enc_node, enc_node_diff)));
    }
    if let Some(v) = &d.meshes {
        tokens.push(format!("meshes={}", enc_collection(v, enc_mesh, enc_mesh_diff)));
    }
    if let Some(v) = &d.accessors {
        tokens.push(format!("accessors={}", enc_collection(v, enc_accessor, enc_accessor_diff)));
    }
    if let Some(v) = &d.buffer_views {
        tokens.push(format!("buffer-views={}", enc_collection(v, enc_buffer_view, enc_buffer_view_diff)));
    }
    if let Some(v) = &d.buffers {
        tokens.push(format!("buffers={}", enc_collection(v, enc_buffer, enc_buffer_diff)));
    }
    if let Some(v) = &d.buffer_bytes {
        tokens.push(format!("buffer-bytes={}", enc_collection(v, |b: &Vec<u8>| enc_bytes(b), |b: &Vec<u8>| enc_bytes(b))));
    }
    if let Some(v) = &d.materials {
        tokens.push(format!("materials={}", enc_collection(v, enc_material, enc_material_diff)));
    }
    if let Some(v) = &d.textures {
        tokens.push(format!("textures={}", enc_collection(v, enc_texture, enc_texture_diff)));
    }
    if let Some(v) = &d.images {
        tokens.push(format!("images={}", enc_collection(v, enc_image, enc_image_diff)));
    }
    if let Some(v) = &d.samplers {
        tokens.push(format!("samplers={}", enc_collection(v, enc_sampler, enc_sampler)));
    }
    if let Some(v) = &d.skins {
        tokens.push(format!("skins={}", enc_collection(v, enc_skin, enc_skin_diff)));
    }
    if let Some(v) = &d.animations {
        tokens.push(format!("animations={}", enc_collection(v, enc_animation, enc_animation_diff)));
    }
    if let Some(v) = &d.cameras {
        tokens.push(format!("cameras={}", enc_collection(v, enc_camera, enc_camera)));
    }
    if let Some(v) = &d.extensions_used {
        tokens.push(format!("extensions-used={}", enc_strings_delta(v)));
    }
    if let Some(v) = &d.extensions_required {
        tokens.push(format!("extensions-required={}", enc_strings_delta(v)));
    }
    if let Some(v) = &d.extensions {
        tokens.push(format!("extensions={}", encode_option(v, enc_json)));
    }
    if let Some(v) = &d.extras {
        tokens.push(format!("extras={}", encode_option(v, enc_json)));
    }
    if let Some(v) = d.source_form {
        tokens.push(format!("source-form={}", enc_source_form(v)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_gltf_diff(line: &str) -> Result<GltfDiff, String> {
    let mut d = GltfDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("asset=") {
            d.asset = Some(dec_asset_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("scene=") {
            d.scene = Some(decode_option(rest, parse_usize)?);
        } else if let Some(rest) = token.strip_prefix("scenes=") {
            d.scenes = Some(dec_collection(rest, dec_scene, dec_scene_diff)?);
        } else if let Some(rest) = token.strip_prefix("nodes=") {
            d.nodes = Some(dec_collection(rest, dec_node, dec_node_diff)?);
        } else if let Some(rest) = token.strip_prefix("meshes=") {
            d.meshes = Some(dec_collection(rest, dec_mesh, dec_mesh_diff)?);
        } else if let Some(rest) = token.strip_prefix("accessors=") {
            d.accessors = Some(dec_collection(rest, dec_accessor, dec_accessor_diff)?);
        } else if let Some(rest) = token.strip_prefix("buffer-views=") {
            d.buffer_views = Some(dec_collection(rest, dec_buffer_view, dec_buffer_view_diff)?);
        } else if let Some(rest) = token.strip_prefix("buffer-bytes=") {
            d.buffer_bytes = Some(dec_collection(rest, dec_bytes, dec_bytes)?);
        } else if let Some(rest) = token.strip_prefix("buffers=") {
            d.buffers = Some(dec_collection(rest, dec_buffer, dec_buffer_diff)?);
        } else if let Some(rest) = token.strip_prefix("materials=") {
            d.materials = Some(dec_collection(rest, dec_material, dec_material_diff)?);
        } else if let Some(rest) = token.strip_prefix("textures=") {
            d.textures = Some(dec_collection(rest, dec_texture, dec_texture_diff)?);
        } else if let Some(rest) = token.strip_prefix("images=") {
            d.images = Some(dec_collection(rest, dec_image, dec_image_diff)?);
        } else if let Some(rest) = token.strip_prefix("samplers=") {
            d.samplers = Some(dec_collection(rest, dec_sampler, dec_sampler)?);
        } else if let Some(rest) = token.strip_prefix("skins=") {
            d.skins = Some(dec_collection(rest, dec_skin, dec_skin_diff)?);
        } else if let Some(rest) = token.strip_prefix("animations=") {
            d.animations = Some(dec_collection(rest, dec_animation, dec_animation_diff)?);
        } else if let Some(rest) = token.strip_prefix("cameras=") {
            d.cameras = Some(dec_collection(rest, dec_camera, dec_camera)?);
        } else if let Some(rest) = token.strip_prefix("extensions-used=") {
            d.extensions_used = Some(dec_strings_delta(rest)?);
        } else if let Some(rest) = token.strip_prefix("extensions-required=") {
            d.extensions_required = Some(dec_strings_delta(rest)?);
        } else if let Some(rest) = token.strip_prefix("extensions=") {
            d.extensions = Some(decode_option(rest, dec_json)?);
        } else if let Some(rest) = token.strip_prefix("extras=") {
            d.extras = Some(decode_option(rest, dec_json)?);
        } else if let Some(rest) = token.strip_prefix("source-form=") {
            d.source_form = Some(dec_source_form(rest)?);
        } else {
            return Err(format!("gltf diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for GltfDiff {
fn print_diff(&self) -> String {
    print_gltf_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_gltf_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
