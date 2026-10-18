//! 🧮️ Accessors and buffers of the glTF snapshot: the sparse accessor parts, the accessors, the buffer views and the buffers. See [`super`].

use super::attribute_map_and_defaults::{default_zero_usize, is_false, is_zero_usize};
use super::document_and_decoded_input::{GltfAccessorType, GltfComponentType};
use super::present_json;
use super::source_form_and_json::GltfJson;
#[cfg(any(test, feature = "testing"))]
use serde::{Deserialize, Serialize};

//#region 🔖️Accessor
/// 🧩️ `accessors[i].sparse.indices` (§5.1.3).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfSparseIndices {
    pub buffer_view: usize,
    #[cfg_attr(any(test, feature = "testing"), serde(default = "default_zero_usize", skip_serializing_if = "is_zero_usize"))]
    #[value(default = "default_zero_usize", skip_serializing_if = "is_zero_usize")]
    pub byte_offset: usize,
    pub component_type: GltfComponentType,
}

/// 🧩️ `accessors[i].sparse.values` (§5.1.3).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfSparseValues {
    pub buffer_view: usize,
    #[cfg_attr(any(test, feature = "testing"), serde(default = "default_zero_usize", skip_serializing_if = "is_zero_usize"))]
    #[value(default = "default_zero_usize", skip_serializing_if = "is_zero_usize")]
    pub byte_offset: usize,
}

/// 🧩️ `accessors[i].sparse` (§5.1.3) -- sparse-storage substitution over a (possibly absent, then
/// zero-filled) dense base.
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct GltfSparseAccessor {
    pub count: usize,
    pub indices: GltfSparseIndices,
    pub values: GltfSparseValues,
}

/// 🔢️ `accessors[i]` (§5.1).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfAccessor {
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub buffer_view: Option<usize>,
    #[cfg_attr(any(test, feature = "testing"), serde(default = "default_zero_usize", skip_serializing_if = "is_zero_usize"))]
    #[value(default = "default_zero_usize", skip_serializing_if = "is_zero_usize")]
    pub byte_offset: usize,
    pub component_type: GltfComponentType,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "is_false"))]
    #[value(default, skip_serializing_if = "is_false")]
    pub normalized: bool,
    pub count: usize,
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "type"))]
    #[value(rename = "type")]
    pub kind: GltfAccessorType,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<Vec<f64>>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<Vec<f64>>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub sparse: Option<GltfSparseAccessor>,
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
//#endregion 🔖️Accessor

//#region 🔖️BufferView
/// 🪟️ `bufferViews[i]` (§5.7).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfBufferView {
    pub buffer: usize,
    #[cfg_attr(any(test, feature = "testing"), serde(default = "default_zero_usize", skip_serializing_if = "is_zero_usize"))]
    #[value(default = "default_zero_usize", skip_serializing_if = "is_zero_usize")]
    pub byte_offset: usize,
    pub byte_length: usize,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub byte_stride: Option<usize>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<u64>,
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
//#endregion 🔖️BufferView

//#region 🔖️Buffer
/// 📦️ `buffers[i]` (§5.6) -- JSON-level metadata only; the resolved raw bytes live index-aligned
/// in `GltfSnapshot::buffers` (the legitimate bytes-payload exception).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfBuffer {
    pub byte_length: usize,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
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
//#endregion 🔖️Buffer
