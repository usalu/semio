//! 🧬️ Remodel artifact schema — every field of the artifact with its state class.

use crate::artifacts::remodel::{
    CalibrationState, GroundControlPoint, ImageAsset, MediaStream, ReconstructionJob,
    ReconstructionParams, ReconstructionResults, RemodelSnapshot,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Artifact
/// 🧬️ Full remodel artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodel.remodel")]
pub struct RemodelArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub id: String,
    #[state(persistent)] pub streams: Vec<MediaStream>,
    #[state(persistent)] pub assets: BTreeMap<String, ImageAsset>,
    #[state(persistent)] pub calibration: CalibrationState,
    #[state(persistent)] pub params: ReconstructionParams,
    #[state(persistent)] pub gcps: Vec<GroundControlPoint>,
    #[state(persistent)] pub job: ReconstructionJob,
    #[state(persistent)] pub results: ReconstructionResults,
    #[state(shared_ui)] pub selection: RemodelUiSelection,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(shared_ui)] pub report_table: String,
    #[state(shared_ui)] pub frame_cursor: RemodelUiFrameCursor,
    #[state(local_ui)] pub camera: RemodelUiCamera,
    #[state(local_ui)] pub layers: RemodelUiLayers,
    #[state(local_ui)] pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️UiHelpers
/// 🎥️ Artifact-owned orbit camera (mirror of app config camera).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelUiCamera {
    pub position: [f64; 3],
    pub target: [f64; 3],
    pub fov: f64,
}

impl Default for RemodelUiCamera {
    fn default() -> Self {
        Self { position: [4.0, -4.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 }
    }
}

/// 🖱️ Artifact-owned selection (mirror of app config selection).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelUiSelection {
    pub mode: String,
    pub ids: Vec<String>,
}

/// 👁️ Artifact-owned layer visibility (mirror of app config layers).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelUiLayers {
    pub mesh: bool,
    pub dense: bool,
    pub sparse: bool,
    pub cameras: bool,
    pub gcps: bool,
}

impl Default for RemodelUiLayers {
    fn default() -> Self {
        Self { mesh: true, dense: true, sparse: true, cameras: true, gcps: true }
    }
}

/// 🎞️ Artifact-owned frame cursor (mirror of app config frame cursor).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelUiFrameCursor {
    pub stream_id: Option<String>,
    pub frame_index: u32,
}
//#endregion 🔖️UiHelpers

//#region 🔖️Conversions
impl Default for RemodelArtifact {
    fn default() -> Self {
        Self::from_snapshot(RemodelSnapshot::default())
    }
}

impl RemodelArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> RemodelSnapshot {
        RemodelSnapshot {
            schema: self.schema.clone(),
            id: self.id.clone(),
            streams: self.streams.clone(),
            assets: self.assets.clone(),
            calibration: self.calibration.clone(),
            params: self.params.clone(),
            gcps: self.gcps.clone(),
            job: self.job.clone(),
            results: self.results.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: RemodelSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id: snapshot.id,
            streams: snapshot.streams,
            assets: snapshot.assets,
            calibration: snapshot.calibration,
            params: snapshot.params,
            gcps: snapshot.gcps,
            job: snapshot.job,
            results: snapshot.results,
            selection: RemodelUiSelection::default(),
            active_utility_id: "select".into(),
            report_table: "frames".into(),
            frame_cursor: RemodelUiFrameCursor::default(),
            camera: RemodelUiCamera::default(),
            layers: RemodelUiLayers::default(),
            locale: "en-US".into(),
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: RemodelSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.streams = snapshot.streams;
        self.assets = snapshot.assets;
        self.calibration = snapshot.calibration;
        self.params = snapshot.params;
        self.gcps = snapshot.gcps;
        self.job = snapshot.job;
        self.results = snapshot.results;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.remodel.remodel` — fifteen handcrafted schema leaves.
pub fn remodel_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.remodel.remodel",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),
            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),
            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
