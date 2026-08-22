//! 🧬️ Remodel diff schema — sparse field delta over the artifact.

use crate::artifacts::remodel::schema::{RemodelArtifact, RemodelUiCamera, RemodelUiFrameCursor, RemodelUiLayers, RemodelUiSelection};
use crate::artifacts::remodel::{CalibrationState, GroundControlPoint, MediaStream, ReconstructionJob, ReconstructionParams, ReconstructionResults, RemodelAssetChild, RemodelDurableArtifactStore};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the remodel artifact; persistent entries apply via MutationDiff.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.remodel.remodel")]
pub struct RemodelDiff {
    #[state(artifact)]
    pub artifact: Option<Box<RemodelArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub id: Option<String>,
    #[state(artifact)]
    pub streams: Option<RemodelMediaStreamList>,
    #[state(artifact)]
    pub assets: Option<BTreeMap<String, RemodelAssetChild>>,
    #[state(artifact)]
    pub durable_artifacts: Option<RemodelDurableArtifactStore>,
    #[state(artifact)]
    pub calibration: Option<CalibrationState>,
    #[state(artifact)]
    pub params: Option<ReconstructionParams>,
    #[state(artifact)]
    pub gcps: Option<RemodelGcpList>,
    #[state(artifact)]
    pub job: Option<ReconstructionJob>,
    #[state(artifact)]
    pub results: Option<ReconstructionResults>,
    #[state(presence)]
    pub selection: Option<RemodelUiSelection>,
    #[state(presence)]
    pub active_utility_id: Option<String>,
    #[state(presence)]
    pub report_table: Option<String>,
    #[state(presence)]
    pub frame_cursor: Option<RemodelUiFrameCursor>,
    #[state(config)]
    pub camera: Option<RemodelUiCamera>,
    #[state(config)]
    pub layers: Option<RemodelUiLayers>,
    #[state(config)]
    pub locale: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 Media-stream list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelMediaStreamList {
    pub values: Vec<MediaStream>,
}

/// 📋 GCP list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelGcpList {
    pub values: Vec<GroundControlPoint>,
}
//#endregion 🔖️DeltaHelpers
