//! 💡️ Xml inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::xml::XmlSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::outline::XmlOutline;

//#region 🔖️Inference
/// 💡️ Everything inferable from an xml snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xml.inference")]
pub struct XmlInference {
    #[derived]
    pub outline: XmlOutline,
}

impl protocol::Inference<XmlSnapshot> for XmlInference {
    fn infer(snapshot: &XmlSnapshot) -> Self {
        Self { outline: XmlOutline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<XmlSnapshot> for XmlInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.xml.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.xml.inference.outline", reads: &["doc"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::xml::standards::v1_0::subsets::base::schema::XmlBuilder {
    type Snapshot = XmlSnapshot;
    type Inference = XmlInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.xml.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `xml_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn xml_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.xml.inference",
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

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = XmlSnapshot::default();
        assert_eq!(XmlInference::infer(&snapshot), XmlInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(XmlInference::infer(&XmlSnapshot::default()), XmlInference::default());
    }
}
//#endregion 🧪️Tests
