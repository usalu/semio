//! 🎪 `stdio.step` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::step::schema::snapshot::StepSnapshot;
pub use crate::artifacts::step::schema::StepArtifact;
pub use crate::artifacts::step::schema::diff::StepDiff;
pub use crate::artifacts::step::schema::mutations::StepMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_STEP_DOCUMENT_SCHEMA: &str = "stdio.step";

/// 🧬️ Artifact schema descriptor id.
pub const STEP_ARTIFACT_SCHEMA_ID: &str = "s.stdio.step";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.step".into(),
        name: "Step".into(),
        source_format: STDIO_STEP_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: STDIO_STEP_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
