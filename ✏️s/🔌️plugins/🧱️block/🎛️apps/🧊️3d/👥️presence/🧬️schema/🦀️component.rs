//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.block.3d.presence")]
pub struct Block3dPresence {
    #[state(presence)] pub selected_ids: Vec<String>,
    #[state(presence)] pub hovered_vortex_full_id: Option<String>,
}
