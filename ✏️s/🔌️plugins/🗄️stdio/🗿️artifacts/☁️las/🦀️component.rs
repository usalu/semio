//! 🎪 `stdio.las` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::las::schema::snapshot::LasSnapshot;
pub use crate::artifacts::las::schema::LasArtifact;
pub use crate::artifacts::las::schema::diff::LasDiff;
pub use crate::artifacts::las::schema::mutations::LasMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_LAS_DOCUMENT_SCHEMA: &str = "stdio.las";

/// 🧬️ Artifact schema descriptor id.
pub const LAS_ARTIFACT_SCHEMA_ID: &str = "s.stdio.las";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.las".into(),
        name: "Las".into(),
        source_format: STDIO_LAS_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_LAS_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
