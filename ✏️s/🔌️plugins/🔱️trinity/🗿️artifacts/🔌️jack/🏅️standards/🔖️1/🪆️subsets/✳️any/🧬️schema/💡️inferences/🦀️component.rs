//! 💡️ Jack inference schema — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors puzzle3d's own `💡️inferences/` (the pattern's exemplar): this file is the family-root
//! assembly (never mod's/includes the slug dirs directly — `📦️glue.rs` is the sole mounting
//! mechanism, same as mutations); each named inference gets its own `<emoji><slug>/` child
//! (currently: `🧭topology/`, honestly derivable from this directed property port graph's own
//! `nodes`/`edges` — the closest fit from the family root's "workflow/dag-shaped" category, this
//! artifact being a directed node/edge graph with a `root_node_id`, not the "positioned" category
//! (its `x`/`y` are already explicit fields, not something to re-derive); and `🎛flat-position/`,
//! ported from the former `Graph::recompute_derived` — each node's flattened `(u, v)` position is
//! honestly re-derivable from `nodes`/`edges`/`root_node_id` alone, so it moved out of the manifest's
//! former `flatPosition` `"derived"` node property into this family instead).

use crate::artifacts::jack::JackSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::flat_position::{compute_flat_position, JackFlatPosition};
use super::topology::{compute_topology, JackTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a jack snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by `🧭topology/`; `flat_position`, backed by
/// `🎛flat-position/`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.jack.inference")]
pub struct JackInference {
    #[derived]
    pub topology: JackTopology,
    #[derived]
    pub flat_position: JackFlatPosition,
}

impl protocol::Inference<JackSnapshot> for JackInference {
    async fn infer(snapshot: &JackSnapshot) -> Self {
        Self { topology: compute_topology(snapshot), flat_position: compute_flat_position(snapshot) }
    }
}

impl protocol::InferenceSpec<JackSnapshot> for JackInference {
    async fn inference_schema_id() -> &'static str {
        "s.trinity.jack.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.trinity.jack.inference.topology", reads: &["nodes", "edges"] }, protocol::InferenceFieldSpec { id: "s.trinity.jack.inference.flatPosition", reads: &["nodes", "edges", "root_node_id"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 🧠️ Uncached: Kahn's algorithm re-runs in one BFS pass over the whole graph — the default
/// `infer_cached` passthrough (just calls `infer`) is exactly right here, no `InferredField` chain
/// needed (there is no honest per-node incremental decomposition of a global topological sort).
impl ArtifactInferrer for crate::artifacts::jack::standards::v1::subsets::any::schema::JackBuilder {
    type Snapshot = JackSnapshot;
    type Inference = JackInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.trinity.jack.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `jack_artifact_schema_descriptor`'s registration.
pub async fn jack_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.trinity.jack.inference",
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
    use crate::artifacts::jack::{Edge, Node, Port, PortDirection, PropertyBag};
    use protocol::Inference;

    //#region 🧸️Fixtures
    async fn node(id: &str) -> Node {
        Node {
            id: id.into(),
            kind: "Piece".into(),
            name: id.into(),
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            properties: PropertyBag::new(),
            ports: vec![
                Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() },
                Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() },
            ],
        }
    }

    async fn edge(id: &str, source: &str, target: &str) -> Edge {
        Edge { id: id.into(), kind: "Connection".into(), source: source.into(), target: target.into(), properties: PropertyBag::new() }
    }

    async fn chain_snapshot() -> JackSnapshot {
        JackSnapshot::with_content(
            "trinity.graph".into(),
            "chain".into(),
            None,
            Default::default(),
            Default::default(),
            vec![node("root"), node("mid"), node("leaf")],
            vec![edge("e1", "root@out", "mid@in"), edge("e2", "mid@out", "leaf@in")],
            Some("root".into()),
        )
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = chain_snapshot();
        assert_eq!(JackInference::infer(&snapshot), JackInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(JackInference::infer(&JackSnapshot::default()), JackInference::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_matches_compute_topology_directly() {
        let snapshot = chain_snapshot();
        let inferred = JackInference::infer(&snapshot);
        assert_eq!(inferred.topology, compute_topology(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_matches_compute_flat_position_directly() {
        let snapshot = chain_snapshot();
        let inferred = JackInference::infer(&snapshot);
        assert_eq!(inferred.flat_position, compute_flat_position(&snapshot));
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
