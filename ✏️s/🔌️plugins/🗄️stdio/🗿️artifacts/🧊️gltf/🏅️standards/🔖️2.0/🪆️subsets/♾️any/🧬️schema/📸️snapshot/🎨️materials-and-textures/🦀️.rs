//! 🎨️ Materials and textures of the glTF snapshot: the texture infos, the PBR metallic roughness model, the alpha mode, the materials, textures, images and samplers. See [`super`].

use super::attribute_map_and_defaults::{
    default_alpha_cutoff, default_one_f64, default_vec3_zero, default_vec4_one, default_wrap, default_zero_u64, is_default_alpha_cutoff, is_default_wrap, is_false, is_one_f64, is_vec3_zero, is_vec4_one, is_zero_u64,
};
use super::present_json;
use super::source_form_and_json::GltfJson;
#[cfg(any(test, feature = "testing"))]
use serde::{Deserialize, Serialize};

//#region 🔖️Material
/// 🖼️ A texture reference (§5.20 `textureInfo`) shared by `baseColorTexture` /
/// `metallicRoughnessTexture` / `emissiveTexture`.
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfTextureInfo {
    pub index: usize,
    #[cfg_attr(any(test, feature = "testing"), serde(default = "default_zero_u64", skip_serializing_if = "is_zero_u64", rename = "texCoord"))]
    #[value(default = "default_zero_u64", skip_serializing_if = "is_zero_u64", rename = "texCoord")]
    pub tex_coord: u64,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extensions: Option<GltfJson>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extras: Option<GltfJson>,
}

/// 🖼️ `material.normalTexture` (§5.21) -- adds `scale` (default 1).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfNormalTextureInfo {
    pub index: usize,
    #[cfg_attr(any(test, feature = "testing"), serde(default = "default_zero_u64", skip_serializing_if = "is_zero_u64", rename = "texCoord"))]
    #[value(default = "default_zero_u64", skip_serializing_if = "is_zero_u64", rename = "texCoord")]
    pub tex_coord: u64,
    #[cfg_attr(any(test, feature = "testing"), serde(default = "default_one_f64", skip_serializing_if = "is_one_f64"))]
    #[value(default = "default_one_f64", skip_serializing_if = "is_one_f64")]
    pub scale: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extensions: Option<GltfJson>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extras: Option<GltfJson>,
}

/// 🖼️ `material.occlusionTexture` (§5.22) -- adds `strength` (default 1).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfOcclusionTextureInfo {
    pub index: usize,
    #[cfg_attr(any(test, feature = "testing"), serde(default = "default_zero_u64", skip_serializing_if = "is_zero_u64", rename = "texCoord"))]
    #[value(default = "default_zero_u64", skip_serializing_if = "is_zero_u64", rename = "texCoord")]
    pub tex_coord: u64,
    #[cfg_attr(any(test, feature = "testing"), serde(default = "default_one_f64", skip_serializing_if = "is_one_f64"))]
    #[value(default = "default_one_f64", skip_serializing_if = "is_one_f64")]
    pub strength: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extensions: Option<GltfJson>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extras: Option<GltfJson>,
}

/// 🎨️ `material.pbrMetallicRoughness` (§5.23).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfPbrMetallicRoughness {
    #[cfg_attr(any(test, feature = "testing"), serde(default = "default_vec4_one", skip_serializing_if = "is_vec4_one"))]
    #[value(default = "default_vec4_one", skip_serializing_if = "is_vec4_one")]
    pub base_color_factor: [f64; 4],
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub base_color_texture: Option<GltfTextureInfo>,
    #[cfg_attr(any(test, feature = "testing"), serde(default = "default_one_f64", skip_serializing_if = "is_one_f64"))]
    #[value(default = "default_one_f64", skip_serializing_if = "is_one_f64")]
    pub metallic_factor: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default = "default_one_f64", skip_serializing_if = "is_one_f64"))]
    #[value(default = "default_one_f64", skip_serializing_if = "is_one_f64")]
    pub roughness_factor: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub metallic_roughness_texture: Option<GltfTextureInfo>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extensions: Option<GltfJson>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extras: Option<GltfJson>,
}

