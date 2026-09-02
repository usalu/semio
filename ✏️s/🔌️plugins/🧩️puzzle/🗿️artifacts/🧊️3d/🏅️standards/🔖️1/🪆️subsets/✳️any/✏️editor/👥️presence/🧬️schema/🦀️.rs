//! 🧬️ schema leaf
use artifact_schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.puzzle.puzzle3d.presence")]
pub struct Puzzle3dPresence {
    #[state(presence)]
    pub camera_position: [f64; 3],
    #[state(presence)]
    pub camera_target: [f64; 3],
    #[state(presence)]
    pub camera_zoom: f64,
    #[state(presence)]
    pub active_utility_id: String,
    #[state(presence)]
    pub active_tool_id: Option<String>,
}
