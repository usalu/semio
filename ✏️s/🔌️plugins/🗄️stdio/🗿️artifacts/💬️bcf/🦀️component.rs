//! 🎪 `stdio.bcf` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::bcf::schema::snapshot::BcfSnapshot;
pub use crate::artifacts::bcf::schema::BcfArtifact;
pub use crate::artifacts::bcf::schema::diff::BcfDiff;
pub use crate::artifacts::bcf::schema::mutations::BcfMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_BCF_DOCUMENT_SCHEMA: &str = "stdio.bcf";

/// 🧬️ Artifact schema descriptor id.
pub const BCF_ARTIFACT_SCHEMA_ID: &str = "s.stdio.bcf";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.bcf".into(),
        name: "Bcf".into(),
        source_format: STDIO_BCF_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_BCF_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
