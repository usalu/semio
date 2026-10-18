//! 🧬️ Remodeling snapshot schema — artifact-lane fields only.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `assets` carries real
//! `store::os_store::artifact_child::ArtifactChild<SemioImageSnapshot>` handles (composed `s.stdio.semio.image` children, one
//! per asset id — see `🗿️artifacts/📸️remodeling/🦀️.rs`'s `🧩️Composition` region) instead of
//! embedded `ImageAsset` bytes. `ArtifactChild<S>: dsl::DslField` is now real
//! (`🏪️store/🦀️.rs:523`), and `BTreeMap<String, T: DslField>: DslField` already exists
//! generically (`🗣️dsl/🦀️.rs:178`), so this struct's `#[derive(dsl::DslRecord)]` keeps
//! working unmodified for `assets` — no hand-rolled codec needed here (unlike `🖨️raster`'s own
//! identical migration, authored before this generic `BTreeMap` impl was confirmed reachable).
//! `child_slots()` is honestly EMPTY for `assets` regardless: the derive's `#[child(kind=...)]`
//! mechanism only recognizes a bare `ArtifactChild<T>`/`Vec<ArtifactChild<T>>` field, not a
//! `BTreeMap` value — kept as `BTreeMap<String, ArtifactChild<S>>` (not reshaped to a `Vec`) to
//! preserve the same id-keyed addressing every existing `create-asset`/`delete-asset` mutation and
//! `MediaStream.frames`/`RemodelingMesh.texture_asset_id`/`GeoProducts.*_asset_id` lookup already
//! assumes — the type/mutation/persistence layer is fully real, only the derive-generated SCHEMA
//! INTROSPECTION table is incomplete for this one field (matches `🖨️raster`'s/`💠️lowpoly`'s own
//! already-accepted gap for the identical shape).

use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

//#region 🔖️Snapshot

/// 📸️ Persisted remodeling document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[dsl(extension = "remodeling")]
#[artifact_schema(id = "s.remodel.remodeling")]
pub struct RemodelingSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub streams: Vec<MediaStream>,
    #[value(default)]
    #[state(artifact)]
    pub assets: BTreeMap<String, RemodelingAssetChild>,
    #[value(default)]
    #[state(artifact)]
    pub durable_artifacts: RemodelingDurableArtifactStore,
    #[value(default)]
    #[dsl(block)]
    #[state(artifact)]
    pub calibration: CalibrationState,
    #[value(default)]
    #[dsl(block)]
    #[state(artifact)]
    pub params: ReconstructionParams,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub gcps: Vec<GroundControlPoint>,
    #[value(default)]
    #[dsl(block)]
    #[state(artifact)]
    pub results: ReconstructionResults,
}

impl Default for RemodelingSnapshot {
    fn default() -> Self {
        Self {
            schema: REMODELING_DOCUMENT_SCHEMA.into(),
            id: "remodeling".into(),
            streams: Vec::new(),
            assets: BTreeMap::new(),
            durable_artifacts: RemodelingDurableArtifactStore::new(),
            calibration: CalibrationState::default(),
            params: ReconstructionParams::default(),
            gcps: Vec::new(),
            results: ReconstructionResults::default(),
        }
    }
}
//#endregion 🔖️Snapshot

/// 🧾️ Document schema id every remodeling snapshot carries.
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
pub const REMODELING_DIALECT: semio_framework_artifact_reference::Dialect = semio_framework_artifact_reference::Dialect { artifact_kind: "s.remodel.remodeling", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId::ANY };

//#region 🧩️Composition

#[path = "🧩️asset-handles-and-durable-content/🦀️.rs"]
mod asset_handles_and_durable_content;
pub use asset_handles_and_durable_content::{
    committed_remodeling_asset_handle, durable_remodeling_asset, image_asset_child_handle, remodeling_asset, remodeling_asset_chunk_source, remodeling_asset_dimensions, remodeling_content_is_complete, remodeling_durable_chunk,
    store_remodeling_asset, RemodelingAssetChild, RemodelingAssetChunkSource, RemodelingContentDigest, RemodelingContentKind, RemodelingDurableArtifact, RemodelingDurableArtifactStore, RemodelingMeshChild, REMODELING_DURABLE_CHUNK_RAW_BYTES,
};

