//! 🧬️ Remodeling diff schema — sparse field delta over the artifact.

use crate::artifacts::remodeling::schema::{RemodelingArtifact, RemodelingUiCamera, RemodelingUiFrameCursor, RemodelingUiLayers, RemodelingUiSelection};
use crate::artifacts::remodeling::{CalibrationState, GroundControlPoint, MediaStream, ReconstructionJob, ReconstructionParams, ReconstructionResults, RemodelingAssetChild, RemodelingDurableArtifactStore};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the remodeling artifact; persistent entries apply via MutationDiff.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.remodeling.remodeling")]
pub struct RemodelingDiff {
    #[state(artifact)]
    pub artifact: Option<Box<RemodelingArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub id: Option<String>,
    #[state(artifact)]
    pub streams: Option<RemodelingMediaStreamList>,
    #[state(artifact)]
    pub assets: Option<BTreeMap<String, RemodelingAssetChild>>,
    #[state(artifact)]
    pub durable_artifacts: Option<RemodelingDurableArtifactStore>,
    #[state(artifact)]
    pub calibration: Option<CalibrationState>,
    #[state(artifact)]
    pub params: Option<ReconstructionParams>,
    #[state(artifact)]
    pub gcps: Option<RemodelingGcpList>,
    #[state(artifact)]
    pub job: Option<ReconstructionJob>,
    #[state(artifact)]
    pub results: Option<ReconstructionResults>,
    #[state(presence)]
    pub selection: Option<RemodelingUiSelection>,
    #[state(presence)]
    pub active_utility_id: Option<String>,
    #[state(presence)]
    pub report_table: Option<String>,
    #[state(presence)]
    pub frame_cursor: Option<RemodelingUiFrameCursor>,
    #[state(config)]
    pub camera: Option<RemodelingUiCamera>,
    #[state(config)]
    pub layers: Option<RemodelingUiLayers>,
    #[state(config)]
    pub locale: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 Media-stream list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelingMediaStreamList {
    pub values: Vec<MediaStream>,
}

/// 📋 GCP list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelingGcpList {
    pub values: Vec<GroundControlPoint>,
}
//#endregion 🔖️DeltaHelpers
