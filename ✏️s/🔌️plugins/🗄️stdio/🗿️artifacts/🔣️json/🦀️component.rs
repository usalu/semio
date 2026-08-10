//! 🎪 `stdio.json` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::json::schema::snapshot::JsonSnapshot;
pub use crate::artifacts::json::schema::JsonArtifact;
pub use crate::artifacts::json::schema::diff::JsonDiff;
pub use crate::artifacts::json::schema::mutations::JsonMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_JSON_DOCUMENT_SCHEMA: &str = "stdio.json";

/// 🧬️ Artifact schema descriptor id.
pub const JSON_ARTIFACT_SCHEMA_ID: &str = "s.stdio.json";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.json".into(),
        name: "Json".into(),
        source_format: STDIO_JSON_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_JSON_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
