//! 🧬️ schema leaf
use artifact_schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.puzzle.puzzle5d.presence")]
pub struct Puzzle5dPresence {
    #[state(presence)]
    pub camera2d_x: f64,
    #[state(presence)]
    pub camera2d_y: f64,
    #[state(presence)]
    pub camera2d_zoom: f64,
    #[state(presence)]
    pub camera3d_position: [f64; 3],
    #[state(presence)]
    pub camera3d_target: [f64; 3],
    #[state(presence)]
    pub camera3d_zoom: f64,
    #[state(presence)]
    pub active_utility_id: String,
}
