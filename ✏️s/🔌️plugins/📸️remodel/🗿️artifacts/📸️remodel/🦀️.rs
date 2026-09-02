//! 📸️ Remodel scene document — schema-only photogrammetry/videogrammetry project state (media
//! streams, calibration, ground control points, reconstruction params/job/results) shared as CRDT
//! operations. The actual algorithms live in the editor surface's own `✏️editor/⚙️engine/` topic files
//! (`images`/`video`/`camera`/`feature`/`sfm`/`dense`/`mesh`/`motion`/`geo`/`reconstruction`,
//! relocated out of this artifact tree by 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES,
//! #2553 — an artifact is a schema plus IO, never an engine), none of which this node references:
//! heavier runtime types (`Se3`, `Intrinsics`, `Distortion`, `WatertightReport`, decoded pyramids,
//! match graphs, depth maps, TSDF volumes) are not designed for durable CRDT persistence, so every
//! reference to their shape below is a plain-JSON (or `Packed*`) snapshot the app fills in, never the
//! library type itself.

use semio_framework::MeshData;
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, OnceLock};

//#region 🔖️ArtifactKind
/// 🗿️ The `3d.remodel` artifact kind — lifted verbatim out of the manifest builder's
/// `.artifact_kind(…)` literal so the artifact node, not the app, owns its own identity.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "3d.remodel".into(),
        name: "3D Remodel".into(),
        source_format: "remodel.scene".into(),
        component_kind: "remodel".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        schema: "remodel.scene".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl"],
        import_stdio_kinds: vec!["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called five different global registries directly from
/// a plugin `.setup()` callback. `crate::editor::remodel::config::schema::register_app_schema()` is the
/// one exception, still called from `📸️remodel/🦀️.rs`'s own `.setup()`: it registers the
/// `RemodelPlayApp` CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration` deliberately
/// has no field for (see that struct's own doc) — `register_app_schema_descriptor` is not in §6's
/// artifact-scoped function set. Relocated from `⚙️engine/🦀️.rs` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g3): `⚙️engine` was removed from the taxonomy
/// and `declaration()` describes the artifact, not engine behaviour, so its home is the artifact
/// root alongside `artifact_kind()`.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.remodel.standard.v1", "standard", "1", &[], None),
        ("s.remodel.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.remodel.schema.artifact", "schema", "s.remodel.remodel", &[("schema", "s.remodel.remodel")], None),
        ("s.remodel.inference.artifact", "inference", "s.remodel.remodel.inference", &[("schema", "s.remodel.remodel.inference")], None),
        ("s.remodel.composer.native", "composer", "s.remodel@1/*", &[("dialect", "s.remodel@1/*")], None),
        ("s.remodel.composer.format-1", "composer", "s.stdio.las@1.0/*", &[("dialect", "s.stdio.las@1.0/*")], None),
        ("s.remodel.composer.format-2", "composer", "s.stdio.ply@1.0/*", &[("dialect", "s.stdio.ply@1.0/*")], None),
        ("s.remodel.composer.format-3", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.remodel.composer.format-4", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.remodel.composer.format-5", "composer", "s.stdio.dwg@ac1018/*", &[("dialect", "s.stdio.dwg@ac1018/*")], None),
        ("s.remodel.composer.format-6", "composer", "s.stdio.stl@ascii/*", &[("dialect", "s.stdio.stl@ascii/*")], None),
        ("s.remodel.composer.format-7", "composer", "s.stdio.gltf@2.0/*", &[("dialect", "s.stdio.gltf@2.0/*")], None),
        ("s.remodel.composer.format-8", "composer", "s.stdio.obj@3.0/*", &[("dialect", "s.stdio.obj@3.0/*")], None),
        ("s.remodel.grammar.1", "grammar", "remodel.document", &[("grammar", "remodel.document")], None),
        ("s.remodel.grammar.2", "grammar", "remodel.op", &[("grammar", "remodel.op")], None),
        ("s.remodel.grammar.3", "grammar", "remodel.diff", &[("grammar", "remodel.diff")], None),
        ("s.remodel.grammar.4", "grammar", "remodel.pack", &[("grammar", "remodel.pack")], None),
        ("s.remodel.grammar.5", "grammar", "remodel.spr", &[("grammar", "remodel.spr")], None),
        ("s.remodel.codec.document-1", "codec", "remodel.scene:remodel", &[("codec", "remodel.scene"), ("extension", "remodel")], None),
        ("s.remodel.localization.en", "localization", "Remodel", &[], Some(("en", "Remodel"))),
        ("s.remodel.localization.de", "localization", "Umbau", &[], Some(("de", "Umbau"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.remodel")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::remodel::schema::remodel_artifact_schema_descriptor())
        .inferences([crate::artifacts::remodel::standards::v1::subsets::any::schema::inferences::remodel_artifact_inference_descriptor()])
        .composers(crate::artifacts::remodel::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::remodel::RemodelPlayApp>>()
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "remodel.document",
                    extension: Some("remodel"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::remodel::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::remodel::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::remodel::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::remodel::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("remodel.document"),
                },
                dsl::LanguageSpec {
                    id: "remodel.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::remodel::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::remodel::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::remodel::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::remodel::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("remodel.op"),
                },
                dsl::LanguageSpec {
                    id: "remodel.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::remodel::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::remodel::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("remodel.diff"),
                },
                dsl::LanguageSpec {
                    id: "remodel.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::remodel::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::remodel::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("remodel.pack"),
                },
                dsl::LanguageSpec {
                    id: "remodel.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::remodel::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::remodel::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("remodel.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Register

pub use crate::artifacts::remodel::schema::mutations::RemodelMutation;

pub use crate::artifacts::remodel::schema::diff::RemodelDiff;

pub const REMODEL_DOCUMENT_SCHEMA: &str = "remodel.scene";

/// 🪪️ Ticket `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` contract §1 canonical surface id
/// grammar (`<artifact_kind>@<standard>/<subset>#<role>`). Lives at the ARTIFACT level (not under
/// `editor`/`viewer`) so a viewer file can read it without ever importing through the sibling editor
/// module. `artifact_kind = "s.remodel.remodel"` matches this artifact's own `definition()` capability
/// row `("s.remodel.schema.artifact", "schema", "s.remodel.remodel", …)` above — the schema-artifact
/// descriptor, not `artifact_kind()`'s OS-level `"3d.remodel"` kind id (a different, unrelated
/// namespace). `standard`/`subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location.
pub const REMODEL_DIALECT: semio_framework_plugin::Dialect = semio_framework_plugin::Dialect { artifact_kind: "s.remodel.remodel", standard: semio_framework_plugin::StandardId("1"), subset: semio_framework_plugin::SubsetId::ANY };

//#region 🧩️Composition
/// 🧩️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (design map §4: "remodel→C:mesh R:image").
/// Two content-duplication shapes, both verified against real code (not assumed from the one-line
/// design summary):
///
/// 1. **`results.mesh.mesh` (was `MeshData`, the reconstructed/placeholder mesh's flat buffers) is now
///    a composed `s.stdio.semio/v1/mesh` CHILD** (`RemodelMeshChild = store::ArtifactChild<
///    SemioMeshSnapshot>`) — exactly `💠️lowpoly`'s own pattern (this ticket's closest precedent for
///    opaque mesh composition), extended with a REAL bidirectional converter
///    (`crate::artifacts::remodel::standards::v1::subsets::any::io::{mesh_data_to_semio_mesh,
///    semio_mesh_to_mesh_data}`, already real — the export path already built the forward direction).
///
/// 2. **`assets: BTreeMap<String, ImageAsset>` (embedded mime+base64 pixel bytes: video frames,
///    baked mesh textures, DSM/DTM/ortho rasters) is now `BTreeMap<String, RemodelAssetChild>`
///    (`RemodelAssetChild = store::ArtifactChild<SemioImageSnapshot>`)** — the design line's "R:image"
///    literally means `ArtifactLink` (an INDEPENDENT-lifecycle reference, `store::ArtifactLink`'s own
///    doc comment: "renders as a chip, never nests inline"). `remodel`'s assets are NOT independent
///    documents referenced from elsewhere — they are embedded content OWNED by this exact document
///    (keyed by an id that only this document's own `MediaStream.frames`/`RemodelMesh.
///    texture_asset_id`/`GeoProducts` fields ever address), the identical shape `🖨️raster`'s own
///    `assets: BTreeMap<String, RasterImageAsset>` had — and raster's own migration (this ticket,
///    same design map, "raster→C:image layers R:drawing") converted that shape to a composed CHILD,
///    not a link, documenting exactly this reasoning in place. Followed here for the same reason:
///    composing (owned CHILD) is the honest verb for content this document owns and mutates through
///    its own `create-asset`/`delete-asset` triad; `ArtifactLink` would be dishonest (there is no
///    independent target document to pin/reference).
///
/// **Schema-introspection gap, documented and accepted** (matches raster's/lowpoly's own identical
/// gap): `#[derive(ArtifactSchema)]`'s `#[child(kind=...)]` mechanism only recognizes a bare
/// `ArtifactChild<T>`/`Vec<ArtifactChild<T>>` field declared DIRECTLY on the struct it derives — not a
/// `BTreeMap` value (`assets`) and not a field nested two levels deep inside `results.mesh.mesh`. Kept
/// as-is (not reshaped) to preserve the exact addressing every existing mutation already assumes
/// (`image_key`-equivalent lookups by asset id; `ReplaceMeshResult`'s whole-`RemodelMesh` payload
/// shape) — the type/mutation/persistence layer is fully real, only the derive's SCHEMA
/// INTROSPECTION table is incomplete for these two fields.
pub type RemodelAssetChild = store::ArtifactChild<SemioImageSnapshot>;
pub type RemodelMeshChild = store::ArtifactChild<SemioMeshSnapshot>;

/// 🧩️ Restart-stable content authority. Every encoded leaf decodes to at most 4 KiB; values carry
/// only bounded metadata and content-addressed leaves, never a whole image or mesh object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelDurableArtifact {
    pub kind: String,
    pub mime: Option<String>,
    pub width: u32,
    pub height: u32,
    #[dsl(table)]
    pub chunks: Vec<String>,
}

pub type RemodelDurableArtifactStore = BTreeMap<String, RemodelDurableArtifact>;

const REMODEL_DURABLE_CHUNK_RAW_BYTES: usize = 4_096;
const REMODEL_MAX_STAGED_BLOBS: usize = 32;
const REMODEL_SPARSE_CONTENT_BYTES: usize = 512 * 3 * 4;
const REMODEL_SPARSE_CONTENT_CHUNKS: u64 = 2;
const REMODEL_RASTER_CONTENT_BYTES: usize = 1_114_112;
const REMODEL_RASTER_CONTENT_CHUNKS: u64 = 272;
const REMODEL_MESH_CONTENT_BYTES: usize = 87_552 + 30;
const REMODEL_MESH_CONTENT_CHUNKS: u64 = 30;
const REMODEL_EMPTY_MESH_CHILD_ID: &str = "remodel-mesh-constant-empty";
const REMODEL_BOX_MESH_CHILD_ID: &str = "remodel-mesh-constant-box";
const REMODEL_BOUNDED_MESH_VERTICES: usize = 512;
const REMODEL_BOUNDED_MESH_TRIANGLES: usize = 512;

//#region 🔖️AssetHandles
fn mint_asset_child_handle(asset_id: &str, content_hash: u64) -> RemodelAssetChild {
    let child_id = format!("remodel-asset-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() };
    let target = store::os_io::ArtifactRef { artifact_id: format!("{asset_id}-image"), dialect };
    store::ArtifactChild::new(child_id, target)
}

pub fn committed_remodel_asset_handle(asset_id: &str, content_id: &str) -> RemodelAssetChild {
    let mut handle = mint_asset_child_handle(asset_id, 0);
    handle.child_id = content_id.into();
    handle
}

/// 🕸️ Deterministic content-addressed CHILD handle for one bounded durable asset.
pub fn image_asset_child_handle(asset_id: &str, asset: &ImageAsset) -> RemodelAssetChild {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    asset.mime.hash(&mut hasher);
    asset.data.hash(&mut hasher);
    mint_asset_child_handle(asset_id, hasher.finish())
}

/// 🧩️ Admits a normal imported asset as independently bounded raw leaves. No whole `ImageAsset`
/// value is retained outside the typed mutation currently being reduced.
pub fn store_remodel_asset(asset_id: &str, asset: &ImageAsset) -> RemodelAssetChild {
    image_asset_child_handle(asset_id, asset)
}

/// 🧶️ Snapshot-owned decoded-leaf view for bounded consumers. Identity and display metadata stay
/// separate from the compressed payload, and no leaf can exceed 4 KiB.
#[derive(Clone)]
pub struct RemodelAssetChunkSource {
    pub identity: String,
    pub mime: String,
    pub width: u32,
    pub height: u32,
    pub leaves: Vec<Arc<[u8]>>,
    pub byte_len: usize,
}

pub fn remodel_asset_chunk_source(snapshot: &RemodelSnapshot, asset_id: &str) -> Option<RemodelAssetChunkSource> {
    let handle = snapshot.assets.get(asset_id)?;
    let artifact = snapshot.durable_artifacts.get(&handle.child_id).filter(|artifact| artifact.kind == "image")?;
    let mime = artifact.mime.clone()?;
    let mut leaves = Vec::with_capacity(artifact.chunks.len());
    let mut byte_len = 0usize;
    for encoded in &artifact.chunks {
        let chunk = decode_remodel_durable_chunk(encoded)?;
        byte_len = byte_len.checked_add(chunk.len())?;
        if byte_len > REMODEL_RASTER_CONTENT_BYTES {
            return None;
        }
        leaves.push(Arc::from(chunk));
    }
    (!leaves.is_empty()).then(|| RemodelAssetChunkSource { identity: handle.child_id.clone(), mime, width: artifact.width, height: artifact.height, leaves, byte_len })
}

pub fn remodel_asset_dimensions(snapshot: &RemodelSnapshot, asset_id: &str) -> Option<(u32, u32)> {
    let handle = snapshot.assets.get(asset_id)?;
    let artifact = snapshot.durable_artifacts.get(&handle.child_id).filter(|artifact| artifact.kind == "image")?;
    Some((artifact.width, artifact.height))
}

/// 🖼️ Reconstructs a bounded API value for export and UI presentation only. Active reconstruction
/// consumes `remodel_asset_chunk_source` directly and never passes through this whole-value facade.
pub fn remodel_asset(snapshot: &RemodelSnapshot, asset_id: &str) -> Option<ImageAsset> {
    let source = remodel_asset_chunk_source(snapshot, asset_id)?;
    let mut bytes = Vec::with_capacity(source.byte_len);
    for leaf in source.leaves {
        bytes.extend_from_slice(&leaf);
    }
    Some(ImageAsset { mime: source.mime, data: base64_codec::base64_standard_encode(bytes), width: source.width, height: source.height })
}

pub fn durable_remodel_asset(asset: &ImageAsset) -> Option<RemodelDurableArtifact> {
    let bytes = base64_codec::base64_standard_decode(asset.data.as_bytes()).ok()?;
    (bytes.len() <= REMODEL_RASTER_CONTENT_BYTES).then_some(RemodelDurableArtifact {
        kind: "image".into(),
        mime: Some(asset.mime.clone()),
        width: asset.width,
        height: asset.height,
        chunks: bytes.chunks(REMODEL_DURABLE_CHUNK_RAW_BYTES).map(|chunk| base64_codec::base64_standard_encode(chunk)).collect(),
    })
}

pub fn durable_staged_remodel_asset(staging_id: &str, kind: &str, mime: Option<String>, width: u32, height: u32) -> Option<RemodelDurableArtifact> {
    let content = remodel_asset_content().lock().expect("remodel asset content lock");
    let blob = content.get(staging_id)?;
    Some(RemodelDurableArtifact {
        kind: kind.into(),
        mime,
        width,
        height,
        chunks: (0..u64::try_from(blob.chunks.len()).ok()?).map(|index| base64_codec::base64_standard_encode(blob.chunks.get(&index).expect("contiguous staged asset"))).collect(),
    })
}

//#region 🔖️ReplayableAssetBlobs
struct RemodelAssetBlob {
    chunks: BTreeMap<u64, Arc<[u8]>>,
    kind: Option<RemodelAssetContentKind>,
    byte_count: usize,
    digest: [u64; 4],
    digest_len: u64,
}

impl Default for RemodelAssetBlob {
    fn default() -> Self {
        Self { chunks: BTreeMap::new(), kind: None, byte_count: 0, digest: [0x6c62272e07bb0142, 0x62b821756295c58d, 0x9e3779b185ebca87, 0xc2b2ae3d27d4eb4f], digest_len: 0 }
    }
}

/// 🛡️ Typed bounded-admission result shared by durable asset and mesh staging.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RemodelStagingFault {
    Busy,
    Invalid,
}

/// 🏷️ Exact durable asset envelope selected before the first chunk is retained.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RemodelAssetContentKind {
    Sparse,
    Raster,
}

impl RemodelAssetContentKind {
    fn wire(self) -> &'static str {
        match self {
            Self::Sparse => "sparse",
            Self::Raster => "raster",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        match value {
            "sparse" => Some(Self::Sparse),
            "raster" => Some(Self::Raster),
            _ => None,
        }
    }

    fn max_bytes(self) -> usize {
        match self {
            Self::Sparse => REMODEL_SPARSE_CONTENT_BYTES,
            Self::Raster => REMODEL_RASTER_CONTENT_BYTES,
        }
    }

    fn max_chunks(self) -> u64 {
        match self {
            Self::Sparse => REMODEL_SPARSE_CONTENT_CHUNKS,
            Self::Raster => REMODEL_RASTER_CONTENT_CHUNKS,
        }
    }
}

fn record_content_digest(digest: &mut [u64; 4], digest_len: &mut u64, bytes: &[u8]) -> Result<(), RemodelStagingFault> {
    for byte in bytes {
        *digest_len = (*digest_len).checked_add(1).ok_or(RemodelStagingFault::Busy)?;
        digest[0] = (digest[0] ^ u64::from(*byte)).wrapping_mul(0x00000100000001b3);
        digest[1] = (digest[1] ^ digest[0].rotate_left(17) ^ *digest_len).wrapping_mul(0x9e3779b185ebca87);
        digest[2] = (digest[2] ^ digest[1].rotate_left(29) ^ u64::from(*byte)).wrapping_mul(0xc2b2ae3d27d4eb4f);
        digest[3] = (digest[3] ^ digest[2].rotate_left(41) ^ (*digest_len).rotate_left(7)).wrapping_mul(0x165667b19e3779f9);
    }
    Ok(())
}

fn content_digest_id(prefix: &str, digest: [u64; 4], digest_len: u64) -> String {
    format!("{prefix}-{:016x}{:016x}{:016x}{:016x}-{digest_len:016x}", digest[0], digest[1], digest[2], digest[3])
}

static REMODEL_PRIVATE_ASSET_STAGING: OnceLock<Mutex<BTreeMap<String, RemodelAssetBlob>>> = OnceLock::new();

fn remodel_asset_content() -> &'static Mutex<BTreeMap<String, RemodelAssetBlob>> {
    REMODEL_PRIVATE_ASSET_STAGING.get_or_init(|| Mutex::new(BTreeMap::new()))
}

pub fn stage_remodel_asset_chunk(staging_id: &str, kind: RemodelAssetContentKind, index: u64, encoded: &str) -> Result<(), RemodelStagingFault> {
    let Some(bytes) = decode_remodel_durable_chunk(encoded) else { return Err(RemodelStagingFault::Invalid) };
    let next_count = index.checked_add(1).ok_or(RemodelStagingFault::Busy)?;
    let mut content = remodel_asset_content().lock().expect("remodel asset content lock");
    if !content.contains_key(staging_id) && content.len() >= REMODEL_MAX_STAGED_BLOBS {
        return Err(RemodelStagingFault::Busy);
    }
    let blob = content.entry(staging_id.into()).or_default();
    if let Some(existing) = blob.chunks.get(&index) {
        return (blob.kind == Some(kind) && existing.as_ref() == bytes).then_some(()).ok_or(RemodelStagingFault::Invalid);
    }
    let next_bytes = blob.byte_count.checked_add(bytes.len()).ok_or(RemodelStagingFault::Busy)?;
    if u64::try_from(blob.chunks.len()).ok() != Some(index) || blob.kind.is_some_and(|existing| existing != kind) || next_count > kind.max_chunks() || next_bytes > kind.max_bytes() {
        content.remove(staging_id);
        return Err(RemodelStagingFault::Invalid);
    }
    let mut digest = blob.digest;
    let mut digest_len = blob.digest_len;
    if let Err(fault) = record_content_digest(&mut digest, &mut digest_len, &bytes) {
        content.remove(staging_id);
        return Err(fault);
    }
    blob.kind = Some(kind);
    blob.digest = digest;
    blob.digest_len = digest_len;
    blob.byte_count = next_bytes;
    blob.chunks.insert(index, Arc::from(bytes));
    Ok(())
}

#[cfg(test)]
pub fn staged_remodel_asset_chunk_count(staging_id: &str) -> u64 {
    remodel_asset_content().lock().expect("remodel asset content lock").get(staging_id).and_then(|blob| u64::try_from(blob.chunks.len()).ok()).unwrap_or(0)
}

fn staged_asset_commit_is_valid(content: &BTreeMap<String, RemodelAssetBlob>, staging_id: &str, content_id: &str, chunk_count: u64, expected_kind: Option<RemodelAssetContentKind>) -> bool {
    let Some(blob) = content.get(staging_id) else { return false };
    let Some(kind) = blob.kind else { return false };
    chunk_count != 0
        && expected_kind.is_none_or(|expected| expected == kind)
        && chunk_count <= kind.max_chunks()
        && blob.byte_count <= kind.max_bytes()
        && u64::try_from(blob.chunks.len()).ok() == Some(chunk_count)
        && blob.digest_len == u64::try_from(blob.byte_count).unwrap_or(u64::MAX)
        && content_digest_id("remodel-asset", blob.digest, blob.digest_len) == content_id
}

pub fn discard_staged_remodel_asset(staging_id: &str) {
    remodel_asset_content().lock().expect("remodel asset content lock").remove(staging_id);
}

pub fn remodel_asset_stage_key(staging_id: &str, kind: RemodelAssetContentKind, index: u64) -> String {
    format!("__remodel_asset_stage__:{}|{staging_id}:{index}", kind.wire())
}

pub fn remodel_asset_stage_parts(key: &str) -> Option<(RemodelAssetContentKind, &str, u64)> {
    let tail = key.strip_prefix("__remodel_asset_stage__:")?;
    let (staging_id, index) = tail.rsplit_once(':')?;
    let (kind, staging_id) = staging_id.split_once('|')?;
    Some((RemodelAssetContentKind::parse(kind)?, staging_id, index.parse().ok()?))
}

pub fn remodel_asset_content_handle(content_id: &str, staging_id: &str, chunk_count: u64) -> String {
    format!("remodel-content:{content_id}|{staging_id}|{chunk_count}")
}

pub fn remodel_asset_content_handle_parts(value: &str) -> Option<(&str, &str, u64)> {
    let tail = value.strip_prefix("remodel-content:")?;
    let (tail, chunk_count) = tail.rsplit_once('|')?;
    let (content_id, staging_id) = tail.split_once('|')?;
    Some((content_id, staging_id, chunk_count.parse().ok()?))
}
//#endregion 🔖️ReplayableAssetBlobs
//#endregion 🔖️AssetHandles

//#region 🔖️MeshHandle
fn mesh_child_handle(child_id: String, artifact_id: String) -> RemodelMeshChild {
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "mesh".into() };
    let target = store::os_io::ArtifactRef { artifact_id, dialect };
    store::ArtifactChild::new(child_id, target)
}

fn decode_remodel_durable_chunk(encoded: &str) -> Option<Vec<u8>> {
    let encoded_limit = REMODEL_DURABLE_CHUNK_RAW_BYTES.checked_add(2)?.checked_div(3)?.checked_mul(4)?;
    if encoded.len() > encoded_limit {
        return None;
    }
    let bytes = base64_codec::base64_standard_decode(encoded).ok()?;
    (bytes.len() <= REMODEL_DURABLE_CHUNK_RAW_BYTES).then_some(bytes)
}

#[cfg(test)]
fn mesh_digest_bytes(chunks: impl IntoIterator<Item = impl AsRef<[u8]>>) -> String {
    let mut digest = [0x6c62272e07bb0142u64, 0x62b821756295c58d, 0x9e3779b185ebca87, 0xc2b2ae3d27d4eb4f];
    let mut len = 0u64;
    for byte in chunks.into_iter().flat_map(|chunk| chunk.as_ref().to_vec()) {
        len = len.checked_add(1).expect("bounded test mesh digest");
        digest[0] = (digest[0] ^ u64::from(byte)).wrapping_mul(0x00000100000001b3);
        digest[1] = (digest[1] ^ digest[0].rotate_left(17) ^ len).wrapping_mul(0x9e3779b185ebca87);
        digest[2] = (digest[2] ^ digest[1].rotate_left(29) ^ u64::from(byte)).wrapping_mul(0xc2b2ae3d27d4eb4f);
        digest[3] = (digest[3] ^ digest[2].rotate_left(41) ^ len.rotate_left(7)).wrapping_mul(0x165667b19e3779f9);
    }
    format!("remodel-mesh-{:016x}{:016x}{:016x}{:016x}-{len:016x}", digest[0], digest[1], digest[2], digest[3])
}

struct RemodelMeshBlob {
    chunks: BTreeMap<u64, Arc<[u8]>>,
    digest: [u64; 4],
    digest_len: u64,
    byte_count: usize,
    last_field: Option<u8>,
}

impl Default for RemodelMeshBlob {
    fn default() -> Self {
        Self { chunks: BTreeMap::new(), digest: [0x6c62272e07bb0142, 0x62b821756295c58d, 0x9e3779b185ebca87, 0xc2b2ae3d27d4eb4f], digest_len: 0, byte_count: 0, last_field: None }
    }
}

impl RemodelMeshBlob {
    fn content_id(&self) -> String {
        format!("remodel-mesh-{:016x}{:016x}{:016x}{:016x}-{:016x}", self.digest[0], self.digest[1], self.digest[2], self.digest[3], self.digest_len)
    }
}

static REMODEL_PRIVATE_MESH_STAGING: OnceLock<Mutex<BTreeMap<String, RemodelMeshBlob>>> = OnceLock::new();

fn remodel_mesh_blobs() -> &'static Mutex<BTreeMap<String, RemodelMeshBlob>> {
    REMODEL_PRIVATE_MESH_STAGING.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn apply_mesh_chunk(mesh: &mut MeshData, last_field: Option<u8>, bytes: &[u8]) -> bool {
    let Some((&field, values)) = bytes.split_first() else { return false };
    if field > 11 || last_field.is_some_and(|previous| field < previous) {
        return false;
    }
    let component_count = values.len() / 4;
    match field {
        0 | 1 | 2 | 4 | 7 | 9 if values.len() % 4 == 0 => {
            let target = match field {
                0 => &mut mesh.positions,
                1 => &mut mesh.normals,
                2 => &mut mesh.colors,
                4 => &mut mesh.uvs,
                7 => &mut mesh.edge_positions,
                _ => &mut mesh.edge_uvs,
            };
            let limit = match field {
                0 | 1 => REMODEL_BOUNDED_MESH_VERTICES * 3,
                2 => REMODEL_BOUNDED_MESH_VERTICES * 4,
                4 => REMODEL_BOUNDED_MESH_VERTICES * 2,
                7 => REMODEL_BOUNDED_MESH_TRIANGLES * 6,
                _ => REMODEL_BOUNDED_MESH_TRIANGLES * 4,
            };
            if target.len().checked_add(component_count).is_none_or(|count| count > limit) {
                return false;
            }
            target.extend(values.chunks_exact(4).map(|value| f32::from_le_bytes(value.try_into().expect("four-byte mesh f32"))));
        }
        3 | 5 | 6 | 8 if values.len() % 4 == 0 => {
            let target = match field {
                3 => &mut mesh.indices,
                5 => &mut mesh.face_ids,
                6 => &mut mesh.vertex_ids,
                _ => &mut mesh.edge_ids,
            };
            let limit = match field {
                3 => REMODEL_BOUNDED_MESH_TRIANGLES * 3,
                5 => REMODEL_BOUNDED_MESH_TRIANGLES,
                6 => REMODEL_BOUNDED_MESH_VERTICES,
                _ => REMODEL_BOUNDED_MESH_TRIANGLES * 3,
            };
            if target.len().checked_add(component_count).is_none_or(|count| count > limit) {
                return false;
            }
            target.extend(values.chunks_exact(4).map(|value| u32::from_le_bytes(value.try_into().expect("four-byte mesh u32"))));
        }
        10 => {
            if mesh.edge_is_seam.len().checked_add(values.len()).is_none_or(|count| count > REMODEL_BOUNDED_MESH_TRIANGLES * 3) {
                return false;
            }
            mesh.edge_is_seam.extend_from_slice(values);
        }
        11 => {
            let Ok(value) = std::str::from_utf8(values) else { return false };
            if mesh.paint_texture_base64.as_ref().map_or(0, String::len).checked_add(value.len()).is_none_or(|count| count > 24_576) {
                return false;
            }
            mesh.paint_texture_base64.get_or_insert_with(String::new).push_str(value);
        }
        _ => return false,
    }
    true
}

fn mesh_from_blob(blob: &RemodelMeshBlob) -> Option<MeshData> {
    let mut mesh = MeshData::default();
    let mut last_field = None;
    for index in 0..u64::try_from(blob.chunks.len()).ok()? {
        let chunk = blob.chunks.get(&index)?;
        if !apply_mesh_chunk(&mut mesh, last_field, chunk) {
            return None;
        }
        last_field = chunk.first().copied();
    }
    Some(mesh)
}

/// 🧱️ Replays one fixed-size full-fidelity mesh chunk into process-wide owned staging.
pub fn stage_remodel_mesh_chunk(staging_id: &str, index: u64, encoded: &str) -> Result<(), RemodelStagingFault> {
    let Some(bytes) = decode_remodel_durable_chunk(encoded) else { return Err(RemodelStagingFault::Invalid) };
    let next_count = index.checked_add(1).ok_or(RemodelStagingFault::Busy)?;
    let mut blobs = remodel_mesh_blobs().lock().expect("remodel mesh blob lock");
    if !blobs.contains_key(staging_id) && blobs.len() >= REMODEL_MAX_STAGED_BLOBS {
        return Err(RemodelStagingFault::Busy);
    }
    let blob = blobs.entry(staging_id.into()).or_default();
    if let Some(existing) = blob.chunks.get(&index) {
        return (existing.as_ref() == bytes).then_some(()).ok_or(RemodelStagingFault::Invalid);
    }
    let next_bytes = blob.byte_count.checked_add(bytes.len()).ok_or(RemodelStagingFault::Busy)?;
    let mut digest = blob.digest;
    let mut digest_len = blob.digest_len;
    let mut candidate = match mesh_from_blob(blob) {
        Some(mesh) => mesh,
        None => {
            blobs.remove(staging_id);
            return Err(RemodelStagingFault::Invalid);
        }
    };
    if record_content_digest(&mut digest, &mut digest_len, &bytes).is_err()
        || u64::try_from(blob.chunks.len()).ok() != Some(index)
        || next_count > REMODEL_MESH_CONTENT_CHUNKS
        || next_bytes > REMODEL_MESH_CONTENT_BYTES
        || !apply_mesh_chunk(&mut candidate, blob.last_field, &bytes)
    {
        blobs.remove(staging_id);
        return Err(RemodelStagingFault::Invalid);
    }
    blob.last_field = bytes.first().copied();
    blob.byte_count = next_bytes;
    blob.digest = digest;
    blob.digest_len = digest_len;
    blob.chunks.insert(index, Arc::from(bytes));
    Ok(())
}

pub fn discard_staged_remodel_mesh(staging_id: &str) {
    remodel_mesh_blobs().lock().expect("remodel mesh blob lock").remove(staging_id);
}

pub fn staged_remodel_mesh_chunk_count(staging_id: &str) -> u64 {
    remodel_mesh_blobs().lock().expect("remodel mesh blob lock").get(staging_id).and_then(|blob| u64::try_from(blob.chunks.len()).ok()).unwrap_or(0)
}

fn staged_mesh_commit_is_valid(blobs: &BTreeMap<String, RemodelMeshBlob>, staging_id: &str, content_id: &str, chunk_count: u64) -> bool {
    let Some(blob) = blobs.get(staging_id) else { return false };
    chunk_count != 0
        && chunk_count <= REMODEL_MESH_CONTENT_CHUNKS
        && blob.byte_count <= REMODEL_MESH_CONTENT_BYTES
        && u64::try_from(blob.chunks.len()).ok() == Some(chunk_count)
        && blob.digest_len == u64::try_from(blob.byte_count).unwrap_or(u64::MAX)
        && mesh_from_blob(blob).is_some_and(|mesh| mesh_is_within_resolution_envelope(&mesh))
        && blob.content_id() == content_id
}

/// 🏁️ Validates every terminal artifact before publishing any staged content, then applies all
/// promotions while both bounded stores remain exclusively held.
pub fn commit_staged_remodel_reconstruction(assets: &[(&str, &str, u64, RemodelAssetContentKind)], mesh: Option<(&str, &str, u64)>) -> bool {
    let mut asset_content = remodel_asset_content().lock().expect("remodel asset content lock");
    let mut mesh_content = remodel_mesh_blobs().lock().expect("remodel mesh blob lock");
    if !assets.iter().all(|(staging_id, content_id, chunk_count, kind)| staged_asset_commit_is_valid(&asset_content, staging_id, content_id, *chunk_count, Some(*kind)))
        || mesh.is_some_and(|(staging_id, content_id, chunk_count)| !staged_mesh_commit_is_valid(&mesh_content, staging_id, content_id, chunk_count))
    {
        return false;
    }
    for (staging_id, _, _, _) in assets {
        asset_content.remove(*staging_id);
    }
    if let Some((staging_id, _, _)) = mesh {
        mesh_content.remove(staging_id);
    }
    true
}

pub fn remodel_mesh_stage_asset_key(staging_id: &str, index: u64) -> String {
    format!("__remodel_mesh_stage__:{staging_id}:{index}")
}

pub fn remodel_mesh_stage_asset_parts(key: &str) -> Option<(&str, u64)> {
    let tail = key.strip_prefix("__remodel_mesh_stage__:")?;
    let (staging_id, index) = tail.rsplit_once(':')?;
    Some((staging_id, index.parse().ok()?))
}

pub fn staged_remodel_mesh_handle(content_id: &str, staging_id: &str) -> RemodelMeshChild {
    mesh_child_handle(content_id.into(), format!("mesh-stage:{staging_id}"))
}

pub fn replayable_remodel_mesh_handle(content_id: &str, staging_id: &str, chunk_count: u64) -> RemodelMeshChild {
    mesh_child_handle(content_id.into(), format!("remodel-mesh-log:{staging_id}:{chunk_count}"))
}

pub fn replayable_remodel_mesh_handle_parts(handle: &RemodelMeshChild) -> Option<(&str, &str, u64)> {
    let tail = handle.target.artifact_id.strip_prefix("remodel-mesh-log:")?;
    let (staging_id, chunk_count) = tail.rsplit_once(':')?;
    Some((&handle.child_id, staging_id, chunk_count.parse().ok()?))
}

/// 🧊️ Stable empty-mesh handle with no process-owned payload.
pub fn empty_remodel_mesh_handle() -> RemodelMeshChild {
    mesh_child_handle(REMODEL_EMPTY_MESH_CHILD_ID.into(), "remodel-mesh-constant:empty".into())
}

/// 📦️ Stable bounded placeholder handle with no process-owned payload.
pub fn placeholder_remodel_mesh_handle() -> RemodelMeshChild {
    mesh_child_handle(REMODEL_BOX_MESH_CHILD_ID.into(), "remodel-mesh-constant:box".into())
}

fn mesh_is_within_resolution_envelope(mesh: &MeshData) -> bool {
    let vertices = mesh.positions.len().checked_div(3);
    let triangles = mesh.indices.len().checked_div(3);
    let Some(vertices) = vertices.filter(|_| mesh.positions.len() % 3 == 0) else { return false };
    let Some(triangles) = triangles.filter(|_| mesh.indices.len() % 3 == 0) else { return false };
    vertices <= REMODEL_BOUNDED_MESH_VERTICES
        && triangles <= REMODEL_BOUNDED_MESH_TRIANGLES
        && mesh.indices.iter().all(|index| usize::try_from(*index).ok().is_some_and(|index| index < vertices))
        && (mesh.normals.is_empty() || vertices.checked_mul(3) == Some(mesh.normals.len()))
        && (mesh.colors.is_empty() || vertices.checked_mul(3) == Some(mesh.colors.len()) || vertices.checked_mul(4) == Some(mesh.colors.len()))
        && (mesh.uvs.is_empty() || vertices.checked_mul(2) == Some(mesh.uvs.len()))
        && (mesh.face_ids.is_empty() || mesh.face_ids.len() == triangles)
        && (mesh.vertex_ids.is_empty() || mesh.vertex_ids.len() == vertices)
        && mesh.normals.len() <= REMODEL_BOUNDED_MESH_VERTICES * 3
        && mesh.colors.len() <= REMODEL_BOUNDED_MESH_VERTICES * 4
        && mesh.uvs.len() <= REMODEL_BOUNDED_MESH_VERTICES * 2
        && mesh.face_ids.len() <= REMODEL_BOUNDED_MESH_TRIANGLES
        && mesh.vertex_ids.len() <= REMODEL_BOUNDED_MESH_VERTICES
        && mesh.edge_positions.len() <= REMODEL_BOUNDED_MESH_TRIANGLES * 6
        && mesh.edge_ids.len() <= REMODEL_BOUNDED_MESH_TRIANGLES * 3
        && mesh.edge_uvs.len() <= REMODEL_BOUNDED_MESH_TRIANGLES * 4
        && mesh.edge_is_seam.len() <= REMODEL_BOUNDED_MESH_TRIANGLES * 3
        && mesh.paint_texture_base64.as_ref().is_none_or(|value| value.len() <= 24_576)
}

/// 🧱️ Returns the durable raw chunk count without reconstructing or cloning mesh fields.
pub fn bounded_remodel_mesh_chunk_count(store: &RemodelDurableArtifactStore, handle: &RemodelMeshChild) -> Option<u64> {
    let (content_id, _, chunk_count) = replayable_remodel_mesh_handle_parts(handle)?;
    let artifact = store.get(content_id).filter(|artifact| artifact.kind == "mesh")?;
    (u64::try_from(artifact.chunks.len()).ok() == Some(chunk_count)).then_some(chunk_count)
}

/// 🧱️ Resolves one admitted durable mesh chunk without cloning the reconstructed mesh.
pub fn bounded_remodel_mesh_chunk(store: &RemodelDurableArtifactStore, handle: &RemodelMeshChild, index: u64) -> Option<Arc<[u8]>> {
    let (content_id, _, chunk_count) = replayable_remodel_mesh_handle_parts(handle)?;
    if index >= chunk_count {
        return None;
    }
    let artifact = store.get(content_id).filter(|artifact| artifact.kind == "mesh")?;
    if u64::try_from(artifact.chunks.len()).ok() != Some(chunk_count) {
        return None;
    }
    Some(Arc::from(decode_remodel_durable_chunk(artifact.chunks.get(usize::try_from(index).ok()?)?)?))
}

/// 🧱️ Resolves only fixed constants or reconstruction output admitted by the 512/512 envelope.
pub fn resolve_bounded_remodel_mesh(store: &RemodelDurableArtifactStore, handle: &RemodelMeshChild) -> Option<MeshData> {
    match handle.child_id.as_str() {
        REMODEL_EMPTY_MESH_CHILD_ID => Some(MeshData::default()),
        REMODEL_BOX_MESH_CHILD_ID => Some(semio_framework::mesh_from_kind("box")),
        _ => {
            let (content_id, _, chunk_count) = replayable_remodel_mesh_handle_parts(handle)?;
            let artifact = store.get(content_id).filter(|artifact| artifact.kind == "mesh")?;
            if u64::try_from(artifact.chunks.len()).ok() != Some(chunk_count) {
                return None;
            }
            let mut mesh = MeshData::default();
            let mut last_field = None;
            for encoded in &artifact.chunks {
                let chunk = decode_remodel_durable_chunk(encoded)?;
                if !apply_mesh_chunk(&mut mesh, last_field, &chunk) {
                    return None;
                }
                last_field = chunk.first().copied();
            }
            mesh_is_within_resolution_envelope(&mesh).then_some(mesh)
        }
    }
}

#[cfg(test)]
static REMODEL_TEST_MESHES: OnceLock<Mutex<BTreeMap<String, MeshData>>> = OnceLock::new();

#[cfg(test)]
fn remodel_test_meshes() -> &'static Mutex<BTreeMap<String, MeshData>> {
    REMODEL_TEST_MESHES.get_or_init(|| Mutex::new(BTreeMap::new()))
}

#[cfg(test)]
pub fn mint_and_stash_mesh(mesh: MeshData) -> RemodelMeshChild {
    let bytes = serde_json::to_vec(&mesh).unwrap_or_default();
    let child_id = mesh_digest_bytes([bytes.as_slice()]);
    remodel_test_meshes().lock().expect("remodel test mesh lock").insert(child_id.clone(), mesh);
    mesh_child_handle(child_id, "remodel-mesh".into())
}

#[cfg(test)]
pub fn remodel_mesh_workspace(handle: &RemodelMeshChild) -> Option<MeshData> {
    if handle.child_id == REMODEL_EMPTY_MESH_CHILD_ID {
        return Some(MeshData::default());
    }
    if handle.child_id == REMODEL_BOX_MESH_CHILD_ID {
        return Some(semio_framework::mesh_from_kind("box"));
    }
    remodel_test_meshes().lock().expect("remodel test mesh lock").get(&handle.child_id).cloned()
}

pub fn durable_staged_remodel_mesh(staging_id: &str) -> Option<RemodelDurableArtifact> {
    let blobs = remodel_mesh_blobs().lock().expect("remodel mesh blob lock");
    let blob = blobs.get(staging_id)?;
    Some(RemodelDurableArtifact {
        kind: "mesh".into(),
        mime: None,
        width: 0,
        height: 0,
        chunks: (0..u64::try_from(blob.chunks.len()).ok()?).map(|index| base64_codec::base64_standard_encode(blob.chunks.get(&index).expect("contiguous staged mesh"))).collect(),
    })
}

#[cfg(test)]
pub fn forget_remodel_mesh_content_for_test(content_id: &str, staging_id: &str) {
    remodel_mesh_blobs().lock().expect("remodel mesh blob lock").remove(staging_id);
    remodel_test_meshes().lock().expect("remodel test mesh lock").remove(content_id);
}

#[cfg(test)]
pub fn forget_all_remodel_content_for_test() {
    remodel_asset_content().lock().expect("remodel asset content lock").clear();
    remodel_mesh_blobs().lock().expect("remodel mesh blob lock").clear();
    remodel_test_meshes().lock().expect("remodel test mesh lock").clear();
}
//#endregion 🔖️MeshHandle
//#endregion 🧩️Composition

//#region 🔖️Packed
/// 📦️ A flat `f32` buffer serialized as a base64 string of its little-endian bytes rather than a JSON
/// array — point clouds and height grids commonly carry 10^5-10^6 elements, where per-element JSON
/// text is both far larger on the wire and far slower to parse than one base64 blob.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[value(transparent)]
#[serde(transparent)]
pub struct PackedF32(pub String);

impl PackedF32 {
    /// 📦️ Encodes a `f32` slice as a base64 string of its little-endian bytes.
    pub fn from_f32_slice(values: &[f32]) -> Self {
        let bytes: Vec<u8> = values.iter().flat_map(|value| value.to_le_bytes()).collect();
        Self(base64_codec::base64_standard_encode(bytes))
    }

    /// 📦️ Decodes back into a `f32` vec; a malformed payload (bad base64, length not a multiple of 4)
    /// decodes as empty rather than panicking, since packed buffers only ever round-trip in-process.
    pub fn to_f32_vec(&self) -> Vec<f32> {
        let Ok(bytes) = base64_codec::base64_standard_decode(self.0.as_bytes()) else {
            return Vec::new();
        };
        let (chunks, remainder) = bytes.as_chunks::<4>();
        if !remainder.is_empty() {
            return Vec::new();
        }
        chunks.iter().map(|chunk| f32::from_le_bytes(*chunk)).collect()
    }

    pub fn to_f32_vec_from(&self, store: &RemodelDurableArtifactStore) -> Vec<f32> {
        let Some((content_id, _, chunk_count)) = remodel_asset_content_handle_parts(&self.0) else { return self.to_f32_vec() };
        let Some(artifact) = store.get(content_id).filter(|artifact| artifact.kind == "sparse" && u64::try_from(artifact.chunks.len()).ok() == Some(chunk_count)) else { return Vec::new() };
        let mut values = Vec::new();
        for encoded in &artifact.chunks {
            let Some(bytes) = decode_remodel_durable_chunk(encoded) else { return Vec::new() };
            let (chunks, remainder) = bytes.as_chunks::<4>();
            if !remainder.is_empty() {
                return Vec::new();
            }
            values.extend(chunks.iter().map(|chunk| f32::from_le_bytes(*chunk)));
        }
        values
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// 📦️ A flat `u8` buffer (vertex colors, classification codes) that serializes as a base64 string
/// directly — same rationale as {@link PackedF32}.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[value(transparent)]
#[serde(transparent)]
pub struct PackedU8(pub String);

impl PackedU8 {
    /// 📦️ Encodes a `u8` slice as a base64 string.
    pub fn from_u8_slice(values: &[u8]) -> Self {
        Self(base64_codec::base64_standard_encode(values))
    }

    /// 📦️ Decodes back into a `u8` vec; a malformed payload decodes as empty.
    pub fn to_u8_vec(&self) -> Vec<u8> {
        base64_codec::base64_standard_decode(self.0.as_bytes()).unwrap_or_default()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// 🌉️ `PackedF32`'s inner string is ALREADY the wire format (base64 text), so it binds as a plain
/// `Shape::Text` rather than `#[dsl(base64)]` (which is for raw `Vec<u8>` fields only) — no double
/// encoding, no `-` sentinel: an empty buffer is just an empty quoted string.
impl dsl::DslField for PackedF32 {
    async fn shape() -> dsl::Shape {
        dsl::Shape::Text
    }
    async fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Text(self.0.clone())
    }
    async fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Text(s) => Ok(Self(s.clone())),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
}

/// 🌉️ Same reasoning as `PackedF32`'s impl above.
impl dsl::DslField for PackedU8 {
    async fn shape() -> dsl::Shape {
        dsl::Shape::Text
    }
    async fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Text(self.0.clone())
    }
    async fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Text(s) => Ok(Self(s.clone())),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
}
//#endregion 🔖️Packed

//#region 🔖️Domain
/// 🖼️ One embedded pixel asset (video frame, ortho tile, texture) referenced by id from
/// `RemodelSnapshot::assets`, `MediaStream.frames`, `RemodelMesh.texture_asset_id`, or
/// `GeoProducts.{dsm,dtm,ortho}_asset_id`. Sampled video frames use `image/jpeg` (~10x smaller than
/// PNG for photographic content); PNG stays reserved for exports/textures/rasters that need
/// lossless round trips.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct ImageAsset {
    pub mime: String,
    pub data: String,
    pub width: u32,
    pub height: u32,
}

/// 🗂️ Which shape a `MediaStream`'s frames were captured as. Video input is always eagerly extracted
/// into individually-addressable `FrameRef`s before persistence (video bytes themselves are never
/// stored) — `MediaKind::Video` only records that provenance, `MediaStream.source` carries the detail.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, dsl::DslScalar)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum MediaKind {
    #[default]
    ImageSequence,
    Video,
}

/// 🎞️ Codec a `VideoSource` was demuxed from — a plain mirror of `remodel_video::VideoCodec` without
/// its `FourCc` payload (an unrecognized four-character code collapses to `Unknown`, which is enough
/// provenance for a QC/diagnostic label).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, dsl::DslScalar)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum VideoCodec {
    Avc,
    Hevc,
    Vp9,
    Av1,
    Mjpeg,
    #[default]
    Unknown,
}

