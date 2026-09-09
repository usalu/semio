//! 🧬️ schema leaf

use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.draw.drawing.presence")]
pub struct DrawingPresence {
    #[state(presence)]
    pub engagement_input: String,
    #[state(presence)]
    pub camera: DrawingCamera,
}

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::DrawingCamera;
//#endregion 🔁️Re-exports
