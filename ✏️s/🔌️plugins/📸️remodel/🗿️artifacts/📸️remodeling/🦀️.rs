//! 📸️ Remodeling scene document — schema-only photogrammetry/videogrammetry project state (media
//! streams, calibration, ground control points, reconstruction params/job/results) shared as CRDT
//! operations. The actual algorithms live in the editor surface's own `✏️editor/⚙️engine/` topic files
//! (`images`/`video`/`camera`/`feature`/`sfm`/`dense`/`mesh`/`motion`/`geo`/`reconstruction`,
//! relocated out of this artifact tree by 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES,
//! #2553 — an artifact is a schema plus IO, never an engine), none of which this node references:
//! heavier runtime types (`Se3`, `Intrinsics`, `Distortion`, `WatertightReport`, decoded pyramids,
//! match graphs, depth maps, TSDF volumes) are not designed for durable CRDT persistence, so every
//! reference to their shape below is a plain-JSON (or `Packed*`) snapshot the app fills in, never the
//! library type itself.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
//#region 🧮️MathInternals
// 🧮️ 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3d: crate-root
// aliases onto the compute-internals mounted below in `artifacts::remodeling::…::schema` — every
// `crate::algebra::`/`crate::optimize::`/`crate::lie::`/`crate::signal::`/`crate::spatial::` call
// site (the moved files' own internal references, and the app-engine files that used to say
// `math::algebra::` etc.) resolves through these, exactly as the old `math::` extern-prelude
// name used to. `semio-framework-math` is no longer a dependency of this crate.
pub(crate) use crate::standards::v1::subsets::any::schema::algebra_internals as algebra;
pub(crate) use crate::standards::v1::subsets::any::schema::lie_internals as lie;
pub(crate) use crate::standards::v1::subsets::any::schema::optimize_internals as optimize;
pub(crate) use crate::standards::v1::subsets::any::schema::signal_internals as signal;
pub(crate) use crate::standards::v1::subsets::any::schema::spatial_internals as spatial;
//#endregion 🧮️MathInternals
extern crate self as semio_s_artifact_remodel_remodeling;

use semio_framework::MeshData;
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, OnceLock};

//#region 🔖️ArtifactKind
/// 🗿️ The `3d.remodeling` artifact kind — lifted verbatim out of the manifest builder's
/// `.artifact_kind(…)` literal so the artifact node, not the app, owns its own identity.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "3d.remodeling".into(),
        name: "3D Remodeling".into(),
        source_format: "remodeling.scene".into(),
        component_kind: "remodeling".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        schema: "remodeling.scene".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.json".into(), "stdio.las".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.png".into(), "stdio.stl".into()],
        import_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.json".into(), "stdio.las".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.png".into(), "stdio.stl".into()],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called five different global registries directly from
