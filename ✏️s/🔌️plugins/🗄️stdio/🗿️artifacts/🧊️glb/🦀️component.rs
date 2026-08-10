//! 🎪 `stdio.glb` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::glb::schema::snapshot::GlbSnapshot;
pub use crate::artifacts::glb::schema::GlbArtifact;
pub use crate::artifacts::glb::schema::diff::GlbDiff;
pub use crate::artifacts::glb::schema::mutations::GlbMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_GLB_DOCUMENT_SCHEMA: &str = "stdio.glb";

/// 🧬️ Artifact schema descriptor id.
pub const GLB_ARTIFACT_SCHEMA_ID: &str = "s.stdio.glb";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.glb".into(),
        name: "Glb".into(),
        source_format: STDIO_GLB_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_GLB_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
