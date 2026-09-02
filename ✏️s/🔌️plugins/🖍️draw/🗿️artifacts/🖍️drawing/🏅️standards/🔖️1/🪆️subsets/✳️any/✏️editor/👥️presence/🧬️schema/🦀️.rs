//! 🧬️ schema leaf
use crate::artifacts::drawing::DrawingCamera;
use schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.draw.drawing.presence")]
pub struct DrawingPresence {
    #[state(presence)]
    pub engagement_input: String,
    #[state(presence)]
    pub camera: DrawingCamera,
    #[state(presence)]
    pub active_utility_id: String,
}
