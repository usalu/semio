//! 🧬️ schema leaf
use crate::artifacts::draw::DrawCamera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.draw.draw.presence")]
pub struct DrawPresence {
    #[state(presence)] pub engagement_input: String,
    #[state(presence)] pub camera: DrawCamera,
    #[state(presence)] pub active_utility_id: String,
}
