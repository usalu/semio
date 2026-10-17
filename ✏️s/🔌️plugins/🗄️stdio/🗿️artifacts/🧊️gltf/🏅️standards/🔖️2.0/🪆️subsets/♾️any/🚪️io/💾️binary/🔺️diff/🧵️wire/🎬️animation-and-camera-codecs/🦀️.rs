//! 🎬️ Binary codecs of the animation, camera and attribute elements and the generic collection triple.

use crate::{standards::v2_0::subsets::any::schema::snapshot::GltfAnimation, standards::v2_0::subsets::any::schema::snapshot::GltfAnimationChannel, standards::v2_0::subsets::any::schema::snapshot::GltfAnimationChannelTarget, standards::v2_0::subsets::any::schema::snapshot::GltfAnimationSampler, standards::v2_0::subsets::any::schema::snapshot::GltfCamera, standards::v2_0::subsets::any::schema::snapshot::GltfCameraProjection, standards::v2_0::subsets::any::schema::snapshot::GltfOrthographic, standards::v2_0::subsets::any::schema::snapshot::GltfPerspective};
use crate::standards::v2_0::subsets::any::io::binary::diff::wire::*;

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
pub fn write_bin_animation(w: &mut dsl::ByteWriter, a: &GltfAnimation) {
    write_bin_vec(w, &a.channels, write_bin_animation_channel);
    write_bin_vec(w, &a.samplers, write_bin_animation_sampler);
    write_bin_option(w, &a.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &a.extensions);
    write_bin_json_opt(w, &a.extras);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_animation(r: &mut dsl::ByteReader<'_>) -> Result<GltfAnimation, dsl::PackRefusal> {
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
pub fn write_bin_camera(w: &mut dsl::ByteWriter, c: &GltfCamera) {
    write_bin_camera_projection(w, &c.projection);
    write_bin_option(w, &c.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &c.extensions);
    write_bin_json_opt(w, &c.extras);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_camera(r: &mut dsl::ByteReader<'_>) -> Result<GltfCamera, dsl::PackRefusal> {
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
    write_bin_vec(w, &c.amended, |w, m: &GltfModified<D>| {
        w.write_varint_u64(m.index as u64);
        write_diff(w, &m.diff);
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
    let amended = read_bin_vec(r, |r| {
        let index = r.read_varint_u64()? as usize;
        let diff = read_diff(r)?;
        Ok(GltfModified { index, diff })
    })?;
    Ok(GltfCollectionDiff { removed, modified, added, amended })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_attribute(w: &mut dsl::ByteWriter, attribute: &GltfAttribute) {
    write_bin_str(w, &attribute.semantic);
    w.write_varint_u64(attribute.accessor as u64);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_attribute(r: &mut dsl::ByteReader<'_>) -> Result<GltfAttribute, dsl::PackRefusal> {
    Ok(GltfAttribute { semantic: read_bin_str(r)?, accessor: r.read_varint_u64()? as usize })
}
