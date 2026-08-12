//! 💡️ Pdf (1.7) inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::Pdf17Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a pdf (1.7) snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf.1.7.inference")]
pub struct Pdf17Inference {
    #[state(inferred)]
    pub outline: Pdf17Outline,
}

impl protocol::Inference<PdfSnapshot> for Pdf17Inference {
    fn infer(snapshot: &PdfSnapshot) -> Self {
        Self { outline: Pdf17Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<PdfSnapshot> for Pdf17Inference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.pdf.1.7.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.pdf.1.7.inference.outline", reads: &["pages", "info"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::pdf::standards::v1_7::subsets::any::schema::PdfBuilder {
    type Snapshot = PdfSnapshot;
    type Inference = Pdf17Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.pdf.1.7.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `pdf_artifact_schema_descriptor`'s registration.
pub fn pdf17_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.pdf.1.7.inference",
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
        let snapshot = PdfSnapshot::default();
        assert_eq!(Pdf17Inference::infer(&snapshot), Pdf17Inference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(Pdf17Inference::infer(&PdfSnapshot::default()), Pdf17Inference::default());
    }
}
//#endregion 🧪️Tests
