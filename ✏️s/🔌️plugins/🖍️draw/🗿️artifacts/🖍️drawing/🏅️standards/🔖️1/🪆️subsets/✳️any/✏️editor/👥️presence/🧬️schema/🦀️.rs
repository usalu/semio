//! 🧬️ schema leaf

use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.draw.drawing.presence")]
pub struct DrawingPresence {
    #[state(presence)]
    pub engagement_input: String,
    #[state(presence)]
    pub camera: store::Viewport2d,
}

impl Default for DrawingPresence {
    fn default() -> Self {
        Self { engagement_input: String::new(), camera: store::Viewport2d { x: 512.0, y: 512.0, zoom: 0.75 } }
    }
}
