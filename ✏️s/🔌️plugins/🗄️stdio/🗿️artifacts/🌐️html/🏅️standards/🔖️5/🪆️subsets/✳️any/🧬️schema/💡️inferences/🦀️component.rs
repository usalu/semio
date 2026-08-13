//! 💡️ Html inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::html::HtmlSnapshot;
use protocol::Inference;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::HtmlOutline;

//#region 🔖️Inference
/// 💡️ Everything inferable from an html snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.html.inference")]
pub struct HtmlInference {
    #[derived]
    pub outline: HtmlOutline,
}

impl protocol::Inference<HtmlSnapshot> for HtmlInference {
    fn infer(snapshot: &HtmlSnapshot) -> Self {
        Self { outline: HtmlOutline::compute(snapshot) }
    }
}

/// 🪞️ Hand impl (not derived): `HtmlSnapshot::default()` is not empty — it carries a root element —
/// so `HtmlOutline::default()`'s all-zero shape disagrees with `HtmlOutline::compute` over that
/// default snapshot, breaking `inference_default_law`. Defining default as "infer the default
/// snapshot" makes the two definitionally equal.
impl Default for HtmlInference {
    fn default() -> Self {
        Self::infer(&HtmlSnapshot::default())
    }
}

impl protocol::InferenceSpec<HtmlSnapshot> for HtmlInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.html.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.html.inference.outline", reads: &["root"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::html::standards::v5::subsets::any::schema::HtmlBuilder {
    type Snapshot = HtmlSnapshot;
    type Inference = HtmlInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.html.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `html_artifact_schema_descriptor`'s registration.
pub fn html_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.html.inference",
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
        let snapshot = HtmlSnapshot::default();
        assert_eq!(HtmlInference::infer(&snapshot), HtmlInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(HtmlInference::infer(&HtmlSnapshot::default()), HtmlInference::default());
    }
}
//#endregion 🧪️Tests
