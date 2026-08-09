//! 🧬️ schema leaf
use artifact_schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.puzzle.puzzle2d.presence")]
pub struct Puzzle2dPresence {
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(shared_ui)] pub camera_x: f64,
    #[state(shared_ui)] pub camera_y: f64,
    #[state(shared_ui)] pub camera_zoom: f64,
    #[state(shared_ui)] pub selection_method: String,
    #[state(shared_ui)] pub active_utility_id: String,
}
