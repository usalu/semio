//! 🎪 `stdio.pptx` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::pptx::schema::snapshot::PptxSnapshot;
pub use crate::artifacts::pptx::schema::PptxArtifact;
pub use crate::artifacts::pptx::schema::diff::PptxDiff;
pub use crate::artifacts::pptx::schema::mutations::PptxMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_PPTX_DOCUMENT_SCHEMA: &str = "stdio.pptx";

/// 🧬️ Artifact schema descriptor id.
pub const PPTX_ARTIFACT_SCHEMA_ID: &str = "s.stdio.pptx";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.pptx".into(),
        name: "Pptx".into(),
        source_format: STDIO_PPTX_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_PPTX_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
