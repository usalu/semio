//! 🎪 `stdio.ifc` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::ifc::schema::snapshot::IfcSnapshot;
pub use crate::artifacts::ifc::schema::IfcArtifact;
pub use crate::artifacts::ifc::schema::diff::IfcDiff;
pub use crate::artifacts::ifc::schema::mutations::IfcMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_IFC_DOCUMENT_SCHEMA: &str = "stdio.ifc";

/// 🧬️ Artifact schema descriptor id.
pub const IFC_ARTIFACT_SCHEMA_ID: &str = "s.stdio.ifc";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.ifc".into(),
        name: "Ifc".into(),
        source_format: STDIO_IFC_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: STDIO_IFC_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
