//! 💡️ Imperative inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::artifacts::imperative::ImperativeSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::topology::{compute_imperative_topology, ImperativeTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from an imperative snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.imperative.imperative.inference")]
pub struct ImperativeInference {
    #[derived]
    pub topology: ImperativeTopology,
}

impl protocol::Inference<ImperativeSnapshot> for ImperativeInference {
    async fn infer(snapshot: &ImperativeSnapshot) -> Self {
        let path = crate::artifacts::imperative::imperative_working_scene(snapshot).path;
        Self { topology: compute_imperative_topology(&path) }
    }
}

/// 🌱 Hand-fixed to agree with `infer(&ImperativeSnapshot::default())` rather than a naive
/// `#[derive(Default)]`, the same "match `infer` of the real default, don't derive structurally"
/// trick as `AddInference`'s hand-written `Default` in `📡️spr/🎮️command/🦀️component.rs` — here it
/// happens to coincide with the structural zero, since `ImperativeSnapshot::default()`'s working-
/// scene `path` is already empty, but the explicit `infer`-based impl keeps every inference family
/// in this fan-out consistent regardless of which artifacts' defaults are trivial.
impl Default for ImperativeInference {
    fn default() -> Self {
        <Self as protocol::Inference<ImperativeSnapshot>>::infer(&ImperativeSnapshot::default())
    }
}

impl protocol::InferenceSpec<ImperativeSnapshot> for ImperativeInference {
    async fn inference_schema_id() -> &'static str {
        "s.imperative.imperative.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.imperative.imperative.inference.topology", reads: &["flow"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::imperative::standards::v1::subsets::any::schema::ImperativeBuilder {
    type Snapshot = ImperativeSnapshot;
    type Inference = ImperativeInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.imperative.imperative.inference`'s facet leaves into the OS-wide inference
/// catalog — call once at plugin init, alongside `imperative_artifact_schema_descriptor`'s
/// registration.
pub async fn imperative_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.imperative.imperative.inference",
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
    use crate::artifacts::imperative::{Path, Step};
    use protocol::Inference;
    use std::collections::BTreeMap;

    async fn chain_snapshot() -> ImperativeSnapshot {
        let path = Path { steps: vec![Step { id: "a".into(), kind: "noop".into(), params: Default::default(), bodies: BTreeMap::new() }, Step { id: "b".into(), kind: "noop".into(), params: Default::default(), bodies: BTreeMap::new() }] };
        crate::artifacts::imperative::imperative_snapshot_with_content("imperative.document", &path, &BTreeMap::new())
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = chain_snapshot();
        assert_eq!(ImperativeInference::infer(&snapshot), ImperativeInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(ImperativeInference::infer(&ImperativeSnapshot::default()), ImperativeInference::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn topology_counts_every_step_exactly_once() {
        let snapshot = chain_snapshot();
        let inferred = ImperativeInference::infer(&snapshot);
        assert_eq!(inferred.topology.node_count, 2);
        assert_eq!(inferred.topology.topo_order, vec!["a".to_string(), "b".to_string()]);
        assert!(inferred.topology.cycle_free);
    }
}
//#endregion 🧪️Tests
