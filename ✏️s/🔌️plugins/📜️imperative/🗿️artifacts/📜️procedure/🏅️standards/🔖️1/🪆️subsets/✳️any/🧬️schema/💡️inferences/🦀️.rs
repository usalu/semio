//! 💡️ Imperative inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::ProcedureSnapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::topology::{compute_procedure_topology, ProcedureTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from an imperative snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.imperative.procedure.inference")]
pub struct ProcedureInference {
    #[derived]
    pub topology: ProcedureTopology,
}

impl protocol::Inference<ProcedureSnapshot> for ProcedureInference {
    fn infer(snapshot: &ProcedureSnapshot) -> Self {
        let path = crate::procedure_working_scene(snapshot).path;
        Self { topology: compute_procedure_topology(&path) }
    }
}

/// 🌱 Hand-fixed to agree with `infer(&ProcedureSnapshot::default())` rather than a naive
/// `#[derive(Default)]`, the same "match `infer` of the real default, don't derive structurally"
/// trick as `AddInference`'s hand-written `Default` in `📡️spr/🎮️command/🦀️.rs` — here it
/// happens to coincide with the structural zero, since `ProcedureSnapshot::default()`'s working-
/// scene `path` is already empty, but the explicit `infer`-based impl keeps every inference family
/// in this fan-out consistent regardless of which artifacts' defaults are trivial.
impl Default for ProcedureInference {
    fn default() -> Self {
        <Self as protocol::Inference<ProcedureSnapshot>>::infer(&ProcedureSnapshot::default())
    }
}

impl protocol::InferenceSpec<ProcedureSnapshot> for ProcedureInference {
    fn inference_schema_id() -> &'static str {
        "s.imperative.procedure.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.imperative.procedure.inference.topology", reads: &["flow"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::ProcedureBuilder {
    type Snapshot = ProcedureSnapshot;
    type Inference = ProcedureInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.imperative.procedure.inference`'s facet leaves into the OS-wide inference
/// catalog — call once at plugin init, alongside `procedure_artifact_schema_descriptor`'s
/// registration.
pub fn procedure_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.imperative.procedure.inference",
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
