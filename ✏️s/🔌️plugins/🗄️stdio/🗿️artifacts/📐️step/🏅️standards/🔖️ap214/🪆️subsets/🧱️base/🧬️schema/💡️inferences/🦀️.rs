//! 💡️ StepInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`, honestly derivable by
//! folding every real `CARTESIAN_POINT('label',(x,y,z));` entity in `entities` — the genuinely
//! different ISO 10303-21 AP214 (ISO 10303-521, Automotive Design) vocabulary for a 3D point,
//! ISO 10303 snake_case with no `IFC`-style prefix, unlike `🏗️ifc`'s `IFCCARTESIANPOINT` even
//! though both ride the identical Part-21 syntax).

use crate::StepSnapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::bounds::{compute_step_bounds};
//#region 🔖️Inference
/// 💡️ Everything inferable from a STEP AP214 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.step.inference")]
pub struct StepInference {
    #[derived]
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
impl ArtifactInferrer for crate::standards::v_ap214::subsets::base::schema::StepBuilder {
    type Snapshot = StepSnapshot;
    type Inference = StepInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.step.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `step_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn step_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.stdio.step.inference",
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

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::bounds::StepBounds;
//#endregion 🔁️Re-exports
