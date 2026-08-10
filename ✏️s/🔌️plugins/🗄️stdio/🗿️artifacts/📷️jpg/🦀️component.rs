//! 🎪 `stdio.jpg` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::jpg::schema::snapshot::JpgSnapshot;
pub use crate::artifacts::jpg::schema::JpgArtifact;
pub use crate::artifacts::jpg::schema::diff::JpgDiff;
pub use crate::artifacts::jpg::schema::mutations::JpgMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_JPG_DOCUMENT_SCHEMA: &str = "stdio.jpg";

/// 🧬️ Artifact schema descriptor id.
pub const JPG_ARTIFACT_SCHEMA_ID: &str = "s.stdio.jpg";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.jpg".into(),
        name: "Jpg".into(),
        source_format: STDIO_JPG_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_JPG_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
