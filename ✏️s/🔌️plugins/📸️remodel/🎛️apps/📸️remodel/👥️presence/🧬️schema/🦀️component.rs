//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.remodel.remodel.presence")]
pub struct RemodelPresence {
    #[state(shared_ui)] pub selection_mode: String,
    #[state(shared_ui)] pub selection_ids: Vec<String>,
    #[state(shared_ui)] pub world_camera_position: [f64; 3],
    #[state(shared_ui)] pub world_camera_target: [f64; 3],
    #[state(shared_ui)] pub world_camera_fov: f64,
    #[state(shared_ui)] pub frame_stream_id: Option<String>,
    #[state(shared_ui)] pub frame_index: u32,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(shared_ui)] pub report_table: String,
}
