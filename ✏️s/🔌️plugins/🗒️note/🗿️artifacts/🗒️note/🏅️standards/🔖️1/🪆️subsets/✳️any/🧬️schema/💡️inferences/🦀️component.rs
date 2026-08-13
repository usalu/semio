//! 💡️ Note inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::note::NoteSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::NoteOutline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a note snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir) — a note document
/// IS its own outline (a flat/grouped block tree), so `outline` here is the block name list
/// (flattened through `Group` nesting) plus real `blockCount`/`wordCount` stats.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.note.note.inference")]
pub struct NoteInference {
    #[derived]
    pub outline: NoteOutline,
}

impl protocol::Inference<NoteSnapshot> for NoteInference {
    fn infer(snapshot: &NoteSnapshot) -> Self {
        Self { outline: NoteOutline::compute(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `NoteSnapshot::default()`'s `blocks` field ever stops being empty.
impl Default for NoteInference {
    fn default() -> Self {
        <Self as protocol::Inference<NoteSnapshot>>::infer(&NoteSnapshot::default())
    }
}

impl protocol::InferenceSpec<NoteSnapshot> for NoteInference {
    fn inference_schema_id() -> &'static str {
        "s.note.note.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.note.note.inference.outline", reads: &["blocks"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::note::standards::v1::subsets::any::schema::NoteBuilder {
    type Snapshot = NoteSnapshot;
    type Inference = NoteInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.note.note.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `note_artifact_schema_descriptor`'s registration.
pub fn note_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.note.note.inference",
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

    #[test]
    fn inference_determinism_law() {
        let snapshot = NoteSnapshot::default();
        assert_eq!(NoteInference::infer(&snapshot), NoteInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(NoteInference::infer(&NoteSnapshot::default()), NoteInference::default());
    }
}
//#endregion 🧪️Tests