/// 🎥️ Provenance of a `MediaStream` that originated from an actual video file (as opposed to a raw
/// image-sequence import) — a lightweight mirror of `remodel_video::{Mp4Info, AviInfo}`, populated
/// once at import time from `remodel_video::probe`. "Video input = image sequence with timestamps":
/// by the time a stream reaches this document its frames are already individually-addressable
/// `ImageAsset`s with true media timestamps; this struct only records where they came from.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct VideoSource {
    pub name: String,
    pub container: String,
    pub codec: VideoCodec,
    pub duration_ms: f64,
    pub frame_count: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct FrameRef {
    pub index: u32,
    pub timestamp_ms: f64,
    pub asset_id: String,
}

/// 🎞️ One imported media source (an image sequence or a video), decoded into `FrameRef`s pointing at
/// `RemodelSnapshot::assets`. Multiple cameras/angles are multiple streams, joined by `camera_id`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct MediaStream {
    pub id: String,
    pub name: String,
    pub kind: MediaKind,
    pub camera_id: Option<String>,
    pub sync_offset_ms: f64,
    pub fps_hint: f64,
    #[dsl(table)]
    pub frames: Vec<FrameRef>,
    #[dsl(block)]
    pub source: Option<VideoSource>,
}

/// 🎯️ Per-camera intrinsics/distortion, a plain-JSON mirror of `remodel_camera::{Intrinsics,
/// Distortion}` rather than a direct reuse of those types: `Distortion` is a Rust enum tuned for the
/// solver's math (`BrownConrady{k1,k2,k3,p1,p2}` / `FisheyeEquidistant{k1,k2,k3,k4}`), which doesn't
/// serialize into a stable arg-form-editable shape — the document instead always carries a flat
/// 5-slot `distortion` array plus a `model` label the plugin uses to decide which slots are live,
/// matching the "pinhole|brownConrady|fisheye" UI select.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct CameraCalibration {
    pub id: String,
    pub label: String,
    pub model: String,
    pub fx: f64,
    pub fy: f64,
    pub cx: f64,
    pub cy: f64,
    pub skew: f64,
    /// 🔢️ `[k1, k2, k3, p1, p2]`.
    pub distortion: [f32; 5],
    pub rms_reprojection_px: Option<f32>,
    pub locked: bool,
}

