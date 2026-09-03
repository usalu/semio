//! 🧬️ schema leaf
use artifact_schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.puzzle.puzzle2d.presence")]
pub struct Puzzle2dPresence {
    #[state(presence)]
    pub camera_x: f64,
    #[state(presence)]
    pub camera_y: f64,
    #[state(presence)]
    pub camera_zoom: f64,
    #[state(presence)]
    pub active_utility_id: String,
}
