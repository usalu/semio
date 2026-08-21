//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.note.note.presence")]
pub struct NotePresence {
    #[state(presence)]
    pub camera_x: f64,
    #[state(presence)]
    pub camera_y: f64,
    #[state(presence)]
    pub camera_zoom: f64,
    #[state(presence)]
    pub active_utility_id: String,
}
