//! 📄️ Document and decoded input of the glTF snapshot: the document root, the component and accessor types and the decoded accessor input. See [`super`].

use super::accessors_and_buffers::{GltfAccessor, GltfBuffer, GltfBufferView};
use super::cameras::GltfCamera;
use super::materials_and_textures::{GltfImage, GltfMaterial, GltfSampler, GltfTexture};
use super::present_json;
use super::scene_graph::{GltfAsset, GltfMesh, GltfNode, GltfScene};
use super::skins_and_animations::{GltfAnimation, GltfSkin};
use super::source_form_and_json::GltfJson;
#[cfg(any(test, feature = "testing"))]
use serde::{Deserialize, Serialize};

//#region 🔖️Document
/// 🌍 The full glTF 2.0 JSON document (§5), fully typed -- one field per spec top-level array/
/// object, `extras`/`extensions` typed via [`GltfJson`], never `serde_json::Value`.
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(Default)]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfDocument {
    pub asset: GltfAsset,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scene: Option<usize>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub scenes: Vec<GltfScene>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<GltfNode>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub meshes: Vec<GltfMesh>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub accessors: Vec<GltfAccessor>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub buffer_views: Vec<GltfBufferView>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub buffers: Vec<GltfBuffer>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub materials: Vec<GltfMaterial>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub textures: Vec<GltfTexture>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<GltfImage>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub samplers: Vec<GltfSampler>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub skins: Vec<GltfSkin>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub animations: Vec<GltfAnimation>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub cameras: Vec<GltfCamera>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty", rename = "extensionsUsed"))]
    #[value(default, skip_serializing_if = "Vec::is_empty", rename = "extensionsUsed")]
    pub extensions_used: Vec<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty", rename = "extensionsRequired"))]
    #[value(default, skip_serializing_if = "Vec::is_empty", rename = "extensionsRequired")]
    pub extensions_required: Vec<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extensions: Option<GltfJson>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extras: Option<GltfJson>,
}
//#endregion 🔖️Document

/// 🔢️ `accessor.componentType` — the 6 values glTF 2.0 permits (§5.1.1).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq)]
#[derive(semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum GltfComponentType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    UnsignedInt,
    Float,
}

/// 🔢️ `accessor.type` — the 7 shapes glTF 2.0 permits (§5.1.2).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq)]
#[derive(semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum GltfAccessorType {
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

impl GltfComponentType {
    pub fn byte_size(self) -> usize {
        match self {
            Self::Byte | Self::UnsignedByte => 1,
            Self::Short | Self::UnsignedShort => 2,
            Self::UnsignedInt | Self::Float => 4,
        }
    }
}

impl GltfAccessorType {
    pub fn components(self) -> usize {
        match self {
            Self::Scalar => 1,
            Self::Vec2 => 2,
            Self::Vec3 => 3,
            Self::Vec4 | Self::Mat2 => 4,
            Self::Mat3 => 9,
            Self::Mat4 => 16,
        }
    }
}

/// 📦️ One decoded accessor: flat row-major `count * accessor_type.components()` values, widened
/// to `f64` and normalized when requested by the accessor before any consumer observes them.
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq)]
pub struct GltfDecodedAccessor {
    pub component_type: GltfComponentType,
    pub accessor_type: GltfAccessorType,
    pub count: usize,
    pub normalized: bool,
    pub components: Vec<f64>,
}

/// 🧮️ Request-scoped geometry input whose accessor values are decoded before semantic inference.
pub struct GltfDecodedSnapshot<'a> {
    pub document:&'a GltfDocument,
    pub accessors:Vec<Result<GltfDecodedAccessor,String>>,
    pub buffer_fingerprints:Vec<String>,
}

impl GltfDecodedSnapshot<'_> {
    pub fn accessor(&self,index:usize)->Result<&GltfDecodedAccessor,String> {
        self.accessors.get(index).ok_or_else(||format!("accessor index {index} out of range"))?.as_ref().map_err(Clone::clone)
    }
}
