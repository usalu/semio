//! 📷️ Cameras of the glTF snapshot: the orthographic and perspective projections, the projection enum, the camera and their value codecs. See [`super`].

use super::present_json;
use super::source_form_and_json::GltfJson;
#[cfg(any(test, feature = "testing"))]
use serde::{Deserialize, Serialize};

//#region 🔖️Camera
/// 📷️ `cameras[i].orthographic` (§5.10.1).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfOrthographic {
    pub xmag: f64,
    pub ymag: f64,
    pub zfar: f64,
    pub znear: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extensions: Option<GltfJson>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extras: Option<GltfJson>,
}

/// 📷️ `cameras[i].perspective` (§5.10.2).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub struct GltfPerspective {
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none", rename = "aspectRatio"))]
    #[value(default, skip_serializing_if = "Option::is_none", rename = "aspectRatio")]
    pub aspect_ratio: Option<f64>,
    pub yfov: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub zfar: Option<f64>,
    pub znear: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extensions: Option<GltfJson>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none", with = "present_json", serialize_controlled_with = "present_json::to_value_controlled")]
    pub extras: Option<GltfJson>,
}

/// 🔀️ A camera is EITHER `perspective` OR `orthographic` (§5.10) -- modeled as a tagged union on
/// the sibling `type` string field.
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum GltfCameraProjection {
    Perspective(GltfPerspective),
    Orthographic(GltfOrthographic),
}

/// 📷️ `cameras[i]` (§5.10).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq)]
#[derive(semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct GltfCamera {
    pub projection: GltfCameraProjection,
    pub name: Option<String>,
    pub extensions: Option<GltfJson>,
    pub extras: Option<GltfJson>,
}

/// 🌉️ Hand-written `ToValue`/`FromValue` for the tagged-union `type`+sibling-key wire shape —
/// mirrors the test-only IO Serde oracle exactly (`{"type":
/// "perspective", "perspective": {...}}`), which neither `#[value(tag = "…")]` (fixed content
/// key, not one named after the tag value) nor `#[value(tag = "…", content = "…")]` (same
/// mismatch) can express generically.
impl semio_framework_value::ToValue for GltfCameraProjection {
    fn to_value(&self) -> semio_framework_value::DslValue {
        match self {
            Self::Perspective(perspective) => semio_framework_value::DslValue::object([("type".to_string(), semio_framework_value::DslValue::String("perspective".to_string())), ("perspective".to_string(), semio_framework_value::ToValue::to_value(perspective))]),
            Self::Orthographic(orthographic) => semio_framework_value::DslValue::object([("type".to_string(), semio_framework_value::DslValue::String("orthographic".to_string())), ("orthographic".to_string(), semio_framework_value::ToValue::to_value(orthographic))]),
        }
    }
}

impl semio_framework_value::FromValue for GltfCameraProjection {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let entries = semio_framework_value::DslValue::into_object(value)?;
        let kind = entries.iter().find(|(k, _)| k == "type").map(|(_, v)| v.clone()).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field `type`"))?;
        let kind = match kind {
            semio_framework_value::DslValue::String(s) => s,
            other => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        };
        match kind.as_str() {
            "perspective" => {
                let payload = entries.iter().find(|(k, _)| k == "perspective").map(|(_, v)| v.clone()).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field `perspective`"))?;
                Ok(Self::Perspective(semio_framework_value::FromValue::from_value(payload)?))
            }
            "orthographic" => {
                let payload = entries.iter().find(|(k, _)| k == "orthographic").map(|(_, v)| v.clone()).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field `orthographic`"))?;
                Ok(Self::Orthographic(semio_framework_value::FromValue::from_value(payload)?))
            }
            other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("camera.type must be 'perspective' or 'orthographic', got {other:?}"))),
        }
    }
}

/// 🌉️ Hand-written `ToValue`/`FromValue` for `GltfCamera` — flattens `projection`'s own
/// `type`+sibling-key entries alongside `name`/`extensions`/`extras`, mirroring the hand-rolled
/// test-only IO Serde oracle.
impl semio_framework_value::ToValue for GltfCamera {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries = match semio_framework_value::ToValue::to_value(&self.projection) {
            semio_framework_value::DslValue::Object(entries) => entries,
            other => vec![("projection".to_string(), other)],
        };
        if let Some(name) = &self.name {
            entries.push(("name".to_string(), semio_framework_value::ToValue::to_value(name)));
        }
        if let Some(extensions) = &self.extensions {
            entries.push(("extensions".to_string(), semio_framework_value::ToValue::to_value(extensions)));
        }
        if let Some(extras) = &self.extras {
            entries.push(("extras".to_string(), semio_framework_value::ToValue::to_value(extras)));
        }
        semio_framework_value::DslValue::Object(entries)
    }
}

impl semio_framework_value::FromValue for GltfCamera {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let entries = semio_framework_value::DslValue::into_object(value)?;
        let projection = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::Object(entries.clone()))?;
        let name = match entries.iter().find(|(k, _)| k == "name") {
            Some((_, v)) => semio_framework_value::FromValue::from_value(v.clone())?,
            None => None,
        };
        let extensions = match entries.iter().find(|(k, _)| k == "extensions") {
            Some((_, v)) => semio_framework_value::FromValue::from_value(v.clone())?,
            None => None,
        };
        let extras = match entries.iter().find(|(k, _)| k == "extras") {
            Some((_, v)) => semio_framework_value::FromValue::from_value(v.clone())?,
            None => None,
        };
        Ok(GltfCamera { projection, name, extensions, extras })
    }
}
//#endregion 🔖️Camera
