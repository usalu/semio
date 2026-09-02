//! 🧬️ Remodel snapshot schema — artifact-lane fields only.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `assets` carries real
//! `store::ArtifactChild<SemioImageSnapshot>` handles (composed `s.stdio.semio.image` children, one
//! per asset id — see `🗿️artifacts/📸️remodel/🦀️.rs`'s `🧩️Composition` region) instead of
//! embedded `ImageAsset` bytes. `ArtifactChild<S>: dsl::DslField` is now real
//! (`🏪️store/🦀️.rs:523`), and `BTreeMap<String, T: DslField>: DslField` already exists
//! generically (`🗣️dsl/🦀️.rs:178`), so this struct's `#[derive(dsl::DslRecord)]` keeps
//! working unmodified for `assets` — no hand-rolled codec needed here (unlike `🖨️raster`'s own
//! identical migration, authored before this generic `BTreeMap` impl was confirmed reachable).
//! `child_slots()` is honestly EMPTY for `assets` regardless: the derive's `#[child(kind=...)]`
//! mechanism only recognizes a bare `ArtifactChild<T>`/`Vec<ArtifactChild<T>>` field, not a
//! `BTreeMap` value — kept as `BTreeMap<String, ArtifactChild<S>>` (not reshaped to a `Vec`) to
//! preserve the same id-keyed addressing every existing `create-asset`/`delete-asset` mutation and
//! `MediaStream.frames`/`RemodelMesh.texture_asset_id`/`GeoProducts.*_asset_id` lookup already
//! assumes — the type/mutation/persistence layer is fully real, only the derive-generated SCHEMA
//! INTROSPECTION table is incomplete for this one field (matches `🖨️raster`'s/`💠️lowpoly`'s own
//! already-accepted gap for the identical shape).

use crate::artifacts::remodel::{CalibrationState, GroundControlPoint, MediaStream, ReconstructionJob, ReconstructionParams, ReconstructionResults, RemodelAssetChild, RemodelDurableArtifactStore, REMODEL_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Snapshot
/// 📸️ Persisted remodel document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "remodel")]
#[artifact_schema(id = "s.remodel.remodel")]
pub struct RemodelSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[serde(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub streams: Vec<MediaStream>,
    #[serde(default)]
    #[state(artifact)]
    pub assets: BTreeMap<String, RemodelAssetChild>,
    #[serde(default)]
    #[state(artifact)]
    pub durable_artifacts: RemodelDurableArtifactStore,
    #[serde(default)]
    #[dsl(block)]
    #[state(artifact)]
    pub calibration: CalibrationState,
    #[serde(default)]
    #[dsl(block)]
    #[state(artifact)]
    pub params: ReconstructionParams,
    #[serde(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub gcps: Vec<GroundControlPoint>,
    #[serde(default)]
    #[dsl(block)]
    #[state(artifact)]
    pub job: ReconstructionJob,
    #[serde(default)]
    #[dsl(block)]
    #[state(artifact)]
    pub results: ReconstructionResults,
}
//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for RemodelSnapshot {
    const EXTENSION: &'static str = "remodel";
    async fn envelope_id() -> &'static str {
        "remodel.remodel"
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    async fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for RemodelSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    async fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for RemodelSnapshot {
    fn default() -> Self {
        Self {
            schema: REMODEL_DOCUMENT_SCHEMA.into(),
            id: "remodel".into(),
            streams: Vec::new(),
            assets: BTreeMap::new(),
            durable_artifacts: RemodelDurableArtifactStore::new(),
            calibration: CalibrationState::default(),
            params: ReconstructionParams::default(),
            gcps: Vec::new(),
            job: ReconstructionJob::default(),
            results: ReconstructionResults::default(),
        }
    }
}
//#endregion 🔖️Snapshot
