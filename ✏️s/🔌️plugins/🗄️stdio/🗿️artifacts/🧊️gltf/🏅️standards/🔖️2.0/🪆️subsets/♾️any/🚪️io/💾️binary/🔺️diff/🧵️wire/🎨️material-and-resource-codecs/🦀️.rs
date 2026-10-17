//! 🎨️ Binary codecs of the texture infos, PBR material, buffer, buffer view, texture, image, sampler and skin elements.

use crate::{standards::v2_0::subsets::any::schema::snapshot::GltfBuffer, standards::v2_0::subsets::any::schema::snapshot::GltfBufferView, standards::v2_0::subsets::any::schema::snapshot::GltfImage, standards::v2_0::subsets::any::schema::snapshot::GltfMaterial, standards::v2_0::subsets::any::schema::snapshot::GltfNormalTextureInfo, standards::v2_0::subsets::any::schema::snapshot::GltfOcclusionTextureInfo, standards::v2_0::subsets::any::schema::snapshot::GltfPbrMetallicRoughness, standards::v2_0::subsets::any::schema::snapshot::GltfSampler, standards::v2_0::subsets::any::schema::snapshot::GltfSkin, standards::v2_0::subsets::any::schema::snapshot::GltfTexture, standards::v2_0::subsets::any::schema::snapshot::GltfTextureInfo};
use crate::standards::v2_0::subsets::any::io::binary::diff::wire::*;

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
pub fn write_bin_material(w: &mut dsl::ByteWriter, m: &GltfMaterial) {
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
pub fn read_bin_material(r: &mut dsl::ByteReader<'_>) -> Result<GltfMaterial, dsl::PackRefusal> {
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
pub fn write_bin_material_diff(w: &mut dsl::ByteWriter, d: &GltfMaterialDiff) {
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
pub fn read_bin_material_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfMaterialDiff, dsl::PackRefusal> {
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
pub fn write_bin_buffer(w: &mut dsl::ByteWriter, b: &GltfBuffer) {
    w.write_varint_u64(b.byte_length as u64);
    write_bin_option(w, &b.uri, |w, v| write_bin_str(w, v));
    write_bin_option(w, &b.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &b.extensions);
    write_bin_json_opt(w, &b.extras);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_buffer(r: &mut dsl::ByteReader<'_>) -> Result<GltfBuffer, dsl::PackRefusal> {
    Ok(GltfBuffer { byte_length: r.read_varint_u64()? as usize, uri: read_bin_option(r, read_bin_str)?, name: read_bin_option(r, read_bin_str)?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_buffer_diff(w: &mut dsl::ByteWriter, d: &GltfBufferDiff) {
    write_bin_option(w, &d.byte_length, |w, v| w.write_varint_u64(*v as u64));
    write_bin_tri(w, &d.uri, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_buffer_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfBufferDiff, dsl::PackRefusal> {
    Ok(GltfBufferDiff {
        byte_length: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        uri: read_bin_tri(r, read_bin_str)?,
        name: read_bin_tri(r, read_bin_str)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_buffer_view(w: &mut dsl::ByteWriter, v: &GltfBufferView) {
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
pub fn read_bin_buffer_view(r: &mut dsl::ByteReader<'_>) -> Result<GltfBufferView, dsl::PackRefusal> {
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
pub fn read_bin_texture(r: &mut dsl::ByteReader<'_>) -> Result<GltfTexture, dsl::PackRefusal> {
    Ok(GltfTexture {
        sampler: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        source: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        name: read_bin_option(r, read_bin_str)?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_image(w: &mut dsl::ByteWriter, i: &GltfImage) {
    write_bin_option(w, &i.uri, |w, v| write_bin_str(w, v));
    write_bin_option(w, &i.mime_type, |w, v| write_bin_str(w, v));
    write_bin_option(w, &i.buffer_view, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &i.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &i.extensions);
    write_bin_json_opt(w, &i.extras);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_image(r: &mut dsl::ByteReader<'_>) -> Result<GltfImage, dsl::PackRefusal> {
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
pub fn write_bin_sampler(w: &mut dsl::ByteWriter, s: &GltfSampler) {
    write_bin_option(w, &s.mag_filter, |w, v| w.write_varint_u64(*v));
    write_bin_option(w, &s.min_filter, |w, v| w.write_varint_u64(*v));
    w.write_varint_u64(s.wrap_s);
    w.write_varint_u64(s.wrap_t);
    write_bin_option(w, &s.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &s.extensions);
    write_bin_json_opt(w, &s.extras);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_sampler(r: &mut dsl::ByteReader<'_>) -> Result<GltfSampler, dsl::PackRefusal> {
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
pub fn write_bin_skin(w: &mut dsl::ByteWriter, v: &GltfSkin) {
    write_bin_option(w, &v.inverse_bind_matrices, |w, x| w.write_varint_u64(*x as u64));
    write_bin_option(w, &v.skeleton, |w, x| w.write_varint_u64(*x as u64));
    write_bin_usize_vec(w, &v.joints);
    write_bin_option(w, &v.name, |w, x| write_bin_str(w, x));
    write_bin_json_opt(w, &v.extensions);
    write_bin_json_opt(w, &v.extras);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_skin(r: &mut dsl::ByteReader<'_>) -> Result<GltfSkin, dsl::PackRefusal> {
    Ok(GltfSkin {
        inverse_bind_matrices: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        skeleton: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        joints: read_bin_usize_vec(r)?,
        name: read_bin_option(r, read_bin_str)?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}