/// a plugin `.setup()` callback. `crate::editor::remodeling::config::schema::register_app_schema()` is the
/// one exception, still called from `📸️remodeling/🦀️.rs`'s own `.setup()`: it registers the
/// `RemodelingPlayApp` CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration` deliberately
/// has no field for (see that struct's own doc) — `register_app_schema_descriptor` is not in §6's
/// artifact-scoped function set. Relocated from `⚙️engine/🦀️.rs` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g3): `⚙️engine` was removed from the taxonomy
/// and `declaration()` describes the artifact, not engine behaviour, so its home is the artifact
/// root alongside `artifact_kind()`.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.remodel.remodeling.standard.v1", "standard", "1", &[], None),
        ("s.remodel.remodeling.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.remodel.remodeling.schema.artifact", "schema", "s.remodel.remodeling", &[("schema", "s.remodel.remodeling")], None),
        ("s.remodel.remodeling.inference.artifact", "inference", "s.remodel.remodeling.inference", &[("schema", "s.remodel.remodeling.inference")], None),
        ("s.remodel.remodeling.composer.native", "composer", "s.remodel.remodeling@1/*", &[("dialect", "s.remodel.remodeling@1/*")], None),
        ("s.remodel.remodeling.composer.format-1", "composer", "s.stdio.las@1.0/*", &[("dialect", "s.stdio.las@1.0/*")], None),
        ("s.remodel.remodeling.composer.format-2", "composer", "s.stdio.ply@1.0/*", &[("dialect", "s.stdio.ply@1.0/*")], None),
        ("s.remodel.remodeling.composer.format-3", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.remodel.remodeling.composer.format-4", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.remodel.remodeling.composer.format-5", "composer", "s.stdio.dwg@ac1018/*", &[("dialect", "s.stdio.dwg@ac1018/*")], None),
        ("s.remodel.remodeling.composer.format-6", "composer", "s.stdio.stl@ascii/*", &[("dialect", "s.stdio.stl@ascii/*")], None),
        ("s.remodel.remodeling.composer.format-7", "composer", "s.stdio.gltf@2.0/*", &[("dialect", "s.stdio.gltf@2.0/*")], None),
        ("s.remodel.remodeling.composer.format-8", "composer", "s.stdio.obj@3.0/*", &[("dialect", "s.stdio.obj@3.0/*")], None),
        ("s.remodel.remodeling.grammar.1", "grammar", "remodeling.document", &[("grammar", "remodeling.document")], None),
        ("s.remodel.remodeling.grammar.2", "grammar", "remodeling.op", &[("grammar", "remodeling.op")], None),
        ("s.remodel.remodeling.grammar.3", "grammar", "remodeling.diff", &[("grammar", "remodeling.diff")], None),
        ("s.remodel.remodeling.grammar.4", "grammar", "remodeling.pack", &[("grammar", "remodeling.pack")], None),
        ("s.remodel.remodeling.grammar.5", "grammar", "remodeling.spr", &[("grammar", "remodeling.spr")], None),
        ("s.remodel.remodeling.codec.document-1", "codec", "remodeling.scene:remodeling", &[("codec", "remodeling.scene"), ("codec-extension", "16:remodeling.scene:remodeling")], None),
        ("s.remodel.remodeling.localization.en", "localization", "Remodeling", &[], Some(("en", "Remodeling"))),
        ("s.remodel.remodeling.localization.de", "localization", "Umbau", &[], Some(("de", "Umbau"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.remodel.remodeling")?);
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

/// 🗿️ New declaration tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §2)
/// — replaces the OLD `declaration()`/`pilot_languages()` pair outright (atomic cutover, no dual
/// registration channel), the same cutover `🗒️note`/`🔱️trinity` already made. The five
/// `dsl::LanguageSpec`s `pilot_languages()` built now live beside their own codec in the subset's
/// `🚪️io/🦀️.rs` `io()`, which `commit_artifact_declarations` registers; `.composers(…)`'s single
/// surviving native row has no field on this tree because `io()`'s typed `IoEntry` slice replaced
/// that channel. `localization: &[]` is a documented shortfall (debt D1): the real en/de names
/// ("Remodeling"/"Umbau") still live on `definition()`'s kept `ArtifactCapability` rows.
pub fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<crate::RemodelApps> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.remodel.remodeling").expect("canonical remodeling kind"), localization: &[], standards: vec![standards::v1::standard()] }
}

//#endregion 🔖️Register

pub use crate::schema::mutations::RemodelingMutation;

pub use crate::schema::diff::RemodelingDiff;

pub const REMODELING_DOCUMENT_SCHEMA: &str = "remodeling.scene";

/// 🪪️ Ticket `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` contract §1 canonical surface id
/// grammar (`<artifact_kind>@<standard>/<subset>#<role>`). Lives at the ARTIFACT level (not under
/// `editor`/`viewer`) so a viewer file can read it without ever importing through the sibling editor
/// module. `artifact_kind = "s.remodel.remodeling"` is `s.<plugin-id>.<artifact-name>`, the fleet-wide
/// dialect grammar (`s.trinity.jack`, `s.puzzle.puzzle3d`, `s.block.block2d`, `s.procedural.generation2d`)
/// and this artifact's own identity (`ArtifactIdentity::parse("s.remodel.remodeling")` plus the
/// `composer.native` capability row above, and the `🚪️io`/`🧬️schema` `DIALECT` constants). It is NOT
/// `artifact_kind()`'s OS-level `"3d.remodeling"` kind id (a different, unrelated namespace).
/// `standard`/`subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location.
pub const REMODELING_DIALECT: semio_framework_plugin::Dialect = semio_framework_plugin::Dialect { artifact_kind: "s.remodel.remodeling", standard: semio_framework_plugin::StandardId("1"), subset: semio_framework_plugin::SubsetId::ANY };

//#region 🧩️Composition
/// 🧩️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (design map §4: "remodeling→C:mesh R:image").
/// Two content-duplication shapes, both verified against real code (not assumed from the one-line
/// design summary):
///
/// 1. **`results.mesh.mesh` (was `MeshData`, the reconstructed/placeholder mesh's flat buffers) is now
///    a composed `s.stdio.semio/v1/mesh` CHILD** (`RemodelingMeshChild = store::ArtifactChild<
///    SemioMeshSnapshot>`) — exactly `💠️lowpoly`'s own pattern (this ticket's closest precedent for
///    opaque mesh composition), extended with a REAL bidirectional converter
///    (`crate::standards::v1::subsets::any::io::{mesh_data_to_semio_mesh,
///    semio_mesh_to_mesh_data}`, already real — the export path already built the forward direction).
///
/// 2. **`assets: BTreeMap<String, ImageAsset>` (embedded mime+base64 pixel bytes: video frames,
///    baked mesh textures, DSM/DTM/ortho rasters) is now `BTreeMap<String, RemodelingAssetChild>`
///    (`RemodelingAssetChild = store::ArtifactChild<SemioImageSnapshot>`)** — the design line's "R:image"
///    literally means `ArtifactLink` (an INDEPENDENT-lifecycle reference, `store::ArtifactLink`'s own
///    doc comment: "renders as a chip, never nests inline"). `remodeling`'s assets are NOT independent
///    documents referenced from elsewhere — they are embedded content OWNED by this exact document
///    (keyed by an id that only this document's own `MediaStream.frames`/`RemodelingMesh.
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
/// (`image_key`-equivalent lookups by asset id; `ReplaceMeshResult`'s whole-`RemodelingMesh` payload
/// shape) — the type/mutation/persistence layer is fully real, only the derive's SCHEMA
/// INTROSPECTION table is incomplete for these two fields.
pub type RemodelingAssetChild = store::ArtifactChild<SemioImageSnapshot>;
pub type RemodelingMeshChild = store::ArtifactChild<SemioMeshSnapshot>;

/// 🧩️ Restart-stable content authority. Every encoded leaf decodes to at most 4 KiB; values carry
/// only bounded metadata and content-addressed leaves, never a whole image or mesh object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelingDurableArtifact {
    pub kind: String,
    pub mime: Option<String>,
    pub width: u32,
    pub height: u32,
    pub chunks: Vec<String>,
}

pub type RemodelingDurableArtifactStore = BTreeMap<String, RemodelingDurableArtifact>;

const REMODELING_DURABLE_CHUNK_RAW_BYTES: usize = 4_096;
const REMODELING_MAX_STAGED_BLOBS: usize = 32;
const REMODELING_SPARSE_CONTENT_BYTES: usize = 512 * 3 * 4;
const REMODELING_SPARSE_CONTENT_CHUNKS: u64 = 2;
const REMODELING_RASTER_CONTENT_BYTES: usize = 1_114_112;
const REMODELING_RASTER_CONTENT_CHUNKS: u64 = 272;
const REMODELING_MESH_CONTENT_BYTES: usize = 87_552 + 30;
const REMODELING_MESH_CONTENT_CHUNKS: u64 = 30;
const REMODELING_EMPTY_MESH_CHILD_ID: &str = "remodeling-mesh-constant-empty";
const REMODELING_BOX_MESH_CHILD_ID: &str = "remodeling-mesh-constant-box";
const REMODELING_BOUNDED_MESH_VERTICES: usize = 512;
const REMODELING_BOUNDED_MESH_TRIANGLES: usize = 512;

//#region 🔖️AssetHandles
fn mint_asset_child_handle(asset_id: &str, content_hash: u64) -> RemodelingAssetChild {
    let child_id = format!("remodeling-asset-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() };
    let target = store::os_io::ArtifactRef { artifact_id: format!("{asset_id}-image"), dialect };
    store::ArtifactChild::new(child_id, target)
}

pub fn committed_remodeling_asset_handle(asset_id: &str, content_id: &str) -> RemodelingAssetChild {
    let mut handle = mint_asset_child_handle(asset_id, 0);
    handle.child_id = content_id.into();
    handle
}

/// 🕸️ Deterministic content-addressed CHILD handle for one bounded durable asset.
pub fn image_asset_child_handle(asset_id: &str, asset: &ImageAsset) -> RemodelingAssetChild {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    asset.mime.hash(&mut hasher);
    asset.data.hash(&mut hasher);
    mint_asset_child_handle(asset_id, hasher.finish())
}

/// 🧩️ Admits a normal imported asset as independently bounded raw leaves. No whole `ImageAsset`
/// value is retained outside the typed mutation currently being reduced.
pub fn store_remodeling_asset(asset_id: &str, asset: &ImageAsset) -> RemodelingAssetChild {
    image_asset_child_handle(asset_id, asset)
}

/// 🧶️ Snapshot-owned decoded-leaf view for bounded consumers. Identity and display metadata stay
/// separate from the compressed payload, and no leaf can exceed 4 KiB.
#[derive(Clone)]
pub struct RemodelingAssetChunkSource {
    pub identity: String,
    pub mime: String,
    pub width: u32,
    pub height: u32,
    pub leaves: Vec<Arc<[u8]>>,
    pub byte_len: usize,
}

pub fn remodeling_asset_chunk_source(snapshot: &RemodelingSnapshot, asset_id: &str) -> Option<RemodelingAssetChunkSource> {
    let handle = snapshot.assets.get(asset_id)?;
    let artifact = snapshot.durable_artifacts.get(&handle.child_id).filter(|artifact| artifact.kind == "image")?;
    let mime = artifact.mime.clone()?;
    let mut leaves = Vec::with_capacity(artifact.chunks.len());
    let mut byte_len = 0usize;
    for encoded in &artifact.chunks {
        let chunk = decode_remodeling_durable_chunk(encoded)?;
        byte_len = byte_len.checked_add(chunk.len())?;
        if byte_len > REMODELING_RASTER_CONTENT_BYTES {
            return None;
        }
        leaves.push(Arc::from(chunk));
    }
    (!leaves.is_empty()).then(|| RemodelingAssetChunkSource { identity: handle.child_id.clone(), mime, width: artifact.width, height: artifact.height, leaves, byte_len })
}

pub fn remodeling_asset_dimensions(snapshot: &RemodelingSnapshot, asset_id: &str) -> Option<(u32, u32)> {
    let handle = snapshot.assets.get(asset_id)?;
    let artifact = snapshot.durable_artifacts.get(&handle.child_id).filter(|artifact| artifact.kind == "image")?;
    Some((artifact.width, artifact.height))
}

/// 🖼️ Reconstructs a bounded API value for export and UI presentation only. Active reconstruction
/// consumes `remodeling_asset_chunk_source` directly and never passes through this whole-value facade.
pub fn remodeling_asset(snapshot: &RemodelingSnapshot, asset_id: &str) -> Option<ImageAsset> {
    let source = remodeling_asset_chunk_source(snapshot, asset_id)?;
    let mut bytes = Vec::with_capacity(source.byte_len);
    for leaf in source.leaves {
        bytes.extend_from_slice(&leaf);
    }
    Some(ImageAsset { mime: source.mime, data: base64_codec::base64_standard_encode(bytes), width: source.width, height: source.height })
}

pub fn durable_remodeling_asset(asset: &ImageAsset) -> Option<RemodelingDurableArtifact> {
    let bytes = base64_codec::base64_standard_decode(asset.data.as_bytes()).ok()?;
    (bytes.len() <= REMODELING_RASTER_CONTENT_BYTES).then_some(RemodelingDurableArtifact {
        kind: "image".into(),
        mime: Some(asset.mime.clone()),
        width: asset.width,
        height: asset.height,
        chunks: bytes.chunks(REMODELING_DURABLE_CHUNK_RAW_BYTES).map(base64_codec::base64_standard_encode).collect(),
    })
}

pub fn durable_staged_remodeling_asset(staging_id: &str, kind: &str, mime: Option<String>, width: u32, height: u32) -> Option<RemodelingDurableArtifact> {
    let content = remodeling_asset_content().lock().expect("remodeling asset content lock");
    let blob = content.get(staging_id)?;
    Some(RemodelingDurableArtifact { kind: kind.into(), mime, width, height, chunks: (0..u64::try_from(blob.chunks.len()).ok()?).map(|index| base64_codec::base64_standard_encode(blob.chunks.get(&index).expect("contiguous staged asset"))).collect() })
}

//#region 🔖️ReplayableAssetBlobs
struct RemodelingAssetBlob {
    chunks: BTreeMap<u64, Arc<[u8]>>,
    kind: Option<RemodelingAssetContentKind>,
    byte_count: usize,
    digest: [u64; 4],
    digest_len: u64,
}

impl Default for RemodelingAssetBlob {
    fn default() -> Self {
        Self { chunks: BTreeMap::new(), kind: None, byte_count: 0, digest: [0x6c62272e07bb0142, 0x62b821756295c58d, 0x9e3779b185ebca87, 0xc2b2ae3d27d4eb4f], digest_len: 0 }
    }
}

/// 🛡️ Typed bounded-admission result shared by durable asset and mesh staging.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RemodelingStagingFault {
    Busy,
    Invalid,
}

/// 🏷️ Exact durable asset envelope selected before the first chunk is retained.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RemodelingAssetContentKind {
    Sparse,
    Raster,
}

impl RemodelingAssetContentKind {
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
            Self::Sparse => REMODELING_SPARSE_CONTENT_BYTES,
            Self::Raster => REMODELING_RASTER_CONTENT_BYTES,
        }
    }

    fn max_chunks(self) -> u64 {
        match self {
            Self::Sparse => REMODELING_SPARSE_CONTENT_CHUNKS,
            Self::Raster => REMODELING_RASTER_CONTENT_CHUNKS,
        }
    }
}

