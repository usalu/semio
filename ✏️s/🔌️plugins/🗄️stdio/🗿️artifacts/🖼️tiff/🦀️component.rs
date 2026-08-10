//! 🎪 `stdio.tiff` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::tiff::schema::snapshot::TiffSnapshot;
pub use crate::artifacts::tiff::schema::TiffArtifact;
pub use crate::artifacts::tiff::schema::diff::TiffDiff;
pub use crate::artifacts::tiff::schema::mutations::TiffMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_TIFF_DOCUMENT_SCHEMA: &str = "stdio.tiff";

/// 🧬️ Artifact schema descriptor id.
pub const TIFF_ARTIFACT_SCHEMA_ID: &str = "s.stdio.tiff";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.tiff".into(),
        name: "Tiff".into(),
        source_format: STDIO_TIFF_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
