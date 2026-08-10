//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodel.remodel.remodelworldcamera")]
pub struct RemodelWorldCamera {
    #[state(local_ui)] pub position: [f64; 3],
    #[state(local_ui)] pub target: [f64; 3],
    #[state(local_ui)] pub fov: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodel.remodel.remodelselection")]
pub struct RemodelSelection {
    #[state(local_ui)] pub mode: String,
    #[state(local_ui)] pub ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodel.remodel.remodellayervisibility")]
pub struct RemodelLayerVisibility {
    #[state(local_ui)] pub mesh: bool,
    #[state(local_ui)] pub dense: bool,
    #[state(local_ui)] pub sparse: bool,
    #[state(local_ui)] pub cameras: bool,
    #[state(local_ui)] pub gcps: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodel.remodel.remodelframecursor")]
pub struct RemodelFrameCursor {
    #[state(local_ui)] pub stream_id: Option<String>,
    #[state(local_ui)] pub frame_index: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodel.remodel.config")]
pub struct RemodelConfig {
    #[state(local_ui)] pub camera: RemodelWorldCamera,
    #[state(local_ui)] pub selection: RemodelSelection,
    #[state(local_ui)] pub layers: RemodelLayerVisibility,
    #[state(local_ui)] pub frame_cursor: RemodelFrameCursor,
    #[state(local_ui)] pub report_table: String,
    #[state(local_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub locale: String,
}

