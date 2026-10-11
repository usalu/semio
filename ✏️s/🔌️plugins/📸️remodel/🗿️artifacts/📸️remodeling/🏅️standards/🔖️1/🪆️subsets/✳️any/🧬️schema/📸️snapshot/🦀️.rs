//! 🧬️ Remodeling snapshot schema — artifact-lane fields only.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `assets` carries real
//! `store::ArtifactChild<SemioImageSnapshot>` handles (composed `s.stdio.semio.image` children, one
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

use crate::{CalibrationState, GroundControlPoint, MediaStream, ReconstructionParams, ReconstructionResults, RemodelingAssetChild, RemodelingDurableArtifactStore, REMODELING_DOCUMENT_SCHEMA};
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
//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

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
