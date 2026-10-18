//! 🎬️ Skins and animations of the glTF snapshot: the skins, the animation paths, channels, interpolation, samplers and animations. See [`super`].

use super::present_json;
use super::source_form_and_json::GltfJson;
#[cfg(any(test, feature = "testing"))]
use serde::{Deserialize, Serialize};

//#region 🔖️Skin
/// 🦴️ `skins[i]` (§5.28).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfSkin {
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub inverse_bind_matrices: Option<usize>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub skeleton: Option<usize>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub joints: Vec<usize>,
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
//#endregion 🔖️Skin

//#region 🔖️Animation
/// 🎞️ `animations[i].channels[j].target.path` (§5.5.2) -- the 4 spec-defined animatable
/// properties.
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[derive(semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum GltfAnimationPath {
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "translation"))]
    #[value(rename = "translation")]
    Translation,
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "rotation"))]
    #[value(rename = "rotation")]
    Rotation,
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "scale"))]
    #[value(rename = "scale")]
    Scale,
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "weights"))]
    #[value(rename = "weights")]
    Weights,
}

/// 🎯️ `animations[i].channels[j].target` (§5.5.2).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfAnimationChannelTarget {
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<usize>,
    pub path: GltfAnimationPath,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extensions: Option<GltfJson>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extras: Option<GltfJson>,
}

/// 🔗️ `animations[i].channels[j]` (§5.5.1).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfAnimationChannel {
    pub sampler: usize,
    pub target: GltfAnimationChannelTarget,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extensions: Option<GltfJson>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extras: Option<GltfJson>,
}

/// 📈️ `animations[i].samplers[j].interpolation` (§5.5.3), default `LINEAR`.
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, Default, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[derive(semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum GltfInterpolation {
    #[default]
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "LINEAR"))]
    #[value(rename = "LINEAR")]
    Linear,
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "STEP"))]
    #[value(rename = "STEP")]
    Step,
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "CUBICSPLINE"))]
    #[value(rename = "CUBICSPLINE")]
    CubicSpline,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn is_linear(v: &GltfInterpolation) -> bool {
    matches!(v, GltfInterpolation::Linear)
}

/// 📈️ `animations[i].samplers[j]` (§5.5.3).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfAnimationSampler {
    pub input: usize,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "is_linear"))]
    #[value(default, skip_serializing_if = "is_linear")]
    pub interpolation: GltfInterpolation,
    pub output: usize,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extensions: Option<GltfJson>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extras: Option<GltfJson>,
}

/// 🎬️ `animations[i]` (§5.5).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfAnimation {
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub channels: Vec<GltfAnimationChannel>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub samplers: Vec<GltfAnimationSampler>,
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
//#endregion 🔖️Animation
