//! 🎪 `stdio.txt` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::txt::schema::snapshot::TxtSnapshot;
pub use crate::artifacts::txt::schema::TxtArtifact;
pub use crate::artifacts::txt::schema::diff::TxtDiff;
pub use crate::artifacts::txt::schema::mutations::TxtMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_TXT_DOCUMENT_SCHEMA: &str = "stdio.txt";

/// 🧬️ Artifact schema descriptor id.
pub const TXT_ARTIFACT_SCHEMA_ID: &str = "s.stdio.txt";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.txt".into(),
        name: "Txt".into(),
        source_format: STDIO_TXT_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: STDIO_TXT_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
