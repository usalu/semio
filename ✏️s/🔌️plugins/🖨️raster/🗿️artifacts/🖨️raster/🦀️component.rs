//! 🖼️ Raster artifact — document entities (constitutional: general).

use std::collections::BTreeMap;

//#region 🔖️Constants
pub const RASTER_DOCUMENT_SCHEMA: &str = "raster.document";
//#endregion 🔖️Constants

//#region 🔖️Types
pub fn default_one() -> f64 {
    1.0
}

pub fn default_true() -> bool {
    true
}

/// 🎞️ Non-destructive raster document: a nested layer tree (pixel/group/adjustment) plus embedded
/// image assets. This is the authoritative projection shared by the wasm compositor bridge and the
/// `raster-plugin` `DocumentApp`. Ephemeral tool/brush/selection/camera state lives in the app's
/// `RasterConfig`, never here.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RasterCamera {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default = "default_one")]
    pub zoom: f64,
}

impl Default for RasterCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

pub fn one_f32() -> f32 {
    1.0
}

pub fn default_blend() -> String {
    "normal".into()
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RasterTransform {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default = "default_one")]
    pub scale_x: f64,
    #[serde(default = "default_one")]
    pub scale_y: f64,
    #[dsl(angle = "deg")]
    #[serde(default)]
    pub rotation: f64,
}

impl Default for RasterTransform {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, scale_x: 1.0, scale_y: 1.0, rotation: 0.0 }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RasterLayerMask {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_true")]
    pub linked: bool,
    #[serde(default)]
    pub invert: bool,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum RasterLayerNode {
    #[serde(rename = "pixel", rename_all = "camelCase")]
    Pixel {
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default = "one_f32")]
        opacity: f32,
        #[dsl(key = "blend")]
        #[serde(default = "default_blend")]
        blend_mode: String,
        #[dsl(block)]
        #[serde(default)]
        transform: RasterTransform,
        #[dsl(block)]
        mask: Option<RasterLayerMask>,
        width: Option<u32>,
        height: Option<u32>,
        #[dsl(key = "image")]
        image_key: Option<String>,
    },
    #[serde(rename = "group", rename_all = "camelCase")]
    Group {
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default = "one_f32")]
        opacity: f32,
        #[dsl(key = "blend")]
        #[serde(default = "default_blend")]
        blend_mode: String,
        #[dsl(block)]
        #[serde(default)]
        transform: RasterTransform,
        #[dsl(block)]
        mask: Option<RasterLayerMask>,
        #[dsl(statements, block)]
        children: Vec<RasterLayerNode>,
    },
    #[serde(rename = "adjustment", rename_all = "camelCase")]
    Adjustment {
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default = "one_f32")]
        opacity: f32,
        #[dsl(key = "blend")]
        #[serde(default = "default_blend")]
        blend_mode: String,
        #[dsl(block)]
        #[serde(default)]
        transform: RasterTransform,
        #[dsl(key = "kind")]
        adjustment_kind: String,
        #[serde(default)]
        params: BTreeMap<String, dsl::DslValue>,
    },
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RasterImageAsset {
    pub mime: String,
    pub data: String,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "raster", layout = "lines")]
pub struct RasterProjection {
    pub schema: String,
    pub id: String,
    #[serde(default)]
    pub title: Option<String>,
    #[dsl(statements, block)]
    #[serde(default)]
    pub layers: Vec<RasterLayerNode>,
    #[serde(default)]
    pub assets: BTreeMap<String, RasterImageAsset>,
}
//#endregion 🔖️Types

//#region 🔖️Operations
/// 🩹️ Sparse patch applied to a single `RasterLayerNode` — the `PatchLayer` operation's payload, and
/// (with fields swapped for their prior values) its own mechanical inverse.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RasterLayerPatch {
    pub name: Option<String>,
    pub visible: Option<bool>,
    pub opacity: Option<f32>,
    #[dsl(key = "blend")]
    pub blend_mode: Option<String>,
    #[dsl(key = "x")]
    pub transform_x: Option<f64>,
    #[dsl(key = "y")]
    pub transform_y: Option<f64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    #[dsl(key = "kind")]
    pub adjustment_kind: Option<String>,
}
//#endregion 🔖️Operations

//#region 🔖️ArtifactKind
/// 🏷️ The `2d.raster` artifact kind — lifted out of `create_raster_app`'s `.artifact_kind(…)` call so
/// both the app manifest and (in the future) any other consumer can share one definition.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "2d.raster".into(),
        name: "2D Raster".into(),
        source_format: "raster.document".into(),
        component_kind: "raster".into(),
        dimension: "2d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Raster },
        schema: RASTER_DOCUMENT_SCHEMA.into(),
        export_formats: vec![semio_framework_plugin::OsMediaFormat::Svg, semio_framework_plugin::OsMediaFormat::Png],
        import_formats: vec![semio_framework_plugin::OsMediaFormat::Svg, semio_framework_plugin::OsMediaFormat::Png],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_kind_keeps_the_media_schema_matching_the_store_schema() {
        assert_eq!(artifact_kind().schema, RASTER_DOCUMENT_SCHEMA);
    }
}
//#endregion 🧪️Tests
