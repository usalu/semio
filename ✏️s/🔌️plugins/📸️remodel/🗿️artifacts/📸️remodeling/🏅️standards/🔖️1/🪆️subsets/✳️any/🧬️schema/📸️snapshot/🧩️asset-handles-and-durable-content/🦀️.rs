//! 🧩️ Asset handles and durable content of the remodeling snapshot: the composed image and mesh child types, the durable artifact store, the asset child handle minting and the content kind, digest and chunk laws. See [`super`].

use super::buffers::ByteBuffer;
use super::media_and_calibration::ImageAsset;
use super::mesh_handles::mesh_from_durable_chunks;
use super::RemodelingSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot;
use semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;

//#region 🧩️Composition
/// 🧩️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (design map §4: "remodeling→C:mesh R:image").
/// Two content-duplication shapes, both verified against real code (not assumed from the one-line
/// design summary):
///
/// 1. **`results.mesh.mesh` (was `MeshData`, the reconstructed/placeholder mesh's flat buffers) is now
///    a composed `s.stdio.semio/v1/mesh` CHILD** (`RemodelingMeshChild = store::os_store::artifact_child::ArtifactChild<
///    SemioMeshSnapshot>`) — exactly `💠️lowpoly`'s own pattern (this ticket's closest precedent for
///    opaque mesh composition), extended with a REAL bidirectional converter
///    (`crate::standards::v1::subsets::any::io::{mesh_data_to_semio_mesh,
///    semio_mesh_to_mesh_data}`, already real — the export path already built the forward direction).
///
/// 2. **`assets: BTreeMap<String, ImageAsset>` (embedded mime+base64 pixel bytes: video frames,
///    baked mesh textures, DSM/DTM/ortho rasters) is now `BTreeMap<String, RemodelingAssetChild>`
///    (`RemodelingAssetChild = store::os_store::artifact_child::ArtifactChild<SemioImageSnapshot>`)** — the design line's "R:image"
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
pub type RemodelingAssetChild = store::os_store::artifact_child::ArtifactChild<SemioImageSnapshot>;

pub type RemodelingMeshChild = store::os_store::artifact_child::ArtifactChild<SemioMeshSnapshot>;

/// 🧩️ Restart-stable content authority. Every literal leaf contains at most 4 KiB; values carry
/// only bounded metadata and content-addressed leaves, never a whole image or mesh object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelingDurableArtifact {
    pub kind: String,
    pub mime: Option<String>,
    pub width: u32,
    pub height: u32,
    pub chunks: Vec<ByteBuffer>,
}

pub type RemodelingDurableArtifactStore = BTreeMap<String, RemodelingDurableArtifact>;

pub const REMODELING_DURABLE_CHUNK_RAW_BYTES: usize = 4_096;

const REMODELING_SPARSE_CONTENT_BYTES: usize = 512 * 3 * 4;

const REMODELING_SPARSE_CONTENT_CHUNKS: usize = 2;

const REMODELING_RASTER_CONTENT_BYTES: usize = 1_114_112;

const REMODELING_RASTER_CONTENT_CHUNKS: usize = 272;

pub(super) const REMODELING_MESH_CONTENT_BYTES: usize = 87_552 + 30;

const REMODELING_MESH_CONTENT_CHUNKS: usize = 30;

pub(super) const REMODELING_EMPTY_MESH_CHILD_ID: &str = "remodeling-mesh-constant-empty";

pub(super) const REMODELING_BOX_MESH_CHILD_ID: &str = "remodeling-mesh-constant-box";

pub(super) const REMODELING_BOUNDED_MESH_VERTICES: usize = 512;

pub(super) const REMODELING_BOUNDED_MESH_TRIANGLES: usize = 512;

//#region 🔖️AssetHandles
fn mint_asset_child_handle(asset_id: &str, child_id: String) -> RemodelingAssetChild {
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: format!("{asset_id}-image"), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}

pub fn committed_remodeling_asset_handle(asset_id: &str, content_id: &str) -> RemodelingAssetChild {
    mint_asset_child_handle(asset_id, content_id.into())
}

/// 🕸️ Deterministic content-addressed CHILD handle for one bounded durable asset.
pub fn image_asset_child_handle(asset_id: &str, asset: &ImageAsset) -> RemodelingAssetChild {
    mint_asset_child_handle(asset_id, store::content_id("remodeling-asset", format!("{}\u{1f}{}", asset.mime, asset.data).as_bytes()))
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
    pub leaves: Vec<Arc<Vec<u8>>>,
    pub byte_len: usize,
}

pub fn remodeling_asset_chunk_source(snapshot: &RemodelingSnapshot, asset_id: &str) -> Option<RemodelingAssetChunkSource> {
    let handle = snapshot.assets.get(asset_id)?;
    let artifact = snapshot.durable_artifacts.get(&handle.child_id).filter(|artifact| artifact.kind == "image")?;
    let mime = artifact.mime.clone()?;
    let mut leaves = Vec::with_capacity(artifact.chunks.len());
    let mut byte_len = 0usize;
    for encoded in &artifact.chunks {
        let chunk = remodeling_durable_chunk(encoded)?;
        byte_len = byte_len.checked_add(chunk.len())?;
        if byte_len > REMODELING_RASTER_CONTENT_BYTES {
            return None;
        }
        leaves.push(Arc::new(chunk.into()));
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
        chunks: bytes.chunks(REMODELING_DURABLE_CHUNK_RAW_BYTES).map(ByteBuffer::from_u8_slice).collect(),
    })
}

