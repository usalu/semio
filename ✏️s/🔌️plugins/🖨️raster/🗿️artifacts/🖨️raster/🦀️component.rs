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
/// `raster-plugin` `ArtifactApp`. Ephemeral tool/brush/selection/camera state lives in the app's
/// `RasterConfig`, never here.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RasterViewportSize {
    pub width: f64,
    pub height: f64,
}

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

mod asset_data_base64 {
    use base64::Engine;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&base64::engine::general_purpose::STANDARD.encode(bytes))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
        let encoded = String::deserialize(deserializer)?;
        base64::engine::general_purpose::STANDARD.decode(encoded.as_bytes()).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RasterImageAsset {
    pub mime: String,
    #[serde(with = "asset_data_base64")]
    #[dsl(base64)]
    pub data: Vec<u8>,
}

/// 📸️ Persisted raster snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
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

pub use crate::artifacts::raster::schema::snapshot::RasterSnapshot;
pub use crate::artifacts::raster::schema::diff::RasterDiff;
pub use crate::artifacts::raster::schema::mutations::RasterMutation;

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
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec!["stdio.svg", "stdio.png"],
        import_stdio_kinds: vec!["stdio.svg", "stdio.png"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1b) —
/// replaces the old side-effecting `register()`, which called four different global registries
/// directly from a plugin `.setup()` callback. `crate::apps::raster::config::schema::
/// register_app_schema()` is the one exception, still called from `🖨️raster/🦀️component.rs`'s own
/// `.setup()`: it registers the `RasterPlayApp` CONFIG schema, an app-scope concern
/// `ArtifactDeclaration` deliberately has no field for (see that struct's own doc) —
/// `register_app_schema_descriptor` is not in the W1 census's artifact-scoped function set.
/// Relocated from `⚙️engine/🦀️component.rs` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// reloc-g3): `⚙️engine` was removed from the taxonomy and `declaration()` describes the artifact,
/// not engine behaviour, so its home is the artifact root alongside `artifact_kind()`. The
/// `io_registry::entries()` call below is now re-qualified onto `subsets::any::io::io_registry`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): the whole `⚙️engine` file this
/// function moved out of has since been dissolved into `🧬️schema/`/`🚪️io/`/the app, per rule 5.
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.raster")
        .schema(crate::artifacts::raster::schema::raster_artifact_schema_descriptor())
        .inferences([crate::artifacts::raster::schema::inferences::raster_artifact_inference_descriptor()])
        .composers(crate::artifacts::raster::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::raster::RasterPlayApp>()
        .build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring
/// 🗒️note's own `pilot_languages()` convention.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "raster.document",
                    extension: Some("raster"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::raster::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::raster::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::raster::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::raster::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("raster.document"),
                },
                dsl::LanguageSpec {
                    id: "raster.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::raster::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::raster::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::raster::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::raster::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("raster.op"),
                },
                dsl::LanguageSpec {
                    id: "raster.document.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::raster::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::raster::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("raster.document.diff"),
                },
                dsl::LanguageSpec {
                    id: "raster.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::raster::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::raster::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("raster.pack"),
                },
                dsl::LanguageSpec {
                    id: "raster.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::raster::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::raster::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("raster.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Register

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
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::raster::standards::v1::subsets::any::io::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("RasterComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
