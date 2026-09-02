//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[value(rename_all = "camelCase", default)]
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
