//! 💡️ Playbook inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use super::topology::compute_playbook_topology;
use crate::PlaybookSnapshot;
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Inference
/// 💡️ Everything inferable from a playbook snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir). A fold is pure over the PARENT snapshot (design
/// §20.9), whose steps live on the `flow` child (§20.15): until inference receives the child packs as dependencies (AGNOSTIC
/// W-b), the parent alone infers the topology of no steps.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.playbook.playbook.inference")]
pub struct PlaybookInference {
    #[derived]
    pub topology: PlaybookTopology,
}

impl Default for PlaybookInference {
    fn default() -> Self {
        Self { topology: compute_playbook_topology(&[]) }
    }
}

impl protocol::Inference<PlaybookSnapshot> for PlaybookInference {
    fn infer(_snapshot: &PlaybookSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok(Self::default())
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
        &[protocol::InferenceFieldSpec { id: "s.playbook.playbook.inference.topology", reads: &["flow"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.playbook.playbook.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `playbook_artifact_schema_descriptor`'s registration.
pub fn playbook_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.playbook.playbook.inference",
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
pub use super::topology::PlaybookTopology;
//#endregion 🔁️Re-exports
