//! 💡️ Dag inference schema — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::artifacts::dag::DagSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::topology::{compute_dag_topology, DagTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a dag snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.dag.dag.inference")]
pub struct DagInference {
    #[state(inferred)]
    pub topology: DagTopology,
}

impl protocol::Inference<DagSnapshot> for DagInference {
    fn infer(snapshot: &DagSnapshot) -> Self {
        Self { topology: compute_dag_topology(&snapshot.nodes, &snapshot.edges) }
    }
}

/// 🌱 Hand-fixed to agree with `infer(&DagSnapshot::default())` rather than a naive
/// `#[derive(Default)]` — `DagSnapshot`'s own `Default` parses the bundled example document
/// (non-empty), the same "match `infer` of the real default, don't derive structurally" trick as
/// `AddInference`'s hand-written `Default` in `📡️spr/🎮️command/🦀️component.rs`.
impl Default for DagInference {
    fn default() -> Self {
        <Self as protocol::Inference<DagSnapshot>>::infer(&DagSnapshot::default())
    }
}

impl protocol::InferenceSpec<DagSnapshot> for DagInference {
    fn inference_schema_id() -> &'static str {
        "s.dag.dag.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.dag.dag.inference.topology", reads: &["nodes", "edges"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::dag::standards::v1::subsets::any::schema::DagBuilder {
    type Snapshot = DagSnapshot;
    type Inference = DagInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.dag.dag.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `dag_artifact_schema_descriptor`'s registration.
pub fn dag_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.dag.dag.inference",
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
    use crate::artifacts::dag::{DagFixtureEdge, DagNodeSpec};
    use protocol::Inference;

    fn chain_snapshot() -> DagSnapshot {
        let a = DagNodeSpec { id: "a".into(), ..Default::default() };
        let b = DagNodeSpec { id: "b".into(), ..Default::default() };
        DagSnapshot { schema: "dag.dag".into(), nodes: vec![a, b], edges: vec![DagFixtureEdge { id: "e1".into(), source: "a".into(), target: "b".into() }] }
    }

    #[test]
    fn inference_determinism_law() {
        let snapshot = chain_snapshot();
        assert_eq!(DagInference::infer(&snapshot), DagInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(DagInference::infer(&DagSnapshot::default()), DagInference::default());
    }

    #[test]
    fn topology_counts_every_node_exactly_once() {
        let snapshot = chain_snapshot();
        let inferred = DagInference::infer(&snapshot);
        assert_eq!(inferred.topology.node_count as usize, snapshot.nodes.len());
        assert_eq!(inferred.topology.topo_order.len(), snapshot.nodes.len());
        assert!(inferred.topology.cycle_free);
    }
}
//#endregion 🧪️Tests
