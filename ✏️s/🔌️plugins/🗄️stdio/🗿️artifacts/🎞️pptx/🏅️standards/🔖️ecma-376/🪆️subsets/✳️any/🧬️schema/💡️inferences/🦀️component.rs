//! 💡️ Pptx inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::pptx::PptxSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::PptxOutline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a pptx snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pptx.inference")]
pub struct PptxInference {
    #[derived]
    pub outline: PptxOutline,
}

impl protocol::Inference<PptxSnapshot> for PptxInference {
    async fn infer(snapshot: &PptxSnapshot) -> Self {
        Self { outline: PptxOutline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<PptxSnapshot> for PptxInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.pptx.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.pptx.inference.outline", reads: &["presentation"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::PptxBuilder {
    type Snapshot = PptxSnapshot;
    type Inference = PptxInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.pptx.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `pptx_artifact_schema_descriptor`'s registration.
pub async fn pptx_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.pptx.inference",
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
    async fn inference_determinism_law() {
        let snapshot = PptxSnapshot::default();
        assert_eq!(PptxInference::infer(&snapshot), PptxInference::infer(&snapshot));
    }

    #[test]
    async fn inference_default_law() {
        assert_eq!(PptxInference::infer(&PptxSnapshot::default()), PptxInference::default());
    }
}
//#endregion 🧪️Tests
