//! 🔺️ The list delta codec macro and its instances, the per-element diff codecs and the late writers.

use crate::{standards::v2_0::subsets::any::schema::snapshot::GltfJson, standards::v2_0::subsets::any::schema::snapshot::GltfMorphTarget, standards::v2_0::subsets::any::schema::snapshot::GltfNormalTextureInfo, standards::v2_0::subsets::any::schema::snapshot::GltfOcclusionTextureInfo, standards::v2_0::subsets::any::schema::snapshot::GltfTexture, standards::v2_0::subsets::any::schema::snapshot::GltfTextureInfo};
use crate::standards::v2_0::subsets::any::io::binary::diff::wire::*;

/// 🧷️ Binary codec of one positional list delta: removed `(id, base index)`, inserted `(after index, row)`, moved `(id, from, to)`.
macro_rules! delta_bin_codec {
    ($write:ident, $read:ident, $delta:ident, $removal:ident, $insertion:ident, $relocation:ident, $write_id:expr, $read_id:expr, $write_row:expr, $read_row:expr) => {
        // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
        pub fn $write(w: &mut dsl::ByteWriter, delta: &$delta) {
            write_bin_vec(w, &delta.removed, |w, entry: &$removal| {
                $write_id(w, &entry.id);
                w.write_varint_u64(entry.index as u64);
            });
            write_bin_vec(w, &delta.inserted, |w, entry: &$insertion| {
                w.write_varint_u64(entry.index as u64);
                $write_row(w, &entry.row);
            });
            write_bin_vec(w, &delta.moved, |w, entry: &$relocation| {
                $write_id(w, &entry.id);
                w.write_varint_u64(entry.from as u64);
                w.write_varint_u64(entry.to as u64);
            });
        }

        // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
        pub fn $read(r: &mut dsl::ByteReader<'_>) -> Result<$delta, dsl::PackRefusal> {
            let removed = read_bin_vec(r, |r| Ok($removal { id: $read_id(r)?, index: r.read_varint_u64()? as usize }))?;
            let inserted = read_bin_vec(r, |r| Ok($insertion { index: r.read_varint_u64()? as usize, row: $read_row(r)? }))?;
            let moved = read_bin_vec(r, |r| Ok($relocation { id: $read_id(r)?, from: r.read_varint_u64()? as usize, to: r.read_varint_u64()? as usize }))?;
            Ok($delta { removed, inserted, moved })
        }
    };
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bin_position(w: &mut dsl::ByteWriter, value: &usize) {
    w.write_varint_u64(*value as u64);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bin_position(r: &mut dsl::ByteReader<'_>) -> Result<usize, dsl::PackRefusal> {
    Ok(r.read_varint_u64()? as usize)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bin_reference(w: &mut dsl::ByteWriter, value: &GltfRef) {
    w.write_varint_u64(value.0 as u64);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bin_reference(r: &mut dsl::ByteReader<'_>) -> Result<GltfRef, dsl::PackRefusal> {
    Ok(GltfRef(r.read_varint_u64()? as usize))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bin_key(w: &mut dsl::ByteWriter, value: &String) {
    write_bin_str(w, value);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bin_target_row(w: &mut dsl::ByteWriter, value: &GltfMorphTarget) {
    write_bin_attr_pairs(w, &value.0);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bin_target_row(r: &mut dsl::ByteReader<'_>) -> Result<GltfMorphTarget, dsl::PackRefusal> {
    read_bin_attr_pairs(r).map(GltfMorphTarget)
}
delta_bin_codec!(write_bin_refs_delta, read_bin_refs_delta, GltfRefsDelta, GltfRefRemoval, GltfRefInsertion, GltfRefRelocation, write_bin_position, read_bin_position, write_bin_reference, read_bin_reference);
delta_bin_codec!(write_bin_strings_delta, read_bin_strings_delta, GltfStringsDelta, GltfStringRemoval, GltfStringInsertion, GltfStringRelocation, write_bin_key, read_bin_str, write_bin_key, read_bin_str);
delta_bin_codec!(write_bin_attributes_delta, read_bin_attributes_delta, GltfAttributesDelta, GltfAttributeRemoval, GltfAttributeInsertion, GltfAttributeRelocation, write_bin_key, read_bin_str, write_bin_attribute, read_bin_attribute);
delta_bin_codec!(write_bin_targets_delta, read_bin_targets_delta, GltfTargetsDelta, GltfTargetRemoval, GltfTargetInsertion, GltfTargetRelocation, write_bin_key, read_bin_str, write_bin_target_row, read_bin_target_row);
delta_bin_codec!(write_bin_channels_delta, read_bin_channels_delta, GltfChannelsDelta, GltfChannelRemoval, GltfChannelInsertion, GltfChannelRelocation, write_bin_key, read_bin_str, write_bin_animation_channel, read_bin_animation_channel);
delta_bin_codec!(write_bin_animation_samplers_delta, read_bin_animation_samplers_delta, GltfAnimationSamplersDelta, GltfAnimationSamplerRemoval, GltfAnimationSamplerInsertion, GltfAnimationSamplerRelocation, write_bin_key, read_bin_str, write_bin_animation_sampler, read_bin_animation_sampler);
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_primitive_diff(w: &mut dsl::ByteWriter, d: &GltfPrimitiveDiff) {
    write_bin_option(w, &d.attributes, write_bin_attributes_delta);
    write_bin_tri(w, &d.indices, |w, v| w.write_varint_u64(*v as u64));
    write_bin_tri(w, &d.material, |w, v| w.write_varint_u64(*v as u64));
    write_bin_tri(w, &d.mode, |w, v| w.write_varint_u64(*v));
    write_bin_option(w, &d.targets, write_bin_targets_delta);
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_primitive_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfPrimitiveDiff, dsl::PackRefusal> {
    Ok(GltfPrimitiveDiff {
        attributes: read_bin_option(r, read_bin_attributes_delta)?,
        indices: read_bin_tri(r, |r| Ok(r.read_varint_u64()? as usize))?,
        material: read_bin_tri(r, |r| Ok(r.read_varint_u64()? as usize))?,
        mode: read_bin_tri(r, |r| r.read_varint_u64())?,
        targets: read_bin_option(r, read_bin_targets_delta)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_texture_diff(w: &mut dsl::ByteWriter, d: &GltfTextureDiff) {
    write_bin_tri(w, &d.sampler, |w, v| w.write_varint_u64(*v as u64));
    write_bin_tri(w, &d.source, |w, v| w.write_varint_u64(*v as u64));
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_texture_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfTextureDiff, dsl::PackRefusal> {
    Ok(GltfTextureDiff {
        sampler: read_bin_tri(r, |r| Ok(r.read_varint_u64()? as usize))?,
        source: read_bin_tri(r, |r| Ok(r.read_varint_u64()? as usize))?,
        name: read_bin_tri(r, read_bin_str)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_image_diff(w: &mut dsl::ByteWriter, d: &GltfImageDiff) {
    write_bin_tri(w, &d.uri, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.mime_type, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.buffer_view, |w, v| w.write_varint_u64(*v as u64));
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_image_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfImageDiff, dsl::PackRefusal> {
    Ok(GltfImageDiff {
        uri: read_bin_tri(r, read_bin_str)?,
        mime_type: read_bin_tri(r, read_bin_str)?,
        buffer_view: read_bin_tri(r, |r| Ok(r.read_varint_u64()? as usize))?,
        name: read_bin_tri(r, read_bin_str)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_buffer_view_diff(w: &mut dsl::ByteWriter, d: &GltfBufferViewDiff) {
    write_bin_option(w, &d.buffer, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &d.byte_offset, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &d.byte_length, |w, v| w.write_varint_u64(*v as u64));
    write_bin_tri(w, &d.byte_stride, |w, v| w.write_varint_u64(*v as u64));
    write_bin_tri(w, &d.target, |w, v| w.write_varint_u64(*v));
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_buffer_view_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfBufferViewDiff, dsl::PackRefusal> {
    Ok(GltfBufferViewDiff {
        buffer: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        byte_offset: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        byte_length: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        byte_stride: read_bin_tri(r, |r| Ok(r.read_varint_u64()? as usize))?,
        target: read_bin_tri(r, |r| r.read_varint_u64())?,
        name: read_bin_tri(r, read_bin_str)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_skin_diff(w: &mut dsl::ByteWriter, d: &GltfSkinDiff) {
    write_bin_tri(w, &d.inverse_bind_matrices, |w, v| w.write_varint_u64(*v as u64));
    write_bin_tri(w, &d.skeleton, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &d.joints, write_bin_refs_delta);
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_skin_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfSkinDiff, dsl::PackRefusal> {
    Ok(GltfSkinDiff {
        inverse_bind_matrices: read_bin_tri(r, |r| Ok(r.read_varint_u64()? as usize))?,
        skeleton: read_bin_tri(r, |r| Ok(r.read_varint_u64()? as usize))?,
        joints: read_bin_option(r, read_bin_refs_delta)?,
        name: read_bin_tri(r, read_bin_str)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_animation_diff(w: &mut dsl::ByteWriter, d: &GltfAnimationDiff) {
    write_bin_option(w, &d.channels, write_bin_channels_delta);
    write_bin_option(w, &d.samplers, write_bin_animation_samplers_delta);
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_animation_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfAnimationDiff, dsl::PackRefusal> {
    Ok(GltfAnimationDiff {
        channels: read_bin_option(r, read_bin_channels_delta)?,
        samplers: read_bin_option(r, read_bin_animation_samplers_delta)?,
        name: read_bin_tri(r, read_bin_str)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}
/// 🧵 A single opaque length-prefixed blob wrapping one collection's real binary encoding --
/// matches `../💾️binary/📡️.protocol.semio`'s `Array(u8, Field(<name>_len))` fields (the
/// blob's OWN internal removed/modified/added shape isn't further protocol-walkable,
/// `protocol-prim-ref-recursion`/`protocol-array-of-records`, same documented limitation as every
/// other stdio pilot's own nested-payload field).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_collection_blob<T, D>(c: &GltfCollectionDiff<T, D>, write_item: impl Fn(&mut dsl::ByteWriter, &T), write_diff: impl Fn(&mut dsl::ByteWriter, &D)) -> Vec<u8> {
    let mut inner = dsl::ByteWriter::new();
    write_bin_collection(&mut inner, c, write_item, write_diff);
    inner.into_bytes()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_collection_blob<T, D>(
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
pub fn write_bin_blob(w: &mut dsl::ByteWriter, bytes: &[u8]) {
    w.write_varint_u64(bytes.len() as u64);
    w.write_bytes(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_str(w: &mut dsl::ByteWriter, s: &str) {
    write_bin_blob(w, s.as_bytes());
}
/// 🧩 2-way presence flag (`0`=None, `1`=Some) — shared by every plain `Option<T>` field.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_option<T>(w: &mut dsl::ByteWriter, v: &Option<T>, write_value: impl FnOnce(&mut dsl::ByteWriter, &T)) {
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
pub fn write_bin_json(w: &mut dsl::ByteWriter, v: &GltfJson) {
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
pub fn write_bin_texture(w: &mut dsl::ByteWriter, t: &GltfTexture) {
    write_bin_option(w, &t.sampler, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &t.source, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &t.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &t.extensions);
    write_bin_json_opt(w, &t.extras);
}
