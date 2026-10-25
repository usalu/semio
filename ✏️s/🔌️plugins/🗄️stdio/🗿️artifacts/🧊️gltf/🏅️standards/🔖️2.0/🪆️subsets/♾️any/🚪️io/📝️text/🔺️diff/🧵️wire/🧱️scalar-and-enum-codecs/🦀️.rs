//! 🧱️ Text primitives, option framing, hex, JSON and f64 helpers and the glTF enum codecs.

use crate::{standards::v2_0::subsets::any::schema::snapshot::GltfAccessorType, standards::v2_0::subsets::any::schema::snapshot::GltfComponentType};
use crate::{standards::v2_0::subsets::any::schema::snapshot::GltfAlphaMode, standards::v2_0::subsets::any::schema::snapshot::GltfAnimationPath, standards::v2_0::subsets::any::schema::snapshot::GltfInterpolation, standards::v2_0::subsets::any::schema::snapshot::GltfJson, standards::v2_0::subsets::any::schema::snapshot::GltfSourceForm};
use crate::standards::v2_0::subsets::any::io::text::diff::wire::*;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn split_top_level(s: &str, sep: char) -> Vec<&str> {
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
pub fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
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
pub fn encode_option_option<T>(opt: &Option<Option<T>>, enc: impl Fn(&T) -> String) -> String {
    encode_option(opt, |inner: &Option<T>| encode_option(inner, &enc))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_option_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<Option<T>>, String> {
    decode_option(s, |inner: &str| decode_option(inner, &dec))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_f64(v: f64) -> String {
    v.to_string()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_f64(s: &str) -> Result<f64, String> {
    s.parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_u64(v: u64) -> String {
    v.to_string()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_u64(s: &str) -> Result<u64, String> {
    s.parse::<u64>().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_bool(v: bool) -> String {
    if v {
        "1".to_string()
    } else {
        "0".to_string()
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_bool(s: &str) -> Result<bool, String> {
    match s {
        "0" => Ok(false),
        "1" => Ok(true),
        other => Err(format!("bool: expected 0/1, got {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_f64_slice(v: &[f64]) -> String {
    format!("[{}]", v.iter().map(|x| enc_f64(*x)).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_f64_vec(s: &str) -> Result<Vec<f64>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_f64).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_f64_array<const N: usize>(s: &str) -> Result<[f64; N], String> {
    let v = dec_f64_vec(s)?;
    let len = v.len();
    v.try_into().map_err(|_| format!("expected {N} floats, got {len}"))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_usize_vec(v: &[usize]) -> String {
    format!("[{}]", v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_usize_vec(s: &str) -> Result<Vec<usize>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect()
}
/// 🏷️ `GltfPrimitive::attributes` -- `Vec<(String, usize)>`, name-keyed and order-preserving.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_attr_pairs(v: &[(String, usize)]) -> String {
    format!("[{}]", v.iter().map(|(k, idx)| format!("{}:{idx}", enc_str(k))).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_attr_pairs(s: &str) -> Result<Vec<(String, usize)>, String> {
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
pub fn enc_json(v: &GltfJson) -> String {
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
pub fn dec_json(s: &str) -> Result<GltfJson, String> {
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
pub fn enc_component_type(t: GltfComponentType) -> String {
    t.code().to_string()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_component_type(s: &str) -> Result<GltfComponentType, String> {
    GltfComponentType::from_code(dec_u64(s)?)
}
/// 🔤️ Word tag -- reuses [`GltfAccessorType::as_str`]/`from_str`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_accessor_type(t: GltfAccessorType) -> String {
    t.as_str().to_string()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_accessor_type(s: &str) -> Result<GltfAccessorType, String> {
    s.parse()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_alpha_mode(m: GltfAlphaMode) -> String {
    match m {
        GltfAlphaMode::Opaque => "OPAQUE",
        GltfAlphaMode::Mask => "MASK",
        GltfAlphaMode::Blend => "BLEND",
    }
    .to_string()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_alpha_mode(s: &str) -> Result<GltfAlphaMode, String> {
    match s {
        "OPAQUE" => Ok(GltfAlphaMode::Opaque),
        "MASK" => Ok(GltfAlphaMode::Mask),
        "BLEND" => Ok(GltfAlphaMode::Blend),
        other => Err(format!("alpha mode: unknown {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_interpolation(i: GltfInterpolation) -> String {
    match i {
        GltfInterpolation::Linear => "LINEAR",
        GltfInterpolation::Step => "STEP",
        GltfInterpolation::CubicSpline => "CUBICSPLINE",
    }
    .to_string()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_interpolation(s: &str) -> Result<GltfInterpolation, String> {
    match s {
        "LINEAR" => Ok(GltfInterpolation::Linear),
        "STEP" => Ok(GltfInterpolation::Step),
        "CUBICSPLINE" => Ok(GltfInterpolation::CubicSpline),
        other => Err(format!("interpolation: unknown {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_animation_path(p: GltfAnimationPath) -> String {
    match p {
        GltfAnimationPath::Translation => "translation",
        GltfAnimationPath::Rotation => "rotation",
        GltfAnimationPath::Scale => "scale",
        GltfAnimationPath::Weights => "weights",
    }
    .to_string()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_animation_path(s: &str) -> Result<GltfAnimationPath, String> {
    match s {
        "translation" => Ok(GltfAnimationPath::Translation),
        "rotation" => Ok(GltfAnimationPath::Rotation),
        "scale" => Ok(GltfAnimationPath::Scale),
        "weights" => Ok(GltfAnimationPath::Weights),
        other => Err(format!("animation path: unknown {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_source_form(f: GltfSourceForm) -> String {
    match f {
        GltfSourceForm::Json => "json",
        GltfSourceForm::Glb => "glb",
    }
    .to_string()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_source_form(s: &str) -> Result<GltfSourceForm, String> {
    match s {
        "json" => Ok(GltfSourceForm::Json),
        "glb" => Ok(GltfSourceForm::Glb),
        other => Err(format!("source form: unknown {other:?}")),
    }
}