/// 🎯️ One rig member's pose relative to the rig origin — a plain mirror of `remodel_camera`'s
/// `RigExtrinsic{camera_id, pose_in_rig: Se3}`, flattened to a quaternion + translation since `Se3`
/// (a `crate::lie` manifold type) is a plugin-runtime concern, not a document one.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct RigExtrinsic {
    pub camera_id: String,
    pub rotation_wxyz: [f32; 4],
    #[dsl(coord)]
    pub translation_m: [f32; 3],
}

impl Default for RigExtrinsic {
    fn default() -> Self {
        Self { camera_id: String::new(), rotation_wxyz: [1.0, 0.0, 0.0, 0.0], translation_m: [0.0; 3] }
    }
}

/// 🎯️ Per-camera intrinsics/distortion plus rig extrinsics, refined by `remodel_camera`/`remodel_sfm`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct CalibrationState {
    #[dsl(table)]
    pub cameras: Vec<CameraCalibration>,
    #[dsl(table)]
    pub rig: Vec<RigExtrinsic>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct GcpObservation {
    pub stream_id: String,
    pub frame_index: u32,
    pub pixel: [f32; 2],
}

/// 📍️ A surveyed ground-control point used by `remodel_geo` to georeference the reconstruction.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct GroundControlPoint {
    pub id: String,
    pub name: String,
    #[dsl(coord)]
    pub world_position: [f64; 3],
    #[dsl(table)]
    pub observations: Vec<GcpObservation>,
}

