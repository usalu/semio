//! 🎪 `stdio.xlsx` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::xlsx::schema::snapshot::XlsxSnapshot;
pub use crate::artifacts::xlsx::schema::XlsxArtifact;
pub use crate::artifacts::xlsx::schema::diff::XlsxDiff;
pub use crate::artifacts::xlsx::schema::mutations::XlsxMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_XLSX_DOCUMENT_SCHEMA: &str = "stdio.xlsx";

/// 🧬️ Artifact schema descriptor id.
pub const XLSX_ARTIFACT_SCHEMA_ID: &str = "s.stdio.xlsx";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.xlsx".into(),
        name: "Xlsx".into(),
        source_format: STDIO_XLSX_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_XLSX_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
