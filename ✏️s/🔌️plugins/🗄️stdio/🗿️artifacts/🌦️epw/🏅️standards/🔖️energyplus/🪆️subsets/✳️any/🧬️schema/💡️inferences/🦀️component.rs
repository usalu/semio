//! 💡️ EpwInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🌡climate/`, honestly derivable
//! from `records` alone — EPW is climate/weather data, not geometry, so the closest honest
//! derived statistic is a min/max/avg fold over the hourly dry-bulb temperature column, not a
//! bounding box).

use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

use super::climate::{compute_epw_climate_summary, EpwClimateSummary};

//#region 🔖️Inference
/// 💡️ Everything inferable from an epw snapshot. One field per named inference under
/// `💡️inferences/` (currently: `climate`, backed by the `🌡climate/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.epw.inference")]
pub struct EpwInference {
    #[derived]
    pub climate: EpwClimateSummary,
}

impl protocol::Inference<EpwSnapshot> for EpwInference {
    fn infer(snapshot: &EpwSnapshot) -> Self {
        Self { climate: compute_epw_climate_summary(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `EpwSnapshot::default()`'s `records` ever stops being empty.
impl Default for EpwInference {
    fn default() -> Self {
        <Self as protocol::Inference<EpwSnapshot>>::infer(&EpwSnapshot::default())
    }
}

impl protocol::InferenceSpec<EpwSnapshot> for EpwInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.epw.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.epw.inference.climate", reads: &["records"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `climate` is a single min/max/avg fold over `records`' own
/// `dry_bulb_temp` column (already O(n) in total record count), with no honest per-entity
/// incremental decomposition (a merkle dep-chain over this flat whole-snapshot fold costs more
/// than the fold it would cache) — the default `infer_cached` passthrough is exact.
impl semio_framework_plugin::ArtifactInferrer for crate::artifacts::epw::standards::energyplus::subsets::any::schema::EpwBuilder {
    type Snapshot = EpwSnapshot;
    type Inference = EpwInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.epw.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `epw_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn epw_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.epw.inference",
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

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = EpwSnapshot::default();
        assert_eq!(EpwInference::infer(&snapshot), EpwInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(EpwInference::infer(&EpwSnapshot::default()), EpwInference::default());
    }
}
//#endregion 🧪️Tests
