//! 🧬️ Remodel diff schema — sparse field delta over the artifact.

use crate::artifacts::remodel::schema::{
    RemodelArtifact, RemodelUiCamera, RemodelUiFrameCursor, RemodelUiLayers, RemodelUiSelection,
};
use crate::artifacts::remodel::{
    CalibrationState, GroundControlPoint, ImageAsset, MediaStream, ReconstructionJob,
    ReconstructionParams, ReconstructionResults,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the remodel artifact; persistent entries apply via MutationDiff.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.remodel.remodel")]
pub struct RemodelDiff {
    #[state(persistent)] pub artifact: Option<Box<RemodelArtifact>>,
    #[state(persistent)] pub schema: Option<String>,
    #[state(persistent)] pub id: Option<String>,
    #[state(persistent)] pub streams: Option<RemodelMediaStreamList>,
    #[state(persistent)] pub assets: Option<BTreeMap<String, ImageAsset>>,
    #[state(persistent)] pub calibration: Option<CalibrationState>,
    #[state(persistent)] pub params: Option<ReconstructionParams>,
    #[state(persistent)] pub gcps: Option<RemodelGcpList>,
    #[state(persistent)] pub job: Option<ReconstructionJob>,
    #[state(persistent)] pub results: Option<ReconstructionResults>,
    #[state(shared_ui)] pub selection: Option<RemodelUiSelection>,
    #[state(shared_ui)] pub active_utility_id: Option<String>,
    #[state(shared_ui)] pub report_table: Option<String>,
    #[state(shared_ui)] pub frame_cursor: Option<RemodelUiFrameCursor>,
    #[state(local_ui)] pub camera: Option<RemodelUiCamera>,
    #[state(local_ui)] pub layers: Option<RemodelUiLayers>,
    #[state(local_ui)] pub locale: Option<String>,
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