/// ⏭️ Frame sampling/decode limits `remodel_engine` applies before feature extraction. `min_sharpness`
/// is the blur gate: a candidate frame is dropped when its sharpness falls below this fraction of the
/// rolling median sharpness of the last ~15 accepted frames.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct IngestParams {
    pub frame_sample_stride: u32,
    pub max_frames: u32,
    pub downscale_long_edge_px: u32,
    pub min_sharpness: f32,
}

impl Default for IngestParams {
    fn default() -> Self {
        Self { frame_sample_stride: 5, max_frames: 200, downscale_long_edge_px: 1600, min_sharpness: 0.3 }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, dsl::DslScalar)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum FeatureDetector {
    #[default]
    Orb,
    Akaze,
    Harris,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct FeatureParams {
    pub detector: FeatureDetector,
    pub target_count: u32,
    pub octaves: u32,
    pub edge_threshold: f32,
}

impl Default for FeatureParams {
    fn default() -> Self {
        Self { detector: FeatureDetector::default(), target_count: 4000, octaves: 4, edge_threshold: 10.0 }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, dsl::DslScalar)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum MatcherKind {
    #[default]
    BruteForce,
    KdTree,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct MatchParams {
    pub matcher: MatcherKind,
    pub ratio_test: f32,
    pub cross_check: bool,
    pub sequential_window: u32,
    pub max_pairs_per_frame: u32,
    pub loop_closure: bool,
}

impl Default for MatchParams {
    fn default() -> Self {
        Self { matcher: MatcherKind::default(), ratio_test: 0.8, cross_check: true, sequential_window: 8, max_pairs_per_frame: 16, loop_closure: true }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, dsl::DslScalar)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum RobustLossKind {
    L2,
    #[default]
    Huber,
    Cauchy,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct SfmParams {
    pub ransac_iterations: u32,
    pub ransac_threshold_px: f32,
    pub min_track_length: u32,
    pub ba_max_iterations: u32,
    pub robust_loss: RobustLossKind,
    pub huber_delta_px: f32,
}

impl Default for SfmParams {
    fn default() -> Self {
        Self { ransac_iterations: 1000, ransac_threshold_px: 2.0, min_track_length: 3, ba_max_iterations: 50, robust_loss: RobustLossKind::default(), huber_delta_px: 1.5 }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, dsl::DslScalar)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum DenseResolution {
    Low,
    #[default]
    Medium,
    High,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct DenseParams {
    pub resolution: DenseResolution,
    pub window_radius_px: u32,
    pub min_view_consistency: u32,
    pub confidence_threshold: f32,
    pub max_points: u32,
}

impl Default for DenseParams {
    fn default() -> Self {
        Self { resolution: DenseResolution::default(), window_radius_px: 3, min_view_consistency: 3, confidence_threshold: 0.5, max_points: 500_000 }
    }
}

/// 🧊️ UI-facing meshing knobs `remodel_engine` translates into `remodel_mesh`'s own internal
/// `MeshParams`/`TsdfVolume` construction args (this document does not depend on `remodel_mesh`, so
/// the two `MeshParams` types are intentionally separate). `guarantee_watertight`,
/// `hole_fill_max_boundary_verts`, and `self_intersection_check` are the watertight-guarantee knobs:
/// when `guarantee_watertight` is set and repair/hole-fill can't recover a closed 2-manifold, the
/// `🔖️Close` fallback triggers and re-validates until the result passes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct MeshParams {
    #[dsl(unit = "mm")]
    pub tsdf_voxel_size_mm: f32,
    #[dsl(unit = "mm")]
    pub tsdf_truncation_mm: f32,
    pub decimate_target_triangles: u32,
    pub smoothing_iterations: u32,
    pub texture_enabled: bool,
    pub texture_size: u32,
    pub guarantee_watertight: bool,
    pub hole_fill_max_boundary_verts: u32,
    pub self_intersection_check: bool,
}

impl Default for MeshParams {
    fn default() -> Self {
        Self {
            tsdf_voxel_size_mm: 5.0,
            tsdf_truncation_mm: 20.0,
            decimate_target_triangles: 200_000,
            smoothing_iterations: 2,
            texture_enabled: true,
            texture_size: 2048,
            guarantee_watertight: true,
            hole_fill_max_boundary_verts: 512,
            self_intersection_check: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct MotionParams {
    pub enabled: bool,
    pub max_tracks: u32,
    pub track_window_px: u32,
    pub min_track_quality: f32,
    pub min_track_length_frames: u32,
}

impl Default for MotionParams {
    fn default() -> Self {
        Self { enabled: false, max_tracks: 64, track_window_px: 21, min_track_quality: 0.3, min_track_length_frames: 5 }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct GeoParams {
    pub enabled: bool,
    pub origin_lon: Option<f64>,
    pub origin_lat: Option<f64>,
    pub origin_alt: Option<f64>,
    #[dsl(unit = "m")]
    pub gsd_m: f32,
    #[dsl(unit = "m")]
    pub dsm_cell_m: f32,
    #[dsl(unit = "m")]
    pub dtm_filter_radius_m: f32,
    pub ortho_max_px: u32,
}

impl Default for GeoParams {
    fn default() -> Self {
        Self { enabled: false, origin_lon: None, origin_lat: None, origin_alt: None, gsd_m: 0.05, dsm_cell_m: 0.1, dtm_filter_radius_m: 2.0, ortho_max_px: 4096 }
    }
}

/// ⚙️ Full reconstruction parameter set, one sub-struct per pipeline stage — `remodel_engine` reads
/// these directly to configure `remodel_image`/`remodel_video`/`remodel_camera`/`remodel_feature`/
/// `remodel_sfm`/`remodel_dense`/`remodel_mesh`/`remodel_motion`/`remodel_geo` without this crate
/// depending on any of them.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct ReconstructionParams {
    #[dsl(block)]
    pub ingest: IngestParams,
    #[dsl(block)]
    pub feature: FeatureParams,
    #[dsl(block)]
    pub matching: MatchParams,
    #[dsl(block)]
    pub sfm: SfmParams,
    #[dsl(block)]
    pub dense: DenseParams,
    #[dsl(block)]
    pub mesh: MeshParams,
    #[dsl(block)]
    pub motion: MotionParams,
    #[dsl(block)]
    pub geo: GeoParams,
}

/// 🚦️ Mirrors `remodel_engine`'s pipeline lifecycle so the document can render progress without
/// polling internals directly.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, dsl::DslScalar)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum ReconstructionStage {
    #[default]
    Idle,
    Ingesting,
    Calibrating,
    ExtractingFeatures,
    MatchingFeatures,
    EstimatingPoses,
    BundleAdjusting,
    Georeferencing,
    DenseStereo,
    FusingVolume,
    ExtractingSurface,
    CleaningMesh,
    Texturing,
    TrackingMotion,
    DerivingGeoProducts,
    ReportingQc,
    Done,
    Failed,
}

/// 📷️ A single recovered camera pose — streamed early into `ReconstructionJob.camera_poses_preview`
/// for live preview during sparse reconstruction, and reused verbatim as `CameraTrajectory.poses` once
/// the run finishes (no separate heavier pose type: both are the same lightweight snapshot).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct CameraPosePreview {
    pub camera_id: String,
    pub rotation_wxyz: [f32; 4],
    #[dsl(coord)]
    pub translation: [f32; 3],
}

impl Default for CameraPosePreview {
    fn default() -> Self {
        Self { camera_id: String::new(), rotation_wxyz: [1.0, 0.0, 0.0, 0.0], translation: [0.0; 3] }
    }
}

/// 🚧️ Live reconstruction run state — deliberately holds no algorithm scratch (descriptors, match
/// graphs, depth maps, TSDF volumes; those stay in the plugin's `PipelineScratch`), only what the UI
/// needs to render progress and what undo/redo needs to restore. `native_port` (a phantom pointer at
/// a `remodel-native` service that was never implemented) has been removed entirely — there is no
/// out-of-process reconstruction backend, only in-process WASM-safe classical algorithms.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct ReconstructionJob {
    pub id: String,
    pub stage: ReconstructionStage,
    pub progress_0_1: f32,
    pub cancel_requested: bool,
    pub stage_cursor: u32,
    pub started_at_ms: Option<f64>,
    pub error: Option<String>,
    #[dsl(table)]
    pub camera_poses_preview: Vec<CameraPosePreview>,
    pub sparse_point_cloud_preview: PackedF32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, dsl::DslScalar)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum MeshSource {
    #[default]
    Placeholder,
    Reconstructed,
    Imported,
}

/// ✅️ A plain-JSON mirror of `remodel_mesh::WatertightReport`'s summary fields (all scalars — the
/// report itself carries no array data, so this is a snapshot only in the sense of avoiding a hard
/// dependency on `remodel_mesh`, not in the sense of trimming size).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct WatertightReportSnapshot {
    pub vertex_count: u32,
    pub triangle_count: u32,
    pub boundary_edge_count: u32,
    pub boundary_loop_count: u32,
    pub non_manifold_edge_count: u32,
    pub non_manifold_vertex_count: u32,
    pub connected_components: u32,
    pub consistently_oriented: bool,
    pub euler_characteristic: i64,
    pub genus: Option<i64>,
    pub signed_volume: f64,
    pub self_intersection_pairs: Option<u32>,
    pub closed_fallback_used: bool,
    pub is_closed: bool,
    pub is_two_manifold: bool,
    pub is_watertight: bool,
}

/// 🧵️ The reconstructed (or placeholder/imported) mesh. Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `mesh` is now a composed `s.stdio.semio/v1/mesh`
/// CHILD handle (`RemodelMeshChild`, `🧩️Composition` region above), never embedded `MeshData` —
/// reconstructed geometry resolves through a bounded durable chunk handle; the empty and box
/// placeholders are typed constants that require no process cache.
/// `source`/`texture_asset_id`/`watertight` are genuinely NOT part of the composed mesh's own content
/// (they describe THIS document's relationship to the mesh — provenance, a separate asset reference, a
/// derived QC summary — not geometry), so they stay sibling fields here rather than folding into the
/// child, matching `puzzle`'s own `*Extra`-sibling precedent for content a composed subset's shape
/// can't represent. Always present (never `Option`) so the 3D view always has something to render —
/// `default_remodel_scene()` seeds it with a placeholder box.
///
/// `ArtifactChild<S>: dsl::DslField` is now real (`🏪️store/🦀️.rs:523`) so this struct keeps
/// its plain `#[derive(dsl::DslRecord)]` instead of hand-rolling — the former `🔖️MeshBridge` region
/// (a `MeshDataTwin` buffer-by-buffer bridge, needed only because `MeshData` is foreign and had no
/// `DslField` impl reachable from this crate) is gone entirely: every field left on this struct now has
/// a real `DslField` impl on its own.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelMesh {
    #[dsl(block)]
    pub mesh: RemodelMeshChild,
    pub source: MeshSource,
    pub texture_asset_id: Option<String>,
    #[dsl(block)]
    pub watertight: Option<WatertightReportSnapshot>,
}

impl Default for RemodelMesh {
    fn default() -> Self {
        Self { mesh: empty_remodel_mesh_handle(), source: MeshSource::default(), texture_asset_id: None, watertight: None }
    }
}

//#region 🔖️MeshBridge
/// 🌉️ `Box<T>` is a `#[fundamental]` std type, so implementing the foreign `dsl::DslField` trait for
/// `Box<RemodelMesh>` (a local type parameter) here is coherence-legal — needed because
/// `RemodelMutation::ReplaceMeshResult` carries `mesh: Box<RemodelMesh>` (boxed only to shrink the
/// enum's overall size; `RemodelMesh` itself is a plain record, not a `DslEnum`, so the derive's
/// `#[dsl(statements)] Box<T>` "exactly-one-tagged-value" idiom doesn't apply — this is the ordinary
/// boxed-scalar case instead). Delegates to `RemodelMesh`'s own (now derive-generated) `DslField` impl.
impl dsl::DslField for Box<RemodelMesh> {
    async fn shape() -> dsl::Shape {
        <RemodelMesh as dsl::DslField>::shape()
    }
    async fn to_value(&self) -> dsl::FieldValue {
        <RemodelMesh as dsl::DslField>::to_value(self.as_ref())
    }
    async fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        <RemodelMesh as dsl::DslField>::from_value(value).map(Box::new)
    }
}
//#endregion 🔖️MeshBridge

/// ☁️ Sparse point cloud from bundle adjustment (`points` = flat xyz triples).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct SparseCloud {
    pub points: PackedF32,
    pub colors: Option<PackedU8>,
}

/// ☁️ Dense point cloud with optional per-point LAS-style classification codes (0 unclassified, 2
/// ground, 6 building, …) — `remodel_dense::PointClass` is a bespoke enum without numeric LAS
/// discriminants, so `remodel_engine` maps it to LAS codes when it distills this snapshot.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct DenseCloud {
    pub positions: PackedF32,
    pub colors: Option<PackedU8>,
    pub confidence: Option<PackedF32>,
    pub classification: Option<PackedU8>,
}

/// 🎥️ Recovered camera trajectory across all registered frames.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct CameraTrajectory {
    #[dsl(table)]
    pub poses: Vec<CameraPosePreview>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, dsl::DslScalar)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum TrackClass {
    #[default]
    Static,
    Moving,
}

/// 🏃️ A distilled summary of one `remodel_motion` track — full per-frame keyframe paths
/// (`Track2d`/`Trajectory3d` in the motion crate) are plugin-runtime scratch, not durable document
/// state; only enough is kept here to list/label tracks and drive the report table.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct MotionTrackSummary {
    pub id: String,
    pub length: u32,
    pub class: TrackClass,
    #[dsl(unit = "m/s")]
    pub mean_speed_m_s: f32,
}

/// 🗺️ Georeferenced raster products, each stored as a pixel `ImageAsset` (DSM/DTM as 16-bit-encoded
/// PNG, ortho as an RGB PNG) rather than an embedded float grid — rasters are pixels, so they follow
/// the same persistence rule as every other image in this document instead of a bespoke height-grid
/// packed-array shape.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct GeoProducts {
    pub dsm_asset_id: Option<String>,
    pub dtm_asset_id: Option<String>,
    pub ortho_asset_id: Option<String>,
}

