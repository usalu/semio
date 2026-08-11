//! 🎪 `stdio.avi` artifact — new-format artifact (master plan "New format artifacts" table).

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
pub use crate::artifacts::avi::standards::v1_0::subsets::any::schema::AviArtifact;
pub use crate::artifacts::avi::standards::v1_0::subsets::any::schema::diff::AviDiff;
pub use crate::artifacts::avi::standards::v1_0::subsets::any::schema::mutations::AviMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_AVI_DOCUMENT_SCHEMA: &str = "stdio.avi";

/// 🧬️ Artifact schema descriptor id.
pub const AVI_ARTIFACT_SCHEMA_ID: &str = "s.stdio.avi";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.avi".into(),
        name: "Avi".into(),
        source_format: STDIO_AVI_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_AVI_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
