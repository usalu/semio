//! 🧬️ schema leaf
use crate::artifacts::draw::DrawCamera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.draw.draw.presence")]
pub struct DrawPresence {
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(shared_ui)] pub hovered_id: Option<String>,
    #[state(shared_ui)] pub engagement_input: String,
    #[state(shared_ui)] pub camera: DrawCamera,
    #[state(shared_ui)] pub active_utility_id: String,
}