/// ✅️ A plain-JSON mirror of the QC-relevant fields of `remodel_geo::QualityReport`, plus the
/// watertight snapshot (mirroring `QualityReport.watertight: Option<WatertightReport>`) and a few
/// cheap scalar summaries (`remodel_engine` computes these once at the end of a run; the underlying
/// per-camera covariance/per-point-sigma arrays and density/overlap rasters stay plugin-runtime).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct QcReportSnapshot {
    pub reprojection_rms_px: f64,
    pub gcp_checkpoint_rmse: Option<f64>,
    #[dsl(block)]
    pub watertight: Option<WatertightReportSnapshot>,
    pub mean_track_length: f32,
    pub registered_frame_ratio: f32,
    pub dense_coverage_ratio: f32,
    pub warnings: Vec<String>,
}

/// 📦️ Everything a completed (or partially completed) reconstruction run has produced so far.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct ReconstructionResults {
    #[dsl(block)]
    pub sparse: Option<SparseCloud>,
    #[dsl(block)]
    pub dense: Option<DenseCloud>,
    #[dsl(block)]
    pub mesh: RemodelMesh,
    #[dsl(block)]
    pub trajectory: Option<CameraTrajectory>,
    #[dsl(table)]
    pub tracks: Vec<MotionTrackSummary>,
    #[dsl(block)]
    pub geo: Option<GeoProducts>,
    #[dsl(block)]
    pub qc: Option<QcReportSnapshot>,
}

