//! 💡️ EnergyModel inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🗃️entries/`).
//!
//! Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM: the old opaque `model_json` field is gone —
//! `structure`/`zones` are composed `s.stdio.semio.value`/`table` children, so the whole-snapshot
//! derivation now reads the real typed `crate::model::Model` behind them (via
//! `crate::energy_model`, the working-scene accessor) and census over ITS OWN
//! first-party `pack::json` serialization (`Model` derives `ToValue`) — always a full JSON object,
//! never a possibly-malformed opaque body.

use crate::EnergyModelSnapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

use super::entries::compute_energy_model_entries;

pub use super::entries::EnergyModelEntries;

//#region 🔖️Inference
/// 💡️ Everything inferable from an energy-model snapshot. One field per named inference under
/// `💡️inferences/` (currently: `entries`, backed by the `🗃️entries/` slug dir).
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.energy.model.inference")]
pub struct EnergyModelInference {
    #[derived]
    pub entries: EnergyModelEntries,
}

impl protocol::Inference<EnergyModelSnapshot> for EnergyModelInference {
    fn infer(snapshot: &EnergyModelSnapshot) -> Self {
        Self { entries: compute_energy_model_entries(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `EnergyModelSnapshot::default()`'s `model_json` ever stops being `"{}"`. Same "match `infer` of
/// the real default, don't derive structurally" trick `AddInference` uses in
/// `📡️spr/🎮️command/🦀️.rs`.
impl Default for EnergyModelInference {
    fn default() -> Self {
        <Self as protocol::Inference<EnergyModelSnapshot>>::infer(&EnergyModelSnapshot::default())
    }
}

impl protocol::InferenceSpec<EnergyModelSnapshot> for EnergyModelInference {
    fn inference_schema_id() -> &'static str {
        "s.energy.model.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.energy.model.inference.entries", reads: &["structure", "zones"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::ModelBuilder {
    type Snapshot = EnergyModelSnapshot;
    type Inference = EnergyModelInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.energy.model.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `energy_model_artifact_schema_descriptor`'s registration.
pub fn energy_model_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.energy.model.inference",
        inference: framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
