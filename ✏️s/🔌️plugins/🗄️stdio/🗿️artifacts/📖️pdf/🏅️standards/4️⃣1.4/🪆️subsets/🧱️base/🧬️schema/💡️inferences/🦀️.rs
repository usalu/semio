//! 💡️ Pdf (1.4) inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::pdf::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::Inference;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::outline::PdfOutline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a pdf (1.4) snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf.inference")]
pub struct PdfInference {
    #[derived]
    pub outline: PdfOutline,
}

impl Inference<PdfSnapshot> for PdfInference {
    fn infer(snapshot: &PdfSnapshot) -> Self {
        Self { outline: PdfOutline::compute(snapshot) }
    }
}

/// 🪞️ Hand impl (not derived): a pdf 1.4 document always has at least one page, so
/// `PdfSnapshot::default()` is non-empty and `PdfOutline::compute` over it reports a real page
/// count — which the derived all-zero `PdfOutline::default()` contradicts, breaking
/// `inference_default_law`. Defining default as "infer the default snapshot" makes the two
/// definitionally equal.
impl Default for PdfInference {
    fn default() -> Self {
        Self::infer(&PdfSnapshot::default())
    }
}

impl protocol::InferenceSpec<PdfSnapshot> for PdfInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.pdf.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.pdf.inference.outline", reads: &["pages"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::pdf::standards::v1_4::subsets::base::schema::PdfBuilder {
    type Snapshot = PdfSnapshot;
    type Inference = PdfInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.pdf.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `pdf_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn pdf_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.pdf.inference",
        inference: schema::FacetLeaves {
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
mod tests {
    use super::*;
    use protocol::Inference;

    #[test]
    fn inference_determinism_law() {
        let snapshot = PdfSnapshot::default();
        assert_eq!(PdfInference::infer(&snapshot), PdfInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(PdfInference::infer(&PdfSnapshot::default()), PdfInference::default());
    }
}
//#endregion 🧪️Tests
