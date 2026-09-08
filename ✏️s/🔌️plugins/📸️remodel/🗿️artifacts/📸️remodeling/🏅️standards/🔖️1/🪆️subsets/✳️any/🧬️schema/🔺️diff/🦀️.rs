//! 🧬️ Remodeling diff schema — sparse field delta over the artifact.

use crate::schema::{RemodelingUiCamera, RemodelingUiFrameCursor, RemodelingUiLayers, RemodelingUiSelection};
use crate::{CalibrationState, GroundControlPoint, MediaStream, ReconstructionJob, ReconstructionParams, ReconstructionResults, RemodelingAssetChild, RemodelingDurableArtifactStore};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the remodeling artifact; persistent entries apply via MutationDiff.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.remodel.remodeling")]
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
    pub report_table: Option<String>,
    #[state(presence)]
    pub frame_cursor: Option<RemodelingUiFrameCursor>,
    #[state(config)]
    pub camera: Option<RemodelingUiCamera>,
    #[state(config)]
    pub layers: Option<RemodelingUiLayers>,
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

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::schema::RemodelingArtifact;
//#endregion 🔁️Re-exports
