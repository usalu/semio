//! 🎪 `stdio.dwg` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::dwg::schema::snapshot::DwgSnapshot;
pub use crate::artifacts::dwg::schema::DwgArtifact;
pub use crate::artifacts::dwg::schema::diff::DwgDiff;
pub use crate::artifacts::dwg::schema::mutations::DwgMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_DWG_DOCUMENT_SCHEMA: &str = "stdio.dwg";

/// 🧬️ Artifact schema descriptor id.
pub const DWG_ARTIFACT_SCHEMA_ID: &str = "s.stdio.dwg";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.dwg".into(),
        name: "Dwg".into(),
        source_format: STDIO_DWG_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