fn record_content_digest(digest: &mut [u64; 4], digest_len: &mut u64, bytes: &[u8]) -> Result<(), RemodelingStagingFault> {
    for byte in bytes {
        *digest_len = (*digest_len).checked_add(1).ok_or(RemodelingStagingFault::Busy)?;
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

static REMODELING_PRIVATE_ASSET_STAGING: OnceLock<Mutex<BTreeMap<String, RemodelingAssetBlob>>> = OnceLock::new();

fn remodeling_asset_content() -> &'static Mutex<BTreeMap<String, RemodelingAssetBlob>> {
    REMODELING_PRIVATE_ASSET_STAGING.get_or_init(|| Mutex::new(BTreeMap::new()))
}

pub fn stage_remodeling_asset_chunk(staging_id: &str, kind: RemodelingAssetContentKind, index: u64, encoded: &str) -> Result<(), RemodelingStagingFault> {
    let Some(bytes) = decode_remodeling_durable_chunk(encoded) else { return Err(RemodelingStagingFault::Invalid) };
    let next_count = index.checked_add(1).ok_or(RemodelingStagingFault::Busy)?;
    let mut content = remodeling_asset_content().lock().expect("remodeling asset content lock");
    if !content.contains_key(staging_id) && content.len() >= REMODELING_MAX_STAGED_BLOBS {
        return Err(RemodelingStagingFault::Busy);
    }
    let blob = content.entry(staging_id.into()).or_default();
    if let Some(existing) = blob.chunks.get(&index) {
        return (blob.kind == Some(kind) && existing.as_ref() == bytes).then_some(()).ok_or(RemodelingStagingFault::Invalid);
    }
    let next_bytes = blob.byte_count.checked_add(bytes.len()).ok_or(RemodelingStagingFault::Busy)?;
    if u64::try_from(blob.chunks.len()).ok() != Some(index) || blob.kind.is_some_and(|existing| existing != kind) || next_count > kind.max_chunks() || next_bytes > kind.max_bytes() {
        content.remove(staging_id);
        return Err(RemodelingStagingFault::Invalid);
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
pub fn staged_remodeling_asset_chunk_count(staging_id: &str) -> u64 {
    remodeling_asset_content().lock().expect("remodeling asset content lock").get(staging_id).and_then(|blob| u64::try_from(blob.chunks.len()).ok()).unwrap_or(0)
}

fn staged_asset_commit_is_valid(content: &BTreeMap<String, RemodelingAssetBlob>, staging_id: &str, content_id: &str, chunk_count: u64, expected_kind: Option<RemodelingAssetContentKind>) -> bool {
    let Some(blob) = content.get(staging_id) else { return false };
    let Some(kind) = blob.kind else { return false };
    chunk_count != 0
        && expected_kind.is_none_or(|expected| expected == kind)
        && chunk_count <= kind.max_chunks()
        && blob.byte_count <= kind.max_bytes()
        && u64::try_from(blob.chunks.len()).ok() == Some(chunk_count)
        && blob.digest_len == u64::try_from(blob.byte_count).unwrap_or(u64::MAX)
        && content_digest_id("remodeling-asset", blob.digest, blob.digest_len) == content_id
}

pub fn discard_staged_remodeling_asset(staging_id: &str) {
    remodeling_asset_content().lock().expect("remodeling asset content lock").remove(staging_id);
}

pub fn remodeling_asset_stage_key(staging_id: &str, kind: RemodelingAssetContentKind, index: u64) -> String {
    format!("__remodeling_asset_stage__:{}|{staging_id}:{index}", kind.wire())
}

pub fn remodeling_asset_stage_parts(key: &str) -> Option<(RemodelingAssetContentKind, &str, u64)> {
    let tail = key.strip_prefix("__remodeling_asset_stage__:")?;
    let (staging_id, index) = tail.rsplit_once(':')?;
    let (kind, staging_id) = staging_id.split_once('|')?;
    Some((RemodelingAssetContentKind::parse(kind)?, staging_id, index.parse().ok()?))
}

pub fn remodeling_asset_content_handle(content_id: &str, staging_id: &str, chunk_count: u64) -> String {
    format!("remodeling-content:{content_id}|{staging_id}|{chunk_count}")
}

pub fn remodeling_asset_content_handle_parts(value: &str) -> Option<(&str, &str, u64)> {
    let tail = value.strip_prefix("remodeling-content:")?;
    let (tail, chunk_count) = tail.rsplit_once('|')?;
    let (content_id, staging_id) = tail.split_once('|')?;
    Some((content_id, staging_id, chunk_count.parse().ok()?))
}
//#endregion 🔖️ReplayableAssetBlobs
//#endregion 🔖️AssetHandles

//#region 🔖️MeshHandle
fn mesh_child_handle(child_id: String, artifact_id: String) -> RemodelingMeshChild {
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "mesh".into() };
    let target = store::os_io::ArtifactRef { artifact_id, dialect };
    store::ArtifactChild::new(child_id, target)
}

fn decode_remodeling_durable_chunk(encoded: &str) -> Option<Vec<u8>> {
    let encoded_limit = REMODELING_DURABLE_CHUNK_RAW_BYTES.checked_add(2)?.checked_div(3)?.checked_mul(4)?;
    if encoded.len() > encoded_limit {
        return None;
    }
    let bytes = base64_codec::base64_standard_decode(encoded).ok()?;
    (bytes.len() <= REMODELING_DURABLE_CHUNK_RAW_BYTES).then_some(bytes)
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
    format!("remodeling-mesh-{:016x}{:016x}{:016x}{:016x}-{len:016x}", digest[0], digest[1], digest[2], digest[3])
}

struct RemodelingMeshBlob {
    chunks: BTreeMap<u64, Arc<[u8]>>,
    digest: [u64; 4],
    digest_len: u64,
    byte_count: usize,
    last_field: Option<u8>,
}

impl Default for RemodelingMeshBlob {
    fn default() -> Self {
        Self { chunks: BTreeMap::new(), digest: [0x6c62272e07bb0142, 0x62b821756295c58d, 0x9e3779b185ebca87, 0xc2b2ae3d27d4eb4f], digest_len: 0, byte_count: 0, last_field: None }
    }
}

impl RemodelingMeshBlob {
    fn content_id(&self) -> String {
        format!("remodeling-mesh-{:016x}{:016x}{:016x}{:016x}-{:016x}", self.digest[0], self.digest[1], self.digest[2], self.digest[3], self.digest_len)
    }
}

static REMODELING_PRIVATE_MESH_STAGING: OnceLock<Mutex<BTreeMap<String, RemodelingMeshBlob>>> = OnceLock::new();

fn remodeling_mesh_blobs() -> &'static Mutex<BTreeMap<String, RemodelingMeshBlob>> {
    REMODELING_PRIVATE_MESH_STAGING.get_or_init(|| Mutex::new(BTreeMap::new()))
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
                0 | 1 => REMODELING_BOUNDED_MESH_VERTICES * 3,
                2 => REMODELING_BOUNDED_MESH_VERTICES * 4,
                4 => REMODELING_BOUNDED_MESH_VERTICES * 2,
                7 => REMODELING_BOUNDED_MESH_TRIANGLES * 6,
                _ => REMODELING_BOUNDED_MESH_TRIANGLES * 4,
            };
            if target.len().checked_add(component_count).is_none_or(|count| count > limit) {
                return false;
            }
            target.extend(values.as_chunks::<4>().0.iter().map(|value| f32::from_le_bytes(value.try_into().expect("four-byte mesh f32"))));
        }
        3 | 5 | 6 | 8 if values.len() % 4 == 0 => {
            let target = match field {
                3 => &mut mesh.indices,
                5 => &mut mesh.face_ids,
                6 => &mut mesh.vertex_ids,
                _ => &mut mesh.edge_ids,
            };
            let limit = match field {
                3 => REMODELING_BOUNDED_MESH_TRIANGLES * 3,
                5 => REMODELING_BOUNDED_MESH_TRIANGLES,
                6 => REMODELING_BOUNDED_MESH_VERTICES,
                _ => REMODELING_BOUNDED_MESH_TRIANGLES * 3,
            };
            if target.len().checked_add(component_count).is_none_or(|count| count > limit) {
                return false;
            }
            target.extend(values.as_chunks::<4>().0.iter().map(|value| u32::from_le_bytes(value.try_into().expect("four-byte mesh u32"))));
        }
        10 => {
            if mesh.edge_is_seam.len().checked_add(values.len()).is_none_or(|count| count > REMODELING_BOUNDED_MESH_TRIANGLES * 3) {
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

fn mesh_from_blob(blob: &RemodelingMeshBlob) -> Option<MeshData> {
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
pub fn stage_remodeling_mesh_chunk(staging_id: &str, index: u64, encoded: &str) -> Result<(), RemodelingStagingFault> {
    let Some(bytes) = decode_remodeling_durable_chunk(encoded) else { return Err(RemodelingStagingFault::Invalid) };
    let next_count = index.checked_add(1).ok_or(RemodelingStagingFault::Busy)?;
    let mut blobs = remodeling_mesh_blobs().lock().expect("remodeling mesh blob lock");
    if !blobs.contains_key(staging_id) && blobs.len() >= REMODELING_MAX_STAGED_BLOBS {
        return Err(RemodelingStagingFault::Busy);
    }
    let blob = blobs.entry(staging_id.into()).or_default();
    if let Some(existing) = blob.chunks.get(&index) {
        return (existing.as_ref() == bytes).then_some(()).ok_or(RemodelingStagingFault::Invalid);
    }
    let next_bytes = blob.byte_count.checked_add(bytes.len()).ok_or(RemodelingStagingFault::Busy)?;
    let mut digest = blob.digest;
    let mut digest_len = blob.digest_len;
    let mut candidate = match mesh_from_blob(blob) {
        Some(mesh) => mesh,
        None => {
            blobs.remove(staging_id);
            return Err(RemodelingStagingFault::Invalid);
        }
    };
    if record_content_digest(&mut digest, &mut digest_len, &bytes).is_err()
        || u64::try_from(blob.chunks.len()).ok() != Some(index)
        || next_count > REMODELING_MESH_CONTENT_CHUNKS
        || next_bytes > REMODELING_MESH_CONTENT_BYTES
        || !apply_mesh_chunk(&mut candidate, blob.last_field, &bytes)
    {
        blobs.remove(staging_id);
        return Err(RemodelingStagingFault::Invalid);
    }
    blob.last_field = bytes.first().copied();
    blob.byte_count = next_bytes;
    blob.digest = digest;
    blob.digest_len = digest_len;
    blob.chunks.insert(index, Arc::from(bytes));
    Ok(())
}

pub fn discard_staged_remodeling_mesh(staging_id: &str) {
    remodeling_mesh_blobs().lock().expect("remodeling mesh blob lock").remove(staging_id);
}

pub fn staged_remodeling_mesh_chunk_count(staging_id: &str) -> u64 {
    remodeling_mesh_blobs().lock().expect("remodeling mesh blob lock").get(staging_id).and_then(|blob| u64::try_from(blob.chunks.len()).ok()).unwrap_or(0)
}

fn staged_mesh_commit_is_valid(blobs: &BTreeMap<String, RemodelingMeshBlob>, staging_id: &str, content_id: &str, chunk_count: u64) -> bool {
    let Some(blob) = blobs.get(staging_id) else { return false };
    chunk_count != 0
        && chunk_count <= REMODELING_MESH_CONTENT_CHUNKS
        && blob.byte_count <= REMODELING_MESH_CONTENT_BYTES
        && u64::try_from(blob.chunks.len()).ok() == Some(chunk_count)
        && blob.digest_len == u64::try_from(blob.byte_count).unwrap_or(u64::MAX)
        && mesh_from_blob(blob).is_some_and(|mesh| mesh_is_within_resolution_envelope(&mesh))
        && blob.content_id() == content_id
}

/// 🏁️ Validates every terminal artifact before publishing any staged content, then applies all
/// promotions while both bounded stores remain exclusively held.
pub fn commit_staged_remodeling_reconstruction(assets: &[(&str, &str, u64, RemodelingAssetContentKind)], mesh: Option<(&str, &str, u64)>) -> bool {
    let mut asset_content = remodeling_asset_content().lock().expect("remodeling asset content lock");
    let mut mesh_content = remodeling_mesh_blobs().lock().expect("remodeling mesh blob lock");
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

pub fn remodeling_mesh_stage_asset_key(staging_id: &str, index: u64) -> String {
    format!("__remodeling_mesh_stage__:{staging_id}:{index}")
}

pub fn remodeling_mesh_stage_asset_parts(key: &str) -> Option<(&str, u64)> {
    let tail = key.strip_prefix("__remodeling_mesh_stage__:")?;
    let (staging_id, index) = tail.rsplit_once(':')?;
    Some((staging_id, index.parse().ok()?))
}

pub fn staged_remodeling_mesh_handle(content_id: &str, staging_id: &str) -> RemodelingMeshChild {
    mesh_child_handle(content_id.into(), format!("mesh-stage:{staging_id}"))
}

pub fn replayable_remodeling_mesh_handle(content_id: &str, staging_id: &str, chunk_count: u64) -> RemodelingMeshChild {
    mesh_child_handle(content_id.into(), format!("remodeling-mesh-log:{staging_id}:{chunk_count}"))
}

pub fn replayable_remodeling_mesh_handle_parts(handle: &RemodelingMeshChild) -> Option<(&str, &str, u64)> {
    let tail = handle.target.artifact_id.strip_prefix("remodeling-mesh-log:")?;
    let (staging_id, chunk_count) = tail.rsplit_once(':')?;
    Some((&handle.child_id, staging_id, chunk_count.parse().ok()?))
}

/// 🧊️ Stable empty-mesh handle with no process-owned payload.
pub fn empty_remodeling_mesh_handle() -> RemodelingMeshChild {
    mesh_child_handle(REMODELING_EMPTY_MESH_CHILD_ID.into(), "remodeling-mesh-constant:empty".into())
}

/// 📦️ Stable bounded placeholder handle with no process-owned payload.
pub fn placeholder_remodeling_mesh_handle() -> RemodelingMeshChild {
    mesh_child_handle(REMODELING_BOX_MESH_CHILD_ID.into(), "remodeling-mesh-constant:box".into())
}

fn mesh_is_within_resolution_envelope(mesh: &MeshData) -> bool {
    let vertices = mesh.positions.len().checked_div(3);
    let triangles = mesh.indices.len().checked_div(3);
    let Some(vertices) = vertices.filter(|_| mesh.positions.len().is_multiple_of(3)) else { return false };
    let Some(triangles) = triangles.filter(|_| mesh.indices.len().is_multiple_of(3)) else { return false };
    vertices <= REMODELING_BOUNDED_MESH_VERTICES
        && triangles <= REMODELING_BOUNDED_MESH_TRIANGLES
        && mesh.indices.iter().all(|index| usize::try_from(*index).ok().is_some_and(|index| index < vertices))
        && (mesh.normals.is_empty() || vertices.checked_mul(3) == Some(mesh.normals.len()))
        && (mesh.colors.is_empty() || vertices.checked_mul(3) == Some(mesh.colors.len()) || vertices.checked_mul(4) == Some(mesh.colors.len()))
        && (mesh.uvs.is_empty() || vertices.checked_mul(2) == Some(mesh.uvs.len()))
        && (mesh.face_ids.is_empty() || mesh.face_ids.len() == triangles)
        && (mesh.vertex_ids.is_empty() || mesh.vertex_ids.len() == vertices)
        && mesh.normals.len() <= REMODELING_BOUNDED_MESH_VERTICES * 3
        && mesh.colors.len() <= REMODELING_BOUNDED_MESH_VERTICES * 4
        && mesh.uvs.len() <= REMODELING_BOUNDED_MESH_VERTICES * 2
        && mesh.face_ids.len() <= REMODELING_BOUNDED_MESH_TRIANGLES
        && mesh.vertex_ids.len() <= REMODELING_BOUNDED_MESH_VERTICES
        && mesh.edge_positions.len() <= REMODELING_BOUNDED_MESH_TRIANGLES * 6
        && mesh.edge_ids.len() <= REMODELING_BOUNDED_MESH_TRIANGLES * 3
        && mesh.edge_uvs.len() <= REMODELING_BOUNDED_MESH_TRIANGLES * 4
        && mesh.edge_is_seam.len() <= REMODELING_BOUNDED_MESH_TRIANGLES * 3
        && mesh.paint_texture_base64.as_ref().is_none_or(|value| value.len() <= 24_576)
}

/// 🧱️ Returns the durable raw chunk count without reconstructing or cloning mesh fields.
pub fn bounded_remodeling_mesh_chunk_count(store: &RemodelingDurableArtifactStore, handle: &RemodelingMeshChild) -> Option<u64> {
    let (content_id, _, chunk_count) = replayable_remodeling_mesh_handle_parts(handle)?;
    let artifact = store.get(content_id).filter(|artifact| artifact.kind == "mesh")?;
    (u64::try_from(artifact.chunks.len()).ok() == Some(chunk_count)).then_some(chunk_count)
}

/// 🧱️ Resolves one admitted durable mesh chunk without cloning the reconstructed mesh.
pub fn bounded_remodeling_mesh_chunk(store: &RemodelingDurableArtifactStore, handle: &RemodelingMeshChild, index: u64) -> Option<Arc<[u8]>> {
    let (content_id, _, chunk_count) = replayable_remodeling_mesh_handle_parts(handle)?;
    if index >= chunk_count {
        return None;
    }
    let artifact = store.get(content_id).filter(|artifact| artifact.kind == "mesh")?;
    if u64::try_from(artifact.chunks.len()).ok() != Some(chunk_count) {
        return None;
    }
    Some(Arc::from(decode_remodeling_durable_chunk(artifact.chunks.get(usize::try_from(index).ok()?)?)?))
}

/// 🧱️ Resolves only fixed constants or reconstruction output admitted by the 512/512 envelope.
pub fn resolve_bounded_remodeling_mesh(store: &RemodelingDurableArtifactStore, handle: &RemodelingMeshChild) -> Option<MeshData> {
    match handle.child_id.as_str() {
        REMODELING_EMPTY_MESH_CHILD_ID => Some(MeshData::default()),
        REMODELING_BOX_MESH_CHILD_ID => Some(semio_framework::mesh_from_kind("box")),
        _ => {
            let (content_id, _, chunk_count) = replayable_remodeling_mesh_handle_parts(handle)?;
            let artifact = store.get(content_id).filter(|artifact| artifact.kind == "mesh")?;
            if u64::try_from(artifact.chunks.len()).ok() != Some(chunk_count) {
                return None;
            }
            let mut mesh = MeshData::default();
            let mut last_field = None;
            for encoded in &artifact.chunks {
                let chunk = decode_remodeling_durable_chunk(encoded)?;
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
static REMODELING_TEST_MESHES: OnceLock<Mutex<BTreeMap<String, MeshData>>> = OnceLock::new();

#[cfg(test)]
fn remodeling_test_meshes() -> &'static Mutex<BTreeMap<String, MeshData>> {
    REMODELING_TEST_MESHES.get_or_init(|| Mutex::new(BTreeMap::new()))
}

#[cfg(test)]
pub fn mint_and_stash_mesh(mesh: MeshData) -> RemodelingMeshChild {
    let bytes = pack::to_json_string(&mesh).into_bytes();
    let child_id = mesh_digest_bytes([bytes.as_slice()]);
    remodeling_test_meshes().lock().expect("remodeling test mesh lock").insert(child_id.clone(), mesh);
    mesh_child_handle(child_id, "remodeling-mesh".into())
}

#[cfg(test)]
pub fn remodeling_mesh_workspace(handle: &RemodelingMeshChild) -> Option<MeshData> {
    if handle.child_id == REMODELING_EMPTY_MESH_CHILD_ID {
        return Some(MeshData::default());
    }
    if handle.child_id == REMODELING_BOX_MESH_CHILD_ID {
        return Some(semio_framework::mesh_from_kind("box"));
    }
    remodeling_test_meshes().lock().expect("remodeling test mesh lock").get(&handle.child_id).cloned()
}

pub fn durable_staged_remodeling_mesh(staging_id: &str) -> Option<RemodelingDurableArtifact> {
    let blobs = remodeling_mesh_blobs().lock().expect("remodeling mesh blob lock");
    let blob = blobs.get(staging_id)?;
    Some(RemodelingDurableArtifact {
        kind: "mesh".into(),
        mime: None,
        width: 0,
        height: 0,
        chunks: (0..u64::try_from(blob.chunks.len()).ok()?).map(|index| base64_codec::base64_standard_encode(blob.chunks.get(&index).expect("contiguous staged mesh"))).collect(),
    })
}

#[cfg(test)]
pub fn forget_remodeling_mesh_content_for_test(content_id: &str, staging_id: &str) {
    remodeling_mesh_blobs().lock().expect("remodeling mesh blob lock").remove(staging_id);
    remodeling_test_meshes().lock().expect("remodeling test mesh lock").remove(content_id);
}

#[cfg(test)]
pub fn forget_all_remodeling_content_for_test() {
    remodeling_asset_content().lock().expect("remodeling asset content lock").clear();
    remodeling_mesh_blobs().lock().expect("remodeling mesh blob lock").clear();
    remodeling_test_meshes().lock().expect("remodeling test mesh lock").clear();
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

    pub fn to_f32_vec_from(&self, store: &RemodelingDurableArtifactStore) -> Vec<f32> {
        let Some((content_id, _, chunk_count)) = remodeling_asset_content_handle_parts(&self.0) else { return self.to_f32_vec() };
        let Some(artifact) = store.get(content_id).filter(|artifact| artifact.kind == "sparse" && u64::try_from(artifact.chunks.len()).ok() == Some(chunk_count)) else { return Vec::new() };
        let mut values = Vec::new();
        for encoded in &artifact.chunks {
            let Some(bytes) = decode_remodeling_durable_chunk(encoded) else { return Vec::new() };
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
    fn shape() -> dsl::Shape {
        dsl::Shape::Text
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Text(self.0.clone())
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Text(s) => Ok(Self(s.clone())),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
}

/// 🌉️ Same reasoning as `PackedF32`'s impl above.
impl dsl::DslField for PackedU8 {
    fn shape() -> dsl::Shape {
        dsl::Shape::Text
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Text(self.0.clone())
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Text(s) => Ok(Self(s.clone())),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
}
//#endregion 🔖️Packed

//#region 🔖️Domain
/// 🖼️ One embedded pixel asset (video frame, ortho tile, texture) referenced by id from
/// `RemodelingSnapshot::assets`, `MediaStream.frames`, `RemodelingMesh.texture_asset_id`, or
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

/// 🎞️ Codec a `VideoSource` was demuxed from — a plain mirror of `remodeling_video::VideoCodec` without
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
/// image-sequence import) — a lightweight mirror of `remodeling_video::{Mp4Info, AviInfo}`, populated
/// once at import time from `remodeling_video::probe`. "Video input = image sequence with timestamps":
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
/// `RemodelingSnapshot::assets`. Multiple cameras/angles are multiple streams, joined by `camera_id`.
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

/// 🎯️ Per-camera intrinsics/distortion, a plain-JSON mirror of `remodeling_camera::{Intrinsics,
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

/// 🎯️ One rig member's pose relative to the rig origin — a plain mirror of `remodeling_camera`'s
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

/// 🎯️ Per-camera intrinsics/distortion plus rig extrinsics, refined by `remodeling_camera`/`remodeling_sfm`.
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

/// 📍️ A surveyed ground-control point used by `remodeling_geo` to georeference the reconstruction.
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

/// ⏭️ Frame sampling/decode limits `remodeling_engine` applies before feature extraction. `min_sharpness`
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

/// 🧊️ UI-facing meshing knobs `remodeling_engine` translates into `remodeling_mesh`'s own internal
/// `MeshParams`/`TsdfVolume` construction args (this document does not depend on `remodeling_mesh`, so
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

/// ⚙️ Full reconstruction parameter set, one sub-struct per pipeline stage — `remodeling_engine` reads
/// these directly to configure `remodeling_image`/`remodeling_video`/`remodeling_camera`/`remodeling_feature`/
/// `remodeling_sfm`/`remodeling_dense`/`remodeling_mesh`/`remodeling_motion`/`remodeling_geo` without this crate
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

/// 🚦️ Mirrors `remodeling_engine`'s pipeline lifecycle so the document can render progress without
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
/// a `remodeling-native` service that was never implemented) has been removed entirely — there is no
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

/// ✅️ A plain-JSON mirror of `remodeling_mesh::WatertightReport`'s summary fields (all scalars — the
/// report itself carries no array data, so this is a snapshot only in the sense of avoiding a hard
/// dependency on `remodeling_mesh`, not in the sense of trimming size).
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
/// CHILD handle (`RemodelingMeshChild`, `🧩️Composition` region above), never embedded `MeshData` —
/// reconstructed geometry resolves through a bounded durable chunk handle; the empty and box
/// placeholders are typed constants that require no process cache.
/// `source`/`texture_asset_id`/`watertight` are genuinely NOT part of the composed mesh's own content
/// (they describe THIS document's relationship to the mesh — provenance, a separate asset reference, a
/// derived QC summary — not geometry), so they stay sibling fields here rather than folding into the
/// child, matching `puzzle`'s own `*Extra`-sibling precedent for content a composed subset's shape
/// can't represent. Always present (never `Option`) so the 3D view always has something to render —
/// `default_remodeling_scene()` seeds it with a placeholder box.
///
/// `ArtifactChild<S>: dsl::DslField` is now real (`🏪️store/🦀️.rs:523`) so this struct keeps
/// its plain `#[derive(dsl::DslRecord)]` instead of hand-rolling — the former `🔖️MeshBridge` region
/// (a `MeshDataTwin` buffer-by-buffer bridge, needed only because `MeshData` is foreign and had no
/// `DslField` impl reachable from this crate) is gone entirely: every field left on this struct now has
/// a real `DslField` impl on its own.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct RemodelingMesh {
    #[dsl(block)]
    #[value(default = "empty_remodeling_mesh_handle")]
    pub mesh: RemodelingMeshChild,
    pub source: MeshSource,
    pub texture_asset_id: Option<String>,
    #[dsl(block)]
    pub watertight: Option<WatertightReportSnapshot>,
}

impl Default for RemodelingMesh {
    fn default() -> Self {
        Self { mesh: empty_remodeling_mesh_handle(), source: MeshSource::default(), texture_asset_id: None, watertight: None }
    }
}

//#region 🔖️MeshBridge
/// 🌉️ `Box<T>` is a `#[fundamental]` std type, so implementing the foreign `dsl::DslField` trait for
/// `Box<RemodelingMesh>` (a local type parameter) here is coherence-legal — needed because
/// `RemodelingMutation::ReplaceMeshResult` carries `mesh: Box<RemodelingMesh>` (boxed only to shrink the
/// enum's overall size; `RemodelingMesh` itself is a plain record, not a `DslEnum`, so the derive's
/// `#[dsl(statements)] Box<T>` "exactly-one-tagged-value" idiom doesn't apply — this is the ordinary
/// boxed-scalar case instead). Delegates to `RemodelingMesh`'s own (now derive-generated) `DslField` impl.
impl dsl::DslField for Box<RemodelingMesh> {
    fn shape() -> dsl::Shape {
        <RemodelingMesh as dsl::DslField>::shape()
    }
    fn to_value(&self) -> dsl::FieldValue {
        <RemodelingMesh as dsl::DslField>::to_value(self.as_ref())
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        <RemodelingMesh as dsl::DslField>::from_value(value).map(Box::new)
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
/// ground, 6 building, …) — `remodeling_dense::PointClass` is a bespoke enum without numeric LAS
/// discriminants, so `remodeling_engine` maps it to LAS codes when it distills this snapshot.
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

/// 🏃️ A distilled summary of one `remodeling_motion` track — full per-frame keyframe paths
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

/// ✅️ A plain-JSON mirror of the QC-relevant fields of `remodeling_geo::QualityReport`, plus the
/// watertight snapshot (mirroring `QualityReport.watertight: Option<WatertightReport>`) and a few
/// cheap scalar summaries (`remodeling_engine` computes these once at the end of a run; the underlying
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
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct ReconstructionResults {
    #[dsl(block)]
    pub sparse: Option<SparseCloud>,
    #[dsl(block)]
    pub dense: Option<DenseCloud>,
    #[dsl(block)]
    pub mesh: RemodelingMesh,
    #[dsl(block)]
    pub trajectory: Option<CameraTrajectory>,
    #[dsl(table)]
    pub tracks: Vec<MotionTrackSummary>,
    #[dsl(block)]
    pub geo: Option<GeoProducts>,
    #[dsl(block)]
    pub qc: Option<QcReportSnapshot>,
}

/// 📸️ Persisted remodeling snapshot — re-exported from `📸️snapshot/🧬️schema`.
pub use crate::schema::snapshot::RemodelingSnapshot;

/// 🌱️ An empty scene seeded with a placeholder box mesh, so the 3D editor/preview always has
/// something to render before any media has been imported/reconstructed.
pub fn default_remodeling_scene() -> RemodelingSnapshot {
    RemodelingSnapshot {
        schema: REMODELING_DOCUMENT_SCHEMA.into(),
        id: "remodeling".into(),
        streams: Vec::new(),
        assets: BTreeMap::new(),
        durable_artifacts: RemodelingDurableArtifactStore::new(),
        calibration: CalibrationState::default(),
        params: ReconstructionParams::default(),
        gcps: Vec::new(),
        job: ReconstructionJob::default(),
        results: ReconstructionResults { mesh: RemodelingMesh { mesh: placeholder_remodeling_mesh_handle(), source: MeshSource::Placeholder, ..RemodelingMesh::default() }, ..ReconstructionResults::default() },
    }
}
//#endregion 🔖️Domain

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "🏅️standards/🔖️1/🦀️.rs"]
                mod standard_root;
                pub use standard_root::*;

                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                        mod subset_root;
                        pub use subset_root::*;

                        #[path = "."]
                        pub mod schema {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            // 🧮️ 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3d:
                            // Rust-only compute-internals, mirroring the `✳️table/🧬️schema/📋️tabular-internals`
                            // and `🧊️brep/🧬️schema/⚙️engine` precedent — moved wholesale from `🧮️math`, sole
                            // repo-wide consumer verified to be this crate. Crate-root aliases (`crate::algebra`,
                            // `crate::optimize`, `crate::lie`, `crate::signal`, `crate::spatial`, below in this
                            // file) let the moved files' own `crate::algebra::` references and the app-engine
                            // consumer files (which used to say `math::algebra::`) resolve unchanged.
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/➕️algebra-internals/🦀️.rs"]
                            pub mod algebra_internals;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔷️lie-internals/🦀️.rs"]
                            pub mod lie_internals;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎯️optimize-internals/🦀️.rs"]
                            pub mod optimize_internals;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📶️signal-internals/🦀️.rs"]
                            pub mod signal_internals;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗺️spatial-internals/🦀️.rs"]
                            pub mod spatial_internals;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod bounds {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod relative_pose {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔄relative-pose/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod create_stream {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🧪️tests/🎥️adds-stream-c-458900/🦀️.rs"]
                                    mod tests_adds_stream_c_458900;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🧪️tests/🛰️adds-a-third-61fb5d/🦀️.rs"]
                                    mod tests_adds_a_third_61fb5d;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🧪️tests/🎞️adds-an-unbound-2b2373/🦀️.rs"]
                                    mod tests_adds_an_unbound_2b2373;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🧪️tests/🔂️rejects-a-6b58da/🦀️.rs"]
                                    mod tests_rejects_a_6b58da;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🧪️tests/👻️rejects-a-stream-aac5c2/🦀️.rs"]
                                    mod tests_rejects_a_stream_aac5c2;
                                }
                                #[path = "."]
                                pub mod delete_stream {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🧪️tests/🚫️refuses-to-3c20ff/🦀️.rs"]
                                    mod tests_refuses_to_3c20ff;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🧪️tests/⏮️removes-the-first-c0fc2a/🦀️.rs"]
                                    mod tests_removes_the_first_c0fc2a;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🧪️tests/🪓removes-the-spare-556d1d/🦀️.rs"]
                                    mod tests_removes_the_spare_556d1d;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🧪️tests/⛓️refuses-to-remove-422a37/🦀️.rs"]
                                    mod tests_refuses_to_remove_422a37;
                                }
                                #[path = "."]
                                pub mod change_stream_sync {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🧪️tests/⏱️shifts-stream-a-5b442c/🦀️.rs"]
                                    mod tests_shifts_stream_a_5b442c;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🧪️tests/🚫️refuses-to-8095d3/🦀️.rs"]
                                    mod tests_refuses_to_8095d3;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🧪️tests/🔁️warns-that-the-a98c13/🦀️.rs"]
                                    mod tests_warns_that_the_a98c13;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🧪️tests/⏱️retimes-the-50dd75/🦀️.rs"]
                                    mod tests_retimes_the_50dd75;
                                }
                                #[path = "."]
                                pub mod add_stream_frame {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🧪️tests/🎞️appends-a-third-8ac259/🦀️.rs"]
                                    mod tests_appends_a_third_8ac259;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🧪️tests/🚫️refuses-to-c93e98/🦀️.rs"]
                                    mod tests_refuses_to_c93e98;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🧪️tests/🔁️warns-that-the-1e8abe/🦀️.rs"]
                                    mod tests_warns_that_the_1e8abe;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🧪️tests/🎞️appends-an-0c2164/🦀️.rs"]
                                    mod tests_appends_an_0c2164;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🧪️tests/🎬️refuses-a-frame-81beea/🦀️.rs"]
                                    mod tests_refuses_a_frame_81beea;
                                }
                                #[path = "."]
                                pub mod remove_stream_frame {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🧪️tests/🚫️removes-the-last-304bdf/🦀️.rs"]
                                    mod tests_removes_the_last_304bdf;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🧪️tests/⏮️drops-the-first-d98a0f/🦀️.rs"]
                                    mod tests_drops_the_first_d98a0f;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🧪️tests/🚫️refuses-a-frame-e7c374/🦀️.rs"]
                                    mod tests_refuses_a_frame_e7c374;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🧪️tests/✂️drops-the-middle-2d6d53/🦀️.rs"]
                                    mod tests_drops_the_middle_2d6d53;
                                }
                                #[path = "."]
                                pub mod replace_stream_source {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🧪️tests/🧹️clears-the-video-143f2b/🦀️.rs"]
                                    mod tests_clears_the_video_143f2b;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🧪️tests/📼️attaches-a-607df8/🦀️.rs"]
                                    mod tests_attaches_a_607df8;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🧪️tests/🚫️refuses-to-f7f40d/🦀️.rs"]
                                    mod tests_refuses_to_f7f40d;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🧪️tests/🎥️reingests-the-311c32/🦀️.rs"]
                                    mod tests_reingests_the_311c32;
                                }
                                #[path = "."]
                                pub mod create_asset {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🧪️tests/🖼️stores-a-new-d56283/🦀️.rs"]
                                    mod tests_stores_a_new_d56283;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🧪️tests/🖼️stores-an-9f39e1/🦀️.rs"]
                                    mod tests_stores_an_9f39e1;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🧪️tests/🚫️refuses-an-asset-cb0d4b/🦀️.rs"]
                                    mod tests_refuses_an_asset_cb0d4b;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🧪️tests/♻️overwrites-an-a34b9d/🦀️.rs"]
                                    mod tests_overwrites_an_a34b9d;
                                }
                                #[path = "."]
                                pub mod delete_asset {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🧪️tests/🚫️refuses-to-c4563a/🦀️.rs"]
                                    mod tests_refuses_to_c4563a;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🧪️tests/🧹️drops-the-spare-c6ffb6/🦀️.rs"]
                                    mod tests_drops_the_spare_c6ffb6;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🧪️tests/🗺️refuses-to-5c6f74/🦀️.rs"]
                                    mod tests_refuses_to_5c6f74;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🧪️tests/🗑️sweeps-the-503b27/🦀️.rs"]
                                    mod tests_sweeps_the_503b27;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🧪️tests/🖼️refuses-to-f9541f/🦀️.rs"]
                                    mod tests_refuses_to_f9541f;
                                }
                                #[path = "."]
                                pub mod create_camera_calibration {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/🧪️tests/📷️adds-the-cam-c-82c8fb/🦀️.rs"]
                                    mod tests_adds_the_cam_c_82c8fb;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/🧪️tests/🚫️refuses-a-camera-e92a02/🦀️.rs"]
                                    mod tests_refuses_a_camera_e92a02;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/🧪️tests/📷️adds-a-fourth-97e912/🦀️.rs"]
                                    mod tests_adds_a_fourth_97e912;
                                }
                                #[path = "."]
                                pub mod update_camera_calibration {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🧪️tests/🔍️refines-the-cam-0eaef0/🦀️.rs"]
                                    mod tests_refines_the_cam_0eaef0;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🧪️tests/🚫️refuses-to-b60a39/🦀️.rs"]
                                    mod tests_refuses_to_b60a39;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🧪️tests/🔁️warns-that-the-697b4f/🦀️.rs"]
                                    mod tests_warns_that_the_697b4f;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🧪️tests/🔍️refines-the-9fd25a/🦀️.rs"]
                                    mod tests_refines_the_9fd25a;
                                }
                                #[path = "."]
                                pub mod delete_camera_calibration {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🧪️tests/🚫️removes-the-cam-f90b89/🦀️.rs"]
                                    mod tests_removes_the_cam_f90b89;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🧪️tests/🚫️refuses-to-73655a/🦀️.rs"]
                                    mod tests_refuses_to_73655a;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🧪️tests/🚫️removes-the-40cba4/🦀️.rs"]
                                    mod tests_removes_the_40cba4;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🧪️tests/⛓️refuses-to-remove-3c8f32/🦀️.rs"]
                                    mod tests_refuses_to_remove_3c8f32;
                                }
                                #[path = "."]
                                pub mod create_rig_extrinsic {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🧪️tests/🔗️adds-a-rig-2df5df/🦀️.rs"]
                                    mod tests_adds_a_rig_2df5df;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🧪️tests/🚫️refuses-a-second-95e04d/🦀️.rs"]
                                    mod tests_refuses_a_second_95e04d;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🧪️tests/🔗️places-the-0d0b8d/🦀️.rs"]
                                    mod tests_places_the_0d0b8d;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🧪️tests/🚫️refuses-a-rig-cb71ba/🦀️.rs"]
                                    mod tests_refuses_a_rig_cb71ba;
                                }
                                #[path = "."]
                                pub mod delete_rig_extrinsic {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🧪️tests/✂️drops-the-cam-a-a1f8a2/🦀️.rs"]
                                    mod tests_drops_the_cam_a_a1f8a2;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🧪️tests/⏮️unplaces-the-f5b35e/🦀️.rs"]
                                    mod tests_unplaces_the_f5b35e;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🧪️tests/🚫️refuses-to-1805df/🦀️.rs"]
                                    mod tests_refuses_to_1805df;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🧪️tests/✂️unplaces-the-a39356/🦀️.rs"]
                                    mod tests_unplaces_the_a39356;
                                }
                                #[path = "."]
                                pub mod update_rig_extrinsic {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🧪️tests/📍️retunes-the-cam-4ca5a2/🦀️.rs"]
                                    mod tests_retunes_the_cam_4ca5a2;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🧪️tests/🚫️refuses-to-2cfb53/🦀️.rs"]
                                    mod tests_refuses_to_2cfb53;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🧪️tests/🔁️warns-that-the-89422a/🦀️.rs"]
                                    mod tests_warns_that_the_89422a;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🧪️tests/📍️retunes-the-675f52/🦀️.rs"]
                                    mod tests_retunes_the_675f52;
                                }
                                #[path = "."]
                                pub mod create_gcp {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🧪️tests/📍️adds-gcp-tower-d71a54/🦀️.rs"]
                                    mod tests_adds_gcp_tower_d71a54;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🧪️tests/🚫️refuses-a-19c1ab/🦀️.rs"]
                                    mod tests_refuses_a_19c1ab;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🧪️tests/📍️adds-a-quay-7569de/🦀️.rs"]
                                    mod tests_adds_a_quay_7569de;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🧪️tests/🕳️adds-a-control-298de4/🦀️.rs"]
                                    mod tests_adds_a_control_298de4;
                                }
                                #[path = "."]
                                pub mod delete_gcp {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🧪️tests/🚫️removes-gcp-209b7d/🦀️.rs"]
                                    mod tests_removes_gcp_209b7d;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🧪️tests/🚫️refuses-to-12366b/🦀️.rs"]
                                    mod tests_refuses_to_12366b;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🧪️tests/🚮removes-the-south-42cd9e/🦀️.rs"]
                                    mod tests_removes_the_south_42cd9e;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🧪️tests/🕳️removes-an-8f3868/🦀️.rs"]
                                    mod tests_removes_an_8f3868;
                                }
                                #[path = "."]
                                pub mod add_gcp_observation {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🧪️tests/🔎️adds-the-first-05b1b5/🦀️.rs"]
                                    mod tests_adds_the_first_05b1b5;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🧪️tests/🚫️refuses-to-pick-3c0570/🦀️.rs"]
                                    mod tests_refuses_to_pick_3c0570;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🧪️tests/🔁️warns-that-this-dca661/🦀️.rs"]
                                    mod tests_warns_that_this_dca661;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🧪️tests/🔎️picks-the-south-eb0c4d/🦀️.rs"]
                                    mod tests_picks_the_south_eb0c4d;
                                }
                                #[path = "."]
                                pub mod remove_gcp_observation {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🧪️tests/🚫️removes-the-only-f82e64/🦀️.rs"]
                                    mod tests_removes_the_only_f82e64;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🧪️tests/⏮️drops-the-first-9ebf0b/🦀️.rs"]
                                    mod tests_drops_the_first_9ebf0b;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🧪️tests/🚫️refuses-an-109cf1/🦀️.rs"]
                                    mod tests_refuses_an_109cf1;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🧪️tests/🚷drops-the-middle-282fb7/🦀️.rs"]
                                    mod tests_drops_the_middle_282fb7;
                                }
                                #[path = "."]
                                pub mod update_ingest_params {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🧪️tests/🔍️tightens-the-499c47/🦀️.rs"]
                                    mod tests_tightens_the_499c47;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🧪️tests/🚫️refuses-an-59752a/🦀️.rs"]
                                    mod tests_refuses_an_59752a;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🧪️tests/🔁️warns-that-the-8eaad8/🦀️.rs"]
                                    mod tests_warns_that_the_8eaad8;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🧪️tests/📥️widens-the-73f33e/🦀️.rs"]
                                    mod tests_widens_the_73f33e;
                                }
                                #[path = "."]
                                pub mod update_feature_params {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🧪️tests/🔎️switches-the-423de9/🦀️.rs"]
                                    mod tests_switches_the_423de9;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🧪️tests/🚫️refuses-a-d82e38/🦀️.rs"]
                                    mod tests_refuses_a_d82e38;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🧪️tests/🔁️warns-that-the-b6b7dc/🦀️.rs"]
                                    mod tests_warns_that_the_b6b7dc;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🧪️tests/🌟️moves-the-3621f6/🦀️.rs"]
                                    mod tests_moves_the_3621f6;
                                }
                                #[path = "."]
                                pub mod update_match_params {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🧪️tests/🌳️switches-the-652d03/🦀️.rs"]
                                    mod tests_switches_the_652d03;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🧪️tests/🚫️refuses-a-ratio-65dcb9/🦀️.rs"]
                                    mod tests_refuses_a_ratio_65dcb9;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🧪️tests/🔁️warns-that-the-414aae/🦀️.rs"]
                                    mod tests_warns_that_the_414aae;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🧪️tests/🌳️switches-to-a-kd-d6fa4b/🦀️.rs"]
                                    mod tests_switches_to_a_kd_d6fa4b;
                                }
                                #[path = "."]
                                pub mod update_sfm_params {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/🧪️tests/🎯️switches-the-7f0371/🦀️.rs"]
                                    mod tests_switches_the_7f0371;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/🧪️tests/🔁️warns-that-the-79a92a/🦀️.rs"]
                                    mod tests_warns_that_the_79a92a;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/🧪️tests/🎯️tightens-the-850036/🦀️.rs"]
                                    mod tests_tightens_the_850036;
                                }
                                #[path = "."]
                                pub mod update_dense_params {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/🧪️tests/🔬️raises-the-dense-ddb263/🦀️.rs"]
                                    mod tests_raises_the_dense_ddb263;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/🧪️tests/🔁️warns-that-the-4e65c8/🦀️.rs"]
                                    mod tests_warns_that_the_4e65c8;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/🧪️tests/🧊️sharpens-the-25044c/🦀️.rs"]
                                    mod tests_sharpens_the_25044c;
                                }
                                #[path = "."]
                                pub mod update_mesh_params {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/🧪️tests/🔳️doubles-the-c245d5/🦀️.rs"]
                                    mod tests_doubles_the_c245d5;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/🧪️tests/🔁️warns-that-the-887e9f/🦀️.rs"]
                                    mod tests_warns_that_the_887e9f;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/🧪️tests/🔳️halves-the-voxel-21b53d/🦀️.rs"]
                                    mod tests_halves_the_voxel_21b53d;
                                }
                                #[path = "."]
                                pub mod update_motion_params {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/🧪️tests/🏃️enables-motion-2444a3/🦀️.rs"]
                                    mod tests_enables_motion_2444a3;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/🧪️tests/🔁️warns-that-the-83ff67/🦀️.rs"]
                                    mod tests_warns_that_the_83ff67;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/🧪️tests/🏃️triples-the-4bb69f/🦀️.rs"]
                                    mod tests_triples_the_4bb69f;
                                }
                                #[path = "."]
                                pub mod update_geo_params {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🧪️tests/🌐️enables-georefere-18a68a/🦀️.rs"]
                                    mod tests_enables_georefere_18a68a;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🧪️tests/🚫️refuses-a-zero-fa917f/🦀️.rs"]
                                    mod tests_refuses_a_zero_fa917f;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🧪️tests/🔁️warns-that-the-efc6e8/🦀️.rs"]
                                    mod tests_warns_that_the_efc6e8;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🧪️tests/🌐️halves-the-002a17/🦀️.rs"]
                                    mod tests_halves_the_002a17;
                                }
                                #[path = "."]
                                pub mod replace_job {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🧪️tests/🎨️advances-the-job-c1e878/🦀️.rs"]
                                    mod tests_advances_the_job_c1e878;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🧪️tests/🔁️warns-that-the-bdf2e9/🦀️.rs"]
                                    mod tests_warns_that_the_bdf2e9;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🧪️tests/🎨️advances-the-555298/🦀️.rs"]
                                    mod tests_advances_the_555298;
                                }
                                #[path = "."]
                                pub mod commit_reconstruction {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁commit-reconstruction/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁commit-reconstruction/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁commit-reconstruction/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁commit-reconstruction/🧪️tests/🖼️rejects-an-e9fa51/🦀️.rs"]
                                    mod tests_rejects_an_e9fa51;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁commit-reconstruction/🧪️tests/🕸️rejects-an-5d3a60/🦀️.rs"]
                                    mod tests_rejects_an_5d3a60;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁commit-reconstruction/🧪️tests/⭐️rejects-an-2e5568/🦀️.rs"]
                                    mod tests_rejects_an_2e5568;
                                }
                                #[path = "."]
                                pub mod replace_sparse {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/🧪️tests/✨️swaps-in-an-6d9ae4/🦀️.rs"]
                                    mod tests_swaps_in_an_6d9ae4;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/🧪️tests/🔁️warns-that-the-56a3a9/🦀️.rs"]
                                    mod tests_warns_that_the_56a3a9;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/🧪️tests/✨️swaps-in-a-re-3cfa6d/🦀️.rs"]
                                    mod tests_swaps_in_a_re_3cfa6d;
                                }
                                #[path = "."]
                                pub mod replace_dense {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/🧪️tests/☁️swaps-in-a-two-c688db/🦀️.rs"]
                                    mod tests_swaps_in_a_two_c688db;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/🧪️tests/🔁️warns-that-the-675b6e/🦀️.rs"]
                                    mod tests_warns_that_the_675b6e;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/🧪️tests/☁️swaps-in-a-denser-4174e1/🦀️.rs"]
                                    mod tests_swaps_in_a_denser_4174e1;
                                }
                                #[path = "."]
                                pub mod replace_mesh_result {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🧪️tests/🕸️swaps-in-an-f23e71/🦀️.rs"]
                                    mod tests_swaps_in_an_f23e71;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🧪️tests/🔁️warns-that-the-b39bab/🦀️.rs"]
                                    mod tests_warns_that_the_b39bab;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🧪️tests/🕸️swaps-the-c43d9c/🦀️.rs"]
                                    mod tests_swaps_the_c43d9c;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🧪️tests/🚫️refuses-a-48f3a6/🦀️.rs"]
                                    mod tests_refuses_a_48f3a6;
                                }
                                #[path = "."]
                                pub mod replace_trajectory {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🧪️tests/🧹️clears-the-d2f81a/🦀️.rs"]
                                    mod tests_clears_the_d2f81a;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🧪️tests/🚫️refuses-to-clear-524569/🦀️.rs"]
                                    mod tests_refuses_to_clear_524569;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🧪️tests/🛣️swaps-in-a-three-49b17f/🦀️.rs"]
                                    mod tests_swaps_in_a_three_49b17f;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🧪️tests/🕳️drops-the-6436a8/🦀️.rs"]
                                    mod tests_drops_the_6436a8;
                                }
                                #[path = "."]
                                pub mod replace_tracks {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🧪️tests/⏸️replaces-the-d40c68/🦀️.rs"]
                                    mod tests_replaces_the_d40c68;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🧪️tests/🕳️clears-every-760061/🦀️.rs"]
                                    mod tests_clears_every_760061;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🧪️tests/🏃️swaps-in-two-166265/🦀️.rs"]
                                    mod tests_swaps_in_two_166265;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🧪️tests/🔁️warns-that-the-8dbf82/🦀️.rs"]
                                    mod tests_warns_that_the_8dbf82;
                                }
                                #[path = "."]
                                pub mod replace_geo_products {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🧪️tests/🗺️adds-the-dtm-and-64d5bb/🦀️.rs"]
                                    mod tests_adds_the_dtm_and_64d5bb;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🧪️tests/🚫️refuses-to-clear-b8c54a/🦀️.rs"]
                                    mod tests_refuses_to_clear_b8c54a;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🧪️tests/🧹️clears-the-geo-f4886e/🦀️.rs"]
                                    mod tests_clears_the_geo_f4886e;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🧪️tests/🗺️records-a-dtm-6e132a/🦀️.rs"]
                                    mod tests_records_a_dtm_6e132a;
                                }
                                #[path = "."]
                                pub mod replace_qc {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🧪️tests/📋️records-a-qc-f5caf4/🦀️.rs"]
                                    mod tests_records_a_qc_f5caf4;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🧪️tests/🚫️refuses-to-clear-30cbb5/🦀️.rs"]
                                    mod tests_refuses_to_clear_30cbb5;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🧪️tests/🧹️clears-the-qc-1d2249/🦀️.rs"]
                                    mod tests_clears_the_qc_1d2249;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🧪️tests/✅️files-a-qc-report-64d222/🦀️.rs"]
                                    mod tests_files_a_qc_report_64d222;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod las {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod ply {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧱️ply/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod stl {
                                            #[path = "."]
                                            pub mod v_ascii {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod gltf {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod obj {
                                            #[path = "."]
                                            pub mod v3_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗿️obj/🔖️3.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod las {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod ply {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧱️ply/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod stl {
                                            #[path = "."]
                                            pub mod v_ascii {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod gltf {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod obj {
                                            #[path = "."]
                                            pub mod v3_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗿️obj/🔖️3.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        pub use crate::standards::v1::subsets::any::schema;
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op {
            pub use crate::standards::v1::subsets::any::schema::mutations::text::*;
        }
        pub mod document_dsl {
            pub use crate::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::schema::diff::text::*;
            }
        }
        pub mod mutations {
            pub use crate::standards::v1::subsets::any::schema::mutations::*;
        }
        pub mod snapshot {
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::schema::snapshot::text::*;
            }
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
                mod tests;
            }
            #[path = "."]
            pub mod synthetic_orbit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🧪️tests/🧩️example/🦀️.rs"]
                mod tests;
            }
        }

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod remodeling {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod engine {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📷️camera/🦀️.rs"]
            pub mod camera;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🌫️dense/🦀️.rs"]
            pub mod dense;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🌟️feature/🦀️.rs"]
            pub mod feature;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗺️geo/🦀️.rs"]
            pub mod geo;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖼️images/🦀️.rs"]
            pub mod images;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🥽️mesh/🦀️.rs"]
            pub mod mesh;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️motion/🦀️.rs"]
            pub mod motion;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🦀️.rs"]
            pub mod reconstruction;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📸️sfm/🦀️.rs"]
            pub mod sfm;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🦀️.rs"]
            pub mod video;
        }

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🦀️.rs"]
        pub mod examples;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧿️add-gcp/🦀️.rs"]
            pub mod add_gcp;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱️add-stream/🦀️.rs"]
            pub mod add_stream;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏩️advance-reconstruction/🦀️.rs"]
            pub mod advance_reconstruction;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️calibrate-cameras/🦀️.rs"]
            pub mod calibrate_cameras;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛑️cancel-reconstruction/🦀️.rs"]
            pub mod cancel_reconstruction;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/☁️clear-dense/🦀️.rs"]
            pub mod clear_dense;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗾️clear-geo-products/🦀️.rs"]
            pub mod clear_geo_products;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧱️clear-mesh-result/🦀️.rs"]
            pub mod clear_mesh_result;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️clear-result/🦀️.rs"]
            pub mod clear_result;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⭐️clear-sparse/🦀️.rs"]
            pub mod clear_sparse;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚂️clear-tracks/🦀️.rs"]
            pub mod clear_tracks;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛠️edit-calibration/🦀️.rs"]
            pub mod edit_calibration;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧾️export-qc-report/🦀️.rs"]
            pub mod export_qc_report;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖼️import-frame-payload/🦀️.rs"]
            pub mod import_frame_payload;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎞️import-frames/🦀️.rs"]
            pub mod import_frames;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️import-video/🦀️.rs"]
            pub mod import_video;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💽️import-video-bytes-payload/🦀️.rs"]
            pub mod import_video_bytes_payload;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️import-video-done/🦀️.rs"]
            pub mod import_video_done;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📼️import-video-frame-payload/🦀️.rs"]
            pub mod import_video_frame_payload;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔎️place-gcp-observation/🦀️.rs"]
            pub mod place_gcp_observation;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚮️remove-gcp/🦀️.rs"]
            pub mod remove_gcp;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪓️remove-stream/🦀️.rs"]
            pub mod remove_stream;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/♻️reset-placeholder-mesh/🦀️.rs"]
            pub mod reset_placeholder_mesh;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔁️retry-stage/🦀️.rs"]
            pub mod retry_stage;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️run-reconstruction/🦀️.rs"]
            pub mod run_reconstruction;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/▶️run-stage/🦀️.rs"]
            pub mod run_stage;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📷️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌁️set-dense-params/🦀️.rs"]
            pub mod set_dense_params;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌠️set-feature-params/🦀️.rs"]
            pub mod set_feature_params;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️set-frame-cursor/🦀️.rs"]
            pub mod set_frame_cursor;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌐️set-geo-params/🦀️.rs"]
            pub mod set_geo_params;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🥣️set-ingest-params/🦀️.rs"]
            pub mod set_ingest_params;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👓️set-layer-visibility/🦀️.rs"]
            pub mod set_layer_visibility;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪢️set-match-params/🦀️.rs"]
            pub mod set_match_params;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️set-mesh-params/🦀️.rs"]
            pub mod set_mesh_params;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏎️set-motion-params/🦀️.rs"]
            pub mod set_motion_params;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📊️set-report-table/🦀️.rs"]
            pub mod set_report_table;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️set-sfm-params/🦀️.rs"]
            pub mod set_sfm_params;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️set-stream-sync/🦀️.rs"]
            pub mod set_stream_sync;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod model {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧊️model/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod model {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/☑️options/👁️layers/🦀️.rs"]
                            pub mod layers;
                        }
                    }
                }
            }

            #[path = "."]
            pub mod capture {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📷️capture/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📷️capture/🪟️windows/🖼️frames/🦀️.rs"]
                    pub mod frames;
                }
            }

            #[path = "."]
            pub mod analyze {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔍️analyze/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔍️analyze/🪟️windows/📊️report/🦀️.rs"]
                    pub mod report;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🎯️calibration/🦀️.rs"]
            pub mod calibration;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗂️media/🦀️.rs"]
            pub mod media;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/⚙️parameters/🦀️.rs"]
            pub mod parameters;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/✅️quality/🦀️.rs"]
            pub mod quality;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🧵️results/🦀️.rs"]
            pub mod results;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🏃️tracks/🦀️.rs"]
            pub mod tracks;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod remodeling {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod model {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️model/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}

//#region 📚️Examples
pub use standards::v1::subsets::any::examples;
//#endregion 📚️Examples
