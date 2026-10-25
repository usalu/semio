//! 🎬️ Text codecs of the animation and camera elements.

use crate::{standards::v2_0::subsets::any::schema::snapshot::GltfAnimation, standards::v2_0::subsets::any::schema::snapshot::GltfAnimationChannel, standards::v2_0::subsets::any::schema::snapshot::GltfAnimationChannelTarget, standards::v2_0::subsets::any::schema::snapshot::GltfAnimationSampler, standards::v2_0::subsets::any::schema::snapshot::GltfCamera, standards::v2_0::subsets::any::schema::snapshot::GltfCameraProjection, standards::v2_0::subsets::any::schema::snapshot::GltfOrthographic, standards::v2_0::subsets::any::schema::snapshot::GltfPerspective};
use crate::standards::v2_0::subsets::any::io::text::diff::wire::*;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_animation_channel_target(t: &GltfAnimationChannelTarget) -> String {
    format!("[{},{},{},{}]", encode_option(&t.node, |v| v.to_string()), enc_animation_path(t.path), encode_option(&t.extensions, enc_json), encode_option(&t.extras, enc_json))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_animation_channel_target(s: &str) -> Result<GltfAnimationChannelTarget, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [node, path, extensions, extras] = parts.as_slice() else { return Err(format!("animation channel target: expected 4 fields, got {}", parts.len())) };
    Ok(GltfAnimationChannelTarget { node: decode_option(node, parse_usize)?, path: dec_animation_path(path)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_animation_channel(c: &GltfAnimationChannel) -> String {
    format!("[{},{},{},{}]", c.sampler, enc_animation_channel_target(&c.target), encode_option(&c.extensions, enc_json), encode_option(&c.extras, enc_json))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_animation_channel(s: &str) -> Result<GltfAnimationChannel, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [sampler, target, extensions, extras] = parts.as_slice() else { return Err(format!("animation channel: expected 4 fields, got {}", parts.len())) };
    Ok(GltfAnimationChannel { sampler: parse_usize(sampler)?, target: dec_animation_channel_target(target)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_animation_sampler(s: &GltfAnimationSampler) -> String {
    format!("[{},{},{},{},{}]", s.input, enc_interpolation(s.interpolation), s.output, encode_option(&s.extensions, enc_json), encode_option(&s.extras, enc_json))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_animation_sampler(s: &str) -> Result<GltfAnimationSampler, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [input, interpolation, output, extensions, extras] = parts.as_slice() else { return Err(format!("animation sampler: expected 5 fields, got {}", parts.len())) };
    Ok(GltfAnimationSampler { input: parse_usize(input)?, interpolation: dec_interpolation(interpolation)?, output: parse_usize(output)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_animation(a: &GltfAnimation) -> String {
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
pub fn dec_animation(s: &str) -> Result<GltfAnimation, String> {
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
pub fn enc_perspective(p: &GltfPerspective) -> String {
    format!("[{},{},{},{},{},{}]", encode_option(&p.aspect_ratio, |v| enc_f64(*v)), enc_f64(p.yfov), encode_option(&p.zfar, |v| enc_f64(*v)), enc_f64(p.znear), encode_option(&p.extensions, enc_json), encode_option(&p.extras, enc_json),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_perspective(s: &str) -> Result<GltfPerspective, String> {
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
pub fn enc_orthographic(o: &GltfOrthographic) -> String {
    format!("[{},{},{},{},{},{}]", enc_f64(o.xmag), enc_f64(o.ymag), enc_f64(o.zfar), enc_f64(o.znear), encode_option(&o.extensions, enc_json), encode_option(&o.extras, enc_json))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_orthographic(s: &str) -> Result<GltfOrthographic, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [xmag, ymag, zfar, znear, extensions, extras] = parts.as_slice() else { return Err(format!("orthographic: expected 6 fields, got {}", parts.len())) };
    Ok(GltfOrthographic { xmag: dec_f64(xmag)?, ymag: dec_f64(ymag)?, zfar: dec_f64(zfar)?, znear: dec_f64(znear)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}
/// 🔀️ `GltfCameraProjection` is a real data-carrying enum (§3a) -- tag prefix `P`=Perspective,
/// `O`=Orthographic.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_camera_projection(p: &GltfCameraProjection) -> String {
    match p {
        GltfCameraProjection::Perspective(v) => format!("P{}", enc_perspective(v)),
        GltfCameraProjection::Orthographic(v) => format!("O{}", enc_orthographic(v)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_camera_projection(s: &str) -> Result<GltfCameraProjection, String> {
    let (tag, rest) = s.split_at(1);
    match tag {
        "P" => Ok(GltfCameraProjection::Perspective(dec_perspective(rest)?)),
        "O" => Ok(GltfCameraProjection::Orthographic(dec_orthographic(rest)?)),
        other => Err(format!("camera projection: unknown tag {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_camera(c: &GltfCamera) -> String {
    format!("[{},{},{},{}]", enc_camera_projection(&c.projection), encode_option(&c.name, |v| enc_str(v)), encode_option(&c.extensions, enc_json), encode_option(&c.extras, enc_json))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_camera(s: &str) -> Result<GltfCamera, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [projection, name, extensions, extras] = parts.as_slice() else { return Err(format!("camera: expected 4 fields, got {}", parts.len())) };
    Ok(GltfCamera { projection: dec_camera_projection(projection)?, name: decode_option(name, dec_str)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}
