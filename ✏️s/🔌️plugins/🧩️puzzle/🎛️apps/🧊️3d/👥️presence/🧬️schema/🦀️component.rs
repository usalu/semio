//! 🧬️ schema leaf
use artifact_schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.puzzle.puzzle3d.presence")]
pub struct Puzzle3dPresence {
    #[state(presence)] pub selected_object_ids: Vec<String>,
    #[state(presence)] pub selected_vortex_ids: Vec<String>,
    #[state(presence)] pub selected_attraction_ids: Vec<String>,
    #[state(presence)] pub selected_target_volume_ids: Vec<String>,
    #[state(presence)] pub selected_reference_ids: Vec<String>,
    #[state(presence)] pub hovered_object_id: Option<String>,
    #[state(presence)] pub hovered_vortex_full_id: Option<String>,
    #[state(presence)] pub camera_position: [f64; 3],
    #[state(presence)] pub camera_target: [f64; 3],
    #[state(presence)] pub camera_zoom: f64,
    #[state(presence)] pub active_utility_id: String,
    #[state(presence)] pub active_tool_id: Option<String>,
}
