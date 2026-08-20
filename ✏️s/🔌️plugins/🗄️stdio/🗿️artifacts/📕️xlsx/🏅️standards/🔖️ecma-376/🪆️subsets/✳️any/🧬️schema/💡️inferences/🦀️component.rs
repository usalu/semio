//! 💡️ Xlsx inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::xlsx::XlsxSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::XlsxOutline;

//#region 🔖️Inference
/// 💡️ Everything inferable from an xlsx snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xlsx.inference")]
pub struct XlsxInference {
    #[derived]
    pub outline: XlsxOutline,
}

impl protocol::Inference<XlsxSnapshot> for XlsxInference {
    async fn infer(snapshot: &XlsxSnapshot) -> Self {
        Self { outline: XlsxOutline::compute(snapshot).await }
    }
}

impl protocol::InferenceSpec<XlsxSnapshot> for XlsxInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.xlsx.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.xlsx.inference.outline", reads: &["workbook"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::XlsxBuilder {
    type Snapshot = XlsxSnapshot;
    type Inference = XlsxInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.xlsx.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `xlsx_artifact_schema_descriptor`'s registration.
pub async fn xlsx_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.xlsx.inference",
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
        let snapshot = XlsxSnapshot::default();
        assert_eq!(XlsxInference::infer(&snapshot), XlsxInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(XlsxInference::infer(&XlsxSnapshot::default()), XlsxInference::default());
    }
}
//#endregion 🧪️Tests
