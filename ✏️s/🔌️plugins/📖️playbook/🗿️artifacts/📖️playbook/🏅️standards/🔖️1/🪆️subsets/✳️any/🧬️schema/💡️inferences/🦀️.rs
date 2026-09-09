//! 💡️ Playbook inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use super::topology::compute_playbook_topology;
use crate::PlaybookSnapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Inference
/// 💡️ Everything inferable from a playbook snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.playbook.playbook.inference")]
pub struct PlaybookInference {
    #[derived]
    pub topology: PlaybookTopology,
}

impl Default for PlaybookInference {
    fn default() -> Self {
        <Self as protocol::Inference<PlaybookSnapshot>>::infer(&PlaybookSnapshot::default())
    }
}

impl protocol::Inference<PlaybookSnapshot> for PlaybookInference {
    fn infer(snapshot: &PlaybookSnapshot) -> Self {
        Self { topology: compute_playbook_topology(&snapshot.steps()) }
    }
}

impl protocol::InferenceSpec<PlaybookSnapshot> for PlaybookInference {
    fn inference_schema_id() -> &'static str {
        "s.playbook.playbook.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.playbook.playbook.inference.topology", reads: &["steps"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::PlaybookBuilder {
    type Snapshot = PlaybookSnapshot;
    type Inference = PlaybookInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.playbook.playbook.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `playbook_artifact_schema_descriptor`'s registration.
pub fn playbook_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.playbook.playbook.inference",
        inference: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
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
pub use super::topology::PlaybookTopology;
//#endregion 🔁️Re-exports
