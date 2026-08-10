//! 🎪 `stdio.bmp` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::bmp::schema::snapshot::BmpSnapshot;
pub use crate::artifacts::bmp::schema::BmpArtifact;
pub use crate::artifacts::bmp::schema::diff::BmpDiff;
pub use crate::artifacts::bmp::schema::mutations::BmpMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_BMP_DOCUMENT_SCHEMA: &str = "stdio.bmp";

/// 🧬️ Artifact schema descriptor id.
pub const BMP_ARTIFACT_SCHEMA_ID: &str = "s.stdio.bmp";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.bmp".into(),
        name: "Bmp".into(),
        source_format: STDIO_BMP_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_BMP_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
