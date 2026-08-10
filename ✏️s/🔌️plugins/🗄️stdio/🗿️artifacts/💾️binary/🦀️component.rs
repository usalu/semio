//! 🎪 `stdio.binary` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::binary::schema::snapshot::BinarySnapshot;
pub use crate::artifacts::binary::schema::BinaryArtifact;
pub use crate::artifacts::binary::schema::diff::BinaryDiff;
pub use crate::artifacts::binary::schema::mutations::BinaryMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_BINARY_DOCUMENT_SCHEMA: &str = "stdio.binary";

/// 🧬️ Artifact schema descriptor id.
pub const BINARY_ARTIFACT_SCHEMA_ID: &str = "s.stdio.binary";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.binary".into(),
        name: "Binary".into(),
        source_format: STDIO_BINARY_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
