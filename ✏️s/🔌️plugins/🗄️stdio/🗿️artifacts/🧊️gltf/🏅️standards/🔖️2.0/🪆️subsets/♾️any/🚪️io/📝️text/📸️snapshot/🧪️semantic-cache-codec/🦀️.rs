//! 🧪️ Independent serde codec oracle of the semantic camera cache (test and testing feature only).

use crate::standards::v2_0::subsets::any::schema::snapshot::*;
use crate::standards::v2_0::subsets::any::schema::snapshot::GltfAccessorType;
use crate::standards::v2_0::subsets::any::schema::snapshot::GltfComponentType;
use crate::standards::v2_0::subsets::any::schema::snapshot_root::STDIO_GLTF_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
impl Serialize for GltfCameraProjection {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(tag = "type", rename_all = "lowercase")]
        enum Wire<'a> {
            Perspective { perspective: &'a GltfPerspective },
            Orthographic { orthographic: &'a GltfOrthographic },
        }
        match self {
            Self::Perspective(perspective) => Wire::Perspective { perspective }.serialize(serializer),
            Self::Orthographic(orthographic) => Wire::Orthographic { orthographic }.serialize(serializer),
        }
    }
}
impl<'de> Deserialize<'de> for GltfCameraProjection {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive()]
#[cfg_attr(any(test, feature = "testing"), derive(Deserialize))]
        #[cfg_attr(any(test, feature = "testing"), serde(tag = "type", rename_all = "lowercase"))]
        enum Wire {
            Perspective { perspective: GltfPerspective },
            Orthographic { orthographic: GltfOrthographic },
        }
        Ok(match Wire::deserialize(deserializer)? {
            Wire::Perspective { perspective } => Self::Perspective(perspective),
            Wire::Orthographic { orthographic } => Self::Orthographic(orthographic),
        })
    }
}
/// 🌱️ See the identical note on `GltfCameraProjection`'s `Serialize`/`Deserialize` impls above —
/// additive alongside [`dsl::ToValue`]/[`dsl::FromValue`] below, kept unconditional for the same
/// reason.
impl Serialize for GltfCamera {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive()]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize))]
        #[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
        struct Wire<'a> {
            #[cfg_attr(any(test, feature = "testing"), serde(rename = "type"))]
            kind: &'static str,
            #[cfg_attr(any(test, feature = "testing"), serde(skip_serializing_if = "Option::is_none"))]
            perspective: Option<&'a GltfPerspective>,
            #[cfg_attr(any(test, feature = "testing"), serde(skip_serializing_if = "Option::is_none"))]
            orthographic: Option<&'a GltfOrthographic>,
            #[cfg_attr(any(test, feature = "testing"), serde(skip_serializing_if = "Option::is_none"))]
            name: Option<&'a String>,
            #[cfg_attr(any(test, feature = "testing"), serde(skip_serializing_if = "Option::is_none"))]
            extensions: Option<&'a GltfJson>,
            #[cfg_attr(any(test, feature = "testing"), serde(skip_serializing_if = "Option::is_none"))]
            extras: Option<&'a GltfJson>,
        }
        let (kind, perspective, orthographic) = match &self.projection {
            GltfCameraProjection::Perspective(p) => ("perspective", Some(p), None),
            GltfCameraProjection::Orthographic(o) => ("orthographic", None, Some(o)),
        };
        Wire { kind, perspective, orthographic, name: self.name.as_ref(), extensions: self.extensions.as_ref(), extras: self.extras.as_ref() }.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for GltfCamera {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive()]
#[cfg_attr(any(test, feature = "testing"), derive(Deserialize))]
        #[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
        struct Wire {
            #[cfg_attr(any(test, feature = "testing"), serde(rename = "type"))]
            kind: String,
            #[cfg_attr(any(test, feature = "testing"), serde(default))]
            perspective: Option<GltfPerspective>,
            #[cfg_attr(any(test, feature = "testing"), serde(default))]
            orthographic: Option<GltfOrthographic>,
            #[cfg_attr(any(test, feature = "testing"), serde(default))]
            name: Option<String>,
            #[cfg_attr(any(test, feature = "testing"), serde(default))]
            extensions: Option<GltfJson>,
            #[cfg_attr(any(test, feature = "testing"), serde(default))]
            extras: Option<GltfJson>,
        }
        let wire = Wire::deserialize(deserializer)?;
        let projection = match wire.kind.as_str() {
            "perspective" => GltfCameraProjection::Perspective(wire.perspective.ok_or_else(|| serde::de::Error::missing_field("perspective"))?),
            "orthographic" => GltfCameraProjection::Orthographic(wire.orthographic.ok_or_else(|| serde::de::Error::missing_field("orthographic"))?),
            other => return Err(serde::de::Error::custom(format!("camera.type must be 'perspective' or 'orthographic', got {other:?}"))),
        };
        Ok(GltfCamera { projection, name: wire.name, extensions: wire.extensions, extras: wire.extras })
    }
}
