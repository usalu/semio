//! 🧬️ schema leaf
use crate::artifacts::draw::DrawCamera;
use schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.draw.draw.presence")]
pub struct DrawPresence {
    #[state(presence)]
    pub engagement_input: String,
    #[state(presence)]
    pub camera: DrawCamera,
    #[state(presence)]
    pub active_utility_id: String,
}
