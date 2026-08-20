//! 💡️ SemioPresentationInference — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/` — same shape `document`'s
//! own outline facet establishes, since `SlideShape::TextBox`/`Table` cell content reuse
//! `document::DocBlock` verbatim, per this subset's own module doc comment).

use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::{compute_semio_presentation_outline, SemioPresentationOutline};

//#region 🔖️Inference
/// 💡️ Everything inferable from a semio presentation snapshot. One field per named inference
/// under `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.presentation.inference")]
pub struct SemioPresentationInference {
    #[derived]
    pub outline: SemioPresentationOutline,
}

impl protocol::Inference<SemioPresentationSnapshot> for SemioPresentationInference {
    async fn infer(snapshot: &SemioPresentationSnapshot) -> Self {
        Self { outline: compute_semio_presentation_outline(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — `SemioPresentationSnapshot::default()` happens
/// to be all-empty today (no masters/layouts/slides), so a naive derive would happen to agree, but
/// tying `Default` to `infer` keeps the law correct even if that default ever stops being
/// all-empty (the same defensive pattern raster's `RasterInference` documents).
impl Default for SemioPresentationInference {
    fn default() -> Self {
        <Self as protocol::Inference<SemioPresentationSnapshot>>::infer(&SemioPresentationSnapshot::default())
    }
}

impl protocol::InferenceSpec<SemioPresentationSnapshot> for SemioPresentationInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.semio.presentation.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.semio.presentation.inference.outline", reads: &["masters", "layouts", "slides"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here (a recursive block walk is a single whole-snapshot pass over
/// already-flat `masters`/`layouts`/`slides` collections, no per-entity incremental decomposition
/// applies) — the default `infer_cached` passthrough (`ArtifactInferrer::infer_cached`) is exact.
impl ArtifactInferrer for crate::artifacts::semio::standards::v1::subsets::presentation::schema::SemioPresentationBuilder {
    type Snapshot = SemioPresentationSnapshot;
    type Inference = SemioPresentationInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.presentation.inference`'s facet leaves into the OS-wide inference
/// catalog — call once at plugin init, alongside `semio_presentation_artifact_schema_descriptor`'s
/// registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_presentation_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.presentation.inference",
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
        let snapshot = SemioPresentationSnapshot::default();
        assert_eq!(SemioPresentationInference::infer(&snapshot), SemioPresentationInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(SemioPresentationInference::infer(&SemioPresentationSnapshot::default()), SemioPresentationInference::default());
    }
}
//#endregion 🧪️Tests