/// 📸️ Persisted remodel snapshot — re-exported from `📸️snapshot/🧬️schema`.
pub use crate::artifacts::remodel::schema::snapshot::RemodelSnapshot;

/// 🌱️ An empty scene seeded with a placeholder box mesh, so the 3D editor/preview always has
/// something to render before any media has been imported/reconstructed.
pub fn default_remodel_scene() -> RemodelSnapshot {
    RemodelSnapshot {
        schema: REMODEL_DOCUMENT_SCHEMA.into(),
        id: "remodel".into(),
        streams: Vec::new(),
        assets: BTreeMap::new(),
        durable_artifacts: RemodelDurableArtifactStore::new(),
        calibration: CalibrationState::default(),
        params: ReconstructionParams::default(),
        gcps: Vec::new(),
        job: ReconstructionJob::default(),
        results: ReconstructionResults { mesh: RemodelMesh { mesh: placeholder_remodel_mesh_handle(), source: MeshSource::Placeholder, ..RemodelMesh::default() }, ..ReconstructionResults::default() },
    }
}
//#endregion 🔖️Domain

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🏗️ Shared fixture for both the JSON and the `.remodel` DSL round-trip tests: a scene that
    /// exercises every optional/collection field at least once, so `assert_dsl_round_trip` (and the
    /// pre-existing `populated_scene_roundtrips_through_json`) actually walk the full document shape
    /// instead of just `default_remodel_scene()`'s mostly-empty surface. Duplicated verbatim into every
    /// taxonomy node that needs it (`🗣️dsl`, `🔧️op`, `🎒️pack`) since it is a private test-only builder.
    async fn populated_scene_fixture() -> RemodelSnapshot {
        let mut scene = default_remodel_scene();
        scene.streams.push(MediaStream {
            id: "stream-1".into(),
            name: "front".into(),
            kind: MediaKind::Video,
            camera_id: Some("cam-1".into()),
            sync_offset_ms: 12.5,
            fps_hint: 30.0,
            frames: vec![FrameRef { index: 0, timestamp_ms: 0.0, asset_id: "asset-1".into() }],
            source: Some(VideoSource { name: "front.mp4".into(), container: "mp4".into(), codec: VideoCodec::Avc, duration_ms: 6633.3, frame_count: 199, width: 1920, height: 1080 }),
        });
        let asset_one = ImageAsset { mime: "image/jpeg".into(), data: "abcd".into(), width: 4, height: 4 };
        scene.assets.insert("asset-1".into(), store_remodel_asset("asset-1", &asset_one));
        scene.calibration.cameras.push(CameraCalibration {
            id: "cam-1".into(),
            label: "Front".into(),
            model: "brownConrady".into(),
            fx: 1000.0,
            fy: 1000.0,
            cx: 512.0,
            cy: 384.0,
            skew: 0.0,
            distortion: [0.01, -0.02, 0.0, 0.0, 0.0],
            rms_reprojection_px: Some(0.4),
            locked: false,
        });
        scene.calibration.rig.push(RigExtrinsic::default());
        scene.gcps.push(GroundControlPoint { id: "gcp-1".into(), name: "Corner".into(), world_position: [1.0, 2.0, 3.0], observations: vec![GcpObservation { stream_id: "stream-1".into(), frame_index: 0, pixel: [10.0, 20.0] }] });
        scene.params.ingest.min_sharpness = 0.4;
        scene.params.mesh.texture_size = 4096;
        scene.job.stage = ReconstructionStage::BundleAdjusting;
        scene.job.progress_0_1 = 0.42;
        scene.job.started_at_ms = Some(1000.0);
        scene.job.error = Some("retry needed".into());
        scene.job.camera_poses_preview.push(CameraPosePreview { camera_id: "cam-1".into(), ..CameraPosePreview::default() });
        scene.job.sparse_point_cloud_preview = PackedF32::from_f32_slice(&[0.1, 0.2, 0.3]);
        scene.results.sparse = Some(SparseCloud { points: PackedF32::from_f32_slice(&[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]), colors: Some(PackedU8::from_u8_slice(&[255, 0, 0, 0, 255, 0])) });
        scene.results.dense =
            Some(DenseCloud { positions: PackedF32::from_f32_slice(&[0.0, 0.0, 0.0]), colors: Some(PackedU8::from_u8_slice(&[0, 0, 255])), confidence: Some(PackedF32::from_f32_slice(&[0.9])), classification: Some(PackedU8::from_u8_slice(&[2])) });
        scene.results.mesh = RemodelMesh {
            mesh: mint_and_stash_mesh(semio_framework::mesh_from_kind("box")),
            source: MeshSource::Reconstructed,
            texture_asset_id: Some("tex-1".into()),
            watertight: Some(WatertightReportSnapshot {
                vertex_count: 512,
                triangle_count: 1020,
                boundary_edge_count: 0,
                boundary_loop_count: 0,
                non_manifold_edge_count: 0,
                non_manifold_vertex_count: 0,
                connected_components: 1,
                consistently_oriented: true,
                euler_characteristic: 2,
                genus: Some(0),
                signed_volume: 12.5,
                self_intersection_pairs: Some(0),
                closed_fallback_used: false,
                is_closed: true,
                is_two_manifold: true,
                is_watertight: true,
            }),
        };
        scene.results.trajectory = Some(CameraTrajectory {
            poses: vec![
                CameraPosePreview { camera_id: "cam-1".into(), rotation_wxyz: [1.0, 0.0, 0.0, 0.0], translation: [0.0, 0.0, 0.0] },
                CameraPosePreview { camera_id: "cam-1".into(), rotation_wxyz: [0.999, 0.001, 0.0, 0.0], translation: [0.1, 0.0, 0.0] },
            ],
        });
        scene.results.tracks.push(MotionTrackSummary { id: "track-1".into(), length: 42, class: TrackClass::Moving, mean_speed_m_s: 1.2 });
        scene.results.geo = Some(GeoProducts { dsm_asset_id: Some("asset-dsm".into()), dtm_asset_id: Some("asset-dtm".into()), ortho_asset_id: Some("asset-ortho".into()) });
        scene.results.qc = Some(QcReportSnapshot {
            reprojection_rms_px: 0.5,
            gcp_checkpoint_rmse: Some(0.02),
            watertight: scene.results.mesh.watertight.clone(),
            mean_track_length: 6.0,
            registered_frame_ratio: 1.0,
            dense_coverage_ratio: 0.95,
            warnings: vec!["low overlap on frame 12".into()],
        });
        scene
    }

    #[semio_framework_async_macros::async_test]
    async fn default_scene_has_placeholder_mesh() {
        let scene = default_remodel_scene();
        assert_eq!(scene.results.mesh.source, MeshSource::Placeholder);
        let mesh = remodel_mesh_workspace(&scene.results.mesh.mesh).expect("working-scene cache warm right after default_remodel_scene()");
        assert!(!mesh.positions.is_empty());
        assert!(!mesh.indices.is_empty());
        assert_eq!(scene.results.mesh.watertight, None);
        assert!(scene.streams.is_empty());
        assert!(scene.assets.is_empty());
        assert!(scene.gcps.is_empty());
        assert_eq!(scene.job, ReconstructionJob::default());
        assert_eq!(scene.results.sparse, None);
        assert_eq!(scene.results.dense, None);
        assert_eq!(scene.results.trajectory, None);
        assert!(scene.results.tracks.is_empty());
        assert_eq!(scene.results.geo, None);
        assert_eq!(scene.results.qc, None);
    }

    #[semio_framework_async_macros::async_test]
    async fn scene_roundtrips_through_json() {
        let scene = default_remodel_scene();
        let json = serde_json::to_string(&scene).expect("serialize");
        let parsed: RemodelSnapshot = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, scene);
    }

    #[semio_framework_async_macros::async_test]
    async fn populated_scene_roundtrips_through_json() {
        let scene = populated_scene_fixture();
        let json = serde_json::to_string(&scene).expect("serialize");
        let parsed: RemodelSnapshot = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, scene);
    }

    #[semio_framework_async_macros::async_test]
    async fn packed_f32_roundtrips_exactly() {
        let values = vec![1.5_f32, -2.25, 3.0, f32::MIN_POSITIVE, -0.0];
        let packed = PackedF32::from_f32_slice(&values);
        let value = serde_json::to_value(&packed).expect("serialize");
        assert!(value.is_string(), "PackedF32 must serialize as a base64 string, got {value:?}");
        let parsed: PackedF32 = serde_json::from_value(value).expect("deserialize");
        assert_eq!(parsed, packed);
        assert_eq!(parsed.to_f32_vec(), values);

        let empty = PackedF32::default();
        assert!(empty.is_empty());
        assert_eq!(empty.to_f32_vec(), Vec::<f32>::new());
    }

    #[semio_framework_async_macros::async_test]
    async fn packed_u8_roundtrips_exactly() {
        let values = vec![0_u8, 128, 255, 64];
        let packed = PackedU8::from_u8_slice(&values);
        let value = serde_json::to_value(&packed).expect("serialize");
        assert!(value.is_string(), "PackedU8 must serialize as a base64 string, got {value:?}");
        let parsed: PackedU8 = serde_json::from_value(value).expect("deserialize");
        assert_eq!(parsed, packed);
        assert_eq!(parsed.to_u8_vec(), values);

        let empty = PackedU8::default();
        assert!(empty.is_empty());
        assert_eq!(empty.to_u8_vec(), Vec::<u8>::new());
    }

    #[semio_framework_async_macros::async_test]
    async fn reconstruction_stage_serde_is_stable() {
        let cases: [(ReconstructionStage, &str); 18] = [
            (ReconstructionStage::Idle, "\"idle\""),
            (ReconstructionStage::Ingesting, "\"ingesting\""),
            (ReconstructionStage::Calibrating, "\"calibrating\""),
            (ReconstructionStage::ExtractingFeatures, "\"extracting-features\""),
            (ReconstructionStage::MatchingFeatures, "\"matching-features\""),
            (ReconstructionStage::EstimatingPoses, "\"estimating-poses\""),
            (ReconstructionStage::BundleAdjusting, "\"bundle-adjusting\""),
            (ReconstructionStage::Georeferencing, "\"georeferencing\""),
            (ReconstructionStage::DenseStereo, "\"dense-stereo\""),
            (ReconstructionStage::FusingVolume, "\"fusing-volume\""),
            (ReconstructionStage::ExtractingSurface, "\"extracting-surface\""),
            (ReconstructionStage::CleaningMesh, "\"cleaning-mesh\""),
            (ReconstructionStage::Texturing, "\"texturing\""),
            (ReconstructionStage::TrackingMotion, "\"tracking-motion\""),
            (ReconstructionStage::DerivingGeoProducts, "\"deriving-geo-products\""),
            (ReconstructionStage::ReportingQc, "\"reporting-qc\""),
            (ReconstructionStage::Done, "\"done\""),
            (ReconstructionStage::Failed, "\"failed\""),
        ];
        for (stage, expected) in cases {
            assert_eq!(serde_json::to_string(&stage).expect("serialize"), expected);
        }
    }
}
//#endregion 🧪️Tests
