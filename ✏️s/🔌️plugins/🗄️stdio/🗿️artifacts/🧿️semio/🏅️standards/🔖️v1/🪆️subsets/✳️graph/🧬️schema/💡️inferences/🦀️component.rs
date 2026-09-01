//! 💡️ SemioGraphInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`, honestly derivable
//! from this typed property graph's own `nodes`/`edges` — the same Kahn's-algorithm shape
//! trinity's own `jack` and sibling `✳️flow` inference facets establish for their own node/edge
//! graphs).

use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::topology::{compute_semio_graph_topology, SemioGraphTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a semio graph snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.graph.inference")]
pub struct SemioGraphInference {
    #[derived]
    pub topology: SemioGraphTopology,
}

impl protocol::Inference<SemioGraphSnapshot> for SemioGraphInference {
    fn infer(snapshot: &SemioGraphSnapshot) -> Self {
        Self { topology: compute_semio_graph_topology(snapshot) }
    }
}

impl protocol::InferenceSpec<SemioGraphSnapshot> for SemioGraphInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.semio.graph.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.semio.graph.inference.topology", reads: &["nodes", "edges"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 🧠️ Uncached: Kahn's algorithm re-runs in one BFS pass over the whole graph — the default
/// `infer_cached` passthrough (just calls `infer`) is exactly right here, no `InferredField`
/// chain needed (there is no honest per-node incremental decomposition of a global topological
/// sort) — same ruling trinity's `jack` and sibling `✳️flow` inference facets document.
impl ArtifactInferrer for crate::artifacts::semio::standards::v1::subsets::graph::schema::SemioGraphBuilder {
    type Snapshot = SemioGraphSnapshot;
    type Inference = SemioGraphInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.graph.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `semio_graph_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_graph_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.graph.inference",
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
        let snapshot = SemioGraphSnapshot::default();
        assert_eq!(SemioGraphInference::infer(&snapshot), SemioGraphInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(SemioGraphInference::infer(&SemioGraphSnapshot::default()), SemioGraphInference::default());
    }
}
//#endregion 🧪️Tests
