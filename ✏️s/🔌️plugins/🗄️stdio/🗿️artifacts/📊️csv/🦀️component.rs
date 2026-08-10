//! 🎪 `stdio.csv` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::csv::schema::snapshot::CsvSnapshot;
pub use crate::artifacts::csv::schema::CsvArtifact;
pub use crate::artifacts::csv::schema::diff::CsvDiff;
pub use crate::artifacts::csv::schema::mutations::CsvMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_CSV_DOCUMENT_SCHEMA: &str = "stdio.csv";

/// 🧬️ Artifact schema descriptor id.
pub const CSV_ARTIFACT_SCHEMA_ID: &str = "s.stdio.csv";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.csv".into(),
        name: "Csv".into(),
        source_format: STDIO_CSV_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Document },
        schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
