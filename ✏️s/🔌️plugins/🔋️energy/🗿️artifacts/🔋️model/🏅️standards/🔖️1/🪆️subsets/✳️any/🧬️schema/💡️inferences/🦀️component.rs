//! 💡️ EnergyModel inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🗃entries/`).
//!
//! `model_json` is an opaque JSON body (`crate::model::Model`'s serialized form, per the field's
//! own doc comment) — it is not guaranteed to decode into the full typed `Model` for every
//! persisted snapshot (e.g. the default snapshot's `"{}"` has none of `Model`'s required fields),
//! so the only honest whole-snapshot derivation treats it as an opaque byte/entry container: real
//! top-level JSON key count, real byte size, real content digest — never a typed `Model` field.

use crate::artifacts::model::EnergyModelSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::entries::compute_energy_model_entries;

pub use super::entries::EnergyModelEntries;

//#region 🔖️Inference
/// 💡️ Everything inferable from an energy-model snapshot. One field per named inference under
/// `💡️inferences/` (currently: `entries`, backed by the `🗃entries/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.energy.model.inference")]
pub struct EnergyModelInference {
    #[state(inferred)]
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
/// `📡️spr/🎮️command/🦀️component.rs`.
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
        &[protocol::InferenceFieldSpec { id: "s.energy.model.inference.entries", reads: &["model_json"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::model::standards::v1::subsets::any::schema::ModelBuilder {
    type Snapshot = EnergyModelSnapshot;
    type Inference = EnergyModelInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.energy.model.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `energy_model_artifact_schema_descriptor`'s registration.
pub fn energy_model_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.energy.model.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use protocol::Inference;

    fn populated_snapshot() -> EnergyModelSnapshot {
        EnergyModelSnapshot { model_json: r#"{"name":"demo","zones":[]}"#.into(), ..EnergyModelSnapshot::default() }
    }

    #[test]
    fn inference_determinism_law() {
        let snapshot = populated_snapshot();
        assert_eq!(EnergyModelInference::infer(&snapshot), EnergyModelInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(EnergyModelInference::infer(&EnergyModelSnapshot::default()), EnergyModelInference::default());
    }

    #[test]
    fn entries_counts_top_level_keys_and_bytes() {
        let inferred = EnergyModelInference::infer(&populated_snapshot());
        assert_eq!(inferred.entries.entry_count, 2);
        assert_eq!(inferred.entries.byte_size, populated_snapshot().model_json.len() as u32);
    }
}
//#endregion 🧪️Tests
