//! 🧬️ Owned glTF 2.0 document, buffer records, and decoded accessor input.
//! Native JSON, GLB, URI, and accessor byte codecs belong to IO.
//! Serde derives are test-only independent codec oracles, compiled for this crate's tests and, through the
//! `testing` feature, for the dependent crates' tests.

#[cfg(any(test, feature = "testing"))]
mod ordered_attr_map {
    use serde::de::{MapAccess, Visitor};
    use serde::ser::SerializeMap;
    use serde::{Deserializer, Serializer};
    use std::fmt;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn serialize<S: Serializer>(attrs: &[(String, usize)], serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(attrs.len()))?;
        for (k, v) in attrs {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }

    struct AttrVisitor;
    impl<'de> Visitor<'de> for AttrVisitor {
        type Value = Vec<(String, usize)>;
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("a JSON object mapping attribute semantic to accessor index")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut out = Vec::new();
            while let Some((k, v)) = map.next_entry::<String, usize>()? {
                out.push((k, v));
            }
            Ok(out)
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<(String, usize)>, D::Error> {
        deserializer.deserialize_map(AttrVisitor)
    }
}

#[path = "🌱️value/🧵️canonical/🦀️.rs"]
mod canonical;

//#region 🔖️SourceForm

#[path = "🧵️source-form-and-json/🦀️.rs"]
mod source_form_and_json;
pub use source_form_and_json::{GltfJson, GltfSourceForm};

#[path = "🔢️attribute-map-and-defaults/🦀️.rs"]
mod attribute_map_and_defaults;
use attribute_map_and_defaults::{is_default_alpha_cutoff, is_default_wrap, is_false, is_one_f64, is_vec3_zero, is_vec4_one, is_zero_u64, is_zero_usize};

#[path = "🌳️scene-graph/🦀️.rs"]
mod scene_graph;
pub use scene_graph::{GltfAsset, GltfMesh, GltfMorphTarget, GltfNode, GltfPrimitive, GltfScene};

#[path = "🧮️accessors-and-buffers/🦀️.rs"]
mod accessors_and_buffers;
pub use accessors_and_buffers::{GltfAccessor, GltfBuffer, GltfBufferView, GltfSparseAccessor, GltfSparseIndices, GltfSparseValues};

#[path = "🎨️materials-and-textures/🦀️.rs"]
mod materials_and_textures;
use materials_and_textures::is_opaque;
pub use materials_and_textures::{GltfAlphaMode, GltfImage, GltfMaterial, GltfNormalTextureInfo, GltfOcclusionTextureInfo, GltfPbrMetallicRoughness, GltfSampler, GltfTexture, GltfTextureInfo};

#[path = "🎬️skins-and-animations/🦀️.rs"]
mod skins_and_animations;
use skins_and_animations::is_linear;
pub use skins_and_animations::{GltfAnimation, GltfAnimationChannel, GltfAnimationChannelTarget, GltfAnimationPath, GltfAnimationSampler, GltfInterpolation, GltfSkin};

#[path = "📷️cameras/🦀️.rs"]
mod cameras;
pub use cameras::{GltfCamera, GltfCameraProjection, GltfOrthographic, GltfPerspective};

#[path = "📄️document-and-decoded-input/🦀️.rs"]
mod document_and_decoded_input;
pub use document_and_decoded_input::{GltfAccessorType, GltfComponentType, GltfDecodedAccessor, GltfDecodedSnapshot, GltfDocument};

//#endregion 🔖️SourceForm

//#region 🔖️GltfJson

/// 🕳️ The field codec EVERY `extras`/`extensions` slot uses, because absence and a present JSON
/// `null` are DIFFERENT states here and the blanket `impl<T: FromValue> FromValue for Option<T>`
/// folds both into `None`. `change-node-extra-data`'s own vocabulary spells the distinction out
/// (`GltfDataPresence::{Absent, Present { value }}`, and `Present { value: Null }` is a legal,
/// separately-validated state), so a snapshot that loses it cannot round-trip its own mutations —
/// `"extras": null` decoded to `None`, re-encoded to nothing, and the canonical vector's projection
/// failed on the missing key. Paired with the field's bare `#[value(default)]`, a MISSING key still
/// decodes to `None`; only a present one reaches this.
pub mod present_json {
    use super::GltfJson;

    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn to_value_controlled(value: &Option<GltfJson>, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_value::DslValue, semio_framework_value::ValueError> {
        match value { Some(value) => semio_framework_value::ToValue::to_value_controlled(value, control), None => semio_framework_value::ToValue::to_value_controlled(&(), control) }
    }

    pub fn to_value(value: &Option<GltfJson>) -> semio_framework_value::DslValue {
        match value {
            Some(inner) => semio_framework_value::ToValue::to_value(inner),
            None => semio_framework_value::DslValue::Null,
        }
    }

    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn from_value(value: semio_framework_value::DslValue) -> Result<Option<GltfJson>, semio_framework_value::ValueError> {
        <GltfJson as semio_framework_value::FromValue>::from_value(value).map(Some)
    }
}
//#endregion 🔖️GltfJson
