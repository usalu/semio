//! 🧬️ schema leaf
use artifact_schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.puzzle.puzzle5d.presence")]
pub struct Puzzle5dPresence {
    #[state(presence)] pub selected_part_ids: Vec<String>,
    #[state(presence)] pub selected_grip_ids: Vec<String>,
    #[state(presence)] pub selected_fastener_ids: Vec<String>,
    #[state(presence)] pub hovered_part_id: Option<String>,
    #[state(presence)] pub camera2d_x: f64,
    #[state(presence)] pub camera2d_y: f64,
    #[state(presence)] pub camera2d_zoom: f64,
    #[state(presence)] pub camera3d_position: [f64; 3],
    #[state(presence)] pub camera3d_target: [f64; 3],
    #[state(presence)] pub camera3d_zoom: f64,
    #[state(presence)] pub active_utility_id: String,
}
