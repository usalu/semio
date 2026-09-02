//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.remodeling.remodeling.presence")]
pub struct RemodelingPresence {
    #[state(presence)]
    pub world_camera_position: [f64; 3],
    #[state(presence)]
    pub world_camera_target: [f64; 3],
    #[state(presence)]
    pub world_camera_fov: f64,
    #[state(presence)]
    pub frame_stream_id: Option<String>,
    #[state(presence)]
    pub frame_index: u32,
    #[state(presence)]
    pub active_utility_id: String,
    #[state(presence)]
    pub report_table: String,
}
