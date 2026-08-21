//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.sourcing.curate.presence")]
pub struct SourcingCuratePresence {
    #[state(presence)]
    pub world_camera_position: [f64; 3],
    #[state(presence)]
    pub world_camera_target: [f64; 3],
    #[state(presence)]
    pub world_camera_fov: f64,
}
