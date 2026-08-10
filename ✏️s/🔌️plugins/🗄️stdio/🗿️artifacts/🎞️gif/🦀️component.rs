//! 🎪 `stdio.gif` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::gif::schema::snapshot::GifSnapshot;
pub use crate::artifacts::gif::schema::GifArtifact;
pub use crate::artifacts::gif::schema::diff::GifDiff;
pub use crate::artifacts::gif::schema::mutations::GifMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_GIF_DOCUMENT_SCHEMA: &str = "stdio.gif";

/// 🧬️ Artifact schema descriptor id.
pub const GIF_ARTIFACT_SCHEMA_ID: &str = "s.stdio.gif";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.gif".into(),
        name: "Gif".into(),
        source_format: STDIO_GIF_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_GIF_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
