//! 🔺️ The list delta codec macro and its instances, the per-element diff codecs, the generic collection triple and the `GltfDiff` print and parse.

use crate::standards::v2_0::subsets::any::schema::snapshot::GltfMorphTarget;
use crate::standards::v2_0::subsets::any::io::text::diff::wire::*;

/// 🧮️ Generic index-keyed collection triple codec, shared by every one of the 14 top-level arrays
/// (STRONG entities pass a real per-item diff encoder; WEAK entities pass the same `enc_item`/
/// `dec_item` for both `enc_item`/`enc_diff` via `GltfWeakCollectionDiff<T> = GltfCollectionDiff<T,
/// T>`) -- one real generic codec, not 14 hand-duplicated ones.
/// 🧷️ Text codec of one positional list delta: `[id@index,…];[index:row,…];[id@from>to,…]` -- removed ids with their base positions,
/// inserted rows with their after positions, moved ids with both.
macro_rules! delta_text_codec {
    ($enc:ident, $dec:ident, $delta:ident, $removal:ident, $insertion:ident, $relocation:ident, $enc_id:expr, $dec_id:expr, $enc_row:expr, $dec_row:expr) => {
        pub(crate) fn $enc(delta: &$delta) -> String {
            let removed = delta.removed.iter().map(|entry| format!("{}@{}", $enc_id(&entry.id), entry.index)).collect::<Vec<_>>().join(",");
            let inserted = delta.inserted.iter().map(|entry| format!("{}:{}", entry.index, $enc_row(&entry.row))).collect::<Vec<_>>().join(",");
            let moved = delta.moved.iter().map(|entry| format!("{}@{}>{}", $enc_id(&entry.id), entry.from, entry.to)).collect::<Vec<_>>().join(",");
            format!("[{removed}];[{inserted}];[{moved}]")
        }

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