#[path = "🕸️mesh-handles/🦀️.rs"]
mod mesh_handles;
pub use mesh_handles::{
    apply_mesh_chunk, bounded_remodeling_mesh_chunk, bounded_remodeling_mesh_chunk_count, empty_remodeling_mesh_handle, mesh_is_within_resolution_envelope, placeholder_remodeling_mesh_handle, remodeling_mesh_content_handle,
    remodeling_mesh_content_handle_parts, resolve_bounded_remodeling_mesh,
};
#[cfg(any(test, feature = "testkit"))]
pub use mesh_handles::{mint_and_stash_mesh, remodeling_mesh_workspace};

#[path = "🧮️buffers/🦀️.rs"]
mod buffers;
pub use buffers::{ByteBuffer, Float32Buffer};

#[path = "🎞️media-and-calibration/🦀️.rs"]
mod media_and_calibration;
pub use media_and_calibration::{CalibrationState, CameraCalibration, FrameRef, GcpObservation, GroundControlPoint, ImageAsset, MediaKind, MediaStream, RigExtrinsic, VideoCodec, VideoSource};

#[path = "⚙️reconstruction-params/🦀️.rs"]
mod reconstruction_params;
pub use reconstruction_params::{
    CameraPosePreview, DenseParams, DenseResolution, FeatureDetector, FeatureParams, GeoParams, IngestParams, MatchParams, MatcherKind, MeshParams, MeshSource, MotionParams, ReconstructionParams, RobustLossKind, SfmParams,
};

#[path = "📊️reconstruction-results/🦀️.rs"]
mod reconstruction_results;
pub use reconstruction_results::{CameraTrajectory, DenseCloud, GeoProducts, MotionTrackSummary, QcReportSnapshot, ReconstructionResults, RemodelingMesh, SparseCloud, TrackClass, WatertightReportSnapshot};

//#endregion 🧩️Composition

//#region 🔖️Domain

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
        results: ReconstructionResults { mesh: RemodelingMesh { mesh: placeholder_remodeling_mesh_handle(), source: MeshSource::Placeholder, ..RemodelingMesh::default() }, ..ReconstructionResults::default() },
    }
}
//#endregion 🔖️Domain

//#region 🔖️CanonicalOrder
/// 🔢️ Insertion point that keeps a keyed collection in ascending key order. Every collection this
/// vocabulary addresses by key — `streams` and `gcps` by `id`, `calibration.cameras` by `id`,
/// `calibration.rig` by `camera_id`, a stream's `frames` by `(index, asset_id)`, a GCP's
/// `observations` by `(stream_id, frame_index)` — is a canonically ordered document invariant, so a
/// `create-*`/`add-*` puts its member exactly where a later `delete-*`/`remove-*` took it from and
/// `inverse(m, before)` applied to `after` restores `before` member positions included.
pub fn ordered_index<T, K: Ord>(items: &[T], key: &K, key_of: impl Fn(&T) -> K) -> usize {
    items.partition_point(|item| key_of(item) < *key)
}
//#endregion 🔖️CanonicalOrder

//#region 🔖️Leaves
/// 🧾️ The snapshot facet's five format leaves.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: concat!(include_str!("🦀️.rs"), include_str!("🧩️asset-handles-and-durable-content/🦀️.rs"), include_str!("🕸️mesh-handles/🦀️.rs"), include_str!("🧮️buffers/🦀️.rs"), include_str!("🎞️media-and-calibration/🦀️.rs"), include_str!("⚙️reconstruction-params/🦀️.rs"), include_str!("📊️reconstruction-results/🦀️.rs")), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🔖️Leaves
