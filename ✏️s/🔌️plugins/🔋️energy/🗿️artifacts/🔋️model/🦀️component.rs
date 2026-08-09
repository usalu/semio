//! 🎪 Energy model artifact — headless BEM document surface over `crate::Model`.

pub use crate::artifacts::model::snapshot::schema::EnergyModelSnapshot;
pub use crate::artifacts::model::schema::EnergyModelArtifact;
pub use crate::artifacts::model::diff::EnergyModelDiff;
pub use crate::artifacts::model::mutations::EnergyModelMutation;

/// @emoji 🔖️ Document schema / DSL envelope id.
pub const ENERGY_MODEL_DOCUMENT_SCHEMA: &str = "energy.model";

/// @emoji 🧬️ Artifact schema descriptor id.
pub const ENERGY_MODEL_ARTIFACT_SCHEMA_ID: &str = "s.energy.model";
