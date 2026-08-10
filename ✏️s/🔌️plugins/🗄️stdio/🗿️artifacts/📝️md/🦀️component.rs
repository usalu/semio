//! 🎪 `stdio.md` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::md::schema::snapshot::MdSnapshot;
pub use crate::artifacts::md::schema::MdArtifact;
pub use crate::artifacts::md::schema::diff::MdDiff;
pub use crate::artifacts::md::schema::mutations::MdMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_MD_DOCUMENT_SCHEMA: &str = "stdio.md";

/// 🧬️ Artifact schema descriptor id.
pub const MD_ARTIFACT_SCHEMA_ID: &str = "s.stdio.md";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.md".into(),
        name: "Md".into(),
        source_format: STDIO_MD_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
