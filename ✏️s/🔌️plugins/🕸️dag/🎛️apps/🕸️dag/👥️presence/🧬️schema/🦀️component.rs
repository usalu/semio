//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.dag.dag.presence")]
pub struct DagPresence {
    #[state(shared_ui)] pub selected_node_ids: Vec<String>,
    #[state(shared_ui)] pub camera_x: f64,
    #[state(shared_ui)] pub camera_y: f64,
    #[state(shared_ui)] pub camera_zoom: f64,
    #[state(shared_ui)] pub hovered_node_id: Option<String>,
    #[state(shared_ui)] pub hovered_edge_id: Option<String>,
}
