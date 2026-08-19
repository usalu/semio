//! 💡️ SemioDocumentInference — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`, honestly derivable
//! from `blocks` alone — the same shape stdio's own `md`/`docx`/`pptx` inference facets already
//! establish for their own recursive block trees).

use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::{compute_semio_document_outline, SemioDocumentOutline};

//#region 🔖️Inference
/// 💡️ Everything inferable from a semio document snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.document.inference")]
pub struct SemioDocumentInference {
    #[derived]
    pub outline: SemioDocumentOutline,
}

impl protocol::Inference<SemioDocumentSnapshot> for SemioDocumentInference {
    async fn infer(snapshot: &SemioDocumentSnapshot) -> Self {
        Self { outline: compute_semio_document_outline(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `SemioDocumentSnapshot::default()`'s `blocks` ever stops being empty.
impl Default for SemioDocumentInference {
    fn default() -> Self {
        <Self as protocol::Inference<SemioDocumentSnapshot>>::infer(&SemioDocumentSnapshot::default())
    }
}

impl protocol::InferenceSpec<SemioDocumentSnapshot> for SemioDocumentInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.semio.document.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.semio.document.inference.outline", reads: &["blocks"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `outline` is a single recursive walk over `blocks` (already
/// O(n) in total block count, gathering headings + block/word counts in one pass), with no honest
/// per-entity incremental decomposition — the default `infer_cached` passthrough is exact.
impl ArtifactInferrer for crate::artifacts::semio::standards::v1::subsets::document::schema::SemioDocumentBuilder {
    type Snapshot = SemioDocumentSnapshot;
    type Inference = SemioDocumentInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.document.inference`'s facet leaves into the OS-wide inference
/// catalog — call once at plugin init, alongside `semio_document_artifact_schema_descriptor`'s
/// registration.
pub async fn semio_document_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.document.inference",
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
        let snapshot = SemioDocumentSnapshot::default();
        assert_eq!(SemioDocumentInference::infer(&snapshot), SemioDocumentInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(SemioDocumentInference::infer(&SemioDocumentSnapshot::default()), SemioDocumentInference::default());
    }
}
//#endregion 🧪️Tests
