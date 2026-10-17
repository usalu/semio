//! 🎨️ The material, buffer, primitive, texture, image, buffer view, skin and animation element diffs and the collection aliases.

use crate::{standards::v2_0::subsets::any::schema::snapshot::GltfAccessor, standards::v2_0::subsets::any::schema::snapshot::GltfAlphaMode, standards::v2_0::subsets::any::schema::snapshot::GltfAnimation, standards::v2_0::subsets::any::schema::snapshot::GltfBuffer, standards::v2_0::subsets::any::schema::snapshot::GltfBufferView, standards::v2_0::subsets::any::schema::snapshot::GltfCamera, standards::v2_0::subsets::any::schema::snapshot::GltfImage, standards::v2_0::subsets::any::schema::snapshot::GltfJson, standards::v2_0::subsets::any::schema::snapshot::GltfMaterial, standards::v2_0::subsets::any::schema::snapshot::GltfMesh, standards::v2_0::subsets::any::schema::snapshot::GltfNode, standards::v2_0::subsets::any::schema::snapshot::GltfNormalTextureInfo, standards::v2_0::subsets::any::schema::snapshot::GltfOcclusionTextureInfo, standards::v2_0::subsets::any::schema::snapshot::GltfPbrMetallicRoughness, standards::v2_0::subsets::any::schema::snapshot::GltfPrimitive, standards::v2_0::subsets::any::schema::snapshot::GltfSampler, standards::v2_0::subsets::any::schema::snapshot::GltfScene, standards::v2_0::subsets::any::schema::snapshot::GltfSkin, standards::v2_0::subsets::any::schema::snapshot::GltfTexture, standards::v2_0::subsets::any::schema::snapshot::GltfTextureInfo};
use crate::standards::v2_0::subsets::any::schema::diff::*;

