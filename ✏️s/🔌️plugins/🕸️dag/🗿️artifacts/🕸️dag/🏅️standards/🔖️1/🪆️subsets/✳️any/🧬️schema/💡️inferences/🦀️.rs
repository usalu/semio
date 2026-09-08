//! 💡️ Dag inference schema — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::DagSnapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::topology::{compute_dag_topology, DagTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a dag snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.dag.dag.inference")]
pub struct DagInference {
    #[derived]
    pub topology: DagTopology,
}

impl protocol::Inference<DagSnapshot> for DagInference {
    fn infer(snapshot: &DagSnapshot) -> Self {
        let scene = crate::dag_working_scene(snapshot);
        Self { topology: compute_dag_topology(&scene.nodes, &scene.edges) }
    }
}

/// 🌱 Hand-fixed to agree with `infer(&DagSnapshot::default())` rather than a naive
/// `#[derive(Default)]` — `DagSnapshot`'s own `Default` parses the bundled example document
/// (non-empty), the same "match `infer` of the real default, don't derive structurally" trick as
/// `AddInference`'s hand-written `Default` in `📡️spr/🎮️command/🦀️.rs`.
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
        &[protocol::InferenceFieldSpec { id: "s.dag.dag.inference.topology", reads: &["content"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 🎯️ `ArtifactInferrer::infer` takes `&Self::Snapshot`, never `&self` — the impl target is a
/// pure type-level anchor, not a live instance. Retargeting onto `semio_framework_plugin::app::
/// SnapshotBuilder<DagSnapshot, DagMutation>` (the recipe's literal suggestion for a deleted
/// `derive_artifact_facets!` builder type) is illegal — `SnapshotBuilder` is a foreign,
/// non-`#[fundamental]` generic struct, so `impl ArtifactInferrer for SnapshotBuilder<Local, Local>`
/// is an orphan-rule violation (E0117) regardless of the type PARAMETERS being local. A trivial
/// local zero-sized marker struct is the real fix (recipeGaps of the `🎬️sequence` W4 pass).
pub struct DagInferrer;
impl ArtifactInferrer for DagInferrer {
    type Snapshot = DagSnapshot;
    type Inference = DagInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.dag.dag.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `dag_artifact_schema_descriptor`'s registration.
pub fn dag_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.dag.dag.inference",
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
