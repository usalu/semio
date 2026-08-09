//! 🧬️ schema leaf
use artifact_schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.puzzle.puzzle3d.presence")]
pub struct Puzzle3dPresence {
    #[state(shared_ui)] pub selected_object_ids: Vec<String>,
    #[state(shared_ui)] pub selected_vortex_ids: Vec<String>,
    #[state(shared_ui)] pub selected_attraction_ids: Vec<String>,
    #[state(shared_ui)] pub selected_target_volume_ids: Vec<String>,
    #[state(shared_ui)] pub selected_reference_ids: Vec<String>,
    #[state(shared_ui)] pub hovered_object_id: Option<String>,
    #[state(shared_ui)] pub hovered_vortex_full_id: Option<String>,
    #[state(shared_ui)] pub camera_position: [f64; 3],
    #[state(shared_ui)] pub camera_target: [f64; 3],
    #[state(shared_ui)] pub camera_zoom: f64,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(shared_ui)] pub active_tool_id: Option<String>,
}
