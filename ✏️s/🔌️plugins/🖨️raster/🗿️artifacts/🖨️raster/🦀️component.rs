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

//#region 🧩️Composition
/// 🧩️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (design map §4: "raster→C:image layers
/// R:drawing"): a pixel layer's real image bytes used to live INLINE on `RasterSnapshot.assets:
/// BTreeMap<String, RasterImageAsset>` (a duplicated bytes-blob type, never `s.stdio.semio/v1/image`
/// itself). `assets` is now `BTreeMap<String, RasterAssetChild>` — one composed `s.stdio.semio.image`
/// CHILD per asset id, content-addressed, never embedded bytes. `image_key: Option<String>` on
/// `RasterLayerNode::Pixel` is UNCHANGED — it still addresses into this same id-keyed collection, only
/// the map's VALUE type changed from bytes to a handle. `drawing` (`SemioDrawingSnapshot`, used by
/// `🚪️io`'s SVG export/DWG import bridge) was checked and found to be ALREADY a pure, non-persisted IO
/// conversion — raster never owns/duplicates a `drawing` field, it only ever calls stdio's real
/// `SemioDrawingSnapshot`/`DrawNode` types directly at conversion time (`drawing_snapshot_from_raster`/
/// `drawing_snapshot_from_dwg`, `🚪️io/🦀️component.rs`). That already satisfies "consumes/reads drawing
/// content but doesn't own it" — no `ArtifactLink` was needed because there was no persisted/duplicated
/// drawing field to convert.
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;

pub type RasterAssetChild = store::ArtifactChild<SemioImageSnapshot>;