//#region 🔖️DurableContent
/// 🏷️ Durable content lanes a reconstruction publishes through `append-content`, each with its exact
/// bounded envelope. The wire spelling is the `RemodelingDurableArtifact.kind` it writes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum RemodelingContentKind {
    #[default]
    Sparse,
    Mesh,
    Image,
}

impl RemodelingContentKind {
    pub const ALL: [Self; 3] = [Self::Sparse, Self::Mesh, Self::Image];

    pub fn wire(self) -> &'static str {
        match self {
            Self::Sparse => "sparse",
            Self::Mesh => "mesh",
            Self::Image => "image",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.wire() == value)
    }

    pub fn max_bytes(self) -> usize {
        match self {
            Self::Sparse => REMODELING_SPARSE_CONTENT_BYTES,
            Self::Mesh => REMODELING_MESH_CONTENT_BYTES,
            Self::Image => REMODELING_RASTER_CONTENT_BYTES,
        }
    }

    pub fn max_chunks(self) -> usize {
        match self {
            Self::Sparse => REMODELING_SPARSE_CONTENT_CHUNKS,
            Self::Mesh => REMODELING_MESH_CONTENT_CHUNKS,
            Self::Image => REMODELING_RASTER_CONTENT_CHUNKS,
        }
    }

    fn digest_prefix(self) -> &'static str {
        match self {
            Self::Mesh => "remodeling-mesh",
            Self::Sparse | Self::Image => "remodeling-asset",
        }
    }
}

/// #️⃣️ Streaming content digest a run folds over every raw leaf it publishes; the resulting id names
/// the durable content, so identical reconstruction output lands on the identical durable entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub struct RemodelingContentDigest {
    pub(super) digest: [u64; 4],
    pub(super) len: u64,
}

impl Default for RemodelingContentDigest {
    fn default() -> Self {
        Self { digest: [0x6c62272e07bb0142, 0x62b821756295c58d, 0x9e3779b185ebca87, 0xc2b2ae3d27d4eb4f], len: 0 }
    }
}

impl RemodelingContentDigest {
    pub fn record(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.len = self.len.wrapping_add(1);
            self.digest[0] = (self.digest[0] ^ u64::from(*byte)).wrapping_mul(0x00000100000001b3);
            self.digest[1] = (self.digest[1] ^ self.digest[0].rotate_left(17) ^ self.len).wrapping_mul(0x9e3779b185ebca87);
            self.digest[2] = (self.digest[2] ^ self.digest[1].rotate_left(29) ^ u64::from(*byte)).wrapping_mul(0xc2b2ae3d27d4eb4f);
            self.digest[3] = (self.digest[3] ^ self.digest[2].rotate_left(41) ^ self.len.rotate_left(7)).wrapping_mul(0x165667b19e3779f9);
        }
    }

    pub fn byte_len(&self) -> u64 {
        self.len
    }

    pub fn parts(&self) -> ([u64; 4], u64) {
        (self.digest, self.len)
    }

    pub fn from_parts(digest: [u64; 4], len: u64) -> Self {
        Self { digest, len }
    }

    pub fn content_id(&self, kind: RemodelingContentKind) -> String {
        format!("{}-{:016x}{:016x}{:016x}{:016x}-{:016x}", kind.digest_prefix(), self.digest[0], self.digest[1], self.digest[2], self.digest[3], self.len)
    }
}

/// 🧱️ Raw bytes of one durable leaf, refusing anything above the 4 KiB leaf envelope.
pub fn remodeling_durable_chunk(chunk: &ByteBuffer) -> Option<&[u8]> {
    (chunk.0.len() <= REMODELING_DURABLE_CHUNK_RAW_BYTES).then_some(chunk.0.as_slice())
}

/// ✅️ Whether `content_id` names complete durable content of `kind` with exactly `chunk_count` leaves
/// inside the kind's envelope; a mesh must additionally resolve inside the 512/512 resolution envelope.
pub fn remodeling_content_is_complete(store: &RemodelingDurableArtifactStore, content_id: &str, kind: RemodelingContentKind, chunk_count: u64) -> bool {
    let Some(artifact) = store.get(content_id).filter(|artifact| artifact.kind == kind.wire()) else { return false };
    if chunk_count == 0 || u64::try_from(artifact.chunks.len()).ok() != Some(chunk_count) {
        return false;
    }
    match kind {
        RemodelingContentKind::Mesh => mesh_from_durable_chunks(&artifact.chunks).is_some(),
        RemodelingContentKind::Sparse | RemodelingContentKind::Image => {
            let mut bytes = 0usize;
            artifact.chunks.iter().all(|encoded| remodeling_durable_chunk(encoded).and_then(|chunk| bytes.checked_add(chunk.len())).is_some_and(|total| {
                bytes = total;
                total <= kind.max_bytes()
            }))
        }
    }
}
//#endregion 🔖️DurableContent
//#endregion 🔖️AssetHandles
//#endregion 🧩️Composition
