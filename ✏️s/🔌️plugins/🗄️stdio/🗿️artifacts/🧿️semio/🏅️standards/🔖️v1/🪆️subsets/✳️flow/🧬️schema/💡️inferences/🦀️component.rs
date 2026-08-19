//! 💡️ SemioFlowInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`, honestly derivable
//! from this directed node/edge graph's own `nodes`/`edges` — the same Kahn's-algorithm shape
//! trinity's own `jack` inference facet establishes for its own node/edge graph).

use crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::topology::{compute_semio_flow_topology, SemioFlowTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a semio flow snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir). Derives
/// `Default` — safe here because `SemioFlowTopology` hand-rolls its own `Default` to agree with
/// `compute_semio_flow_topology(&SemioFlowSnapshot::default())` (see the slug dir's own doc
/// comment), so the derived struct-level `Default` (all-default fields) matches `infer` honestly.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.flow.inference")]
pub struct SemioFlowInference {
    #[derived]
    pub topology: SemioFlowTopology,
}

impl protocol::Inference<SemioFlowSnapshot> for SemioFlowInference {
    async fn infer(snapshot: &SemioFlowSnapshot) -> Self {
        Self { topology: compute_semio_flow_topology(snapshot) }
    }
}

impl protocol::InferenceSpec<SemioFlowSnapshot> for SemioFlowInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.semio.flow.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.semio.flow.inference.topology", reads: &["nodes", "edges"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 🧠️ Uncached: Kahn's algorithm re-runs in one BFS pass over the whole graph — the default
/// `infer_cached` passthrough (just calls `infer`) is exactly right here, no `InferredField`
/// chain needed (there is no honest per-node incremental decomposition of a global topological
/// sort) — same ruling trinity's `jack` inference facet documents for its own node/edge graph.
impl ArtifactInferrer for crate::artifacts::semio::standards::v1::subsets::flow::schema::SemioFlowBuilder {
    type Snapshot = SemioFlowSnapshot;
    type Inference = SemioFlowInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.flow.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `semio_flow_artifact_schema_descriptor`'s registration.
pub async fn semio_flow_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.flow.inference",
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
        let snapshot = SemioFlowSnapshot::default();
        assert_eq!(SemioFlowInference::infer(&snapshot), SemioFlowInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(SemioFlowInference::infer(&SemioFlowSnapshot::default()), SemioFlowInference::default());
    }
}
//#endregion 🧪️Tests