fn mint_asset_child_handle(asset_id: &str, content_hash: u64) -> RasterAssetChild {
    let child_id = format!("raster-asset-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() };
    let target = store::os_io::ArtifactRef { artifact_id: format!("{asset_id}-image"), dialect };
    store::ArtifactChild::new(child_id, target)
}

/// 🕸️ Deterministic content-addressed CHILD handle, hashed off the RAW `(mime, data)` bytes — the
/// fallback shape used only when the bytes can't be decoded into real `SemioImageSnapshot` content
/// (see `mint_and_stash_asset`), and by pure-codec tests that need SOME stable handle without
/// exercising the real png bridge. Prefer `mint_and_stash_asset` at every real call site.
pub fn image_asset_child_handle(asset_id: &str, asset: &RasterImageAsset) -> RasterAssetChild {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    asset.mime.hash(&mut hasher);
    asset.data.hash(&mut hasher);
    mint_asset_child_handle(asset_id, hasher.finish())
}

/// 🕸️ Deterministic content-addressed CHILD handle, hashed off the composed child's own CANONICAL
/// content (`SemioImageSnapshot`'s real pack bytes) rather than the source encoding's raw bytes —
/// this is the handle `mint_and_stash_asset` actually persists whenever decode succeeds. Necessary
/// because two different (but pixel-identical) PNG byte streams — e.g. a hand-authored fixture vs.
/// this plugin's own re-encode of the SAME decoded content — are NOT byte-identical in general
/// (different encoders/compression settings), so hashing raw bytes would mint two different handles
/// for what is honestly the same image; hashing the canonical DECODED content instead makes
/// `decode → cache → re-encode → decode` idempotent at the handle level, which `add-layer-asset`'s
/// inverse (`🧬️mutations/🖇️add-layer-asset/↩️inverse`) depends on to restore the exact prior handle.
fn image_content_child_handle(asset_id: &str, image: &SemioImageSnapshot) -> RasterAssetChild {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    <SemioImageSnapshot as store::ArtifactPack>::encode_pack(image).hash(&mut hasher);
    mint_asset_child_handle(asset_id, hasher.finish())
}

/// 🩹️ Ephemeral working-scene cache (`EngineRep` contract): no `LinkResolver`/child-dispatch seam
/// exists in `ArtifactApp::handle` yet (checked directly against `🔌️plugin/🦀️component.rs`, W1-owned,
/// read-only), so the app layer cannot resolve a `RasterAssetChild` handle back to its real decoded
/// `SemioImageSnapshot` content through the framework. This `thread_local!` bridges that gap — matches
/// `➗️mathematical`'s `MATH_SCRATCH`/`🌊️flow`'s `FLOW_SCRATCH` pattern exactly: keyed by `child_id`
/// (content-addressed, so identical bytes always land in the identical slot), populated at
/// mutation-diff-build time (`mint_and_stash_asset`, called from every `assets` diff-apply site) and at
/// fixture-construction time (`semio_fixture_snapshot`/`empty_raster_document`), read through the ONE
/// `raster_asset` accessor every render/export/inference call site funnels through. **Staleness gap,
/// documented honestly**: store-level undo/redo bypasses `ArtifactApp::handle` entirely, so a live
/// session's cache can go stale relative to a snapshot's `assets` handles across an undo/redo spanning a
/// process boundary; every read fails soft (`None`) on a cache miss, never panics — the same gap every
/// prior exemplar in this ticket (lowpoly/cad/writer/mathematical) documents rather than silently papers
/// over. Never a durable struct field, never derived incrementally from itself, droppable at any instant.
thread_local! {
    static RASTER_SCRATCH: std::cell::RefCell<std::collections::HashMap<String, SemioImageSnapshot>> = std::cell::RefCell::new(std::collections::HashMap::new());
}

pub fn stash_raster_asset(child_id: &str, image: SemioImageSnapshot) {
    RASTER_SCRATCH.with(|cache| { cache.borrow_mut().insert(child_id.to_string(), image); });
}

pub fn cached_raster_asset(child_id: &str) -> Option<SemioImageSnapshot> {
    RASTER_SCRATCH.with(|cache| cache.borrow().get(child_id).cloned())
}

/// 🌉️ The single funnel-through "add real content" primitive: converts the real bytes into the
/// composed child's own real content (`SemioImageSnapshot`, via the real `🚪️io` png↔semio/image
/// bridge — never a stub), mints the CANONICAL content-addressed handle off that decoded content
/// (`image_content_child_handle`), and stashes it into the working-scene cache. An unsupported mime
/// OR undecodable bytes (e.g. a placeholder/malformed PNG in a test fixture) fall back to the
/// raw-bytes handle (`image_asset_child_handle`) and honestly leave the cache slot UNPOPULATED —
/// "no content" is the fail-soft outcome (a clean, documented `raster_asset` cache-miss), never a
/// fabricated placeholder. Every call site that used to do `assets.insert(id, RasterImageAsset{..})`
/// now calls this instead, and gets back only the handle.
pub fn mint_and_stash_asset(asset_id: &str, asset: &RasterImageAsset) -> RasterAssetChild {
    match crate::artifacts::raster::io::semio_image_snapshot_from_raster_asset(asset) {
        Ok(image) => {
            let handle = image_content_child_handle(asset_id, &image);
            stash_raster_asset(&handle.child_id, image);
            handle
        }
        Err(_) => image_asset_child_handle(asset_id, asset),
    }
}

/// 🌉️ The single read accessor every render/export/inference call site funnels through — resolves
/// `asset_id` (the SAME id `image_key` already addressed under the old inline-bytes shape) through the
/// persisted handle map, then through the working-scene cache, then back through the real
/// `SemioImageSnapshot` → `RasterImageAsset` converter. `None` on either a missing handle OR a cold
/// cache — fails soft, documented above, never panics.
pub fn raster_asset(assets: &BTreeMap<String, RasterAssetChild>, asset_id: &str) -> Option<RasterImageAsset> {
    let handle = assets.get(asset_id)?;
    let image = cached_raster_asset(&handle.child_id)?;
    crate::artifacts::raster::io::raster_asset_from_semio_image_snapshot(&image).ok()
}
//#endregion 🧩️Composition

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