//#region 🔖️MaterialDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfMaterialDiff {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub pbr_metallic_roughness: Option<Option<GltfPbrMetallicRoughness>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub normal_texture: Option<Option<GltfNormalTextureInfo>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub occlusion_texture: Option<Option<GltfOcclusionTextureInfo>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub emissive_texture: Option<Option<GltfTextureInfo>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub emissive_factor: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub alpha_mode: Option<GltfAlphaMode>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub alpha_cutoff: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub double_sided: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfMaterial> for GltfMaterialDiff {
    fn apply(&self, base: &GltfMaterial, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfMaterial> {
        let mut next = base.clone();
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.pbr_metallic_roughness {
            next.pbr_metallic_roughness = v.clone();
        }
        if let Some(v) = &self.normal_texture {
            next.normal_texture = v.clone();
        }
        if let Some(v) = &self.occlusion_texture {
            next.occlusion_texture = v.clone();
        }
        if let Some(v) = &self.emissive_texture {
            next.emissive_texture = v.clone();
        }
        if let Some(v) = self.emissive_factor {
            next.emissive_factor = v;
        }
        if let Some(v) = self.alpha_mode {
            next.alpha_mode = v;
        }
        if let Some(v) = self.alpha_cutoff {
            next.alpha_cutoff = v;
        }
        if let Some(v) = self.double_sided {
            next.double_sided = v;
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfMaterial) -> Self {
        Self {
            name: self.name.as_ref().map(|_| base.name.clone()),
            pbr_metallic_roughness: self.pbr_metallic_roughness.as_ref().map(|_| base.pbr_metallic_roughness.clone()),
            normal_texture: self.normal_texture.as_ref().map(|_| base.normal_texture.clone()),
            occlusion_texture: self.occlusion_texture.as_ref().map(|_| base.occlusion_texture.clone()),
            emissive_texture: self.emissive_texture.as_ref().map(|_| base.emissive_texture.clone()),
            emissive_factor: self.emissive_factor.map(|_| base.emissive_factor),
            alpha_mode: self.alpha_mode.map(|_| base.alpha_mode),
            alpha_cutoff: self.alpha_cutoff.map(|_| base.alpha_cutoff),
            double_sided: self.double_sided.map(|_| base.double_sided),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.pbr_metallic_roughness.is_some() {
            self.pbr_metallic_roughness = other.pbr_metallic_roughness;
        }
        if other.normal_texture.is_some() {
            self.normal_texture = other.normal_texture;
        }
        if other.occlusion_texture.is_some() {
            self.occlusion_texture = other.occlusion_texture;
        }
        if other.emissive_texture.is_some() {
            self.emissive_texture = other.emissive_texture;
        }
        if other.emissive_factor.is_some() {
            self.emissive_factor = other.emissive_factor;
        }
        if other.alpha_mode.is_some() {
            self.alpha_mode = other.alpha_mode;
        }
        if other.alpha_cutoff.is_some() {
            self.alpha_cutoff = other.alpha_cutoff;
        }
        if other.double_sided.is_some() {
            self.double_sided = other.double_sided;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️MaterialDiff

//#region 🔖️BufferDiff
/// 🔺️ Diff for `document.buffers[i]` METADATA (byteLength/uri/name/ext). The parallel raw-byte
/// payload lives in `GltfSnapshot::buffers` and is diffed separately by
/// [`GltfBufferBytesDiff`]/`buffer_bytes` (WEAK, whole-`Vec<u8>` replace) — see the recipe's
/// explicit "buffers: Vec<Vec<u8>> stays as-is" instruction: two index-aligned collections, not
/// one combined entity, matching the existing snapshot shape exactly.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfBufferDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub byte_length: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub uri: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfBuffer> for GltfBufferDiff {
    fn apply(&self, base: &GltfBuffer, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfBuffer> {
        let mut next = base.clone();
        if let Some(v) = self.byte_length {
            next.byte_length = v;
        }
        if let Some(v) = &self.uri {
            next.uri = v.clone();
        }
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfBuffer) -> Self {
        Self {
            byte_length: self.byte_length.map(|_| base.byte_length),
            uri: self.uri.as_ref().map(|_| base.uri.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        if other.byte_length.is_some() {
            self.byte_length = other.byte_length;
        }
        if other.uri.is_some() {
            self.uri = other.uri;
        }
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️BufferDiff

//#region 🔖️PrimitiveDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfPrimitiveDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<GltfAttributesDelta>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub indices: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub material: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub mode: Option<Option<u64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub targets: Option<GltfTargetsDelta>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfPrimitive> for GltfPrimitiveDiff {
    fn apply(&self, base: &GltfPrimitive, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfPrimitive> {
        let mut next = base.clone();
        if let Some(v) = &self.attributes {
            next.attributes = attribute_pairs(v.commit_onto(&attribute_rows(&base.attributes), capability).map_err(|error| error.under(["attributes"]))?);
        }
        if let Some(v) = &self.indices {
            next.indices = v.clone();
        }
        if let Some(v) = &self.material {
            next.material = v.clone();
        }
        if let Some(v) = &self.mode {
            next.mode = v.clone();
        }
        if let Some(v) = &self.targets {
            next.targets = v.commit_onto(&base.targets, capability).map_err(|error| error.under(["targets"]))?;
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfPrimitive) -> Self {
        Self {
            attributes: self.attributes.as_ref().map(|delta| delta.inverse(&attribute_rows(&base.attributes))),
            indices: self.indices.as_ref().map(|_| base.indices.clone()),
            material: self.material.as_ref().map(|_| base.material.clone()),
            mode: self.mode.as_ref().map(|_| base.mode.clone()),
            targets: self.targets.as_ref().map(|delta| delta.inverse(&base.targets)),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        absorb_slot!(self.attributes, other.attributes);
        if other.indices.is_some() {
            self.indices = other.indices;
        }
        if other.material.is_some() {
            self.material = other.material;
        }
        if other.mode.is_some() {
            self.mode = other.mode;
        }
        absorb_slot!(self.targets, other.targets);
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️PrimitiveDiff

//#region 🔖️TextureDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfTextureDiff {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub sampler: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub source: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfTexture> for GltfTextureDiff {
    fn apply(&self, base: &GltfTexture, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfTexture> {
        let mut next = base.clone();
        if let Some(v) = &self.sampler {
            next.sampler = v.clone();
        }
        if let Some(v) = &self.source {
            next.source = v.clone();
        }
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfTexture) -> Self {
        Self {
            sampler: self.sampler.as_ref().map(|_| base.sampler.clone()),
            source: self.source.as_ref().map(|_| base.source.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        if other.sampler.is_some() {
            self.sampler = other.sampler;
        }
        if other.source.is_some() {
            self.source = other.source;
        }
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️TextureDiff

//#region 🔖️ImageDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfImageDiff {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub uri: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub mime_type: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub buffer_view: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfImage> for GltfImageDiff {
    fn apply(&self, base: &GltfImage, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfImage> {
        let mut next = base.clone();
        if let Some(v) = &self.uri {
            next.uri = v.clone();
        }
        if let Some(v) = &self.mime_type {
            next.mime_type = v.clone();
        }
        if let Some(v) = &self.buffer_view {
            next.buffer_view = v.clone();
        }
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfImage) -> Self {
        Self {
            uri: self.uri.as_ref().map(|_| base.uri.clone()),
            mime_type: self.mime_type.as_ref().map(|_| base.mime_type.clone()),
            buffer_view: self.buffer_view.as_ref().map(|_| base.buffer_view.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        if other.uri.is_some() {
            self.uri = other.uri;
        }
        if other.mime_type.is_some() {
            self.mime_type = other.mime_type;
        }
        if other.buffer_view.is_some() {
            self.buffer_view = other.buffer_view;
        }
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️ImageDiff

//#region 🔖️BufferViewDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfBufferViewDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub buffer: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub byte_offset: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub byte_length: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub byte_stride: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub target: Option<Option<u64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfBufferView> for GltfBufferViewDiff {
    fn apply(&self, base: &GltfBufferView, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfBufferView> {
        let mut next = base.clone();
        if let Some(v) = &self.buffer {
            next.buffer = v.clone();
        }
        if let Some(v) = &self.byte_offset {
            next.byte_offset = v.clone();
        }
        if let Some(v) = &self.byte_length {
            next.byte_length = v.clone();
        }
        if let Some(v) = &self.byte_stride {
            next.byte_stride = v.clone();
        }
        if let Some(v) = &self.target {
            next.target = v.clone();
        }
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfBufferView) -> Self {
        Self {
            buffer: self.buffer.as_ref().map(|_| base.buffer.clone()),
            byte_offset: self.byte_offset.as_ref().map(|_| base.byte_offset.clone()),
            byte_length: self.byte_length.as_ref().map(|_| base.byte_length.clone()),
            byte_stride: self.byte_stride.as_ref().map(|_| base.byte_stride.clone()),
            target: self.target.as_ref().map(|_| base.target.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        if other.buffer.is_some() {
            self.buffer = other.buffer;
        }
        if other.byte_offset.is_some() {
            self.byte_offset = other.byte_offset;
        }
        if other.byte_length.is_some() {
            self.byte_length = other.byte_length;
        }
        if other.byte_stride.is_some() {
            self.byte_stride = other.byte_stride;
        }
        if other.target.is_some() {
            self.target = other.target;
        }
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️BufferViewDiff

//#region 🔖️SkinDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfSkinDiff {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub inverse_bind_matrices: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub skeleton: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub joints: Option<GltfRefsDelta>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfSkin> for GltfSkinDiff {
    fn apply(&self, base: &GltfSkin, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfSkin> {
        let mut next = base.clone();
        if let Some(v) = &self.inverse_bind_matrices {
            next.inverse_bind_matrices = v.clone();
        }
        if let Some(v) = &self.skeleton {
            next.skeleton = v.clone();
        }
        if let Some(v) = &self.joints {
            next.joints = unrefs(v.commit_onto(&refs(&base.joints), capability).map_err(|error| error.under(["joints"]))?);
        }
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfSkin) -> Self {
        Self {
            inverse_bind_matrices: self.inverse_bind_matrices.as_ref().map(|_| base.inverse_bind_matrices.clone()),
            skeleton: self.skeleton.as_ref().map(|_| base.skeleton.clone()),
            joints: self.joints.as_ref().map(|delta| delta.inverse(&refs(&base.joints))),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        if other.inverse_bind_matrices.is_some() {
            self.inverse_bind_matrices = other.inverse_bind_matrices;
        }
        if other.skeleton.is_some() {
            self.skeleton = other.skeleton;
        }
        absorb_slot!(self.joints, other.joints);
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️SkinDiff

//#region 🔖️AnimationDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfAnimationDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub channels: Option<GltfChannelsDelta>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub samplers: Option<GltfAnimationSamplersDelta>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfAnimation> for GltfAnimationDiff {
    fn apply(&self, base: &GltfAnimation, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfAnimation> {
        let mut next = base.clone();
        if let Some(v) = &self.channels {
            next.channels = v.commit_onto(&base.channels, capability).map_err(|error| error.under(["channels"]))?;
        }
        if let Some(v) = &self.samplers {
            next.samplers = v.commit_onto(&base.samplers, capability).map_err(|error| error.under(["samplers"]))?;
        }
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfAnimation) -> Self {
        Self {
            channels: self.channels.as_ref().map(|delta| delta.inverse(&base.channels)),
            samplers: self.samplers.as_ref().map(|delta| delta.inverse(&base.samplers)),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        absorb_slot!(self.channels, other.channels);
        absorb_slot!(self.samplers, other.samplers);
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️AnimationDiff

//#region 🔖️CollectionTypeAliases
pub type GltfScenesDiff = GltfCollectionDiff<GltfScene, GltfSceneDiff>;
pub type GltfNodesDiff = GltfCollectionDiff<GltfNode, GltfNodeDiff>;
pub type GltfMeshesDiff = GltfCollectionDiff<GltfMesh, GltfMeshDiff>;
pub type GltfAccessorsDiff = GltfCollectionDiff<GltfAccessor, GltfAccessorDiff>;
pub type GltfMaterialsDiff = GltfCollectionDiff<GltfMaterial, GltfMaterialDiff>;
pub type GltfBuffersDiff = GltfCollectionDiff<GltfBuffer, GltfBufferDiff>;
pub type GltfBufferViewsDiff = GltfCollectionDiff<GltfBufferView, GltfBufferViewDiff>;
pub type GltfPrimitivesDiff = GltfCollectionDiff<GltfPrimitive, GltfPrimitiveDiff>;
pub type GltfBufferBytesDiff = GltfWeakCollectionDiff<Vec<u8>>;
pub type GltfTexturesDiff = GltfCollectionDiff<GltfTexture, GltfTextureDiff>;
pub type GltfImagesDiff = GltfCollectionDiff<GltfImage, GltfImageDiff>;
pub type GltfSamplersDiff = GltfWeakCollectionDiff<GltfSampler>;
pub type GltfSkinsDiff = GltfCollectionDiff<GltfSkin, GltfSkinDiff>;
pub type GltfAnimationsDiff = GltfCollectionDiff<GltfAnimation, GltfAnimationDiff>;
pub type GltfCamerasDiff = GltfWeakCollectionDiff<GltfCamera>;
//#endregion 🔖️CollectionTypeAliases
