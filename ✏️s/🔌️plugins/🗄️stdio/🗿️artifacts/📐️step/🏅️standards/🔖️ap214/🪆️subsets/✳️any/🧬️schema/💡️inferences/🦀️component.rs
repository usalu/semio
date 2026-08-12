//! 💡️ StepInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`, honestly derivable by
//! folding every real `CARTESIAN_POINT('label',(x,y,z));` entity in `entities` — the genuinely
//! different ISO 10303-21 AP214 (ISO 10303-521, Automotive Design) vocabulary for a 3D point,
//! ISO 10303 snake_case with no `IFC`-style prefix, unlike `🏗️ifc`'s `IFCCARTESIANPOINT` even
//! though both ride the identical Part-21 syntax).

use crate::artifacts::step::StepSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::bounds::{compute_step_bounds, StepBounds};

//#region 🔖️Inference
/// 💡️ Everything inferable from a STEP AP214 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.step.inference")]
pub struct StepInference {
    #[state(inferred)]
    pub bounds: StepBounds,
}

impl protocol::Inference<StepSnapshot> for StepInference {
    fn infer(snapshot: &StepSnapshot) -> Self {
        Self { bounds: compute_step_bounds(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `StepSnapshot::default()`'s `entities` ever stops being empty.
impl Default for StepInference {
    fn default() -> Self {
        <Self as protocol::Inference<StepSnapshot>>::infer(&StepSnapshot::default())
    }
}

impl protocol::InferenceSpec<StepSnapshot> for StepInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.step.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.step.inference.bounds", reads: &["entities"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `bounds` is a single min/max fold over every `CARTESIAN_POINT`
/// entity in `entities`, already O(n) in total entity count with no honest per-entity incremental
/// decomposition (a merkle dep-chain over this flat entity list costs more than the fold it would
/// cache) — the default `infer_cached` passthrough is exact.
impl ArtifactInferrer for crate::artifacts::step::standards::v_ap214::subsets::any::schema::StepBuilder {
    type Snapshot = StepSnapshot;
    type Inference = StepInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.step.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `step_artifact_schema_descriptor`'s registration.
pub fn step_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.step.inference",
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

    #[test]
    fn inference_determinism_law() {
        let snapshot = StepSnapshot::default();
        assert_eq!(StepInference::infer(&snapshot), StepInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(StepInference::infer(&StepSnapshot::default()), StepInference::default());
    }
}
//#endregion 🧪️Tests
