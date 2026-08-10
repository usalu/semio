//! 🎪 `stdio.dxf` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::dxf::schema::snapshot::DxfSnapshot;
pub use crate::artifacts::dxf::schema::DxfArtifact;
pub use crate::artifacts::dxf::schema::diff::DxfDiff;
pub use crate::artifacts::dxf::schema::mutations::DxfMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_DXF_DOCUMENT_SCHEMA: &str = "stdio.dxf";

/// 🧬️ Artifact schema descriptor id.
pub const DXF_ARTIFACT_SCHEMA_ID: &str = "s.stdio.dxf";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.dxf".into(),
        name: "Dxf".into(),
        source_format: STDIO_DXF_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: STDIO_DXF_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
