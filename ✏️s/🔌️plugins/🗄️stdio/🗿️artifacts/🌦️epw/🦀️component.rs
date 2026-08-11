//! 🎪 `stdio.epw` artifact — new-format artifact (master plan "New format artifacts" table).

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;
pub use crate::artifacts::epw::standards::energyplus::subsets::any::schema::EpwArtifact;
pub use crate::artifacts::epw::standards::energyplus::subsets::any::schema::diff::EpwDiff;
pub use crate::artifacts::epw::standards::energyplus::subsets::any::schema::mutations::EpwMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_EPW_DOCUMENT_SCHEMA: &str = "stdio.epw";

/// 🧬️ Artifact schema descriptor id.
pub const EPW_ARTIFACT_SCHEMA_ID: &str = "s.stdio.epw";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.epw".into(),
        name: "Epw".into(),
        source_format: STDIO_EPW_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_EPW_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
