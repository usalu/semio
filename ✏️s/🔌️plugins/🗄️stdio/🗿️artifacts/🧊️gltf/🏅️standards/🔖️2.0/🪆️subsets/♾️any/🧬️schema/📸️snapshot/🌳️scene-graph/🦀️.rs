//! 🌳️ Scene graph of the glTF snapshot: the asset, scenes, nodes, morph targets, mesh primitives and meshes. See [`super`].

use super::attribute_map_and_defaults::{ordered_attr_map_from_value, ordered_attr_map_to_value, ordered_attr_map_to_value_controlled};
#[cfg(any(test, feature = "testing"))]
use super::ordered_attr_map;
use super::present_json;
use super::source_form_and_json::GltfJson;
#[cfg(any(test, feature = "testing"))]
use serde::{Deserialize, Serialize};

//#region 🔖️Asset
/// 📛 `asset` (§3.9) — the one universally mandatory glTF object; `version` is the one mandatory
/// field on it.
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfAsset {
    pub version: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub generator: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none", rename = "minVersion"))]
    #[value(default, skip_serializing_if = "Option::is_none", rename = "minVersion")]
    pub min_version: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extensions: Option<GltfJson>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extras: Option<GltfJson>,
}

impl Default for GltfAsset {
    fn default() -> Self {
        Self { version: "2.0".into(), generator: None, copyright: None, min_version: None, extensions: None, extras: None }
    }
}
//#endregion 🔖️Asset

//#region 🔖️Scene
/// 🎬 `scenes[i]` (§5.26).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfScene {
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<usize>,
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
//#endregion 🔖️Scene

//#region 🔖️Node
/// 🧍 `nodes[i]` (§5.25). `matrix`/`translation`+`rotation`+`scale` are mutually exclusive per
/// spec but modeled as independently-optional fields rather than an enum -- a real document that
/// (incorrectly) carries both should still round-trip losslessly, and this shape keeps the diff
/// symmetric with every other nullable field instead of needing a `Replace`-only whole-transform
/// diff.
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfNode {
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<usize>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<usize>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera: Option<usize>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub skin: Option<usize>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<[f64; 16]>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub translation: Option<[f64; 3]>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<[f64; 4]>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<[f64; 3]>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub weights: Vec<f64>,
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
//#endregion 🔖️Node

//#region 🔖️Mesh
/// 🎭 One `meshes[i].primitives[j].targets[k]` morph-target attribute map (§5.19.4).
/// 🩹 Hand-written `ToValue`/`FromValue` (not `#[derive(..., transparent)]`): `#[value(transparent)]`
/// forwards straight to the raw `Vec<(String, usize)>` field's OWN `ToValue`/`FromValue` (a
/// 2-element-array-per-entry encoding), bypassing the `ordered_attr_map` object-shaped encoding
/// this type actually needs — same wire shape [`GltfPrimitive::attributes`] uses below.
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(transparent))]
pub struct GltfMorphTarget(#[cfg_attr(any(test, feature = "testing"), serde(with = "ordered_attr_map"))] pub Vec<(String, usize)>);

impl semio_framework_value::ToValue for GltfMorphTarget {
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_value::DslValue, semio_framework_value::ValueError> { ordered_attr_map_to_value_controlled(&self.0, control) }
    fn to_value(&self) -> semio_framework_value::DslValue {
        ordered_attr_map_to_value(&self.0)
    }
}

impl semio_framework_value::FromValue for GltfMorphTarget {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Ok(GltfMorphTarget(ordered_attr_map_from_value(value)?))
    }
}

/// 🔺 `meshes[i].primitives[j]` (§5.19.4).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct GltfPrimitive {
    #[cfg_attr(any(test, feature = "testing"), serde(default, with = "ordered_attr_map"))]
    #[value(default, serialize_with = "ordered_attr_map_to_value", deserialize_with = "ordered_attr_map_from_value", serialize_controlled_with = "ordered_attr_map_to_value_controlled")]
    pub attributes: Vec<(String, usize)>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub indices: Option<usize>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub material: Option<usize>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<u64>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<GltfMorphTarget>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extensions: Option<GltfJson>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extras: Option<GltfJson>,
}

/// 🕸️ `meshes[i]` (§5.19).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfMesh {
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub primitives: Vec<GltfPrimitive>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub weights: Vec<f64>,
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
//#endregion 🔖️Mesh