impl Default for GltfPbrMetallicRoughness {
    fn default() -> Self {
        Self { base_color_factor: default_vec4_one(), base_color_texture: None, metallic_factor: 1.0, roughness_factor: 1.0, metallic_roughness_texture: None, extensions: None, extras: None }
    }
}

/// 🔀️ `material.alphaMode` (§5.23.1).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, Default, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[derive(semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum GltfAlphaMode {
    #[default]
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "OPAQUE"))]
    #[value(rename = "OPAQUE")]
    Opaque,
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "MASK"))]
    #[value(rename = "MASK")]
    Mask,
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "BLEND"))]
    #[value(rename = "BLEND")]
    Blend,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn is_opaque(v: &GltfAlphaMode) -> bool {
    matches!(v, GltfAlphaMode::Opaque)
}

/// 🎨️ `materials[i]` (§5.23).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfMaterial {
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pbr_metallic_roughness: Option<GltfPbrMetallicRoughness>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub normal_texture: Option<GltfNormalTextureInfo>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub occlusion_texture: Option<GltfOcclusionTextureInfo>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub emissive_texture: Option<GltfTextureInfo>,
    #[cfg_attr(any(test, feature = "testing"), serde(default = "default_vec3_zero", skip_serializing_if = "is_vec3_zero"))]
    #[value(default = "default_vec3_zero", skip_serializing_if = "is_vec3_zero")]
    pub emissive_factor: [f64; 3],
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "is_opaque", rename = "alphaMode"))]
    #[value(default, skip_serializing_if = "is_opaque", rename = "alphaMode")]
    pub alpha_mode: GltfAlphaMode,
    #[cfg_attr(any(test, feature = "testing"), serde(default = "default_alpha_cutoff", skip_serializing_if = "is_default_alpha_cutoff"))]
    #[value(default = "default_alpha_cutoff", skip_serializing_if = "is_default_alpha_cutoff")]
    pub alpha_cutoff: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "is_false"))]
    #[value(default, skip_serializing_if = "is_false")]
    pub double_sided: bool,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extensions: Option<GltfJson>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extras: Option<GltfJson>,
}

impl Default for GltfMaterial {
    fn default() -> Self {
        Self {
            name: None,
            pbr_metallic_roughness: None,
            normal_texture: None,
            occlusion_texture: None,
            emissive_texture: None,
            emissive_factor: default_vec3_zero(),
            alpha_mode: GltfAlphaMode::Opaque,
            alpha_cutoff: 0.5,
            double_sided: false,
            extensions: None,
            extras: None,
        }
    }
}
//#endregion 🔖️Material

//#region 🔖️TextureImageSampler
/// 🧵️ `textures[i]` (§5.30).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct GltfTexture {
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub sampler: Option<usize>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<usize>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extensions: Option<GltfJson>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extras: Option<GltfJson>,
}

/// 🖼️ `images[i]` (§5.15) -- image bytes are addressed EITHER by `uri` (external/data-uri) OR by
/// `bufferView` (embedded), never both.
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct GltfImage {
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none", rename = "mimeType"))]
    #[value(default, skip_serializing_if = "Option::is_none", rename = "mimeType")]
    pub mime_type: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub buffer_view: Option<usize>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extensions: Option<GltfJson>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extras: Option<GltfJson>,
}

/// 🧲️ `samplers[i]` (§5.27) -- `wrapS`/`wrapT` both default to `10497` (`REPEAT`).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfSampler {
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mag_filter: Option<u64>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min_filter: Option<u64>,
    #[cfg_attr(any(test, feature = "testing"), serde(default = "default_wrap", skip_serializing_if = "is_default_wrap"))]
    #[value(default = "default_wrap", skip_serializing_if = "is_default_wrap")]
    pub wrap_s: u64,
    #[cfg_attr(any(test, feature = "testing"), serde(default = "default_wrap", skip_serializing_if = "is_default_wrap"))]
    #[value(default = "default_wrap", skip_serializing_if = "is_default_wrap")]
    pub wrap_t: u64,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extensions: Option<GltfJson>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extras: Option<GltfJson>,
}

impl Default for GltfSampler {
    fn default() -> Self {
        Self { mag_filter: None, min_filter: None, wrap_s: 10497, wrap_t: 10497, name: None, extensions: None, extras: None }
    }
}
//#endregion 🔖️TextureImageSampler
