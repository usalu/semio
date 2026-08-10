//! 🎪 `stdio.docx` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::docx::schema::snapshot::DocxSnapshot;
pub use crate::artifacts::docx::schema::DocxArtifact;
pub use crate::artifacts::docx::schema::diff::DocxDiff;
pub use crate::artifacts::docx::schema::mutations::DocxMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_DOCX_DOCUMENT_SCHEMA: &str = "stdio.docx";

/// 🧬️ Artifact schema descriptor id.
pub const DOCX_ARTIFACT_SCHEMA_ID: &str = "s.stdio.docx";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.docx".into(),
        name: "Docx".into(),
        source_format: STDIO_DOCX_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_DOCX_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
