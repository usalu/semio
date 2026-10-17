//! 🎨️ Text codecs of the texture infos, PBR material, buffer, buffer view, texture, image, sampler and skin elements.

use crate::{standards::v2_0::subsets::any::schema::snapshot::GltfBuffer, standards::v2_0::subsets::any::schema::snapshot::GltfBufferView, standards::v2_0::subsets::any::schema::snapshot::GltfImage, standards::v2_0::subsets::any::schema::snapshot::GltfMaterial, standards::v2_0::subsets::any::schema::snapshot::GltfNormalTextureInfo, standards::v2_0::subsets::any::schema::snapshot::GltfOcclusionTextureInfo, standards::v2_0::subsets::any::schema::snapshot::GltfPbrMetallicRoughness, standards::v2_0::subsets::any::schema::snapshot::GltfSampler, standards::v2_0::subsets::any::schema::snapshot::GltfSkin, standards::v2_0::subsets::any::schema::snapshot::GltfTexture, standards::v2_0::subsets::any::schema::snapshot::GltfTextureInfo};
use crate::standards::v2_0::subsets::any::io::text::diff::wire::*;

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
