//! 💡️ SemioFlowInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`, honestly derivable
//! from this directed node/edge graph's own `nodes`/`edges` — the same Kahn's-algorithm shape
//! trinity's own `jack` inference facet establishes for its own node/edge graph).

use crate::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use framework_schema::ArtifactSchema;

use super::topology::compute_semio_flow_topology;
//#region 🔖️Inference
/// 💡️ Everything inferable from a semio flow snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir). Derives
/// `Default` — safe here because `SemioFlowTopology` hand-rolls its own `Default` to agree with
/// `compute_semio_flow_topology(&SemioFlowSnapshot::default())` (see the slug dir's own doc
/// comment), so the derived struct-level `Default` (all-default fields) matches `infer` honestly.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.flow.inference")]
pub struct SemioFlowInference {
    #[derived]
    pub topology: SemioFlowTopology,
}

impl protocol::Inference<SemioFlowSnapshot> for SemioFlowInference {
    fn infer(snapshot: &SemioFlowSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { topology: compute_semio_flow_topology(snapshot) }
    
        })
    }
}

impl protocol::InferenceSpec<SemioFlowSnapshot> for SemioFlowInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.semio.flow.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.semio.flow.inference.topology", reads: &["nodes", "edges"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.flow.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `semio_flow_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_flow_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.flow.inference",
        inference: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
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
pub use super::topology::SemioFlowTopology;
//#endregion 🔁️Re-exports
