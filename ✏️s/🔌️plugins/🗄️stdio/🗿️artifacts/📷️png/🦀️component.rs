//! 🎪 `stdio.png` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::png::schema::snapshot::PngSnapshot;
pub use crate::artifacts::png::schema::PngArtifact;
pub use crate::artifacts::png::schema::diff::PngDiff;
pub use crate::artifacts::png::schema::mutations::PngMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_PNG_DOCUMENT_SCHEMA: &str = "stdio.png";

/// 🧬️ Artifact schema descriptor id.
pub const PNG_ARTIFACT_SCHEMA_ID: &str = "s.stdio.png";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.png".into(),
        name: "Png".into(),
        source_format: STDIO_PNG_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_PNG_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
