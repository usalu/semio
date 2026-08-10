//! 🎪 Energy model artifact — headless BEM document surface over `crate::Model`.


pub use crate::artifacts::model::schema::snapshot::EnergyModelSnapshot;
pub use crate::artifacts::model::schema::mutations::EnergyModelMutation;
pub use crate::artifacts::model::schema::diff::EnergyModelDiff;

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::model::schema::EnergyModelArtifact;

/// @emoji 🔖️ Document schema / DSL envelope id.


pub const ENERGY_MODEL_DOCUMENT_SCHEMA: &str = "energy.model";

/// @emoji 🧬️ Artifact schema descriptor id.
pub const ENERGY_MODEL_ARTIFACT_SCHEMA_ID: &str = "s.energy.model";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — Data × Value per owner-table (`data.🔋️model`).
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "data.🔋️model".into(),
        name: "Energy Model".into(),
        source_format: ENERGY_MODEL_DOCUMENT_SCHEMA.into(),
        component_kind: "energy".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: ENERGY_MODEL_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.xlsx", "stdio.zip"],
        import_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.xlsx", "stdio.zip"],
    }
}
//#endregion 🔖️ArtifactKind
