//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.dag.dag.presence")]
pub struct DagPresence {
    #[state(presence)] pub camera_x: f64,
    #[state(presence)] pub camera_y: f64,
    #[state(presence)] pub camera_zoom: f64,
}
